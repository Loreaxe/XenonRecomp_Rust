pub fn sub_82060000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82060000 size=540
    let mut pc: u32 = 0x82060000;
    'dispatch: loop {
        match pc {
            0x82060000 => {
    //   block [0x82060000..0x8206021C)
	// 82060000: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82060004: 39610020  addi r11, r1, 0x20
	ctx.r[11].s64 = ctx.r[1].s64 + 32;
	// 82060008: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82060220 size=228
    let mut pc: u32 = 0x82060220;
    'dispatch: loop {
        match pc {
            0x82060220 => {
    //   block [0x82060220..0x82060304)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82060308 size=360
    let mut pc: u32 = 0x82060308;
    'dispatch: loop {
        match pc {
            0x82060308 => {
    //   block [0x82060308..0x82060470)
	// 82060308: 3D600124  lis r11, 0x124
	ctx.r[11].s64 = 19136512;
	// 8206030C: D021FFC0  stfs f1, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82060310: 8141FFC0  lwz r10, -0x40(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) } as u64;
	// 82060314: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82060318: 616B3F6D  ori r11, r11, 0x3f6d
	ctx.r[11].u64 = ctx.r[11].u64 | 16237;
	// 8206031C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82060320: 516AF07E  rlwimi r10, r11, 0x1e, 1, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(30) as u64) & 0x000000007FFFFFFF) | (ctx.r[10].u64 & 0xFFFFFFFF80000000);
	// 82060324: 9141FFC0  stw r10, -0x40(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[10].u32 ) };
	// 82060328: C181FFC0  lfs f12, -0x40(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8206032C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82060330: ED8C082A  fadds f12, f12, f1
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[1].f64) as f32) as f64;
	// 82060334: C00906E8  lfs f0, 0x6e8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(1768 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82060338: 3941FFD0  addi r10, r1, -0x30
	ctx.r[10].s64 = ctx.r[1].s64 + -48;
	// 8206033C: C1A806E4  lfs f13, 0x6e4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(1764 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82060340: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82060344: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82060348: 392906C0  addi r9, r9, 0x6c0
	ctx.r[9].s64 = ctx.r[9].s64 + 1728;
	// 8206034C: 39080680  addi r8, r8, 0x680
	ctx.r[8].s64 = ctx.r[8].s64 + 1664;
	// 82060350: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82060354: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82060358: 38E706A0  addi r7, r7, 0x6a0
	ctx.r[7].s64 = ctx.r[7].s64 + 1696;
	// 8206035C: 134048C7  vcmpequd (lvx128) v26, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[58] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82060360: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 82060364: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82060368: C00B06E0  lfs f0, 0x6e0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206036C: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82060370: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82060374: 136040C7  vcmpequd (lvx128) v27, v0, v8
	tmp.u32 = ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[59] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82060378: 3901FFD0  addi r8, r1, -0x30
	ctx.r[8].s64 = ctx.r[1].s64 + -48;
	// 8206037C: 396B06B0  addi r11, r11, 0x6b0
	ctx.r[11].s64 = ctx.r[11].s64 + 1712;
	// 82060380: 13C038C7  vcmpequd (lvx128) v30, v0, v7
	tmp.u32 = ctx.r[7].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82060384: 38C60670  addi r6, r6, 0x670
	ctx.r[6].s64 = ctx.r[6].s64 + 1648;
	// 82060388: 38E1FFC0  addi r7, r1, -0x40
	ctx.r[7].s64 = ctx.r[1].s64 + -64;
	// 8206038C: 138058C7  vcmpequd (lvx128) v28, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82060390: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82060394: 13A030C7  vcmpequd (lvx128) v29, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82060398: 38C1FFE0  addi r6, r1, -0x20
	ctx.r[6].s64 = ctx.r[1].s64 + -32;
	// 8206039C: FC00601E  fctiwz f0, f12
	ctx.f[0].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 820603A0: D801FFC0  stfd f0, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[0].u64 ) };
	// 820603A4: E8A1FFC6  lwa r5, -0x3c(r1)
	ctx.r[5].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-60 as u32) ) } as i32) as i64;
	// 820603A8: 396B0690  addi r11, r11, 0x690
	ctx.r[11].s64 = ctx.r[11].s64 + 1680;
	// 820603AC: F8A1FFC0  std r5, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.r[5].u64 ) };
	// 820603B0: 132058C7  vcmpequd (lvx128) v25, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[57] using VectorMaskL[(tmp.u32 & 0xF)]
	// 820603B4: C801FFC0  lfd f0, -0x40(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 820603B8: 3961FFD0  addi r11, r1, -0x30
	ctx.r[11].s64 = ctx.r[1].s64 + -48;
	// 820603BC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 820603C0: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 820603C4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 820603C8: EC000B7C  fnmsubs f0, f0, f13, f1
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[1].f64) as f32) as f64);
	// 820603CC: D001FFD4  stfs f0, -0x2c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 820603D0: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 820603D4: D1A1FFD8  stfs f13, -0x28(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 820603D8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 820603DC: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 820603E0: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060470 size=92
    let mut pc: u32 = 0x82060470;
    'dispatch: loop {
        match pc {
            0x82060470 => {
    //   block [0x82060470..0x820604CC)
	// 82060470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82060474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8206047C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82060480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82060484: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82060488: C81F0020  lfd f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 8206048C: C9AB06F0  lfd f13, 0x6f0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 82060490: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82060494: 409A0020  bne cr6, 0x820604b4
	if !ctx.cr[6].eq {
	pc = 0x820604B4; continue 'dispatch;
	}
	// 82060498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8206049C: 480245F5  bl 0x82084a90
	ctx.lr = 0x820604A0;
	sub_82084A90(ctx, base);
	// 820604A0: C81F0058  lfd f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 820604A4: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 820604A8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 820604AC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 820604B0: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 820604B4: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 820604B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 820604BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 820604C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 820604C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 820604C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820604D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x820604D0 size=124
    let mut pc: u32 = 0x820604D0;
    'dispatch: loop {
        match pc {
            0x820604D0 => {
    //   block [0x820604D0..0x8206054C)
	// 820604D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 820604D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 820604D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 820604DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 820604E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 820604E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 820604E8: 480245A9  bl 0x82084a90
	ctx.lr = 0x820604EC;
	sub_82084A90(ctx, base);
	// 820604EC: C81F0058  lfd f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 820604F0: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 820604F4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 820604F8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 820604FC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82060500: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82060504: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 82060508: 419A0018  beq cr6, 0x82060520
	if ctx.cr[6].eq {
	pc = 0x82060520; continue 'dispatch;
	}
	// 8206050C: C9BF0020  lfd f13, 0x20(r31)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 82060510: FDA06828  fsub f13, f0, f13
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82060514: C99F0018  lfd f12, 0x18(r31)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 82060518: FDAD602A  fadd f13, f13, f12
	ctx.f[13].f64 = ctx.f[13].f64 + ctx.f[12].f64;
	// 8206051C: D9BF0018  stfd f13, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.f[13].u64 ) };
	// 82060520: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82060524: D81F0010  stfd f0, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.f[0].u64 ) };
	// 82060528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8206052C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82060530: C80B06F0  lfd f0, 0x6f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1776 as u32) ) };
	// 82060534: D81F0020  stfd f0, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.f[0].u64 ) };
	// 82060538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8206053C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82060540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82060544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82060548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060550 size=88
    let mut pc: u32 = 0x82060550;
    'dispatch: loop {
        match pc {
            0x82060550 => {
    //   block [0x82060550..0x820605A8)
	// 82060550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82060554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8206055C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82060560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82060564: 387F026C  addi r3, r31, 0x26c
	ctx.r[3].s64 = ctx.r[31].s64 + 620;
	// 82060568: 4802BA29  bl 0x8208bf90
	ctx.lr = 0x8206056C;
	sub_8208BF90(ctx, base);
	// 8206056C: 387F0230  addi r3, r31, 0x230
	ctx.r[3].s64 = ctx.r[31].s64 + 560;
	// 82060570: 4802BA21  bl 0x8208bf90
	ctx.lr = 0x82060574;
	sub_8208BF90(ctx, base);
	// 82060574: 387F0220  addi r3, r31, 0x220
	ctx.r[3].s64 = ctx.r[31].s64 + 544;
	// 82060578: 4802BA19  bl 0x8208bf90
	ctx.lr = 0x8206057C;
	sub_8208BF90(ctx, base);
	// 8206057C: 387F0210  addi r3, r31, 0x210
	ctx.r[3].s64 = ctx.r[31].s64 + 528;
	// 82060580: 4802BA11  bl 0x8208bf90
	ctx.lr = 0x82060584;
	sub_8208BF90(ctx, base);
	// 82060584: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 82060588: 4802A8D9  bl 0x8208ae60
	ctx.lr = 0x8206058C;
	sub_8208AE60(ctx, base);
	// 8206058C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82060590: 4802B291  bl 0x8208b820
	ctx.lr = 0x82060594;
	sub_8208B820(ctx, base);
	// 82060594: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82060598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8206059C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 820605A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 820605A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820605A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x820605A8 size=80
    let mut pc: u32 = 0x820605A8;
    'dispatch: loop {
        match pc {
            0x820605A8 => {
    //   block [0x820605A8..0x820605F8)
	// 820605A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 820605AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 820605B0: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 820605B4: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 820605B8: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 820605BC: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 820605C0: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 820605C4: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 820605C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 820605CC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 820605D0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 820605D4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 820605D8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 820605DC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 820605E0: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 820605E4: 4802E125  bl 0x8208e708
	ctx.lr = 0x820605E8;
	sub_8208E708(ctx, base);
	// 820605E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 820605EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 820605F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 820605F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820605F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x820605F8 size=80
    let mut pc: u32 = 0x820605F8;
    'dispatch: loop {
        match pc {
            0x820605F8 => {
    //   block [0x820605F8..0x82060648)
	// 820605F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 820605FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060600: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82060604: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82060608: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8206060C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82060610: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82060614: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82060618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8206061C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82060620: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82060624: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82060628: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 8206062C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82060630: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82060634: 4802E2AD  bl 0x8208e8e0
	ctx.lr = 0x82060638;
	sub_8208E8E0(ctx, base);
	// 82060638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8206063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82060640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82060644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060648 size=80
    let mut pc: u32 = 0x82060648;
    'dispatch: loop {
        match pc {
            0x82060648 => {
    //   block [0x82060648..0x82060698)
	// 82060648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060650: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82060654: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82060658: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8206065C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82060660: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82060664: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82060668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8206066C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82060670: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82060674: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82060678: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 8206067C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82060680: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82060684: 4802E085  bl 0x8208e708
	ctx.lr = 0x82060688;
	sub_8208E708(ctx, base);
	// 82060688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8206068C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82060690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82060694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060698 size=176
    let mut pc: u32 = 0x82060698;
    'dispatch: loop {
        match pc {
            0x82060698 => {
    //   block [0x82060698..0x82060748)
	// 82060698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 820606A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 820606A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 820606A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 820606AC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 820606B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 820606B4: 4BFFFC55  bl 0x82060308
	ctx.lr = 0x820606B8;
	sub_82060308(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060748 size=176
    let mut pc: u32 = 0x82060748;
    'dispatch: loop {
        match pc {
            0x82060748 => {
    //   block [0x82060748..0x820607F8)
	// 82060748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82060754: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82060758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8206075C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82060760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82060764: 4BFFFBA5  bl 0x82060308
	ctx.lr = 0x82060768;
	sub_82060308(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820607F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x820607F8 size=128
    let mut pc: u32 = 0x820607F8;
    'dispatch: loop {
        match pc {
            0x820607F8 => {
    //   block [0x820607F8..0x82060878)
	// 820607F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 820607FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060800: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82060804: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82060808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8206080C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82060810: 480242A1  bl 0x82084ab0
	ctx.lr = 0x82060814;
	sub_82084AB0(ctx, base);
	// 82060814: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82060818: 48024279  bl 0x82084a90
	ctx.lr = 0x8206081C;
	sub_82084A90(ctx, base);
	// 8206081C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82060820: C9A10050  lfd f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82060824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82060828: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8206082C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82060830: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82060834: B17F002C  sth r11, 0x2c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u16 ) };
	// 82060838: C80A06F0  lfd f0, 0x6f0(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(1776 as u32) ) };
	// 8206083C: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82060840: D81F0020  stfd f0, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.f[0].u64 ) };
	// 82060844: C81F0058  lfd f0, 0x58(r31)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 82060848: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8206084C: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 82060850: D81F0008  stfd f0, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.f[0].u64 ) };
	// 82060854: D81F0000  stfd f0, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82060858: D81F0018  stfd f0, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.f[0].u64 ) };
	// 8206085C: D81F0010  stfd f0, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.f[0].u64 ) };
	// 82060860: D81F0050  stfd f0, 0x50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82060864: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82060868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8206086C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82060870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82060874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060878 size=156
    let mut pc: u32 = 0x82060878;
    'dispatch: loop {
        match pc {
            0x82060878 => {
    //   block [0x82060878..0x82060914)
	// 82060878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060880: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82060884: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82060888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8206088C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82060890: 48024201  bl 0x82084a90
	ctx.lr = 0x82060894;
	sub_82084A90(ctx, base);
	// 82060894: C81F0058  lfd f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 82060898: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8206089C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 820608A0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 820608A4: C97F0050  lfd f11, 0x50(r31)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	// 820608A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 820608AC: C98B0718  lfd f12, 0x718(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(1816 as u32) ) };
	// 820608B0: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 820608B4: FDA05828  fsub f13, f0, f11
	ctx.f[13].f64 = ctx.f[0].f64 - ctx.f[11].f64;
	// 820608B8: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 820608BC: 40990040  ble cr6, 0x820608fc
	if !ctx.cr[6].gt {
	pc = 0x820608FC; continue 'dispatch;
	}
	// 820608C0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 820608C4: D81F0050  stfd f0, 0x50(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 820608C8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 820608CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 820608D0: 388A0700  addi r4, r10, 0x700
	ctx.r[4].s64 = ctx.r[10].s64 + 1792;
	// 820608D4: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 820608D8: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 820608DC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 820608E0: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 820608E4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 820608E8: FC006824  fdiv f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 / ctx.f[13].f64;
	// 820608EC: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 820608F0: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 820608F4: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 820608F8: 4BFFFCB1  bl 0x820605a8
	ctx.lr = 0x820608FC;
	sub_820605A8(ctx, base);
	// 820608FC: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82060900: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82060904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82060908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8206090C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82060910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82060918 size=112
    let mut pc: u32 = 0x82060918;
    'dispatch: loop {
        match pc {
            0x82060918 => {
    //   block [0x82060918..0x82060988)
	// 82060918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206091C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82060920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82060924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82060928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8206092C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82060930: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82060934: 396B0720  addi r11, r11, 0x720
	ctx.r[11].s64 = ctx.r[11].s64 + 1824;
	// 82060938: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8206093C: 4BFFFEBD  bl 0x820607f8
	ctx.lr = 0x82060940;
	sub_820607F8(ctx, base);
	// 82060940: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82060944: 4802A525  bl 0x8208ae68
	ctx.lr = 0x82060948;
	sub_8208AE68(ctx, base);
	// 82060948: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 8206094C: 4802A19D  bl 0x8208aae8
	ctx.lr = 0x82060950;
	sub_8208AAE8(ctx, base);
	// 82060950: 387F0210  addi r3, r31, 0x210
	ctx.r[3].s64 = ctx.r[31].s64 + 528;
	// 82060954: 4802B61D  bl 0x8208bf70
	ctx.lr = 0x82060958;
	sub_8208BF70(ctx, base);
	// 82060958: 387F0220  addi r3, r31, 0x220
	ctx.r[3].s64 = ctx.r[31].s64 + 544;
	// 8206095C: 4802B615  bl 0x8208bf70
	ctx.lr = 0x82060960;
	sub_8208BF70(ctx, base);
	// 82060960: 387F0230  addi r3, r31, 0x230
	ctx.r[3].s64 = ctx.r[31].s64 + 560;
	// 82060964: 4802B60D  bl 0x8208bf70
	ctx.lr = 0x82060968;
	sub_8208BF70(ctx, base);
	// 82060968: 387F026C  addi r3, r31, 0x26c
	ctx.r[3].s64 = ctx.r[31].s64 + 620;
	// 8206096C: 4802B605  bl 0x8208bf70
	ctx.lr = 0x82060970;
	sub_8208BF70(ctx, base);
	// 82060970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82060974: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82060978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8206097C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82060980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82060984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82060988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82060988 size=1804
    let mut pc: u32 = 0x82060988;
    'dispatch: loop {
        match pc {
            0x82060988 => {
    //   block [0x82060988..0x82061094)
	// 82060988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206098C: 4802E0F9  bl 0x8208ea84
	ctx.lr = 0x82060990;
	sub_8208EA60(ctx, base);
	// 82060990: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82060994: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82061098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82061098 size=3180
    let mut pc: u32 = 0x82061098;
    'dispatch: loop {
        match pc {
            0x82061098 => {
    //   block [0x82061098..0x82061D04)
	// 82061098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8206109C: 4802D9D5  bl 0x8208ea70
	ctx.lr = 0x820610A0;
	sub_8208EA60(ctx, base);
	// 820610A0: 3981FF88  addi r12, r1, -0x78
	ctx.r[12].s64 = ctx.r[1].s64 + -120;
	// 820610A4: 4802DBB1  bl 0x8208ec54
	ctx.lr = 0x820610A8;
	sub_8208EC20(ctx, base);
	// 820610A8: 3981FF60  addi r12, r1, -0xa0
	ctx.r[12].s64 = ctx.r[1].s64 + -160;
	// 820610AC: 4802DE79  bl 0x8208ef24
	ctx.lr = 0x820610B0;
	sub_8208ECC0(ctx, base);
	// 820610B0: 9421FC50  stwu r1, -0x3b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-944 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 820610B4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 820610B8: 3BFA0008  addi r31, r26, 8
	ctx.r[31].s64 = ctx.r[26].s64 + 8;
	// 820610BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 820610C0: 4BFFF3B1  bl 0x82060470
	ctx.lr = 0x820610C4;
	sub_82060470(ctx, base);
	// 820610C4: C81A0020  lfd f0, 0x20(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) };
	// 820610C8: FC010028  fsub f0, f1, f0
	ctx.f[0].f64 = ctx.f[1].f64 - ctx.f[0].f64;
	// 820610CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 820610D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 820610D4: FFC00018  frsp f30, f0
	ctx.f[30].f64 = (ctx.f[0].f64 as f32) as f64;
	// 820610D8: 4802CC59  bl 0x8208dd30
	ctx.lr = 0x820610DC;
	sub_8208DD30(ctx, base);
	// 820610DC: A1630028  lhz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 820610E0: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 820610E4: 41820048  beq 0x8206112c
	if ctx.cr[0].eq {
	pc = 0x8206112C; continue 'dispatch;
	}
	// 820610E8: 3D408228  lis r10, -0x7dd8
	ctx.r[10].s64 = -2111307776;
	// 820610EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 820610F0: 816A70E4  lwz r11, 0x70e4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28900 as u32) ) } as u64;
	// 820610F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 820610F8: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 820610FC: 916A70E4  stw r11, 0x70e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28900 as u32), ctx.r[11].u32 ) };
	// 82061100: 41820028  beq 0x82061128
	if ctx.cr[0].eq {
	pc = 0x82061128; continue 'dispatch;
	}
	// 82061104: 4BFFF36D  bl 0x82060470
	ctx.lr = 0x82061108;
	sub_82060470(ctx, base);
	// 82061108: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8206110C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82061110: 409A001C  bne cr6, 0x8206112c
	if !ctx.cr[6].eq {
	pc = 0x8206112C; continue 'dispatch;
	}
	// 82061114: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82061118: D83F0020  stfd f1, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 8206111C: D83F0010  stfd f1, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.f[1].u64 ) };
	// 82061120: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82061124: 48000008  b 0x8206112c
	pc = 0x8206112C; continue 'dispatch;
	// 82061128: 4BFFF3A9  bl 0x820604d0
	ctx.lr = 0x8206112C;
	sub_820604D0(ctx, base);
	// 8206112C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82061130: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82061134: C00B0A40  lfs f0, 0xa40(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82061138: EFBE0032  fmuls f29, f30, f0
	ctx.f[29].f64 = (((ctx.f[30].f64 * ctx.f[0].f64) as f32) as f64);
	// 8206113C: C1AA0A3C  lfs f13, 0xa3c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2620 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82061140: EFFE0372  fmuls f31, f30, f13
	ctx.f[31].f64 = (((ctx.f[30].f64 * ctx.f[13].f64) as f32) as f64);
	// 82061144: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82061148: 4802D1A1  bl 0x8208e2e8
	ctx.lr = 0x8206114C;
	sub_8208E2E8(ctx, base);
	// 8206114C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82061D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82061D08 size=1256
    let mut pc: u32 = 0x82061D08;
    'dispatch: loop {
        match pc {
            0x82061D08 => {
    //   block [0x82061D08..0x820621F0)
	// 82061D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82061D0C: 4802CD85  bl 0x8208ea90
	ctx.lr = 0x82061D10;
	sub_8208EA60(ctx, base);
	// 82061D10: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 82061D14: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82061D18: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82061D1C: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82061D20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82061D24: 3F608229  lis r27, -0x7dd7
	ctx.r[27].s64 = -2111242240;
	// 82061D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82061D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82061D30: 38E04080  li r7, 0x4080
	ctx.r[7].s64 = 16512;
	// 82061D34: C3CB06E0  lfs f30, 0x6e0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1760 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82061D38: 38C0003F  li r6, 0x3f
	ctx.r[6].s64 = 63;
	// 82061D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82061D40: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82061D44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061D48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82061D4C: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061D50: 4800E579  bl 0x820702c8
	ctx.lr = 0x82061D54;
	sub_820702C8(ctx, base);
	// 82061D54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82061D58: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061D5C: 48000F1D  bl 0x82062c78
	ctx.lr = 0x82061D60;
	sub_82062C78(ctx, base);
	// 82061D60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061D64: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061D68: 48000881  bl 0x820625e8
	ctx.lr = 0x82061D6C;
	sub_820625E8(ctx, base);
	// 82061D6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82061D70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061D74: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061D78: 48002081  bl 0x82063df8
	ctx.lr = 0x82061D7C;
	sub_82063DF8(ctx, base);
	// 82061D7C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82061D80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061D84: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061D88: 48002219  bl 0x82063fa0
	ctx.lr = 0x82061D8C;
	sub_82063FA0(ctx, base);
	// 82061D8C: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061D90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82061D94: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82061D98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82061D9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82061DA0: 810B048C  lwz r8, 0x48c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1164 as u32) ) } as u64;
	// 82061DA4: 5508024C  rlwinm r8, r8, 0, 9, 6
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82061DA8: 910B048C  stw r8, 0x48c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1164 as u32), ctx.r[8].u32 ) };
	// 82061DAC: E9690018  ld r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	// 82061DB0: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82061DB4: F9690018  std r11, 0x18(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82061DB8: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061DBC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82061DC0: 810B0480  lwz r8, 0x480(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1152 as u32) ) } as u64;
	// 82061DC4: 514854EA  rlwimi r8, r10, 0xa, 0x13, 0x15
	ctx.r[8].u64 = (((ctx.r[10].u32).rotate_left(10) as u64) & 0x0000000000001C00) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFE3FF);
	// 82061DC8: 910B0480  stw r8, 0x480(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1152 as u32), ctx.r[8].u32 ) };
	// 82061DCC: E9690018  ld r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	// 82061DD0: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82061DD4: F9690018  std r11, 0x18(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82061DD8: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061DDC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82061DE0: 810B0480  lwz r8, 0x480(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1152 as u32) ) } as u64;
	// 82061DE4: 51486C24  rlwimi r8, r10, 0xd, 0x10, 0x12
	ctx.r[8].u64 = (((ctx.r[10].u32).rotate_left(13) as u64) & 0x000000000000E000) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF1FFF);
	// 82061DE8: 910B0480  stw r8, 0x480(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1152 as u32), ctx.r[8].u32 ) };
	// 82061DEC: E9690018  ld r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) };
	// 82061DF0: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82061DF4: F9690018  std r11, 0x18(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82061DF8: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061DFC: 48001FFD  bl 0x82063df8
	ctx.lr = 0x82061E00;
	sub_82063DF8(ctx, base);
	// 82061E00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82061E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82061E08: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061E0C: 48002195  bl 0x82063fa0
	ctx.lr = 0x82061E10;
	sub_82063FA0(ctx, base);
	// 82061E10: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061E14: 394B0018  addi r10, r11, 0x18
	ctx.r[10].s64 = ctx.r[11].s64 + 24;
	// 82061E18: 814B04A4  lwz r10, 0x4a4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1188 as u32) ) } as u64;
	// 82061E1C: 554A024C  rlwinm r10, r10, 0, 9, 6
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82061E20: 914B04A4  stw r10, 0x4a4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1188 as u32), ctx.r[10].u32 ) };
	// 82061E24: E94B0018  ld r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82061E28: 654A4000  oris r10, r10, 0x4000
	ctx.r[10].u64 = ctx.r[10].u64 | 1073741824;
	// 82061E2C: F94B0018  std r10, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 82061E30: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061E34: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82061E38: 812B0498  lwz r9, 0x498(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1176 as u32) ) } as u64;
	// 82061E3C: 552905A4  rlwinm r9, r9, 0, 0x16, 0x12
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82061E40: 912B0498  stw r9, 0x498(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1176 as u32), ctx.r[9].u32 ) };
	// 82061E44: E96A0018  ld r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) };
	// 82061E48: 656B4000  oris r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 1073741824;
	// 82061E4C: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82061E50: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061E54: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82061E58: 812B0498  lwz r9, 0x498(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1176 as u32) ) } as u64;
	// 82061E5C: 552904DE  rlwinm r9, r9, 0, 0x13, 0xf
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82061E60: 912B0498  stw r9, 0x498(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1176 as u32), ctx.r[9].u32 ) };
	// 82061E64: E96A0018  ld r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) };
	// 82061E68: 656B4000  oris r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 1073741824;
	// 82061E6C: F96A0018  std r11, 0x18(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82061E70: 809F0324  lwz r4, 0x324(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(804 as u32) ) } as u64;
	// 82061E74: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061E78: 4802ACE1  bl 0x8208cb58
	ctx.lr = 0x82061E7C;
	sub_8208CB58(ctx, base);
	// 82061E7C: 815BC6B8  lwz r10, -0x3948(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061E80: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82061E84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82061E88: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82061E8C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82061E90: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82061E94: D3C9178C  stfs f30, 0x178c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(6028 as u32), tmp.u32 ) };
	// 82061E98: 3CA08227  lis r5, -0x7dd9
	ctx.r[5].s64 = -2111373312;
	// 82061E9C: C1AB06FC  lfs f13, 0x6fc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1788 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82061EA0: 78CBFFE6  rldicr r11, r6, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[6].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82061EA4: C3E806F8  lfs f31, 0x6f8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(1784 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82061EA8: 38650050  addi r3, r5, 0x50
	ctx.r[3].s64 = ctx.r[5].s64 + 80;
	// 82061EAC: C0070A20  lfs f0, 0xa20(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2592 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82061EB0: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82061EB4: D3EA1780  stfs f31, 0x1780(r10)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6016 as u32), tmp.u32 ) };
	// 82061EB8: 3CC04000  lis r6, 0x4000
	ctx.r[6].s64 = 1073741824;
	// 82061EBC: D1A91788  stfs f13, 0x1788(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(6024 as u32), tmp.u32 ) };
	// 82061EC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82061EC4: D0091784  stfs f0, 0x1784(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(6020 as u32), tmp.u32 ) };
	// 82061EC8: E9490008  ld r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82061ECC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82061ED0: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82061ED4: C0050050  lfs f0, 0x50(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82061ED8: 817BC6B8  lwz r11, -0x3948(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061EDC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82061EE0: D00B1790  stfs f0, 0x1790(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(6032 as u32), tmp.u32 ) };
	// 82061EE4: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82061EE8: D00A1794  stfs f0, 0x1794(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6036 as u32), tmp.u32 ) };
	// 82061EEC: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82061EF0: D00A1798  stfs f0, 0x1798(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6040 as u32), tmp.u32 ) };
	// 82061EF4: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82061EF8: D00A179C  stfs f0, 0x179c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6044 as u32), tmp.u32 ) };
	// 82061EFC: E96A0008  ld r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82061F00: 7D6B3B78  or r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 82061F04: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82061F08: 80BF0320  lwz r5, 0x320(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 82061F0C: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F10: 48005231  bl 0x82067140
	ctx.lr = 0x82061F14;
	sub_82067140(ctx, base);
	// 82061F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82061F18: 64C68000  oris r6, r6, 0x8000
	ctx.r[6].u64 = ctx.r[6].u64 | 2147483648;
	// 82061F1C: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061F24: 80BF027C  lwz r5, 0x27c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 82061F28: 48005219  bl 0x82067140
	ctx.lr = 0x82061F2C;
	sub_82067140(ctx, base);
	// 82061F2C: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F30: 809F0298  lwz r4, 0x298(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) } as u64;
	// 82061F34: 4802AB65  bl 0x8208ca98
	ctx.lr = 0x82061F38;
	sub_8208CA98(ctx, base);
	// 82061F38: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F3C: 809F029C  lwz r4, 0x29c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 82061F40: 4802ABB1  bl 0x8208caf0
	ctx.lr = 0x82061F44;
	sub_8208CAF0(ctx, base);
	// 82061F44: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82061F48: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82061F50: 80FF028C  lwz r7, 0x28c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(652 as u32) ) } as u64;
	// 82061F54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061F58: 80BF0280  lwz r5, 0x280(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(640 as u32) ) } as u64;
	// 82061F5C: 480029BD  bl 0x82064918
	ctx.lr = 0x82061F60;
	sub_82064918(ctx, base);
	// 82061F60: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F64: 809F0284  lwz r4, 0x284(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(644 as u32) ) } as u64;
	// 82061F68: 48002AD1  bl 0x82064a38
	ctx.lr = 0x82061F6C;
	sub_82064A38(ctx, base);
	// 82061F6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82061F70: 809F0288  lwz r4, 0x288(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(648 as u32) ) } as u64;
	// 82061F74: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 82061F78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82061F7C: 3BCB0600  addi r30, r11, 0x600
	ctx.r[30].s64 = ctx.r[11].s64 + 1536;
	// 82061F80: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061F84: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82061F88: 393E0004  addi r9, r30, 4
	ctx.r[9].s64 = ctx.r[30].s64 + 4;
	// 82061F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82061F90: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82061F94: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82061F98: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82061F9C: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82061FA0: 4800E891  bl 0x82070830
	ctx.lr = 0x82061FA4;
	sub_82070830(ctx, base);
	// 82061FA4: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061FA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82061FAC: 80BF0240  lwz r5, 0x240(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(576 as u32) ) } as u64;
	// 82061FB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061FB4: 64C68000  oris r6, r6, 0x8000
	ctx.r[6].u64 = ctx.r[6].u64 | 2147483648;
	// 82061FB8: 48005189  bl 0x82067140
	ctx.lr = 0x82061FBC;
	sub_82067140(ctx, base);
	// 82061FBC: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061FC0: 809F0264  lwz r4, 0x264(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(612 as u32) ) } as u64;
	// 82061FC4: 4802AAD5  bl 0x8208ca98
	ctx.lr = 0x82061FC8;
	sub_8208CA98(ctx, base);
	// 82061FC8: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061FCC: 809F0268  lwz r4, 0x268(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82061FD0: 4802AB21  bl 0x8208caf0
	ctx.lr = 0x82061FD4;
	sub_8208CAF0(ctx, base);
	// 82061FD4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82061FD8: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82061FE0: 80FF0258  lwz r7, 0x258(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 82061FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82061FE8: 80BF0244  lwz r5, 0x244(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82061FEC: 4800292D  bl 0x82064918
	ctx.lr = 0x82061FF0;
	sub_82064918(ctx, base);
	// 82061FF0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82061FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82061FF8: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82061FFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82062000: 80FF0258  lwz r7, 0x258(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 82062004: 80BF0248  lwz r5, 0x248(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 82062008: 48002911  bl 0x82064918
	ctx.lr = 0x8206200C;
	sub_82064918(ctx, base);
	// 8206200C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82062010: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82062014: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 82062018: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8206201C: 80FF0258  lwz r7, 0x258(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 82062020: 80BF024C  lwz r5, 0x24c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 82062024: 480028F5  bl 0x82064918
	ctx.lr = 0x82062028;
	sub_82064918(ctx, base);
	// 82062028: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 8206202C: 809F0250  lwz r4, 0x250(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 82062030: 48002A09  bl 0x82064a38
	ctx.lr = 0x82062034;
	sub_82064A38(ctx, base);
	// 82062034: 809F0254  lwz r4, 0x254(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(596 as u32) ) } as u64;
	// 82062038: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8206203C: 815F0260  lwz r10, 0x260(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 82062040: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82062044: 54891838  slwi r9, r4, 3
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82062048: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 8206204C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82062050: 7D09F02E  lwzx r8, r9, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82062054: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82062058: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8206205C: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82062060: 4800E7D1  bl 0x82070830
	ctx.lr = 0x82062064;
	sub_82070830(ctx, base);
	// 82062064: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82062068: 3BDF0068  addi r30, r31, 0x68
	ctx.r[30].s64 = ctx.r[31].s64 + 104;
	// 8206206C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82062070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82062074: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82062078: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 8206207C: 48029385  bl 0x8208b400
	ctx.lr = 0x82062080;
	sub_8208B400(ctx, base);
	// 82062080: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82062084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82062088: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8206208C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82062090: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82062094: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82062098: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8206209C: C00B0AA0  lfs f0, 0xaa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2720 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 820620A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 820620A4: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 820620A8: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 820620AC: 38EB0A90  addi r7, r11, 0xa90
	ctx.r[7].s64 = ctx.r[11].s64 + 2704;
	// 820620B0: 48029829  bl 0x8208b8d8
	ctx.lr = 0x820620B4;
	sub_8208B8D8(ctx, base);
	// 820620B4: D3DF0094  stfs f30, 0x94(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 820620B8: D3DF0098  stfs f30, 0x98(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 820620BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 820620C0: 4BFFE7B9  bl 0x82060878
	ctx.lr = 0x820620C4;
	sub_82060878(ctx, base);
	// 820620C4: 3B80FF00  li r28, -0x100
	ctx.r[28].s64 = -256;
	// 820620C8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 820620CC: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 820620D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 820620D4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 820620D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 820620DC: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 820620E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 820620E4: 480297F5  bl 0x8208b8d8
	ctx.lr = 0x820620E8;
	sub_8208B8D8(ctx, base);
	// 820620E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 820620EC: 4BFFE385  bl 0x82060470
	ctx.lr = 0x820620F0;
	sub_82060470(ctx, base);
	// 820620F0: C81F0020  lfd f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 820620F4: FC210028  fsub f1, f1, f0
	ctx.f[1].f64 = ctx.f[1].f64 - ctx.f[0].f64;
	// 820620F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 820620FC: C80B0A88  lfd f0, 0xa88(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(2696 as u32) ) };
	// 82062100: FFC10032  fmul f30, f1, f0
	ctx.f[30].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 82062104: FFBE0032  fmul f29, f30, f0
	ctx.f[29].f64 = ctx.f[30].f64 * ctx.f[0].f64;
	// 82062108: 4802D619  bl 0x8208f720
	ctx.lr = 0x8206210C;
	sub_8208F720(ctx, base);
	// 8206210C: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82062110: 3BE0003C  li r31, 0x3c
	ctx.r[31].s64 = 60;
	// 82062114: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82062118: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8206211C: 7D4BFB96  divwu r10, r11, r31
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82062120: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82062124: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 82062128: 7FAA5850  subf r29, r10, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8206212C: 4802D5F5  bl 0x8208f720
	ctx.lr = 0x82062130;
	sub_8208F720(ctx, base);
	// 82062130: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82062134: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82062138: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8206213C: 7D4BFB96  divwu r10, r11, r31
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82062140: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 82062144: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82062148: 7FEA5850  subf r31, r10, r11
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8206214C: 4802D5D5  bl 0x8208f720
	ctx.lr = 0x82062150;
	sub_8208F720(ctx, base);
	// 82062150: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82062154: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82062158: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8206215C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82062160: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82062164: 7D695B96  divwu r11, r9, r11
	ctx.r[11].u32 = ctx.r[9].u32 / ctx.r[11].u32;
	// 82062168: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8206216C: C80A0A80  lfd f0, 0xa80(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(2688 as u32) ) };
	// 82062170: FC3D0032  fmul f1, f29, f0
	ctx.f[1].f64 = ctx.f[29].f64 * ctx.f[0].f64;
	// 82062174: 7F4B4850  subf r26, r11, r9
	ctx.r[26].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82062178: 4802D5A9  bl 0x8208f720
	ctx.lr = 0x8206217C;
	sub_8208F720(ctx, base);
	// 8206217C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82062180: FC000E5E  fctidz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[1].f64.trunc() as i64 };
	// 82062184: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82062188: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8206218C: 388B0A48  addi r4, r11, 0xa48
	ctx.r[4].s64 = ctx.r[11].s64 + 2632;
	// 82062190: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82062194: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82062198: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8206219C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 820621A0: 4BFFE4A9  bl 0x82060648
	ctx.lr = 0x820621A4;
	sub_82060648(ctx, base);
	// 820621A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 820621A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 820621AC: C04B0A44  lfs f2, 0xa44(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2628 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 820621B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 820621B4: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 820621B8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 820621BC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 820621C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 820621C4: 48029715  bl 0x8208b8d8
	ctx.lr = 0x820621C8;
	sub_8208B8D8(ctx, base);
	// 820621C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 820621CC: 480294E5  bl 0x8208b6b0
	ctx.lr = 0x820621D0;
	sub_8208B6B0(ctx, base);
	// 820621D0: 807BC6B8  lwz r3, -0x3948(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-14664 as u32) ) } as u64;
	// 820621D4: 4800CCF5  bl 0x8206eec8
	ctx.lr = 0x820621D8;
	sub_8206EEC8(ctx, base);
	// 820621D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 820621DC: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 820621E0: CBA1FFB0  lfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 820621E4: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 820621E8: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 820621EC: 4802C8F4  b 0x8208eae0
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820621F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x820621F0 size=112
    let mut pc: u32 = 0x820621F0;
    'dispatch: loop {
        match pc {
            0x820621F0 => {
    //   block [0x820621F0..0x82062260)
	// 820621F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 820621F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 820621F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 820621FC: 9421FC70  stwu r1, -0x390(r1)
	ea = ctx.r[1].u32.wrapping_add(-912 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82062200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82062204: 4BFFE715  bl 0x82060918
	ctx.lr = 0x82062208;
	sub_82060918(ctx, base);
	// 82062208: 3D608227  lis r11, -0x7dd9
	ctx.r[11].s64 = -2111373312;
	// 8206220C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82062210: 3BEB1A20  addi r31, r11, 0x1a20
	ctx.r[31].s64 = ctx.r[11].s64 + 6688;
	// 82062214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82062218: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8206221C: 4802A275  bl 0x8208c490
	ctx.lr = 0x82062220;
	sub_8208C490(ctx, base);
	// 82062220: 3D601828  lis r11, 0x1828
	ctx.r[11].s64 = 405274624;
	// 82062224: 3D402828  lis r10, 0x2828
	ctx.r[10].s64 = 673710080;
	// 82062228: 616B7F86  ori r11, r11, 0x7f86
	ctx.r[11].u64 = ctx.r[11].u64 | 32646;
	// 8206222C: 614A7F06  ori r10, r10, 0x7f06
	ctx.r[10].u64 = ctx.r[10].u64 | 32518;
	// 82062230: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82062234: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82062238: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8206223C: 4802A065  bl 0x8208c2a0
	ctx.lr = 0x82062240;
	sub_8208C2A0(ctx, base);
	// 82062240: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82062244: 4BFFE30D  bl 0x82060550
	ctx.lr = 0x82062248;
	sub_82060550(ctx, base);
	// 82062248: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8206224C: 38210390  addi r1, r1, 0x390
	ctx.r[1].s64 = ctx.r[1].s64 + 912;
	// 82062250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82062254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82062258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8206225C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82062260 size=752
    let mut pc: u32 = 0x82062260;
    'dispatch: loop {
        match pc {
            0x82062260 => {
    //   block [0x82062260..0x82062550)
	// 82062260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82062264: 4802C819  bl 0x8208ea7c
	ctx.lr = 0x82062268;
	sub_8208EA60(ctx, base);
	// 82062268: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8206226C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82062270: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82062274: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82062278: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8206227C: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82062280: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82062284: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82062288: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8206228C: 4099000C  ble cr6, 0x82062298
	if !ctx.cr[6].gt {
	pc = 0x82062298; continue 'dispatch;
	}
	// 82062290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82062294: 48010F9D  bl 0x82073230
	ctx.lr = 0x82062298;
	sub_82073230(ctx, base);
	// 82062298: 897F2ABC  lbz r11, 0x2abc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 8206229C: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820622A0: 4182000C  beq 0x820622ac
	if ctx.cr[0].eq {
	pc = 0x820622AC; continue 'dispatch;
	}
	// 820622A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 820622A8: 48000094  b 0x8206233c
	pc = 0x8206233C; continue 'dispatch;
	// 820622AC: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 820622B0: 41820084  beq 0x82062334
	if ctx.cr[0].eq {
	pc = 0x82062334; continue 'dispatch;
	}
	// 820622B4: 817F3098  lwz r11, 0x3098(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12440 as u32) ) } as u64;
	// 820622B8: 815F31B8  lwz r10, 0x31b8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12728 as u32) ) } as u64;
	// 820622BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 820622C0: 419A000C  beq cr6, 0x820622cc
	if ctx.cr[6].eq {
	pc = 0x820622CC; continue 'dispatch;
	}
	// 820622C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 820622C8: 409A006C  bne cr6, 0x82062334
	if !ctx.cr[6].eq {
	pc = 0x82062334; continue 'dispatch;
	}
	// 820622CC: 817F309C  lwz r11, 0x309c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12444 as u32) ) } as u64;
	// 820622D0: 815F31BC  lwz r10, 0x31bc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12732 as u32) ) } as u64;
	// 820622D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 820622D8: 419A000C  beq cr6, 0x820622e4
	if ctx.cr[6].eq {
	pc = 0x820622E4; continue 'dispatch;
	}
	// 820622DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 820622E0: 409A0054  bne cr6, 0x82062334
	if !ctx.cr[6].eq {
	pc = 0x82062334; continue 'dispatch;
	}
	// 820622E4: 817F30A0  lwz r11, 0x30a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12448 as u32) ) } as u64;
	// 820622E8: 815F31C0  lwz r10, 0x31c0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12736 as u32) ) } as u64;
	// 820622EC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 820622F0: 419A000C  beq cr6, 0x820622fc
	if ctx.cr[6].eq {
	pc = 0x820622FC; continue 'dispatch;
	}
	// 820622F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 820622F8: 409A003C  bne cr6, 0x82062334
	if !ctx.cr[6].eq {
	pc = 0x82062334; continue 'dispatch;
	}
	// 820622FC: 817F30A4  lwz r11, 0x30a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82062300: 815F31C4  lwz r10, 0x31c4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12740 as u32) ) } as u64;
	// 82062304: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82062308: 419A000C  beq cr6, 0x82062314
	if ctx.cr[6].eq {
	pc = 0x82062314; continue 'dispatch;
	}
	// 8206230C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82062310: 409A0024  bne cr6, 0x82062334
	if !ctx.cr[6].eq {
	pc = 0x82062334; continue 'dispatch;
	}
	// 82062314: 817F30A8  lwz r11, 0x30a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82062318: 815F31C8  lwz r10, 0x31c8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12744 as u32) ) } as u64;
	// 8206231C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82062320: 419A000C  beq cr6, 0x8206232c
	if ctx.cr[6].eq {
	pc = 0x8206232C; continue 'dispatch;
	}
	// 82062324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82062328: 409A000C  bne cr6, 0x82062334
	if !ctx.cr[6].eq {
	pc = 0x82062334; continue 'dispatch;
	}
	// 8206232C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82062330: 48000008  b 0x82062338
	pc = 0x82062338; continue 'dispatch;
	// 82062334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82062338: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8206233C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82062340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82062344: 4082003C  bne 0x82062380
	if !ctx.cr[0].eq {
	pc = 0x82062380; continue 'dispatch;
	}
	// 82062348: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8206234C: 5709809E  rlwinm r9, r24, 0x10, 2, 0xf
	ctx.r[9].u64 = ctx.r[24].u32 as u64 & 0x0000FFFFu64;
	// 82062350: 616B2080  ori r11, r11, 0x2080
	ctx.r[11].u64 = ctx.r[11].u64 | 8320;
	// 82062354: 56C8809E  rlwinm r8, r22, 0x10, 2, 0xf
	ctx.r[8].u64 = ctx.r[22].u32 as u64 & 0x0000FFFFu64;
	// 82062358: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 8206235C: 572B04BE  clrlwi r11, r25, 0x12
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x00003FFFu64;
	// 82062360: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82062364: 56E904BE  clrlwi r9, r23, 0x12
	ctx.r[9].u64 = ctx.r[23].u32 as u64 & 0x00003FFFu64;
	// 82062368: 95430004  stwu r10, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[3].u32 = ea;
	// 8206236C: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 82062370: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82062374: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82062378: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 8206237C: 480001C8  b 0x82062544
	pc = 0x82062544; continue 'dispatch;
	// 82062380: 3D60C000  lis r11, -0x4000
	ctx.r[11].s64 = -1073741824;
	// 82062384: 3D20C000  lis r9, -0x4000
	ctx.r[9].s64 = -1073741824;
	// 82062388: 616B6100  ori r11, r11, 0x6100
	ctx.r[11].u64 = ctx.r[11].u64 | 24832;
	// 8206238C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82062390: 95630004  stwu r11, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[11].u32) };
	ctx.r[3].u32 = ea;
	// 82062394: 613A6000  ori r26, r9, 0x6000
	ctx.r[26].u64 = ctx.r[9].u64 | 24576;
	// 82062398: 95430004  stwu r10, 4(r3)
	ea = ctx.r[3].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[3].u32 = ea;
	// 8206239C: 815F31CC  lwz r10, 0x31cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12748 as u32) ) } as u64;
	// 820623A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 820623A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 820623A8: 4099011C  ble cr6, 0x820624c4
	if !ctx.cr[6].gt {
	pc = 0x820624C4; continue 'dispatch;
	}
	// 820623AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 820623B0: 3BDF31D4  addi r30, r31, 0x31d4
	ctx.r[30].s64 = ctx.r[31].s64 + 12756;
	// 820623B4: 3B9F32C4  addi r28, r31, 0x32c4
	ctx.r[28].s64 = ctx.r[31].s64 + 12996;
	// 820623B8: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 820623BC: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 820623C0: 80DCFFFC  lwz r6, -4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-4 as u32) ) } as u64;
	// 820623C4: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 820623C8: 7F195000  cmpw cr6, r25, r10
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[10].s32, &mut ctx.xer);
	// 820623CC: 41990008  bgt cr6, 0x820623d4
	if ctx.cr[6].gt {
	pc = 0x820623D4; continue 'dispatch;
	}
	// 820623D0: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 820623D4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 820623D8: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 820623DC: 7F185000  cmpw cr6, r24, r10
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[10].s32, &mut ctx.xer);
	// 820623E0: 41990008  bgt cr6, 0x820623e8
	if ctx.cr[6].gt {
	pc = 0x820623E8; continue 'dispatch;
	}
	// 820623E4: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 820623E8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 820623EC: 7F174800  cmpw cr6, r23, r9
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[9].s32, &mut ctx.xer);
	// 820623F0: 40980008  bge cr6, 0x820623f8
	if !ctx.cr[6].lt {
	pc = 0x820623F8; continue 'dispatch;
	}
	// 820623F4: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 820623F8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 820623FC: 7F165000  cmpw cr6, r22, r10
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82062400: 40980008  bge cr6, 0x82062408
	if !ctx.cr[6].lt {
	pc = 0x82062408; continue 'dispatch;
	}
	// 82062404: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 82062408: 7F074800  cmpw cr6, r7, r9
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8206240C: 4098000C  bge cr6, 0x82062418
	if !ctx.cr[6].lt {
	pc = 0x82062418; continue 'dispatch;
	}
	// 82062410: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82062414: 41980014  blt cr6, 0x82062428
	if ctx.cr[6].lt {
	pc = 0x82062428; continue 'dispatch;
	}
	// 82062418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8206241C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82062420: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82062424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82062428: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8206242C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82062430: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 82062434: 3C80C003  lis r4, -0x3ffd
	ctx.r[4].s64 = -1073545216;
	// 82062438: 7C63E830  slw r3, r3, r29
	if (ctx.r[29].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[3].u32) << ((ctx.r[29].u8 & 0x1F) as u32)) as u64;
	}
	// 8206243C: 946B0004  stwu r3, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[3].u32) };
	ctx.r[11].u32 = ea;
	// 82062440: 60842D01  ori r4, r4, 0x2d01
	ctx.r[4].u64 = ctx.r[4].u64 | 11521;
	// 82062444: 3EA00004  lis r21, 4
	ctx.r[21].s64 = 262144;
	// 82062448: 7CC600D0  neg r6, r6
	ctx.r[6].s64 = -ctx.r[6].s64;
	// 8206244C: 7CA500D0  neg r5, r5
	ctx.r[5].s64 = -ctx.r[5].s64;
	// 82062450: 62B50080  ori r21, r21, 0x80
	ctx.r[21].u64 = ctx.r[21].u64 | 128;
	// 82062454: 948B0004  stwu r4, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[4].u32) };
	ctx.r[11].u32 = ea;
	// 82062458: 54A5805E  rlwinm r5, r5, 0x10, 1, 0xf
	ctx.r[5].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 8206245C: 54C6047E  clrlwi r6, r6, 0x11
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x00007FFFu64;
	// 82062460: 5508809E  rlwinm r8, r8, 0x10, 2, 0xf
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82062464: 7CA63378  or r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 82062468: 54E704BE  clrlwi r7, r7, 0x12
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00003FFFu64;
	// 8206246C: 96AB0004  stwu r21, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[21].u32) };
	ctx.r[11].u32 = ea;
	// 82062470: 554A809E  rlwinm r10, r10, 0x10, 2, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82062474: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82062478: 552904BE  clrlwi r9, r9, 0x12
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 8206247C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82062480: 94CB0004  stwu r6, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[6].u32) };
	ctx.r[11].u32 = ea;
	// 82062484: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 82062488: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 8206248C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82062490: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82062494: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82062498: 40990010  ble cr6, 0x820624a8
	if !ctx.cr[6].gt {
	pc = 0x820624A8; continue 'dispatch;
	}
	// 8206249C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 820624A0: 48010D91  bl 0x82073230
	ctx.lr = 0x820624A4;
	sub_82073230(ctx, base);
	// 820624A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 820624A8: 815F31CC  lwz r10, 0x31cc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12748 as u32) ) } as u64;
	// 820624AC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 820624B0: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 820624B4: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 820624B8: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 820624BC: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 820624C0: 4198FEF8  blt cr6, 0x820623b8
	if ctx.cr[6].lt {
	pc = 0x820623B8; continue 'dispatch;
	}
	// 820624C4: 895F2ABF  lbz r10, 0x2abf(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10943 as u32) ) } as u64;
	// 820624C8: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820624CC: 41820058  beq 0x82062524
	if ctx.cr[0].eq {
	pc = 0x82062524; continue 'dispatch;
	}
	// 820624D0: 895F2ABC  lbz r10, 0x2abc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10940 as u32) ) } as u64;
	// 820624D4: 554A0673  rlwinm. r10, r10, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820624D8: 4182004C  beq 0x82062524
	if ctx.cr[0].eq {
	pc = 0x82062524; continue 'dispatch;
	}
	// 820624DC: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 820624E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 820624E4: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 820624E8: 3D00C002  lis r8, -0x3ffe
	ctx.r[8].s64 = -1073610752;
	// 820624EC: 3CE00004  lis r7, 4
	ctx.r[7].s64 = 262144;
	// 820624F0: 610A2D01  ori r10, r8, 0x2d01
	ctx.r[10].u64 = ctx.r[8].u64 | 11521;
	// 820624F4: 60E80081  ori r8, r7, 0x81
	ctx.r[8].u64 = ctx.r[7].u64 | 129;
	// 820624F8: 5707809E  rlwinm r7, r24, 0x10, 2, 0xf
	ctx.r[7].u64 = ctx.r[24].u32 as u64 & 0x0000FFFFu64;
	// 820624FC: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82062500: 572604BE  clrlwi r6, r25, 0x12
	ctx.r[6].u64 = ctx.r[25].u32 as u64 & 0x00003FFFu64;
	// 82062504: 56C5809E  rlwinm r5, r22, 0x10, 2, 0xf
	ctx.r[5].u64 = ctx.r[22].u32 as u64 & 0x0000FFFFu64;
	// 82062508: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 8206250C: 56E904BE  clrlwi r9, r23, 0x12
	ctx.r[9].u64 = ctx.r[23].u32 as u64 & 0x00003FFFu64;
	// 82062510: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82062514: 7CA94B78  or r9, r5, r9
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 82062518: 950B0004  stwu r8, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[8].u32) };
	ctx.r[11].u32 = ea;
	// 8206251C: 94EB0004  stwu r7, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[7].u32) };
	ctx.r[11].u32 = ea;
	// 82062520: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82062524: 974B0004  stwu r26, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[26].u32) };
	ctx.r[11].u32 = ea;
	// 82062528: 3D40C000  lis r10, -0x4000
	ctx.r[10].s64 = -1073741824;
	// 8206252C: 813F31A4  lwz r9, 0x31a4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12708 as u32) ) } as u64;
	// 82062530: 614A6100  ori r10, r10, 0x6100
	ctx.r[10].u64 = ctx.r[10].u64 | 24832;
	// 82062534: 952B0004  stwu r9, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[9].u32) };
	ctx.r[11].u32 = ea;
	// 82062538: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 8206253C: 815F31A8  lwz r10, 0x31a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12712 as u32) ) } as u64;
	// 82062540: 954B0004  stwu r10, 4(r11)
	ea = ctx.r[11].u32.wrapping_add(4 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[10].u32) };
	ctx.r[11].u32 = ea;
	// 82062544: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82062548: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8206254C: 4802C580  b 0x8208eacc
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062550 size=28
    let mut pc: u32 = 0x82062550;
    'dispatch: loop {
        match pc {
            0x82062550 => {
    //   block [0x82062550..0x8206256C)
	// 82062550: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82062554: 51640038  rlwimi r4, r11, 0, 0, 0x1c
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF8) | (ctx.r[4].u64 & 0xFFFFFFFF00000007);
	// 82062558: 90832948  stw r4, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[4].u32 ) };
	// 8206255C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062560: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82062564: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062570 size=12
    let mut pc: u32 = 0x82062570;
    'dispatch: loop {
        match pc {
            0x82062570 => {
    //   block [0x82062570..0x8206257C)
	// 82062570: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82062574: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82062578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062580 size=28
    let mut pc: u32 = 0x82062580;
    'dispatch: loop {
        match pc {
            0x82062580 => {
    //   block [0x82062580..0x8206259C)
	// 82062580: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82062584: 508B1D78  rlwimi r11, r4, 3, 0x15, 0x1c
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x00000000000007F8) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF807);
	// 82062588: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 8206258C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062590: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82062594: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820625A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820625A0 size=12
    let mut pc: u32 = 0x820625A0;
    'dispatch: loop {
        match pc {
            0x820625A0 => {
    //   block [0x820625A0..0x820625AC)
	// 820625A0: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 820625A4: 5563EE3E  rlwinm r3, r11, 0x1d, 0x18, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 820625A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820625B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820625B0 size=36
    let mut pc: u32 = 0x820625B0;
    'dispatch: loop {
        match pc {
            0x820625B0 => {
    //   block [0x820625B0..0x820625D4)
	// 820625B0: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 820625B4: 508B1F38  rlwimi r11, r4, 3, 0x1c, 0x1c
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x0000000000000008) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFF7);
	// 820625B8: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 820625BC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820625C0: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 820625C4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820625C8: 656B0004  oris r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 262144;
	// 820625CC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820625D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820625D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820625D8 size=12
    let mut pc: u32 = 0x820625D8;
    'dispatch: loop {
        match pc {
            0x820625D8 => {
    //   block [0x820625D8..0x820625E4)
	// 820625D8: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 820625DC: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 820625E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820625E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820625E8 size=128
    let mut pc: u32 = 0x820625E8;
    'dispatch: loop {
        match pc {
            0x820625E8 => {
    //   block [0x820625E8..0x82062668)
	// 820625E8: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 820625EC: 508BF800  rlwimi r11, r4, 0x1f, 0, 0
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(31) as u64) & 0x0000000080000000) | (ctx.r[11].u64 & 0xFFFFFFFF7FFFFFFF);
	// 820625F0: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 820625F4: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820625F8: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 820625FC: 40820024  bne 0x82062620
	if !ctx.cr[0].eq {
	pc = 0x82062620; continue 'dispatch;
	}
	// 82062600: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82062604: 55692336  rlwinm r9, r11, 4, 0xc, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 82062608: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8206260C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82062610: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 82062614: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82062618: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8206261C: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82062620: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82062624: 409A000C  bne cr6, 0x82062630
	if !ctx.cr[6].eq {
	pc = 0x82062630; continue 'dispatch;
	}
	// 82062628: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8206262C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82062630: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 82062634: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 82062638: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 8206263C: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 82062640: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062644: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 82062648: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8206264C: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82062650: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062654: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82062658: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8206265C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82062660: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062668 size=12
    let mut pc: u32 = 0x82062668;
    'dispatch: loop {
        match pc {
            0x82062668 => {
    //   block [0x82062668..0x82062674)
	// 82062668: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 8206266C: 55630FFE  srwi r3, r11, 0x1f
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82062670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062678 size=24
    let mut pc: u32 = 0x82062678;
    'dispatch: loop {
        match pc {
            0x82062678 => {
    //   block [0x82062678..0x82062690)
	// 82062678: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206267C: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82062680: 508A2E34  rlwimi r10, r4, 5, 0x18, 0x1a
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(5) as u64) & 0x00000000000000E0) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF1F);
	// 82062684: 55690001  rlwinm. r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82062688: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 8206268C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062690 size=100
    let mut pc: u32 = 0x82062690;
    'dispatch: loop {
        match pc {
            0x82062690 => {
    //   block [0x82062690..0x820626F4)
	// 82062690: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82062694: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 82062698: 40820024  bne 0x820626bc
	if !ctx.cr[0].eq {
	pc = 0x820626BC; continue 'dispatch;
	}
	// 8206269C: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820626A0: 55692336  rlwinm r9, r11, 4, 0xc, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 820626A4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 820626A8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 820626AC: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 820626B0: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 820626B4: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 820626B8: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 820626BC: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 820626C0: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 820626C4: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 820626C8: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 820626CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820626D0: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 820626D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820626D8: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 820626DC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820626E0: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 820626E4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820626E8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 820626EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820626F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820626F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820626F8 size=12
    let mut pc: u32 = 0x820626F8;
    'dispatch: loop {
        match pc {
            0x820626F8 => {
    //   block [0x820626F8..0x82062704)
	// 820626F8: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 820626FC: 5563DF7E  rlwinm r3, r11, 0x1b, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82062700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062708 size=24
    let mut pc: u32 = 0x82062708;
    'dispatch: loop {
        match pc {
            0x82062708 => {
    //   block [0x82062708..0x82062720)
	// 82062708: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206270C: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82062710: 508A06FE  rlwimi r10, r4, 0, 0x1b, 0x1f
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x000000000000001F) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFE0);
	// 82062714: 55690001  rlwinm. r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82062718: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 8206271C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062720 size=100
    let mut pc: u32 = 0x82062720;
    'dispatch: loop {
        match pc {
            0x82062720 => {
    //   block [0x82062720..0x82062784)
	// 82062720: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82062724: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 82062728: 40820024  bne 0x8206274c
	if !ctx.cr[0].eq {
	pc = 0x8206274C; continue 'dispatch;
	}
	// 8206272C: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82062730: 55692336  rlwinm r9, r11, 4, 0xc, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 82062734: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82062738: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 8206273C: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 82062740: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82062744: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82062748: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8206274C: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 82062750: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 82062754: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 82062758: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 8206275C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062760: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 82062764: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062768: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 8206276C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062770: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82062774: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062778: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8206277C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062788 size=12
    let mut pc: u32 = 0x82062788;
    'dispatch: loop {
        match pc {
            0x82062788 => {
    //   block [0x82062788..0x82062794)
	// 82062788: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206278C: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82062790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062798 size=24
    let mut pc: u32 = 0x82062798;
    'dispatch: loop {
        match pc {
            0x82062798 => {
    //   block [0x82062798..0x820627B0)
	// 82062798: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206279C: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 820627A0: 508A44EE  rlwimi r10, r4, 8, 0x13, 0x17
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000001F00) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFE0FF);
	// 820627A4: 55690001  rlwinm. r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 820627A8: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 820627AC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820627B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820627B0 size=100
    let mut pc: u32 = 0x820627B0;
    'dispatch: loop {
        match pc {
            0x820627B0 => {
    //   block [0x820627B0..0x82062814)
	// 820627B0: 556A0043  rlwinm. r10, r11, 0, 1, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820627B4: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 820627B8: 40820024  bne 0x820627dc
	if !ctx.cr[0].eq {
	pc = 0x820627DC; continue 'dispatch;
	}
	// 820627BC: 716A1010  andi. r10, r11, 0x1010
	ctx.r[10].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820627C0: 55692336  rlwinm r9, r11, 4, 0xc, 0x1b
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 820627C4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 820627C8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 820627CC: 554A601E  rlwinm r10, r10, 0xc, 0, 0xf
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000FFFFFu64;
	// 820627D0: 554A0314  rlwinm r10, r10, 0, 0xc, 0xa
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 820627D4: 554A0104  rlwinm r10, r10, 0, 4, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 820627D8: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 820627DC: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 820627E0: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 820627E4: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 820627E8: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 820627EC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820627F0: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 820627F4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820627F8: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 820627FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062800: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82062804: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062808: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8206280C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062818 size=12
    let mut pc: u32 = 0x82062818;
    'dispatch: loop {
        match pc {
            0x82062818 => {
    //   block [0x82062818..0x82062824)
	// 82062818: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206281C: 5563C6FE  rlwinm r3, r11, 0x18, 0x1b, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82062820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062828 size=24
    let mut pc: u32 = 0x82062828;
    'dispatch: loop {
        match pc {
            0x82062828 => {
    //   block [0x82062828..0x82062840)
	// 82062828: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206282C: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82062830: 508AAA14  rlwimi r10, r4, 0x15, 8, 0xa
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(21) as u64) & 0x0000000000E00000) | (ctx.r[10].u64 & 0xFFFFFFFFFF1FFFFF);
	// 82062834: 55690001  rlwinm. r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82062838: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 8206283C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062840 size=8
    let mut pc: u32 = 0x82062840;
    'dispatch: loop {
        match pc {
            0x82062840 => {
    //   block [0x82062840..0x82062848)
	// 82062840: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82062844: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062848 size=60
    let mut pc: u32 = 0x82062848;
    'dispatch: loop {
        match pc {
            0x82062848 => {
    //   block [0x82062848..0x82062884)
	// 82062848: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8206284C: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 82062850: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 82062854: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 82062858: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 8206285C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062860: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 82062864: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062868: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 8206286C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062870: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82062874: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062878: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8206287C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062888 size=12
    let mut pc: u32 = 0x82062888;
    'dispatch: loop {
        match pc {
            0x82062888 => {
    //   block [0x82062888..0x82062894)
	// 82062888: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206288C: 55635F7E  rlwinm r3, r11, 0xb, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 82062890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062898 size=24
    let mut pc: u32 = 0x82062898;
    'dispatch: loop {
        match pc {
            0x82062898 => {
    //   block [0x82062898..0x820628B0)
	// 82062898: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206289C: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 820628A0: 508A82DE  rlwimi r10, r4, 0x10, 0xb, 0xf
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000001F0000) | (ctx.r[10].u64 & 0xFFFFFFFFFFE0FFFF);
	// 820628A4: 55690001  rlwinm. r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 820628A8: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 820628AC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820628B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820628B0 size=8
    let mut pc: u32 = 0x820628B0;
    'dispatch: loop {
        match pc {
            0x820628B0 => {
    //   block [0x820628B0..0x820628B8)
	// 820628B0: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 820628B4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820628B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820628B8 size=60
    let mut pc: u32 = 0x820628B8;
    'dispatch: loop {
        match pc {
            0x820628B8 => {
    //   block [0x820628B8..0x820628F4)
	// 820628B8: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 820628BC: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 820628C0: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 820628C4: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 820628C8: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 820628CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820628D0: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 820628D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820628D8: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 820628DC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820628E0: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 820628E4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820628E8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 820628EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820628F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820628F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820628F8 size=12
    let mut pc: u32 = 0x820628F8;
    'dispatch: loop {
        match pc {
            0x820628F8 => {
    //   block [0x820628F8..0x82062904)
	// 820628F8: A1632E48  lhz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 820628FC: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82062900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062908 size=24
    let mut pc: u32 = 0x82062908;
    'dispatch: loop {
        match pc {
            0x82062908 => {
    //   block [0x82062908..0x82062920)
	// 82062908: 81432E48  lwz r10, 0x2e48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206290C: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82062910: 508AC0CE  rlwimi r10, r4, 0x18, 3, 7
	ctx.r[10].u64 = (((ctx.r[4].u32).rotate_left(24) as u64) & 0x000000001F000000) | (ctx.r[10].u64 & 0xFFFFFFFFE0FFFFFF);
	// 82062914: 55690001  rlwinm. r9, r11, 0, 0, 0
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82062918: 91432E48  stw r10, 0x2e48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11848 as u32), ctx.r[10].u32 ) };
	// 8206291C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062920 size=8
    let mut pc: u32 = 0x82062920;
    'dispatch: loop {
        match pc {
            0x82062920 => {
    //   block [0x82062920..0x82062928)
	// 82062920: 556B0043  rlwinm. r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82062924: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062928 size=60
    let mut pc: u32 = 0x82062928;
    'dispatch: loop {
        match pc {
            0x82062928 => {
    //   block [0x82062928..0x82062964)
	// 82062928: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8206292C: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 82062930: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 82062934: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 82062938: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 8206293C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062940: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 82062944: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062948: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 8206294C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062950: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82062954: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062958: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 8206295C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062968 size=12
    let mut pc: u32 = 0x82062968;
    'dispatch: loop {
        match pc {
            0x82062968 => {
    //   block [0x82062968..0x82062974)
	// 82062968: 89632E48  lbz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 8206296C: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82062970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062978 size=132
    let mut pc: u32 = 0x82062978;
    'dispatch: loop {
        match pc {
            0x82062978 => {
    //   block [0x82062978..0x820629FC)
	// 82062978: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 8206297C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82062980: 508BF042  rlwimi r11, r4, 0x1e, 1, 1
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(30) as u64) & 0x0000000040000000) | (ctx.r[11].u64 & 0xFFFFFFFFBFFFFFFF);
	// 82062984: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82062988: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8206298C: 81632E48  lwz r11, 0x2e48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11848 as u32) ) } as u64;
	// 82062990: 409A0024  bne cr6, 0x820629b4
	if !ctx.cr[6].eq {
	pc = 0x820629B4; continue 'dispatch;
	}
	// 82062994: 71691010  andi. r9, r11, 0x1010
	ctx.r[9].u64 = ctx.r[11].u64 & 4112;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82062998: 55682336  rlwinm r8, r11, 4, 0xc, 0x1b
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0FFFFFFFu64;
	// 8206299C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 820629A0: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 820629A4: 5529601E  rlwinm r9, r9, 0xc, 0, 0xf
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000FFFFFu64;
	// 820629A8: 55290314  rlwinm r9, r9, 0, 0xc, 0xa
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 820629AC: 55290104  rlwinm r9, r9, 0, 4, 2
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 820629B0: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 820629B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820629B8: 409A000C  bne cr6, 0x820629c4
	if !ctx.cr[6].eq {
	pc = 0x820629C4; continue 'dispatch;
	}
	// 820629BC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 820629C0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 820629C4: 91632938  stw r11, 0x2938(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10552 as u32), ctx.r[11].u32 ) };
	// 820629C8: 91632958  stw r11, 0x2958(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10584 as u32), ctx.r[11].u32 ) };
	// 820629CC: 9163295C  stw r11, 0x295c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10588 as u32), ctx.r[11].u32 ) };
	// 820629D0: 91632960  stw r11, 0x2960(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10592 as u32), ctx.r[11].u32 ) };
	// 820629D4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820629D8: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 820629DC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820629E0: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 820629E4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820629E8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 820629EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820629F0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 820629F4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820629F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062A00 size=12
    let mut pc: u32 = 0x82062A00;
    'dispatch: loop {
        match pc {
            0x82062A00 => {
    //   block [0x82062A00..0x82062A0C)
	// 82062A00: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82062A04: 556317FE  rlwinm r3, r11, 2, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82062A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062A10 size=52
    let mut pc: u32 = 0x82062A10;
    'dispatch: loop {
        match pc {
            0x82062A10 => {
    //   block [0x82062A10..0x82062A44)
	// 82062A10: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82062A14: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82062A18: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82062A1C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82062A20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82062A24: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82062A28: C00B0B18  lfs f0, 0xb18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2840 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82062A2C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062A30: D0032904  stfs f0, 0x2904(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10500 as u32), tmp.u32 ) };
	// 82062A34: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062A38: 656B0800  oris r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 134217728;
	// 82062A3C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062A48 size=40
    let mut pc: u32 = 0x82062A48;
    'dispatch: loop {
        match pc {
            0x82062A48 => {
    //   block [0x82062A48..0x82062A70)
	// 82062A48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82062A4C: C1832904  lfs f12, 0x2904(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10500 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82062A50: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82062A54: C00B0B1C  lfs f0, 0xb1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2844 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82062A58: C1AA06FC  lfs f13, 0x6fc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1788 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82062A5C: EC0C683A  fmadds f0, f12, f0, f13
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82062A60: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82062A64: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82062A68: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82062A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062A70 size=28
    let mut pc: u32 = 0x82062A70;
    'dispatch: loop {
        match pc {
            0x82062A70 => {
    //   block [0x82062A70..0x82062A8C)
	// 82062A70: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82062A74: 51640038  rlwimi r4, r11, 0, 0, 0x1c
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF8) | (ctx.r[4].u64 & 0xFFFFFFFF00000007);
	// 82062A78: 9083293C  stw r4, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[4].u32 ) };
	// 82062A7C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062A80: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82062A84: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062A90 size=12
    let mut pc: u32 = 0x82062A90;
    'dispatch: loop {
        match pc {
            0x82062A90 => {
    //   block [0x82062A90..0x82062A9C)
	// 82062A90: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82062A94: 5563077E  clrlwi r3, r11, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82062A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062AA0 size=144
    let mut pc: u32 = 0x82062AA0;
    'dispatch: loop {
        match pc {
            0x82062AA0 => {
    //   block [0x82062AA0..0x82062B30)
	// 82062AA0: 548B463E  srwi r11, r4, 0x18
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(24);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82062AA4: 5489C63E  rlwinm r9, r4, 0x18, 0x18, 0x1f
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82062AA8: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82062AAC: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82062AB0: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 82062AB4: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82062AB8: 548A863E  rlwinm r10, r4, 0x10, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82062ABC: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82062AC0: F941FFF8  std r10, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[10].u64 ) };
	// 82062AC4: C981FFF8  lfd f12, -8(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82062AC8: F961FFF8  std r11, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[11].u64 ) };
	// 82062ACC: C961FFF8  lfd f11, -8(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82062AD0: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 82062AD4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82062AD8: FD40069C  fcfid f10, f0
	ctx.f[10].f64 = (ctx.f[0].s64 as f64);
	// 82062ADC: C00A0B18  lfs f0, 0xb18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2840 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82062AE0: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82062AE4: 3980000F  li r12, 0xf
	ctx.r[12].s64 = 15;
	// 82062AE8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82062AEC: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 82062AF0: 798C0F86  sldi r12, r12, 0x21
	ctx.r[12].u64 = ctx.r[12].u64.wrapping_shl(33);
	ctx.r[12].u32 = ctx.r[12].u64 as u32;
	// 82062AF4: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82062AF8: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82062AFC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82062B00: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062B04: D16328E8  stfs f11, 0x28e8(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10472 as u32), tmp.u32 ) };
	// 82062B08: ED6A0032  fmuls f11, f10, f0
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062B0C: D16328E4  stfs f11, 0x28e4(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10468 as u32), tmp.u32 ) };
	// 82062B10: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062B14: D18328E0  stfs f12, 0x28e0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10464 as u32), tmp.u32 ) };
	// 82062B18: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062B1C: D00328EC  stfs f0, 0x28ec(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10476 as u32), tmp.u32 ) };
	// 82062B20: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062B24: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82062B28: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062B30 size=112
    let mut pc: u32 = 0x82062B30;
    'dispatch: loop {
        match pc {
            0x82062B30 => {
    //   block [0x82062B30..0x82062BA0)
	// 82062B30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82062B34: C18328E4  lfs f12, 0x28e4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10468 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82062B38: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82062B3C: C16328E8  lfs f11, 0x28e8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10472 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82062B40: C14328E0  lfs f10, 0x28e0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10464 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82062B44: C12328EC  lfs f9, 0x28ec(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10476 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82062B48: C00B0B1C  lfs f0, 0xb1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2844 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82062B4C: C1AA06FC  lfs f13, 0x6fc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1788 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82062B50: ED8C683A  fmadds f12, f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82062B54: ED6B683A  fmadds f11, f11, f0, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82062B58: ED4A683A  fmadds f10, f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82062B5C: EC09683A  fmadds f0, f9, f0, f13
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82062B60: FDA0665E  fctidz f13, f12
	ctx.f[13].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 82062B64: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 82062B68: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82062B6C: FDA05E5E  fctidz f13, f11
	ctx.f[13].s64 = if ctx.f[11].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[11].f64.trunc() as i64 };
	// 82062B70: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 82062B74: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82062B78: FDA0565E  fctidz f13, f10
	ctx.f[13].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 82062B7C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 82062B80: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82062B84: D801FFF8  stfd f0, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[0].u64 ) };
	// 82062B88: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82062B8C: 8121FFFC  lwz r9, -4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82062B90: 512A402E  rlwimi r10, r9, 8, 0, 0x17
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[10].u64 & 0xFFFFFFFF000000FF);
	// 82062B94: 514B402E  rlwimi r11, r10, 8, 0, 0x17
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[11].u64 & 0xFFFFFFFF000000FF);
	// 82062B98: 5163402E  rlwimi r3, r11, 8, 0, 0x17
	ctx.r[3].u64 = (((ctx.r[11].u32).rotate_left(8) as u64) & 0x00000000FFFFFF00) | (ctx.r[3].u64 & 0xFFFFFFFF000000FF);
	// 82062B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062BA0 size=36
    let mut pc: u32 = 0x82062BA0;
    'dispatch: loop {
        match pc {
            0x82062BA0 => {
    //   block [0x82062BA0..0x82062BC4)
	// 82062BA0: 816329B8  lwz r11, 0x29b8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10680 as u32) ) } as u64;
	// 82062BA4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82062BA8: 508B556A  rlwimi r11, r4, 0xa, 0x15, 0x15
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(10) as u64) & 0x0000000000000400) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFBFF);
	// 82062BAC: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 82062BB0: 916329B8  stw r11, 0x29b8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10680 as u32), ctx.r[11].u32 ) };
	// 82062BB4: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82062BB8: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82062BBC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82062BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062BC8 size=12
    let mut pc: u32 = 0x82062BC8;
    'dispatch: loop {
        match pc {
            0x82062BC8 => {
    //   block [0x82062BC8..0x82062BD4)
	// 82062BC8: 816329B8  lwz r11, 0x29b8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10680 as u32) ) } as u64;
	// 82062BCC: 5563B7FE  rlwinm r3, r11, 0x16, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82062BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062BD8 size=60
    let mut pc: u32 = 0x82062BD8;
    'dispatch: loop {
        match pc {
            0x82062BD8 => {
    //   block [0x82062BD8..0x82062C14)
	// 82062BD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82062BDC: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82062BE0: C1A1001C  lfs f13, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82062BE4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82062BE8: 798CA7E6  rldicr r12, r12, 0x34, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(52) & 0xFFFFFFFFFFFFFFFF;
	// 82062BEC: C00B0B14  lfs f0, 0xb14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2836 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82062BF0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062BF4: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82062BF8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82062BFC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82062C00: B163296E  sth r11, 0x296e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10606 as u32), ctx.r[11].u16 ) };
	// 82062C04: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82062C08: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82062C0C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82062C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062C18 size=48
    let mut pc: u32 = 0x82062C18;
    'dispatch: loop {
        match pc {
            0x82062C18 => {
    //   block [0x82062C18..0x82062C48)
	// 82062C18: A163296E  lhz r11, 0x296e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10606 as u32) ) } as u64;
	// 82062C1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82062C20: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82062C24: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82062C28: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82062C2C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82062C30: C00A0A10  lfs f0, 0xa10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2576 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82062C34: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82062C38: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82062C3C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82062C40: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82062C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062C48 size=20
    let mut pc: u32 = 0x82062C48;
    'dispatch: loop {
        match pc {
            0x82062C48 => {
    //   block [0x82062C48..0x82062C5C)
	// 82062C48: 81635DAC  lwz r11, 0x5dac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(23980 as u32) ) } as u64;
	// 82062C4C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82062C50: 419A000C  beq cr6, 0x82062c5c
	if ctx.cr[6].eq {
		sub_82062C5C(ctx, base);
		return;
	}
	// 82062C54: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82062C58: 48000008  b 0x82062c60
	sub_82062C5C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062C5C size=12
    let mut pc: u32 = 0x82062C5C;
    'dispatch: loop {
        match pc {
            0x82062C5C => {
    //   block [0x82062C5C..0x82062C68)
	// 82062C5C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82062C60: 91635DAC  stw r11, 0x5dac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(23980 as u32), ctx.r[11].u32 ) };
	// 82062C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062C68 size=12
    let mut pc: u32 = 0x82062C68;
    'dispatch: loop {
        match pc {
            0x82062C68 => {
    //   block [0x82062C68..0x82062C74)
	// 82062C68: 81635DAC  lwz r11, 0x5dac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(23980 as u32) ) } as u64;
	// 82062C6C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82062C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062C78 size=56
    let mut pc: u32 = 0x82062C78;
    'dispatch: loop {
        match pc {
            0x82062C78 => {
    //   block [0x82062C78..0x82062CB0)
	// 82062C78: 816330A8  lwz r11, 0x30a8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82062C7C: 90832E64  stw r4, 0x2e64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11876 as u32), ctx.r[4].u32 ) };
	// 82062C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82062C84: 409A0008  bne cr6, 0x82062c8c
	if !ctx.cr[6].eq {
	pc = 0x82062C8C; continue 'dispatch;
	}
	// 82062C88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82062C8C: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062C90: 508B0FBC  rlwimi r11, r4, 1, 0x1e, 0x1e
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(1) as u64) & 0x0000000000000002) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFD);
	// 82062C94: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062C98: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062C9C: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062CA0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062CA4: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062CA8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062CB0 size=8
    let mut pc: u32 = 0x82062CB0;
    'dispatch: loop {
        match pc {
            0x82062CB0 => {
    //   block [0x82062CB0..0x82062CB8)
	// 82062CB0: 80632E64  lwz r3, 0x2e64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11876 as u32) ) } as u64;
	// 82062CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062CB8 size=28
    let mut pc: u32 = 0x82062CB8;
    'dispatch: loop {
        match pc {
            0x82062CB8 => {
    //   block [0x82062CB8..0x82062CD4)
	// 82062CB8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062CBC: 508B177A  rlwimi r11, r4, 2, 0x1d, 0x1d
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(2) as u64) & 0x0000000000000004) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82062CC0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062CC4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062CC8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062CCC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062CD8 size=12
    let mut pc: u32 = 0x82062CD8;
    'dispatch: loop {
        match pc {
            0x82062CD8 => {
    //   block [0x82062CD8..0x82062CE4)
	// 82062CD8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062CDC: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82062CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062CE8 size=36
    let mut pc: u32 = 0x82062CE8;
    'dispatch: loop {
        match pc {
            0x82062CE8 => {
    //   block [0x82062CE8..0x82062D0C)
	// 82062CE8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062CEC: 508B2676  rlwimi r11, r4, 4, 0x19, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x0000000000000070) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF8F);
	// 82062CF0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062CF4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062CF8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062CFC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062D00: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062D04: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062D10 size=12
    let mut pc: u32 = 0x82062D10;
    'dispatch: loop {
        match pc {
            0x82062D10 => {
    //   block [0x82062D10..0x82062D1C)
	// 82062D10: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062D14: 5563E77E  rlwinm r3, r11, 0x1c, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82062D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062D20 size=56
    let mut pc: u32 = 0x82062D20;
    'dispatch: loop {
        match pc {
            0x82062D20 => {
    //   block [0x82062D20..0x82062D58)
	// 82062D20: 816330A8  lwz r11, 0x30a8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12456 as u32) ) } as u64;
	// 82062D24: 90832E68  stw r4, 0x2e68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11880 as u32), ctx.r[4].u32 ) };
	// 82062D28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82062D2C: 409A0008  bne cr6, 0x82062d34
	if !ctx.cr[6].eq {
	pc = 0x82062D34; continue 'dispatch;
	}
	// 82062D30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82062D34: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062D38: 508B07FE  rlwimi r11, r4, 0, 0x1f, 0x1f
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x0000000000000001) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFE);
	// 82062D3C: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062D40: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062D44: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062D48: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062D4C: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062D50: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062D58 size=8
    let mut pc: u32 = 0x82062D58;
    'dispatch: loop {
        match pc {
            0x82062D58 => {
    //   block [0x82062D58..0x82062D60)
	// 82062D58: 80632E68  lwz r3, 0x2e68(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11880 as u32) ) } as u64;
	// 82062D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062D60 size=36
    let mut pc: u32 = 0x82062D60;
    'dispatch: loop {
        match pc {
            0x82062D60 => {
    //   block [0x82062D60..0x82062D84)
	// 82062D60: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062D64: 508B3E30  rlwimi r11, r4, 7, 0x18, 0x18
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(7) as u64) & 0x0000000000000080) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF7F);
	// 82062D68: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062D6C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062D70: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062D74: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062D78: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062D7C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062D88 size=12
    let mut pc: u32 = 0x82062D88;
    'dispatch: loop {
        match pc {
            0x82062D88 => {
    //   block [0x82062D88..0x82062D94)
	// 82062D88: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062D8C: 5563CFFE  rlwinm r3, r11, 0x19, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 82062D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062D98 size=28
    let mut pc: u32 = 0x82062D98;
    'dispatch: loop {
        match pc {
            0x82062D98 => {
    //   block [0x82062D98..0x82062DB4)
	// 82062D98: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062D9C: 508B456E  rlwimi r11, r4, 8, 0x15, 0x17
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000000700) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF8FF);
	// 82062DA0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062DA4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062DA8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062DAC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062DB8 size=12
    let mut pc: u32 = 0x82062DB8;
    'dispatch: loop {
        match pc {
            0x82062DB8 => {
    //   block [0x82062DB8..0x82062DC4)
	// 82062DB8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062DBC: 5563C77E  rlwinm r3, r11, 0x18, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82062DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062DC8 size=36
    let mut pc: u32 = 0x82062DC8;
    'dispatch: loop {
        match pc {
            0x82062DC8 => {
    //   block [0x82062DC8..0x82062DEC)
	// 82062DC8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062DCC: 508B5CA8  rlwimi r11, r4, 0xb, 0x12, 0x14
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(11) as u64) & 0x0000000000003800) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFC7FF);
	// 82062DD0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062DD4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062DD8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062DDC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062DE0: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062DE4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062DF0 size=12
    let mut pc: u32 = 0x82062DF0;
    'dispatch: loop {
        match pc {
            0x82062DF0 => {
    //   block [0x82062DF0..0x82062DFC)
	// 82062DF0: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062DF4: 5563AF7E  rlwinm r3, r11, 0x15, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 82062DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E00 size=36
    let mut pc: u32 = 0x82062E00;
    'dispatch: loop {
        match pc {
            0x82062E00 => {
    //   block [0x82062E00..0x82062E24)
	// 82062E00: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E04: 508B8B1C  rlwimi r11, r4, 0x11, 0xc, 0xe
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(17) as u64) & 0x00000000000E0000) | (ctx.r[11].u64 & 0xFFFFFFFFFFF1FFFF);
	// 82062E08: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062E0C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062E10: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062E14: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062E18: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062E1C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E28 size=12
    let mut pc: u32 = 0x82062E28;
    'dispatch: loop {
        match pc {
            0x82062E28 => {
    //   block [0x82062E28..0x82062E34)
	// 82062E28: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E2C: 55637F7E  rlwinm r3, r11, 0xf, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 82062E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E38 size=28
    let mut pc: u32 = 0x82062E38;
    'dispatch: loop {
        match pc {
            0x82062E38 => {
    //   block [0x82062E38..0x82062E54)
	// 82062E38: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E3C: 508B73E2  rlwimi r11, r4, 0xe, 0xf, 0x11
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(14) as u64) & 0x000000000001C000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFE3FFF);
	// 82062E40: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062E44: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062E48: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062E4C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E58 size=12
    let mut pc: u32 = 0x82062E58;
    'dispatch: loop {
        match pc {
            0x82062E58 => {
    //   block [0x82062E58..0x82062E64)
	// 82062E58: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E5C: 5563977E  rlwinm r3, r11, 0x12, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82062E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E68 size=28
    let mut pc: u32 = 0x82062E68;
    'dispatch: loop {
        match pc {
            0x82062E68 => {
    //   block [0x82062E68..0x82062E84)
	// 82062E68: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E6C: 508BA256  rlwimi r11, r4, 0x14, 9, 0xb
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(20) as u64) & 0x0000000000700000) | (ctx.r[11].u64 & 0xFFFFFFFFFF8FFFFF);
	// 82062E70: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062E74: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062E78: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062E7C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E88 size=12
    let mut pc: u32 = 0x82062E88;
    'dispatch: loop {
        match pc {
            0x82062E88 => {
    //   block [0x82062E88..0x82062E94)
	// 82062E88: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E8C: 5563677E  rlwinm r3, r11, 0xc, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82062E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062E98 size=36
    let mut pc: u32 = 0x82062E98;
    'dispatch: loop {
        match pc {
            0x82062E98 => {
    //   block [0x82062E98..0x82062EBC)
	// 82062E98: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062E9C: 508BB990  rlwimi r11, r4, 0x17, 6, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x0000000003800000) | (ctx.r[11].u64 & 0xFFFFFFFFFC7FFFFF);
	// 82062EA0: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062EA4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062EA8: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062EAC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062EB0: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062EB4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062EC0 size=12
    let mut pc: u32 = 0x82062EC0;
    'dispatch: loop {
        match pc {
            0x82062EC0 => {
    //   block [0x82062EC0..0x82062ECC)
	// 82062EC0: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062EC4: 55634F7E  rlwinm r3, r11, 9, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82062EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062ED0 size=36
    let mut pc: u32 = 0x82062ED0;
    'dispatch: loop {
        match pc {
            0x82062ED0 => {
    //   block [0x82062ED0..0x82062EF4)
	// 82062ED0: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062ED4: 508BE804  rlwimi r11, r4, 0x1d, 0, 2
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(29) as u64) & 0x00000000E0000000) | (ctx.r[11].u64 & 0xFFFFFFFF1FFFFFFF);
	// 82062ED8: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062EDC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062EE0: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062EE4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062EE8: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82062EEC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062EF8 size=12
    let mut pc: u32 = 0x82062EF8;
    'dispatch: loop {
        match pc {
            0x82062EF8 => {
    //   block [0x82062EF8..0x82062F04)
	// 82062EF8: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062EFC: 55631F7E  srwi r3, r11, 0x1d
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(29);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82062F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F08 size=28
    let mut pc: u32 = 0x82062F08;
    'dispatch: loop {
        match pc {
            0x82062F08 => {
    //   block [0x82062F08..0x82062F24)
	// 82062F08: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062F0C: 508BD0CA  rlwimi r11, r4, 0x1a, 3, 5
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(26) as u64) & 0x000000001C000000) | (ctx.r[11].u64 & 0xFFFFFFFFE3FFFFFF);
	// 82062F10: 91632934  stw r11, 0x2934(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10548 as u32), ctx.r[11].u32 ) };
	// 82062F14: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062F18: 616B0800  ori r11, r11, 0x800
	ctx.r[11].u64 = ctx.r[11].u64 | 2048;
	// 82062F1C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F28 size=12
    let mut pc: u32 = 0x82062F28;
    'dispatch: loop {
        match pc {
            0x82062F28 => {
    //   block [0x82062F28..0x82062F34)
	// 82062F28: 81632934  lwz r11, 0x2934(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10548 as u32) ) } as u64;
	// 82062F2C: 5563377E  rlwinm r3, r11, 6, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x03FFFFFFu64;
	// 82062F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F38 size=20
    let mut pc: u32 = 0x82062F38;
    'dispatch: loop {
        match pc {
            0x82062F38 => {
    //   block [0x82062F38..0x82062F4C)
	// 82062F38: 98832903  stb r4, 0x2903(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10499 as u32), ctx.r[4].u8 ) };
	// 82062F3C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062F40: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 82062F44: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F50 size=8
    let mut pc: u32 = 0x82062F50;
    'dispatch: loop {
        match pc {
            0x82062F50 => {
    //   block [0x82062F50..0x82062F58)
	// 82062F50: 88632903  lbz r3, 0x2903(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10499 as u32) ) } as u64;
	// 82062F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F58 size=20
    let mut pc: u32 = 0x82062F58;
    'dispatch: loop {
        match pc {
            0x82062F58 => {
    //   block [0x82062F58..0x82062F6C)
	// 82062F58: 98832902  stb r4, 0x2902(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10498 as u32), ctx.r[4].u8 ) };
	// 82062F5C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062F60: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 82062F64: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F70 size=8
    let mut pc: u32 = 0x82062F70;
    'dispatch: loop {
        match pc {
            0x82062F70 => {
    //   block [0x82062F70..0x82062F78)
	// 82062F70: 88632902  lbz r3, 0x2902(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10498 as u32) ) } as u64;
	// 82062F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F78 size=20
    let mut pc: u32 = 0x82062F78;
    'dispatch: loop {
        match pc {
            0x82062F78 => {
    //   block [0x82062F78..0x82062F8C)
	// 82062F78: 98832901  stb r4, 0x2901(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10497 as u32), ctx.r[4].u8 ) };
	// 82062F7C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062F80: 656B1000  oris r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 268435456;
	// 82062F84: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F90 size=8
    let mut pc: u32 = 0x82062F90;
    'dispatch: loop {
        match pc {
            0x82062F90 => {
    //   block [0x82062F90..0x82062F98)
	// 82062F90: 88632901  lbz r3, 0x2901(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10497 as u32) ) } as u64;
	// 82062F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062F98 size=20
    let mut pc: u32 = 0x82062F98;
    'dispatch: loop {
        match pc {
            0x82062F98 => {
    //   block [0x82062F98..0x82062FAC)
	// 82062F98: 988328FF  stb r4, 0x28ff(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10495 as u32), ctx.r[4].u8 ) };
	// 82062F9C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062FA0: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 82062FA4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062FB0 size=8
    let mut pc: u32 = 0x82062FB0;
    'dispatch: loop {
        match pc {
            0x82062FB0 => {
    //   block [0x82062FB0..0x82062FB8)
	// 82062FB0: 886328FF  lbz r3, 0x28ff(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10495 as u32) ) } as u64;
	// 82062FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062FB8 size=20
    let mut pc: u32 = 0x82062FB8;
    'dispatch: loop {
        match pc {
            0x82062FB8 => {
    //   block [0x82062FB8..0x82062FCC)
	// 82062FB8: 988328FE  stb r4, 0x28fe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10494 as u32), ctx.r[4].u8 ) };
	// 82062FBC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062FC0: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 82062FC4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062FD0 size=8
    let mut pc: u32 = 0x82062FD0;
    'dispatch: loop {
        match pc {
            0x82062FD0 => {
    //   block [0x82062FD0..0x82062FD8)
	// 82062FD0: 886328FE  lbz r3, 0x28fe(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10494 as u32) ) } as u64;
	// 82062FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062FD8 size=20
    let mut pc: u32 = 0x82062FD8;
    'dispatch: loop {
        match pc {
            0x82062FD8 => {
    //   block [0x82062FD8..0x82062FEC)
	// 82062FD8: 988328FD  stb r4, 0x28fd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10493 as u32), ctx.r[4].u8 ) };
	// 82062FDC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82062FE0: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 82062FE4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82062FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82062FF0 size=8
    let mut pc: u32 = 0x82062FF0;
    'dispatch: loop {
        match pc {
            0x82062FF0 => {
    //   block [0x82062FF0..0x82062FF8)
	// 82062FF0: 886328FD  lbz r3, 0x28fd(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10493 as u32) ) } as u64;
	// 82062FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82062FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82062FF8 size=64
    let mut pc: u32 = 0x82062FF8;
    'dispatch: loop {
        match pc {
            0x82062FF8 => {
    //   block [0x82062FF8..0x82063038)
	// 82062FF8: 81632944  lwz r11, 0x2944(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 82062FFC: 21440000  subfic r10, r4, 0
	ctx.xer.ca = ctx.r[4].u32 <= 0 as u32;
	ctx.r[10].s64 = (0 as i64) - ctx.r[4].s64;
	// 82063000: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063004: 556B0032  rlwinm r11, r11, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063008: 7D4A5110  subfe r10, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8206300C: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82063010: 554A04E6  rlwinm r10, r10, 0, 0x13, 0x13
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82063014: 91632944  stw r11, 0x2944(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10564 as u32), ctx.r[11].u32 ) };
	// 82063018: 798C67E6  rldicr r12, r12, 0x2c, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(44) & 0xFFFFFFFFFFFFFFFF;
	// 8206301C: 914328B4  stw r10, 0x28b4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10420 as u32), ctx.r[10].u32 ) };
	// 82063020: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063024: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 82063028: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8206302C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063030: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063038 size=12
    let mut pc: u32 = 0x82063038;
    'dispatch: loop {
        match pc {
            0x82063038 => {
    //   block [0x82063038..0x82063044)
	// 82063038: 81632944  lwz r11, 0x2944(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 8206303C: 556306BE  clrlwi r3, r11, 0x1a
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82063040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063048 size=8
    let mut pc: u32 = 0x82063048;
    'dispatch: loop {
        match pc {
            0x82063048 => {
    //   block [0x82063048..0x82063050)
	// 82063048: 80632E50  lwz r3, 0x2e50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11856 as u32) ) } as u64;
	// 8206304C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063050 size=164
    let mut pc: u32 = 0x82063050;
    'dispatch: loop {
        match pc {
            0x82063050 => {
    //   block [0x82063050..0x820630F4)
	// 82063050: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82063054: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063058: C1A1001C  lfs f13, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8206305C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82063060: C00B0B20  lfs f0, 0xb20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2848 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063064: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82063068: C00A06F8  lfs f0, 0x6f8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206306C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82063070: 409A0014  bne cr6, 0x82063084
	if !ctx.cr[6].eq {
	pc = 0x82063084; continue 'dispatch;
	}
	// 82063074: C1832A54  lfs f12, 0x2a54(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10836 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82063078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8206307C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82063080: 419A0008  beq cr6, 0x82063088
	if ctx.cr[6].eq {
	pc = 0x82063088; continue 'dispatch;
	}
	// 82063084: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82063088: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 8206308C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82063090: 514B5D28  rlwimi r11, r10, 0xb, 0x14, 0x14
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF7FF);
	// 82063094: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 82063098: 409A0014  bne cr6, 0x820630ac
	if !ctx.cr[6].eq {
	pc = 0x820630AC; continue 'dispatch;
	}
	// 8206309C: C1832A5C  lfs f12, 0x2a5c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10844 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 820630A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 820630A4: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 820630A8: 419A0008  beq cr6, 0x820630b0
	if ctx.cr[6].eq {
	pc = 0x820630B0; continue 'dispatch;
	}
	// 820630AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 820630B0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 820630B4: D1A32A50  stfs f13, 0x2a50(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10832 as u32), tmp.u32 ) };
	// 820630B8: 514B64E6  rlwimi r11, r10, 0xc, 0x13, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x0000000000001000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFEFFF);
	// 820630BC: D1A32A58  stfs f13, 0x2a58(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10840 as u32), tmp.u32 ) };
	// 820630C0: 798C6FE6  rldicr r12, r12, 0x2d, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(45) & 0xFFFFFFFFFFFFFFFF;
	// 820630C4: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 820630C8: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 820630CC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 820630D0: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 820630D4: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 820630D8: 798C5FE6  rldicr r12, r12, 0x2b, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(43) & 0xFFFFFFFFFFFFFFFF;
	// 820630DC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 820630E0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 820630E4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820630E8: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 820630EC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820630F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820630F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x820630F8 size=28
    let mut pc: u32 = 0x820630F8;
    'dispatch: loop {
        match pc {
            0x820630F8 => {
    //   block [0x820630F8..0x82063114)
	// 820630F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 820630FC: C1A32A50  lfs f13, 0x2a50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10832 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82063100: C00B0B24  lfs f0, 0xb24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063104: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82063108: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8206310C: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82063110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063118 size=152
    let mut pc: u32 = 0x82063118;
    'dispatch: loop {
        match pc {
            0x82063118 => {
    //   block [0x82063118..0x820631B0)
	// 82063118: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8206311C: C1832A50  lfs f12, 0x2a50(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10832 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82063120: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063124: C1A1001C  lfs f13, 0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82063128: C00B06F8  lfs f0, 0x6f8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206312C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82063130: 409A0010  bne cr6, 0x82063140
	if !ctx.cr[6].eq {
	pc = 0x82063140; continue 'dispatch;
	}
	// 82063134: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82063138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8206313C: 419A0008  beq cr6, 0x82063144
	if ctx.cr[6].eq {
	pc = 0x82063144; continue 'dispatch;
	}
	// 82063140: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82063144: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82063148: C1832A58  lfs f12, 0x2a58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10840 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8206314C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82063150: 514B5D28  rlwimi r11, r10, 0xb, 0x14, 0x14
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF7FF);
	// 82063154: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 82063158: 409A0010  bne cr6, 0x82063168
	if !ctx.cr[6].eq {
	pc = 0x82063168; continue 'dispatch;
	}
	// 8206315C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82063160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82063164: 419A0008  beq cr6, 0x8206316c
	if ctx.cr[6].eq {
	pc = 0x8206316C; continue 'dispatch;
	}
	// 82063168: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8206316C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063170: D1A32A54  stfs f13, 0x2a54(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10836 as u32), tmp.u32 ) };
	// 82063174: 514B64E6  rlwimi r11, r10, 0xc, 0x13, 0x13
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(12) as u64) & 0x0000000000001000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFEFFF);
	// 82063178: D1A32A5C  stfs f13, 0x2a5c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10844 as u32), tmp.u32 ) };
	// 8206317C: 798C67E6  rldicr r12, r12, 0x2c, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(44) & 0xFFFFFFFFFFFFFFFF;
	// 82063180: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 82063184: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82063188: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 8206318C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063190: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063194: 798C57E6  rldicr r12, r12, 0x2a, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(42) & 0xFFFFFFFFFFFFFFFF;
	// 82063198: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 8206319C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 820631A0: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820631A4: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 820631A8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820631AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820631B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x820631B0 size=16
    let mut pc: u32 = 0x820631B0;
    'dispatch: loop {
        match pc {
            0x820631B0 => {
    //   block [0x820631B0..0x820631C0)
	// 820631B0: C0032A54  lfs f0, 0x2a54(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10836 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 820631B4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 820631B8: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 820631BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820631C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820631C0 size=28
    let mut pc: u32 = 0x820631C0;
    'dispatch: loop {
        match pc {
            0x820631C0 => {
    //   block [0x820631C0..0x820631DC)
	// 820631C0: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 820631C4: 508B7C20  rlwimi r11, r4, 0xf, 0x10, 0x10
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(15) as u64) & 0x0000000000008000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF7FFF);
	// 820631C8: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 820631CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820631D0: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 820631D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820631D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820631E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820631E0 size=12
    let mut pc: u32 = 0x820631E0;
    'dispatch: loop {
        match pc {
            0x820631E0 => {
    //   block [0x820631E0..0x820631EC)
	// 820631E0: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 820631E4: 55638FFE  rlwinm r3, r11, 0x11, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 820631E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820631F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820631F0 size=24
    let mut pc: u32 = 0x820631F0;
    'dispatch: loop {
        match pc {
            0x820631F0 => {
    //   block [0x820631F0..0x82063208)
	// 820631F0: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 820631F4: 91632A00  stw r11, 0x2a00(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10752 as u32), ctx.r[11].u32 ) };
	// 820631F8: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 820631FC: 656B0008  oris r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 524288;
	// 82063200: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063208 size=8
    let mut pc: u32 = 0x82063208;
    'dispatch: loop {
        match pc {
            0x82063208 => {
    //   block [0x82063208..0x82063210)
	// 82063208: 80632A00  lwz r3, 0x2a00(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10752 as u32) ) } as u64;
	// 8206320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063210 size=56
    let mut pc: u32 = 0x82063210;
    'dispatch: loop {
        match pc {
            0x82063210 => {
    //   block [0x82063210..0x82063248)
	// 82063210: 81633098  lwz r11, 0x3098(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12440 as u32) ) } as u64;
	// 82063214: 90832E54  stw r4, 0x2e54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11860 as u32), ctx.r[4].u32 ) };
	// 82063218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8206321C: 409A0008  bne cr6, 0x82063224
	if !ctx.cr[6].eq {
	pc = 0x82063224; continue 'dispatch;
	}
	// 82063220: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82063224: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 82063228: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 8206322C: 51640036  rlwimi r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFF0) | (ctx.r[4].u64 & 0xFFFFFFFF0000000F);
	// 82063230: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 82063234: 908328DC  stw r4, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[4].u32 ) };
	// 82063238: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8206323C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063240: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063248 size=8
    let mut pc: u32 = 0x82063248;
    'dispatch: loop {
        match pc {
            0x82063248 => {
    //   block [0x82063248..0x82063250)
	// 82063248: 80632E54  lwz r3, 0x2e54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11860 as u32) ) } as u64;
	// 8206324C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063250 size=56
    let mut pc: u32 = 0x82063250;
    'dispatch: loop {
        match pc {
            0x82063250 => {
    //   block [0x82063250..0x82063288)
	// 82063250: 8163309C  lwz r11, 0x309c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82063254: 90832E58  stw r4, 0x2e58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11864 as u32), ctx.r[4].u32 ) };
	// 82063258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8206325C: 409A0008  bne cr6, 0x82063264
	if !ctx.cr[6].eq {
	pc = 0x82063264; continue 'dispatch;
	}
	// 82063260: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82063264: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 82063268: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 8206326C: 508B2636  rlwimi r11, r4, 4, 0x18, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x00000000000000F0) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF0F);
	// 82063270: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 82063274: 916328DC  stw r11, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[11].u32 ) };
	// 82063278: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8206327C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063280: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063288 size=8
    let mut pc: u32 = 0x82063288;
    'dispatch: loop {
        match pc {
            0x82063288 => {
    //   block [0x82063288..0x82063290)
	// 82063288: 80632E58  lwz r3, 0x2e58(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11864 as u32) ) } as u64;
	// 8206328C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063290 size=56
    let mut pc: u32 = 0x82063290;
    'dispatch: loop {
        match pc {
            0x82063290 => {
    //   block [0x82063290..0x820632C8)
	// 82063290: 816330A0  lwz r11, 0x30a0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12448 as u32) ) } as u64;
	// 82063294: 90832E5C  stw r4, 0x2e5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11868 as u32), ctx.r[4].u32 ) };
	// 82063298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8206329C: 409A0008  bne cr6, 0x820632a4
	if !ctx.cr[6].eq {
	pc = 0x820632A4; continue 'dispatch;
	}
	// 820632A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 820632A4: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 820632A8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 820632AC: 508B452E  rlwimi r11, r4, 8, 0x14, 0x17
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(8) as u64) & 0x0000000000000F00) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFF0FF);
	// 820632B0: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 820632B4: 916328DC  stw r11, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[11].u32 ) };
	// 820632B8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820632BC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 820632C0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820632C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820632C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820632C8 size=8
    let mut pc: u32 = 0x820632C8;
    'dispatch: loop {
        match pc {
            0x820632C8 => {
    //   block [0x820632C8..0x820632D0)
	// 820632C8: 80632E5C  lwz r3, 0x2e5c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11868 as u32) ) } as u64;
	// 820632CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820632D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820632D0 size=56
    let mut pc: u32 = 0x820632D0;
    'dispatch: loop {
        match pc {
            0x820632D0 => {
    //   block [0x820632D0..0x82063308)
	// 820632D0: 816330A4  lwz r11, 0x30a4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12452 as u32) ) } as u64;
	// 820632D4: 90832E60  stw r4, 0x2e60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11872 as u32), ctx.r[4].u32 ) };
	// 820632D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 820632DC: 409A0008  bne cr6, 0x820632e4
	if !ctx.cr[6].eq {
	pc = 0x820632E4; continue 'dispatch;
	}
	// 820632E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 820632E4: 816328DC  lwz r11, 0x28dc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10460 as u32) ) } as u64;
	// 820632E8: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 820632EC: 508B6426  rlwimi r11, r4, 0xc, 0x10, 0x13
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(12) as u64) & 0x000000000000F000) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0FFF);
	// 820632F0: 798C2FE6  rldicr r12, r12, 0x25, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(37) & 0xFFFFFFFFFFFFFFFF;
	// 820632F4: 916328DC  stw r11, 0x28dc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10460 as u32), ctx.r[11].u32 ) };
	// 820632F8: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820632FC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063300: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063308 size=8
    let mut pc: u32 = 0x82063308;
    'dispatch: loop {
        match pc {
            0x82063308 => {
    //   block [0x82063308..0x82063310)
	// 82063308: 80632E60  lwz r3, 0x2e60(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11872 as u32) ) } as u64;
	// 8206330C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063310 size=8
    let mut pc: u32 = 0x82063310;
    'dispatch: loop {
        match pc {
            0x82063310 => {
    //   block [0x82063310..0x82063318)
	// 82063310: 90832E6C  stw r4, 0x2e6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11884 as u32), ctx.r[4].u32 ) };
	// 82063314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063318 size=8
    let mut pc: u32 = 0x82063318;
    'dispatch: loop {
        match pc {
            0x82063318 => {
    //   block [0x82063318..0x82063320)
	// 82063318: 80632E6C  lwz r3, 0x2e6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11884 as u32) ) } as u64;
	// 8206331C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063320 size=72
    let mut pc: u32 = 0x82063320;
    'dispatch: loop {
        match pc {
            0x82063320 => {
    //   block [0x82063320..0x82063368)
	// 82063320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82063324: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063328: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206332C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063330: D0032E74  stfs f0, 0x2e74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11892 as u32), tmp.u32 ) };
	// 82063334: C1AB0B14  lfs f13, 0xb14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2836 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82063338: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 8206333C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82063340: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82063344: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82063348: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8206334C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82063350: B1632966  sth r11, 0x2966(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10598 as u32), ctx.r[11].u16 ) };
	// 82063354: B1632964  sth r11, 0x2964(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10596 as u32), ctx.r[11].u16 ) };
	// 82063358: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 8206335C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063360: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063368 size=16
    let mut pc: u32 = 0x82063368;
    'dispatch: loop {
        match pc {
            0x82063368 => {
    //   block [0x82063368..0x82063378)
	// 82063368: C0032E74  lfs f0, 0x2e74(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11892 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206336C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82063370: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82063374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063378 size=64
    let mut pc: u32 = 0x82063378;
    'dispatch: loop {
        match pc {
            0x82063378 => {
    //   block [0x82063378..0x820633B8)
	// 82063378: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8206337C: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063380: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063384: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063388: D0032E78  stfs f0, 0x2e78(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11896 as u32), tmp.u32 ) };
	// 8206338C: C1AB0B20  lfs f13, 0xb20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2848 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82063390: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 82063394: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82063398: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8206339C: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 820633A0: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 820633A4: B163296A  sth r11, 0x296a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10602 as u32), ctx.r[11].u16 ) };
	// 820633A8: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 820633AC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 820633B0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 820633B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820633B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x820633B8 size=16
    let mut pc: u32 = 0x820633B8;
    'dispatch: loop {
        match pc {
            0x820633B8 => {
    //   block [0x820633B8..0x820633C8)
	// 820633B8: C0032E78  lfs f0, 0x2e78(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11896 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 820633BC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 820633C0: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 820633C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820633C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x820633C8 size=64
    let mut pc: u32 = 0x820633C8;
    'dispatch: loop {
        match pc {
            0x820633C8 => {
    //   block [0x820633C8..0x82063408)
	// 820633C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 820633CC: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 820633D0: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 820633D4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 820633D8: D0032E7C  stfs f0, 0x2e7c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11900 as u32), tmp.u32 ) };
	// 820633DC: C1AB0B20  lfs f13, 0xb20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2848 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 820633E0: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 820633E4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 820633E8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 820633EC: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 820633F0: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 820633F4: B1632968  sth r11, 0x2968(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10600 as u32), ctx.r[11].u16 ) };
	// 820633F8: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 820633FC: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063400: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063408 size=16
    let mut pc: u32 = 0x82063408;
    'dispatch: loop {
        match pc {
            0x82063408 => {
    //   block [0x82063408..0x82063418)
	// 82063408: C0032E7C  lfs f0, 0x2e7c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11900 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206340C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82063410: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82063414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063418 size=32
    let mut pc: u32 = 0x82063418;
    'dispatch: loop {
        match pc {
            0x82063418 => {
    //   block [0x82063418..0x82063438)
	// 82063418: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206341C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063420: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82063424: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82063428: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8206342C: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82063430: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063438 size=36
    let mut pc: u32 = 0x82063438;
    'dispatch: loop {
        match pc {
            0x82063438 => {
    //   block [0x82063438..0x8206345C)
	// 82063438: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206343C: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063440: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063444: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063448: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 8206344C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063450: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 82063454: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063460 size=36
    let mut pc: u32 = 0x82063460;
    'dispatch: loop {
        match pc {
            0x82063460 => {
    //   block [0x82063460..0x82063484)
	// 82063460: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82063464: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063468: 556B0626  rlwinm r11, r11, 0, 0x18, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8206346C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063470: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82063474: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063478: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 8206347C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063488 size=36
    let mut pc: u32 = 0x82063488;
    'dispatch: loop {
        match pc {
            0x82063488 => {
    //   block [0x82063488..0x820634AC)
	// 82063488: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206348C: 548A6026  slwi r10, r4, 0xc
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(12);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063490: 556B051E  rlwinm r11, r11, 0, 0x14, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063494: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063498: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 8206349C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820634A0: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 820634A4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820634A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820634B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820634B0 size=36
    let mut pc: u32 = 0x820634B0;
    'dispatch: loop {
        match pc {
            0x820634B0 => {
    //   block [0x820634B0..0x820634D4)
	// 820634B0: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820634B4: 548A801E  slwi r10, r4, 0x10
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 820634B8: 556B0416  rlwinm r11, r11, 0, 0x10, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 820634BC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820634C0: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 820634C4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820634C8: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 820634CC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820634D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820634D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820634D8 size=36
    let mut pc: u32 = 0x820634D8;
    'dispatch: loop {
        match pc {
            0x820634D8 => {
    //   block [0x820634D8..0x820634FC)
	// 820634D8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820634DC: 548AA016  slwi r10, r4, 0x14
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 820634E0: 556B030E  rlwinm r11, r11, 0, 0xc, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 820634E4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820634E8: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 820634EC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820634F0: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 820634F4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820634F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063500 size=36
    let mut pc: u32 = 0x82063500;
    'dispatch: loop {
        match pc {
            0x82063500 => {
    //   block [0x82063500..0x82063524)
	// 82063500: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 82063504: 548AC00E  slwi r10, r4, 0x18
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063508: 556B0206  rlwinm r11, r11, 0, 8, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8206350C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063510: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82063514: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063518: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 8206351C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063528 size=28
    let mut pc: u32 = 0x82063528;
    'dispatch: loop {
        match pc {
            0x82063528 => {
    //   block [0x82063528..0x82063544)
	// 82063528: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206352C: 508BE006  rlwimi r11, r4, 0x1c, 0, 3
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[11].u64 & 0xFFFFFFFF0FFFFFFF);
	// 82063530: 9163292C  stw r11, 0x292c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10540 as u32), ctx.r[11].u32 ) };
	// 82063534: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063538: 616B2000  ori r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 8192;
	// 8206353C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063548 size=32
    let mut pc: u32 = 0x82063548;
    'dispatch: loop {
        match pc {
            0x82063548 => {
    //   block [0x82063548..0x82063568)
	// 82063548: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206354C: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063550: 7D6B2378  or r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[4].u64;
	// 82063554: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82063558: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8206355C: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82063560: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063568 size=36
    let mut pc: u32 = 0x82063568;
    'dispatch: loop {
        match pc {
            0x82063568 => {
    //   block [0x82063568..0x8206358C)
	// 82063568: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206356C: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063570: 556B072E  rlwinm r11, r11, 0, 0x1c, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063574: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063578: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 8206357C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063580: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82063584: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063590 size=36
    let mut pc: u32 = 0x82063590;
    'dispatch: loop {
        match pc {
            0x82063590 => {
    //   block [0x82063590..0x820635B4)
	// 82063590: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82063594: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063598: 556B0626  rlwinm r11, r11, 0, 0x18, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8206359C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820635A0: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 820635A4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820635A8: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 820635AC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820635B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820635B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820635B8 size=36
    let mut pc: u32 = 0x820635B8;
    'dispatch: loop {
        match pc {
            0x820635B8 => {
    //   block [0x820635B8..0x820635DC)
	// 820635B8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 820635BC: 548A6026  slwi r10, r4, 0xc
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(12);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 820635C0: 556B051E  rlwinm r11, r11, 0, 0x14, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 820635C4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820635C8: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 820635CC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820635D0: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 820635D4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820635D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820635E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820635E0 size=36
    let mut pc: u32 = 0x820635E0;
    'dispatch: loop {
        match pc {
            0x820635E0 => {
    //   block [0x820635E0..0x82063604)
	// 820635E0: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 820635E4: 548A801E  slwi r10, r4, 0x10
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 820635E8: 556B0416  rlwinm r11, r11, 0, 0x10, 0xb
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 820635EC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820635F0: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 820635F4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820635F8: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 820635FC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063608 size=36
    let mut pc: u32 = 0x82063608;
    'dispatch: loop {
        match pc {
            0x82063608 => {
    //   block [0x82063608..0x8206362C)
	// 82063608: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206360C: 548AA016  slwi r10, r4, 0x14
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(20);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063610: 556B030E  rlwinm r11, r11, 0, 0xc, 7
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063614: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063618: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 8206361C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063620: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 82063624: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063630 size=36
    let mut pc: u32 = 0x82063630;
    'dispatch: loop {
        match pc {
            0x82063630 => {
    //   block [0x82063630..0x82063654)
	// 82063630: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 82063634: 548AC00E  slwi r10, r4, 0x18
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063638: 556B0206  rlwinm r11, r11, 0, 8, 3
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8206363C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063640: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82063644: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063648: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 8206364C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063658 size=28
    let mut pc: u32 = 0x82063658;
    'dispatch: loop {
        match pc {
            0x82063658 => {
    //   block [0x82063658..0x82063674)
	// 82063658: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206365C: 508BE006  rlwimi r11, r4, 0x1c, 0, 3
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(28) as u64) & 0x00000000F0000000) | (ctx.r[11].u64 & 0xFFFFFFFF0FFFFFFF);
	// 82063660: 91632930  stw r11, 0x2930(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10544 as u32), ctx.r[11].u32 ) };
	// 82063664: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063668: 616B1000  ori r11, r11, 0x1000
	ctx.r[11].u64 = ctx.r[11].u64 | 4096;
	// 8206366C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063678 size=12
    let mut pc: u32 = 0x82063678;
    'dispatch: loop {
        match pc {
            0x82063678 => {
    //   block [0x82063678..0x82063684)
	// 82063678: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206367C: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063688 size=12
    let mut pc: u32 = 0x82063688;
    'dispatch: loop {
        match pc {
            0x82063688 => {
    //   block [0x82063688..0x82063694)
	// 82063688: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206368C: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063698 size=12
    let mut pc: u32 = 0x82063698;
    'dispatch: loop {
        match pc {
            0x82063698 => {
    //   block [0x82063698..0x820636A4)
	// 82063698: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 8206369C: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 820636A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820636A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820636A8 size=12
    let mut pc: u32 = 0x820636A8;
    'dispatch: loop {
        match pc {
            0x820636A8 => {
    //   block [0x820636A8..0x820636B4)
	// 820636A8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820636AC: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 820636B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820636B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820636B8 size=12
    let mut pc: u32 = 0x820636B8;
    'dispatch: loop {
        match pc {
            0x820636B8 => {
    //   block [0x820636B8..0x820636C4)
	// 820636B8: A163292C  lhz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820636BC: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 820636C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820636C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820636C8 size=12
    let mut pc: u32 = 0x820636C8;
    'dispatch: loop {
        match pc {
            0x820636C8 => {
    //   block [0x820636C8..0x820636D4)
	// 820636C8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820636CC: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 820636D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820636D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820636D8 size=12
    let mut pc: u32 = 0x820636D8;
    'dispatch: loop {
        match pc {
            0x820636D8 => {
    //   block [0x820636D8..0x820636E4)
	// 820636D8: 8963292C  lbz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820636DC: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 820636E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820636E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820636E8 size=12
    let mut pc: u32 = 0x820636E8;
    'dispatch: loop {
        match pc {
            0x820636E8 => {
    //   block [0x820636E8..0x820636F4)
	// 820636E8: 8163292C  lwz r11, 0x292c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10540 as u32) ) } as u64;
	// 820636EC: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 820636F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820636F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820636F8 size=12
    let mut pc: u32 = 0x820636F8;
    'dispatch: loop {
        match pc {
            0x820636F8 => {
    //   block [0x820636F8..0x82063704)
	// 820636F8: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 820636FC: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063708 size=12
    let mut pc: u32 = 0x82063708;
    'dispatch: loop {
        match pc {
            0x82063708 => {
    //   block [0x82063708..0x82063714)
	// 82063708: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206370C: 5563E73E  rlwinm r3, r11, 0x1c, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063718 size=12
    let mut pc: u32 = 0x82063718;
    'dispatch: loop {
        match pc {
            0x82063718 => {
    //   block [0x82063718..0x82063724)
	// 82063718: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206371C: 5563C73E  rlwinm r3, r11, 0x18, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82063720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063728 size=12
    let mut pc: u32 = 0x82063728;
    'dispatch: loop {
        match pc {
            0x82063728 => {
    //   block [0x82063728..0x82063734)
	// 82063728: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206372C: 5563A73E  rlwinm r3, r11, 0x14, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000FFFu64;
	// 82063730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063738 size=12
    let mut pc: u32 = 0x82063738;
    'dispatch: loop {
        match pc {
            0x82063738 => {
    //   block [0x82063738..0x82063744)
	// 82063738: A1632930  lhz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206373C: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063748 size=12
    let mut pc: u32 = 0x82063748;
    'dispatch: loop {
        match pc {
            0x82063748 => {
    //   block [0x82063748..0x82063754)
	// 82063748: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206374C: 5563673E  rlwinm r3, r11, 0xc, 0x1c, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82063750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063758 size=12
    let mut pc: u32 = 0x82063758;
    'dispatch: loop {
        match pc {
            0x82063758 => {
    //   block [0x82063758..0x82063764)
	// 82063758: 89632930  lbz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206375C: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063768 size=12
    let mut pc: u32 = 0x82063768;
    'dispatch: loop {
        match pc {
            0x82063768 => {
    //   block [0x82063768..0x82063774)
	// 82063768: 81632930  lwz r11, 0x2930(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10544 as u32) ) } as u64;
	// 8206376C: 5563273E  srwi r3, r11, 0x1c
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(28);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82063770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063778 size=64
    let mut pc: u32 = 0x82063778;
    'dispatch: loop {
        match pc {
            0x82063778 => {
    //   block [0x82063778..0x820637B8)
	// 82063778: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8206377C: 3960043F  li r11, 0x43f
	ctx.r[11].s64 = 1087;
	// 82063780: 409A0008  bne cr6, 0x82063788
	if !ctx.cr[6].eq {
	pc = 0x82063788; continue 'dispatch;
	}
	// 82063784: 39600400  li r11, 0x400
	ctx.r[11].s64 = 1024;
	// 82063788: 9163294C  stw r11, 0x294c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10572 as u32), ctx.r[11].u32 ) };
	// 8206378C: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82063790: 81432944  lwz r10, 0x2944(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10564 as u32) ) } as u64;
	// 82063794: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82063798: 516A83DE  rlwimi r10, r11, 0x10, 0xf, 0xf
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x0000000000010000) | (ctx.r[10].u64 & 0xFFFFFFFFFFFEFFFF);
	// 8206379C: 91432944  stw r10, 0x2944(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10564 as u32), ctx.r[10].u32 ) };
	// 820637A0: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820637A4: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 820637A8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820637AC: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 820637B0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820637B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820637B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820637B8 size=12
    let mut pc: u32 = 0x820637B8;
    'dispatch: loop {
        match pc {
            0x820637B8 => {
    //   block [0x820637B8..0x820637C4)
	// 820637B8: 8163294C  lwz r11, 0x294c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10572 as u32) ) } as u64;
	// 820637BC: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 820637C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820637C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820637C8 size=16
    let mut pc: u32 = 0x820637C8;
    'dispatch: loop {
        match pc {
            0x820637C8 => {
    //   block [0x820637C8..0x820637D8)
	// 820637C8: 81633098  lwz r11, 0x3098(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12440 as u32) ) } as u64;
	// 820637CC: 90832EFC  stw r4, 0x2efc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12028 as u32), ctx.r[4].u32 ) };
	// 820637D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 820637D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820637D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820637D8 size=40
    let mut pc: u32 = 0x820637D8;
    'dispatch: loop {
        match pc {
            0x820637D8 => {
    //   block [0x820637D8..0x82063800)
	// 820637D8: 81032884  lwz r8, 0x2884(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10372 as u32) ) } as u64;
	// 820637DC: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 820637E0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 820637E4: 419A001C  beq cr6, 0x82063800
	if ctx.cr[6].eq {
		sub_82063800(ctx, base);
		return;
	}
	// 820637E8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 820637EC: 419A0014  beq cr6, 0x82063800
	if ctx.cr[6].eq {
		sub_82063800(ctx, base);
		return;
	}
	// 820637F0: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 820637F4: 419A000C  beq cr6, 0x82063800
	if ctx.cr[6].eq {
		sub_82063800(ctx, base);
		return;
	}
	// 820637F8: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 820637FC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063800 size=12
    let mut pc: u32 = 0x82063800;
    'dispatch: loop {
        match pc {
            0x82063800 => {
    //   block [0x82063800..0x8206380C)
	// 82063800: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82063804: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82063808: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8206380C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8206380C size=76
    let mut pc: u32 = 0x8206380C;
    'dispatch: loop {
        match pc {
            0x8206380C => {
    //   block [0x8206380C..0x82063858)
	// 8206380C: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82063810: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82063814: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82063818: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8206381C: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82063820: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82063824: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82063828: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8206382C: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82063830: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063834: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063838: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8206383C: 798CC7E6  rldicr r12, r12, 0x38, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(56) & 0xFFFFFFFFFFFFFFFF;
	// 82063840: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82063844: 91632884  stw r11, 0x2884(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10372 as u32), ctx.r[11].u32 ) };
	// 82063848: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8206384C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063850: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063858 size=8
    let mut pc: u32 = 0x82063858;
    'dispatch: loop {
        match pc {
            0x82063858 => {
    //   block [0x82063858..0x82063860)
	// 82063858: 80632EFC  lwz r3, 0x2efc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12028 as u32) ) } as u64;
	// 8206385C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063860 size=16
    let mut pc: u32 = 0x82063860;
    'dispatch: loop {
        match pc {
            0x82063860 => {
    //   block [0x82063860..0x82063870)
	// 82063860: 8163309C  lwz r11, 0x309c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12444 as u32) ) } as u64;
	// 82063864: 90832F00  stw r4, 0x2f00(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12032 as u32), ctx.r[4].u32 ) };
	// 82063868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8206386C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063870 size=40
    let mut pc: u32 = 0x82063870;
    'dispatch: loop {
        match pc {
            0x82063870 => {
    //   block [0x82063870..0x82063898)
	// 82063870: 8103288C  lwz r8, 0x288c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10380 as u32) ) } as u64;
	// 82063874: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82063878: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8206387C: 419A001C  beq cr6, 0x82063898
	if ctx.cr[6].eq {
		sub_82063898(ctx, base);
		return;
	}
	// 82063880: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82063884: 419A0014  beq cr6, 0x82063898
	if ctx.cr[6].eq {
		sub_82063898(ctx, base);
		return;
	}
	// 82063888: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 8206388C: 419A000C  beq cr6, 0x82063898
	if ctx.cr[6].eq {
		sub_82063898(ctx, base);
		return;
	}
	// 82063890: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 82063894: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063898 size=12
    let mut pc: u32 = 0x82063898;
    'dispatch: loop {
        match pc {
            0x82063898 => {
    //   block [0x82063898..0x820638A4)
	// 82063898: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 8206389C: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820638A0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820638A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820638A4 size=76
    let mut pc: u32 = 0x820638A4;
    'dispatch: loop {
        match pc {
            0x820638A4 => {
    //   block [0x820638A4..0x820638F0)
	// 820638A4: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 820638A8: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 820638AC: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 820638B0: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 820638B4: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 820638B8: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 820638BC: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 820638C0: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 820638C4: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 820638C8: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 820638CC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 820638D0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820638D4: 798CB7E6  rldicr r12, r12, 0x36, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(54) & 0xFFFFFFFFFFFFFFFF;
	// 820638D8: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 820638DC: 9163288C  stw r11, 0x288c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10380 as u32), ctx.r[11].u32 ) };
	// 820638E0: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 820638E4: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 820638E8: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 820638EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820638F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820638F0 size=8
    let mut pc: u32 = 0x820638F0;
    'dispatch: loop {
        match pc {
            0x820638F0 => {
    //   block [0x820638F0..0x820638F8)
	// 820638F0: 80632F00  lwz r3, 0x2f00(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12032 as u32) ) } as u64;
	// 820638F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820638F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820638F8 size=16
    let mut pc: u32 = 0x820638F8;
    'dispatch: loop {
        match pc {
            0x820638F8 => {
    //   block [0x820638F8..0x82063908)
	// 820638F8: 816330A0  lwz r11, 0x30a0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12448 as u32) ) } as u64;
	// 820638FC: 90832F04  stw r4, 0x2f04(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12036 as u32), ctx.r[4].u32 ) };
	// 82063900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82063904: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063908 size=40
    let mut pc: u32 = 0x82063908;
    'dispatch: loop {
        match pc {
            0x82063908 => {
    //   block [0x82063908..0x82063930)
	// 82063908: 81032890  lwz r8, 0x2890(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10384 as u32) ) } as u64;
	// 8206390C: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 82063910: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82063914: 419A001C  beq cr6, 0x82063930
	if ctx.cr[6].eq {
		sub_82063930(ctx, base);
		return;
	}
	// 82063918: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8206391C: 419A0014  beq cr6, 0x82063930
	if ctx.cr[6].eq {
		sub_82063930(ctx, base);
		return;
	}
	// 82063920: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82063924: 419A000C  beq cr6, 0x82063930
	if ctx.cr[6].eq {
		sub_82063930(ctx, base);
		return;
	}
	// 82063928: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 8206392C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063930 size=12
    let mut pc: u32 = 0x82063930;
    'dispatch: loop {
        match pc {
            0x82063930 => {
    //   block [0x82063930..0x8206393C)
	// 82063930: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 82063934: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82063938: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8206393C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8206393C size=76
    let mut pc: u32 = 0x8206393C;
    'dispatch: loop {
        match pc {
            0x8206393C => {
    //   block [0x8206393C..0x82063988)
	// 8206393C: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82063940: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 82063944: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82063948: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 8206394C: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 82063950: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82063954: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82063958: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 8206395C: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 82063960: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063964: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063968: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8206396C: 798CAFE6  rldicr r12, r12, 0x35, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(53) & 0xFFFFFFFFFFFFFFFF;
	// 82063970: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82063974: 91632890  stw r11, 0x2890(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10384 as u32), ctx.r[11].u32 ) };
	// 82063978: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 8206397C: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063980: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063988 size=8
    let mut pc: u32 = 0x82063988;
    'dispatch: loop {
        match pc {
            0x82063988 => {
    //   block [0x82063988..0x82063990)
	// 82063988: 80632F04  lwz r3, 0x2f04(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12036 as u32) ) } as u64;
	// 8206398C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063990 size=16
    let mut pc: u32 = 0x82063990;
    'dispatch: loop {
        match pc {
            0x82063990 => {
    //   block [0x82063990..0x820639A0)
	// 82063990: 816330A4  lwz r11, 0x30a4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12452 as u32) ) } as u64;
	// 82063994: 90832F08  stw r4, 0x2f08(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12040 as u32), ctx.r[4].u32 ) };
	// 82063998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8206399C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820639A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820639A0 size=40
    let mut pc: u32 = 0x820639A0;
    'dispatch: loop {
        match pc {
            0x820639A0 => {
    //   block [0x820639A0..0x820639C8)
	// 820639A0: 81032894  lwz r8, 0x2894(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10388 as u32) ) } as u64;
	// 820639A4: 550B873E  rlwinm r11, r8, 0x10, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 820639A8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 820639AC: 419A001C  beq cr6, 0x820639c8
	if ctx.cr[6].eq {
		sub_820639C8(ctx, base);
		return;
	}
	// 820639B0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 820639B4: 419A0014  beq cr6, 0x820639c8
	if ctx.cr[6].eq {
		sub_820639C8(ctx, base);
		return;
	}
	// 820639B8: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 820639BC: 419A000C  beq cr6, 0x820639c8
	if ctx.cr[6].eq {
		sub_820639C8(ctx, base);
		return;
	}
	// 820639C0: 2B0B000C  cmplwi cr6, r11, 0xc
	ctx.cr[6].compare_u32(ctx.r[11].u32, 12 as u32, &mut ctx.xer);
	// 820639C4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820639C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820639C8 size=12
    let mut pc: u32 = 0x820639C8;
    'dispatch: loop {
        match pc {
            0x820639C8 => {
    //   block [0x820639C8..0x820639D4)
	// 820639C8: 550A6FFE  rlwinm r10, r8, 0xd, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x0007FFFFu64;
	// 820639CC: 7D4A2279  xor. r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 ^ ctx.r[4].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 820639D0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820639D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820639D4 size=76
    let mut pc: u32 = 0x820639D4;
    'dispatch: loop {
        match pc {
            0x820639D4 => {
    //   block [0x820639D4..0x82063A20)
	// 820639D4: 5569F87E  srwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 820639D8: 3944FFFF  addi r10, r4, -1
	ctx.r[10].s64 = ctx.r[4].s64 + -1;
	// 820639DC: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 820639E0: 7D4750F8  nor r7, r10, r10
	ctx.r[7].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 820639E4: 3929FFFD  addi r9, r9, -3
	ctx.r[9].s64 = ctx.r[9].s64 + -3;
	// 820639E8: 556B881C  slwi r11, r11, 0x11
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(17);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 820639EC: 54E7801E  slwi r7, r7, 0x10
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(16);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 820639F0: 7D2A5038  and r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & ctx.r[10].u64;
	// 820639F4: 7CEB5838  and r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 & ctx.r[11].u64;
	// 820639F8: 554A801E  slwi r10, r10, 0x10
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 820639FC: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063A00: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82063A04: 798CA7E6  rldicr r12, r12, 0x34, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(52) & 0xFFFFFFFFFFFFFFFF;
	// 82063A08: 510B0416  rlwimi r11, r8, 0, 0x10, 0xb
	ctx.r[11].u64 = (((ctx.r[8].u32).rotate_left(0) as u64) & 0xFFFFFFFFFFF0FFFF) | (ctx.r[11].u64 & 0x00000000000F0000);
	// 82063A0C: 91632894  stw r11, 0x2894(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10388 as u32), ctx.r[11].u32 ) };
	// 82063A10: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063A14: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063A18: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063A20 size=8
    let mut pc: u32 = 0x82063A20;
    'dispatch: loop {
        match pc {
            0x82063A20 => {
    //   block [0x82063A20..0x82063A28)
	// 82063A20: 80632F08  lwz r3, 0x2f08(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12040 as u32) ) } as u64;
	// 82063A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063A28 size=36
    let mut pc: u32 = 0x82063A28;
    'dispatch: loop {
        match pc {
            0x82063A28 => {
    //   block [0x82063A28..0x82063A4C)
	// 82063A28: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063A2C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063A30: D0032980  stfs f0, 0x2980(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10624 as u32), tmp.u32 ) };
	// 82063A34: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063A38: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82063A3C: 798C7FE6  rldicr r12, r12, 0x2f, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(47) & 0xFFFFFFFFFFFFFFFF;
	// 82063A40: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82063A44: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063A50 size=16
    let mut pc: u32 = 0x82063A50;
    'dispatch: loop {
        match pc {
            0x82063A50 => {
    //   block [0x82063A50..0x82063A60)
	// 82063A50: C0032980  lfs f0, 0x2980(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10624 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063A54: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82063A58: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82063A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063A60 size=36
    let mut pc: u32 = 0x82063A60;
    'dispatch: loop {
        match pc {
            0x82063A60 => {
    //   block [0x82063A60..0x82063A84)
	// 82063A60: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063A64: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063A68: D003297C  stfs f0, 0x297c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10620 as u32), tmp.u32 ) };
	// 82063A6C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063A70: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82063A74: 798C87E6  rldicr r12, r12, 0x30, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(48) & 0xFFFFFFFFFFFFFFFF;
	// 82063A78: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82063A7C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063A88 size=16
    let mut pc: u32 = 0x82063A88;
    'dispatch: loop {
        match pc {
            0x82063A88 => {
    //   block [0x82063A88..0x82063A98)
	// 82063A88: C003297C  lfs f0, 0x297c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10620 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063A8C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82063A90: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82063A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063A98 size=36
    let mut pc: u32 = 0x82063A98;
    'dispatch: loop {
        match pc {
            0x82063A98 => {
    //   block [0x82063A98..0x82063ABC)
	// 82063A98: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 82063A9C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063AA0: 5164003A  rlwimi r4, r11, 0, 0, 0x1d
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[4].u64 & 0xFFFFFFFF00000003);
	// 82063AA4: 798C8FE6  rldicr r12, r12, 0x31, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(49) & 0xFFFFFFFFFFFFFFFF;
	// 82063AA8: 90832978  stw r4, 0x2978(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10616 as u32), ctx.r[4].u32 ) };
	// 82063AAC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82063AB0: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063AB4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063AC0 size=12
    let mut pc: u32 = 0x82063AC0;
    'dispatch: loop {
        match pc {
            0x82063AC0 => {
    //   block [0x82063AC0..0x82063ACC)
	// 82063AC0: 81632978  lwz r11, 0x2978(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10616 as u32) ) } as u64;
	// 82063AC4: 556307BE  clrlwi r3, r11, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82063AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063AD0 size=36
    let mut pc: u32 = 0x82063AD0;
    'dispatch: loop {
        match pc {
            0x82063AD0 => {
    //   block [0x82063AD0..0x82063AF4)
	// 82063AD0: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 82063AD4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063AD8: 5164003C  rlwimi r4, r11, 0, 0, 0x1e
	ctx.r[4].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFE) | (ctx.r[4].u64 & 0xFFFFFFFF00000001);
	// 82063ADC: 798C1FE6  rldicr r12, r12, 0x23, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(35) & 0xFFFFFFFFFFFFFFFF;
	// 82063AE0: 908329C0  stw r4, 0x29c0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10688 as u32), ctx.r[4].u32 ) };
	// 82063AE4: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82063AE8: 7D6B6378  or r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[12].u64;
	// 82063AEC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063AF8 size=12
    let mut pc: u32 = 0x82063AF8;
    'dispatch: loop {
        match pc {
            0x82063AF8 => {
    //   block [0x82063AF8..0x82063B04)
	// 82063AF8: 816329C0  lwz r11, 0x29c0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10688 as u32) ) } as u64;
	// 82063AFC: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82063B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B08 size=28
    let mut pc: u32 = 0x82063B08;
    'dispatch: loop {
        match pc {
            0x82063B08 => {
    //   block [0x82063B08..0x82063B24)
	// 82063B08: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82063B0C: 508BAA94  rlwimi r11, r4, 0x15, 0xa, 0xa
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(21) as u64) & 0x0000000000200000) | (ctx.r[11].u64 & 0xFFFFFFFFFFDFFFFF);
	// 82063B10: 91632948  stw r11, 0x2948(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10568 as u32), ctx.r[11].u32 ) };
	// 82063B14: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063B18: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 82063B1C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B28 size=12
    let mut pc: u32 = 0x82063B28;
    'dispatch: loop {
        match pc {
            0x82063B28 => {
    //   block [0x82063B28..0x82063B34)
	// 82063B28: 81632948  lwz r11, 0x2948(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10568 as u32) ) } as u64;
	// 82063B2C: 55635FFE  rlwinm r3, r11, 0xb, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	// 82063B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B38 size=28
    let mut pc: u32 = 0x82063B38;
    'dispatch: loop {
        match pc {
            0x82063B38 => {
    //   block [0x82063B38..0x82063B54)
	// 82063B38: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063B3C: 908328D8  stw r4, 0x28d8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10456 as u32), ctx.r[4].u32 ) };
	// 82063B40: E9430010  ld r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063B44: 798C37E6  rldicr r12, r12, 0x26, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(38) & 0xFFFFFFFFFFFFFFFF;
	// 82063B48: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82063B4C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B58 size=8
    let mut pc: u32 = 0x82063B58;
    'dispatch: loop {
        match pc {
            0x82063B58 => {
    //   block [0x82063B58..0x82063B60)
	// 82063B58: 806328D8  lwz r3, 0x28d8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10456 as u32) ) } as u64;
	// 82063B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B60 size=28
    let mut pc: u32 = 0x82063B60;
    'dispatch: loop {
        match pc {
            0x82063B60 => {
    //   block [0x82063B60..0x82063B7C)
	// 82063B60: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82063B64: 508B26F6  rlwimi r11, r4, 4, 0x1b, 0x1b
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(4) as u64) & 0x0000000000000010) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFEF);
	// 82063B68: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 82063B6C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063B70: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82063B74: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B80 size=12
    let mut pc: u32 = 0x82063B80;
    'dispatch: loop {
        match pc {
            0x82063B80 => {
    //   block [0x82063B80..0x82063B8C)
	// 82063B80: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82063B84: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82063B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063B90 size=28
    let mut pc: u32 = 0x82063B90;
    'dispatch: loop {
        match pc {
            0x82063B90 => {
    //   block [0x82063B90..0x82063BAC)
	// 82063B90: 8163293C  lwz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82063B94: 508BC00E  rlwimi r11, r4, 0x18, 0, 7
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(24) as u64) & 0x00000000FF000000) | (ctx.r[11].u64 & 0xFFFFFFFF00FFFFFF);
	// 82063B98: 9163293C  stw r11, 0x293c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10556 as u32), ctx.r[11].u32 ) };
	// 82063B9C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063BA0: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82063BA4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063BB0 size=20
    let mut pc: u32 = 0x82063BB0;
    'dispatch: loop {
        match pc {
            0x82063BB0 => {
    //   block [0x82063BB0..0x82063BC4)
	// 82063BB0: 8963293C  lbz r11, 0x293c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10556 as u32) ) } as u64;
	// 82063BB4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82063BB8: 514B063A  rlwimi r11, r10, 0, 0x18, 0x1d
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000000000FC) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF03);
	// 82063BBC: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82063BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063BC8 size=36
    let mut pc: u32 = 0x82063BC8;
    'dispatch: loop {
        match pc {
            0x82063BC8 => {
    //   block [0x82063BC8..0x82063BEC)
	// 82063BC8: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063BCC: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063BD0: D00329CC  stfs f0, 0x29cc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10700 as u32), tmp.u32 ) };
	// 82063BD4: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063BD8: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82063BDC: 798C07E6  rldicr r12, r12, 0x20, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 82063BE0: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82063BE4: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063BF0 size=8
    let mut pc: u32 = 0x82063BF0;
    'dispatch: loop {
        match pc {
            0x82063BF0 => {
    //   block [0x82063BF0..0x82063BF8)
	// 82063BF0: 806329CC  lwz r3, 0x29cc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10700 as u32) ) } as u64;
	// 82063BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063BF8 size=36
    let mut pc: u32 = 0x82063BF8;
    'dispatch: loop {
        match pc {
            0x82063BF8 => {
    //   block [0x82063BF8..0x82063C1C)
	// 82063BF8: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063BFC: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063C00: D00329C4  stfs f0, 0x29c4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10692 as u32), tmp.u32 ) };
	// 82063C04: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063C08: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82063C0C: 798C17E6  rldicr r12, r12, 0x22, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(34) & 0xFFFFFFFFFFFFFFFF;
	// 82063C10: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82063C14: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063C20 size=8
    let mut pc: u32 = 0x82063C20;
    'dispatch: loop {
        match pc {
            0x82063C20 => {
    //   block [0x82063C20..0x82063C28)
	// 82063C20: 806329C4  lwz r3, 0x29c4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10692 as u32) ) } as u64;
	// 82063C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063C28 size=28
    let mut pc: u32 = 0x82063C28;
    'dispatch: loop {
        match pc {
            0x82063C28 => {
    //   block [0x82063C28..0x82063C44)
	// 82063C28: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063C2C: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063C30: D00329D0  stfs f0, 0x29d0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10704 as u32), tmp.u32 ) };
	// 82063C34: E9630020  ld r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82063C38: 656B8000  oris r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 2147483648;
	// 82063C3C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063C48 size=8
    let mut pc: u32 = 0x82063C48;
    'dispatch: loop {
        match pc {
            0x82063C48 => {
    //   block [0x82063C48..0x82063C50)
	// 82063C48: 806329D0  lwz r3, 0x29d0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10704 as u32) ) } as u64;
	// 82063C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82063C50 size=36
    let mut pc: u32 = 0x82063C50;
    'dispatch: loop {
        match pc {
            0x82063C50 => {
    //   block [0x82063C50..0x82063C74)
	// 82063C50: 9081001C  stw r4, 0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82063C54: C001001C  lfs f0, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82063C58: D00329C8  stfs f0, 0x29c8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10696 as u32), tmp.u32 ) };
	// 82063C5C: 39800001  li r12, 1
	ctx.r[12].s64 = 1;
	// 82063C60: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82063C64: 798C0FE6  rldicr r12, r12, 0x21, 0x3f
	ctx.r[12].u64 = (ctx.r[12].u64).rotate_left(33) & 0xFFFFFFFFFFFFFFFF;
	// 82063C68: 7D4B6378  or r11, r10, r12
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[12].u64;
	// 82063C6C: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82063C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063C78 size=8
    let mut pc: u32 = 0x82063C78;
    'dispatch: loop {
        match pc {
            0x82063C78 => {
    //   block [0x82063C78..0x82063C80)
	// 82063C78: 806329C8  lwz r3, 0x29c8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10696 as u32) ) } as u64;
	// 82063C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063C80 size=28
    let mut pc: u32 = 0x82063C80;
    'dispatch: loop {
        match pc {
            0x82063C80 => {
    //   block [0x82063C80..0x82063C9C)
	// 82063C80: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82063C84: 508BA256  rlwimi r11, r4, 0x14, 9, 0xb
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(20) as u64) & 0x0000000000700000) | (ctx.r[11].u64 & 0xFFFFFFFFFF8FFFFF);
	// 82063C88: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82063C8C: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063C90: 656B0002  oris r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 131072;
	// 82063C94: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063CA0 size=12
    let mut pc: u32 = 0x82063CA0;
    'dispatch: loop {
        match pc {
            0x82063CA0 => {
    //   block [0x82063CA0..0x82063CAC)
	// 82063CA0: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82063CA4: 5563677E  rlwinm r3, r11, 0xc, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 82063CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063CB0 size=56
    let mut pc: u32 = 0x82063CB0;
    'dispatch: loop {
        match pc {
            0x82063CB0 => {
    //   block [0x82063CB0..0x82063CE8)
	// 82063CB0: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82063CB4: 89432ABF  lbz r10, 0x2abf(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10943 as u32) ) } as u64;
	// 82063CB8: 508B8B1C  rlwimi r11, r4, 0x11, 0xc, 0xe
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(17) as u64) & 0x00000000000E0000) | (ctx.r[11].u64 & 0xFFFFFFFFFFF1FFFF);
	// 82063CBC: 554A06B5  rlwinm. r10, r10, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82063CC0: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82063CC4: 40820024  bne 0x82063ce8
	if !ctx.cr[0].eq {
		sub_82063CE8(ctx, base);
		return;
	}
	// 82063CC8: 556B031C  rlwinm r11, r11, 0, 0xc, 0xe
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82063CCC: 3D400004  lis r10, 4
	ctx.r[10].s64 = 262144;
	// 82063CD0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82063CD4: 409A0014  bne cr6, 0x82063ce8
	if !ctx.cr[6].eq {
		sub_82063CE8(ctx, base);
		return;
	}
	// 82063CD8: E9630028  ld r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 82063CDC: 3980FEFF  li r12, -0x101
	ctx.r[12].s64 = -257;
	// 82063CE0: 7D6B6038  and r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[12].u64;
	// 82063CE4: 4800000C  b 0x82063cf0
	sub_82063CE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063CE8 size=28
    let mut pc: u32 = 0x82063CE8;
    'dispatch: loop {
        match pc {
            0x82063CE8 => {
    //   block [0x82063CE8..0x82063D04)
	// 82063CE8: E9630028  ld r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 82063CEC: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82063CF0: F9630028  std r11, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 82063CF4: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063CF8: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82063CFC: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D08 size=12
    let mut pc: u32 = 0x82063D08;
    'dispatch: loop {
        match pc {
            0x82063D08 => {
    //   block [0x82063D08..0x82063D14)
	// 82063D08: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82063D0C: 55637F7E  rlwinm r3, r11, 0xf, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 82063D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D18 size=28
    let mut pc: u32 = 0x82063D18;
    'dispatch: loop {
        match pc {
            0x82063D18 => {
    //   block [0x82063D18..0x82063D34)
	// 82063D18: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82063D1C: 508B1F38  rlwimi r11, r4, 3, 0x1c, 0x1c
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(3) as u64) & 0x0000000000000008) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFF7);
	// 82063D20: 91632940  stw r11, 0x2940(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82063D24: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063D28: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82063D2C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D38 size=12
    let mut pc: u32 = 0x82063D38;
    'dispatch: loop {
        match pc {
            0x82063D38 => {
    //   block [0x82063D38..0x82063D44)
	// 82063D38: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82063D3C: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82063D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D48 size=28
    let mut pc: u32 = 0x82063D48;
    'dispatch: loop {
        match pc {
            0x82063D48 => {
    //   block [0x82063D48..0x82063D64)
	// 82063D48: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82063D4C: 508B177A  rlwimi r11, r4, 2, 0x1d, 0x1d
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(2) as u64) & 0x0000000000000004) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82063D50: 91632940  stw r11, 0x2940(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82063D54: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063D58: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82063D5C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D68 size=12
    let mut pc: u32 = 0x82063D68;
    'dispatch: loop {
        match pc {
            0x82063D68 => {
    //   block [0x82063D68..0x82063D74)
	// 82063D68: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82063D6C: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82063D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D78 size=28
    let mut pc: u32 = 0x82063D78;
    'dispatch: loop {
        match pc {
            0x82063D78 => {
    //   block [0x82063D78..0x82063D94)
	// 82063D78: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82063D7C: 508B2EB4  rlwimi r11, r4, 5, 0x1a, 0x1a
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(5) as u64) & 0x0000000000000020) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFFDF);
	// 82063D80: 91632940  stw r11, 0x2940(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(10560 as u32), ctx.r[11].u32 ) };
	// 82063D84: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063D88: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82063D8C: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063D98 size=12
    let mut pc: u32 = 0x82063D98;
    'dispatch: loop {
        match pc {
            0x82063D98 => {
    //   block [0x82063D98..0x82063DA4)
	// 82063D98: 81632940  lwz r11, 0x2940(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(10560 as u32) ) } as u64;
	// 82063D9C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82063DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DA8 size=20
    let mut pc: u32 = 0x82063DA8;
    'dispatch: loop {
        match pc {
            0x82063DA8 => {
    //   block [0x82063DA8..0x82063DBC)
	// 82063DA8: 98832942  stb r4, 0x2942(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(10562 as u32), ctx.r[4].u8 ) };
	// 82063DAC: E9630010  ld r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 82063DB0: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 82063DB4: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82063DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DC0 size=8
    let mut pc: u32 = 0x82063DC0;
    'dispatch: loop {
        match pc {
            0x82063DC0 => {
    //   block [0x82063DC0..0x82063DC8)
	// 82063DC0: 88632942  lbz r3, 0x2942(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10562 as u32) ) } as u64;
	// 82063DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DC8 size=8
    let mut pc: u32 = 0x82063DC8;
    'dispatch: loop {
        match pc {
            0x82063DC8 => {
    //   block [0x82063DC8..0x82063DD0)
	// 82063DC8: 9083351C  stw r4, 0x351c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(13596 as u32), ctx.r[4].u32 ) };
	// 82063DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DD0 size=8
    let mut pc: u32 = 0x82063DD0;
    'dispatch: loop {
        match pc {
            0x82063DD0 => {
    //   block [0x82063DD0..0x82063DD8)
	// 82063DD0: 8063351C  lwz r3, 0x351c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(13596 as u32) ) } as u64;
	// 82063DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DD8 size=16
    let mut pc: u32 = 0x82063DD8;
    'dispatch: loop {
        match pc {
            0x82063DD8 => {
    //   block [0x82063DD8..0x82063DE8)
	// 82063DD8: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82063DDC: 508BB890  rlwimi r11, r4, 0x17, 2, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x000000003F800000) | (ctx.r[11].u64 & 0xFFFFFFFFC07FFFFF);
	// 82063DE0: 91632E4C  stw r11, 0x2e4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(11852 as u32), ctx.r[11].u32 ) };
	// 82063DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DE8 size=12
    let mut pc: u32 = 0x82063DE8;
    'dispatch: loop {
        match pc {
            0x82063DE8 => {
    //   block [0x82063DE8..0x82063DF4)
	// 82063DE8: 81632E4C  lwz r11, 0x2e4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(11852 as u32) ) } as u64;
	// 82063DEC: 55634E7E  rlwinm r3, r11, 9, 0x19, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82063DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063DF8 size=200
    let mut pc: u32 = 0x82063DF8;
    'dispatch: loop {
        match pc {
            0x82063DF8 => {
    //   block [0x82063DF8..0x82063EC0)
	// 82063DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82063DFC: 4802ACA1  bl 0x8208ea9c
	ctx.lr = 0x82063E00;
	sub_8208EA60(ctx, base);
	// 82063E00: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82063E04: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82063E08: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82063E0C: 892A2E94  lbz r9, 0x2e94(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(11924 as u32) ) } as u64;
	// 82063E10: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82063E14: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82063E18: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82063E1C: 39080AD0  addi r8, r8, 0xad0
	ctx.r[8].s64 = ctx.r[8].s64 + 2768;
	// 82063E20: 54A7F0BE  srwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82063E24: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82063E28: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82063E2C: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82063E30: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82063E34: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82063E38: 54C6B7FE  rlwinm r6, r6, 0x16, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x000003FFu64;
	// 82063E3C: 7D09402E  lwzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82063E40: 50FD5D28  rlwimi r29, r7, 0xb, 0x14, 0x14
	ctx.r[29].u64 = (((ctx.r[7].u32).rotate_left(11) as u64) & 0x0000000000000800) | (ctx.r[29].u64 & 0xFFFFFFFFFFFFF7FF);
	// 82063E44: 7CC93B78  or r9, r6, r7
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[7].u64;
	// 82063E48: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82063E4C: 7BC6FFE6  rldicr r6, r30, 0x3f, 0x3f
	ctx.r[6].u64 = (ctx.r[30].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82063E50: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82063E54: 7D094878  andc r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 & !ctx.r[9].u64;
	// 82063E58: 78880020  clrldi r8, r4, 0x20
	ctx.r[8].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82063E5C: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82063E60: 7CC84436  srd r8, r6, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = (ctx.r[6].u64) >> ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 82063E64: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82063E68: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82063E6C: 513FAA54  rlwimi r31, r9, 0x15, 9, 0xa
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(21) as u64) & 0x0000000000600000) | (ctx.r[31].u64 & 0xFFFFFFFFFF9FFFFF);
	// 82063E70: 513FA90C  rlwimi r31, r9, 0x15, 4, 6
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(21) as u64) & 0x000000000E000000) | (ctx.r[31].u64 & 0xFFFFFFFFF1FFFFFF);
	// 82063E74: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82063E78: 892A2EE2  lbz r9, 0x2ee2(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82063E7C: 57EA003E  slwi r10, r31, 0
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063E80: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82063E84: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82063E88: 552AF0BE  srwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063E8C: 50E6FB7E  rlwimi r6, r7, 0x1f, 0xd, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFF80000);
	// 82063E90: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82063E94: 50E6F856  rlwimi r6, r7, 0x1f, 1, 0xb
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[6].u64 & 0xFFFFFFFF800FFFFF);
	// 82063E98: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82063E9C: 54C76D3E  rlwinm r7, r6, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x0007FFFFu64;
	// 82063EA0: 7CEA5038  and r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 82063EA4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82063EA8: 53AA003A  rlwimi r10, r29, 0, 0, 0x1d
	ctx.r[10].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[10].u64 & 0xFFFFFFFF00000003);
	// 82063EAC: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82063EB0: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82063EB4: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82063EB8: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063EBC: 4802AC30  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063EC0 size=52
    let mut pc: u32 = 0x82063EC0;
    'dispatch: loop {
        match pc {
            0x82063EC0 => {
    //   block [0x82063EC0..0x82063EF4)
	// 82063EC0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82063EC4: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82063EC8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82063ECC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82063ED0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82063ED4: 554BAFFE  rlwinm r11, r10, 0x15, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000007FFu64;
	// 82063ED8: 552A5D7E  srwi r10, r9, 0x15
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(21);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82063EDC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82063EE0: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82063EE4: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82063EE8: 516A077A  rlwimi r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x0000000000000004) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82063EEC: 5543077E  clrlwi r3, r10, 0x1d
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82063EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063EF8 size=148
    let mut pc: u32 = 0x82063EF8;
    'dispatch: loop {
        match pc {
            0x82063EF8 => {
    //   block [0x82063EF8..0x82063F8C)
	// 82063EF8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82063EFC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82063F00: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82063F04: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82063F08: 54A8083C  slwi r8, r5, 1
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82063F0C: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82063F10: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82063F14: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82063F18: 552907FA  rlwinm r9, r9, 0, 0x1f, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82063F1C: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82063F20: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82063F24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82063F28: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82063F2C: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82063F30: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82063F34: 78A5FFE6  rldicr r5, r5, 0x3f, 0x3f
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82063F38: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82063F3C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82063F40: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82063F44: 53FEFB7E  rlwimi r30, r31, 0x1f, 0xd, 0x1f
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[30].u64 & 0xFFFFFFFFFFF80000);
	// 82063F48: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 82063F4C: 53FEF856  rlwimi r30, r31, 0x1f, 1, 0xb
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[30].u64 & 0xFFFFFFFF800FFFFF);
	// 82063F50: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 82063F54: 57DF6D3E  rlwinm r31, r30, 0xd, 0x14, 0x1f
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x0007FFFFu64;
	// 82063F58: 7CA73C36  srd r7, r5, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[5].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82063F5C: 7FE94838  and r9, r31, r9
	ctx.r[9].u64 = ctx.r[31].u64 & ctx.r[9].u64;
	// 82063F60: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82063F64: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82063F68: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 82063F6C: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82063F70: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82063F74: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82063F78: 7CEB5B78  or r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 82063F7C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82063F80: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82063F84: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82063F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063F90 size=16
    let mut pc: u32 = 0x82063F90;
    'dispatch: loop {
        match pc {
            0x82063F90 => {
    //   block [0x82063F90..0x82063FA0)
	// 82063F90: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82063F94: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82063F98: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82063F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82063FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82063FA0 size=200
    let mut pc: u32 = 0x82063FA0;
    'dispatch: loop {
        match pc {
            0x82063FA0 => {
    //   block [0x82063FA0..0x82064068)
	// 82063FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82063FA4: 4802AAF9  bl 0x8208ea9c
	ctx.lr = 0x82063FA8;
	sub_8208EA60(ctx, base);
	// 82063FA8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82063FAC: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82063FB0: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82063FB4: 892A2E94  lbz r9, 0x2e94(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(11924 as u32) ) } as u64;
	// 82063FB8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82063FBC: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82063FC0: 5529103E  rotlwi r9, r9, 2
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82063FC4: 39080AD0  addi r8, r8, 0xad0
	ctx.r[8].s64 = ctx.r[8].s64 + 2768;
	// 82063FC8: 54A7F0BE  srwi r7, r5, 2
	ctx.r[7].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82063FCC: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82063FD0: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82063FD4: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82063FD8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82063FDC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82063FE0: 54C6AFFE  rlwinm r6, r6, 0x15, 0x1f, 0x1f
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x000007FFu64;
	// 82063FE4: 7D09402E  lwzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82063FE8: 50FD556A  rlwimi r29, r7, 0xa, 0x15, 0x15
	ctx.r[29].u64 = (((ctx.r[7].u32).rotate_left(10) as u64) & 0x0000000000000400) | (ctx.r[29].u64 & 0xFFFFFFFFFFFFFBFF);
	// 82063FEC: 7CC93B78  or r9, r6, r7
	ctx.r[9].u64 = ctx.r[6].u64 | ctx.r[7].u64;
	// 82063FF0: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82063FF4: 7BC6FFE6  rldicr r6, r30, 0x3f, 0x3f
	ctx.r[6].u64 = (ctx.r[30].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82063FF8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82063FFC: 7D094878  andc r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 & !ctx.r[9].u64;
	// 82064000: 78880020  clrldi r8, r4, 0x20
	ctx.r[8].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82064004: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82064008: 7CC84436  srd r8, r6, r8
	if (ctx.r[8].u8 & 0x40) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = (ctx.r[6].u64) >> ((ctx.r[8].u8 & 0x3F) as u32);
	}
	// 8206400C: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 82064010: 7D292B78  or r9, r9, r5
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 82064014: 513F9AD8  rlwimi r31, r9, 0x13, 0xb, 0xc
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(19) as u64) & 0x0000000000180000) | (ctx.r[31].u64 & 0xFFFFFFFFFFE7FFFF);
	// 82064018: 513F990C  rlwimi r31, r9, 0x13, 4, 6
	ctx.r[31].u64 = (((ctx.r[9].u32).rotate_left(19) as u64) & 0x000000000E000000) | (ctx.r[31].u64 & 0xFFFFFFFFF1FFFFFF);
	// 8206401C: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82064020: 892A2EE2  lbz r9, 0x2ee2(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82064024: 57EA003E  slwi r10, r31, 0
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82064028: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 8206402C: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82064030: 552AF0BE  srwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82064034: 50E6FB7E  rlwimi r6, r7, 0x1f, 0xd, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFF80000);
	// 82064038: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8206403C: 50E6F856  rlwimi r6, r7, 0x1f, 1, 0xb
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[6].u64 & 0xFFFFFFFF800FFFFF);
	// 82064040: 7D295078  andc r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82064044: 54C76D3E  rlwinm r7, r6, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0x0007FFFFu64;
	// 82064048: 7CEA5038  and r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 & ctx.r[10].u64;
	// 8206404C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82064050: 53AA003A  rlwimi r10, r29, 0, 0, 0x1d
	ctx.r[10].u64 = (((ctx.r[29].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[10].u64 & 0xFFFFFFFF00000003);
	// 82064054: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82064058: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 8206405C: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82064060: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064064: 4802AA88  b 0x8208eaec
	sub_8208EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064068 size=52
    let mut pc: u32 = 0x82064068;
    'dispatch: loop {
        match pc {
            0x82064068 => {
    //   block [0x82064068..0x8206409C)
	// 82064068: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 8206406C: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82064070: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82064074: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82064078: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8206407C: 554BB7FE  rlwinm r11, r10, 0x16, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000003FFu64;
	// 82064080: 552A6CFE  srwi r10, r9, 0x13
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shr(19);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82064084: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82064088: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8206408C: 7D6B58F8  nor r11, r11, r11
	ctx.r[11].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82064090: 516A077A  rlwimi r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x0000000000000004) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFFFB);
	// 82064094: 5543077E  clrlwi r3, r10, 0x1d
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000007u64;
	// 82064098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820640A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820640A0 size=140
    let mut pc: u32 = 0x820640A0;
    'dispatch: loop {
        match pc {
            0x820640A0 => {
    //   block [0x820640A0..0x8206412C)
	// 820640A0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 820640A4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 820640A8: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 820640AC: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 820640B0: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 820640B4: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 820640B8: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 820640BC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 820640C0: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 820640C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 820640C8: 7D262B78  or r6, r9, r5
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 820640CC: 791FFFE6  rldicr r31, r8, 0x3f, 0x3f
	ctx.r[31].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 820640D0: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 820640D4: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 820640D8: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 820640DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 820640E0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 820640E4: 53C5FB7E  rlwimi r5, r30, 0x1f, 0xd, 0x1f
	ctx.r[5].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFF80000);
	// 820640E8: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 820640EC: 53C5F856  rlwimi r5, r30, 0x1f, 1, 0xb
	ctx.r[5].u64 = (((ctx.r[30].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[5].u64 & 0xFFFFFFFF800FFFFF);
	// 820640F0: 78FE0020  clrldi r30, r7, 0x20
	ctx.r[30].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 820640F4: 54A76D3E  rlwinm r7, r5, 0xd, 0x14, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x0007FFFFu64;
	// 820640F8: 7FE5F436  srd r5, r31, r30
	if (ctx.r[30].u8 & 0x40) != 0 {
		ctx.r[5].u64 = 0;
	} else {
		ctx.r[5].u64 = (ctx.r[31].u64) >> ((ctx.r[30].u8 & 0x3F) as u32);
	}
	// 820640FC: 7CE74838  and r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 & ctx.r[9].u64;
	// 82064100: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82064104: 7D074214  add r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82064108: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 8206410C: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82064110: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82064114: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064118: 7CAB5B78  or r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 8206411C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064120: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82064124: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82064128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064130 size=16
    let mut pc: u32 = 0x82064130;
    'dispatch: loop {
        match pc {
            0x82064130 => {
    //   block [0x82064130..0x82064140)
	// 82064130: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82064134: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82064138: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8206413C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064140 size=60
    let mut pc: u32 = 0x82064140;
    'dispatch: loop {
        match pc {
            0x82064140 => {
    //   block [0x82064140..0x8206417C)
	// 82064140: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82064144: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82064148: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8206414C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82064150: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82064154: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82064158: 7929FFE6  rldicr r9, r9, 0x3f, 0x3f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 8206415C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82064160: 7D2A5436  srd r10, r9, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[9].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82064164: 50A8B9D0  rlwimi r8, r5, 0x17, 7, 8
	ctx.r[8].u64 = (((ctx.r[5].u32).rotate_left(23) as u64) & 0x0000000001800000) | (ctx.r[8].u64 & 0xFFFFFFFFFE7FFFFF);
	// 82064168: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8206416C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064170: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 82064174: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064180 size=20
    let mut pc: u32 = 0x82064180;
    'dispatch: loop {
        match pc {
            0x82064180 => {
    //   block [0x82064180..0x82064194)
	// 82064180: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82064184: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82064188: 816B048C  lwz r11, 0x48c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1164 as u32) ) } as u64;
	// 8206418C: 55634FBE  rlwinm r3, r11, 9, 0x1e, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x007FFFFFu64;
	// 82064190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064198 size=148
    let mut pc: u32 = 0x82064198;
    'dispatch: loop {
        match pc {
            0x82064198 => {
    //   block [0x82064198..0x8206422C)
	// 82064198: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8206419C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 820641A0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 820641A4: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 820641A8: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 820641AC: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 820641B0: 892B2EE2  lbz r9, 0x2ee2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 820641B4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 820641B8: 552907B8  rlwinm r9, r9, 0, 0x1e, 0x1c
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 820641BC: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 820641C0: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 820641C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 820641C8: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 820641CC: 54C9F0BE  srwi r9, r6, 2
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 820641D0: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 820641D4: 78A5FFE6  rldicr r5, r5, 0x3f, 0x3f
	ctx.r[5].u64 = (ctx.r[5].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 820641D8: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 820641DC: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 820641E0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 820641E4: 53FEFB7E  rlwimi r30, r31, 0x1f, 0xd, 0x1f
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000000007FFFF) | (ctx.r[30].u64 & 0xFFFFFFFFFFF80000);
	// 820641E8: 7CC84878  andc r8, r6, r9
	ctx.r[8].u64 = ctx.r[6].u64 & !ctx.r[9].u64;
	// 820641EC: 53FEF856  rlwimi r30, r31, 0x1f, 1, 0xb
	ctx.r[30].u64 = (((ctx.r[31].u32).rotate_left(31) as u64) & 0x000000007FF00000) | (ctx.r[30].u64 & 0xFFFFFFFF800FFFFF);
	// 820641F0: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 820641F4: 57DF6D3E  rlwinm r31, r30, 0xd, 0x14, 0x1f
	ctx.r[31].u64 = ctx.r[30].u32 as u64 & 0x0007FFFFu64;
	// 820641F8: 7CA73C36  srd r7, r5, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = (ctx.r[5].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 820641FC: 7FE94838  and r9, r31, r9
	ctx.r[9].u64 = ctx.r[31].u64 & ctx.r[9].u64;
	// 82064200: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82064204: 392B2EE2  addi r9, r11, 0x2ee2
	ctx.r[9].s64 = ctx.r[11].s64 + 12002;
	// 82064208: 5088003A  rlwimi r8, r4, 0, 0, 0x1d
	ctx.r[8].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x00000000FFFFFFFC) | (ctx.r[8].u64 & 0xFFFFFFFF00000003);
	// 8206420C: 910A0010  stw r8, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82064210: 98CB2EE2  stb r6, 0x2ee2(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12002 as u32), ctx.r[6].u8 ) };
	// 82064214: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064218: 7CEB5B78  or r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 | ctx.r[11].u64;
	// 8206421C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064220: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82064224: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82064228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064230 size=16
    let mut pc: u32 = 0x82064230;
    'dispatch: loop {
        match pc {
            0x82064230 => {
    //   block [0x82064230..0x82064240)
	// 82064230: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82064234: 896B2EE2  lbz r11, 0x2ee2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12002 as u32) ) } as u64;
	// 82064238: 5563F7FE  rlwinm r3, r11, 0x1e, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8206423C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064240 size=104
    let mut pc: u32 = 0x82064240;
    'dispatch: loop {
        match pc {
            0x82064240 => {
    //   block [0x82064240..0x820642A8)
	// 82064240: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82064244: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82064248: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8206424C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82064250: 55490529  rlwinm. r9, r10, 0, 0x14, 0x14
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82064254: 4082000C  bne 0x82064260
	if !ctx.cr[0].eq {
	pc = 0x82064260; continue 'dispatch;
	}
	// 82064258: 554A056B  rlwinm. r10, r10, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8206425C: 41820040  beq 0x8206429c
	if ctx.cr[0].eq {
	pc = 0x8206429C; continue 'dispatch;
	}
	// 82064260: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82064264: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82064268: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8206426C: 394A0AD0  addi r10, r10, 0xad0
	ctx.r[10].s64 = ctx.r[10].s64 + 2768;
	// 82064270: 38E40020  addi r7, r4, 0x20
	ctx.r[7].s64 = ctx.r[4].s64 + 32;
	// 82064274: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82064278: 78E70020  clrldi r7, r7, 0x20
	ctx.r[7].u64 = ctx.r[7].u64 & 0x00000000FFFFFFFFu64;
	// 8206427C: 78C6FFE6  rldicr r6, r6, 0x3f, 0x3f
	ctx.r[6].u64 = (ctx.r[6].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82064280: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82064284: 7CC83C36  srd r8, r6, r7
	if (ctx.r[7].u8 & 0x40) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = (ctx.r[6].u64) >> ((ctx.r[7].u8 & 0x3F) as u32);
	}
	// 82064288: 5149C90C  rlwimi r9, r10, 0x19, 4, 6
	ctx.r[9].u64 = (((ctx.r[10].u32).rotate_left(25) as u64) & 0x000000000E000000) | (ctx.r[9].u64 & 0xFFFFFFFFF1FFFFFF);
	// 8206428C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82064290: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064294: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 82064298: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 8206429C: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 820642A0: 98AB2E94  stb r5, 0x2e94(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11924 as u32), ctx.r[5].u8 ) };
	// 820642A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820642A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820642A8 size=12
    let mut pc: u32 = 0x820642A8;
    'dispatch: loop {
        match pc {
            0x820642A8 => {
    //   block [0x820642A8..0x820642B4)
	// 820642A8: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 820642AC: 886B2E94  lbz r3, 0x2e94(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11924 as u32) ) } as u64;
	// 820642B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820642B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x820642B8 size=92
    let mut pc: u32 = 0x820642B8;
    'dispatch: loop {
        match pc {
            0x820642B8 => {
    //   block [0x820642B8..0x82064314)
	// 820642B8: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 820642BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 820642C0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 820642C4: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 820642C8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 820642CC: C00A0B28  lfs f0, 0xb28(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2856 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 820642D0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 820642D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 820642D8: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 820642DC: 794AFFE6  rldicr r10, r10, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 820642E0: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 820642E4: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 820642E8: C1A10024  lfs f13, 0x24(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 820642EC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 820642F0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 820642F4: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 820642F8: 8121FFF4  lwz r9, -0xc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 820642FC: 51282DF4  rlwimi r8, r9, 5, 0x17, 0x1a
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(5) as u64) & 0x00000000000001E0) | (ctx.r[8].u64 & 0xFFFFFFFFFFFFFE1F);
	// 82064300: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82064304: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064308: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8206430C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82064318 size=68
    let mut pc: u32 = 0x82064318;
    'dispatch: loop {
        match pc {
            0x82064318 => {
    //   block [0x82064318..0x8206435C)
	// 82064318: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 8206431C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82064320: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82064324: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82064328: C00A0A10  lfs f0, 0xa10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2576 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8206432C: 556BB810  slwi r11, r11, 0x17
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(23);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82064330: 7D6BE670  srawi r11, r11, 0x1c
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 28) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 28) as i64;
	// 82064334: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82064338: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8206433C: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82064340: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82064344: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82064348: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8206434C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82064350: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82064354: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82064358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82064360 size=92
    let mut pc: u32 = 0x82064360;
    'dispatch: loop {
        match pc {
            0x82064360 => {
    //   block [0x82064360..0x820643BC)
	// 82064360: 90A10024  stw r5, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82064364: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82064368: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 8206436C: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82064370: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82064374: C00A0A2C  lfs f0, 0xa2c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2604 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82064378: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8206437C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82064380: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82064384: 794AFFE6  rldicr r10, r10, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[10].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82064388: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8206438C: 7D4A4C36  srd r10, r10, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = (ctx.r[10].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82064390: C1A10024  lfs f13, 0x24(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82064394: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82064398: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8206439C: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 820643A0: 8121FFF4  lwz r9, -0xc(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 820643A4: 512862A6  rlwimi r8, r9, 0xc, 0xa, 0x13
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(12) as u64) & 0x00000000003FF000) | (ctx.r[8].u64 & 0xFFFFFFFFFFC00FFF);
	// 820643A8: 910B0010  stw r8, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 820643AC: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 820643B0: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 820643B4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 820643B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820643C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x820643C0 size=64
    let mut pc: u32 = 0x820643C0;
    'dispatch: loop {
        match pc {
            0x820643C0 => {
    //   block [0x820643C0..0x82064400)
	// 820643C0: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 820643C4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 820643C8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 820643CC: 816B0490  lwz r11, 0x490(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1168 as u32) ) } as u64;
	// 820643D0: C00A0B2C  lfs f0, 0xb2c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2860 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 820643D4: 556B502A  slwi r11, r11, 0xa
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(10);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 820643D8: 7D6BB670  srawi r11, r11, 0x16
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 22) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 22) as i64;
	// 820643DC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 820643E0: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 820643E4: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 820643E8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 820643EC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 820643F0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 820643F4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 820643F8: 8061FFF0  lwz r3, -0x10(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 820643FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064400 size=108
    let mut pc: u32 = 0x82064400;
    'dispatch: loop {
        match pc {
            0x82064400 => {
    //   block [0x82064400..0x8206446C)
	// 82064400: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 82064404: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82064408: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8206440C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82064410: 419A0050  beq cr6, 0x82064460
	if ctx.cr[6].eq {
	pc = 0x82064460; continue 'dispatch;
	}
	// 82064414: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82064418: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 8206441C: 556BF73E  rlwinm r11, r11, 0x1e, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82064420: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82064424: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82064428: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8206442C: 41990008  bgt cr6, 0x82064434
	if ctx.cr[6].gt {
	pc = 0x82064434; continue 'dispatch;
	}
	// 82064430: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82064434: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82064438: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
	// 8206443C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82064440: 516916BA  rlwimi r9, r11, 2, 0x1a, 0x1d
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(2) as u64) & 0x000000000000003C) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFFC3);
	// 82064444: 78EBFFE6  rldicr r11, r7, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[7].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82064448: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8206444C: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 82064450: 7D6B5436  srd r11, r11, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 82064454: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064458: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8206445C: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064460: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82064464: 98AB2EAE  stb r5, 0x2eae(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11950 as u32), ctx.r[5].u8 ) };
	// 82064468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064470 size=12
    let mut pc: u32 = 0x82064470;
    'dispatch: loop {
        match pc {
            0x82064470 => {
    //   block [0x82064470..0x8206447C)
	// 82064470: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 82064474: 886B2EAE  lbz r3, 0x2eae(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11950 as u32) ) } as u64;
	// 82064478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064480 size=108
    let mut pc: u32 = 0x82064480;
    'dispatch: loop {
        match pc {
            0x82064480 => {
    //   block [0x82064480..0x820644EC)
	// 82064480: 39640C40  addi r11, r4, 0xc40
	ctx.r[11].s64 = ctx.r[4].s64 + 3136;
	// 82064484: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82064488: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8206448C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82064490: 419A0050  beq cr6, 0x820644e0
	if ctx.cr[6].eq {
	pc = 0x820644E0; continue 'dispatch;
	}
	// 82064494: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82064498: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 8206449C: 556BD73E  rlwinm r11, r11, 0x1a, 0x1c, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 820644A0: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 820644A4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 820644A8: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 820644AC: 41980008  blt cr6, 0x820644b4
	if ctx.cr[6].lt {
	pc = 0x820644B4; continue 'dispatch;
	}
	// 820644B0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 820644B4: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 820644B8: 39040020  addi r8, r4, 0x20
	ctx.r[8].s64 = ctx.r[4].s64 + 32;
	// 820644BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 820644C0: 516935B2  rlwimi r9, r11, 6, 0x16, 0x19
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(6) as u64) & 0x00000000000003C0) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFFC3F);
	// 820644C4: 78EBFFE6  rldicr r11, r7, 0x3f, 0x3f
	ctx.r[11].u64 = (ctx.r[7].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 820644C8: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 820644CC: 790A0020  clrldi r10, r8, 0x20
	ctx.r[10].u64 = ctx.r[8].u64 & 0x00000000FFFFFFFFu64;
	// 820644D0: 7D6B5436  srd r11, r11, r10
	if (ctx.r[10].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[11].u64) >> ((ctx.r[10].u8 & 0x3F) as u32);
	}
	// 820644D4: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 820644D8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820644DC: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 820644E0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 820644E4: 98AB2EC8  stb r5, 0x2ec8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11976 as u32), ctx.r[5].u8 ) };
	// 820644E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820644F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820644F0 size=12
    let mut pc: u32 = 0x820644F0;
    'dispatch: loop {
        match pc {
            0x820644F0 => {
    //   block [0x820644F0..0x820644FC)
	// 820644F0: 7D632214  add r11, r3, r4
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 820644F4: 886B2EC8  lbz r3, 0x2ec8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11976 as u32) ) } as u64;
	// 820644F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064500 size=76
    let mut pc: u32 = 0x82064500;
    'dispatch: loop {
        match pc {
            0x82064500 => {
    //   block [0x82064500..0x8206454C)
	// 82064500: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82064504: 7CAA0034  cntlzw r10, r5
	ctx.r[10].u64 = if ctx.r[5].u32 == 0 { 32 } else { ctx.r[5].u32.leading_zeros() as u64 };
	// 82064508: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8206450C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82064510: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82064514: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82064518: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8206451C: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82064520: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82064524: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 82064528: 7908FFE6  rldicr r8, r8, 0x3f, 0x3f
	ctx.r[8].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 8206452C: 54E7003A  rlwinm r7, r7, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82064530: 7D094C36  srd r9, r8, r9
	if (ctx.r[9].u8 & 0x40) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = (ctx.r[8].u64) >> ((ctx.r[9].u8 & 0x3F) as u32);
	}
	// 82064534: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82064538: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8206453C: E9630018  ld r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 82064540: 7D2B5B78  or r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 | ctx.r[11].u64;
	// 82064544: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 82064548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82064550 size=28
    let mut pc: u32 = 0x82064550;
    'dispatch: loop {
        match pc {
            0x82064550 => {
    //   block [0x82064550..0x8206456C)
	// 82064550: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 82064554: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82064558: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8206455C: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82064560: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82064564: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82064568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82064570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82064570 size=56
    let mut pc: u32 = 0x82064570;
    'dispatch: loop {
        match pc {
            0x82064570 => {
    //   block [0x82064570..0x820645A8)
	// 82064570: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82064574: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 82064578: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8206457C: 7D2B182E  lwzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82064580: 50A954EA  rlwimi r9, r5, 0xa, 0x13, 0x15
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(10) as u64) & 0x0000000000001C00) | (ctx.r[9].u64 & 0xFFFFFFFFFFFFE3FF);
	// 82064584: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82064588: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 8206458C: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82064590: 790AFFE6  rldicr r10, r8, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 82064594: 7D4B5C36  srd r11, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[10].u64) >> ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 82064598: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 8206459C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820645A0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 820645A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820645A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820645A8 size=20
    let mut pc: u32 = 0x820645A8;
    'dispatch: loop {
        match pc {
            0x820645A8 => {
    //   block [0x820645A8..0x820645BC)
	// 820645A8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 820645AC: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 820645B0: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 820645B4: 5563B77E  rlwinm r3, r11, 0x16, 0x1d, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 820645B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_820645C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x820645C0 size=56
    let mut pc: u32 = 0x820645C0;
    'dispatch: loop {
        match pc {
            0x820645C0 => {
    //   block [0x820645C0..0x820645F8)
	// 820645C0: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 820645C4: 39440020  addi r10, r4, 0x20
	ctx.r[10].s64 = ctx.r[4].s64 + 32;
	// 820645C8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 820645CC: 7D2B182E  lwzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 820645D0: 50A96C24  rlwimi r9, r5, 0xd, 0x10, 0x12
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(13) as u64) & 0x000000000000E000) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF1FFF);
	// 820645D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 820645D8: 7D2B192E  stwx r9, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u32) };
	// 820645DC: 794B0020  clrldi r11, r10, 0x20
	ctx.r[11].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 820645E0: 790AFFE6  rldicr r10, r8, 0x3f, 0x3f
	ctx.r[10].u64 = (ctx.r[8].u64).rotate_left(63) & 0xFFFFFFFFFFFFFFFF;
	// 820645E4: 7D4B5C36  srd r11, r10, r11
	if (ctx.r[11].u8 & 0x40) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = (ctx.r[10].u64) >> ((ctx.r[11].u8 & 0x3F) as u32);
	}
	// 820645E8: E9430018  ld r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	// 820645EC: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 820645F0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 820645F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


