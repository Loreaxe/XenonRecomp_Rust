pub fn sub_825A3320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A3320 size=132
    let mut pc: u32 = 0x825A3320;
    'dispatch: loop {
        match pc {
            0x825A3320 => {
    //   block [0x825A3320..0x825A33A4)
	// 825A3320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A332C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3330: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A3334: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A3338: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825A333C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A3340: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825A3344: C00B9F60  lfs f0, -0x60a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24736 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3348: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A334C: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A3350: C189AC1C  lfs f12, -0x53e4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-21476 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A3354: D03F0010  stfs f1, 0x10(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A3358: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825A335C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A3360: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3364: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A3368: 911F0018  stw r8, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 825A336C: 4884FD85  bl 0x82df30f0
	ctx.lr = 0x825A3370;
	sub_82DF30F0(ctx, base);
	// 825A3370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3378: 419A000C  beq cr6, 0x825a3384
	if ctx.cr[6].eq {
	pc = 0x825A3384; continue 'dispatch;
	}
	// 825A337C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3380: 48000008  b 0x825a3388
	pc = 0x825A3388; continue 'dispatch;
	// 825A3384: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3388: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A338C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A3394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A339C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A33A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A33A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A33A8 size=108
    let mut pc: u32 = 0x825A33A8;
    'dispatch: loop {
        match pc {
            0x825A33A8 => {
    //   block [0x825A33A8..0x825A3414)
	// 825A33A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A33AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A33B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A33B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A33B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A33BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A33C0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A33C4: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825A33C8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 825A33CC: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 825A33D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A33D4: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 825A33D8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A33DC: 4884FD15  bl 0x82df30f0
	ctx.lr = 0x825A33E0;
	sub_82DF30F0(ctx, base);
	// 825A33E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A33E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A33E8: 419A000C  beq cr6, 0x825a33f4
	if ctx.cr[6].eq {
	pc = 0x825A33F4; continue 'dispatch;
	}
	// 825A33EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A33F0: 48000008  b 0x825a33f8
	pc = 0x825A33F8; continue 'dispatch;
	// 825A33F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A33F8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A33FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A3404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A340C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3418 size=108
    let mut pc: u32 = 0x825A3418;
    'dispatch: loop {
        match pc {
            0x825A3418 => {
    //   block [0x825A3418..0x825A3484)
	// 825A3418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A342C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3430: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A3434: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825A3438: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825A343C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 825A3440: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A3444: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 825A3448: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A344C: 488507B5  bl 0x82df3c00
	ctx.lr = 0x825A3450;
	sub_82DF3C00(ctx, base);
	// 825A3450: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3458: 419A000C  beq cr6, 0x825a3464
	if ctx.cr[6].eq {
	pc = 0x825A3464; continue 'dispatch;
	}
	// 825A345C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3460: 48000008  b 0x825a3468
	pc = 0x825A3468; continue 'dispatch;
	// 825A3464: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3468: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A346C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A3474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A347C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3488 size=136
    let mut pc: u32 = 0x825A3488;
    'dispatch: loop {
        match pc {
            0x825A3488 => {
    //   block [0x825A3488..0x825A3510)
	// 825A3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A349C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A34A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A34A4: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 825A34A8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825A34AC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825A34B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A34B4: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 825A34B8: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 825A34BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A34C0: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 825A34C4: 4BFE8A4D  bl 0x8258bf10
	ctx.lr = 0x825A34C8;
	sub_8258BF10(ctx, base);
	// 825A34C8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A34CC: 4884FC25  bl 0x82df30f0
	ctx.lr = 0x825A34D0;
	sub_82DF30F0(ctx, base);
	// 825A34D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A34D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A34D8: 419A000C  beq cr6, 0x825a34e4
	if ctx.cr[6].eq {
	pc = 0x825A34E4; continue 'dispatch;
	}
	// 825A34DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A34E0: 48000008  b 0x825a34e8
	pc = 0x825A34E8; continue 'dispatch;
	// 825A34E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A34E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A34EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A34F0: 4BD257C9  bl 0x822c8cb8
	ctx.lr = 0x825A34F4;
	sub_822C8CB8(ctx, base);
	// 825A34F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A34F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A34FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3504: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A350C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3510 size=120
    let mut pc: u32 = 0x825A3510;
    'dispatch: loop {
        match pc {
            0x825A3510 => {
    //   block [0x825A3510..0x825A3588)
	// 825A3510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3514: 48C04C59  bl 0x831a816c
	ctx.lr = 0x825A3518;
	sub_831A8130(ctx, base);
	// 825A3518: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A351C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A3520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3524: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 825A3528: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A352C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A3530: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825A3534: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825A3538: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825A353C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 825A3540: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A3544: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 825A3548: 4BFE89C9  bl 0x8258bf10
	ctx.lr = 0x825A354C;
	sub_8258BF10(ctx, base);
	// 825A354C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A3550: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A3554: 488506AD  bl 0x82df3c00
	ctx.lr = 0x825A3558;
	sub_82DF3C00(ctx, base);
	// 825A3558: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A355C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3560: 419A000C  beq cr6, 0x825a356c
	if ctx.cr[6].eq {
	pc = 0x825A356C; continue 'dispatch;
	}
	// 825A3564: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3568: 48000008  b 0x825a3570
	pc = 0x825A3570; continue 'dispatch;
	// 825A356C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A3570: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A3574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3578: 4BD25741  bl 0x822c8cb8
	ctx.lr = 0x825A357C;
	sub_822C8CB8(ctx, base);
	// 825A357C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3584: 48C04C38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3588 size=112
    let mut pc: u32 = 0x825A3588;
    'dispatch: loop {
        match pc {
            0x825A3588 => {
    //   block [0x825A3588..0x825A35F8)
	// 825A3588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A358C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A359C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A35A0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 825A35A4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A35A8: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825A35AC: 90BF0010  stw r5, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 825A35B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A35B4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 825A35B8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A35BC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A35C0: 4884FB31  bl 0x82df30f0
	ctx.lr = 0x825A35C4;
	sub_82DF30F0(ctx, base);
	// 825A35C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A35C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A35CC: 419A000C  beq cr6, 0x825a35d8
	if ctx.cr[6].eq {
	pc = 0x825A35D8; continue 'dispatch;
	}
	// 825A35D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A35D4: 48000008  b 0x825a35dc
	pc = 0x825A35DC; continue 'dispatch;
	// 825A35D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A35DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A35E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A35E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A35E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A35EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A35F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A35F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A35F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A35F8 size=92
    let mut pc: u32 = 0x825A35F8;
    'dispatch: loop {
        match pc {
            0x825A35F8 => {
    //   block [0x825A35F8..0x825A3654)
	// 825A35F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A35FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A360C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A3610: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A3614: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A3618: 396BAC24  addi r11, r11, -0x53dc
	ctx.r[11].s64 = ctx.r[11].s64 + -21468;
	// 825A361C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3620: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3624: 4884FE05  bl 0x82df3428
	ctx.lr = 0x825A3628;
	sub_82DF3428(ctx, base);
	// 825A3628: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A362C: 4182000C  beq 0x825a3638
	if ctx.cr[0].eq {
	pc = 0x825A3638; continue 'dispatch;
	}
	// 825A3630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3634: 4BD1CC35  bl 0x822c0268
	ctx.lr = 0x825A3638;
	sub_822C0268(ctx, base);
	// 825A3638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A363C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3648: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A364C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3658 size=72
    let mut pc: u32 = 0x825A3658;
    'dispatch: loop {
        match pc {
            0x825A3658 => {
    //   block [0x825A3658..0x825A36A0)
	// 825A3658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A366C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A3670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A3674: 396BAC24  addi r11, r11, -0x53dc
	ctx.r[11].s64 = ctx.r[11].s64 + -21468;
	// 825A3678: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A367C: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 825A3680: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A3684: 4885057D  bl 0x82df3c00
	ctx.lr = 0x825A3688;
	sub_82DF3C00(ctx, base);
	// 825A3688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A368C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A3690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A369C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A36A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A36A0 size=20
    let mut pc: u32 = 0x825A36A0;
    'dispatch: loop {
        match pc {
            0x825A36A0 => {
    //   block [0x825A36A0..0x825A36B4)
	// 825A36A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A36A4: 38830014  addi r4, r3, 0x14
	ctx.r[4].s64 = ctx.r[3].s64 + 20;
	// 825A36A8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A36AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A36B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A36B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A36B8 size=100
    let mut pc: u32 = 0x825A36B8;
    'dispatch: loop {
        match pc {
            0x825A36B8 => {
    //   block [0x825A36B8..0x825A371C)
	// 825A36B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A36BC: 48C04AAD  bl 0x831a8168
	ctx.lr = 0x825A36C0;
	sub_831A8130(ctx, base);
	// 825A36C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A36C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A36C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A36CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A36D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A36D4: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A36D8: 48850331  bl 0x82df3a08
	ctx.lr = 0x825A36DC;
	sub_82DF3A08(ctx, base);
	// 825A36DC: 3BBE0038  addi r29, r30, 0x38
	ctx.r[29].s64 = ctx.r[30].s64 + 56;
	// 825A36E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A36E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A36E8: 4884FC21  bl 0x82df3308
	ctx.lr = 0x825A36EC;
	sub_82DF3308(ctx, base);
	// 825A36EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A36F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A36F4: 4884FD35  bl 0x82df3428
	ctx.lr = 0x825A36F8;
	sub_82DF3428(ctx, base);
	// 825A36F8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A36FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3700: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A3704: 40820008  bne 0x825a370c
	if !ctx.cr[0].eq {
	pc = 0x825A370C; continue 'dispatch;
	}
	// 825A3708: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A370C: 488504F5  bl 0x82df3c00
	ctx.lr = 0x825A3710;
	sub_82DF3C00(ctx, base);
	// 825A3710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A3718: 48C04AA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3720 size=20
    let mut pc: u32 = 0x825A3720;
    'dispatch: loop {
        match pc {
            0x825A3720 => {
    //   block [0x825A3720..0x825A3734)
	// 825A3720: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3724: 38830041  addi r4, r3, 0x41
	ctx.r[4].s64 = ctx.r[3].s64 + 65;
	// 825A3728: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A372C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3730: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3738 size=20
    let mut pc: u32 = 0x825A3738;
    'dispatch: loop {
        match pc {
            0x825A3738 => {
    //   block [0x825A3738..0x825A374C)
	// 825A3738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A373C: 38830054  addi r4, r3, 0x54
	ctx.r[4].s64 = ctx.r[3].s64 + 84;
	// 825A3740: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3748: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3750 size=100
    let mut pc: u32 = 0x825A3750;
    'dispatch: loop {
        match pc {
            0x825A3750 => {
    //   block [0x825A3750..0x825A37B4)
	// 825A3750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3754: 48C04A15  bl 0x831a8168
	ctx.lr = 0x825A3758;
	sub_831A8130(ctx, base);
	// 825A3758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A375C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A3760: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A3764: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3768: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A376C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A3770: 48850299  bl 0x82df3a08
	ctx.lr = 0x825A3774;
	sub_82DF3A08(ctx, base);
	// 825A3774: 3BBE0048  addi r29, r30, 0x48
	ctx.r[29].s64 = ctx.r[30].s64 + 72;
	// 825A3778: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A377C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A3780: 4884FB89  bl 0x82df3308
	ctx.lr = 0x825A3784;
	sub_82DF3308(ctx, base);
	// 825A3784: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A3788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A378C: 4884FC9D  bl 0x82df3428
	ctx.lr = 0x825A3790;
	sub_82DF3428(ctx, base);
	// 825A3790: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3798: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A379C: 40820008  bne 0x825a37a4
	if !ctx.cr[0].eq {
	pc = 0x825A37A4; continue 'dispatch;
	}
	// 825A37A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A37A4: 4885045D  bl 0x82df3c00
	ctx.lr = 0x825A37A8;
	sub_82DF3C00(ctx, base);
	// 825A37A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A37AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A37B0: 48C04A08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A37B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A37B8 size=100
    let mut pc: u32 = 0x825A37B8;
    'dispatch: loop {
        match pc {
            0x825A37B8 => {
    //   block [0x825A37B8..0x825A381C)
	// 825A37B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A37BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A37C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A37C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A37C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A37CC: F8C10098  std r6, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[6].u64 ) };
	// 825A37D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A37D4: 83C10098  lwz r30, 0x98(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 825A37D8: 80C100C4  lwz r6, 0xc4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 825A37DC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A37E0: F90100A8  std r8, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[8].u64 ) };
	// 825A37E4: 810100A8  lwz r8, 0xa8(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 825A37E8: 790807E6  rldicr r8, r8, 0x20, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A37EC: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 825A37F0: 7BC607E6  rldicr r6, r30, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[30].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A37F4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A37F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A37FC: 4E800421  bctrl
	ctx.lr = 0x825A3800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A380C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3810: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3820 size=104
    let mut pc: u32 = 0x825A3820;
    'dispatch: loop {
        match pc {
            0x825A3820 => {
    //   block [0x825A3820..0x825A3888)
	// 825A3820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A382C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3830: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A3838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A383C: 419A000C  beq cr6, 0x825a3848
	if ctx.cr[6].eq {
	pc = 0x825A3848; continue 'dispatch;
	}
	// 825A3840: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3844: 48000008  b 0x825a384c
	pc = 0x825A384C; continue 'dispatch;
	// 825A3848: 89640040  lbz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A384C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A3850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3854: 41820010  beq 0x825a3864
	if ctx.cr[0].eq {
	pc = 0x825A3864; continue 'dispatch;
	}
	// 825A3858: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A385C: 388B9FFC  addi r4, r11, -0x6004
	ctx.r[4].s64 = ctx.r[11].s64 + -24580;
	// 825A3860: 4800000C  b 0x825a386c
	pc = 0x825A386C; continue 'dispatch;
	// 825A3864: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A3868: 388B9FF4  addi r4, r11, -0x600c
	ctx.r[4].s64 = ctx.r[11].s64 + -24588;
	// 825A386C: 4885019D  bl 0x82df3a08
	ctx.lr = 0x825A3870;
	sub_82DF3A08(ctx, base);
	// 825A3870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A3878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A387C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3888 size=148
    let mut pc: u32 = 0x825A3888;
    'dispatch: loop {
        match pc {
            0x825A3888 => {
    //   block [0x825A3888..0x825A391C)
	// 825A3888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A388C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A389C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A38A0: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A38A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A38A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A38AC: 38CB7EC4  addi r6, r11, 0x7ec4
	ctx.r[6].s64 = ctx.r[11].s64 + 32452;
	// 825A38B0: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A38B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A38B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A38BC: 48C0668D  bl 0x831a9f48
	ctx.lr = 0x825A38C0;
	sub_831A9F48(ctx, base);
	// 825A38C0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A38C4: 41820040  beq 0x825a3904
	if ctx.cr[0].eq {
	pc = 0x825A3904; continue 'dispatch;
	}
	// 825A38C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A38CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A38D0: 419A000C  beq cr6, 0x825a38dc
	if ctx.cr[6].eq {
	pc = 0x825A38DC; continue 'dispatch;
	}
	// 825A38D4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A38D8: 48000008  b 0x825a38e0
	pc = 0x825A38E0; continue 'dispatch;
	// 825A38DC: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A38E0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A38E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A38E8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 825A38EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A38F0: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A38F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A38F8: 4E800421  bctrl
	ctx.lr = 0x825A38FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A38FC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3900: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 825A3904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A390C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3910: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3920 size=20
    let mut pc: u32 = 0x825A3920;
    'dispatch: loop {
        match pc {
            0x825A3920 => {
    //   block [0x825A3920..0x825A3934)
	// 825A3920: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3924: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3928: 419A000C  beq cr6, 0x825a3934
	if ctx.cr[6].eq {
		sub_825A3934(ctx, base);
		return;
	}
	// 825A392C: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3930: 48000008  b 0x825a3938
	sub_825A3934(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3934 size=12
    let mut pc: u32 = 0x825A3934;
    'dispatch: loop {
        match pc {
            0x825A3934 => {
    //   block [0x825A3934..0x825A3940)
	// 825A3934: 89630040  lbz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3938: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 825A393C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3940 size=128
    let mut pc: u32 = 0x825A3940;
    'dispatch: loop {
        match pc {
            0x825A3940 => {
    //   block [0x825A3940..0x825A39C0)
	// 825A3940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A394C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3954: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A3958: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A395C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3960: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3964: 419A000C  beq cr6, 0x825a3970
	if ctx.cr[6].eq {
	pc = 0x825A3970; continue 'dispatch;
	}
	// 825A3968: 894A0000  lbz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A396C: 48000008  b 0x825a3974
	pc = 0x825A3974; continue 'dispatch;
	// 825A3970: 894B0040  lbz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3974: 994B0040  stb r10, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 825A3978: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 825A397C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3980: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 825A3984: 4885027D  bl 0x82df3c00
	ctx.lr = 0x825A3988;
	sub_82DF3C00(ctx, base);
	// 825A3988: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A398C: 4884F825  bl 0x82df31b0
	ctx.lr = 0x825A3990;
	sub_82DF31B0(ctx, base);
	// 825A3990: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A3994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3998: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A399C: 48036485  bl 0x825d9e20
	ctx.lr = 0x825A39A0;
	sub_825D9E20(ctx, base);
	// 825A39A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A39A4: 4884FA85  bl 0x82df3428
	ctx.lr = 0x825A39A8;
	sub_82DF3428(ctx, base);
	// 825A39A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A39AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A39B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A39B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A39B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A39BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A39C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A39C0 size=20
    let mut pc: u32 = 0x825A39C0;
    'dispatch: loop {
        match pc {
            0x825A39C0 => {
    //   block [0x825A39C0..0x825A39D4)
	// 825A39C0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A39C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A39C8: 419A000C  beq cr6, 0x825a39d4
	if ctx.cr[6].eq {
		sub_825A39D4(ctx, base);
		return;
	}
	// 825A39CC: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A39D0: 48000008  b 0x825a39d8
	sub_825A39D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A39D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A39D4 size=12
    let mut pc: u32 = 0x825A39D4;
    'dispatch: loop {
        match pc {
            0x825A39D4 => {
    //   block [0x825A39D4..0x825A39E0)
	// 825A39D4: 89630040  lbz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A39D8: 99630041  stb r11, 0x41(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(65 as u32), ctx.r[11].u8 ) };
	// 825A39DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A39E0 size=148
    let mut pc: u32 = 0x825A39E0;
    'dispatch: loop {
        match pc {
            0x825A39E0 => {
    //   block [0x825A39E0..0x825A3A74)
	// 825A39E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A39E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A39E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A39EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A39F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A39F4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A39F8: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A39FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A3A00: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A3A04: 38CB7EEC  addi r6, r11, 0x7eec
	ctx.r[6].s64 = ctx.r[11].s64 + 32492;
	// 825A3A08: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3A10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A3A14: 48C06535  bl 0x831a9f48
	ctx.lr = 0x825A3A18;
	sub_831A9F48(ctx, base);
	// 825A3A18: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A3A1C: 41820040  beq 0x825a3a5c
	if ctx.cr[0].eq {
	pc = 0x825A3A5C; continue 'dispatch;
	}
	// 825A3A20: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3A24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3A28: 419A000C  beq cr6, 0x825a3a34
	if ctx.cr[6].eq {
	pc = 0x825A3A34; continue 'dispatch;
	}
	// 825A3A2C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A30: 48000008  b 0x825a3a38
	pc = 0x825A3A38; continue 'dispatch;
	// 825A3A34: C01F0050  lfs f0, 0x50(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A38: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3A3C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A3A40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A3A44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3A48: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3A4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3A50: 4E800421  bctrl
	ctx.lr = 0x825A3A54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3A54: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3A58: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 825A3A5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3A68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3A78 size=20
    let mut pc: u32 = 0x825A3A78;
    'dispatch: loop {
        match pc {
            0x825A3A78 => {
    //   block [0x825A3A78..0x825A3A8C)
	// 825A3A78: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3A7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3A80: 419A000C  beq cr6, 0x825a3a8c
	if ctx.cr[6].eq {
		sub_825A3A8C(ctx, base);
		return;
	}
	// 825A3A84: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A88: 48000008  b 0x825a3a90
	sub_825A3A8C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3A8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3A8C size=12
    let mut pc: u32 = 0x825A3A8C;
    'dispatch: loop {
        match pc {
            0x825A3A8C => {
    //   block [0x825A3A8C..0x825A3A98)
	// 825A3A8C: C0030050  lfs f0, 0x50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3A90: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A3A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A3A98 size=128
    let mut pc: u32 = 0x825A3A98;
    'dispatch: loop {
        match pc {
            0x825A3A98 => {
    //   block [0x825A3A98..0x825A3B18)
	// 825A3A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3AA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3AA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3AA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3AAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A3AB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3AB4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3AB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3ABC: 419A000C  beq cr6, 0x825a3ac8
	if ctx.cr[6].eq {
	pc = 0x825A3AC8; continue 'dispatch;
	}
	// 825A3AC0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3AC4: 48000008  b 0x825a3acc
	pc = 0x825A3ACC; continue 'dispatch;
	// 825A3AC8: C00B0050  lfs f0, 0x50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3ACC: D00B0050  stfs f0, 0x50(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A3AD0: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 825A3AD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3AD8: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 825A3ADC: 48850125  bl 0x82df3c00
	ctx.lr = 0x825A3AE0;
	sub_82DF3C00(ctx, base);
	// 825A3AE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3AE4: 4884F6CD  bl 0x82df31b0
	ctx.lr = 0x825A3AE8;
	sub_82DF31B0(ctx, base);
	// 825A3AE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A3AEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3AF0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A3AF4: 48036335  bl 0x825d9e28
	ctx.lr = 0x825A3AF8;
	sub_825D9E28(ctx, base);
	// 825A3AF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3AFC: 4884F92D  bl 0x82df3428
	ctx.lr = 0x825A3B00;
	sub_82DF3428(ctx, base);
	// 825A3B00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3B0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3B10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3B18 size=20
    let mut pc: u32 = 0x825A3B18;
    'dispatch: loop {
        match pc {
            0x825A3B18 => {
    //   block [0x825A3B18..0x825A3B2C)
	// 825A3B18: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3B1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3B20: 419A000C  beq cr6, 0x825a3b2c
	if ctx.cr[6].eq {
		sub_825A3B2C(ctx, base);
		return;
	}
	// 825A3B24: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B28: 48000008  b 0x825a3b30
	sub_825A3B2C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3B2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A3B2C size=12
    let mut pc: u32 = 0x825A3B2C;
    'dispatch: loop {
        match pc {
            0x825A3B2C => {
    //   block [0x825A3B2C..0x825A3B38)
	// 825A3B2C: C0030050  lfs f0, 0x50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A3B30: D0030054  stfs f0, 0x54(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825A3B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3B38 size=148
    let mut pc: u32 = 0x825A3B38;
    'dispatch: loop {
        match pc {
            0x825A3B38 => {
    //   block [0x825A3B38..0x825A3BCC)
	// 825A3B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3B40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3B44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3B4C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A3B50: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A3B54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A3B58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A3B5C: 38CB7F10  addi r6, r11, 0x7f10
	ctx.r[6].s64 = ctx.r[11].s64 + 32528;
	// 825A3B60: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3B68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A3B6C: 48C063DD  bl 0x831a9f48
	ctx.lr = 0x825A3B70;
	sub_831A9F48(ctx, base);
	// 825A3B70: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A3B74: 41820040  beq 0x825a3bb4
	if ctx.cr[0].eq {
	pc = 0x825A3BB4; continue 'dispatch;
	}
	// 825A3B78: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3B80: 419A000C  beq cr6, 0x825a3b8c
	if ctx.cr[6].eq {
	pc = 0x825A3B8C; continue 'dispatch;
	}
	// 825A3B84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3B88: 48000008  b 0x825a3b90
	pc = 0x825A3B90; continue 'dispatch;
	// 825A3B8C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3B90: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3B94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A3B98: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A3B9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3BA0: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3BA8: 4E800421  bctrl
	ctx.lr = 0x825A3BAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3BAC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3BB0: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 825A3BB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3BC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3BC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3BD0 size=20
    let mut pc: u32 = 0x825A3BD0;
    'dispatch: loop {
        match pc {
            0x825A3BD0 => {
    //   block [0x825A3BD0..0x825A3BE4)
	// 825A3BD0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3BD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3BD8: 419A000C  beq cr6, 0x825a3be4
	if ctx.cr[6].eq {
		sub_825A3BE4(ctx, base);
		return;
	}
	// 825A3BDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3BE0: 48000008  b 0x825a3be8
	sub_825A3BE4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3BE4 size=12
    let mut pc: u32 = 0x825A3BE4;
    'dispatch: loop {
        match pc {
            0x825A3BE4 => {
    //   block [0x825A3BE4..0x825A3BF0)
	// 825A3BE4: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3BE8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A3BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3BF0 size=128
    let mut pc: u32 = 0x825A3BF0;
    'dispatch: loop {
        match pc {
            0x825A3BF0 => {
    //   block [0x825A3BF0..0x825A3C70)
	// 825A3BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3BF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3BFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3C04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A3C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3C0C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3C10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3C14: 419A000C  beq cr6, 0x825a3c20
	if ctx.cr[6].eq {
	pc = 0x825A3C20; continue 'dispatch;
	}
	// 825A3C18: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3C1C: 48000008  b 0x825a3c24
	pc = 0x825A3C24; continue 'dispatch;
	// 825A3C20: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3C24: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A3C28: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 825A3C2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3C30: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 825A3C34: 4884FFCD  bl 0x82df3c00
	ctx.lr = 0x825A3C38;
	sub_82DF3C00(ctx, base);
	// 825A3C38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3C3C: 4884F575  bl 0x82df31b0
	ctx.lr = 0x825A3C40;
	sub_82DF31B0(ctx, base);
	// 825A3C40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A3C44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3C48: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A3C4C: 480361FD  bl 0x825d9e48
	ctx.lr = 0x825A3C50;
	sub_825D9E48(ctx, base);
	// 825A3C50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3C54: 4884F7D5  bl 0x82df3428
	ctx.lr = 0x825A3C58;
	sub_82DF3428(ctx, base);
	// 825A3C58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3C64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3C70 size=20
    let mut pc: u32 = 0x825A3C70;
    'dispatch: loop {
        match pc {
            0x825A3C70 => {
    //   block [0x825A3C70..0x825A3C84)
	// 825A3C70: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3C74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3C78: 419A000C  beq cr6, 0x825a3c84
	if ctx.cr[6].eq {
		sub_825A3C84(ctx, base);
		return;
	}
	// 825A3C7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3C80: 48000008  b 0x825a3c88
	sub_825A3C84(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A3C84 size=12
    let mut pc: u32 = 0x825A3C84;
    'dispatch: loop {
        match pc {
            0x825A3C84 => {
    //   block [0x825A3C84..0x825A3C90)
	// 825A3C84: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3C88: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A3C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3C90 size=148
    let mut pc: u32 = 0x825A3C90;
    'dispatch: loop {
        match pc {
            0x825A3C90 => {
    //   block [0x825A3C90..0x825A3D24)
	// 825A3C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3CA4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A3CA8: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A3CAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A3CB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A3CB4: 38CB7F34  addi r6, r11, 0x7f34
	ctx.r[6].s64 = ctx.r[11].s64 + 32564;
	// 825A3CB8: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3CC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A3CC4: 48C06285  bl 0x831a9f48
	ctx.lr = 0x825A3CC8;
	sub_831A9F48(ctx, base);
	// 825A3CC8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A3CCC: 41820040  beq 0x825a3d0c
	if ctx.cr[0].eq {
	pc = 0x825A3D0C; continue 'dispatch;
	}
	// 825A3CD0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3CD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3CD8: 419A000C  beq cr6, 0x825a3ce4
	if ctx.cr[6].eq {
	pc = 0x825A3CE4; continue 'dispatch;
	}
	// 825A3CDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3CE0: 48000008  b 0x825a3ce8
	pc = 0x825A3CE8; continue 'dispatch;
	// 825A3CE4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3CE8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3CEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A3CF0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A3CF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3CF8: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3CFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3D00: 4E800421  bctrl
	ctx.lr = 0x825A3D04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3D04: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3D08: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 825A3D0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3D18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3D1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3D28 size=128
    let mut pc: u32 = 0x825A3D28;
    'dispatch: loop {
        match pc {
            0x825A3D28 => {
    //   block [0x825A3D28..0x825A3DA8)
	// 825A3D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3D30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3D34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3D3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A3D40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3D44: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3D48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3D4C: 419A000C  beq cr6, 0x825a3d58
	if ctx.cr[6].eq {
	pc = 0x825A3D58; continue 'dispatch;
	}
	// 825A3D50: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3D54: 48000008  b 0x825a3d5c
	pc = 0x825A3D5C; continue 'dispatch;
	// 825A3D58: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3D5C: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A3D60: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 825A3D64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3D68: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 825A3D6C: 4884FE95  bl 0x82df3c00
	ctx.lr = 0x825A3D70;
	sub_82DF3C00(ctx, base);
	// 825A3D70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3D74: 4884F43D  bl 0x82df31b0
	ctx.lr = 0x825A3D78;
	sub_82DF31B0(ctx, base);
	// 825A3D78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A3D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3D80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A3D84: 480360CD  bl 0x825d9e50
	ctx.lr = 0x825A3D88;
	sub_825D9E50(ctx, base);
	// 825A3D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3D8C: 4884F69D  bl 0x82df3428
	ctx.lr = 0x825A3D90;
	sub_82DF3428(ctx, base);
	// 825A3D90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3D9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3DA8 size=148
    let mut pc: u32 = 0x825A3DA8;
    'dispatch: loop {
        match pc {
            0x825A3DA8 => {
    //   block [0x825A3DA8..0x825A3E3C)
	// 825A3DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3DB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3DB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3DBC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A3DC0: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A3DC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A3DC8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A3DCC: 38CB7F58  addi r6, r11, 0x7f58
	ctx.r[6].s64 = ctx.r[11].s64 + 32600;
	// 825A3DD0: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3DD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A3DDC: 48C0616D  bl 0x831a9f48
	ctx.lr = 0x825A3DE0;
	sub_831A9F48(ctx, base);
	// 825A3DE0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A3DE4: 41820040  beq 0x825a3e24
	if ctx.cr[0].eq {
	pc = 0x825A3E24; continue 'dispatch;
	}
	// 825A3DE8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3DEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A3DF0: 419A000C  beq cr6, 0x825a3dfc
	if ctx.cr[6].eq {
	pc = 0x825A3DFC; continue 'dispatch;
	}
	// 825A3DF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3DF8: 48000008  b 0x825a3e00
	pc = 0x825A3E00; continue 'dispatch;
	// 825A3DFC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3E00: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3E04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A3E08: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A3E0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3E10: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A3E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3E18: 4E800421  bctrl
	ctx.lr = 0x825A3E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3E1C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A3E20: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 825A3E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3E30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3E40 size=128
    let mut pc: u32 = 0x825A3E40;
    'dispatch: loop {
        match pc {
            0x825A3E40 => {
    //   block [0x825A3E40..0x825A3EC0)
	// 825A3E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A3E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A3E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A3E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3E54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A3E58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3E5C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A3E60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A3E64: 419A000C  beq cr6, 0x825a3e70
	if ctx.cr[6].eq {
	pc = 0x825A3E70; continue 'dispatch;
	}
	// 825A3E68: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3E6C: 48000008  b 0x825a3e74
	pc = 0x825A3E74; continue 'dispatch;
	// 825A3E70: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A3E74: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A3E78: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 825A3E7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3E80: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	// 825A3E84: 4884FD7D  bl 0x82df3c00
	ctx.lr = 0x825A3E88;
	sub_82DF3C00(ctx, base);
	// 825A3E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3E8C: 4884F325  bl 0x82df31b0
	ctx.lr = 0x825A3E90;
	sub_82DF31B0(ctx, base);
	// 825A3E90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A3E94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A3E98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A3E9C: 48035FC5  bl 0x825d9e60
	ctx.lr = 0x825A3EA0;
	sub_825D9E60(ctx, base);
	// 825A3EA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A3EA4: 4884F585  bl 0x82df3428
	ctx.lr = 0x825A3EA8;
	sub_82DF3428(ctx, base);
	// 825A3EA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A3EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A3EB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A3EB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A3EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3EC0 size=128
    let mut pc: u32 = 0x825A3EC0;
    'dispatch: loop {
        match pc {
            0x825A3EC0 => {
    //   block [0x825A3EC0..0x825A3F40)
	// 825A3EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3EC4: 48C042A9  bl 0x831a816c
	ctx.lr = 0x825A3EC8;
	sub_831A8130(ctx, base);
	// 825A3EC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3ECC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3ED0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A3ED4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3ED8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A3EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3EE0: 4E800421  bctrl
	ctx.lr = 0x825A3EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3EE4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A3EE8: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A3EEC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3EF0: 38CB7EC4  addi r6, r11, 0x7ec4
	ctx.r[6].s64 = ctx.r[11].s64 + 32452;
	// 825A3EF4: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3EF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3EFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A3F00: 48C06049  bl 0x831a9f48
	ctx.lr = 0x825A3F04;
	sub_831A9F48(ctx, base);
	// 825A3F04: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A3F08: 4182002C  beq 0x825a3f34
	if ctx.cr[0].eq {
	pc = 0x825A3F34; continue 'dispatch;
	}
	// 825A3F0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3F10: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825A3F14: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A3F18: 4BD24DA1  bl 0x822c8cb8
	ctx.lr = 0x825A3F1C;
	sub_822C8CB8(ctx, base);
	// 825A3F1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3F20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A3F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3F28: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3F30: 4E800421  bctrl
	ctx.lr = 0x825A3F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3F34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A3F38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3F3C: 48C04280  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3F40 size=128
    let mut pc: u32 = 0x825A3F40;
    'dispatch: loop {
        match pc {
            0x825A3F40 => {
    //   block [0x825A3F40..0x825A3FC0)
	// 825A3F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3F44: 48C04229  bl 0x831a816c
	ctx.lr = 0x825A3F48;
	sub_831A8130(ctx, base);
	// 825A3F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3F4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3F50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A3F54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3F58: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A3F5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3F60: 4E800421  bctrl
	ctx.lr = 0x825A3F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3F64: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A3F68: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A3F6C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3F70: 38CB7EEC  addi r6, r11, 0x7eec
	ctx.r[6].s64 = ctx.r[11].s64 + 32492;
	// 825A3F74: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3F78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3F7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A3F80: 48C05FC9  bl 0x831a9f48
	ctx.lr = 0x825A3F84;
	sub_831A9F48(ctx, base);
	// 825A3F84: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A3F88: 4182002C  beq 0x825a3fb4
	if ctx.cr[0].eq {
	pc = 0x825A3FB4; continue 'dispatch;
	}
	// 825A3F8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A3F90: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A3F94: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A3F98: 4BD24D21  bl 0x822c8cb8
	ctx.lr = 0x825A3F9C;
	sub_822C8CB8(ctx, base);
	// 825A3F9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3FA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A3FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A3FA8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A3FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3FB0: 4E800421  bctrl
	ctx.lr = 0x825A3FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3FB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A3FB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A3FBC: 48C04200  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A3FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A3FC0 size=128
    let mut pc: u32 = 0x825A3FC0;
    'dispatch: loop {
        match pc {
            0x825A3FC0 => {
    //   block [0x825A3FC0..0x825A4040)
	// 825A3FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A3FC4: 48C041A9  bl 0x831a816c
	ctx.lr = 0x825A3FC8;
	sub_831A8130(ctx, base);
	// 825A3FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A3FCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A3FD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A3FD4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3FD8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A3FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A3FE0: 4E800421  bctrl
	ctx.lr = 0x825A3FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A3FE4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A3FE8: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A3FEC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A3FF0: 38CB7F10  addi r6, r11, 0x7f10
	ctx.r[6].s64 = ctx.r[11].s64 + 32528;
	// 825A3FF4: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A3FF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A3FFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A4000: 48C05F49  bl 0x831a9f48
	ctx.lr = 0x825A4004;
	sub_831A9F48(ctx, base);
	// 825A4004: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A4008: 4182002C  beq 0x825a4034
	if ctx.cr[0].eq {
	pc = 0x825A4034; continue 'dispatch;
	}
	// 825A400C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A4010: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A4014: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A4018: 4BD24CA1  bl 0x822c8cb8
	ctx.lr = 0x825A401C;
	sub_822C8CB8(ctx, base);
	// 825A401C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4020: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A4024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4028: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A402C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A4030: 4E800421  bctrl
	ctx.lr = 0x825A4034;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A4034: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A4038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A403C: 48C04180  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4040 size=128
    let mut pc: u32 = 0x825A4040;
    'dispatch: loop {
        match pc {
            0x825A4040 => {
    //   block [0x825A4040..0x825A40C0)
	// 825A4040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4044: 48C04129  bl 0x831a816c
	ctx.lr = 0x825A4048;
	sub_831A8130(ctx, base);
	// 825A4048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A404C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4050: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A4054: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4058: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A405C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A4060: 4E800421  bctrl
	ctx.lr = 0x825A4064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A4064: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A4068: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A406C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4070: 38CB7F34  addi r6, r11, 0x7f34
	ctx.r[6].s64 = ctx.r[11].s64 + 32564;
	// 825A4074: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A4078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A407C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A4080: 48C05EC9  bl 0x831a9f48
	ctx.lr = 0x825A4084;
	sub_831A9F48(ctx, base);
	// 825A4084: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A4088: 4182002C  beq 0x825a40b4
	if ctx.cr[0].eq {
	pc = 0x825A40B4; continue 'dispatch;
	}
	// 825A408C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A4090: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A4094: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A4098: 4BD24C21  bl 0x822c8cb8
	ctx.lr = 0x825A409C;
	sub_822C8CB8(ctx, base);
	// 825A409C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A40A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A40A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A40A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A40AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A40B0: 4E800421  bctrl
	ctx.lr = 0x825A40B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A40B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A40B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A40BC: 48C04100  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A40C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A40C0 size=128
    let mut pc: u32 = 0x825A40C0;
    'dispatch: loop {
        match pc {
            0x825A40C0 => {
    //   block [0x825A40C0..0x825A4140)
	// 825A40C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A40C4: 48C040A9  bl 0x831a816c
	ctx.lr = 0x825A40C8;
	sub_831A8130(ctx, base);
	// 825A40C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A40CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A40D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A40D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A40D8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A40DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A40E0: 4E800421  bctrl
	ctx.lr = 0x825A40E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A40E4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A40E8: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A40EC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A40F0: 38CB7F58  addi r6, r11, 0x7f58
	ctx.r[6].s64 = ctx.r[11].s64 + 32600;
	// 825A40F4: 38AA7DDC  addi r5, r10, 0x7ddc
	ctx.r[5].s64 = ctx.r[10].s64 + 32220;
	// 825A40F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A40FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A4100: 48C05E49  bl 0x831a9f48
	ctx.lr = 0x825A4104;
	sub_831A9F48(ctx, base);
	// 825A4104: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A4108: 4182002C  beq 0x825a4134
	if ctx.cr[0].eq {
	pc = 0x825A4134; continue 'dispatch;
	}
	// 825A410C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A4110: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A4114: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A4118: 4BD24BA1  bl 0x822c8cb8
	ctx.lr = 0x825A411C;
	sub_822C8CB8(ctx, base);
	// 825A411C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4120: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A4124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4128: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A412C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A4130: 4E800421  bctrl
	ctx.lr = 0x825A4134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A4134: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A4138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A413C: 48C04080  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4140 size=96
    let mut pc: u32 = 0x825A4140;
    'dispatch: loop {
        match pc {
            0x825A4140 => {
    //   block [0x825A4140..0x825A41A0)
	// 825A4140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A414C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4154: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4158: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A415C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A4160: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A4164: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4168: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A416C: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4170: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 825A4174: 4BFE7D9D  bl 0x8258bf10
	ctx.lr = 0x825A4178;
	sub_8258BF10(ctx, base);
	// 825A4178: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A417C: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 825A4180: 4884FA81  bl 0x82df3c00
	ctx.lr = 0x825A4184;
	sub_82DF3C00(ctx, base);
	// 825A4184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4188: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A418C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4194: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A419C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A41A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A41A0 size=128
    let mut pc: u32 = 0x825A41A0;
    'dispatch: loop {
        match pc {
            0x825A41A0 => {
    //   block [0x825A41A0..0x825A4220)
	// 825A41A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A41A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A41A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A41AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A41B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A41B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A41B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A41BC: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 825A41C0: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825A41C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A41C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A41CC: C01E0004  lfs f0, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41D0: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 825A41D4: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41D8: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825A41DC: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41E0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825A41E4: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41E8: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825A41EC: C01E0014  lfs f0, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A41F0: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A41F4: 4BFE7D1D  bl 0x8258bf10
	ctx.lr = 0x825A41F8;
	sub_8258BF10(ctx, base);
	// 825A41F8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A41FC: 389E0038  addi r4, r30, 0x38
	ctx.r[4].s64 = ctx.r[30].s64 + 56;
	// 825A4200: 4884FA01  bl 0x82df3c00
	ctx.lr = 0x825A4204;
	sub_82DF3C00(ctx, base);
	// 825A4204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4208: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A420C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4214: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4218: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A421C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4220 size=128
    let mut pc: u32 = 0x825A4220;
    'dispatch: loop {
        match pc {
            0x825A4220 => {
    //   block [0x825A4220..0x825A42A0)
	// 825A4220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A422C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4234: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4238: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A423C: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 825A4240: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825A4244: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4248: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A424C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4250: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A4254: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A4258: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A425C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A4260: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A4264: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4268: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A426C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A4270: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A4274: 4BFE7C9D  bl 0x8258bf10
	ctx.lr = 0x825A4278;
	sub_8258BF10(ctx, base);
	// 825A4278: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 825A427C: 389E0038  addi r4, r30, 0x38
	ctx.r[4].s64 = ctx.r[30].s64 + 56;
	// 825A4280: 4884F981  bl 0x82df3c00
	ctx.lr = 0x825A4284;
	sub_82DF3C00(ctx, base);
	// 825A4284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A428C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4294: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A429C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A42A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A42A0 size=104
    let mut pc: u32 = 0x825A42A0;
    'dispatch: loop {
        match pc {
            0x825A42A0 => {
    //   block [0x825A42A0..0x825A4308)
	// 825A42A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A42A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A42A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A42AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A42B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A42B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A42B8: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A42BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A42C0: 419A0008  beq cr6, 0x825a42c8
	if ctx.cr[6].eq {
	pc = 0x825A42C8; continue 'dispatch;
	}
	// 825A42C4: 4BD1C5CD  bl 0x822c0890
	ctx.lr = 0x825A42C8;
	sub_822C0890(ctx, base);
	// 825A42C8: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 825A42CC: 387E0038  addi r3, r30, 0x38
	ctx.r[3].s64 = ctx.r[30].s64 + 56;
	// 825A42D0: 4884F159  bl 0x82df3428
	ctx.lr = 0x825A42D4;
	sub_82DF3428(ctx, base);
	// 825A42D4: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 825A42D8: 4BD249E1  bl 0x822c8cb8
	ctx.lr = 0x825A42DC;
	sub_822C8CB8(ctx, base);
	// 825A42DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A42E0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A42E4: 396BAC24  addi r11, r11, -0x53dc
	ctx.r[11].s64 = ctx.r[11].s64 + -21468;
	// 825A42E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A42EC: 4884F13D  bl 0x82df3428
	ctx.lr = 0x825A42F0;
	sub_82DF3428(ctx, base);
	// 825A42F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A42F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A42F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A42FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4308 size=104
    let mut pc: u32 = 0x825A4308;
    'dispatch: loop {
        match pc {
            0x825A4308 => {
    //   block [0x825A4308..0x825A4370)
	// 825A4308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A430C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A4314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A431C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4320: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 825A4324: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A4328: 419A0008  beq cr6, 0x825a4330
	if ctx.cr[6].eq {
	pc = 0x825A4330; continue 'dispatch;
	}
	// 825A432C: 4BD1C565  bl 0x822c0890
	ctx.lr = 0x825A4330;
	sub_822C0890(ctx, base);
	// 825A4330: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 825A4334: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 825A4338: 4884F0F1  bl 0x82df3428
	ctx.lr = 0x825A433C;
	sub_82DF3428(ctx, base);
	// 825A433C: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 825A4340: 4BD24979  bl 0x822c8cb8
	ctx.lr = 0x825A4344;
	sub_822C8CB8(ctx, base);
	// 825A4344: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4348: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A434C: 396BAC24  addi r11, r11, -0x53dc
	ctx.r[11].s64 = ctx.r[11].s64 + -21468;
	// 825A4350: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4354: 4884F0D5  bl 0x82df3428
	ctx.lr = 0x825A4358;
	sub_82DF3428(ctx, base);
	// 825A4358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A435C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A436C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4370 size=124
    let mut pc: u32 = 0x825A4370;
    'dispatch: loop {
        match pc {
            0x825A4370 => {
    //   block [0x825A4370..0x825A43EC)
	// 825A4370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A437C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4388: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A438C: 4BFFF2CD  bl 0x825a3658
	ctx.lr = 0x825A4390;
	sub_825A3658(ctx, base);
	// 825A4390: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4394: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A4398: 396BAC6C  addi r11, r11, -0x5394
	ctx.r[11].s64 = ctx.r[11].s64 + -21396;
	// 825A439C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A43A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A43A4: 4BFFFD9D  bl 0x825a4140
	ctx.lr = 0x825A43A8;
	sub_825A4140(ctx, base);
	// 825A43A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A43AC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 825A43B0: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 825A43B4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A43B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A43BC: 419A000C  beq cr6, 0x825a43c8
	if ctx.cr[6].eq {
	pc = 0x825A43C8; continue 'dispatch;
	}
	// 825A43C0: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A43C4: 48000008  b 0x825a43cc
	pc = 0x825A43CC; continue 'dispatch;
	// 825A43C8: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A43CC: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 825A43D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A43D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A43D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A43DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A43E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A43E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A43E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A43F0 size=124
    let mut pc: u32 = 0x825A43F0;
    'dispatch: loop {
        match pc {
            0x825A43F0 => {
    //   block [0x825A43F0..0x825A446C)
	// 825A43F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A43F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A43F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A43FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4408: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A440C: 4BFFF24D  bl 0x825a3658
	ctx.lr = 0x825A4410;
	sub_825A3658(ctx, base);
	// 825A4410: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4414: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A4418: 396BACB4  addi r11, r11, -0x534c
	ctx.r[11].s64 = ctx.r[11].s64 + -21324;
	// 825A441C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A4420: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4424: 4BFFFD7D  bl 0x825a41a0
	ctx.lr = 0x825A4428;
	sub_825A41A0(ctx, base);
	// 825A4428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A442C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825A4430: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A4434: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A443C: 419A000C  beq cr6, 0x825a4448
	if ctx.cr[6].eq {
	pc = 0x825A4448; continue 'dispatch;
	}
	// 825A4440: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4444: 48000008  b 0x825a444c
	pc = 0x825A444C; continue 'dispatch;
	// 825A4448: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A444C: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A4450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A4458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A445C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4460: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4470 size=60
    let mut pc: u32 = 0x825A4470;
    'dispatch: loop {
        match pc {
            0x825A4470 => {
    //   block [0x825A4470..0x825A44AC)
	// 825A4470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A447C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4484: 4BFFFF6D  bl 0x825a43f0
	ctx.lr = 0x825A4488;
	sub_825A43F0(ctx, base);
	// 825A4488: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A448C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4490: 396BACFC  addi r11, r11, -0x5304
	ctx.r[11].s64 = ctx.r[11].s64 + -21252;
	// 825A4494: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A449C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A44A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A44A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A44A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A44B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A44B0 size=176
    let mut pc: u32 = 0x825A44B0;
    'dispatch: loop {
        match pc {
            0x825A44B0 => {
    //   block [0x825A44B0..0x825A4560)
	// 825A44B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A44B4: 48C03CB5  bl 0x831a8168
	ctx.lr = 0x825A44B8;
	sub_831A8130(ctx, base);
	// 825A44B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A44BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A44C0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A44C4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A44C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A44CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A44D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A44D4: 388BAD40  addi r4, r11, -0x52c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21184;
	// 825A44D8: 38A000CD  li r5, 0xcd
	ctx.r[5].s64 = 205;
	// 825A44DC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A44E0: 4BD1BEF9  bl 0x822c03d8
	ctx.lr = 0x825A44E4;
	sub_822C03D8(ctx, base);
	// 825A44E4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A44E8: 41820034  beq 0x825a451c
	if ctx.cr[0].eq {
	pc = 0x825A451C; continue 'dispatch;
	}
	// 825A44EC: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A44F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A44F4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A44F8: 4884F709  bl 0x82df3c00
	ctx.lr = 0x825A44FC;
	sub_82DF3C00(ctx, base);
	// 825A44FC: 38BE0010  addi r5, r30, 0x10
	ctx.r[5].s64 = ctx.r[30].s64 + 16;
	// 825A4500: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4508: 4BFFFEE9  bl 0x825a43f0
	ctx.lr = 0x825A450C;
	sub_825A43F0(ctx, base);
	// 825A450C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4510: 396BACFC  addi r11, r11, -0x5304
	ctx.r[11].s64 = ctx.r[11].s64 + -21252;
	// 825A4514: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4518: 48000008  b 0x825a4520
	pc = 0x825A4520; continue 'dispatch;
	// 825A451C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A4520: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A4524: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825A4528: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A452C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4530: 4BFFE471  bl 0x825a29a0
	ctx.lr = 0x825A4534;
	sub_825A29A0(ctx, base);
	// 825A4534: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A4538: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A453C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4540: 4BD1BAC1  bl 0x822c0000
	ctx.lr = 0x825A4544;
	sub_822C0000(ctx, base);
	// 825A4544: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A4548: 4182000C  beq 0x825a4554
	if ctx.cr[0].eq {
	pc = 0x825A4554; continue 'dispatch;
	}
	// 825A454C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A4550: 4884EED9  bl 0x82df3428
	ctx.lr = 0x825A4554;
	sub_82DF3428(ctx, base);
	// 825A4554: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A4558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A455C: 48C03C5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4560 size=124
    let mut pc: u32 = 0x825A4560;
    'dispatch: loop {
        match pc {
            0x825A4560 => {
    //   block [0x825A4560..0x825A45DC)
	// 825A4560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A456C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4578: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A457C: 4BFFF0DD  bl 0x825a3658
	ctx.lr = 0x825A4580;
	sub_825A3658(ctx, base);
	// 825A4580: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4584: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A4588: 396BAD8C  addi r11, r11, -0x5274
	ctx.r[11].s64 = ctx.r[11].s64 + -21108;
	// 825A458C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A4590: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4594: 4BFFFC8D  bl 0x825a4220
	ctx.lr = 0x825A4598;
	sub_825A4220(ctx, base);
	// 825A4598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A459C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825A45A0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A45A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A45A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A45AC: 419A000C  beq cr6, 0x825a45b8
	if ctx.cr[6].eq {
	pc = 0x825A45B8; continue 'dispatch;
	}
	// 825A45B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A45B4: 48000008  b 0x825a45bc
	pc = 0x825A45BC; continue 'dispatch;
	// 825A45B8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A45BC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A45C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A45C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A45C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A45CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A45D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A45D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A45D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A45E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A45E0 size=60
    let mut pc: u32 = 0x825A45E0;
    'dispatch: loop {
        match pc {
            0x825A45E0 => {
    //   block [0x825A45E0..0x825A461C)
	// 825A45E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A45E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A45E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A45EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A45F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A45F4: 4BFFFF6D  bl 0x825a4560
	ctx.lr = 0x825A45F8;
	sub_825A4560(ctx, base);
	// 825A45F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A45FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4600: 396BADD4  addi r11, r11, -0x522c
	ctx.r[11].s64 = ctx.r[11].s64 + -21036;
	// 825A4604: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A460C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4620 size=176
    let mut pc: u32 = 0x825A4620;
    'dispatch: loop {
        match pc {
            0x825A4620 => {
    //   block [0x825A4620..0x825A46D0)
	// 825A4620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4624: 48C03B45  bl 0x831a8168
	ctx.lr = 0x825A4628;
	sub_831A8130(ctx, base);
	// 825A4628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A462C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A4630: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4634: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A4638: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A463C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4640: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A4644: 388BAD40  addi r4, r11, -0x52c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21184;
	// 825A4648: 38A000CD  li r5, 0xcd
	ctx.r[5].s64 = 205;
	// 825A464C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A4650: 4BD1BD89  bl 0x822c03d8
	ctx.lr = 0x825A4654;
	sub_822C03D8(ctx, base);
	// 825A4654: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A4658: 41820034  beq 0x825a468c
	if ctx.cr[0].eq {
	pc = 0x825A468C; continue 'dispatch;
	}
	// 825A465C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A4660: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A4664: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A4668: 4884F599  bl 0x82df3c00
	ctx.lr = 0x825A466C;
	sub_82DF3C00(ctx, base);
	// 825A466C: 38BE0010  addi r5, r30, 0x10
	ctx.r[5].s64 = ctx.r[30].s64 + 16;
	// 825A4670: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4678: 4BFFFEE9  bl 0x825a4560
	ctx.lr = 0x825A467C;
	sub_825A4560(ctx, base);
	// 825A467C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4680: 396BADD4  addi r11, r11, -0x522c
	ctx.r[11].s64 = ctx.r[11].s64 + -21036;
	// 825A4684: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4688: 48000008  b 0x825a4690
	pc = 0x825A4690; continue 'dispatch;
	// 825A468C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A4690: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A4694: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825A4698: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A469C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A46A0: 4BFFE3C9  bl 0x825a2a68
	ctx.lr = 0x825A46A4;
	sub_825A2A68(ctx, base);
	// 825A46A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A46A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A46AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A46B0: 4BD1B951  bl 0x822c0000
	ctx.lr = 0x825A46B4;
	sub_822C0000(ctx, base);
	// 825A46B4: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A46B8: 4182000C  beq 0x825a46c4
	if ctx.cr[0].eq {
	pc = 0x825A46C4; continue 'dispatch;
	}
	// 825A46BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A46C0: 4884ED69  bl 0x82df3428
	ctx.lr = 0x825A46C4;
	sub_82DF3428(ctx, base);
	// 825A46C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A46C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A46CC: 48C03AEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A46D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A46D0 size=124
    let mut pc: u32 = 0x825A46D0;
    'dispatch: loop {
        match pc {
            0x825A46D0 => {
    //   block [0x825A46D0..0x825A474C)
	// 825A46D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A46D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A46D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A46DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A46E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A46E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A46E8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A46EC: 4BFFEF6D  bl 0x825a3658
	ctx.lr = 0x825A46F0;
	sub_825A3658(ctx, base);
	// 825A46F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A46F4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A46F8: 396BAE1C  addi r11, r11, -0x51e4
	ctx.r[11].s64 = ctx.r[11].s64 + -20964;
	// 825A46FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A4700: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4704: 4BFFFB1D  bl 0x825a4220
	ctx.lr = 0x825A4708;
	sub_825A4220(ctx, base);
	// 825A4708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A470C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825A4710: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A4714: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A471C: 419A000C  beq cr6, 0x825a4728
	if ctx.cr[6].eq {
	pc = 0x825A4728; continue 'dispatch;
	}
	// 825A4720: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4724: 48000008  b 0x825a472c
	pc = 0x825A472C; continue 'dispatch;
	// 825A4728: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A472C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A4730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4734: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A4738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A473C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4740: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4750 size=60
    let mut pc: u32 = 0x825A4750;
    'dispatch: loop {
        match pc {
            0x825A4750 => {
    //   block [0x825A4750..0x825A478C)
	// 825A4750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A475C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4760: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4764: 4BFFFF6D  bl 0x825a46d0
	ctx.lr = 0x825A4768;
	sub_825A46D0(ctx, base);
	// 825A4768: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A476C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4770: 396BAE64  addi r11, r11, -0x519c
	ctx.r[11].s64 = ctx.r[11].s64 + -20892;
	// 825A4774: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A477C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4790 size=176
    let mut pc: u32 = 0x825A4790;
    'dispatch: loop {
        match pc {
            0x825A4790 => {
    //   block [0x825A4790..0x825A4840)
	// 825A4790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4794: 48C039D5  bl 0x831a8168
	ctx.lr = 0x825A4798;
	sub_831A8130(ctx, base);
	// 825A4798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A479C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A47A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A47A4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A47A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A47AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A47B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A47B4: 388BAD40  addi r4, r11, -0x52c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21184;
	// 825A47B8: 38A000CD  li r5, 0xcd
	ctx.r[5].s64 = 205;
	// 825A47BC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A47C0: 4BD1BC19  bl 0x822c03d8
	ctx.lr = 0x825A47C4;
	sub_822C03D8(ctx, base);
	// 825A47C4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A47C8: 41820034  beq 0x825a47fc
	if ctx.cr[0].eq {
	pc = 0x825A47FC; continue 'dispatch;
	}
	// 825A47CC: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A47D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A47D4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A47D8: 4884F429  bl 0x82df3c00
	ctx.lr = 0x825A47DC;
	sub_82DF3C00(ctx, base);
	// 825A47DC: 38BE0010  addi r5, r30, 0x10
	ctx.r[5].s64 = ctx.r[30].s64 + 16;
	// 825A47E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A47E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A47E8: 4BFFFEE9  bl 0x825a46d0
	ctx.lr = 0x825A47EC;
	sub_825A46D0(ctx, base);
	// 825A47EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A47F0: 396BAE64  addi r11, r11, -0x519c
	ctx.r[11].s64 = ctx.r[11].s64 + -20892;
	// 825A47F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A47F8: 48000008  b 0x825a4800
	pc = 0x825A4800; continue 'dispatch;
	// 825A47FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A4800: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A4804: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825A4808: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A480C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4810: 4BFFE321  bl 0x825a2b30
	ctx.lr = 0x825A4814;
	sub_825A2B30(ctx, base);
	// 825A4814: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A4818: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A481C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4820: 4BD1B7E1  bl 0x822c0000
	ctx.lr = 0x825A4824;
	sub_822C0000(ctx, base);
	// 825A4824: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A4828: 4182000C  beq 0x825a4834
	if ctx.cr[0].eq {
	pc = 0x825A4834; continue 'dispatch;
	}
	// 825A482C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A4830: 4884EBF9  bl 0x82df3428
	ctx.lr = 0x825A4834;
	sub_82DF3428(ctx, base);
	// 825A4834: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A4838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A483C: 48C0397C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4840 size=124
    let mut pc: u32 = 0x825A4840;
    'dispatch: loop {
        match pc {
            0x825A4840 => {
    //   block [0x825A4840..0x825A48BC)
	// 825A4840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A484C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4858: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A485C: 4BFFEDFD  bl 0x825a3658
	ctx.lr = 0x825A4860;
	sub_825A3658(ctx, base);
	// 825A4860: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4864: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A4868: 396BAEAC  addi r11, r11, -0x5154
	ctx.r[11].s64 = ctx.r[11].s64 + -20820;
	// 825A486C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A4870: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4874: 4BFFF9AD  bl 0x825a4220
	ctx.lr = 0x825A4878;
	sub_825A4220(ctx, base);
	// 825A4878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A487C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825A4880: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A4884: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A488C: 419A000C  beq cr6, 0x825a4898
	if ctx.cr[6].eq {
	pc = 0x825A4898; continue 'dispatch;
	}
	// 825A4890: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4894: 48000008  b 0x825a489c
	pc = 0x825A489C; continue 'dispatch;
	// 825A4898: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A489C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A48A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A48A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A48A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A48AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A48B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A48B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A48B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A48C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A48C0 size=60
    let mut pc: u32 = 0x825A48C0;
    'dispatch: loop {
        match pc {
            0x825A48C0 => {
    //   block [0x825A48C0..0x825A48FC)
	// 825A48C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A48C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A48C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A48CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A48D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A48D4: 4BFFFF6D  bl 0x825a4840
	ctx.lr = 0x825A48D8;
	sub_825A4840(ctx, base);
	// 825A48D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A48DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A48E0: 396BAEF4  addi r11, r11, -0x510c
	ctx.r[11].s64 = ctx.r[11].s64 + -20748;
	// 825A48E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A48E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A48EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A48F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A48F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A48F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4900 size=176
    let mut pc: u32 = 0x825A4900;
    'dispatch: loop {
        match pc {
            0x825A4900 => {
    //   block [0x825A4900..0x825A49B0)
	// 825A4900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4904: 48C03865  bl 0x831a8168
	ctx.lr = 0x825A4908;
	sub_831A8130(ctx, base);
	// 825A4908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A490C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A4910: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4914: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A4918: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A491C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4920: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A4924: 388BAD40  addi r4, r11, -0x52c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21184;
	// 825A4928: 38A000CD  li r5, 0xcd
	ctx.r[5].s64 = 205;
	// 825A492C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A4930: 4BD1BAA9  bl 0x822c03d8
	ctx.lr = 0x825A4934;
	sub_822C03D8(ctx, base);
	// 825A4934: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A4938: 41820034  beq 0x825a496c
	if ctx.cr[0].eq {
	pc = 0x825A496C; continue 'dispatch;
	}
	// 825A493C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A4940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A4944: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A4948: 4884F2B9  bl 0x82df3c00
	ctx.lr = 0x825A494C;
	sub_82DF3C00(ctx, base);
	// 825A494C: 38BE0010  addi r5, r30, 0x10
	ctx.r[5].s64 = ctx.r[30].s64 + 16;
	// 825A4950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4958: 4BFFFEE9  bl 0x825a4840
	ctx.lr = 0x825A495C;
	sub_825A4840(ctx, base);
	// 825A495C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4960: 396BAEF4  addi r11, r11, -0x510c
	ctx.r[11].s64 = ctx.r[11].s64 + -20748;
	// 825A4964: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4968: 48000008  b 0x825a4970
	pc = 0x825A4970; continue 'dispatch;
	// 825A496C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A4970: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A4974: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825A4978: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A497C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4980: 4BFFDDC9  bl 0x825a2748
	ctx.lr = 0x825A4984;
	sub_825A2748(ctx, base);
	// 825A4984: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A4988: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A498C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4990: 4BD1B671  bl 0x822c0000
	ctx.lr = 0x825A4994;
	sub_822C0000(ctx, base);
	// 825A4994: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A4998: 4182000C  beq 0x825a49a4
	if ctx.cr[0].eq {
	pc = 0x825A49A4; continue 'dispatch;
	}
	// 825A499C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A49A0: 4884EA89  bl 0x82df3428
	ctx.lr = 0x825A49A4;
	sub_82DF3428(ctx, base);
	// 825A49A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A49A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A49AC: 48C0380C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A49B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A49B0 size=76
    let mut pc: u32 = 0x825A49B0;
    'dispatch: loop {
        match pc {
            0x825A49B0 => {
    //   block [0x825A49B0..0x825A49FC)
	// 825A49B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A49B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A49B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A49BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A49C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A49C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A49C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A49CC: 4BFFF8D5  bl 0x825a42a0
	ctx.lr = 0x825A49D0;
	sub_825A42A0(ctx, base);
	// 825A49D0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A49D4: 4182000C  beq 0x825a49e0
	if ctx.cr[0].eq {
	pc = 0x825A49E0; continue 'dispatch;
	}
	// 825A49D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A49DC: 4BD1B88D  bl 0x822c0268
	ctx.lr = 0x825A49E0;
	sub_822C0268(ctx, base);
	// 825A49E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A49E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A49E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A49EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A49F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A49F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A49F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4A00 size=60
    let mut pc: u32 = 0x825A4A00;
    'dispatch: loop {
        match pc {
            0x825A4A00 => {
    //   block [0x825A4A00..0x825A4A3C)
	// 825A4A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4A0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4A10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4A14: 4BFFF95D  bl 0x825a4370
	ctx.lr = 0x825A4A18;
	sub_825A4370(ctx, base);
	// 825A4A18: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4A20: 396BAF3C  addi r11, r11, -0x50c4
	ctx.r[11].s64 = ctx.r[11].s64 + -20676;
	// 825A4A24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A4A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4A34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4A40 size=76
    let mut pc: u32 = 0x825A4A40;
    'dispatch: loop {
        match pc {
            0x825A4A40 => {
    //   block [0x825A4A40..0x825A4A8C)
	// 825A4A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A4A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A4A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4A54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A4A58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4A5C: 4BFFF8AD  bl 0x825a4308
	ctx.lr = 0x825A4A60;
	sub_825A4308(ctx, base);
	// 825A4A60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A4A64: 4182000C  beq 0x825a4a70
	if ctx.cr[0].eq {
	pc = 0x825A4A70; continue 'dispatch;
	}
	// 825A4A68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4A6C: 4BD1B7FD  bl 0x822c0268
	ctx.lr = 0x825A4A70;
	sub_822C0268(ctx, base);
	// 825A4A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4A74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A4A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A4A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A4A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4A90 size=176
    let mut pc: u32 = 0x825A4A90;
    'dispatch: loop {
        match pc {
            0x825A4A90 => {
    //   block [0x825A4A90..0x825A4B40)
	// 825A4A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4A94: 48C036D5  bl 0x831a8168
	ctx.lr = 0x825A4A98;
	sub_831A8130(ctx, base);
	// 825A4A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4A9C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A4AA0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4AA4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A4AA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A4AAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A4AB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A4AB4: 388BAD40  addi r4, r11, -0x52c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21184;
	// 825A4AB8: 38A00116  li r5, 0x116
	ctx.r[5].s64 = 278;
	// 825A4ABC: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 825A4AC0: 4BD1B919  bl 0x822c03d8
	ctx.lr = 0x825A4AC4;
	sub_822C03D8(ctx, base);
	// 825A4AC4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A4AC8: 41820034  beq 0x825a4afc
	if ctx.cr[0].eq {
	pc = 0x825A4AFC; continue 'dispatch;
	}
	// 825A4ACC: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825A4AD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A4AD4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A4AD8: 4884F129  bl 0x82df3c00
	ctx.lr = 0x825A4ADC;
	sub_82DF3C00(ctx, base);
	// 825A4ADC: 38BE0010  addi r5, r30, 0x10
	ctx.r[5].s64 = ctx.r[30].s64 + 16;
	// 825A4AE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A4AE8: 4BFFF889  bl 0x825a4370
	ctx.lr = 0x825A4AEC;
	sub_825A4370(ctx, base);
	// 825A4AEC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A4AF0: 396BAF3C  addi r11, r11, -0x50c4
	ctx.r[11].s64 = ctx.r[11].s64 + -20676;
	// 825A4AF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A4AF8: 48000008  b 0x825a4b00
	pc = 0x825A4B00; continue 'dispatch;
	// 825A4AFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A4B00: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A4B04: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825A4B08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A4B0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B10: 4BFFDD01  bl 0x825a2810
	ctx.lr = 0x825A4B14;
	sub_825A2810(ctx, base);
	// 825A4B14: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A4B18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A4B1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A4B20: 4BD1B4E1  bl 0x822c0000
	ctx.lr = 0x825A4B24;
	sub_822C0000(ctx, base);
	// 825A4B24: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A4B28: 4182000C  beq 0x825a4b34
	if ctx.cr[0].eq {
	pc = 0x825A4B34; continue 'dispatch;
	}
	// 825A4B2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A4B30: 4884E8F9  bl 0x82df3428
	ctx.lr = 0x825A4B34;
	sub_82DF3428(ctx, base);
	// 825A4B34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A4B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A4B3C: 48C0367C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4B40 size=48
    let mut pc: u32 = 0x825A4B40;
    'dispatch: loop {
        match pc {
            0x825A4B40 => {
    //   block [0x825A4B40..0x825A4B70)
	// 825A4B40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4B44: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4B48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4B4C: 419A000C  beq cr6, 0x825a4b58
	if ctx.cr[6].eq {
	pc = 0x825A4B58; continue 'dispatch;
	}
	// 825A4B50: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4B54: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 825A4B58: 88840000  lbz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4B5C: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A4B60: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4B64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4B68: 988B0040  stb r4, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[4].u8 ) };
	// 825A4B6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4B70 size=16
    let mut pc: u32 = 0x825A4B70;
    'dispatch: loop {
        match pc {
            0x825A4B70 => {
    //   block [0x825A4B70..0x825A4B80)
	// 825A4B70: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A4B74: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A4B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4B7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4B80 size=8
    let mut pc: u32 = 0x825A4B80;
    'dispatch: loop {
        match pc {
            0x825A4B80 => {
    //   block [0x825A4B80..0x825A4B88)
	// 825A4B80: 4800CD48  b 0x825b18c8
	sub_825B18C8(ctx, base);
	return;
	// 825A4B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4B88 size=48
    let mut pc: u32 = 0x825A4B88;
    'dispatch: loop {
        match pc {
            0x825A4B88 => {
    //   block [0x825A4B88..0x825A4BB8)
	// 825A4B88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4B8C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4B90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4B94: 419A000C  beq cr6, 0x825a4ba0
	if ctx.cr[6].eq {
	pc = 0x825A4BA0; continue 'dispatch;
	}
	// 825A4B98: 892B0040  lbz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A4B9C: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 825A4BA0: 888B0040  lbz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A4BA4: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825A4BA8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4BAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4BB0: 988B0014  stb r4, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[4].u8 ) };
	// 825A4BB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4BB8 size=16
    let mut pc: u32 = 0x825A4BB8;
    'dispatch: loop {
        match pc {
            0x825A4BB8 => {
    //   block [0x825A4BB8..0x825A4BC8)
	// 825A4BB8: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A4BBC: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A4BC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4BC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4BC8 size=8
    let mut pc: u32 = 0x825A4BC8;
    'dispatch: loop {
        match pc {
            0x825A4BC8 => {
    //   block [0x825A4BC8..0x825A4BD0)
	// 825A4BC8: 4800CD00  b 0x825b18c8
	sub_825B18C8(ctx, base);
	return;
	// 825A4BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A4BD0 size=48
    let mut pc: u32 = 0x825A4BD0;
    'dispatch: loop {
        match pc {
            0x825A4BD0 => {
    //   block [0x825A4BD0..0x825A4C00)
	// 825A4BD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4BD4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4BD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4BDC: 419A000C  beq cr6, 0x825a4be8
	if ctx.cr[6].eq {
	pc = 0x825A4BE8; continue 'dispatch;
	}
	// 825A4BE0: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4BE4: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A4BE8: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A4BEC: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A4BF0: D02B0050  stfs f1, 0x50(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A4BF4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 825A4BF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4BFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4C00 size=16
    let mut pc: u32 = 0x825A4C00;
    'dispatch: loop {
        match pc {
            0x825A4C00 => {
    //   block [0x825A4C00..0x825A4C10)
	// 825A4C00: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A4C04: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A4C08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4C0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4C10 size=8
    let mut pc: u32 = 0x825A4C10;
    'dispatch: loop {
        match pc {
            0x825A4C10 => {
    //   block [0x825A4C10..0x825A4C18)
	// 825A4C10: 4BD65C40  b 0x8230a850
	sub_8230A850(ctx, base);
	return;
	// 825A4C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A4C18 size=48
    let mut pc: u32 = 0x825A4C18;
    'dispatch: loop {
        match pc {
            0x825A4C18 => {
    //   block [0x825A4C18..0x825A4C48)
	// 825A4C18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4C1C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4C20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4C24: 419A000C  beq cr6, 0x825a4c30
	if ctx.cr[6].eq {
	pc = 0x825A4C30; continue 'dispatch;
	}
	// 825A4C28: C00B0050  lfs f0, 0x50(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C2C: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825A4C30: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A4C34: C02B0050  lfs f1, 0x50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A4C38: D02B0014  stfs f1, 0x14(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 825A4C3C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 825A4C40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4C44: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4C48 size=16
    let mut pc: u32 = 0x825A4C48;
    'dispatch: loop {
        match pc {
            0x825A4C48 => {
    //   block [0x825A4C48..0x825A4C58)
	// 825A4C48: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A4C4C: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A4C50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4C54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4C58 size=8
    let mut pc: u32 = 0x825A4C58;
    'dispatch: loop {
        match pc {
            0x825A4C58 => {
    //   block [0x825A4C58..0x825A4C60)
	// 825A4C58: 4BD65BF8  b 0x8230a850
	sub_8230A850(ctx, base);
	return;
	// 825A4C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A4C60 size=112
    let mut pc: u32 = 0x825A4C60;
    'dispatch: loop {
        match pc {
            0x825A4C60 => {
    //   block [0x825A4C60..0x825A4CD0)
	// 825A4C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4C6C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4C70: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C74: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A4C78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4C7C: 419A000C  beq cr6, 0x825a4c88
	if ctx.cr[6].eq {
	pc = 0x825A4C88; continue 'dispatch;
	}
	// 825A4C80: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C84: 48000008  b 0x825a4c8c
	pc = 0x825A4C8C; continue 'dispatch;
	// 825A4C88: C0030050  lfs f0, 0x50(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4C8C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 825A4C90: C1A3001C  lfs f13, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4C94: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825A4C98: 40990014  ble cr6, 0x825a4cac
	if !ctx.cr[6].gt {
	pc = 0x825A4CAC; continue 'dispatch;
	}
	// 825A4C9C: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4CA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A4CA4: 41820024  beq 0x825a4cc8
	if ctx.cr[0].eq {
	pc = 0x825A4CC8; continue 'dispatch;
	}
	// 825A4CA8: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4CAC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A4CB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4CB4: 4BFFFF1D  bl 0x825a4bd0
	ctx.lr = 0x825A4CB8;
	sub_825A4BD0(ctx, base);
	// 825A4CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A4CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4CC4: 4E800020  blr
	return;
	// 825A4CC8: D1A10050  stfs f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A4CCC: 4BFFFFE4  b 0x825a4cb0
	pc = 0x825A4CB0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A4CD0 size=112
    let mut pc: u32 = 0x825A4CD0;
    'dispatch: loop {
        match pc {
            0x825A4CD0 => {
    //   block [0x825A4CD0..0x825A4D40)
	// 825A4CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4CDC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4CE0: C0030020  lfs f0, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4CE4: ED800072  fmuls f12, f0, f1
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A4CE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4CEC: 419A000C  beq cr6, 0x825a4cf8
	if ctx.cr[6].eq {
	pc = 0x825A4CF8; continue 'dispatch;
	}
	// 825A4CF0: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4CF4: 48000008  b 0x825a4cfc
	pc = 0x825A4CFC; continue 'dispatch;
	// 825A4CF8: C1A30050  lfs f13, 0x50(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A4CFC: C0030018  lfs f0, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4D00: ED60602A  fadds f11, f0, f12
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 825A4D04: FF0D5800  fcmpu cr6, f13, f11
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[11].f64);
	// 825A4D08: 40980018  bge cr6, 0x825a4d20
	if !ctx.cr[6].lt {
	pc = 0x825A4D20; continue 'dispatch;
	}
	// 825A4D0C: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4D10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A4D14: 41820010  beq 0x825a4d24
	if ctx.cr[0].eq {
	pc = 0x825A4D24; continue 'dispatch;
	}
	// 825A4D18: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A4D1C: 48000008  b 0x825a4d24
	pc = 0x825A4D24; continue 'dispatch;
	// 825A4D20: EC0D6028  fsubs f0, f13, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 825A4D24: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A4D28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4D2C: 4BFFFEA5  bl 0x825a4bd0
	ctx.lr = 0x825A4D30;
	sub_825A4BD0(ctx, base);
	// 825A4D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A4D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4D40 size=48
    let mut pc: u32 = 0x825A4D40;
    'dispatch: loop {
        match pc {
            0x825A4D40 => {
    //   block [0x825A4D40..0x825A4D70)
	// 825A4D40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4D44: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4D48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4D4C: 419A000C  beq cr6, 0x825a4d58
	if ctx.cr[6].eq {
	pc = 0x825A4D58; continue 'dispatch;
	}
	// 825A4D50: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4D54: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A4D58: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4D5C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 825A4D60: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A4D64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4D68: 908B0050  stw r4, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 825A4D6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4D70 size=16
    let mut pc: u32 = 0x825A4D70;
    'dispatch: loop {
        match pc {
            0x825A4D70 => {
    //   block [0x825A4D70..0x825A4D80)
	// 825A4D70: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A4D74: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A4D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4D7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4D80 size=8
    let mut pc: u32 = 0x825A4D80;
    'dispatch: loop {
        match pc {
            0x825A4D80 => {
    //   block [0x825A4D80..0x825A4D88)
	// 825A4D80: 4800CB48  b 0x825b18c8
	sub_825B18C8(ctx, base);
	return;
	// 825A4D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4D88 size=140
    let mut pc: u32 = 0x825A4D88;
    'dispatch: loop {
        match pc {
            0x825A4D88 => {
    //   block [0x825A4D88..0x825A4E14)
	// 825A4D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4D94: E9430022  lwa r10, 0x20(r3)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as i32) as i64;
	// 825A4D98: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 825A4D9C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A4DA0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A4DA4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4DA8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4DB0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A4DB4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825A4DB8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825A4DBC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A4DC0: 419A000C  beq cr6, 0x825a4dcc
	if ctx.cr[6].eq {
	pc = 0x825A4DCC; continue 'dispatch;
	}
	// 825A4DC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4DC8: 48000008  b 0x825a4dd0
	pc = 0x825A4DD0; continue 'dispatch;
	// 825A4DCC: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A4DD0: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A4DD4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825A4DD8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825A4DDC: 40990014  ble cr6, 0x825a4df0
	if !ctx.cr[6].gt {
	pc = 0x825A4DF0; continue 'dispatch;
	}
	// 825A4DE0: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4DE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A4DE8: 41820024  beq 0x825a4e0c
	if ctx.cr[0].eq {
	pc = 0x825A4E0C; continue 'dispatch;
	}
	// 825A4DEC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4DF0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A4DF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4DF8: 4BFFFF49  bl 0x825a4d40
	ctx.lr = 0x825A4DFC;
	sub_825A4D40(ctx, base);
	// 825A4DFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A4E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4E08: 4E800020  blr
	return;
	// 825A4E0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A4E10: 4BFFFFE4  b 0x825a4df4
	pc = 0x825A4DF4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4E18 size=148
    let mut pc: u32 = 0x825A4E18;
    'dispatch: loop {
        match pc {
            0x825A4E18 => {
    //   block [0x825A4E18..0x825A4EAC)
	// 825A4E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4E24: E9430022  lwa r10, 0x20(r3)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as i32) as i64;
	// 825A4E28: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 825A4E2C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A4E30: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A4E34: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4E38: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4E3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4E40: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A4E44: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 825A4E48: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825A4E4C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A4E50: 419A000C  beq cr6, 0x825a4e5c
	if ctx.cr[6].eq {
	pc = 0x825A4E5C; continue 'dispatch;
	}
	// 825A4E54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4E58: 48000008  b 0x825a4e60
	pc = 0x825A4E60; continue 'dispatch;
	// 825A4E5C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A4E60: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4E64: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 825A4E68: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 825A4E6C: 40980020  bge cr6, 0x825a4e8c
	if !ctx.cr[6].lt {
	pc = 0x825A4E8C; continue 'dispatch;
	}
	// 825A4E70: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4E74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A4E78: 4182000C  beq 0x825a4e84
	if ctx.cr[0].eq {
	pc = 0x825A4E84; continue 'dispatch;
	}
	// 825A4E7C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A4E80: 48000010  b 0x825a4e90
	pc = 0x825A4E90; continue 'dispatch;
	// 825A4E84: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A4E88: 4800000C  b 0x825a4e94
	pc = 0x825A4E94; continue 'dispatch;
	// 825A4E8C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 825A4E90: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A4E94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4E98: 4BFFFEA9  bl 0x825a4d40
	ctx.lr = 0x825A4E9C;
	sub_825A4D40(ctx, base);
	// 825A4E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A4EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4EB0 size=48
    let mut pc: u32 = 0x825A4EB0;
    'dispatch: loop {
        match pc {
            0x825A4EB0 => {
    //   block [0x825A4EB0..0x825A4EE0)
	// 825A4EB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A4EB4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4EB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4EBC: 419A000C  beq cr6, 0x825a4ec8
	if ctx.cr[6].eq {
	pc = 0x825A4EC8; continue 'dispatch;
	}
	// 825A4EC0: 812B0050  lwz r9, 0x50(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A4EC4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A4EC8: 808B0050  lwz r4, 0x50(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A4ECC: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 825A4ED0: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A4ED4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A4ED8: 908B0014  stw r4, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 825A4EDC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4EE0 size=16
    let mut pc: u32 = 0x825A4EE0;
    'dispatch: loop {
        match pc {
            0x825A4EE0 => {
    //   block [0x825A4EE0..0x825A4EF0)
	// 825A4EE0: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A4EE4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A4EE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4EEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A4EF0 size=8
    let mut pc: u32 = 0x825A4EF0;
    'dispatch: loop {
        match pc {
            0x825A4EF0 => {
    //   block [0x825A4EF0..0x825A4EF8)
	// 825A4EF0: 4800C9D8  b 0x825b18c8
	sub_825B18C8(ctx, base);
	return;
	// 825A4EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4EF8 size=140
    let mut pc: u32 = 0x825A4EF8;
    'dispatch: loop {
        match pc {
            0x825A4EF8 => {
    //   block [0x825A4EF8..0x825A4F84)
	// 825A4EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4F04: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825A4F08: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4F0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4F10: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 825A4F14: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A4F18: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A4F1C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4F20: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A4F24: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4F28: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825A4F2C: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A4F30: 419A000C  beq cr6, 0x825a4f3c
	if ctx.cr[6].eq {
	pc = 0x825A4F3C; continue 'dispatch;
	}
	// 825A4F34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4F38: 48000008  b 0x825a4f40
	pc = 0x825A4F40; continue 'dispatch;
	// 825A4F3C: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A4F40: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A4F44: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825A4F48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A4F4C: 40990014  ble cr6, 0x825a4f60
	if !ctx.cr[6].gt {
	pc = 0x825A4F60; continue 'dispatch;
	}
	// 825A4F50: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4F54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A4F58: 41820024  beq 0x825a4f7c
	if ctx.cr[0].eq {
	pc = 0x825A4F7C; continue 'dispatch;
	}
	// 825A4F5C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4F60: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A4F64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A4F68: 4BFFFDD9  bl 0x825a4d40
	ctx.lr = 0x825A4F6C;
	sub_825A4D40(ctx, base);
	// 825A4F6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A4F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A4F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A4F78: 4E800020  blr
	return;
	// 825A4F7C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A4F80: 4BFFFFE4  b 0x825a4f64
	pc = 0x825A4F64; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A4F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A4F88 size=148
    let mut pc: u32 = 0x825A4F88;
    'dispatch: loop {
        match pc {
            0x825A4F88 => {
    //   block [0x825A4F88..0x825A501C)
	// 825A4F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A4F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A4F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A4F94: 81430020  lwz r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 825A4F98: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A4F9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A4FA0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 825A4FA4: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A4FA8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825A4FAC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825A4FB0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 825A4FB4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 825A4FB8: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 825A4FBC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A4FC0: 419A000C  beq cr6, 0x825a4fcc
	if ctx.cr[6].eq {
	pc = 0x825A4FCC; continue 'dispatch;
	}
	// 825A4FC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A4FC8: 48000008  b 0x825a4fd0
	pc = 0x825A4FD0; continue 'dispatch;
	// 825A4FCC: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A4FD0: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A4FD4: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 825A4FD8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 825A4FDC: 40980020  bge cr6, 0x825a4ffc
	if !ctx.cr[6].lt {
	pc = 0x825A4FFC; continue 'dispatch;
	}
	// 825A4FE0: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A4FE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A4FE8: 4182000C  beq 0x825a4ff4
	if ctx.cr[0].eq {
	pc = 0x825A4FF4; continue 'dispatch;
	}
	// 825A4FEC: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A4FF0: 48000010  b 0x825a5000
	pc = 0x825A5000; continue 'dispatch;
	// 825A4FF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A4FF8: 4800000C  b 0x825a5004
	pc = 0x825A5004; continue 'dispatch;
	// 825A4FFC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 825A5000: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A5004: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A5008: 4BFFFD39  bl 0x825a4d40
	ctx.lr = 0x825A500C;
	sub_825A4D40(ctx, base);
	// 825A500C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5020 size=76
    let mut pc: u32 = 0x825A5020;
    'dispatch: loop {
        match pc {
            0x825A5020 => {
    //   block [0x825A5020..0x825A506C)
	// 825A5020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A502C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A5030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A5034: 419A000C  beq cr6, 0x825a5040
	if ctx.cr[6].eq {
	pc = 0x825A5040; continue 'dispatch;
	}
	// 825A5038: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A503C: 48000008  b 0x825a5044
	pc = 0x825A5044; continue 'dispatch;
	// 825A5040: 89630040  lbz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A5044: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A5048: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A504C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825A5050: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825A5054: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 825A5058: 4BFFFAE9  bl 0x825a4b40
	ctx.lr = 0x825A505C;
	sub_825A4B40(ctx, base);
	// 825A505C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5070 size=104
    let mut pc: u32 = 0x825A5070;
    'dispatch: loop {
        match pc {
            0x825A5070 => {
    //   block [0x825A5070..0x825A50D8)
	// 825A5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A507C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5084: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A508C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A5090: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A5094: 388B9FF4  addi r4, r11, -0x600c
	ctx.r[4].s64 = ctx.r[11].s64 + -24588;
	// 825A5098: 4884E971  bl 0x82df3a08
	ctx.lr = 0x825A509C;
	sub_82DF3A08(ctx, base);
	// 825A509C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825A50A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A50A4: 4884E1FD  bl 0x82df32a0
	ctx.lr = 0x825A50A8;
	sub_82DF32A0(ctx, base);
	// 825A50A8: 98610050  stb r3, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u8 ) };
	// 825A50AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A50B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A50B4: 4BFFFA8D  bl 0x825a4b40
	ctx.lr = 0x825A50B8;
	sub_825A4B40(ctx, base);
	// 825A50B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A50BC: 4884E36D  bl 0x82df3428
	ctx.lr = 0x825A50C0;
	sub_82DF3428(ctx, base);
	// 825A50C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A50C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A50C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A50CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A50D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A50D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A50D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A50D8 size=300
    let mut pc: u32 = 0x825A50D8;
    'dispatch: loop {
        match pc {
            0x825A50D8 => {
    //   block [0x825A50D8..0x825A5204)
	// 825A50D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A50DC: 48C03089  bl 0x831a8164
	ctx.lr = 0x825A50E0;
	sub_831A8130(ctx, base);
	// 825A50E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A50E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A50E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A50EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A50F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A50F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825A50F8: 4823C441  bl 0x827e1538
	ctx.lr = 0x825A50FC;
	sub_827E1538(ctx, base);
	// 825A50FC: 8961005C  lbz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A5100: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5104: 418200B4  beq 0x825a51b8
	if ctx.cr[0].eq {
	pc = 0x825A51B8; continue 'dispatch;
	}
	// 825A5108: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A510C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5110: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5114: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A5118: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A511C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5120: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A5124: 4BDFBF35  bl 0x823a1058
	ctx.lr = 0x825A5128;
	sub_823A1058(ctx, base);
	// 825A5128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A512C: 4BE09DE5  bl 0x823aef10
	ctx.lr = 0x825A5130;
	sub_823AEF10(ctx, base);
	// 825A5130: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A5134: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825A5138: 419A0028  beq cr6, 0x825a5160
	if ctx.cr[6].eq {
	pc = 0x825A5160; continue 'dispatch;
	}
	// 825A513C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5140: 4BDFBF71  bl 0x823a10b0
	ctx.lr = 0x825A5144;
	sub_823A10B0(ctx, base);
	// 825A5144: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5148: 41820018  beq 0x825a5160
	if ctx.cr[0].eq {
	pc = 0x825A5160; continue 'dispatch;
	}
	// 825A514C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5150: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A5154: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A515C: 4E800421  bctrl
	ctx.lr = 0x825A5160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A5160: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5164: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A5168: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A516C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 825A5170: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 825A5174: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5178: 9B810064  stb r28, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u8 ) };
	// 825A517C: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 825A5180: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5184: 83C7001C  lwz r30, 0x1c(r7)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A5188: 7D7F4214  add r11, r31, r8
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 825A518C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 825A5190: 7CFF5214  add r7, r31, r10
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 825A5194: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5198: 890B0030  lbz r8, 0x30(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A519C: 794607E6  rldicr r6, r10, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A51A0: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 825A51A4: 4E800421  bctrl
	ctx.lr = 0x825A51A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A51A8: 89610074  lbz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A51AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A51B0: 41820008  beq 0x825a51b8
	if ctx.cr[0].eq {
	pc = 0x825A51B8; continue 'dispatch;
	}
	// 825A51B4: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 825A51B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A51BC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825A51C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A51C4: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A51C8: 419A0028  beq cr6, 0x825a51f0
	if ctx.cr[6].eq {
	pc = 0x825A51F0; continue 'dispatch;
	}
	// 825A51CC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A51D0: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A51D4: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 825A51D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A51DC: 409A0008  bne cr6, 0x825a51e4
	if !ctx.cr[6].eq {
	pc = 0x825A51E4; continue 'dispatch;
	}
	// 825A51E0: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A51E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A51E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A51EC: 4BE08CD5  bl 0x823adec0
	ctx.lr = 0x825A51F0;
	sub_823ADEC0(ctx, base);
	// 825A51F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A51F4: 4BE0BA0D  bl 0x823b0c00
	ctx.lr = 0x825A51F8;
	sub_823B0C00(ctx, base);
	// 825A51F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A51FC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A5200: 48C02FB4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5208 size=300
    let mut pc: u32 = 0x825A5208;
    'dispatch: loop {
        match pc {
            0x825A5208 => {
    //   block [0x825A5208..0x825A5334)
	// 825A5208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A520C: 48C02F59  bl 0x831a8164
	ctx.lr = 0x825A5210;
	sub_831A8130(ctx, base);
	// 825A5210: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5218: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A521C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5220: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A5224: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825A5228: 4823C311  bl 0x827e1538
	ctx.lr = 0x825A522C;
	sub_827E1538(ctx, base);
	// 825A522C: 8961005C  lbz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A5230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5234: 418200B4  beq 0x825a52e8
	if ctx.cr[0].eq {
	pc = 0x825A52E8; continue 'dispatch;
	}
	// 825A5238: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A523C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5240: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5244: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A5248: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A524C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5250: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A5254: 4BDFBE05  bl 0x823a1058
	ctx.lr = 0x825A5258;
	sub_823A1058(ctx, base);
	// 825A5258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A525C: 4BE09CB5  bl 0x823aef10
	ctx.lr = 0x825A5260;
	sub_823AEF10(ctx, base);
	// 825A5260: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A5264: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825A5268: 419A0028  beq cr6, 0x825a5290
	if ctx.cr[6].eq {
	pc = 0x825A5290; continue 'dispatch;
	}
	// 825A526C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5270: 4BDFBE41  bl 0x823a10b0
	ctx.lr = 0x825A5274;
	sub_823A10B0(ctx, base);
	// 825A5274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5278: 41820018  beq 0x825a5290
	if ctx.cr[0].eq {
	pc = 0x825A5290; continue 'dispatch;
	}
	// 825A527C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5280: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A5284: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A528C: 4E800421  bctrl
	ctx.lr = 0x825A5290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A5290: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5294: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A5298: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A529C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 825A52A0: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 825A52A4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A52A8: 9B810064  stb r28, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u8 ) };
	// 825A52AC: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 825A52B0: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A52B4: 83C70018  lwz r30, 0x18(r7)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A52B8: 7D7F4214  add r11, r31, r8
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[8].u64;
	// 825A52BC: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 825A52C0: 7CFF5214  add r7, r31, r10
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 825A52C4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A52C8: 890B0030  lbz r8, 0x30(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A52CC: 794607E6  rldicr r6, r10, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A52D0: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 825A52D4: 4E800421  bctrl
	ctx.lr = 0x825A52D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A52D8: 89610074  lbz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A52DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A52E0: 41820008  beq 0x825a52e8
	if ctx.cr[0].eq {
	pc = 0x825A52E8; continue 'dispatch;
	}
	// 825A52E4: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 825A52E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A52EC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825A52F0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A52F4: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A52F8: 419A0028  beq cr6, 0x825a5320
	if ctx.cr[6].eq {
	pc = 0x825A5320; continue 'dispatch;
	}
	// 825A52FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5300: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5304: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 825A5308: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A530C: 409A0008  bne cr6, 0x825a5314
	if !ctx.cr[6].eq {
	pc = 0x825A5314; continue 'dispatch;
	}
	// 825A5310: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A5314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A5318: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A531C: 4BE08BA5  bl 0x823adec0
	ctx.lr = 0x825A5320;
	sub_823ADEC0(ctx, base);
	// 825A5320: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A5324: 4BE0B8DD  bl 0x823b0c00
	ctx.lr = 0x825A5328;
	sub_823B0C00(ctx, base);
	// 825A5328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A532C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A5330: 48C02E84  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5338 size=308
    let mut pc: u32 = 0x825A5338;
    'dispatch: loop {
        match pc {
            0x825A5338 => {
    //   block [0x825A5338..0x825A546C)
	// 825A5338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A533C: 48C02E2D  bl 0x831a8168
	ctx.lr = 0x825A5340;
	sub_831A8130(ctx, base);
	// 825A5340: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825A5344: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A534C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825A5350: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A5354: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5358: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825A535C: 4823C1DD  bl 0x827e1538
	ctx.lr = 0x825A5360;
	sub_827E1538(ctx, base);
	// 825A5360: 8961005C  lbz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A5364: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5368: 418200B4  beq 0x825a541c
	if ctx.cr[0].eq {
	pc = 0x825A541C; continue 'dispatch;
	}
	// 825A536C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5370: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5374: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5378: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A537C: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5384: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A5388: 4BDFBCD1  bl 0x823a1058
	ctx.lr = 0x825A538C;
	sub_823A1058(ctx, base);
	// 825A538C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A5390: 4BE09B81  bl 0x823aef10
	ctx.lr = 0x825A5394;
	sub_823AEF10(ctx, base);
	// 825A5394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A5398: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825A539C: 419A0028  beq cr6, 0x825a53c4
	if ctx.cr[6].eq {
	pc = 0x825A53C4; continue 'dispatch;
	}
	// 825A53A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A53A4: 4BDFBD0D  bl 0x823a10b0
	ctx.lr = 0x825A53A8;
	sub_823A10B0(ctx, base);
	// 825A53A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A53AC: 41820018  beq 0x825a53c4
	if ctx.cr[0].eq {
	pc = 0x825A53C4; continue 'dispatch;
	}
	// 825A53B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A53B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A53B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A53BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A53C0: 4E800421  bctrl
	ctx.lr = 0x825A53C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A53C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A53C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825A53CC: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A53D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A53D4: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 825A53D8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A53DC: 9B810064  stb r28, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u8 ) };
	// 825A53E0: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 825A53E4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A53E8: 7D7F4A14  add r11, r31, r9
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[9].u64;
	// 825A53EC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 825A53F0: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A53F4: 7CFF5214  add r7, r31, r10
	ctx.r[7].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 825A53F8: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A53FC: 890B0030  lbz r8, 0x30(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A5400: 794607E6  rldicr r6, r10, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A5404: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825A5408: 4E800421  bctrl
	ctx.lr = 0x825A540C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A540C: 89610074  lbz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825A5410: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5414: 41820008  beq 0x825a541c
	if ctx.cr[0].eq {
	pc = 0x825A541C; continue 'dispatch;
	}
	// 825A5418: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 825A541C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5420: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825A5424: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5428: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A542C: 419A0028  beq cr6, 0x825a5454
	if ctx.cr[6].eq {
	pc = 0x825A5454; continue 'dispatch;
	}
	// 825A5430: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5434: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5438: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 825A543C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A5440: 409A0008  bne cr6, 0x825a5448
	if !ctx.cr[6].eq {
	pc = 0x825A5448; continue 'dispatch;
	}
	// 825A5444: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A5448: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A544C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A5450: 4BE08A71  bl 0x823adec0
	ctx.lr = 0x825A5454;
	sub_823ADEC0(ctx, base);
	// 825A5454: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A5458: 4BE0B7A9  bl 0x823b0c00
	ctx.lr = 0x825A545C;
	sub_823B0C00(ctx, base);
	// 825A545C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5460: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A5464: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 825A5468: 48C02D50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5470 size=352
    let mut pc: u32 = 0x825A5470;
    'dispatch: loop {
        match pc {
            0x825A5470 => {
    //   block [0x825A5470..0x825A55D0)
	// 825A5470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5474: 48C02CED  bl 0x831a8160
	ctx.lr = 0x825A5478;
	sub_831A8130(ctx, base);
	// 825A5478: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A547C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A5480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5484: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825A5488: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A548C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5490: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A5494: 4BDFF08D  bl 0x823a4520
	ctx.lr = 0x825A5498;
	sub_823A4520(ctx, base);
	// 825A5498: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A549C: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A54A0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825A54A4: 4BE09BAD  bl 0x823af050
	ctx.lr = 0x825A54A8;
	sub_823AF050(ctx, base);
	// 825A54A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A54AC: 418200BC  beq 0x825a5568
	if ctx.cr[0].eq {
	pc = 0x825A5568; continue 'dispatch;
	}
	// 825A54B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A54B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A54B8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A54BC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A54C0: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A54C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A54C8: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 825A54CC: 4BDFBB8D  bl 0x823a1058
	ctx.lr = 0x825A54D0;
	sub_823A1058(ctx, base);
	// 825A54D0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A54D4: 4BF5D8DD  bl 0x82502db0
	ctx.lr = 0x825A54D8;
	sub_82502DB0(ctx, base);
	// 825A54D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A54DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825A54E0: 419A0028  beq cr6, 0x825a5508
	if ctx.cr[6].eq {
	pc = 0x825A5508; continue 'dispatch;
	}
	// 825A54E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A54E8: 4BDFBBC9  bl 0x823a10b0
	ctx.lr = 0x825A54EC;
	sub_823A10B0(ctx, base);
	// 825A54EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A54F0: 41820018  beq 0x825a5508
	if ctx.cr[0].eq {
	pc = 0x825A5508; continue 'dispatch;
	}
	// 825A54F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A54F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A54FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A5504: 4E800421  bctrl
	ctx.lr = 0x825A5508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A5508: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A550C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825A5510: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825A5514: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825A5518: 99210070  stb r9, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u8 ) };
	// 825A551C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A5520: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 825A5524: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825A5528: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 825A552C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5530: E8E10068  ld r7, 0x68(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825A5534: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5538: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825A553C: 796807E6  rldicr r8, r11, 0x20, 0x3f
	ctx.r[8].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A5540: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 825A5544: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5548: 7D660034  cntlzw r6, r11
	ctx.r[6].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825A554C: 54C6DFFE  rlwinm r6, r6, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 825A5550: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825A5554: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 825A5558: 98C10080  stb r6, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[6].u8 ) };
	// 825A555C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 825A5560: 796607E6  rldicr r6, r11, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A5564: 4BF5A4DD  bl 0x824ffa40
	ctx.lr = 0x825A5568;
	sub_824FFA40(ctx, base);
	// 825A5568: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A556C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825A5570: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825A5574: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5578: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A557C: 419A0028  beq cr6, 0x825a55a4
	if ctx.cr[6].eq {
	pc = 0x825A55A4; continue 'dispatch;
	}
	// 825A5580: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5584: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5588: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 825A558C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825A5590: 409A0008  bne cr6, 0x825a5598
	if !ctx.cr[6].eq {
	pc = 0x825A5598; continue 'dispatch;
	}
	// 825A5594: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A5598: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A559C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A55A0: 4BE08921  bl 0x823adec0
	ctx.lr = 0x825A55A4;
	sub_823ADEC0(ctx, base);
	// 825A55A4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A55A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A55AC: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 825A55B0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A55B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A55B8: 419A000C  beq cr6, 0x825a55c4
	if ctx.cr[6].eq {
	pc = 0x825A55C4; continue 'dispatch;
	}
	// 825A55BC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 825A55C0: 4884BF19  bl 0x82df14d8
	ctx.lr = 0x825A55C4;
	sub_82DF14D8(ctx, base);
	// 825A55C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A55C8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825A55CC: 48C02BE4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A55D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A55D0 size=352
    let mut pc: u32 = 0x825A55D0;
    'dispatch: loop {
        match pc {
            0x825A55D0 => {
    //   block [0x825A55D0..0x825A5730)
	// 825A55D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A55D4: 48C02B8D  bl 0x831a8160
	ctx.lr = 0x825A55D8;
	sub_831A8130(ctx, base);
	// 825A55D8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A55DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A55E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A55E4: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825A55E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A55EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A55F0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A55F4: 4BDFEF2D  bl 0x823a4520
	ctx.lr = 0x825A55F8;
	sub_823A4520(ctx, base);
	// 825A55F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A55FC: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A5600: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825A5604: 4BE09A4D  bl 0x823af050
	ctx.lr = 0x825A5608;
	sub_823AF050(ctx, base);
	// 825A5608: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A560C: 418200BC  beq 0x825a56c8
	if ctx.cr[0].eq {
	pc = 0x825A56C8; continue 'dispatch;
	}
	// 825A5610: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5614: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5618: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A561C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A5620: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5624: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5628: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 825A562C: 4BDFBA2D  bl 0x823a1058
	ctx.lr = 0x825A5630;
	sub_823A1058(ctx, base);
	// 825A5630: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A5634: 4BF5D77D  bl 0x82502db0
	ctx.lr = 0x825A5638;
	sub_82502DB0(ctx, base);
	// 825A5638: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A563C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825A5640: 419A0028  beq cr6, 0x825a5668
	if ctx.cr[6].eq {
	pc = 0x825A5668; continue 'dispatch;
	}
	// 825A5644: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A5648: 4BDFBA69  bl 0x823a10b0
	ctx.lr = 0x825A564C;
	sub_823A10B0(ctx, base);
	// 825A564C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5650: 41820018  beq 0x825a5668
	if ctx.cr[0].eq {
	pc = 0x825A5668; continue 'dispatch;
	}
	// 825A5654: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5658: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A565C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A5664: 4E800421  bctrl
	ctx.lr = 0x825A5668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A5668: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A566C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825A5670: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825A5674: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825A5678: 99210070  stb r9, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u8 ) };
	// 825A567C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A5680: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 825A5684: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825A5688: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 825A568C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5690: E8E10068  ld r7, 0x68(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825A5694: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5698: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825A569C: 796807E6  rldicr r8, r11, 0x20, 0x3f
	ctx.r[8].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A56A0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 825A56A4: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A56A8: 7D660034  cntlzw r6, r11
	ctx.r[6].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825A56AC: 54C6DFFE  rlwinm r6, r6, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 825A56B0: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825A56B4: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 825A56B8: 98C10080  stb r6, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[6].u8 ) };
	// 825A56BC: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 825A56C0: 796607E6  rldicr r6, r11, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A56C4: 4BFFE0F5  bl 0x825a37b8
	ctx.lr = 0x825A56C8;
	sub_825A37B8(ctx, base);
	// 825A56C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A56CC: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825A56D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825A56D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A56D8: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A56DC: 419A0028  beq cr6, 0x825a5704
	if ctx.cr[6].eq {
	pc = 0x825A5704; continue 'dispatch;
	}
	// 825A56E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A56E4: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A56E8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 825A56EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825A56F0: 409A0008  bne cr6, 0x825a56f8
	if !ctx.cr[6].eq {
	pc = 0x825A56F8; continue 'dispatch;
	}
	// 825A56F4: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A56F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A56FC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A5700: 4BE087C1  bl 0x823adec0
	ctx.lr = 0x825A5704;
	sub_823ADEC0(ctx, base);
	// 825A5704: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5708: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A570C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 825A5710: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5714: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5718: 419A000C  beq cr6, 0x825a5724
	if ctx.cr[6].eq {
	pc = 0x825A5724; continue 'dispatch;
	}
	// 825A571C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 825A5720: 4884BDB9  bl 0x82df14d8
	ctx.lr = 0x825A5724;
	sub_82DF14D8(ctx, base);
	// 825A5724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5728: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825A572C: 48C02A84  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5730 size=412
    let mut pc: u32 = 0x825A5730;
    'dispatch: loop {
        match pc {
            0x825A5730 => {
    //   block [0x825A5730..0x825A58CC)
	// 825A5730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5734: 48C02A29  bl 0x831a815c
	ctx.lr = 0x825A5738;
	sub_831A8130(ctx, base);
	// 825A5738: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A573C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825A5740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5744: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A5748: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 825A574C: 4884DCE5  bl 0x82df3430
	ctx.lr = 0x825A5750;
	sub_82DF3430(ctx, base);
	// 825A5750: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5754: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A5758: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A575C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5760: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A5764: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5768: 40990010  ble cr6, 0x825a5778
	if !ctx.cr[6].gt {
	pc = 0x825A5778; continue 'dispatch;
	}
	// 825A576C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825A5770: 7F7D5850  subf r27, r29, r11
	ctx.r[27].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 825A5774: 41990008  bgt cr6, 0x825a577c
	if ctx.cr[6].gt {
	pc = 0x825A577C; continue 'dispatch;
	}
	// 825A5778: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 825A577C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A5784: 4823BDB5  bl 0x827e1538
	ctx.lr = 0x825A5788;
	sub_827E1538(ctx, base);
	// 825A5788: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A578C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A5790: 4082000C  bne 0x825a579c
	if !ctx.cr[0].eq {
	pc = 0x825A579C; continue 'dispatch;
	}
	// 825A5794: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	// 825A5798: 480000E8  b 0x825a5880
	pc = 0x825A5880; continue 'dispatch;
	// 825A579C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A57A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A57A4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A57A8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A57AC: 556B05F2  rlwinm r11, r11, 0, 0x17, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825A57B0: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 825A57B4: 419A0034  beq cr6, 0x825a57e8
	if ctx.cr[6].eq {
	pc = 0x825A57E8; continue 'dispatch;
	}
	// 825A57B8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825A57BC: 4099002C  ble cr6, 0x825a57e8
	if !ctx.cr[6].gt {
	pc = 0x825A57E8; continue 'dispatch;
	}
	// 825A57C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A57C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A57C8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A57CC: 888B0030  lbz r4, 0x30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A57D0: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A57D4: 4BE08CB5  bl 0x823ae488
	ctx.lr = 0x825A57D8;
	sub_823AE488(ctx, base);
	// 825A57D8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A57DC: 419A008C  beq cr6, 0x825a5868
	if ctx.cr[6].eq {
	pc = 0x825A5868; continue 'dispatch;
	}
	// 825A57E0: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825A57E4: 4181FFDC  bgt 0x825a57c0
	if ctx.cr[0].gt {
	pc = 0x825A57C0; continue 'dispatch;
	}
	// 825A57E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A57EC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825A57F0: 40990044  ble cr6, 0x825a5834
	if !ctx.cr[6].gt {
	pc = 0x825A5834; continue 'dispatch;
	}
	// 825A57F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A57F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A57FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A5800: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5804: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5808: 832B0028  lwz r25, 0x28(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A580C: 4884DA05  bl 0x82df3210
	ctx.lr = 0x825A5810;
	sub_82DF3210(ctx, base);
	// 825A5810: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A5814: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825A5818: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A581C: 4BE08C6D  bl 0x823ae488
	ctx.lr = 0x825A5820;
	sub_823AE488(ctx, base);
	// 825A5820: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A5824: 419A0044  beq cr6, 0x825a5868
	if ctx.cr[6].eq {
	pc = 0x825A5868; continue 'dispatch;
	}
	// 825A5828: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A582C: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825A5830: 4198FFC4  blt cr6, 0x825a57f4
	if ctx.cr[6].lt {
	pc = 0x825A57F4; continue 'dispatch;
	}
	// 825A5834: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825A5838: 40990034  ble cr6, 0x825a586c
	if !ctx.cr[6].gt {
	pc = 0x825A586C; continue 'dispatch;
	}
	// 825A583C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5840: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5844: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5848: 888B0030  lbz r4, 0x30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A584C: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A5850: 4BE08C39  bl 0x823ae488
	ctx.lr = 0x825A5854;
	sub_823AE488(ctx, base);
	// 825A5854: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A5858: 419A0010  beq cr6, 0x825a5868
	if ctx.cr[6].eq {
	pc = 0x825A5868; continue 'dispatch;
	}
	// 825A585C: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825A5860: 4181FFDC  bgt 0x825a583c
	if ctx.cr[0].gt {
	pc = 0x825A583C; continue 'dispatch;
	}
	// 825A5864: 48000008  b 0x825a586c
	pc = 0x825A586C; continue 'dispatch;
	// 825A5868: 3B400004  li r26, 4
	ctx.r[26].s64 = 4;
	// 825A586C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A5874: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5878: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A587C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 825A5880: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5884: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 825A5888: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A588C: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5890: 419A0028  beq cr6, 0x825a58b8
	if ctx.cr[6].eq {
	pc = 0x825A58B8; continue 'dispatch;
	}
	// 825A5894: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5898: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A589C: 7D6BD378  or r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[26].u64;
	// 825A58A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A58A4: 409A0008  bne cr6, 0x825a58ac
	if !ctx.cr[6].eq {
	pc = 0x825A58AC; continue 'dispatch;
	}
	// 825A58A8: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A58AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A58B0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A58B4: 4BE0860D  bl 0x823adec0
	ctx.lr = 0x825A58B8;
	sub_823ADEC0(ctx, base);
	// 825A58B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A58BC: 4BE0B345  bl 0x823b0c00
	ctx.lr = 0x825A58C0;
	sub_823B0C00(ctx, base);
	// 825A58C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A58C4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A58C8: 48C028E4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A58D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A58D0 size=96
    let mut pc: u32 = 0x825A58D0;
    'dispatch: loop {
        match pc {
            0x825A58D0 => {
    //   block [0x825A58D0..0x825A5930)
	// 825A58D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A58D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A58D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A58DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A58E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A58E4: 4BF5E4D5  bl 0x82503db8
	ctx.lr = 0x825A58E8;
	sub_82503DB8(ctx, base);
	// 825A58E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A58EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A58F0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A58F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A58F8: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A58FC: 40820018  bne 0x825a5914
	if !ctx.cr[0].eq {
	pc = 0x825A5914; continue 'dispatch;
	}
	// 825A5900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5904: 4BE0B9D5  bl 0x823b12d8
	ctx.lr = 0x825A5908;
	sub_823B12D8(ctx, base);
	// 825A5908: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A590C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A5910: 419A0008  beq cr6, 0x825a5918
	if ctx.cr[6].eq {
	pc = 0x825A5918; continue 'dispatch;
	}
	// 825A5914: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A5918: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A591C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A592C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5930 size=96
    let mut pc: u32 = 0x825A5930;
    'dispatch: loop {
        match pc {
            0x825A5930 => {
    //   block [0x825A5930..0x825A5990)
	// 825A5930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A593C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5944: 4BE0BA75  bl 0x823b13b8
	ctx.lr = 0x825A5948;
	sub_823B13B8(ctx, base);
	// 825A5948: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A594C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5950: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5954: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5958: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A595C: 40820018  bne 0x825a5974
	if !ctx.cr[0].eq {
	pc = 0x825A5974; continue 'dispatch;
	}
	// 825A5960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5964: 4BE0B975  bl 0x823b12d8
	ctx.lr = 0x825A5968;
	sub_823B12D8(ctx, base);
	// 825A5968: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A596C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A5970: 419A0008  beq cr6, 0x825a5978
	if ctx.cr[6].eq {
	pc = 0x825A5978; continue 'dispatch;
	}
	// 825A5974: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A5978: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A597C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A598C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5990 size=96
    let mut pc: u32 = 0x825A5990;
    'dispatch: loop {
        match pc {
            0x825A5990 => {
    //   block [0x825A5990..0x825A59F0)
	// 825A5990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A599C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A59A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A59A4: 4BFFFACD  bl 0x825a5470
	ctx.lr = 0x825A59A8;
	sub_825A5470(ctx, base);
	// 825A59A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A59AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A59B0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A59B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A59B8: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A59BC: 40820018  bne 0x825a59d4
	if !ctx.cr[0].eq {
	pc = 0x825A59D4; continue 'dispatch;
	}
	// 825A59C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A59C4: 4BE0B915  bl 0x823b12d8
	ctx.lr = 0x825A59C8;
	sub_823B12D8(ctx, base);
	// 825A59C8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A59CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A59D0: 419A0008  beq cr6, 0x825a59d8
	if ctx.cr[6].eq {
	pc = 0x825A59D8; continue 'dispatch;
	}
	// 825A59D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A59D8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A59DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A59E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A59E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A59E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A59EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A59F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A59F0 size=96
    let mut pc: u32 = 0x825A59F0;
    'dispatch: loop {
        match pc {
            0x825A59F0 => {
    //   block [0x825A59F0..0x825A5A50)
	// 825A59F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A59F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A59F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A59FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5A00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5A04: 4BF5E20D  bl 0x82503c10
	ctx.lr = 0x825A5A08;
	sub_82503C10(ctx, base);
	// 825A5A08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5A0C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5A10: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5A14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5A18: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5A1C: 40820018  bne 0x825a5a34
	if !ctx.cr[0].eq {
	pc = 0x825A5A34; continue 'dispatch;
	}
	// 825A5A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5A24: 4BE0B8B5  bl 0x823b12d8
	ctx.lr = 0x825A5A28;
	sub_823B12D8(ctx, base);
	// 825A5A28: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A5A2C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A5A30: 419A0008  beq cr6, 0x825a5a38
	if ctx.cr[6].eq {
	pc = 0x825A5A38; continue 'dispatch;
	}
	// 825A5A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A5A38: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A5A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5A50 size=100
    let mut pc: u32 = 0x825A5A50;
    'dispatch: loop {
        match pc {
            0x825A5A50 => {
    //   block [0x825A5A50..0x825A5AB4)
	// 825A5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5A58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5A5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5A60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825A5A64: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 825A5A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5A6C: 4BE0BF0D  bl 0x823b1978
	ctx.lr = 0x825A5A70;
	sub_823B1978(ctx, base);
	// 825A5A70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5A74: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 825A5A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5A7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5A80: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5A84: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A5A88: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 825A5A8C: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 825A5A90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5A94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5A98: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5A9C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 825A5AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5AB8 size=100
    let mut pc: u32 = 0x825A5AB8;
    'dispatch: loop {
        match pc {
            0x825A5AB8 => {
    //   block [0x825A5AB8..0x825A5B1C)
	// 825A5AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5AC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5AC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5AC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825A5ACC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 825A5AD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5AD4: 4BE0BEA5  bl 0x823b1978
	ctx.lr = 0x825A5AD8;
	sub_823B1978(ctx, base);
	// 825A5AD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5ADC: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 825A5AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5AE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5AE8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5AEC: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A5AF0: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 825A5AF4: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 825A5AF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5AFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5B00: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A5B04: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 825A5B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A5B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A5B20 size=204
    let mut pc: u32 = 0x825A5B20;
    'dispatch: loop {
        match pc {
            0x825A5B20 => {
    //   block [0x825A5B20..0x825A5BEC)
	// 825A5B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5B28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A5B2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5B30: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5B38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5B3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A5B40: 4BFFFF11  bl 0x825a5a50
	ctx.lr = 0x825A5B44;
	sub_825A5A50(ctx, base);
	// 825A5B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5B48: 4884D5A9  bl 0x82df30f0
	ctx.lr = 0x825A5B4C;
	sub_82DF30F0(ctx, base);
	// 825A5B4C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A5B50: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825A5B54: 4BFFF7E5  bl 0x825a5338
	ctx.lr = 0x825A5B58;
	sub_825A5338(ctx, base);
	// 825A5B58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5B5C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5B60: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5B64: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5B68: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5B6C: 40820018  bne 0x825a5b84
	if !ctx.cr[0].eq {
	pc = 0x825A5B84; continue 'dispatch;
	}
	// 825A5B70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5B74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5B78: 4BFFFDB9  bl 0x825a5930
	ctx.lr = 0x825A5B7C;
	sub_825A5930(ctx, base);
	// 825A5B7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5B80: 40820048  bne 0x825a5bc8
	if !ctx.cr[0].eq {
	pc = 0x825A5BC8; continue 'dispatch;
	}
	// 825A5B84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5B88: 3D008326  lis r8, -0x7cda
	ctx.r[8].s64 = -2094661632;
	// 825A5B8C: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A5B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A5B94: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A5B98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A5B9C: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A5BA0: 392999D0  addi r9, r9, -0x6630
	ctx.r[9].s64 = ctx.r[9].s64 + -26160;
	// 825A5BA4: 39687624  addi r11, r8, 0x7624
	ctx.r[11].s64 = ctx.r[8].s64 + 30244;
	// 825A5BA8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A5BAC: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 825A5BB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A5BB4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A5BB8: 4BD1A449  bl 0x822c0000
	ctx.lr = 0x825A5BBC;
	sub_822C0000(ctx, base);
	// 825A5BBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A5BC0: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A5BC4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A5BC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5BCC: 4BE02205  bl 0x823a7dd0
	ctx.lr = 0x825A5BD0;
	sub_823A7DD0(ctx, base);
	// 825A5BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5BD4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 825A5BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5BE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A5BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A5BF0 size=196
    let mut pc: u32 = 0x825A5BF0;
    'dispatch: loop {
        match pc {
            0x825A5BF0 => {
    //   block [0x825A5BF0..0x825A5CB4)
	// 825A5BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5BF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5BFC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825A5C00: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5C08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5C0C: 4BFFFE45  bl 0x825a5a50
	ctx.lr = 0x825A5C10;
	sub_825A5A50(ctx, base);
	// 825A5C10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5C14: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825A5C18: 4BFFFB19  bl 0x825a5730
	ctx.lr = 0x825A5C1C;
	sub_825A5730(ctx, base);
	// 825A5C1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5C20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5C24: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5C28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5C2C: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5C30: 40820018  bne 0x825a5c48
	if !ctx.cr[0].eq {
	pc = 0x825A5C48; continue 'dispatch;
	}
	// 825A5C34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A5C38: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5C3C: 4BFFFC95  bl 0x825a58d0
	ctx.lr = 0x825A5C40;
	sub_825A58D0(ctx, base);
	// 825A5C40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5C44: 40820048  bne 0x825a5c8c
	if !ctx.cr[0].eq {
	pc = 0x825A5C8C; continue 'dispatch;
	}
	// 825A5C48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5C4C: 3D008326  lis r8, -0x7cda
	ctx.r[8].s64 = -2094661632;
	// 825A5C50: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A5C54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A5C58: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A5C5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825A5C60: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A5C64: 39297624  addi r9, r9, 0x7624
	ctx.r[9].s64 = ctx.r[9].s64 + 30244;
	// 825A5C68: 396899D0  addi r11, r8, -0x6630
	ctx.r[11].s64 = ctx.r[8].s64 + -26160;
	// 825A5C6C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A5C70: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 825A5C74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825A5C7C: 4BD1A385  bl 0x822c0000
	ctx.lr = 0x825A5C80;
	sub_822C0000(ctx, base);
	// 825A5C80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A5C84: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A5C88: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825A5C8C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5C90: C3E10050  lfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825A5C94: 4BE0213D  bl 0x823a7dd0
	ctx.lr = 0x825A5C98;
	sub_823A7DD0(ctx, base);
	// 825A5C98: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825A5C9C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 825A5CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5CA8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A5CAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5CB8 size=204
    let mut pc: u32 = 0x825A5CB8;
    'dispatch: loop {
        match pc {
            0x825A5CB8 => {
    //   block [0x825A5CB8..0x825A5D84)
	// 825A5CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5CC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A5CC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5CC8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5CD0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5CD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A5CD8: 4BFFFDE1  bl 0x825a5ab8
	ctx.lr = 0x825A5CDC;
	sub_825A5AB8(ctx, base);
	// 825A5CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5CE0: 4884D411  bl 0x82df30f0
	ctx.lr = 0x825A5CE4;
	sub_82DF30F0(ctx, base);
	// 825A5CE4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A5CE8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5CEC: 4BFFF3ED  bl 0x825a50d8
	ctx.lr = 0x825A5CF0;
	sub_825A50D8(ctx, base);
	// 825A5CF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5CF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5CF8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5CFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5D00: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5D04: 40820018  bne 0x825a5d1c
	if !ctx.cr[0].eq {
	pc = 0x825A5D1C; continue 'dispatch;
	}
	// 825A5D08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5D0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5D10: 4BFFFC21  bl 0x825a5930
	ctx.lr = 0x825A5D14;
	sub_825A5930(ctx, base);
	// 825A5D14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5D18: 40820048  bne 0x825a5d60
	if !ctx.cr[0].eq {
	pc = 0x825A5D60; continue 'dispatch;
	}
	// 825A5D1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5D20: 3D008326  lis r8, -0x7cda
	ctx.r[8].s64 = -2094661632;
	// 825A5D24: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A5D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A5D2C: 3D208327  lis r9, -0x7cd9
	ctx.r[9].s64 = -2094596096;
	// 825A5D30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A5D34: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A5D38: 39293CA4  addi r9, r9, 0x3ca4
	ctx.r[9].s64 = ctx.r[9].s64 + 15524;
	// 825A5D3C: 39687624  addi r11, r8, 0x7624
	ctx.r[11].s64 = ctx.r[8].s64 + 30244;
	// 825A5D40: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A5D44: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 825A5D48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A5D4C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A5D50: 4BD1A2B1  bl 0x822c0000
	ctx.lr = 0x825A5D54;
	sub_822C0000(ctx, base);
	// 825A5D54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A5D58: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A5D5C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A5D60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5D64: 4BE0206D  bl 0x823a7dd0
	ctx.lr = 0x825A5D68;
	sub_823A7DD0(ctx, base);
	// 825A5D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5D6C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 825A5D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5D78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A5D7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5D88 size=188
    let mut pc: u32 = 0x825A5D88;
    'dispatch: loop {
        match pc {
            0x825A5D88 => {
    //   block [0x825A5D88..0x825A5E44)
	// 825A5D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5D94: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5D9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5DA0: 4BFFFD19  bl 0x825a5ab8
	ctx.lr = 0x825A5DA4;
	sub_825A5AB8(ctx, base);
	// 825A5DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5DA8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825A5DAC: 4BFFF985  bl 0x825a5730
	ctx.lr = 0x825A5DB0;
	sub_825A5730(ctx, base);
	// 825A5DB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5DB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5DB8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5DBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5DC0: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5DC4: 40820018  bne 0x825a5ddc
	if !ctx.cr[0].eq {
	pc = 0x825A5DDC; continue 'dispatch;
	}
	// 825A5DC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A5DCC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5DD0: 4BFFFBC1  bl 0x825a5990
	ctx.lr = 0x825A5DD4;
	sub_825A5990(ctx, base);
	// 825A5DD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5DD8: 40820048  bne 0x825a5e20
	if !ctx.cr[0].eq {
	pc = 0x825A5E20; continue 'dispatch;
	}
	// 825A5DDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5DE0: 3D008327  lis r8, -0x7cd9
	ctx.r[8].s64 = -2094596096;
	// 825A5DE4: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A5DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A5DEC: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A5DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825A5DF4: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A5DF8: 39297624  addi r9, r9, 0x7624
	ctx.r[9].s64 = ctx.r[9].s64 + 30244;
	// 825A5DFC: 39683CA4  addi r11, r8, 0x3ca4
	ctx.r[11].s64 = ctx.r[8].s64 + 15524;
	// 825A5E00: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A5E04: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 825A5E08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5E0C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825A5E10: 4BD1A1F1  bl 0x822c0000
	ctx.lr = 0x825A5E14;
	sub_822C0000(ctx, base);
	// 825A5E14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A5E18: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A5E1C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825A5E20: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5E24: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A5E28: 4BE01FA9  bl 0x823a7dd0
	ctx.lr = 0x825A5E2C;
	sub_823A7DD0(ctx, base);
	// 825A5E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5E30: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 825A5E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5E48 size=204
    let mut pc: u32 = 0x825A5E48;
    'dispatch: loop {
        match pc {
            0x825A5E48 => {
    //   block [0x825A5E48..0x825A5F14)
	// 825A5E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A5E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5E58: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5E60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5E64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A5E68: 4BFFFC51  bl 0x825a5ab8
	ctx.lr = 0x825A5E6C;
	sub_825A5AB8(ctx, base);
	// 825A5E6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5E70: 4884D281  bl 0x82df30f0
	ctx.lr = 0x825A5E74;
	sub_82DF30F0(ctx, base);
	// 825A5E74: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A5E78: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5E7C: 4BFFF38D  bl 0x825a5208
	ctx.lr = 0x825A5E80;
	sub_825A5208(ctx, base);
	// 825A5E80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5E84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5E88: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5E8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5E90: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5E94: 40820018  bne 0x825a5eac
	if !ctx.cr[0].eq {
	pc = 0x825A5EAC; continue 'dispatch;
	}
	// 825A5E98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5E9C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5EA0: 4BFFFA91  bl 0x825a5930
	ctx.lr = 0x825A5EA4;
	sub_825A5930(ctx, base);
	// 825A5EA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5EA8: 40820048  bne 0x825a5ef0
	if !ctx.cr[0].eq {
	pc = 0x825A5EF0; continue 'dispatch;
	}
	// 825A5EAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5EB0: 3D008326  lis r8, -0x7cda
	ctx.r[8].s64 = -2094661632;
	// 825A5EB4: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A5EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A5EBC: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A5EC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A5EC4: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A5EC8: 392986A8  addi r9, r9, -0x7958
	ctx.r[9].s64 = ctx.r[9].s64 + -31064;
	// 825A5ECC: 39687624  addi r11, r8, 0x7624
	ctx.r[11].s64 = ctx.r[8].s64 + 30244;
	// 825A5ED0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A5ED4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 825A5ED8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A5EDC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825A5EE0: 4BD1A121  bl 0x822c0000
	ctx.lr = 0x825A5EE4;
	sub_822C0000(ctx, base);
	// 825A5EE4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A5EE8: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A5EEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A5EF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5EF4: 4BE01EDD  bl 0x823a7dd0
	ctx.lr = 0x825A5EF8;
	sub_823A7DD0(ctx, base);
	// 825A5EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5EFC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 825A5F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5F08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A5F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5F18 size=188
    let mut pc: u32 = 0x825A5F18;
    'dispatch: loop {
        match pc {
            0x825A5F18 => {
    //   block [0x825A5F18..0x825A5FD4)
	// 825A5F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5F24: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5F28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5F2C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5F30: 4BFFFB89  bl 0x825a5ab8
	ctx.lr = 0x825A5F34;
	sub_825A5AB8(ctx, base);
	// 825A5F34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5F38: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825A5F3C: 4BFFF7F5  bl 0x825a5730
	ctx.lr = 0x825A5F40;
	sub_825A5730(ctx, base);
	// 825A5F40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A5F44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A5F48: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A5F4C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A5F50: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5F54: 40820018  bne 0x825a5f6c
	if !ctx.cr[0].eq {
	pc = 0x825A5F6C; continue 'dispatch;
	}
	// 825A5F58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A5F5C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5F60: 48023F59  bl 0x825c9eb8
	ctx.lr = 0x825A5F64;
	sub_825C9EB8(ctx, base);
	// 825A5F64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A5F68: 40820048  bne 0x825a5fb0
	if !ctx.cr[0].eq {
	pc = 0x825A5FB0; continue 'dispatch;
	}
	// 825A5F6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A5F70: 3D008326  lis r8, -0x7cda
	ctx.r[8].s64 = -2094661632;
	// 825A5F74: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A5F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A5F7C: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A5F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825A5F84: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A5F88: 39297624  addi r9, r9, 0x7624
	ctx.r[9].s64 = ctx.r[9].s64 + 30244;
	// 825A5F8C: 396886A8  addi r11, r8, -0x7958
	ctx.r[11].s64 = ctx.r[8].s64 + -31064;
	// 825A5F90: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A5F94: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 825A5F98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A5F9C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825A5FA0: 4BD1A061  bl 0x822c0000
	ctx.lr = 0x825A5FA4;
	sub_822C0000(ctx, base);
	// 825A5FA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A5FA8: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A5FAC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825A5FB0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5FB4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A5FB8: 4BE01E19  bl 0x823a7dd0
	ctx.lr = 0x825A5FBC;
	sub_823A7DD0(ctx, base);
	// 825A5FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A5FC0: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 825A5FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A5FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A5FCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A5FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A5FD8 size=184
    let mut pc: u32 = 0x825A5FD8;
    'dispatch: loop {
        match pc {
            0x825A5FD8 => {
    //   block [0x825A5FD8..0x825A6090)
	// 825A5FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A5FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A5FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A5FE4: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A5FE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A5FEC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A5FF0: 4BFFFAC9  bl 0x825a5ab8
	ctx.lr = 0x825A5FF4;
	sub_825A5AB8(ctx, base);
	// 825A5FF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A5FF8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825A5FFC: 4BFFF735  bl 0x825a5730
	ctx.lr = 0x825A6000;
	sub_825A5730(ctx, base);
	// 825A6000: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6004: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6008: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A600C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6010: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6014: 40820018  bne 0x825a602c
	if !ctx.cr[0].eq {
	pc = 0x825A602C; continue 'dispatch;
	}
	// 825A6018: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A601C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A6020: 4BFFF9D1  bl 0x825a59f0
	ctx.lr = 0x825A6024;
	sub_825A59F0(ctx, base);
	// 825A6024: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6028: 40820048  bne 0x825a6070
	if !ctx.cr[0].eq {
	pc = 0x825A6070; continue 'dispatch;
	}
	// 825A602C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A6030: 3D008326  lis r8, -0x7cda
	ctx.r[8].s64 = -2094661632;
	// 825A6034: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A6038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A603C: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A6040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825A6044: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A6048: 39297624  addi r9, r9, 0x7624
	ctx.r[9].s64 = ctx.r[9].s64 + 30244;
	// 825A604C: 39687618  addi r11, r8, 0x7618
	ctx.r[11].s64 = ctx.r[8].s64 + 30232;
	// 825A6050: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A6054: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 825A6058: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A605C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825A6060: 4BD19FA1  bl 0x822c0000
	ctx.lr = 0x825A6064;
	sub_822C0000(ctx, base);
	// 825A6064: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A6068: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A606C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825A6070: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A6074: 4BE01D5D  bl 0x823a7dd0
	ctx.lr = 0x825A6078;
	sub_823A7DD0(ctx, base);
	// 825A6078: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A607C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 825A6080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A608C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A6090 size=160
    let mut pc: u32 = 0x825A6090;
    'dispatch: loop {
        match pc {
            0x825A6090 => {
    //   block [0x825A6090..0x825A6130)
	// 825A6090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A609C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A60A0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A60A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A60A8: 808B0058  lwz r4, 0x58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A60AC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A60B0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825A60B4: 419A0038  beq cr6, 0x825a60ec
	if ctx.cr[6].eq {
	pc = 0x825A60EC; continue 'dispatch;
	}
	// 825A60B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A60BC: 419A000C  beq cr6, 0x825a60c8
	if ctx.cr[6].eq {
	pc = 0x825A60C8; continue 'dispatch;
	}
	// 825A60C0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A60C4: 48000008  b 0x825a60cc
	pc = 0x825A60CC; continue 'dispatch;
	// 825A60C8: C00B0050  lfs f0, 0x50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A60CC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A60D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A60D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A60D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A60DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A60E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A60E4: 4E800421  bctrl
	ctx.lr = 0x825A60E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A60E8: 48000030  b 0x825a6118
	pc = 0x825A6118; continue 'dispatch;
	// 825A60EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A60F0: 419A000C  beq cr6, 0x825a60fc
	if ctx.cr[6].eq {
	pc = 0x825A60FC; continue 'dispatch;
	}
	// 825A60F4: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A60F8: 48000008  b 0x825a6100
	pc = 0x825A6100; continue 'dispatch;
	// 825A60FC: C00B0050  lfs f0, 0x50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A6100: C1AB0024  lfs f13, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A6104: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6108: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 825A610C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A6110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6114: 4BFFFA0D  bl 0x825a5b20
	ctx.lr = 0x825A6118;
	sub_825A5B20(ctx, base);
	// 825A6118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A611C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A612C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A6130 size=64
    let mut pc: u32 = 0x825A6130;
    'dispatch: loop {
        match pc {
            0x825A6130 => {
    //   block [0x825A6130..0x825A6170)
	// 825A6130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A613C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6140: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6144: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A6148: 4BFFFAA9  bl 0x825a5bf0
	ctx.lr = 0x825A614C;
	sub_825A5BF0(ctx, base);
	// 825A614C: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A6150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6158: 4BFFEA79  bl 0x825a4bd0
	ctx.lr = 0x825A615C;
	sub_825A4BD0(ctx, base);
	// 825A615C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A616C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6170 size=160
    let mut pc: u32 = 0x825A6170;
    'dispatch: loop {
        match pc {
            0x825A6170 => {
    //   block [0x825A6170..0x825A6210)
	// 825A6170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A617C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6180: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A6184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6188: 808B0058  lwz r4, 0x58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A618C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6190: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825A6194: 419A0038  beq cr6, 0x825a61cc
	if ctx.cr[6].eq {
	pc = 0x825A61CC; continue 'dispatch;
	}
	// 825A6198: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A619C: 419A000C  beq cr6, 0x825a61a8
	if ctx.cr[6].eq {
	pc = 0x825A61A8; continue 'dispatch;
	}
	// 825A61A0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A61A4: 48000008  b 0x825a61ac
	pc = 0x825A61AC; continue 'dispatch;
	// 825A61A8: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A61AC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A61B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A61B4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A61B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A61BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A61C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A61C4: 4E800421  bctrl
	ctx.lr = 0x825A61C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A61C8: 48000030  b 0x825a61f8
	pc = 0x825A61F8; continue 'dispatch;
	// 825A61CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A61D0: 419A000C  beq cr6, 0x825a61dc
	if ctx.cr[6].eq {
	pc = 0x825A61DC; continue 'dispatch;
	}
	// 825A61D4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A61D8: 48000008  b 0x825a61e0
	pc = 0x825A61E0; continue 'dispatch;
	// 825A61DC: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A61E0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A61E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A61E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A61EC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A61F0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A61F4: 4BFFFAC5  bl 0x825a5cb8
	ctx.lr = 0x825A61F8;
	sub_825A5CB8(ctx, base);
	// 825A61F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A61FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A620C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6210 size=64
    let mut pc: u32 = 0x825A6210;
    'dispatch: loop {
        match pc {
            0x825A6210 => {
    //   block [0x825A6210..0x825A6250)
	// 825A6210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A621C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6224: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A6228: 4BFFFB61  bl 0x825a5d88
	ctx.lr = 0x825A622C;
	sub_825A5D88(ctx, base);
	// 825A622C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825A6230: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6238: 4BFFEB09  bl 0x825a4d40
	ctx.lr = 0x825A623C;
	sub_825A4D40(ctx, base);
	// 825A623C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A624C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6250 size=160
    let mut pc: u32 = 0x825A6250;
    'dispatch: loop {
        match pc {
            0x825A6250 => {
    //   block [0x825A6250..0x825A62F0)
	// 825A6250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A625C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6260: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A6264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6268: 808B0058  lwz r4, 0x58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A626C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6270: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825A6274: 419A0038  beq cr6, 0x825a62ac
	if ctx.cr[6].eq {
	pc = 0x825A62AC; continue 'dispatch;
	}
	// 825A6278: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A627C: 419A000C  beq cr6, 0x825a6288
	if ctx.cr[6].eq {
	pc = 0x825A6288; continue 'dispatch;
	}
	// 825A6280: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6284: 48000008  b 0x825a628c
	pc = 0x825A628C; continue 'dispatch;
	// 825A6288: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A628C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A6290: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A6294: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A629C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A62A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A62A4: 4E800421  bctrl
	ctx.lr = 0x825A62A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A62A8: 48000030  b 0x825a62d8
	pc = 0x825A62D8; continue 'dispatch;
	// 825A62AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A62B0: 419A000C  beq cr6, 0x825a62bc
	if ctx.cr[6].eq {
	pc = 0x825A62BC; continue 'dispatch;
	}
	// 825A62B4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A62B8: 48000008  b 0x825a62c0
	pc = 0x825A62C0; continue 'dispatch;
	// 825A62BC: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A62C0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A62C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A62C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A62CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A62D0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A62D4: 4BFFFB75  bl 0x825a5e48
	ctx.lr = 0x825A62D8;
	sub_825A5E48(ctx, base);
	// 825A62D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A62DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A62E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A62E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A62E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A62EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A62F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A62F0 size=64
    let mut pc: u32 = 0x825A62F0;
    'dispatch: loop {
        match pc {
            0x825A62F0 => {
    //   block [0x825A62F0..0x825A6330)
	// 825A62F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A62F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A62F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A62FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6304: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A6308: 4BFFFC11  bl 0x825a5f18
	ctx.lr = 0x825A630C;
	sub_825A5F18(ctx, base);
	// 825A630C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825A6310: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6318: 4BFFEA29  bl 0x825a4d40
	ctx.lr = 0x825A631C;
	sub_825A4D40(ctx, base);
	// 825A631C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A632C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6330 size=160
    let mut pc: u32 = 0x825A6330;
    'dispatch: loop {
        match pc {
            0x825A6330 => {
    //   block [0x825A6330..0x825A63D0)
	// 825A6330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A633C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6340: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A6344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6348: 808B0058  lwz r4, 0x58(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A634C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6350: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825A6354: 419A0038  beq cr6, 0x825a638c
	if ctx.cr[6].eq {
	pc = 0x825A638C; continue 'dispatch;
	}
	// 825A6358: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A635C: 419A000C  beq cr6, 0x825a6368
	if ctx.cr[6].eq {
	pc = 0x825A6368; continue 'dispatch;
	}
	// 825A6360: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6364: 48000008  b 0x825a636c
	pc = 0x825A636C; continue 'dispatch;
	// 825A6368: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A636C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A6370: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825A6374: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A637C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6384: 4E800421  bctrl
	ctx.lr = 0x825A6388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A6388: 48000030  b 0x825a63b8
	pc = 0x825A63B8; continue 'dispatch;
	// 825A638C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A6390: 419A000C  beq cr6, 0x825a639c
	if ctx.cr[6].eq {
	pc = 0x825A639C; continue 'dispatch;
	}
	// 825A6394: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6398: 48000008  b 0x825a63a0
	pc = 0x825A63A0; continue 'dispatch;
	// 825A639C: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A63A0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A63A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A63A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A63AC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A63B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A63B4: 4BE0D7E5  bl 0x823b3b98
	ctx.lr = 0x825A63B8;
	sub_823B3B98(ctx, base);
	// 825A63B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A63BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A63C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A63C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A63C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A63CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A63D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A63D0 size=64
    let mut pc: u32 = 0x825A63D0;
    'dispatch: loop {
        match pc {
            0x825A63D0 => {
    //   block [0x825A63D0..0x825A6410)
	// 825A63D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A63D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A63D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A63DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A63E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A63E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A63E8: 4BFFFBF1  bl 0x825a5fd8
	ctx.lr = 0x825A63EC;
	sub_825A5FD8(ctx, base);
	// 825A63EC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825A63F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A63F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A63F8: 4BFFE949  bl 0x825a4d40
	ctx.lr = 0x825A63FC;
	sub_825A4D40(ctx, base);
	// 825A63FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A640C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6410 size=24
    let mut pc: u32 = 0x825A6410;
    'dispatch: loop {
        match pc {
            0x825A6410 => {
    //   block [0x825A6410..0x825A6428)
	// 825A6410: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A6414: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825A6418: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 825A641C: 394AAF84  addi r10, r10, -0x507c
	ctx.r[10].s64 = ctx.r[10].s64 + -20604;
	// 825A6420: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A6424: 4884D004  b 0x82df3428
	sub_82DF3428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6428 size=92
    let mut pc: u32 = 0x825A6428;
    'dispatch: loop {
        match pc {
            0x825A6428 => {
    //   block [0x825A6428..0x825A6484)
	// 825A6428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6430: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A6434: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6438: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A643C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6440: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A6444: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A6448: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A644C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6450: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A6454: 4884CFD5  bl 0x82df3428
	ctx.lr = 0x825A6458;
	sub_82DF3428(ctx, base);
	// 825A6458: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A645C: 4182000C  beq 0x825a6468
	if ctx.cr[0].eq {
	pc = 0x825A6468; continue 'dispatch;
	}
	// 825A6460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6464: 4BD19E05  bl 0x822c0268
	ctx.lr = 0x825A6468;
	sub_822C0268(ctx, base);
	// 825A6468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A646C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6478: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A647C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6488 size=52
    let mut pc: u32 = 0x825A6488;
    'dispatch: loop {
        match pc {
            0x825A6488 => {
    //   block [0x825A6488..0x825A64BC)
	// 825A6488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6498: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 825A649C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A64A0: 4884D761  bl 0x82df3c00
	ctx.lr = 0x825A64A4;
	sub_82DF3C00(ctx, base);
	// 825A64A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A64A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A64AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A64B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A64B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A64B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A64C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A64C0 size=92
    let mut pc: u32 = 0x825A64C0;
    'dispatch: loop {
        match pc {
            0x825A64C0 => {
    //   block [0x825A64C0..0x825A651C)
	// 825A64C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A64C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A64C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A64CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A64D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A64D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A64D8: 3BC3000C  addi r30, r3, 0xc
	ctx.r[30].s64 = ctx.r[3].s64 + 12;
	// 825A64DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A64E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A64E4: 4BFFFFA5  bl 0x825a6488
	ctx.lr = 0x825A64E8;
	sub_825A6488(ctx, base);
	// 825A64E8: 4884CCC9  bl 0x82df31b0
	ctx.lr = 0x825A64EC;
	sub_82DF31B0(ctx, base);
	// 825A64EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A64F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A64F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A64F8: 48033929  bl 0x825d9e20
	ctx.lr = 0x825A64FC;
	sub_825D9E20(ctx, base);
	// 825A64FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6500: 4884CF29  bl 0x82df3428
	ctx.lr = 0x825A6504;
	sub_82DF3428(ctx, base);
	// 825A6504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A650C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6520 size=16
    let mut pc: u32 = 0x825A6520;
    'dispatch: loop {
        match pc {
            0x825A6520 => {
    //   block [0x825A6520..0x825A6530)
	// 825A6520: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6524: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A6528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A652C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6530 size=108
    let mut pc: u32 = 0x825A6530;
    'dispatch: loop {
        match pc {
            0x825A6530 => {
    //   block [0x825A6530..0x825A659C)
	// 825A6530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6534: 48C01C35  bl 0x831a8168
	ctx.lr = 0x825A6538;
	sub_831A8130(ctx, base);
	// 825A6538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A653C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A6540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6544: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A654C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A6550: 4884D4B9  bl 0x82df3a08
	ctx.lr = 0x825A6554;
	sub_82DF3A08(ctx, base);
	// 825A6554: 3BBE0030  addi r29, r30, 0x30
	ctx.r[29].s64 = ctx.r[30].s64 + 48;
	// 825A6558: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A655C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A6560: 4884CDA9  bl 0x82df3308
	ctx.lr = 0x825A6564;
	sub_82DF3308(ctx, base);
	// 825A6564: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A6568: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A656C: 4884CEBD  bl 0x82df3428
	ctx.lr = 0x825A6570;
	sub_82DF3428(ctx, base);
	// 825A6570: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6578: 41820010  beq 0x825a6588
	if ctx.cr[0].eq {
	pc = 0x825A6588; continue 'dispatch;
	}
	// 825A657C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6580: 4BFFFF09  bl 0x825a6488
	ctx.lr = 0x825A6584;
	sub_825A6488(ctx, base);
	// 825A6584: 4800000C  b 0x825a6590
	pc = 0x825A6590; continue 'dispatch;
	// 825A6588: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A658C: 4884D675  bl 0x82df3c00
	ctx.lr = 0x825A6590;
	sub_82DF3C00(ctx, base);
	// 825A6590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A6598: 48C01C20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A65A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A65A0 size=36
    let mut pc: u32 = 0x825A65A0;
    'dispatch: loop {
        match pc {
            0x825A65A0 => {
    //   block [0x825A65A0..0x825A65C4)
	// 825A65A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A65A4: 8943000C  lbz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A65A8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A65AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A65B0: 8943000C  lbz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A65B4: 9943000D  stb r10, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 825A65B8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A65BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A65C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A65C8 size=12
    let mut pc: u32 = 0x825A65C8;
    'dispatch: loop {
        match pc {
            0x825A65C8 => {
    //   block [0x825A65C8..0x825A65D4)
	// 825A65C8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A65CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A65D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A65D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A65D4 size=24
    let mut pc: u32 = 0x825A65D4;
    'dispatch: loop {
        match pc {
            0x825A65D4 => {
    //   block [0x825A65D4..0x825A65EC)
	// 825A65D4: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A65D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A65DC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A65E0: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A65E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A65E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A65EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A65EC size=4
    let mut pc: u32 = 0x825A65EC;
    'dispatch: loop {
        match pc {
            0x825A65EC => {
    //   block [0x825A65EC..0x825A65F0)
	// 825A65EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A65F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A65F0 size=100
    let mut pc: u32 = 0x825A65F0;
    'dispatch: loop {
        match pc {
            0x825A65F0 => {
    //   block [0x825A65F0..0x825A6654)
	// 825A65F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A65F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A65F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A65FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6604: F8C10098  std r6, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[6].u64 ) };
	// 825A6608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A660C: 83C10098  lwz r30, 0x98(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 825A6610: 80C100C4  lwz r6, 0xc4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 825A6614: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6618: F90100A8  std r8, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[8].u64 ) };
	// 825A661C: 810100A8  lwz r8, 0xa8(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 825A6620: 790807E6  rldicr r8, r8, 0x20, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A6624: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 825A6628: 7BC607E6  rldicr r6, r30, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[30].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A662C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A6630: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6634: 4E800421  bctrl
	ctx.lr = 0x825A6638;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A6638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A663C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6648: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A664C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6658 size=176
    let mut pc: u32 = 0x825A6658;
    'dispatch: loop {
        match pc {
            0x825A6658 => {
    //   block [0x825A6658..0x825A6708)
	// 825A6658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A6664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A666C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6670: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6674: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6678: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A667C: 419A000C  beq cr6, 0x825a6688
	if ctx.cr[6].eq {
	pc = 0x825A6688; continue 'dispatch;
	}
	// 825A6680: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6684: 48000008  b 0x825a668c
	pc = 0x825A668C; continue 'dispatch;
	// 825A6688: 895F000C  lbz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A668C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 825A6690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6694: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 825A6698: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 825A669C: 419A0008  beq cr6, 0x825a66a4
	if ctx.cr[6].eq {
	pc = 0x825A66A4; continue 'dispatch;
	}
	// 825A66A0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A66A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A66AC: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 825A66B0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A66B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A66B8: 4E800421  bctrl
	ctx.lr = 0x825A66BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A66BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A66C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A66C8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A66CC: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A66D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A66D4: 4E800421  bctrl
	ctx.lr = 0x825A66D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A66D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A66DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A66E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A66E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A66E8: 4E800421  bctrl
	ctx.lr = 0x825A66EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A66EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A66F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A66F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A66F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A66FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6708 size=40
    let mut pc: u32 = 0x825A6708;
    'dispatch: loop {
        match pc {
            0x825A6708 => {
    //   block [0x825A6708..0x825A6730)
	// 825A6708: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A670C: 8943000D  lbz r10, 0xd(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 825A6710: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6714: 419A0008  beq cr6, 0x825a671c
	if ctx.cr[6].eq {
	pc = 0x825A671C; continue 'dispatch;
	}
	// 825A6718: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A671C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6720: 9943000C  stb r10, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 825A6724: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A6728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A672C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6730 size=20
    let mut pc: u32 = 0x825A6730;
    'dispatch: loop {
        match pc {
            0x825A6730 => {
    //   block [0x825A6730..0x825A6744)
	// 825A6730: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6738: 419A000C  beq cr6, 0x825a6744
	if ctx.cr[6].eq {
		sub_825A6744(ctx, base);
		return;
	}
	// 825A673C: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6740: 48000008  b 0x825a6748
	sub_825A6744(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6744(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6744 size=12
    let mut pc: u32 = 0x825A6744;
    'dispatch: loop {
        match pc {
            0x825A6744 => {
    //   block [0x825A6744..0x825A6750)
	// 825A6744: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A6748: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 825A674C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6750 size=144
    let mut pc: u32 = 0x825A6750;
    'dispatch: loop {
        match pc {
            0x825A6750 => {
    //   block [0x825A6750..0x825A67E0)
	// 825A6750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A675C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6760: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A6764: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A6768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A676C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A6770: 38CB7F84  addi r6, r11, 0x7f84
	ctx.r[6].s64 = ctx.r[11].s64 + 32644;
	// 825A6774: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825A6778: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A677C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A6780: 48C037C9  bl 0x831a9f48
	ctx.lr = 0x825A6784;
	sub_831A9F48(ctx, base);
	// 825A6784: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A6788: 41820044  beq 0x825a67cc
	if ctx.cr[0].eq {
	pc = 0x825A67CC; continue 'dispatch;
	}
	// 825A678C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6794: 419A000C  beq cr6, 0x825a67a0
	if ctx.cr[6].eq {
	pc = 0x825A67A0; continue 'dispatch;
	}
	// 825A6798: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A679C: 48000008  b 0x825a67a4
	pc = 0x825A67A4; continue 'dispatch;
	// 825A67A0: 8943000C  lbz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A67A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A67A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A67AC: 419A0008  beq cr6, 0x825a67b4
	if ctx.cr[6].eq {
	pc = 0x825A67B4; continue 'dispatch;
	}
	// 825A67B0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A67B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A67B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A67BC: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 825A67C0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A67C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A67C8: 4E800421  bctrl
	ctx.lr = 0x825A67CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A67CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A67D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A67D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A67D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A67DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A67E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A67E0 size=104
    let mut pc: u32 = 0x825A67E0;
    'dispatch: loop {
        match pc {
            0x825A67E0 => {
    //   block [0x825A67E0..0x825A6848)
	// 825A67E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A67E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A67E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A67EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A67F0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A67F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A67F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A67FC: 419A000C  beq cr6, 0x825a6808
	if ctx.cr[6].eq {
	pc = 0x825A6808; continue 'dispatch;
	}
	// 825A6800: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6804: 48000008  b 0x825a680c
	pc = 0x825A680C; continue 'dispatch;
	// 825A6808: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A680C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6810: 41820010  beq 0x825a6820
	if ctx.cr[0].eq {
	pc = 0x825A6820; continue 'dispatch;
	}
	// 825A6814: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A6818: 388B9FFC  addi r4, r11, -0x6004
	ctx.r[4].s64 = ctx.r[11].s64 + -24580;
	// 825A681C: 4800000C  b 0x825a6828
	pc = 0x825A6828; continue 'dispatch;
	// 825A6820: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A6824: 388B9FF4  addi r4, r11, -0x600c
	ctx.r[4].s64 = ctx.r[11].s64 + -24588;
	// 825A6828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A682C: 4884D1DD  bl 0x82df3a08
	ctx.lr = 0x825A6830;
	sub_82DF3A08(ctx, base);
	// 825A6830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6848 size=20
    let mut pc: u32 = 0x825A6848;
    'dispatch: loop {
        match pc {
            0x825A6848 => {
    //   block [0x825A6848..0x825A685C)
	// 825A6848: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A684C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6850: 419A000C  beq cr6, 0x825a685c
	if ctx.cr[6].eq {
		sub_825A685C(ctx, base);
		return;
	}
	// 825A6854: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6858: 48000008  b 0x825a6860
	sub_825A685C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A685C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A685C size=48
    let mut pc: u32 = 0x825A685C;
    'dispatch: loop {
        match pc {
            0x825A685C => {
    //   block [0x825A685C..0x825A688C)
	// 825A685C: 8943000C  lbz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A6860: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 825A6864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6868: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 825A686C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 825A6870: 419A0008  beq cr6, 0x825a6878
	if ctx.cr[6].eq {
	pc = 0x825A6878; continue 'dispatch;
	}
	// 825A6874: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A6878: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A687C: 9943000C  stb r10, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 825A6880: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A6884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6888: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6890 size=116
    let mut pc: u32 = 0x825A6890;
    'dispatch: loop {
        match pc {
            0x825A6890 => {
    //   block [0x825A6890..0x825A6904)
	// 825A6890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6894: 48C018D5  bl 0x831a8168
	ctx.lr = 0x825A6898;
	sub_831A8130(ctx, base);
	// 825A6898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A689C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A68A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A68A4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A68A8: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A68AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A68B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A68B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A68B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825A68BC: 4884C835  bl 0x82df30f0
	ctx.lr = 0x825A68C0;
	sub_82DF30F0(ctx, base);
	// 825A68C0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A68C4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825A68C8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A68CC: 396BAFDC  addi r11, r11, -0x5024
	ctx.r[11].s64 = ctx.r[11].s64 + -20516;
	// 825A68D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A68D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A68D8: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A68DC: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A68E0: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A68E4: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 825A68E8: 4BFE5629  bl 0x8258bf10
	ctx.lr = 0x825A68EC;
	sub_8258BF10(ctx, base);
	// 825A68EC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825A68F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A68F4: 4884D30D  bl 0x82df3c00
	ctx.lr = 0x825A68F8;
	sub_82DF3C00(ctx, base);
	// 825A68F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A68FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A6900: 48C018B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6908 size=108
    let mut pc: u32 = 0x825A6908;
    'dispatch: loop {
        match pc {
            0x825A6908 => {
    //   block [0x825A6908..0x825A6974)
	// 825A6908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A690C: 48C01861  bl 0x831a816c
	ctx.lr = 0x825A6910;
	sub_831A8130(ctx, base);
	// 825A6910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6918: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A691C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A6920: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A6924: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6928: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A692C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A6930: 4884C7C1  bl 0x82df30f0
	ctx.lr = 0x825A6934;
	sub_82DF30F0(ctx, base);
	// 825A6934: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A6938: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825A693C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A6940: 396BAFDC  addi r11, r11, -0x5024
	ctx.r[11].s64 = ctx.r[11].s64 + -20516;
	// 825A6944: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825A6948: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A694C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6950: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6954: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 825A6958: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A695C: 997F000D  stb r11, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 825A6960: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825A6964: 4884D29D  bl 0x82df3c00
	ctx.lr = 0x825A6968;
	sub_82DF3C00(ctx, base);
	// 825A6968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A696C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6970: 48C0184C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6978 size=96
    let mut pc: u32 = 0x825A6978;
    'dispatch: loop {
        match pc {
            0x825A6978 => {
    //   block [0x825A6978..0x825A69D8)
	// 825A6978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A6984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A698C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6990: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6994: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825A6998: 4884CA91  bl 0x82df3428
	ctx.lr = 0x825A699C;
	sub_82DF3428(ctx, base);
	// 825A699C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 825A69A0: 4BD22319  bl 0x822c8cb8
	ctx.lr = 0x825A69A4;
	sub_822C8CB8(ctx, base);
	// 825A69A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A69A8: 4BFFFA69  bl 0x825a6410
	ctx.lr = 0x825A69AC;
	sub_825A6410(ctx, base);
	// 825A69AC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A69B0: 4182000C  beq 0x825a69bc
	if ctx.cr[0].eq {
	pc = 0x825A69BC; continue 'dispatch;
	}
	// 825A69B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A69B8: 4BD198B1  bl 0x822c0268
	ctx.lr = 0x825A69BC;
	sub_822C0268(ctx, base);
	// 825A69BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A69C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A69C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A69C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A69CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A69D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A69D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A69D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A69D8 size=296
    let mut pc: u32 = 0x825A69D8;
    'dispatch: loop {
        match pc {
            0x825A69D8 => {
    //   block [0x825A69D8..0x825A6B00)
	// 825A69D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A69DC: 48C01785  bl 0x831a8160
	ctx.lr = 0x825A69E0;
	sub_831A8130(ctx, base);
	// 825A69E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A69E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A69E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A69EC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A69F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A69F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A69F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A69FC: 388BB02C  addi r4, r11, -0x4fd4
	ctx.r[4].s64 = ctx.r[11].s64 + -20436;
	// 825A6A00: 38A0004E  li r5, 0x4e
	ctx.r[5].s64 = 78;
	// 825A6A04: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 825A6A08: 4BD199D1  bl 0x822c03d8
	ctx.lr = 0x825A6A0C;
	sub_822C03D8(ctx, base);
	// 825A6A0C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825A6A10: 41820044  beq 0x825a6a54
	if ctx.cr[0].eq {
	pc = 0x825A6A54; continue 'dispatch;
	}
	// 825A6A14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6A18: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A6A1C: 837F0008  lwz r27, 8(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6A20: 3B5F0010  addi r26, r31, 0x10
	ctx.r[26].s64 = ctx.r[31].s64 + 16;
	// 825A6A24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A6A28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6A2C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A6A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6A34: 4E800421  bctrl
	ctx.lr = 0x825A6A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A6A38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A6A3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6A40: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 825A6A44: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825A6A48: 4BFFFE49  bl 0x825a6890
	ctx.lr = 0x825A6A4C;
	sub_825A6890(ctx, base);
	// 825A6A4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A6A50: 48000008  b 0x825a6a58
	pc = 0x825A6A58; continue 'dispatch;
	// 825A6A54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A6A58: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825A6A5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6A60: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A6A64: 4BFFA405  bl 0x825a0e68
	ctx.lr = 0x825A6A68;
	sub_825A0E68(ctx, base);
	// 825A6A68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A6A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6A70: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A6A74: 4BD1958D  bl 0x822c0000
	ctx.lr = 0x825A6A78;
	sub_822C0000(ctx, base);
	// 825A6A78: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6A7C: 4182000C  beq 0x825a6a88
	if ctx.cr[0].eq {
	pc = 0x825A6A88; continue 'dispatch;
	}
	// 825A6A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6A84: 4884C9A5  bl 0x82df3428
	ctx.lr = 0x825A6A88;
	sub_82DF3428(ctx, base);
	// 825A6A88: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A6A8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6A90: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 825A6A94: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A6A98: 4BD22221  bl 0x822c8cb8
	ctx.lr = 0x825A6A9C;
	sub_822C8CB8(ctx, base);
	// 825A6A9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A6AA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6AA4: 4BFFF9E5  bl 0x825a6488
	ctx.lr = 0x825A6AA8;
	sub_825A6488(ctx, base);
	// 825A6AA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A6AAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6AB0: 4822EFB9  bl 0x827d5a68
	ctx.lr = 0x825A6AB4;
	sub_827D5A68(ctx, base);
	// 825A6AB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6AB8: 4884C971  bl 0x82df3428
	ctx.lr = 0x825A6ABC;
	sub_82DF3428(ctx, base);
	// 825A6ABC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A6AC0: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825A6AC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A6AC8: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825A6ACC: 419A0028  beq cr6, 0x825a6af4
	if ctx.cr[6].eq {
	pc = 0x825A6AF4; continue 'dispatch;
	}
	// 825A6AD0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825A6AD4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825A6AD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A6ADC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825A6AE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A6AE4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825A6AE8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A6AEC: 4082FFE8  bne 0x825a6ad4
	if !ctx.cr[0].eq {
	pc = 0x825A6AD4; continue 'dispatch;
	}
	// 825A6AF0: 4BD19DA1  bl 0x822c0890
	ctx.lr = 0x825A6AF4;
	sub_822C0890(ctx, base);
	// 825A6AF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A6AF8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A6AFC: 48C016B4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6B00 size=20
    let mut pc: u32 = 0x825A6B00;
    'dispatch: loop {
        match pc {
            0x825A6B00 => {
    //   block [0x825A6B00..0x825A6B14)
	// 825A6B00: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 825A6B04: 386A0010  addi r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 16;
	// 825A6B08: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6B0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6B10: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6B14 size=16
    let mut pc: u32 = 0x825A6B14;
    'dispatch: loop {
        match pc {
            0x825A6B14 => {
    //   block [0x825A6B14..0x825A6B24)
	// 825A6B14: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A6B18: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A6B1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6B20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6B24 size=20
    let mut pc: u32 = 0x825A6B24;
    'dispatch: loop {
        match pc {
            0x825A6B24 => {
    //   block [0x825A6B24..0x825A6B38)
	// 825A6B24: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6B2C: 419A000C  beq cr6, 0x825a6b38
	if ctx.cr[6].eq {
		sub_825A6B38(ctx, base);
		return;
	}
	// 825A6B30: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6B34: 48000008  b 0x825a6b3c
	sub_825A6B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6B38 size=8
    let mut pc: u32 = 0x825A6B38;
    'dispatch: loop {
        match pc {
            0x825A6B38 => {
    //   block [0x825A6B38..0x825A6B40)
	// 825A6B38: 888A000C  lbz r4, 0xc(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A6B3C: 4800AD8C  b 0x825b18c8
	sub_825B18C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A6B40 size=4
    let mut pc: u32 = 0x825A6B40;
    'dispatch: loop {
        match pc {
            0x825A6B40 => {
    //   block [0x825A6B40..0x825A6B44)
	// 825A6B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6B48 size=352
    let mut pc: u32 = 0x825A6B48;
    'dispatch: loop {
        match pc {
            0x825A6B48 => {
    //   block [0x825A6B48..0x825A6CA8)
	// 825A6B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6B4C: 48C01615  bl 0x831a8160
	ctx.lr = 0x825A6B50;
	sub_831A8130(ctx, base);
	// 825A6B50: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6B54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A6B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6B5C: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825A6B60: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A6B64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A6B68: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A6B6C: 4BDFD9B5  bl 0x823a4520
	ctx.lr = 0x825A6B70;
	sub_823A4520(ctx, base);
	// 825A6B70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A6B74: 83410068  lwz r26, 0x68(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A6B78: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825A6B7C: 4BE084D5  bl 0x823af050
	ctx.lr = 0x825A6B80;
	sub_823AF050(ctx, base);
	// 825A6B80: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6B84: 418200BC  beq 0x825a6c40
	if ctx.cr[0].eq {
	pc = 0x825A6C40; continue 'dispatch;
	}
	// 825A6B88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6B8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6B90: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A6B94: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825A6B98: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6B9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A6BA0: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 825A6BA4: 4BDFA4B5  bl 0x823a1058
	ctx.lr = 0x825A6BA8;
	sub_823A1058(ctx, base);
	// 825A6BA8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A6BAC: 4BF5C205  bl 0x82502db0
	ctx.lr = 0x825A6BB0;
	sub_82502DB0(ctx, base);
	// 825A6BB0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A6BB4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825A6BB8: 419A0028  beq cr6, 0x825a6be0
	if ctx.cr[6].eq {
	pc = 0x825A6BE0; continue 'dispatch;
	}
	// 825A6BBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A6BC0: 4BDFA4F1  bl 0x823a10b0
	ctx.lr = 0x825A6BC4;
	sub_823A10B0(ctx, base);
	// 825A6BC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A6BC8: 41820018  beq 0x825a6be0
	if ctx.cr[0].eq {
	pc = 0x825A6BE0; continue 'dispatch;
	}
	// 825A6BCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6BD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A6BD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6BDC: 4E800421  bctrl
	ctx.lr = 0x825A6BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A6BE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6BE4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825A6BE8: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825A6BEC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825A6BF0: 99210070  stb r9, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u8 ) };
	// 825A6BF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A6BF8: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 825A6BFC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825A6C00: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 825A6C04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6C08: E8E10068  ld r7, 0x68(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 825A6C0C: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A6C10: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825A6C14: 796807E6  rldicr r8, r11, 0x20, 0x3f
	ctx.r[8].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A6C18: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 825A6C1C: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A6C20: 7D660034  cntlzw r6, r11
	ctx.r[6].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825A6C24: 54C6DFFE  rlwinm r6, r6, 0x1b, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 825A6C28: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825A6C2C: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 825A6C30: 98C10080  stb r6, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[6].u8 ) };
	// 825A6C34: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 825A6C38: 796607E6  rldicr r6, r11, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 825A6C3C: 4BFFF9B5  bl 0x825a65f0
	ctx.lr = 0x825A6C40;
	sub_825A65F0(ctx, base);
	// 825A6C40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6C44: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825A6C48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825A6C4C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6C50: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A6C54: 419A0028  beq cr6, 0x825a6c7c
	if ctx.cr[6].eq {
	pc = 0x825A6C7C; continue 'dispatch;
	}
	// 825A6C58: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6C5C: 81230028  lwz r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A6C60: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 825A6C64: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825A6C68: 409A0008  bne cr6, 0x825a6c70
	if !ctx.cr[6].eq {
	pc = 0x825A6C70; continue 'dispatch;
	}
	// 825A6C6C: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 825A6C70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A6C74: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A6C78: 4BE07249  bl 0x823adec0
	ctx.lr = 0x825A6C7C;
	sub_823ADEC0(ctx, base);
	// 825A6C7C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6C80: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6C84: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 825A6C88: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A6C8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6C90: 419A000C  beq cr6, 0x825a6c9c
	if ctx.cr[6].eq {
	pc = 0x825A6C9C; continue 'dispatch;
	}
	// 825A6C94: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 825A6C98: 4884A841  bl 0x82df14d8
	ctx.lr = 0x825A6C9C;
	sub_82DF14D8(ctx, base);
	// 825A6C9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6CA0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825A6CA4: 48C0150C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6CA8 size=96
    let mut pc: u32 = 0x825A6CA8;
    'dispatch: loop {
        match pc {
            0x825A6CA8 => {
    //   block [0x825A6CA8..0x825A6D08)
	// 825A6CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6CBC: 4BFFFE8D  bl 0x825a6b48
	ctx.lr = 0x825A6CC0;
	sub_825A6B48(ctx, base);
	// 825A6CC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6CC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6CC8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A6CCC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6CD0: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6CD4: 40820018  bne 0x825a6cec
	if !ctx.cr[0].eq {
	pc = 0x825A6CEC; continue 'dispatch;
	}
	// 825A6CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6CDC: 4BE0A5FD  bl 0x823b12d8
	ctx.lr = 0x825A6CE0;
	sub_823B12D8(ctx, base);
	// 825A6CE0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 825A6CE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825A6CE8: 419A0008  beq cr6, 0x825a6cf0
	if ctx.cr[6].eq {
	pc = 0x825A6CF0; continue 'dispatch;
	}
	// 825A6CEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6CF0: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825A6CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6D08 size=100
    let mut pc: u32 = 0x825A6D08;
    'dispatch: loop {
        match pc {
            0x825A6D08 => {
    //   block [0x825A6D08..0x825A6D6C)
	// 825A6D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6D10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6D18: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825A6D1C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 825A6D20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6D24: 4BE0AC55  bl 0x823b1978
	ctx.lr = 0x825A6D28;
	sub_823B1978(ctx, base);
	// 825A6D28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6D2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A6D30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6D34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6D38: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A6D3C: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A6D40: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 825A6D44: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 825A6D48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6D4C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6D50: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 825A6D54: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 825A6D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6D64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6D70 size=188
    let mut pc: u32 = 0x825A6D70;
    'dispatch: loop {
        match pc {
            0x825A6D70 => {
    //   block [0x825A6D70..0x825A6E2C)
	// 825A6D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6D7C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6D84: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A6D88: 4BFFFF81  bl 0x825a6d08
	ctx.lr = 0x825A6D8C;
	sub_825A6D08(ctx, base);
	// 825A6D8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A6D90: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 825A6D94: 4BFFE99D  bl 0x825a5730
	ctx.lr = 0x825A6D98;
	sub_825A5730(ctx, base);
	// 825A6D98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6D9C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A6DA0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 825A6DA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6DA8: 556B077D  rlwinm. r11, r11, 0, 0x1d, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6DAC: 40820018  bne 0x825a6dc4
	if !ctx.cr[0].eq {
	pc = 0x825A6DC4; continue 'dispatch;
	}
	// 825A6DB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A6DB4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A6DB8: 4BFFFEF1  bl 0x825a6ca8
	ctx.lr = 0x825A6DBC;
	sub_825A6CA8(ctx, base);
	// 825A6DBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A6DC0: 40820048  bne 0x825a6e08
	if !ctx.cr[0].eq {
	pc = 0x825A6E08; continue 'dispatch;
	}
	// 825A6DC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A6DC8: 3D008325  lis r8, -0x7cdb
	ctx.r[8].s64 = -2094727168;
	// 825A6DCC: 396B94B4  addi r11, r11, -0x6b4c
	ctx.r[11].s64 = ctx.r[11].s64 + -27468;
	// 825A6DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825A6DD4: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 825A6DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825A6DDC: 394AC284  addi r10, r10, -0x3d7c
	ctx.r[10].s64 = ctx.r[10].s64 + -15740;
	// 825A6DE0: 39297624  addi r9, r9, 0x7624
	ctx.r[9].s64 = ctx.r[9].s64 + 30244;
	// 825A6DE4: 39687FC4  addi r11, r8, 0x7fc4
	ctx.r[11].s64 = ctx.r[8].s64 + 32708;
	// 825A6DE8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A6DEC: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 825A6DF0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A6DF4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825A6DF8: 4BD19209  bl 0x822c0000
	ctx.lr = 0x825A6DFC;
	sub_822C0000(ctx, base);
	// 825A6DFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A6E00: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 825A6E04: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825A6E08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A6E0C: 8BE10050  lbz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A6E10: 4BE00FC1  bl 0x823a7dd0
	ctx.lr = 0x825A6E14;
	sub_823A7DD0(ctx, base);
	// 825A6E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6E18: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 825A6E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6E24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6E30 size=92
    let mut pc: u32 = 0x825A6E30;
    'dispatch: loop {
        match pc {
            0x825A6E30 => {
    //   block [0x825A6E30..0x825A6E8C)
	// 825A6E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6E38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6E3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6E40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6E44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A6E48: 4BFFFF29  bl 0x825a6d70
	ctx.lr = 0x825A6E4C;
	sub_825A6D70(ctx, base);
	// 825A6E4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A6E50: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 825A6E54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6E58: 419A0008  beq cr6, 0x825a6e60
	if ctx.cr[6].eq {
	pc = 0x825A6E60; continue 'dispatch;
	}
	// 825A6E5C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A6E60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6E68: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 825A6E6C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A6E70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6E74: 4E800421  bctrl
	ctx.lr = 0x825A6E78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A6E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6E84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6E90 size=52
    let mut pc: u32 = 0x825A6E90;
    'dispatch: loop {
        match pc {
            0x825A6E90 => {
    //   block [0x825A6E90..0x825A6EC4)
	// 825A6E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6E98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6E9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6EA0: 38840028  addi r4, r4, 0x28
	ctx.r[4].s64 = ctx.r[4].s64 + 40;
	// 825A6EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6EA8: 4884CD59  bl 0x82df3c00
	ctx.lr = 0x825A6EAC;
	sub_82DF3C00(ctx, base);
	// 825A6EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A6EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6EC8 size=196
    let mut pc: u32 = 0x825A6EC8;
    'dispatch: loop {
        match pc {
            0x825A6EC8 => {
    //   block [0x825A6EC8..0x825A6F8C)
	// 825A6EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A6ED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A6ED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A6ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A6EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6EE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825A6EE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A6EEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A6EF0: 4BD19A49  bl 0x822c0938
	ctx.lr = 0x825A6EF4;
	sub_822C0938(ctx, base);
	// 825A6EF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A6EF8: 41820028  beq 0x825a6f20
	if ctx.cr[0].eq {
	pc = 0x825A6F20; continue 'dispatch;
	}
	// 825A6EFC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A6F00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825A6F04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A6F08: 392BB070  addi r9, r11, -0x4f90
	ctx.r[9].s64 = ctx.r[11].s64 + -20368;
	// 825A6F0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A6F10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A6F14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A6F18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A6F1C: 48000008  b 0x825a6f24
	pc = 0x825A6F24; continue 'dispatch;
	// 825A6F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A6F24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A6F28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A6F2C: 409A0044  bne cr6, 0x825a6f70
	if !ctx.cr[6].eq {
	pc = 0x825A6F70; continue 'dispatch;
	}
	// 825A6F30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A6F34: 419A001C  beq cr6, 0x825a6f50
	if ctx.cr[6].eq {
	pc = 0x825A6F50; continue 'dispatch;
	}
	// 825A6F38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6F3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A6F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6F44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A6F48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A6F4C: 4E800421  bctrl
	ctx.lr = 0x825A6F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A6F50: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825A6F54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A6F58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A6F5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825A6F60: 816B7FA4  lwz r11, 0x7fa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32676 as u32) ) } as u64;
	// 825A6F64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A6F68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A6F6C: 4BD19095  bl 0x822c0000
	ctx.lr = 0x825A6F70;
	sub_822C0000(ctx, base);
	// 825A6F70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A6F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A6F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A6F80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A6F84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A6F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6F90 size=92
    let mut pc: u32 = 0x825A6F90;
    'dispatch: loop {
        match pc {
            0x825A6F90 => {
    //   block [0x825A6F90..0x825A6FEC)
	// 825A6F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6F94: 48C011D9  bl 0x831a816c
	ctx.lr = 0x825A6F98;
	sub_831A8130(ctx, base);
	// 825A6F98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6F9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A6FA0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A6FA4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A6FA8: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A6FAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A6FB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A6FB4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A6FB8: 4884C139  bl 0x82df30f0
	ctx.lr = 0x825A6FBC;
	sub_82DF30F0(ctx, base);
	// 825A6FBC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A6FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A6FC4: 396BB084  addi r11, r11, -0x4f7c
	ctx.r[11].s64 = ctx.r[11].s64 + -20348;
	// 825A6FC8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A6FCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A6FD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A6FD4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A6FD8: 4884CC29  bl 0x82df3c00
	ctx.lr = 0x825A6FDC;
	sub_82DF3C00(ctx, base);
	// 825A6FDC: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 825A6FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A6FE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A6FE8: 48C011D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A6FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A6FF0 size=100
    let mut pc: u32 = 0x825A6FF0;
    'dispatch: loop {
        match pc {
            0x825A6FF0 => {
    //   block [0x825A6FF0..0x825A7054)
	// 825A6FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A6FF4: 48C01175  bl 0x831a8168
	ctx.lr = 0x825A6FF8;
	sub_831A8130(ctx, base);
	// 825A6FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A6FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7000: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A7004: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A7008: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A700C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A7010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7014: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A7018: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825A701C: 4884C0D5  bl 0x82df30f0
	ctx.lr = 0x825A7020;
	sub_82DF30F0(ctx, base);
	// 825A7020: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A7024: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A7028: 396BB084  addi r11, r11, -0x4f7c
	ctx.r[11].s64 = ctx.r[11].s64 + -20348;
	// 825A702C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A7030: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7034: 4BFE4EDD  bl 0x8258bf10
	ctx.lr = 0x825A7038;
	sub_8258BF10(ctx, base);
	// 825A7038: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A703C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A7040: 4884CBC1  bl 0x82df3c00
	ctx.lr = 0x825A7044;
	sub_82DF3C00(ctx, base);
	// 825A7044: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 825A7048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A704C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A7050: 48C01168  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7058 size=96
    let mut pc: u32 = 0x825A7058;
    'dispatch: loop {
        match pc {
            0x825A7058 => {
    //   block [0x825A7058..0x825A70B8)
	// 825A7058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A705C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A7064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A706C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A7074: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825A7078: 4884C3B1  bl 0x82df3428
	ctx.lr = 0x825A707C;
	sub_82DF3428(ctx, base);
	// 825A707C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A7080: 4BD21C39  bl 0x822c8cb8
	ctx.lr = 0x825A7084;
	sub_822C8CB8(ctx, base);
	// 825A7084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7088: 4BFFF389  bl 0x825a6410
	ctx.lr = 0x825A708C;
	sub_825A6410(ctx, base);
	// 825A708C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A7090: 4182000C  beq 0x825a709c
	if ctx.cr[0].eq {
	pc = 0x825A709C; continue 'dispatch;
	}
	// 825A7094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7098: 4BD191D1  bl 0x822c0268
	ctx.lr = 0x825A709C;
	sub_822C0268(ctx, base);
	// 825A709C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A70A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A70A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A70A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A70AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A70B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A70B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A70B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A70B8 size=268
    let mut pc: u32 = 0x825A70B8;
    'dispatch: loop {
        match pc {
            0x825A70B8 => {
    //   block [0x825A70B8..0x825A71C4)
	// 825A70B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A70BC: 48C010AD  bl 0x831a8168
	ctx.lr = 0x825A70C0;
	sub_831A8130(ctx, base);
	// 825A70C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A70C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A70C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A70CC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825A70D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A70D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A70D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A70DC: 388BB0D8  addi r4, r11, -0x4f28
	ctx.r[4].s64 = ctx.r[11].s64 + -20264;
	// 825A70E0: 38A0003F  li r5, 0x3f
	ctx.r[5].s64 = 63;
	// 825A70E4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 825A70E8: 4BD192F1  bl 0x822c03d8
	ctx.lr = 0x825A70EC;
	sub_822C03D8(ctx, base);
	// 825A70EC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825A70F0: 41820038  beq 0x825a7128
	if ctx.cr[0].eq {
	pc = 0x825A7128; continue 'dispatch;
	}
	// 825A70F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A70F8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A70FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A7100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A7104: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A7108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A710C: 4E800421  bctrl
	ctx.lr = 0x825A7110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7110: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A7114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7118: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825A711C: 4BFFFE75  bl 0x825a6f90
	ctx.lr = 0x825A7120;
	sub_825A6F90(ctx, base);
	// 825A7120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7124: 48000008  b 0x825a712c
	pc = 0x825A712C; continue 'dispatch;
	// 825A7128: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A712C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825A7130: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A7134: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A7138: 4BFFFD91  bl 0x825a6ec8
	ctx.lr = 0x825A713C;
	sub_825A6EC8(ctx, base);
	// 825A713C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A7140: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A7144: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A7148: 4BD18EB9  bl 0x822c0000
	ctx.lr = 0x825A714C;
	sub_822C0000(ctx, base);
	// 825A714C: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A7150: 4182000C  beq 0x825a715c
	if ctx.cr[0].eq {
	pc = 0x825A715C; continue 'dispatch;
	}
	// 825A7154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A7158: 4884C2D1  bl 0x82df3428
	ctx.lr = 0x825A715C;
	sub_82DF3428(ctx, base);
	// 825A715C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A7160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A7164: 4BFFF325  bl 0x825a6488
	ctx.lr = 0x825A7168;
	sub_825A6488(ctx, base);
	// 825A7168: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A716C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A7170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7174: 4822E8F5  bl 0x827d5a68
	ctx.lr = 0x825A7178;
	sub_827D5A68(ctx, base);
	// 825A7178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A717C: 4884C2AD  bl 0x82df3428
	ctx.lr = 0x825A7180;
	sub_82DF3428(ctx, base);
	// 825A7180: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A7184: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A7188: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A718C: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825A7190: 419A0028  beq cr6, 0x825a71b8
	if ctx.cr[6].eq {
	pc = 0x825A71B8; continue 'dispatch;
	}
	// 825A7194: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825A7198: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825A719C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A71A0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825A71A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A71A8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825A71AC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A71B0: 4082FFE8  bne 0x825a7198
	if !ctx.cr[0].eq {
	pc = 0x825A7198; continue 'dispatch;
	}
	// 825A71B4: 4BD196DD  bl 0x822c0890
	ctx.lr = 0x825A71B8;
	sub_822C0890(ctx, base);
	// 825A71B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A71BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A71C0: 48C00FF8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A71C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A71C8 size=88
    let mut pc: u32 = 0x825A71C8;
    'dispatch: loop {
        match pc {
            0x825A71C8 => {
    //   block [0x825A71C8..0x825A7220)
	// 825A71C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A71CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A71D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A71D4: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A71D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825A71DC: 409A0030  bne cr6, 0x825a720c
	if !ctx.cr[6].eq {
	pc = 0x825A720C; continue 'dispatch;
	}
	// 825A71E0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825A71E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A71E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A71EC: 419A0018  beq cr6, 0x825a7204
	if ctx.cr[6].eq {
	pc = 0x825A7204; continue 'dispatch;
	}
	// 825A71F0: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A71F4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A71F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A71FC: 419A0008  beq cr6, 0x825a7204
	if ctx.cr[6].eq {
	pc = 0x825A7204; continue 'dispatch;
	}
	// 825A7200: 4BEAC481  bl 0x82453680
	ctx.lr = 0x825A7204;
	sub_82453680(ctx, base);
	// 825A7204: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A7208: 48000008  b 0x825a7210
	pc = 0x825A7210; continue 'dispatch;
	// 825A720C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A7210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A7214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A721C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7220 size=16
    let mut pc: u32 = 0x825A7220;
    'dispatch: loop {
        match pc {
            0x825A7220 => {
    //   block [0x825A7220..0x825A7230)
	// 825A7220: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825A7224: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7228: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A722C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7230 size=16
    let mut pc: u32 = 0x825A7230;
    'dispatch: loop {
        match pc {
            0x825A7230 => {
    //   block [0x825A7230..0x825A7240)
	// 825A7230: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A7234: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A7238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A723C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7240 size=8
    let mut pc: u32 = 0x825A7240;
    'dispatch: loop {
        match pc {
            0x825A7240 => {
    //   block [0x825A7240..0x825A7248)
	// 825A7240: 4BEAC440  b 0x82453680
	sub_82453680(ctx, base);
	return;
	// 825A7244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7248 size=60
    let mut pc: u32 = 0x825A7248;
    'dispatch: loop {
        match pc {
            0x825A7248 => {
    //   block [0x825A7248..0x825A7284)
	// 825A7248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A724C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7254: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825A7258: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A725C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825A7260: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7264: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A726C: 4E800421  bctrl
	ctx.lr = 0x825A7270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A7274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A7278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A727C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7288 size=108
    let mut pc: u32 = 0x825A7288;
    'dispatch: loop {
        match pc {
            0x825A7288 => {
    //   block [0x825A7288..0x825A72F4)
	// 825A7288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A728C: 48C00EDD  bl 0x831a8168
	ctx.lr = 0x825A7290;
	sub_831A8130(ctx, base);
	// 825A7290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A7298: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A729C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A72A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A72A4: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A72A8: 4884C761  bl 0x82df3a08
	ctx.lr = 0x825A72AC;
	sub_82DF3A08(ctx, base);
	// 825A72AC: 3BBE003C  addi r29, r30, 0x3c
	ctx.r[29].s64 = ctx.r[30].s64 + 60;
	// 825A72B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A72B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A72B8: 4884C051  bl 0x82df3308
	ctx.lr = 0x825A72BC;
	sub_82DF3308(ctx, base);
	// 825A72BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A72C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A72C4: 4884C165  bl 0x82df3428
	ctx.lr = 0x825A72C8;
	sub_82DF3428(ctx, base);
	// 825A72C8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A72CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A72D0: 41820010  beq 0x825a72e0
	if ctx.cr[0].eq {
	pc = 0x825A72E0; continue 'dispatch;
	}
	// 825A72D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A72D8: 4BFFF1B1  bl 0x825a6488
	ctx.lr = 0x825A72DC;
	sub_825A6488(ctx, base);
	// 825A72DC: 4800000C  b 0x825a72e8
	pc = 0x825A72E8; continue 'dispatch;
	// 825A72E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A72E4: 4884C91D  bl 0x82df3c00
	ctx.lr = 0x825A72E8;
	sub_82DF3C00(ctx, base);
	// 825A72E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A72EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A72F0: 48C00EC8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A72F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A72F8 size=136
    let mut pc: u32 = 0x825A72F8;
    'dispatch: loop {
        match pc {
            0x825A72F8 => {
    //   block [0x825A72F8..0x825A7380)
	// 825A72F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A72FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A7304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A730C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A7310: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A7314: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825A7318: 409A0020  bne cr6, 0x825a7338
	if !ctx.cr[6].eq {
	pc = 0x825A7338; continue 'dispatch;
	}
	// 825A731C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A7320: 419A0048  beq cr6, 0x825a7368
	if ctx.cr[6].eq {
	pc = 0x825A7368; continue 'dispatch;
	}
	// 825A7324: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7328: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A732C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7330: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A7334: 48000034  b 0x825a7368
	pc = 0x825A7368; continue 'dispatch;
	// 825A7338: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825A733C: 419A002C  beq cr6, 0x825a7368
	if ctx.cr[6].eq {
	pc = 0x825A7368; continue 'dispatch;
	}
	// 825A7340: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A7344: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7348: 388B80F8  addi r4, r11, -0x7f08
	ctx.r[4].s64 = ctx.r[11].s64 + -32520;
	// 825A734C: 48C00DAD  bl 0x831a80f8
	ctx.lr = 0x825A7350;
	sub_831A80F8(ctx, base);
	// 825A7350: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A7354: 4182000C  beq 0x825a7360
	if ctx.cr[0].eq {
	pc = 0x825A7360; continue 'dispatch;
	}
	// 825A7358: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825A735C: 4800000C  b 0x825a7368
	pc = 0x825A7368; continue 'dispatch;
	// 825A7360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A7364: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7368: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A736C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7374: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A7378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A737C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7380 size=136
    let mut pc: u32 = 0x825A7380;
    'dispatch: loop {
        match pc {
            0x825A7380 => {
    //   block [0x825A7380..0x825A7408)
	// 825A7380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7388: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A738C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A7398: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A739C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825A73A0: 409A0020  bne cr6, 0x825a73c0
	if !ctx.cr[6].eq {
	pc = 0x825A73C0; continue 'dispatch;
	}
	// 825A73A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A73A8: 419A0048  beq cr6, 0x825a73f0
	if ctx.cr[6].eq {
	pc = 0x825A73F0; continue 'dispatch;
	}
	// 825A73AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A73B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A73B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A73B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A73BC: 48000034  b 0x825a73f0
	pc = 0x825A73F0; continue 'dispatch;
	// 825A73C0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825A73C4: 419A002C  beq cr6, 0x825a73f0
	if ctx.cr[6].eq {
	pc = 0x825A73F0; continue 'dispatch;
	}
	// 825A73C8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A73CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A73D0: 388B81D0  addi r4, r11, -0x7e30
	ctx.r[4].s64 = ctx.r[11].s64 + -32304;
	// 825A73D4: 48C00D25  bl 0x831a80f8
	ctx.lr = 0x825A73D8;
	sub_831A80F8(ctx, base);
	// 825A73D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A73DC: 4182000C  beq 0x825a73e8
	if ctx.cr[0].eq {
	pc = 0x825A73E8; continue 'dispatch;
	}
	// 825A73E0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825A73E4: 4800000C  b 0x825a73f0
	pc = 0x825A73F0; continue 'dispatch;
	// 825A73E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A73EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A73F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A73F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A73F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A73FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A7400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A7404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7408 size=36
    let mut pc: u32 = 0x825A7408;
    'dispatch: loop {
        match pc {
            0x825A7408 => {
    //   block [0x825A7408..0x825A742C)
	// 825A7408: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A740C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7410: 409A0008  bne cr6, 0x825a7418
	if !ctx.cr[6].eq {
	pc = 0x825A7418; continue 'dispatch;
	}
	// 825A7414: 3963001C  addi r11, r3, 0x1c
	ctx.r[11].s64 = ctx.r[3].s64 + 28;
	// 825A7418: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A741C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A7420: 409A000C  bne cr6, 0x825a742c
	if !ctx.cr[6].eq {
		sub_825A742C(ctx, base);
		return;
	}
	// 825A7424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A7428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A742C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A742C size=20
    let mut pc: u32 = 0x825A742C;
    'dispatch: loop {
        match pc {
            0x825A742C => {
    //   block [0x825A742C..0x825A7440)
	// 825A742C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7430: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825A7434: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 825A7438: 7C6B4BD6  divw r3, r11, r9
	ctx.r[3].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 825A743C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7440 size=80
    let mut pc: u32 = 0x825A7440;
    'dispatch: loop {
        match pc {
            0x825A7440 => {
    //   block [0x825A7440..0x825A7490)
	// 825A7440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7444: 48C00D29  bl 0x831a816c
	ctx.lr = 0x825A7448;
	sub_831A8130(ctx, base);
	// 825A7448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A744C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7450: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A7454: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A7458: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A745C: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A7460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7464: 4E800421  bctrl
	ctx.lr = 0x825A7468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7468: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A746C: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A7470: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A7474: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A7478: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A747C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7484: 4E800421  bctrl
	ctx.lr = 0x825A7488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A748C: 48C00D30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7490 size=80
    let mut pc: u32 = 0x825A7490;
    'dispatch: loop {
        match pc {
            0x825A7490 => {
    //   block [0x825A7490..0x825A74E0)
	// 825A7490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7494: 48C00CD9  bl 0x831a816c
	ctx.lr = 0x825A7498;
	sub_831A8130(ctx, base);
	// 825A7498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A749C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A74A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A74A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A74A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A74AC: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A74B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A74B4: 4E800421  bctrl
	ctx.lr = 0x825A74B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A74B8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A74BC: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A74C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A74C4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A74C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A74CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A74D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A74D4: 4E800421  bctrl
	ctx.lr = 0x825A74D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A74D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A74DC: 48C00CE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A74E0 size=36
    let mut pc: u32 = 0x825A74E0;
    'dispatch: loop {
        match pc {
            0x825A74E0 => {
    //   block [0x825A74E0..0x825A7504)
	// 825A74E0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A74E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A74E8: 409A0008  bne cr6, 0x825a74f0
	if !ctx.cr[6].eq {
	pc = 0x825A74F0; continue 'dispatch;
	}
	// 825A74EC: 3963001C  addi r11, r3, 0x1c
	ctx.r[11].s64 = ctx.r[3].s64 + 28;
	// 825A74F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A74F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A74F8: 409A000C  bne cr6, 0x825a7504
	if !ctx.cr[6].eq {
		sub_825A7504(ctx, base);
		return;
	}
	// 825A74FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A7500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7504 size=16
    let mut pc: u32 = 0x825A7504;
    'dispatch: loop {
        match pc {
            0x825A7504 => {
    //   block [0x825A7504..0x825A7514)
	// 825A7504: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7508: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 825A750C: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 825A7510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7518 size=108
    let mut pc: u32 = 0x825A7518;
    'dispatch: loop {
        match pc {
            0x825A7518 => {
    //   block [0x825A7518..0x825A7584)
	// 825A7518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A751C: 48C00C51  bl 0x831a816c
	ctx.lr = 0x825A7520;
	sub_831A8130(ctx, base);
	// 825A7520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7524: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A7528: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A752C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A7530: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A7534: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A7538: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A753C: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 825A7540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7544: 4E800421  bctrl
	ctx.lr = 0x825A7548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7548: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A754C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7550: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A7554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7558: 4E800421  bctrl
	ctx.lr = 0x825A755C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A755C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A7560: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A7564: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A7568: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A756C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7570: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A7574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7578: 4E800421  bctrl
	ctx.lr = 0x825A757C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A757C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7580: 48C00C3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7588 size=88
    let mut pc: u32 = 0x825A7588;
    'dispatch: loop {
        match pc {
            0x825A7588 => {
    //   block [0x825A7588..0x825A75E0)
	// 825A7588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A758C: 48C00BE1  bl 0x831a816c
	ctx.lr = 0x825A7590;
	sub_831A8130(ctx, base);
	// 825A7590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7594: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A7598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A759C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A75A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A75A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A75A8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A75AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A75B0: 4E800421  bctrl
	ctx.lr = 0x825A75B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A75B4: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A75B8: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A75BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A75C0: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825A75C4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A75C8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A75CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A75D0: 4E800421  bctrl
	ctx.lr = 0x825A75D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A75D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A75D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A75DC: 48C00BE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A75E0 size=160
    let mut pc: u32 = 0x825A75E0;
    'dispatch: loop {
        match pc {
            0x825A75E0 => {
    //   block [0x825A75E0..0x825A7680)
	// 825A75E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A75E4: 48C00B81  bl 0x831a8164
	ctx.lr = 0x825A75E8;
	sub_831A8130(ctx, base);
	// 825A75E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A75EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A75F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A75F4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825A75F8: 48032799  bl 0x825d9d90
	ctx.lr = 0x825A75FC;
	sub_825D9D90(ctx, base);
	// 825A75FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7604: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A7608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A760C: 4E800421  bctrl
	ctx.lr = 0x825A7610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7610: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 825A7614: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A7618: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A761C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7620: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7624: 419A0040  beq cr6, 0x825a7664
	if ctx.cr[6].eq {
	pc = 0x825A7664; continue 'dispatch;
	}
	// 825A7628: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A762C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A7630: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825A7634: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A7638: 4098002C  bge cr6, 0x825a7664
	if !ctx.cr[6].lt {
	pc = 0x825A7664; continue 'dispatch;
	}
	// 825A763C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A7640: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A7644: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 825A7648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A764C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825A7650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7654: 4E800421  bctrl
	ctx.lr = 0x825A7658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7658: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A765C: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 825A7660: 4BFFFFBC  b 0x825a761c
	pc = 0x825A761C; continue 'dispatch;
	// 825A7664: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825A7668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A766C: 48032795  bl 0x825d9e00
	ctx.lr = 0x825A7670;
	sub_825D9E00(ctx, base);
	// 825A7670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A7674: 48032755  bl 0x825d9dc8
	ctx.lr = 0x825A7678;
	sub_825D9DC8(ctx, base);
	// 825A7678: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A767C: 48C00B38  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7680 size=76
    let mut pc: u32 = 0x825A7680;
    'dispatch: loop {
        match pc {
            0x825A7680 => {
    //   block [0x825A7680..0x825A76CC)
	// 825A7680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A768C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7694: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A7698: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A769C: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A76A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A76A4: 4884BA4D  bl 0x82df30f0
	ctx.lr = 0x825A76A8;
	sub_82DF30F0(ctx, base);
	// 825A76A8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A76AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A76B0: 396BB15C  addi r11, r11, -0x4ea4
	ctx.r[11].s64 = ctx.r[11].s64 + -20132;
	// 825A76B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A76B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A76BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A76C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A76C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A76C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A76D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A76D0 size=76
    let mut pc: u32 = 0x825A76D0;
    'dispatch: loop {
        match pc {
            0x825A76D0 => {
    //   block [0x825A76D0..0x825A771C)
	// 825A76D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A76D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A76D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A76DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A76E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A76E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A76E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A76EC: 4BFFED25  bl 0x825a6410
	ctx.lr = 0x825A76F0;
	sub_825A6410(ctx, base);
	// 825A76F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A76F4: 4182000C  beq 0x825a7700
	if ctx.cr[0].eq {
	pc = 0x825A7700; continue 'dispatch;
	}
	// 825A76F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A76FC: 4BD18B6D  bl 0x822c0268
	ctx.lr = 0x825A7700;
	sub_822C0268(ctx, base);
	// 825A7700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A770C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7710: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A7714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A7718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7720 size=196
    let mut pc: u32 = 0x825A7720;
    'dispatch: loop {
        match pc {
            0x825A7720 => {
    //   block [0x825A7720..0x825A77E4)
	// 825A7720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A772C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7734: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A7738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A773C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825A7740: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A7744: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7748: 4BD191F1  bl 0x822c0938
	ctx.lr = 0x825A774C;
	sub_822C0938(ctx, base);
	// 825A774C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A7750: 41820028  beq 0x825a7778
	if ctx.cr[0].eq {
	pc = 0x825A7778; continue 'dispatch;
	}
	// 825A7754: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A7758: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825A775C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A7760: 392BB11C  addi r9, r11, -0x4ee4
	ctx.r[9].s64 = ctx.r[11].s64 + -20196;
	// 825A7764: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A7768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A776C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A7770: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A7774: 48000008  b 0x825a777c
	pc = 0x825A777C; continue 'dispatch;
	// 825A7778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A777C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7784: 409A0044  bne cr6, 0x825a77c8
	if !ctx.cr[6].eq {
	pc = 0x825A77C8; continue 'dispatch;
	}
	// 825A7788: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A778C: 419A001C  beq cr6, 0x825a77a8
	if ctx.cr[6].eq {
	pc = 0x825A77A8; continue 'dispatch;
	}
	// 825A7790: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7794: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A7798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A779C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A77A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A77A4: 4E800421  bctrl
	ctx.lr = 0x825A77A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A77A8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A77AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A77B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A77B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825A77B8: 816B800C  lwz r11, -0x7ff4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32756 as u32) ) } as u64;
	// 825A77BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A77C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A77C4: 4BD1883D  bl 0x822c0000
	ctx.lr = 0x825A77C8;
	sub_822C0000(ctx, base);
	// 825A77C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A77CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A77D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A77D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A77D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A77DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A77E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A77E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A77E8 size=196
    let mut pc: u32 = 0x825A77E8;
    'dispatch: loop {
        match pc {
            0x825A77E8 => {
    //   block [0x825A77E8..0x825A78AC)
	// 825A77E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A77EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A77F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A77F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A77F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A77FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A7800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A7804: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825A7808: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A780C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7810: 4BD19129  bl 0x822c0938
	ctx.lr = 0x825A7814;
	sub_822C0938(ctx, base);
	// 825A7814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A7818: 41820028  beq 0x825a7840
	if ctx.cr[0].eq {
	pc = 0x825A7840; continue 'dispatch;
	}
	// 825A781C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A7820: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825A7824: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A7828: 392BB130  addi r9, r11, -0x4ed0
	ctx.r[9].s64 = ctx.r[11].s64 + -20176;
	// 825A782C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A7830: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A7834: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A7838: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A783C: 48000008  b 0x825a7844
	pc = 0x825A7844; continue 'dispatch;
	// 825A7840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A7844: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A784C: 409A0044  bne cr6, 0x825a7890
	if !ctx.cr[6].eq {
	pc = 0x825A7890; continue 'dispatch;
	}
	// 825A7850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A7854: 419A001C  beq cr6, 0x825a7870
	if ctx.cr[6].eq {
	pc = 0x825A7870; continue 'dispatch;
	}
	// 825A7858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A785C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A7860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7864: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7868: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A786C: 4E800421  bctrl
	ctx.lr = 0x825A7870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7870: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A7874: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A7878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A787C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825A7880: 816B800C  lwz r11, -0x7ff4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32756 as u32) ) } as u64;
	// 825A7884: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A7888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A788C: 4BD18775  bl 0x822c0000
	ctx.lr = 0x825A7890;
	sub_822C0000(ctx, base);
	// 825A7890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A7894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A789C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A78A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A78A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A78A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A78B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A78B0 size=196
    let mut pc: u32 = 0x825A78B0;
    'dispatch: loop {
        match pc {
            0x825A78B0 => {
    //   block [0x825A78B0..0x825A7974)
	// 825A78B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A78B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A78B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A78BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A78C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A78C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A78C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A78CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825A78D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A78D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A78D8: 4BD19061  bl 0x822c0938
	ctx.lr = 0x825A78DC;
	sub_822C0938(ctx, base);
	// 825A78DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A78E0: 41820028  beq 0x825a7908
	if ctx.cr[0].eq {
	pc = 0x825A7908; continue 'dispatch;
	}
	// 825A78E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A78E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825A78EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825A78F0: 392BB144  addi r9, r11, -0x4ebc
	ctx.r[9].s64 = ctx.r[11].s64 + -20156;
	// 825A78F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A78F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A78FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A7900: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A7904: 48000008  b 0x825a790c
	pc = 0x825A790C; continue 'dispatch;
	// 825A7908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A790C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7914: 409A0044  bne cr6, 0x825a7958
	if !ctx.cr[6].eq {
	pc = 0x825A7958; continue 'dispatch;
	}
	// 825A7918: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A791C: 419A001C  beq cr6, 0x825a7938
	if ctx.cr[6].eq {
	pc = 0x825A7938; continue 'dispatch;
	}
	// 825A7920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7924: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825A7928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A792C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7934: 4E800421  bctrl
	ctx.lr = 0x825A7938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A7938: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A793C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A7940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A7944: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825A7948: 816B800C  lwz r11, -0x7ff4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32756 as u32) ) } as u64;
	// 825A794C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825A7950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825A7954: 4BD186AD  bl 0x822c0000
	ctx.lr = 0x825A7958;
	sub_822C0000(ctx, base);
	// 825A7958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A795C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A796C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A7970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7978 size=28
    let mut pc: u32 = 0x825A7978;
    'dispatch: loop {
        match pc {
            0x825A7978 => {
    //   block [0x825A7978..0x825A7994)
	// 825A7978: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A797C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7980: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825A7984: 409A0008  bne cr6, 0x825a798c
	if !ctx.cr[6].eq {
	pc = 0x825A798C; continue 'dispatch;
	}
	// 825A7988: 3943001C  addi r10, r3, 0x1c
	ctx.r[10].s64 = ctx.r[3].s64 + 28;
	// 825A798C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7990: 48000018  b 0x825a79a8
	sub_825A7994(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7994 size=36
    let mut pc: u32 = 0x825A7994;
    'dispatch: loop {
        match pc {
            0x825A7994 => {
    //   block [0x825A7994..0x825A79B8)
	// 825A7994: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7998: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825A799C: 409A0008  bne cr6, 0x825a79a4
	if !ctx.cr[6].eq {
	pc = 0x825A79A4; continue 'dispatch;
	}
	// 825A79A0: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825A79A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A79A8: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A79AC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A79B0: 409AFFE4  bne cr6, 0x825a7994
	if !ctx.cr[6].eq {
	pc = 0x825A7994; continue 'dispatch;
	}
	// 825A79B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A79B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A79B8 size=92
    let mut pc: u32 = 0x825A79B8;
    'dispatch: loop {
        match pc {
            0x825A79B8 => {
    //   block [0x825A79B8..0x825A7A14)
	// 825A79B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A79BC: 48C007AD  bl 0x831a8168
	ctx.lr = 0x825A79C0;
	sub_831A8130(ctx, base);
	// 825A79C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A79C4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825A79C8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825A79CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A79D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A79D4: 7F1F2840  cmplw cr6, r31, r5
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[5].u32, &mut ctx.xer);
	// 825A79D8: 419A002C  beq cr6, 0x825a7a04
	if ctx.cr[6].eq {
	pc = 0x825A7A04; continue 'dispatch;
	}
	// 825A79DC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A79E0: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 825A79E4: 7D6B0E71  srawi. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A79E8: 5566083C  slwi r6, r11, 1
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 825A79EC: 7FA6FA14  add r29, r6, r31
	ctx.r[29].u64 = ctx.r[6].u64 + ctx.r[31].u64;
	// 825A79F0: 40810010  ble 0x825a7a00
	if !ctx.cr[0].gt {
	pc = 0x825A7A00; continue 'dispatch;
	}
	// 825A79F4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 825A79F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A79FC: 48C0130D  bl 0x831a8d08
	ctx.lr = 0x825A7A00;
	sub_831A8D08(ctx, base);
	// 825A7A00: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825A7A04: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A7A08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A7A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A7A10: 48C007A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7A18 size=72
    let mut pc: u32 = 0x825A7A18;
    'dispatch: loop {
        match pc {
            0x825A7A18 => {
    //   block [0x825A7A18..0x825A7A60)
	// 825A7A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7A24: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825A7A28: 419A001C  beq cr6, 0x825a7a44
	if ctx.cr[6].eq {
	pc = 0x825A7A44; continue 'dispatch;
	}
	// 825A7A2C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A7A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A7A34: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A7A38: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A7A3C: 4BFFF8BD  bl 0x825a72f8
	ctx.lr = 0x825A7A40;
	sub_825A72F8(ctx, base);
	// 825A7A40: 48000010  b 0x825a7a50
	pc = 0x825A7A50; continue 'dispatch;
	// 825A7A44: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A7A48: 396B80F8  addi r11, r11, -0x7f08
	ctx.r[11].s64 = ctx.r[11].s64 + -32520;
	// 825A7A4C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A7A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7A60 size=72
    let mut pc: u32 = 0x825A7A60;
    'dispatch: loop {
        match pc {
            0x825A7A60 => {
    //   block [0x825A7A60..0x825A7AA8)
	// 825A7A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7A6C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825A7A70: 419A001C  beq cr6, 0x825a7a8c
	if ctx.cr[6].eq {
	pc = 0x825A7A8C; continue 'dispatch;
	}
	// 825A7A74: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A7A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A7A7C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825A7A80: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A7A84: 4BFFF8FD  bl 0x825a7380
	ctx.lr = 0x825A7A88;
	sub_825A7380(ctx, base);
	// 825A7A88: 48000010  b 0x825a7a98
	pc = 0x825A7A98; continue 'dispatch;
	// 825A7A8C: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A7A90: 396B81D0  addi r11, r11, -0x7e30
	ctx.r[11].s64 = ctx.r[11].s64 + -32304;
	// 825A7A94: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A7A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A7A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7AA8 size=72
    let mut pc: u32 = 0x825A7AA8;
    'dispatch: loop {
        match pc {
            0x825A7AA8 => {
    //   block [0x825A7AA8..0x825A7AF0)
	// 825A7AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A7AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7AC0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825A7AC4: 4884C13D  bl 0x82df3c00
	ctx.lr = 0x825A7AC8;
	sub_82DF3C00(ctx, base);
	// 825A7AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A7ACC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 825A7AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7AD4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A7AD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7AE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A7AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A7AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7AF0 size=28
    let mut pc: u32 = 0x825A7AF0;
    'dispatch: loop {
        match pc {
            0x825A7AF0 => {
    //   block [0x825A7AF0..0x825A7B0C)
	// 825A7AF0: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A7AF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A7AF8: 409A0008  bne cr6, 0x825a7b00
	if !ctx.cr[6].eq {
	pc = 0x825A7B00; continue 'dispatch;
	}
	// 825A7AFC: 3943001C  addi r10, r3, 0x1c
	ctx.r[10].s64 = ctx.r[3].s64 + 28;
	// 825A7B00: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7B04: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7B08: 48000014  b 0x825a7b1c
	sub_825A7B0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7B0C size=32
    let mut pc: u32 = 0x825A7B0C;
    'dispatch: loop {
        match pc {
            0x825A7B0C => {
    //   block [0x825A7B0C..0x825A7B2C)
	// 825A7B0C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7B10: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825A7B14: 419A0018  beq cr6, 0x825a7b2c
	if ctx.cr[6].eq {
		sub_825A7B2C(ctx, base);
		return;
	}
	// 825A7B18: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A7B1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A7B20: 409AFFEC  bne cr6, 0x825a7b0c
	if !ctx.cr[6].eq {
	pc = 0x825A7B0C; continue 'dispatch;
	}
	// 825A7B24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A7B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7B2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7B2C size=8
    let mut pc: u32 = 0x825A7B2C;
    'dispatch: loop {
        match pc {
            0x825A7B2C => {
    //   block [0x825A7B2C..0x825A7B34)
	// 825A7B2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A7B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7B38 size=88
    let mut pc: u32 = 0x825A7B38;
    'dispatch: loop {
        match pc {
            0x825A7B38 => {
    //   block [0x825A7B38..0x825A7B90)
	// 825A7B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7B3C: 48C00631  bl 0x831a816c
	ctx.lr = 0x825A7B40;
	sub_831A8130(ctx, base);
	// 825A7B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7B44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A7B48: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825A7B4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A7B50: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 825A7B54: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7B58: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 825A7B5C: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A7B60: 40810014  ble 0x825a7b74
	if !ctx.cr[0].gt {
	pc = 0x825A7B74; continue 'dispatch;
	}
	// 825A7B64: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 825A7B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7B6C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 825A7B70: 48C01199  bl 0x831a8d08
	ctx.lr = 0x825A7B74;
	sub_831A8D08(ctx, base);
	// 825A7B74: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7B78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A7B7C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A7B80: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 825A7B84: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A7B88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7B8C: 48C00630  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7B90 size=68
    let mut pc: u32 = 0x825A7B90;
    'dispatch: loop {
        match pc {
            0x825A7B90 => {
    //   block [0x825A7B90..0x825A7BD4)
	// 825A7B90: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A7B94: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825A7B98: 409A0008  bne cr6, 0x825a7ba0
	if !ctx.cr[6].eq {
	pc = 0x825A7BA0; continue 'dispatch;
	}
	// 825A7B9C: 3903001C  addi r8, r3, 0x1c
	ctx.r[8].s64 = ctx.r[3].s64 + 28;
	// 825A7BA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7BA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A7BA8: 81280008  lwz r9, 8(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7BAC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7BB0: 419A0064  beq cr6, 0x825a7c14
	if ctx.cr[6].eq {
		sub_825A7BD4(ctx, base);
		return;
	}
	// 825A7BB4: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A7BB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A7BBC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825A7BC0: 419A0014  beq cr6, 0x825a7bd4
	if ctx.cr[6].eq {
		sub_825A7BD4(ctx, base);
		return;
	}
	// 825A7BC4: 80E80008  lwz r7, 8(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7BC8: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 825A7BCC: 409AFFE8  bne cr6, 0x825a7bb4
	if !ctx.cr[6].eq {
	pc = 0x825A7BB4; continue 'dispatch;
	}
	// 825A7BD0: 48000044  b 0x825a7c14
	sub_825A7BD4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7BD4 size=80
    let mut pc: u32 = 0x825A7BD4;
    'dispatch: loop {
        match pc {
            0x825A7BD4 => {
    //   block [0x825A7BD4..0x825A7C24)
	// 825A7BD4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7BD8: 419A0030  beq cr6, 0x825a7c08
	if ctx.cr[6].eq {
	pc = 0x825A7C08; continue 'dispatch;
	}
	// 825A7BDC: 394BFFF4  addi r10, r11, -0xc
	ctx.r[10].s64 = ctx.r[11].s64 + -12;
	// 825A7BE0: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7BE4: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 825A7BE8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7BEC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 825A7BF0: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7BF4: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825A7BF8: 90EA0008  stw r7, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 825A7BFC: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 825A7C00: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7C04: 409AFFDC  bne cr6, 0x825a7be0
	if !ctx.cr[6].eq {
	pc = 0x825A7BE0; continue 'dispatch;
	}
	// 825A7C08: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7C0C: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 825A7C10: 91680008  stw r11, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A7C14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7C18: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A7C1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7C20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7C28 size=72
    let mut pc: u32 = 0x825A7C28;
    'dispatch: loop {
        match pc {
            0x825A7C28 => {
    //   block [0x825A7C28..0x825A7C70)
	// 825A7C28: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A7C2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7C30: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 825A7C34: 409A0008  bne cr6, 0x825a7c3c
	if !ctx.cr[6].eq {
	pc = 0x825A7C3C; continue 'dispatch;
	}
	// 825A7C38: 3903001C  addi r8, r3, 0x1c
	ctx.r[8].s64 = ctx.r[3].s64 + 28;
	// 825A7C3C: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A7C44: 81280008  lwz r9, 8(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7C48: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7C4C: 419A0054  beq cr6, 0x825a7ca0
	if ctx.cr[6].eq {
		sub_825A7C70(ctx, base);
		return;
	}
	// 825A7C50: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A7C54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A7C58: 419A0018  beq cr6, 0x825a7c70
	if ctx.cr[6].eq {
		sub_825A7C70(ctx, base);
		return;
	}
	// 825A7C5C: 80E80008  lwz r7, 8(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7C60: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825A7C64: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 825A7C68: 409AFFE8  bne cr6, 0x825a7c50
	if !ctx.cr[6].eq {
	pc = 0x825A7C50; continue 'dispatch;
	}
	// 825A7C6C: 48000034  b 0x825a7ca0
	sub_825A7C70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7C70 size=64
    let mut pc: u32 = 0x825A7C70;
    'dispatch: loop {
        match pc {
            0x825A7C70 => {
    //   block [0x825A7C70..0x825A7CB0)
	// 825A7C70: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 825A7C74: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7C78: 419A001C  beq cr6, 0x825a7c94
	if ctx.cr[6].eq {
	pc = 0x825A7C94; continue 'dispatch;
	}
	// 825A7C7C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 825A7C80: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7C84: 7CEB512E  stwx r7, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u32) };
	// 825A7C88: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 825A7C8C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7C90: 409AFFF0  bne cr6, 0x825a7c80
	if !ctx.cr[6].eq {
	pc = 0x825A7C80; continue 'dispatch;
	}
	// 825A7C94: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7C98: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 825A7C9C: 91680008  stw r11, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A7CA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7CA4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A7CA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A7CAC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7CB0 size=16
    let mut pc: u32 = 0x825A7CB0;
    'dispatch: loop {
        match pc {
            0x825A7CB0 => {
    //   block [0x825A7CB0..0x825A7CC0)
	// 825A7CB0: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 825A7CB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7CB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7CBC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7CC0 size=16
    let mut pc: u32 = 0x825A7CC0;
    'dispatch: loop {
        match pc {
            0x825A7CC0 => {
    //   block [0x825A7CC0..0x825A7CD0)
	// 825A7CC0: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A7CC4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A7CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7CCC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A7CD0 size=8
    let mut pc: u32 = 0x825A7CD0;
    'dispatch: loop {
        match pc {
            0x825A7CD0 => {
    //   block [0x825A7CD0..0x825A7CD8)
	// 825A7CD0: 4BEAB9B0  b 0x82453680
	sub_82453680(ctx, base);
	return;
	// 825A7CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7CD8 size=156
    let mut pc: u32 = 0x825A7CD8;
    'dispatch: loop {
        match pc {
            0x825A7CD8 => {
    //   block [0x825A7CD8..0x825A7D74)
	// 825A7CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A7CE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A7CE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A7CE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7CEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A7CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A7CF4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A7CF8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825A7CFC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A7D00: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A7D04: 409A000C  bne cr6, 0x825a7d10
	if !ctx.cr[6].eq {
	pc = 0x825A7D10; continue 'dispatch;
	}
	// 825A7D08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825A7D0C: 48000050  b 0x825a7d5c
	pc = 0x825A7D5C; continue 'dispatch;
	// 825A7D10: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 825A7D14: 616B5555  ori r11, r11, 0x5555
	ctx.r[11].u64 = ctx.r[11].u64 | 21845;
	// 825A7D18: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A7D1C: 4099000C  ble cr6, 0x825a7d28
	if !ctx.cr[6].gt {
	pc = 0x825A7D28; continue 'dispatch;
	}
	// 825A7D20: 48610001  bl 0x82bb7d20
	ctx.lr = 0x825A7D24;
	sub_82BB7D20(ctx, base);
	// 825A7D24: 48000034  b 0x825a7d58
	pc = 0x825A7D58; continue 'dispatch;
	// 825A7D28: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825A7D2C: 1FC4000C  mulli r30, r4, 0xc
	ctx.r[30].s64 = ctx.r[4].s64 * 12;
	// 825A7D30: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825A7D34: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A7D38: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825A7D3C: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 825A7D40: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 825A7D44: 4884A385  bl 0x82df20c8
	ctx.lr = 0x825A7D48;
	sub_82DF20C8(ctx, base);
	// 825A7D48: 7D7E1A14  add r11, r30, r3
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 825A7D4C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825A7D50: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 825A7D54: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A7D58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825A7D5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A7D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A7D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A7D68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A7D6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A7D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7D78 size=128
    let mut pc: u32 = 0x825A7D78;
    'dispatch: loop {
        match pc {
            0x825A7D78 => {
    //   block [0x825A7D78..0x825A7DF8)
	// 825A7D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7D7C: 48C003F1  bl 0x831a816c
	ctx.lr = 0x825A7D80;
	sub_831A8130(ctx, base);
	// 825A7D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7D84: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825A7D88: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825A7D8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A7D90: 3BEB7B70  addi r31, r11, 0x7b70
	ctx.r[31].s64 = ctx.r[11].s64 + 31600;
	// 825A7D94: 816A7B78  lwz r11, 0x7b78(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31608 as u32) ) } as u64;
	// 825A7D98: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825A7D9C: 40820024  bne 0x825a7dc0
	if !ctx.cr[0].eq {
	pc = 0x825A7DC0; continue 'dispatch;
	}
	// 825A7DA0: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 825A7DA4: 3D00825A  lis r8, -0x7da6
	ctx.r[8].s64 = -2108030976;
	// 825A7DA8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825A7DAC: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 825A7DB0: 39087A18  addi r8, r8, 0x7a18
	ctx.r[8].s64 = ctx.r[8].s64 + 31256;
	// 825A7DB4: 916A7B78  stw r11, 0x7b78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31608 as u32), ctx.r[11].u32 ) };
	// 825A7DB8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825A7DBC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825A7DC0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A7DC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A7DC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7DCC: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 825A7DD0: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825A7DD4: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A7DD8: 4802AB49  bl 0x825d2920
	ctx.lr = 0x825A7DDC;
	sub_825D2920(ctx, base);
	// 825A7DDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A7DE0: 4182000C  beq 0x825a7dec
	if ctx.cr[0].eq {
	pc = 0x825A7DEC; continue 'dispatch;
	}
	// 825A7DE4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A7DE8: 48000008  b 0x825a7df0
	pc = 0x825A7DF0; continue 'dispatch;
	// 825A7DEC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A7DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A7DF4: 48C003C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7DF8 size=128
    let mut pc: u32 = 0x825A7DF8;
    'dispatch: loop {
        match pc {
            0x825A7DF8 => {
    //   block [0x825A7DF8..0x825A7E78)
	// 825A7DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7DFC: 48C00371  bl 0x831a816c
	ctx.lr = 0x825A7E00;
	sub_831A8130(ctx, base);
	// 825A7E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7E04: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825A7E08: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825A7E0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A7E10: 3BEB7B7C  addi r31, r11, 0x7b7c
	ctx.r[31].s64 = ctx.r[11].s64 + 31612;
	// 825A7E14: 816A7B84  lwz r11, 0x7b84(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31620 as u32) ) } as u64;
	// 825A7E18: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825A7E1C: 40820024  bne 0x825a7e40
	if !ctx.cr[0].eq {
	pc = 0x825A7E40; continue 'dispatch;
	}
	// 825A7E20: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 825A7E24: 3D00825A  lis r8, -0x7da6
	ctx.r[8].s64 = -2108030976;
	// 825A7E28: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825A7E2C: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 825A7E30: 39087A60  addi r8, r8, 0x7a60
	ctx.r[8].s64 = ctx.r[8].s64 + 31328;
	// 825A7E34: 916A7B84  stw r11, 0x7b84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31620 as u32), ctx.r[11].u32 ) };
	// 825A7E38: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825A7E3C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825A7E40: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A7E44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A7E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A7E4C: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 825A7E50: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825A7E54: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A7E58: 4802AAC9  bl 0x825d2920
	ctx.lr = 0x825A7E5C;
	sub_825D2920(ctx, base);
	// 825A7E5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A7E60: 4182000C  beq 0x825a7e6c
	if ctx.cr[0].eq {
	pc = 0x825A7E6C; continue 'dispatch;
	}
	// 825A7E64: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A7E68: 48000008  b 0x825a7e70
	pc = 0x825A7E70; continue 'dispatch;
	// 825A7E6C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825A7E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A7E74: 48C00348  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A7E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A7E78 size=528
    let mut pc: u32 = 0x825A7E78;
    'dispatch: loop {
        match pc {
            0x825A7E78 => {
    //   block [0x825A7E78..0x825A8088)
	// 825A7E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A7E7C: 48C002F1  bl 0x831a816c
	ctx.lr = 0x825A7E80;
	sub_831A8130(ctx, base);
	// 825A7E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A7E84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A7E88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A7E8C: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 825A7E90: 419A01EC  beq cr6, 0x825a807c
	if ctx.cr[6].eq {
	pc = 0x825A807C; continue 'dispatch;
	}
	// 825A7E94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7E9C: 419A0018  beq cr6, 0x825a7eb4
	if ctx.cr[6].eq {
	pc = 0x825A7EB4; continue 'dispatch;
	}
	// 825A7EA0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7EA4: 3BC0000C  li r30, 0xc
	ctx.r[30].s64 = 12;
	// 825A7EA8: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 825A7EAC: 7D0AF3D7  divw. r8, r10, r30
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[30].s32;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825A7EB0: 40820010  bne 0x825a7ec0
	if !ctx.cr[0].eq {
	pc = 0x825A7EC0; continue 'dispatch;
	}
	// 825A7EB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A7EB8: 4815DEC9  bl 0x82705d80
	ctx.lr = 0x825A7EBC;
	sub_82705D80(ctx, base);
	// 825A7EBC: 480001C0  b 0x825a807c
	pc = 0x825A807C; continue 'dispatch;
	// 825A7EC0: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7EC4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A7EC8: 409A000C  bne cr6, 0x825a7ed4
	if !ctx.cr[6].eq {
	pc = 0x825A7ED4; continue 'dispatch;
	}
	// 825A7ECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A7ED0: 48000010  b 0x825a7ee0
	pc = 0x825A7EE0; continue 'dispatch;
	// 825A7ED4: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7ED8: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 825A7EDC: 7D4AF3D6  divw r10, r10, r30
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[30].s32;
	// 825A7EE0: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A7EE4: 40990034  ble cr6, 0x825a7f18
	if !ctx.cr[6].gt {
	pc = 0x825A7F18; continue 'dispatch;
	}
	// 825A7EE8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A7EEC: 409A005C  bne cr6, 0x825a7f48
	if !ctx.cr[6].eq {
	pc = 0x825A7F48; continue 'dispatch;
	}
	// 825A7EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A7EF4: 48000060  b 0x825a7f54
	pc = 0x825A7F54; continue 'dispatch;
	// 825A7EF8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7EFC: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A7F00: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7F04: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825A7F08: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7F0C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825A7F10: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A7F14: 3884000C  addi r4, r4, 0xc
	ctx.r[4].s64 = ctx.r[4].s64 + 12;
	// 825A7F18: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7F1C: 409AFFDC  bne cr6, 0x825a7ef8
	if !ctx.cr[6].eq {
	pc = 0x825A7EF8; continue 'dispatch;
	}
	// 825A7F20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7F24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7F28: 419A0010  beq cr6, 0x825a7f38
	if ctx.cr[6].eq {
	pc = 0x825A7F38; continue 'dispatch;
	}
	// 825A7F2C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7F30: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A7F34: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 825A7F38: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7F3C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 825A7F40: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825A7F44: 480000A8  b 0x825a7fec
	pc = 0x825A7FEC; continue 'dispatch;
	// 825A7F48: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A7F4C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 825A7F50: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 825A7F54: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A7F58: 4199009C  bgt cr6, 0x825a7ff4
	if ctx.cr[6].gt {
	pc = 0x825A7FF4; continue 'dispatch;
	}
	// 825A7F5C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A7F60: 409A000C  bne cr6, 0x825a7f6c
	if !ctx.cr[6].eq {
	pc = 0x825A7F6C; continue 'dispatch;
	}
	// 825A7F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A7F68: 48000010  b 0x825a7f78
	pc = 0x825A7F78; continue 'dispatch;
	// 825A7F6C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7F70: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 825A7F74: 7D4BF3D6  divw r10, r11, r30
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 825A7F78: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7F7C: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 * 12;
	// 825A7F80: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825A7F84: 48000024  b 0x825a7fa8
	pc = 0x825A7FA8; continue 'dispatch;
	// 825A7F88: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7F8C: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A7F90: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7F94: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825A7F98: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7F9C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825A7FA0: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825A7FA4: 3884000C  addi r4, r4, 0xc
	ctx.r[4].s64 = ctx.r[4].s64 + 12;
	// 825A7FA8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825A7FAC: 409AFFDC  bne cr6, 0x825a7f88
	if !ctx.cr[6].eq {
	pc = 0x825A7F88; continue 'dispatch;
	}
	// 825A7FB0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7FB4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7FB8: 4800002C  b 0x825a7fe4
	pc = 0x825A7FE4; continue 'dispatch;
	// 825A7FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A7FC0: 419A001C  beq cr6, 0x825a7fdc
	if ctx.cr[6].eq {
	pc = 0x825A7FDC; continue 'dispatch;
	}
	// 825A7FC4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A7FC8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825A7FCC: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A7FD0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825A7FD4: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A7FD8: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 825A7FDC: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 825A7FE0: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825A7FE4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A7FE8: 409AFFD4  bne cr6, 0x825a7fbc
	if !ctx.cr[6].eq {
	pc = 0x825A7FBC; continue 'dispatch;
	}
	// 825A7FEC: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A7FF0: 4800008C  b 0x825a807c
	pc = 0x825A807C; continue 'dispatch;
	// 825A7FF4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A7FF8: 419A0010  beq cr6, 0x825a8008
	if ctx.cr[6].eq {
	pc = 0x825A8008; continue 'dispatch;
	}
	// 825A7FFC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825A8000: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825A8004: 4884A185  bl 0x82df2188
	ctx.lr = 0x825A8008;
	sub_82DF2188(ctx, base);
	// 825A8008: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A800C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8010: 409A000C  bne cr6, 0x825a801c
	if !ctx.cr[6].eq {
	pc = 0x825A801C; continue 'dispatch;
	}
	// 825A8014: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A8018: 48000010  b 0x825a8028
	pc = 0x825A8028; continue 'dispatch;
	// 825A801C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8020: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A8024: 7C8BF3D6  divw r4, r11, r30
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 825A8028: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A802C: 4BFFFCAD  bl 0x825a7cd8
	ctx.lr = 0x825A8030;
	sub_825A7CD8(ctx, base);
	// 825A8030: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A8034: 41820048  beq 0x825a807c
	if ctx.cr[0].eq {
	pc = 0x825A807C; continue 'dispatch;
	}
	// 825A8038: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A803C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8040: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8044: 4800002C  b 0x825a8070
	pc = 0x825A8070; continue 'dispatch;
	// 825A8048: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825A804C: 419A001C  beq cr6, 0x825a8068
	if ctx.cr[6].eq {
	pc = 0x825A8068; continue 'dispatch;
	}
	// 825A8050: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8054: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825A8058: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A805C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825A8060: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8064: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 825A8068: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 825A806C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 825A8070: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825A8074: 409AFFD4  bne cr6, 0x825a8048
	if !ctx.cr[6].eq {
	pc = 0x825A8048; continue 'dispatch;
	}
	// 825A8078: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A807C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A8084: 48C00138  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8088 size=132
    let mut pc: u32 = 0x825A8088;
    'dispatch: loop {
        match pc {
            0x825A8088 => {
    //   block [0x825A8088..0x825A810C)
	// 825A8088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A808C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A8094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8098: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A809C: 3D60825A  lis r11, -0x7da6
	ctx.r[11].s64 = -2108030976;
	// 825A80A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A80A4: 396B75E0  addi r11, r11, 0x75e0
	ctx.r[11].s64 = ctx.r[11].s64 + 30176;
	// 825A80A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A80AC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825A80B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A80B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A80B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A80BC: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A80C0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A80C4: 4BFFFCB5  bl 0x825a7d78
	ctx.lr = 0x825A80C8;
	sub_825A7D78(ctx, base);
	// 825A80C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A80CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A80D0: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825A80D4: 4BFFE3B5  bl 0x825a6488
	ctx.lr = 0x825A80D8;
	sub_825A6488(ctx, base);
	// 825A80D8: 4884B0D9  bl 0x82df31b0
	ctx.lr = 0x825A80DC;
	sub_82DF31B0(ctx, base);
	// 825A80DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A80E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A80E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A80E8: 48031DF9  bl 0x825d9ee0
	ctx.lr = 0x825A80EC;
	sub_825D9EE0(ctx, base);
	// 825A80EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A80F0: 4884B339  bl 0x82df3428
	ctx.lr = 0x825A80F4;
	sub_82DF3428(ctx, base);
	// 825A80F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A80F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A80FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8100: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A8104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8110 size=132
    let mut pc: u32 = 0x825A8110;
    'dispatch: loop {
        match pc {
            0x825A8110 => {
    //   block [0x825A8110..0x825A8194)
	// 825A8110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A811C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8120: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8124: 3D60825A  lis r11, -0x7da6
	ctx.r[11].s64 = -2108030976;
	// 825A8128: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A812C: 396B75E0  addi r11, r11, 0x75e0
	ctx.r[11].s64 = ctx.r[11].s64 + 30176;
	// 825A8130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A8134: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825A8138: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A813C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A8140: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A8144: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A8148: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A814C: 4BFFFCAD  bl 0x825a7df8
	ctx.lr = 0x825A8150;
	sub_825A7DF8(ctx, base);
	// 825A8150: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8158: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825A815C: 4BFFE32D  bl 0x825a6488
	ctx.lr = 0x825A8160;
	sub_825A6488(ctx, base);
	// 825A8160: 4884B051  bl 0x82df31b0
	ctx.lr = 0x825A8164;
	sub_82DF31B0(ctx, base);
	// 825A8164: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A816C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8170: 48031D71  bl 0x825d9ee0
	ctx.lr = 0x825A8174;
	sub_825D9EE0(ctx, base);
	// 825A8174: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8178: 4884B2B1  bl 0x82df3428
	ctx.lr = 0x825A817C;
	sub_82DF3428(ctx, base);
	// 825A817C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A8180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8188: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A818C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8198 size=88
    let mut pc: u32 = 0x825A8198;
    'dispatch: loop {
        match pc {
            0x825A8198 => {
    //   block [0x825A8198..0x825A81F0)
	// 825A8198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A819C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A81A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A81A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A81A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A81AC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825A81B0: 4BD20B09  bl 0x822c8cb8
	ctx.lr = 0x825A81B4;
	sub_822C8CB8(ctx, base);
	// 825A81B4: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825A81B8: 4884B271  bl 0x82df3428
	ctx.lr = 0x825A81BC;
	sub_82DF3428(ctx, base);
	// 825A81BC: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 825A81C0: 4BEC1E41  bl 0x8246a000
	ctx.lr = 0x825A81C4;
	sub_8246A000(ctx, base);
	// 825A81C4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 825A81C8: 4BEC1E39  bl 0x8246a000
	ctx.lr = 0x825A81CC;
	sub_8246A000(ctx, base);
	// 825A81CC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825A81D0: 4BF090D1  bl 0x824b12a0
	ctx.lr = 0x825A81D4;
	sub_824B12A0(ctx, base);
	// 825A81D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A81D8: 4BFFE239  bl 0x825a6410
	ctx.lr = 0x825A81DC;
	sub_825A6410(ctx, base);
	// 825A81DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A81E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A81E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A81E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A81EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A81F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A81F0 size=156
    let mut pc: u32 = 0x825A81F0;
    'dispatch: loop {
        match pc {
            0x825A81F0 => {
    //   block [0x825A81F0..0x825A828C)
	// 825A81F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A81F4: 48BFFF71  bl 0x831a8164
	ctx.lr = 0x825A81F8;
	sub_831A8130(ctx, base);
	// 825A81F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A81FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8200: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A8204: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825A8208: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825A820C: 4BFFF475  bl 0x825a7680
	ctx.lr = 0x825A8210;
	sub_825A7680(ctx, base);
	// 825A8210: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825A8214: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A8218: 394AB214  addi r10, r10, -0x4dec
	ctx.r[10].s64 = ctx.r[10].s64 + -19948;
	// 825A821C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A8220: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A8224: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825A8228: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A822C: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 825A8230: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A8234: 3BBF002C  addi r29, r31, 0x2c
	ctx.r[29].s64 = ctx.r[31].s64 + 44;
	// 825A8238: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A823C: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 825A8240: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A8244: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825A8248: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825A824C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825A8250: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825A8254: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825A8258: 4884B9A9  bl 0x82df3c00
	ctx.lr = 0x825A825C;
	sub_82DF3C00(ctx, base);
	// 825A825C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825A8260: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825A8264: 4BFE3CAD  bl 0x8258bf10
	ctx.lr = 0x825A8268;
	sub_8258BF10(ctx, base);
	// 825A8268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A826C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8270: 4BFFFC09  bl 0x825a7e78
	ctx.lr = 0x825A8274;
	sub_825A7E78(ctx, base);
	// 825A8274: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8278: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A827C: 4BFFFBFD  bl 0x825a7e78
	ctx.lr = 0x825A8280;
	sub_825A7E78(ctx, base);
	// 825A8280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A8288: 48BFFF2C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8290 size=24
    let mut pc: u32 = 0x825A8290;
    'dispatch: loop {
        match pc {
            0x825A8290 => {
    //   block [0x825A8290..0x825A82A8)
	// 825A8290: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8294: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A8298: 409A0008  bne cr6, 0x825a82a0
	if !ctx.cr[6].eq {
	pc = 0x825A82A0; continue 'dispatch;
	}
	// 825A829C: 3883001C  addi r4, r3, 0x1c
	ctx.r[4].s64 = ctx.r[3].s64 + 28;
	// 825A82A0: 3863002C  addi r3, r3, 0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + 44;
	// 825A82A4: 4BFFFBD4  b 0x825a7e78
	sub_825A7E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A82A8 size=232
    let mut pc: u32 = 0x825A82A8;
    'dispatch: loop {
        match pc {
            0x825A82A8 => {
    //   block [0x825A82A8..0x825A8390)
	// 825A82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A82AC: 48BFFEC1  bl 0x831a816c
	ctx.lr = 0x825A82B0;
	sub_831A8130(ctx, base);
	// 825A82B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A82B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A82B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A82BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A82C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A82C4: 388BB1D0  addi r4, r11, -0x4e30
	ctx.r[4].s64 = ctx.r[11].s64 + -20016;
	// 825A82C8: 38A00086  li r5, 0x86
	ctx.r[5].s64 = 134;
	// 825A82CC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A82D0: 4BD18109  bl 0x822c03d8
	ctx.lr = 0x825A82D4;
	sub_822C03D8(ctx, base);
	// 825A82D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A82D8: 4182001C  beq 0x825a82f4
	if ctx.cr[0].eq {
	pc = 0x825A82F4; continue 'dispatch;
	}
	// 825A82DC: 38DE0040  addi r6, r30, 0x40
	ctx.r[6].s64 = ctx.r[30].s64 + 64;
	// 825A82E0: 80BE0018  lwz r5, 0x18(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A82E4: 389E003C  addi r4, r30, 0x3c
	ctx.r[4].s64 = ctx.r[30].s64 + 60;
	// 825A82E8: 4BFFFF09  bl 0x825a81f0
	ctx.lr = 0x825A82EC;
	sub_825A81F0(ctx, base);
	// 825A82EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A82F0: 48000008  b 0x825a82f8
	pc = 0x825A82F8; continue 'dispatch;
	// 825A82F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A82F8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825A82FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8300: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A8304: 4BFF8F4D  bl 0x825a1250
	ctx.lr = 0x825A8308;
	sub_825A1250(ctx, base);
	// 825A8308: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A830C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8310: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A8314: 4BD17CED  bl 0x822c0000
	ctx.lr = 0x825A8318;
	sub_822C0000(ctx, base);
	// 825A8318: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A831C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A8320: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825A8324: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A8328: 4BD20991  bl 0x822c8cb8
	ctx.lr = 0x825A832C;
	sub_822C8CB8(ctx, base);
	// 825A832C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A8330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8334: 4BFFE155  bl 0x825a6488
	ctx.lr = 0x825A8338;
	sub_825A6488(ctx, base);
	// 825A8338: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A833C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8340: 4822D729  bl 0x827d5a68
	ctx.lr = 0x825A8344;
	sub_827D5A68(ctx, base);
	// 825A8344: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8348: 4884B0E1  bl 0x82df3428
	ctx.lr = 0x825A834C;
	sub_82DF3428(ctx, base);
	// 825A834C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A8350: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A8354: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8358: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825A835C: 419A0028  beq cr6, 0x825a8384
	if ctx.cr[6].eq {
	pc = 0x825A8384; continue 'dispatch;
	}
	// 825A8360: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825A8364: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825A8368: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A836C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825A8370: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A8374: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825A8378: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A837C: 4082FFE8  bne 0x825a8364
	if !ctx.cr[0].eq {
	pc = 0x825A8364; continue 'dispatch;
	}
	// 825A8380: 4BD18511  bl 0x822c0890
	ctx.lr = 0x825A8384;
	sub_822C0890(ctx, base);
	// 825A8384: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A838C: 48BFFE30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A8390 size=116
    let mut pc: u32 = 0x825A8390;
    'dispatch: loop {
        match pc {
            0x825A8390 => {
    //   block [0x825A8390..0x825A8404)
	// 825A8390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A839C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A83A0: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A83A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A83A8: 409A0008  bne cr6, 0x825a83b0
	if !ctx.cr[6].eq {
	pc = 0x825A83B0; continue 'dispatch;
	}
	// 825A83AC: 3BE3001C  addi r31, r3, 0x1c
	ctx.r[31].s64 = ctx.r[3].s64 + 28;
	// 825A83B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825A83B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A83B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A83BC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A83C0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A83C4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825A83C8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 825A83CC: 481F477D  bl 0x8279cb48
	ctx.lr = 0x825A83D0;
	sub_8279CB48(ctx, base);
	// 825A83D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A83D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A83D8: 419A0014  beq cr6, 0x825a83ec
	if ctx.cr[6].eq {
	pc = 0x825A83EC; continue 'dispatch;
	}
	// 825A83DC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A83E0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825A83E4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A83E8: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 825A83EC: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 825A83F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A83F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A83F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A83FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8408 size=80
    let mut pc: u32 = 0x825A8408;
    'dispatch: loop {
        match pc {
            0x825A8408 => {
    //   block [0x825A8408..0x825A8458)
	// 825A8408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A840C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A8414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A841C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A8420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A8424: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A842C: 419A0008  beq cr6, 0x825a8434
	if ctx.cr[6].eq {
	pc = 0x825A8434; continue 'dispatch;
	}
	// 825A8430: 4BFFFA49  bl 0x825a7e78
	ctx.lr = 0x825A8434;
	sub_825A7E78(ctx, base);
	// 825A8434: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8438: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 825A843C: 4BFFFA3D  bl 0x825a7e78
	ctx.lr = 0x825A8440;
	sub_825A7E78(ctx, base);
	// 825A8440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A8444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A844C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A8450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8458 size=144
    let mut pc: u32 = 0x825A8458;
    'dispatch: loop {
        match pc {
            0x825A8458 => {
    //   block [0x825A8458..0x825A84E8)
	// 825A8458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A845C: 48BFFD09  bl 0x831a8164
	ctx.lr = 0x825A8460;
	sub_831A8130(ctx, base);
	// 825A8460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8468: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A846C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825A8470: 4BFFF211  bl 0x825a7680
	ctx.lr = 0x825A8474;
	sub_825A7680(ctx, base);
	// 825A8474: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A8478: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A847C: 396BB28C  addi r11, r11, -0x4d74
	ctx.r[11].s64 = ctx.r[11].s64 + -19828;
	// 825A8480: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A8484: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A8488: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825A848C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 825A8490: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 825A8494: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 825A8498: 3B9F002C  addi r28, r31, 0x2c
	ctx.r[28].s64 = ctx.r[31].s64 + 44;
	// 825A849C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 825A84A0: 937F0018  stw r27, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 825A84A4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 825A84A8: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 825A84AC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 825A84B0: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 825A84B4: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 825A84B8: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 825A84BC: 4884B745  bl 0x82df3c00
	ctx.lr = 0x825A84C0;
	sub_82DF3C00(ctx, base);
	// 825A84C0: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 825A84C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A84C8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A84CC: 4BEC265D  bl 0x8246ab28
	ctx.lr = 0x825A84D0;
	sub_8246AB28(ctx, base);
	// 825A84D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A84D4: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A84D8: 4BEC2651  bl 0x8246ab28
	ctx.lr = 0x825A84DC;
	sub_8246AB28(ctx, base);
	// 825A84DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A84E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A84E4: 48BFFCD0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A84E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A84E8 size=156
    let mut pc: u32 = 0x825A84E8;
    'dispatch: loop {
        match pc {
            0x825A84E8 => {
    //   block [0x825A84E8..0x825A8584)
	// 825A84E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A84EC: 48BFFC79  bl 0x831a8164
	ctx.lr = 0x825A84F0;
	sub_831A8130(ctx, base);
	// 825A84F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A84F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A84F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A84FC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825A8500: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825A8504: 4BFFF17D  bl 0x825a7680
	ctx.lr = 0x825A8508;
	sub_825A7680(ctx, base);
	// 825A8508: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825A850C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A8510: 394AB28C  addi r10, r10, -0x4d74
	ctx.r[10].s64 = ctx.r[10].s64 + -19828;
	// 825A8514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A8518: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A851C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 825A8520: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825A8524: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 825A8528: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825A852C: 3BBF002C  addi r29, r31, 0x2c
	ctx.r[29].s64 = ctx.r[31].s64 + 44;
	// 825A8530: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825A8534: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 825A8538: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825A853C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 825A8540: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825A8544: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 825A8548: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 825A854C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825A8550: 4884B6B1  bl 0x82df3c00
	ctx.lr = 0x825A8554;
	sub_82DF3C00(ctx, base);
	// 825A8554: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825A8558: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825A855C: 4BFE39B5  bl 0x8258bf10
	ctx.lr = 0x825A8560;
	sub_8258BF10(ctx, base);
	// 825A8560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A8564: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8568: 4BEC25C1  bl 0x8246ab28
	ctx.lr = 0x825A856C;
	sub_8246AB28(ctx, base);
	// 825A856C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8570: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8574: 4BEC25B5  bl 0x8246ab28
	ctx.lr = 0x825A8578;
	sub_8246AB28(ctx, base);
	// 825A8578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A857C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A8580: 48BFFC34  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A8588 size=24
    let mut pc: u32 = 0x825A8588;
    'dispatch: loop {
        match pc {
            0x825A8588 => {
    //   block [0x825A8588..0x825A85A0)
	// 825A8588: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A858C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A8590: 409A0008  bne cr6, 0x825a8598
	if !ctx.cr[6].eq {
	pc = 0x825A8598; continue 'dispatch;
	}
	// 825A8594: 3883001C  addi r4, r3, 0x1c
	ctx.r[4].s64 = ctx.r[3].s64 + 28;
	// 825A8598: 3863002C  addi r3, r3, 0x2c
	ctx.r[3].s64 = ctx.r[3].s64 + 44;
	// 825A859C: 4BEC258C  b 0x8246ab28
	sub_8246AB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A85A0 size=232
    let mut pc: u32 = 0x825A85A0;
    'dispatch: loop {
        match pc {
            0x825A85A0 => {
    //   block [0x825A85A0..0x825A8688)
	// 825A85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A85A4: 48BFFBC9  bl 0x831a816c
	ctx.lr = 0x825A85A8;
	sub_831A8130(ctx, base);
	// 825A85A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A85AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A85B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A85B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A85B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A85BC: 388BB1D0  addi r4, r11, -0x4e30
	ctx.r[4].s64 = ctx.r[11].s64 + -20016;
	// 825A85C0: 38A00086  li r5, 0x86
	ctx.r[5].s64 = 134;
	// 825A85C4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A85C8: 4BD17E11  bl 0x822c03d8
	ctx.lr = 0x825A85CC;
	sub_822C03D8(ctx, base);
	// 825A85CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A85D0: 4182001C  beq 0x825a85ec
	if ctx.cr[0].eq {
	pc = 0x825A85EC; continue 'dispatch;
	}
	// 825A85D4: 38DE0040  addi r6, r30, 0x40
	ctx.r[6].s64 = ctx.r[30].s64 + 64;
	// 825A85D8: 80BE0018  lwz r5, 0x18(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A85DC: 389E003C  addi r4, r30, 0x3c
	ctx.r[4].s64 = ctx.r[30].s64 + 60;
	// 825A85E0: 4BFFFF09  bl 0x825a84e8
	ctx.lr = 0x825A85E4;
	sub_825A84E8(ctx, base);
	// 825A85E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A85E8: 48000008  b 0x825a85f0
	pc = 0x825A85F0; continue 'dispatch;
	// 825A85EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A85F0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825A85F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A85F8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A85FC: 4BFFF2B5  bl 0x825a78b0
	ctx.lr = 0x825A8600;
	sub_825A78B0(ctx, base);
	// 825A8600: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8604: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8608: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A860C: 4BD179F5  bl 0x822c0000
	ctx.lr = 0x825A8610;
	sub_822C0000(ctx, base);
	// 825A8610: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A8614: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A8618: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825A861C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825A8620: 4BD20699  bl 0x822c8cb8
	ctx.lr = 0x825A8624;
	sub_822C8CB8(ctx, base);
	// 825A8624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A8628: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A862C: 4BFFDE5D  bl 0x825a6488
	ctx.lr = 0x825A8630;
	sub_825A6488(ctx, base);
	// 825A8630: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8638: 4822D431  bl 0x827d5a68
	ctx.lr = 0x825A863C;
	sub_827D5A68(ctx, base);
	// 825A863C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8640: 4884ADE9  bl 0x82df3428
	ctx.lr = 0x825A8644;
	sub_82DF3428(ctx, base);
	// 825A8644: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A8648: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A864C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8650: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825A8654: 419A0028  beq cr6, 0x825a867c
	if ctx.cr[6].eq {
	pc = 0x825A867C; continue 'dispatch;
	}
	// 825A8658: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825A865C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825A8660: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A8664: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825A8668: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A866C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825A8670: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A8674: 4082FFE8  bne 0x825a865c
	if !ctx.cr[0].eq {
	pc = 0x825A865C; continue 'dispatch;
	}
	// 825A8678: 4BD18219  bl 0x822c0890
	ctx.lr = 0x825A867C;
	sub_822C0890(ctx, base);
	// 825A867C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A8684: 48BFFB38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8688 size=104
    let mut pc: u32 = 0x825A8688;
    'dispatch: loop {
        match pc {
            0x825A8688 => {
    //   block [0x825A8688..0x825A86F0)
	// 825A8688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A868C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8694: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8698: 83E30018  lwz r31, 0x18(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A869C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A86A0: 409A0008  bne cr6, 0x825a86a8
	if !ctx.cr[6].eq {
	pc = 0x825A86A8; continue 'dispatch;
	}
	// 825A86A4: 3BE3001C  addi r31, r3, 0x1c
	ctx.r[31].s64 = ctx.r[3].s64 + 28;
	// 825A86A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825A86AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A86B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A86B4: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825A86B8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A86BC: 4BF0CB1D  bl 0x824b51d8
	ctx.lr = 0x825A86C0;
	sub_824B51D8(ctx, base);
	// 825A86C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A86C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A86C8: 419A0010  beq cr6, 0x825a86d8
	if ctx.cr[6].eq {
	pc = 0x825A86D8; continue 'dispatch;
	}
	// 825A86CC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A86D0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A86D4: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 825A86D8: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 825A86DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A86E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A86E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A86E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A86EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A86F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A86F0 size=80
    let mut pc: u32 = 0x825A86F0;
    'dispatch: loop {
        match pc {
            0x825A86F0 => {
    //   block [0x825A86F0..0x825A8740)
	// 825A86F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A86F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A86F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A86FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8704: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A8708: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A870C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8714: 419A0008  beq cr6, 0x825a871c
	if ctx.cr[6].eq {
	pc = 0x825A871C; continue 'dispatch;
	}
	// 825A8718: 4BEC2411  bl 0x8246ab28
	ctx.lr = 0x825A871C;
	sub_8246AB28(ctx, base);
	// 825A871C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8720: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 825A8724: 4BEC2405  bl 0x8246ab28
	ctx.lr = 0x825A8728;
	sub_8246AB28(ctx, base);
	// 825A8728: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A872C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8734: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A8738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A873C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8740 size=76
    let mut pc: u32 = 0x825A8740;
    'dispatch: loop {
        match pc {
            0x825A8740 => {
    //   block [0x825A8740..0x825A878C)
	// 825A8740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A874C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8758: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A875C: 4BFFFA3D  bl 0x825a8198
	ctx.lr = 0x825A8760;
	sub_825A8198(ctx, base);
	// 825A8760: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A8764: 4182000C  beq 0x825a8770
	if ctx.cr[0].eq {
	pc = 0x825A8770; continue 'dispatch;
	}
	// 825A8768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A876C: 4BD17AFD  bl 0x822c0268
	ctx.lr = 0x825A8770;
	sub_822C0268(ctx, base);
	// 825A8770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A8778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A877C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8780: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A8784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8790 size=68
    let mut pc: u32 = 0x825A8790;
    'dispatch: loop {
        match pc {
            0x825A8790 => {
    //   block [0x825A8790..0x825A87D4)
	// 825A8790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8798: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A879C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A87A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A87A4: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 825A87A8: 4BFFFC61  bl 0x825a8408
	ctx.lr = 0x825A87AC;
	sub_825A8408(ctx, base);
	// 825A87AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A87B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A87B4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A87B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A87BC: 4E800421  bctrl
	ctx.lr = 0x825A87C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A87C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A87C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A87C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A87CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A87D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A87D8 size=104
    let mut pc: u32 = 0x825A87D8;
    'dispatch: loop {
        match pc {
            0x825A87D8 => {
    //   block [0x825A87D8..0x825A8840)
	// 825A87D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A87DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A87E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A87E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A87E8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A87EC: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A87F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A87F4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A87F8: 38CB82C0  addi r6, r11, -0x7d40
	ctx.r[6].s64 = ctx.r[11].s64 + -32064;
	// 825A87FC: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825A8800: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A8804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A8808: 48C01741  bl 0x831a9f48
	ctx.lr = 0x825A880C;
	sub_831A9F48(ctx, base);
	// 825A880C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A8810: 4182001C  beq 0x825a882c
	if ctx.cr[0].eq {
	pc = 0x825A882C; continue 'dispatch;
	}
	// 825A8814: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8818: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A881C: 409A0008  bne cr6, 0x825a8824
	if !ctx.cr[6].eq {
	pc = 0x825A8824; continue 'dispatch;
	}
	// 825A8820: 3883001C  addi r4, r3, 0x1c
	ctx.r[4].s64 = ctx.r[3].s64 + 28;
	// 825A8824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8828: 4BFFFBE1  bl 0x825a8408
	ctx.lr = 0x825A882C;
	sub_825A8408(ctx, base);
	// 825A882C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A8830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8840 size=68
    let mut pc: u32 = 0x825A8840;
    'dispatch: loop {
        match pc {
            0x825A8840 => {
    //   block [0x825A8840..0x825A8884)
	// 825A8840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A884C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8854: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 825A8858: 4BFFFE99  bl 0x825a86f0
	ctx.lr = 0x825A885C;
	sub_825A86F0(ctx, base);
	// 825A885C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8864: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A8868: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A886C: 4E800421  bctrl
	ctx.lr = 0x825A8870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A8874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A887C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8888 size=104
    let mut pc: u32 = 0x825A8888;
    'dispatch: loop {
        match pc {
            0x825A8888 => {
    //   block [0x825A8888..0x825A88F0)
	// 825A8888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8898: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A889C: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A88A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A88A4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A88A8: 38CB82FC  addi r6, r11, -0x7d04
	ctx.r[6].s64 = ctx.r[11].s64 + -32004;
	// 825A88AC: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825A88B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A88B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A88B8: 48C01691  bl 0x831a9f48
	ctx.lr = 0x825A88BC;
	sub_831A9F48(ctx, base);
	// 825A88BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A88C0: 4182001C  beq 0x825a88dc
	if ctx.cr[0].eq {
	pc = 0x825A88DC; continue 'dispatch;
	}
	// 825A88C4: 80830018  lwz r4, 0x18(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A88C8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825A88CC: 409A0008  bne cr6, 0x825a88d4
	if !ctx.cr[6].eq {
	pc = 0x825A88D4; continue 'dispatch;
	}
	// 825A88D0: 3883001C  addi r4, r3, 0x1c
	ctx.r[4].s64 = ctx.r[3].s64 + 28;
	// 825A88D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A88D8: 4BFFFE19  bl 0x825a86f0
	ctx.lr = 0x825A88DC;
	sub_825A86F0(ctx, base);
	// 825A88DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A88E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A88E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A88E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A88EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A88F0 size=200
    let mut pc: u32 = 0x825A88F0;
    'dispatch: loop {
        match pc {
            0x825A88F0 => {
    //   block [0x825A88F0..0x825A89B8)
	// 825A88F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A88F4: 48BFF879  bl 0x831a816c
	ctx.lr = 0x825A88F8;
	sub_831A8130(ctx, base);
	// 825A88F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A88FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A8900: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 825A8904: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A8908: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A890C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8910: 388BB300  addi r4, r11, -0x4d00
	ctx.r[4].s64 = ctx.r[11].s64 + -19712;
	// 825A8914: 4884B0F5  bl 0x82df3a08
	ctx.lr = 0x825A8918;
	sub_82DF3A08(ctx, base);
	// 825A8918: 388100A4  addi r4, r1, 0xa4
	ctx.r[4].s64 = ctx.r[1].s64 + 164;
	// 825A891C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A8920: 4BFFD529  bl 0x825a5e48
	ctx.lr = 0x825A8924;
	sub_825A5E48(ctx, base);
	// 825A8924: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A892C: 4884AF85  bl 0x82df38b0
	ctx.lr = 0x825A8930;
	sub_82DF38B0(ctx, base);
	// 825A8930: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A8934: 4884AAF5  bl 0x82df3428
	ctx.lr = 0x825A8938;
	sub_82DF3428(ctx, base);
	// 825A8938: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A8940: 388BB1D0  addi r4, r11, -0x4e30
	ctx.r[4].s64 = ctx.r[11].s64 + -20016;
	// 825A8944: 38A0017F  li r5, 0x17f
	ctx.r[5].s64 = 383;
	// 825A8948: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 825A894C: 4BD17A8D  bl 0x822c03d8
	ctx.lr = 0x825A8950;
	sub_822C03D8(ctx, base);
	// 825A8950: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A8954: 4182001C  beq 0x825a8970
	if ctx.cr[0].eq {
	pc = 0x825A8970; continue 'dispatch;
	}
	// 825A8958: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A895C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8960: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A8964: 480016C5  bl 0x825aa028
	ctx.lr = 0x825A8968;
	sub_825AA028(ctx, base);
	// 825A8968: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A896C: 48000008  b 0x825a8974
	pc = 0x825A8974; continue 'dispatch;
	// 825A8970: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A8974: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A8978: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 825A897C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8980: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8984: 4BFFED9D  bl 0x825a7720
	ctx.lr = 0x825A8988;
	sub_825A7720(ctx, base);
	// 825A8988: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A898C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8990: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8994: 4BD1766D  bl 0x822c0000
	ctx.lr = 0x825A8998;
	sub_822C0000(ctx, base);
	// 825A8998: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A899C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A89A0: 4822D0C9  bl 0x827d5a68
	ctx.lr = 0x825A89A4;
	sub_827D5A68(ctx, base);
	// 825A89A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A89A8: 4884AA81  bl 0x82df3428
	ctx.lr = 0x825A89AC;
	sub_82DF3428(ctx, base);
	// 825A89AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A89B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A89B4: 48BFF808  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A89B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A89B8 size=200
    let mut pc: u32 = 0x825A89B8;
    'dispatch: loop {
        match pc {
            0x825A89B8 => {
    //   block [0x825A89B8..0x825A8A80)
	// 825A89B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A89BC: 48BFF7B1  bl 0x831a816c
	ctx.lr = 0x825A89C0;
	sub_831A8130(ctx, base);
	// 825A89C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A89C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825A89C8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 825A89CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A89D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A89D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A89D8: 388BAF04  addi r4, r11, -0x50fc
	ctx.r[4].s64 = ctx.r[11].s64 + -20732;
	// 825A89DC: 4884B02D  bl 0x82df3a08
	ctx.lr = 0x825A89E0;
	sub_82DF3A08(ctx, base);
	// 825A89E0: 388100A4  addi r4, r1, 0xa4
	ctx.r[4].s64 = ctx.r[1].s64 + 164;
	// 825A89E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A89E8: 4BFFD461  bl 0x825a5e48
	ctx.lr = 0x825A89EC;
	sub_825A5E48(ctx, base);
	// 825A89EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A89F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A89F4: 4884AEBD  bl 0x82df38b0
	ctx.lr = 0x825A89F8;
	sub_82DF38B0(ctx, base);
	// 825A89F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A89FC: 4884AA2D  bl 0x82df3428
	ctx.lr = 0x825A8A00;
	sub_82DF3428(ctx, base);
	// 825A8A00: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A8A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A8A08: 388BB1D0  addi r4, r11, -0x4e30
	ctx.r[4].s64 = ctx.r[11].s64 + -20016;
	// 825A8A0C: 38A0018A  li r5, 0x18a
	ctx.r[5].s64 = 394;
	// 825A8A10: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 825A8A14: 4BD179C5  bl 0x822c03d8
	ctx.lr = 0x825A8A18;
	sub_822C03D8(ctx, base);
	// 825A8A18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825A8A1C: 4182001C  beq 0x825a8a38
	if ctx.cr[0].eq {
	pc = 0x825A8A38; continue 'dispatch;
	}
	// 825A8A20: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825A8A24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8A28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A8A2C: 48000BDD  bl 0x825a9608
	ctx.lr = 0x825A8A30;
	sub_825A9608(ctx, base);
	// 825A8A30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8A34: 48000008  b 0x825a8a3c
	pc = 0x825A8A3C; continue 'dispatch;
	// 825A8A38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825A8A3C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825A8A40: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 825A8A44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8A48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8A4C: 4BFFED9D  bl 0x825a77e8
	ctx.lr = 0x825A8A50;
	sub_825A77E8(ctx, base);
	// 825A8A50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8A54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8A58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8A5C: 4BD175A5  bl 0x822c0000
	ctx.lr = 0x825A8A60;
	sub_822C0000(ctx, base);
	// 825A8A60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A8A64: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8A68: 4822D001  bl 0x827d5a68
	ctx.lr = 0x825A8A6C;
	sub_827D5A68(ctx, base);
	// 825A8A6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8A70: 4884A9B9  bl 0x82df3428
	ctx.lr = 0x825A8A74;
	sub_82DF3428(ctx, base);
	// 825A8A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A8A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A8A7C: 48BFF740  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8A80 size=196
    let mut pc: u32 = 0x825A8A80;
    'dispatch: loop {
        match pc {
            0x825A8A80 => {
    //   block [0x825A8A80..0x825A8B44)
	// 825A8A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8A84: 48BFF6E5  bl 0x831a8168
	ctx.lr = 0x825A8A88;
	sub_831A8130(ctx, base);
	// 825A8A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8A8C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8A90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8A94: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 825A8A98: 409A0008  bne cr6, 0x825a8aa0
	if !ctx.cr[6].eq {
	pc = 0x825A8AA0; continue 'dispatch;
	}
	// 825A8A9C: 3B83001C  addi r28, r3, 0x1c
	ctx.r[28].s64 = ctx.r[3].s64 + 28;
	// 825A8AA0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8AA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8AA8: 409A000C  bne cr6, 0x825a8ab4
	if !ctx.cr[6].eq {
	pc = 0x825A8AB4; continue 'dispatch;
	}
	// 825A8AAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A8AB0: 48000014  b 0x825a8ac4
	pc = 0x825A8AC4; continue 'dispatch;
	// 825A8AB4: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8AB8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825A8ABC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A8AC0: 7D4B4BD6  divw r10, r11, r9
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 825A8AC4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A8AC8: 3BA30008  addi r29, r3, 8
	ctx.r[29].s64 = ctx.r[3].s64 + 8;
	// 825A8ACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8AD0: 419A0010  beq cr6, 0x825a8ae0
	if ctx.cr[6].eq {
	pc = 0x825A8AE0; continue 'dispatch;
	}
	// 825A8AD4: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8AD8: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 825A8ADC: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825A8AE0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A8AE4: 419A0058  beq cr6, 0x825a8b3c
	if ctx.cr[6].eq {
	pc = 0x825A8B3C; continue 'dispatch;
	}
	// 825A8AE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8AEC: 4BD1E0BD  bl 0x822c6ba8
	ctx.lr = 0x825A8AF0;
	sub_822C6BA8(ctx, base);
	// 825A8AF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A8AF4: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8AF8: 48000038  b 0x825a8b30
	pc = 0x825A8B30; continue 'dispatch;
	// 825A8AFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A8B00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8B04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8B08: 4BFFFEB1  bl 0x825a89b8
	ctx.lr = 0x825A8B0C;
	sub_825A89B8(ctx, base);
	// 825A8B0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A8B10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8B14: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A8B18: 4860A5F1  bl 0x82bb3108
	ctx.lr = 0x825A8B1C;
	sub_82BB3108(ctx, base);
	// 825A8B1C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A8B20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8B24: 419A0008  beq cr6, 0x825a8b2c
	if ctx.cr[6].eq {
	pc = 0x825A8B2C; continue 'dispatch;
	}
	// 825A8B28: 4BD17D69  bl 0x822c0890
	ctx.lr = 0x825A8B2C;
	sub_822C0890(ctx, base);
	// 825A8B2C: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 825A8B30: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8B34: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A8B38: 409AFFC4  bne cr6, 0x825a8afc
	if !ctx.cr[6].eq {
	pc = 0x825A8AFC; continue 'dispatch;
	}
	// 825A8B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A8B40: 48BFF678  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8B48 size=312
    let mut pc: u32 = 0x825A8B48;
    'dispatch: loop {
        match pc {
            0x825A8B48 => {
    //   block [0x825A8B48..0x825A8C80)
	// 825A8B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8B4C: 48BFF61D  bl 0x831a8168
	ctx.lr = 0x825A8B50;
	sub_831A8130(ctx, base);
	// 825A8B50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8B58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A8B5C: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 825A8B60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A8B64: 4BD1E045  bl 0x822c6ba8
	ctx.lr = 0x825A8B68;
	sub_822C6BA8(ctx, base);
	// 825A8B68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A8B6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8B70: 48030E31  bl 0x825d99a0
	ctx.lr = 0x825A8B74;
	sub_825D99A0(ctx, base);
	// 825A8B74: 480000E8  b 0x825a8c5c
	pc = 0x825A8C5C; continue 'dispatch;
	// 825A8B78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8B80: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A8B84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A8B88: 4E800421  bctrl
	ctx.lr = 0x825A8B8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8B8C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8B90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A8B94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8B98: 409A0008  bne cr6, 0x825a8ba0
	if !ctx.cr[6].eq {
	pc = 0x825A8BA0; continue 'dispatch;
	}
	// 825A8B9C: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 825A8BA0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8BA4: 1D7E000C  mulli r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 * 12;
	// 825A8BA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A8BAC: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825A8BB0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A8BB4: 4BFFFE05  bl 0x825a89b8
	ctx.lr = 0x825A8BB8;
	sub_825A89B8(ctx, base);
	// 825A8BB8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825A8BBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A8BC0: 4860A549  bl 0x82bb3108
	ctx.lr = 0x825A8BC4;
	sub_82BB3108(ctx, base);
	// 825A8BC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A8BC8: 480311C9  bl 0x825d9d90
	ctx.lr = 0x825A8BCC;
	sub_825D9D90(ctx, base);
	// 825A8BCC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A8BD0: 57DE1838  slwi r30, r30, 3
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825A8BD4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825A8BD8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825A8BDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8BE0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8BE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A8BE8: 4E800421  bctrl
	ctx.lr = 0x825A8BEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8BEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A8BF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A8BF4: 48031215  bl 0x825d9e08
	ctx.lr = 0x825A8BF8;
	sub_825D9E08(ctx, base);
	// 825A8BF8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A8BFC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825A8C00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8C04: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A8C08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A8C0C: 4E800421  bctrl
	ctx.lr = 0x825A8C10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8C10: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A8C14: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A8C18: 48030EA1  bl 0x825d9ab8
	ctx.lr = 0x825A8C1C;
	sub_825D9AB8(ctx, base);
	// 825A8C1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A8C20: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A8C24: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825A8C28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8C2C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A8C30: 4BD1B831  bl 0x822c4460
	ctx.lr = 0x825A8C34;
	sub_822C4460(ctx, base);
	// 825A8C34: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A8C38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8C3C: 419A0008  beq cr6, 0x825a8c44
	if ctx.cr[6].eq {
	pc = 0x825A8C44; continue 'dispatch;
	}
	// 825A8C40: 4BD17C51  bl 0x822c0890
	ctx.lr = 0x825A8C44;
	sub_822C0890(ctx, base);
	// 825A8C44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A8C48: 48031181  bl 0x825d9dc8
	ctx.lr = 0x825A8C4C;
	sub_825D9DC8(ctx, base);
	// 825A8C4C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825A8C50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8C54: 419A0008  beq cr6, 0x825a8c5c
	if ctx.cr[6].eq {
	pc = 0x825A8C5C; continue 'dispatch;
	}
	// 825A8C58: 4BD17C39  bl 0x822c0890
	ctx.lr = 0x825A8C5C;
	sub_822C0890(ctx, base);
	// 825A8C5C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A8C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8C64: 409AFF14  bne cr6, 0x825a8b78
	if !ctx.cr[6].eq {
	pc = 0x825A8B78; continue 'dispatch;
	}
	// 825A8C68: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A8C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8C70: 419A0008  beq cr6, 0x825a8c78
	if ctx.cr[6].eq {
	pc = 0x825A8C78; continue 'dispatch;
	}
	// 825A8C74: 4BD17C1D  bl 0x822c0890
	ctx.lr = 0x825A8C78;
	sub_822C0890(ctx, base);
	// 825A8C78: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A8C7C: 48BFF53C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8C80 size=192
    let mut pc: u32 = 0x825A8C80;
    'dispatch: loop {
        match pc {
            0x825A8C80 => {
    //   block [0x825A8C80..0x825A8D40)
	// 825A8C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8C84: 48BFF4E5  bl 0x831a8168
	ctx.lr = 0x825A8C88;
	sub_831A8130(ctx, base);
	// 825A8C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8C8C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8C90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8C94: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 825A8C98: 409A0008  bne cr6, 0x825a8ca0
	if !ctx.cr[6].eq {
	pc = 0x825A8CA0; continue 'dispatch;
	}
	// 825A8C9C: 3B83001C  addi r28, r3, 0x1c
	ctx.r[28].s64 = ctx.r[3].s64 + 28;
	// 825A8CA0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8CA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8CA8: 409A000C  bne cr6, 0x825a8cb4
	if !ctx.cr[6].eq {
	pc = 0x825A8CB4; continue 'dispatch;
	}
	// 825A8CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A8CB0: 48000010  b 0x825a8cc0
	pc = 0x825A8CC0; continue 'dispatch;
	// 825A8CB4: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8CB8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825A8CBC: 7D6A1670  srawi r10, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 825A8CC0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A8CC4: 3BA30008  addi r29, r3, 8
	ctx.r[29].s64 = ctx.r[3].s64 + 8;
	// 825A8CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8CCC: 419A0010  beq cr6, 0x825a8cdc
	if ctx.cr[6].eq {
	pc = 0x825A8CDC; continue 'dispatch;
	}
	// 825A8CD0: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8CD4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 825A8CD8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825A8CDC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A8CE0: 419A0058  beq cr6, 0x825a8d38
	if ctx.cr[6].eq {
	pc = 0x825A8D38; continue 'dispatch;
	}
	// 825A8CE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8CE8: 4BD1DEC1  bl 0x822c6ba8
	ctx.lr = 0x825A8CEC;
	sub_822C6BA8(ctx, base);
	// 825A8CEC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A8CF0: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8CF4: 48000038  b 0x825a8d2c
	pc = 0x825A8D2C; continue 'dispatch;
	// 825A8CF8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A8CFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8D00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8D04: 4BFFFBED  bl 0x825a88f0
	ctx.lr = 0x825A8D08;
	sub_825A88F0(ctx, base);
	// 825A8D08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A8D0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A8D10: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825A8D14: 4860A3F5  bl 0x82bb3108
	ctx.lr = 0x825A8D18;
	sub_82BB3108(ctx, base);
	// 825A8D18: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A8D1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8D20: 419A0008  beq cr6, 0x825a8d28
	if ctx.cr[6].eq {
	pc = 0x825A8D28; continue 'dispatch;
	}
	// 825A8D24: 4BD17B6D  bl 0x822c0890
	ctx.lr = 0x825A8D28;
	sub_822C0890(ctx, base);
	// 825A8D28: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 825A8D2C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A8D30: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825A8D34: 409AFFC4  bne cr6, 0x825a8cf8
	if !ctx.cr[6].eq {
	pc = 0x825A8CF8; continue 'dispatch;
	}
	// 825A8D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A8D3C: 48BFF47C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8D40 size=312
    let mut pc: u32 = 0x825A8D40;
    'dispatch: loop {
        match pc {
            0x825A8D40 => {
    //   block [0x825A8D40..0x825A8E78)
	// 825A8D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8D44: 48BFF425  bl 0x831a8168
	ctx.lr = 0x825A8D48;
	sub_831A8130(ctx, base);
	// 825A8D48: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8D50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825A8D54: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 825A8D58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A8D5C: 4BD1DE4D  bl 0x822c6ba8
	ctx.lr = 0x825A8D60;
	sub_822C6BA8(ctx, base);
	// 825A8D60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A8D64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8D68: 48030C39  bl 0x825d99a0
	ctx.lr = 0x825A8D6C;
	sub_825D99A0(ctx, base);
	// 825A8D6C: 480000E8  b 0x825a8e54
	pc = 0x825A8E54; continue 'dispatch;
	// 825A8D70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A8D78: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A8D7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A8D80: 4E800421  bctrl
	ctx.lr = 0x825A8D84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8D84: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8D88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A8D8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8D90: 409A0008  bne cr6, 0x825a8d98
	if !ctx.cr[6].eq {
	pc = 0x825A8D98; continue 'dispatch;
	}
	// 825A8D94: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 825A8D98: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825A8D9C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825A8DA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A8DA4: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825A8DA8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A8DAC: 4BFFFB45  bl 0x825a88f0
	ctx.lr = 0x825A8DB0;
	sub_825A88F0(ctx, base);
	// 825A8DB0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825A8DB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825A8DB8: 4860A351  bl 0x82bb3108
	ctx.lr = 0x825A8DBC;
	sub_82BB3108(ctx, base);
	// 825A8DBC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A8DC0: 48030FD1  bl 0x825d9d90
	ctx.lr = 0x825A8DC4;
	sub_825D9D90(ctx, base);
	// 825A8DC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A8DC8: 57DE1838  slwi r30, r30, 3
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825A8DCC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825A8DD0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825A8DD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8DD8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825A8DDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A8DE0: 4E800421  bctrl
	ctx.lr = 0x825A8DE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8DE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A8DE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A8DEC: 4803101D  bl 0x825d9e08
	ctx.lr = 0x825A8DF0;
	sub_825D9E08(ctx, base);
	// 825A8DF0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825A8DF4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825A8DF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8DFC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825A8E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A8E04: 4E800421  bctrl
	ctx.lr = 0x825A8E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A8E08: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825A8E0C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A8E10: 48030CA9  bl 0x825d9ab8
	ctx.lr = 0x825A8E14;
	sub_825D9AB8(ctx, base);
	// 825A8E14: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A8E18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A8E1C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 825A8E20: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A8E24: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A8E28: 4BD1B639  bl 0x822c4460
	ctx.lr = 0x825A8E2C;
	sub_822C4460(ctx, base);
	// 825A8E2C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A8E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8E34: 419A0008  beq cr6, 0x825a8e3c
	if ctx.cr[6].eq {
	pc = 0x825A8E3C; continue 'dispatch;
	}
	// 825A8E38: 4BD17A59  bl 0x822c0890
	ctx.lr = 0x825A8E3C;
	sub_822C0890(ctx, base);
	// 825A8E3C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A8E40: 48030F89  bl 0x825d9dc8
	ctx.lr = 0x825A8E44;
	sub_825D9DC8(ctx, base);
	// 825A8E44: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825A8E48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8E4C: 419A0008  beq cr6, 0x825a8e54
	if ctx.cr[6].eq {
	pc = 0x825A8E54; continue 'dispatch;
	}
	// 825A8E50: 4BD17A41  bl 0x822c0890
	ctx.lr = 0x825A8E54;
	sub_822C0890(ctx, base);
	// 825A8E54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A8E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A8E5C: 409AFF14  bne cr6, 0x825a8d70
	if !ctx.cr[6].eq {
	pc = 0x825A8D70; continue 'dispatch;
	}
	// 825A8E60: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A8E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A8E68: 419A0008  beq cr6, 0x825a8e70
	if ctx.cr[6].eq {
	pc = 0x825A8E70; continue 'dispatch;
	}
	// 825A8E6C: 4BD17A25  bl 0x822c0890
	ctx.lr = 0x825A8E70;
	sub_822C0890(ctx, base);
	// 825A8E70: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A8E74: 48BFF344  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8E78 size=132
    let mut pc: u32 = 0x825A8E78;
    'dispatch: loop {
        match pc {
            0x825A8E78 => {
    //   block [0x825A8E78..0x825A8EFC)
	// 825A8E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8E80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A8E84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8E88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8E8C: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825A8E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8E94: 396B8B48  addi r11, r11, -0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + -29880;
	// 825A8E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A8E9C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825A8EA0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A8EA4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A8EA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A8EAC: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A8EB0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A8EB4: 4BFFEEC5  bl 0x825a7d78
	ctx.lr = 0x825A8EB8;
	sub_825A7D78(ctx, base);
	// 825A8EB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8EC0: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825A8EC4: 4BFFD5C5  bl 0x825a6488
	ctx.lr = 0x825A8EC8;
	sub_825A6488(ctx, base);
	// 825A8EC8: 4884A2E9  bl 0x82df31b0
	ctx.lr = 0x825A8ECC;
	sub_82DF31B0(ctx, base);
	// 825A8ECC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8ED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A8ED4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8ED8: 48031009  bl 0x825d9ee0
	ctx.lr = 0x825A8EDC;
	sub_825D9EE0(ctx, base);
	// 825A8EDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8EE0: 4884A549  bl 0x82df3428
	ctx.lr = 0x825A8EE4;
	sub_82DF3428(ctx, base);
	// 825A8EE4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A8EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8EF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A8EF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8F00 size=132
    let mut pc: u32 = 0x825A8F00;
    'dispatch: loop {
        match pc {
            0x825A8F00 => {
    //   block [0x825A8F00..0x825A8F84)
	// 825A8F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8F08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A8F0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8F10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8F14: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825A8F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A8F1C: 396B8D40  addi r11, r11, -0x72c0
	ctx.r[11].s64 = ctx.r[11].s64 + -29376;
	// 825A8F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A8F24: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825A8F28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A8F2C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825A8F30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A8F34: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825A8F38: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825A8F3C: 4BFFEEBD  bl 0x825a7df8
	ctx.lr = 0x825A8F40;
	sub_825A7DF8(ctx, base);
	// 825A8F40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A8F44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8F48: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 825A8F4C: 4BFFD53D  bl 0x825a6488
	ctx.lr = 0x825A8F50;
	sub_825A6488(ctx, base);
	// 825A8F50: 4884A261  bl 0x82df31b0
	ctx.lr = 0x825A8F54;
	sub_82DF31B0(ctx, base);
	// 825A8F54: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A8F5C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8F60: 48030F81  bl 0x825d9ee0
	ctx.lr = 0x825A8F64;
	sub_825D9EE0(ctx, base);
	// 825A8F64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8F68: 4884A4C1  bl 0x82df3428
	ctx.lr = 0x825A8F6C;
	sub_82DF3428(ctx, base);
	// 825A8F6C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A8F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A8F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A8F78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A8F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A8F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A8F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A8F88 size=136
    let mut pc: u32 = 0x825A8F88;
    'dispatch: loop {
        match pc {
            0x825A8F88 => {
    //   block [0x825A8F88..0x825A9010)
	// 825A8F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A8F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A8F90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A8F94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A8F98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A8F9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A8FA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A8FA4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825A8FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8FAC: 83EB0040  lwz r31, 0x40(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A8FB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825A8FB4: 419A0020  beq cr6, 0x825a8fd4
	if ctx.cr[6].eq {
	pc = 0x825A8FD4; continue 'dispatch;
	}
	// 825A8FB8: 4BFFD4D1  bl 0x825a6488
	ctx.lr = 0x825A8FBC;
	sub_825A6488(ctx, base);
	// 825A8FBC: 4884A1F5  bl 0x82df31b0
	ctx.lr = 0x825A8FC0;
	sub_82DF31B0(ctx, base);
	// 825A8FC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8FC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A8FC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8FCC: 48030E6D  bl 0x825d9e38
	ctx.lr = 0x825A8FD0;
	sub_825D9E38(ctx, base);
	// 825A8FD0: 48000020  b 0x825a8ff0
	pc = 0x825A8FF0; continue 'dispatch;
	// 825A8FD4: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 825A8FD8: 4BFFD4B1  bl 0x825a6488
	ctx.lr = 0x825A8FDC;
	sub_825A6488(ctx, base);
	// 825A8FDC: 4884A1D5  bl 0x82df31b0
	ctx.lr = 0x825A8FE0;
	sub_82DF31B0(ctx, base);
	// 825A8FE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A8FE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A8FE8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825A8FEC: 48030E45  bl 0x825d9e30
	ctx.lr = 0x825A8FF0;
	sub_825D9E30(ctx, base);
	// 825A8FF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A8FF4: 4884A435  bl 0x82df3428
	ctx.lr = 0x825A8FF8;
	sub_82DF3428(ctx, base);
	// 825A8FF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A8FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A9008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A900C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9010 size=92
    let mut pc: u32 = 0x825A9010;
    'dispatch: loop {
        match pc {
            0x825A9010 => {
    //   block [0x825A9010..0x825A906C)
	// 825A9010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A901C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9024: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9028: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 825A902C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9034: 4BFFD455  bl 0x825a6488
	ctx.lr = 0x825A9038;
	sub_825A6488(ctx, base);
	// 825A9038: 4884A179  bl 0x82df31b0
	ctx.lr = 0x825A903C;
	sub_82DF31B0(ctx, base);
	// 825A903C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9044: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A9048: 48030DE9  bl 0x825d9e30
	ctx.lr = 0x825A904C;
	sub_825D9E30(ctx, base);
	// 825A904C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9050: 4884A3D9  bl 0x82df3428
	ctx.lr = 0x825A9054;
	sub_82DF3428(ctx, base);
	// 825A9054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A9058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A905C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9060: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A9064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9070 size=108
    let mut pc: u32 = 0x825A9070;
    'dispatch: loop {
        match pc {
            0x825A9070 => {
    //   block [0x825A9070..0x825A90DC)
	// 825A9070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9074: 48BFF0F5  bl 0x831a8168
	ctx.lr = 0x825A9078;
	sub_831A8130(ctx, base);
	// 825A9078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A907C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825A9080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9084: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9088: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A908C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825A9090: 4884A979  bl 0x82df3a08
	ctx.lr = 0x825A9094;
	sub_82DF3A08(ctx, base);
	// 825A9094: 3BBE0068  addi r29, r30, 0x68
	ctx.r[29].s64 = ctx.r[30].s64 + 104;
	// 825A9098: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825A909C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A90A0: 4884A269  bl 0x82df3308
	ctx.lr = 0x825A90A4;
	sub_82DF3308(ctx, base);
	// 825A90A4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A90A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A90AC: 4884A37D  bl 0x82df3428
	ctx.lr = 0x825A90B0;
	sub_82DF3428(ctx, base);
	// 825A90B0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A90B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A90B8: 41820010  beq 0x825a90c8
	if ctx.cr[0].eq {
	pc = 0x825A90C8; continue 'dispatch;
	}
	// 825A90BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A90C0: 4BFFD3C9  bl 0x825a6488
	ctx.lr = 0x825A90C4;
	sub_825A6488(ctx, base);
	// 825A90C4: 4800000C  b 0x825a90d0
	pc = 0x825A90D0; continue 'dispatch;
	// 825A90C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825A90CC: 4884AB35  bl 0x82df3c00
	ctx.lr = 0x825A90D0;
	sub_82DF3C00(ctx, base);
	// 825A90D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A90D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825A90D8: 48BFF0E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A90E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A90E0 size=44
    let mut pc: u32 = 0x825A90E0;
    'dispatch: loop {
        match pc {
            0x825A90E0 => {
    //   block [0x825A90E0..0x825A910C)
	// 825A90E0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 825A90E4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A910C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A910C size=52
    let mut pc: u32 = 0x825A910C;
    'dispatch: loop {
        match pc {
            0x825A910C => {
    //   block [0x825A910C..0x825A9140)
	// 825A910C: C0040008  lfs f0, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9110: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 825A9114: 8141FFF8  lwz r10, -8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9118: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A911C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 825A9120: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9124: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825A9128: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825A912C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825A9130: 8121FFF4  lwz r9, -0xc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 825A9134: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825A9138: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825A913C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A9140 size=20
    let mut pc: u32 = 0x825A9140;
    'dispatch: loop {
        match pc {
            0x825A9140 => {
    //   block [0x825A9140..0x825A9154)
	// 825A9140: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9144: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9148: 419A000C  beq cr6, 0x825a9154
	if ctx.cr[6].eq {
		sub_825A9154(ctx, base);
		return;
	}
	// 825A914C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825A9150: 48000048  b 0x825a9198
	sub_825A9190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9154(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A9154 size=60
    let mut pc: u32 = 0x825A9154;
    'dispatch: loop {
        match pc {
            0x825A9154 => {
    //   block [0x825A9154..0x825A9190)
	// 825A9154: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 825A9158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A915C: 419A0034  beq cr6, 0x825a9190
	if ctx.cr[6].eq {
		sub_825A9190(ctx, base);
		return;
	}
	// 825A9160: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A9164: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9168: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 825A916C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 825A9170: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A9174: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A9178: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 825A917C: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9180: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 825A9184: D181FFF8  stfs f12, -8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 825A9188: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825A918C: 4800000C  b 0x825a9198
	sub_825A9190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825A9190 size=16
    let mut pc: u32 = 0x825A9190;
    'dispatch: loop {
        match pc {
            0x825A9190 => {
    //   block [0x825A9190..0x825A91A0)
	// 825A9190: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 825A9194: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A91A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A91A0 size=232
    let mut pc: u32 = 0x825A91A0;
    'dispatch: loop {
        match pc {
            0x825A91A0 => {
    //   block [0x825A91A0..0x825A9288)
	// 825A91A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A91A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A91A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A91AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A91B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A91B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A91B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A91BC: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A91C0: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A91C4: 41820018  beq 0x825a91dc
	if ctx.cr[0].eq {
	pc = 0x825A91DC; continue 'dispatch;
	}
	// 825A91C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A91CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A91D0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 825A91D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A91D8: 4E800421  bctrl
	ctx.lr = 0x825A91DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A91DC: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A91E0: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A91E4: 41820018  beq 0x825a91fc
	if ctx.cr[0].eq {
	pc = 0x825A91FC; continue 'dispatch;
	}
	// 825A91E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A91EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A91F0: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825A91F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A91F8: 4E800421  bctrl
	ctx.lr = 0x825A91FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A91FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9200: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A9204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9208: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825A920C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9210: 4E800421  bctrl
	ctx.lr = 0x825A9214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9214: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A9218: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 825A921C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9220: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A9224: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9288 size=316
    let mut pc: u32 = 0x825A9288;
    'dispatch: loop {
        match pc {
            0x825A9288 => {
    //   block [0x825A9288..0x825A93C4)
	// 825A9288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A928C: 48BFEEE1  bl 0x831a816c
	ctx.lr = 0x825A9290;
	sub_831A8130(ctx, base);
	// 825A9290: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9294: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9298: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A929C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A92A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A92A4: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825A92A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A92AC: 4E800421  bctrl
	ctx.lr = 0x825A92B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A92B0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825A92B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A92B8: 4BFFFE29  bl 0x825a90e0
	ctx.lr = 0x825A92BC;
	sub_825A90E0(ctx, base);
	// 825A92BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A92C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825A92C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A92C8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825A92CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A92D0: 4E800421  bctrl
	ctx.lr = 0x825A92D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A92D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A92D8: 41820008  beq 0x825a92e0
	if ctx.cr[0].eq {
	pc = 0x825A92E0; continue 'dispatch;
	}
	// 825A92DC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A92E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A92E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A92E8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A92EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A92F0: 4E800421  bctrl
	ctx.lr = 0x825A92F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A92F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A92F8: 41820014  beq 0x825a930c
	if ctx.cr[0].eq {
	pc = 0x825A930C; continue 'dispatch;
	}
	// 825A92FC: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 825A9300: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A9304: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825A9308: 4BFFFDD9  bl 0x825a90e0
	ctx.lr = 0x825A930C;
	sub_825A90E0(ctx, base);
	// 825A930C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9310: 41820068  beq 0x825a9378
	if ctx.cr[0].eq {
	pc = 0x825A9378; continue 'dispatch;
	}
	// 825A9314: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A9318: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A931C: 41820018  beq 0x825a9334
	if ctx.cr[0].eq {
	pc = 0x825A9334; continue 'dispatch;
	}
	// 825A9320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9328: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825A932C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9330: 4E800421  bctrl
	ctx.lr = 0x825A9334;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9334: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A9338: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A933C: 41820018  beq 0x825a9354
	if ctx.cr[0].eq {
	pc = 0x825A9354; continue 'dispatch;
	}
	// 825A9340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9348: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A934C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9350: 4E800421  bctrl
	ctx.lr = 0x825A9354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9354: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A9358: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A935C: 4182001C  beq 0x825a9378
	if ctx.cr[0].eq {
	pc = 0x825A9378; continue 'dispatch;
	}
	// 825A9360: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9364: 389E0070  addi r4, r30, 0x70
	ctx.r[4].s64 = ctx.r[30].s64 + 112;
	// 825A9368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A936C: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 825A9370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9374: 4E800421  bctrl
	ctx.lr = 0x825A9378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9378: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A937C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9380: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9384: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9388: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825A938C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A9390: 4E800421  bctrl
	ctx.lr = 0x825A9394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9394: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9398: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825A939C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A93A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A93A4: 4E800421  bctrl
	ctx.lr = 0x825A93A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A93A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A93AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A93B0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A93B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A93B8: 4E800421  bctrl
	ctx.lr = 0x825A93BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A93BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825A93C0: 48BFEDFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A93C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A93C8 size=52
    let mut pc: u32 = 0x825A93C8;
    'dispatch: loop {
        match pc {
            0x825A93C8 => {
    //   block [0x825A93C8..0x825A93FC)
	// 825A93C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A93CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A93D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A93D4: 38830020  addi r4, r3, 0x20
	ctx.r[4].s64 = ctx.r[3].s64 + 32;
	// 825A93D8: 4BFFFD09  bl 0x825a90e0
	ctx.lr = 0x825A93DC;
	sub_825A90E0(ctx, base);
	// 825A93DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A93E0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825A93E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A93E8: 4E800421  bctrl
	ctx.lr = 0x825A93EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A93EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A93F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A93F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A93F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9400 size=80
    let mut pc: u32 = 0x825A9400;
    'dispatch: loop {
        match pc {
            0x825A9400 => {
    //   block [0x825A9400..0x825A9450)
	// 825A9400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A940C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 825A9410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9414: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 825A9418: 4BFFFD29  bl 0x825a9140
	ctx.lr = 0x825A941C;
	sub_825A9140(ctx, base);
	// 825A941C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825A9420: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 825A9424: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A9428: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9450 size=100
    let mut pc: u32 = 0x825A9450;
    'dispatch: loop {
        match pc {
            0x825A9450 => {
    //   block [0x825A9450..0x825A94B4)
	// 825A9450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A945C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9460: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825A9464: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825A9468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A946C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825A9470: 38CB8334  addi r6, r11, -0x7ccc
	ctx.r[6].s64 = ctx.r[11].s64 + -31948;
	// 825A9474: 38AA7B58  addi r5, r10, 0x7b58
	ctx.r[5].s64 = ctx.r[10].s64 + 31576;
	// 825A9478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825A947C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825A9480: 48C00AC9  bl 0x831a9f48
	ctx.lr = 0x825A9484;
	sub_831A9F48(ctx, base);
	// 825A9484: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 825A9488: 41820018  beq 0x825a94a0
	if ctx.cr[0].eq {
	pc = 0x825A94A0; continue 'dispatch;
	}
	// 825A948C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9490: 4BFFFCB1  bl 0x825a9140
	ctx.lr = 0x825A9494;
	sub_825A9140(ctx, base);
	// 825A9494: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A949C: 4BFFFC45  bl 0x825a90e0
	ctx.lr = 0x825A94A0;
	sub_825A90E0(ctx, base);
	// 825A94A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A94A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A94A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A94AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A94B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A94B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A94B8 size=64
    let mut pc: u32 = 0x825A94B8;
    'dispatch: loop {
        match pc {
            0x825A94B8 => {
    //   block [0x825A94B8..0x825A94F8)
	// 825A94B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A94BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A94C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A94C4: 38830010  addi r4, r3, 0x10
	ctx.r[4].s64 = ctx.r[3].s64 + 16;
	// 825A94C8: 4BFFFC19  bl 0x825a90e0
	ctx.lr = 0x825A94CC;
	sub_825A90E0(ctx, base);
	// 825A94CC: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 825A94D0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A94F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A94F8 size=128
    let mut pc: u32 = 0x825A94F8;
    'dispatch: loop {
        match pc {
            0x825A94F8 => {
    //   block [0x825A94F8..0x825A9578)
	// 825A94F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A94FC: 48BFEC6D  bl 0x831a8168
	ctx.lr = 0x825A9500;
	sub_831A8130(ctx, base);
	// 825A9500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9508: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A950C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A9510: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A9514: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9518: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A951C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A9520: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825A9524: 48849BCD  bl 0x82df30f0
	ctx.lr = 0x825A9528;
	sub_82DF30F0(ctx, base);
	// 825A9528: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825A952C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A9530: 394AB314  addi r10, r10, -0x4cec
	ctx.r[10].s64 = ctx.r[10].s64 + -19692;
	// 825A9534: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 825A9538: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825A953C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A9540: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9544: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 825A9548: 4884A6B9  bl 0x82df3c00
	ctx.lr = 0x825A954C;
	sub_82DF3C00(ctx, base);
	// 825A954C: 939F006C  stw r28, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 825A9550: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825A9554: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825A9558: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9578 size=140
    let mut pc: u32 = 0x825A9578;
    'dispatch: loop {
        match pc {
            0x825A9578 => {
    //   block [0x825A9578..0x825A9604)
	// 825A9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A957C: 48BFEBE9  bl 0x831a8164
	ctx.lr = 0x825A9580;
	sub_831A8130(ctx, base);
	// 825A9580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9588: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A958C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A9590: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A9594: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9598: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A959C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A95A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825A95A4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825A95A8: 48849B49  bl 0x82df30f0
	ctx.lr = 0x825A95AC;
	sub_82DF30F0(ctx, base);
	// 825A95AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A95B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825A95B4: 396BB314  addi r11, r11, -0x4cec
	ctx.r[11].s64 = ctx.r[11].s64 + -19692;
	// 825A95B8: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 825A95BC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 825A95C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A95C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825A95C8: 4BFE2949  bl 0x8258bf10
	ctx.lr = 0x825A95CC;
	sub_8258BF10(ctx, base);
	// 825A95CC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825A95D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A95D4: 4884A62D  bl 0x82df3c00
	ctx.lr = 0x825A95D8;
	sub_82DF3C00(ctx, base);
	// 825A95D8: 937F006C  stw r27, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 825A95DC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825A95E0: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825A95E4: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825A9608 size=200
    let mut pc: u32 = 0x825A9608;
    'dispatch: loop {
        match pc {
            0x825A9608 => {
    //   block [0x825A9608..0x825A96D0)
	// 825A9608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A960C: 48BFEB5D  bl 0x831a8168
	ctx.lr = 0x825A9610;
	sub_831A8130(ctx, base);
	// 825A9610: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9618: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A961C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825A9620: 396BAF84  addi r11, r11, -0x507c
	ctx.r[11].s64 = ctx.r[11].s64 + -20604;
	// 825A9624: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A9628: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825A962C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825A9630: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825A9634: 48849ABD  bl 0x82df30f0
	ctx.lr = 0x825A9638;
	sub_82DF30F0(ctx, base);
	// 825A9638: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825A963C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A9640: 394AB314  addi r10, r10, -0x4cec
	ctx.r[10].s64 = ctx.r[10].s64 + -19692;
	// 825A9644: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A9648: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825A964C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825A9650: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A9654: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 825A9658: 4884A5A9  bl 0x82df3c00
	ctx.lr = 0x825A965C;
	sub_82DF3C00(ctx, base);
	// 825A965C: 939F006C  stw r28, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 825A9660: 93BF0040  stw r29, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 825A9664: C1BD0008  lfs f13, 8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825A9668: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825A966C: C19D0000  lfs f12, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825A9670: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825A9674: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 825A9678: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 825A967C: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9680: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 825A9684: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825A9688: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825A968C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 825A9690: C01D0004  lfs f0, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825A9694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9698: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825A969C: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 825A96A0: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A96D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A96D0 size=96
    let mut pc: u32 = 0x825A96D0;
    'dispatch: loop {
        match pc {
            0x825A96D0 => {
    //   block [0x825A96D0..0x825A9730)
	// 825A96D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A96D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A96D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825A96DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A96E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A96E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A96E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825A96EC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 825A96F0: 48849D39  bl 0x82df3428
	ctx.lr = 0x825A96F4;
	sub_82DF3428(ctx, base);
	// 825A96F4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 825A96F8: 4BD1F5C1  bl 0x822c8cb8
	ctx.lr = 0x825A96FC;
	sub_822C8CB8(ctx, base);
	// 825A96FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9700: 4BFFCD11  bl 0x825a6410
	ctx.lr = 0x825A9704;
	sub_825A6410(ctx, base);
	// 825A9704: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A9708: 4182000C  beq 0x825a9714
	if ctx.cr[0].eq {
	pc = 0x825A9714; continue 'dispatch;
	}
	// 825A970C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9710: 4BD16B59  bl 0x822c0268
	ctx.lr = 0x825A9714;
	sub_822C0268(ctx, base);
	// 825A9714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825A971C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9724: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825A9728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9730 size=308
    let mut pc: u32 = 0x825A9730;
    'dispatch: loop {
        match pc {
            0x825A9730 => {
    //   block [0x825A9730..0x825A9864)
	// 825A9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9734: 48BFEA2D  bl 0x831a8160
	ctx.lr = 0x825A9738;
	sub_831A8130(ctx, base);
	// 825A9738: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A973C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825A9740: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A9744: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 825A9748: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825A974C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825A9750: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825A9754: 388BB368  addi r4, r11, -0x4c98
	ctx.r[4].s64 = ctx.r[11].s64 + -19608;
	// 825A9758: 38A00093  li r5, 0x93
	ctx.r[5].s64 = 147;
	// 825A975C: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 825A9760: 4BD16C79  bl 0x822c03d8
	ctx.lr = 0x825A9764;
	sub_822C03D8(ctx, base);
	// 825A9764: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825A9768: 41820048  beq 0x825a97b0
	if ctx.cr[0].eq {
	pc = 0x825A97B0; continue 'dispatch;
	}
	// 825A976C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9770: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 825A9774: 837F0008  lwz r27, 8(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825A9778: 3B5F0048  addi r26, r31, 0x48
	ctx.r[26].s64 = ctx.r[31].s64 + 72;
	// 825A977C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A9780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9784: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825A9788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825A978C: 4E800421  bctrl
	ctx.lr = 0x825A9790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825A9790: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9794: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A9798: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 825A979C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825A97A0: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825A97A4: 4BFFFDD5  bl 0x825a9578
	ctx.lr = 0x825A97A8;
	sub_825A9578(ctx, base);
	// 825A97A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A97AC: 48000008  b 0x825a97b4
	pc = 0x825A97B4; continue 'dispatch;
	// 825A97B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825A97B4: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825A97B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A97BC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A97C0: 4BFFE029  bl 0x825a77e8
	ctx.lr = 0x825A97C4;
	sub_825A77E8(ctx, base);
	// 825A97C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A97C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825A97CC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A97D0: 4BD16831  bl 0x822c0000
	ctx.lr = 0x825A97D4;
	sub_822C0000(ctx, base);
	// 825A97D4: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825A97D8: 4182000C  beq 0x825a97e4
	if ctx.cr[0].eq {
	pc = 0x825A97E4; continue 'dispatch;
	}
	// 825A97DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A97E0: 48849C49  bl 0x82df3428
	ctx.lr = 0x825A97E4;
	sub_82DF3428(ctx, base);
	// 825A97E4: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825A97E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825A97EC: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 825A97F0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825A97F4: 4BD1F4C5  bl 0x822c8cb8
	ctx.lr = 0x825A97F8;
	sub_822C8CB8(ctx, base);
	// 825A97F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825A97FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9800: 4BFFCC89  bl 0x825a6488
	ctx.lr = 0x825A9804;
	sub_825A6488(ctx, base);
	// 825A9804: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825A980C: 4822C25D  bl 0x827d5a68
	ctx.lr = 0x825A9810;
	sub_827D5A68(ctx, base);
	// 825A9810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9814: 48849C15  bl 0x82df3428
	ctx.lr = 0x825A9818;
	sub_82DF3428(ctx, base);
	// 825A9818: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 825A981C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825A9820: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825A9824: 917E006C  stw r11, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825A9828: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825A982C: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 825A9830: 419A0028  beq cr6, 0x825a9858
	if ctx.cr[6].eq {
	pc = 0x825A9858; continue 'dispatch;
	}
	// 825A9834: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825A9838: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825A983C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A9840: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825A9844: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825A9848: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825A984C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825A9850: 4082FFE8  bne 0x825a9838
	if !ctx.cr[0].eq {
	pc = 0x825A9838; continue 'dispatch;
	}
	// 825A9854: 4BD1703D  bl 0x822c0890
	ctx.lr = 0x825A9858;
	sub_822C0890(ctx, base);
	// 825A9858: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825A985C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825A9860: 48BFE950  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9868 size=16
    let mut pc: u32 = 0x825A9868;
    'dispatch: loop {
        match pc {
            0x825A9868 => {
    //   block [0x825A9868..0x825A9878)
	// 825A9868: 38630048  addi r3, r3, 0x48
	ctx.r[3].s64 = ctx.r[3].s64 + 72;
	// 825A986C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825A9870: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9874: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9878 size=16
    let mut pc: u32 = 0x825A9878;
    'dispatch: loop {
        match pc {
            0x825A9878 => {
    //   block [0x825A9878..0x825A9888)
	// 825A9878: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 825A987C: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 825A9880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825A9884: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825A9888 size=8
    let mut pc: u32 = 0x825A9888;
    'dispatch: loop {
        match pc {
            0x825A9888 => {
    //   block [0x825A9888..0x825A9890)
	// 825A9888: 4BEA9DF8  b 0x82453680
	sub_82453680(ctx, base);
	return;
	// 825A988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9890 size=200
    let mut pc: u32 = 0x825A9890;
    'dispatch: loop {
        match pc {
            0x825A9890 => {
    //   block [0x825A9890..0x825A9958)
	// 825A9890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A9894: 48BFE8D5  bl 0x831a8168
	ctx.lr = 0x825A9898;
	sub_831A8130(ctx, base);
	// 825A9898: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A989C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A98A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825A98A4: 4BFFF89D  bl 0x825a9140
	ctx.lr = 0x825A98A8;
	sub_825A9140(ctx, base);
	// 825A98A8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 825A98AC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825A98B0: 4BFFC271  bl 0x825a5b20
	ctx.lr = 0x825A98B4;
	sub_825A5B20(ctx, base);
	// 825A98B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825A98B8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825A98BC: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 825A98C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A98C4: 3BABB3AC  addi r29, r11, -0x4c54
	ctx.r[29].s64 = ctx.r[11].s64 + -19540;
	// 825A98C8: 4BFFC259  bl 0x825a5b20
	ctx.lr = 0x825A98CC;
	sub_825A5B20(ctx, base);
	// 825A98CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825A98D0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825A98D4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A98D8: 4BFFC249  bl 0x825a5b20
	ctx.lr = 0x825A98DC;
	sub_825A5B20(ctx, base);
	// 825A98DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A98E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825A98E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A98E8: 4884A411  bl 0x82df3cf8
	ctx.lr = 0x825A98EC;
	sub_82DF3CF8(ctx, base);
	// 825A98EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A98F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A98F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825A98F8: 4884A349  bl 0x82df3c40
	ctx.lr = 0x825A98FC;
	sub_82DF3C40(ctx, base);
	// 825A98FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9904: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825A9908: 4884A3F1  bl 0x82df3cf8
	ctx.lr = 0x825A990C;
	sub_82DF3CF8(ctx, base);
	// 825A990C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825A9910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9914: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825A9918: 4884A329  bl 0x82df3c40
	ctx.lr = 0x825A991C;
	sub_82DF3C40(ctx, base);
	// 825A991C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825A9920: 48849B09  bl 0x82df3428
	ctx.lr = 0x825A9924;
	sub_82DF3428(ctx, base);
	// 825A9924: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825A9928: 48849B01  bl 0x82df3428
	ctx.lr = 0x825A992C;
	sub_82DF3428(ctx, base);
	// 825A992C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825A9930: 48849AF9  bl 0x82df3428
	ctx.lr = 0x825A9934;
	sub_82DF3428(ctx, base);
	// 825A9934: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825A9938: 48849AF1  bl 0x82df3428
	ctx.lr = 0x825A993C;
	sub_82DF3428(ctx, base);
	// 825A993C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825A9940: 48849AE9  bl 0x82df3428
	ctx.lr = 0x825A9944;
	sub_82DF3428(ctx, base);
	// 825A9944: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825A9948: 48849AE1  bl 0x82df3428
	ctx.lr = 0x825A994C;
	sub_82DF3428(ctx, base);
	// 825A994C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9950: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825A9954: 48BFE864  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825A9958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825A9958 size=52
    let mut pc: u32 = 0x825A9958;
    'dispatch: loop {
        match pc {
            0x825A9958 => {
    //   block [0x825A9958..0x825A998C)
	// 825A9958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825A995C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825A9960: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825A9964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825A9968: 3884003C  addi r4, r4, 0x3c
	ctx.r[4].s64 = ctx.r[4].s64 + 60;
	// 825A996C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825A9970: 4884A291  bl 0x82df3c00
	ctx.lr = 0x825A9974;
	sub_82DF3C00(ctx, base);
	// 825A9974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825A9978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825A997C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825A9980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825A9984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825A9988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


