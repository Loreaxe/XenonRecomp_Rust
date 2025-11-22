pub fn sub_8287B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287B858 size=52
    let mut pc: u32 = 0x8287B858;
    'dispatch: loop {
        match pc {
            0x8287B858 => {
    //   block [0x8287B858..0x8287B88C)
	// 8287B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287B860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287B864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287B868: 80840070  lwz r4, 0x70(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 8287B86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287B870: 4808DDE9  bl 0x82909658
	ctx.lr = 0x8287B874;
	sub_82909658(ctx, base);
	// 8287B874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287B878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287B87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287B880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287B884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287B888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8287B890 size=28
    let mut pc: u32 = 0x8287B890;
    'dispatch: loop {
        match pc {
            0x8287B890 => {
    //   block [0x8287B890..0x8287B8AC)
	// 8287B890: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 8287B894: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287B898: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287B89C: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287B8A0: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287B8A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287B8A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287B8B0 size=88
    let mut pc: u32 = 0x8287B8B0;
    'dispatch: loop {
        match pc {
            0x8287B8B0 => {
    //   block [0x8287B8B0..0x8287B908)
	// 8287B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287B8B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287B8BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287B8C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287B8C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287B8C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287B8CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8287B8D0: 396B33F4  addi r11, r11, 0x33f4
	ctx.r[11].s64 = ctx.r[11].s64 + 13300;
	// 8287B8D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287B8D8: 4809B291  bl 0x82916b68
	ctx.lr = 0x8287B8DC;
	sub_82916B68(ctx, base);
	// 8287B8DC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287B8E0: 4182000C  beq 0x8287b8ec
	if ctx.cr[0].eq {
	pc = 0x8287B8EC; continue 'dispatch;
	}
	// 8287B8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287B8E8: 4BA44981  bl 0x822c0268
	ctx.lr = 0x8287B8EC;
	sub_822C0268(ctx, base);
	// 8287B8EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287B8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287B8FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287B900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287B904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287B908 size=180
    let mut pc: u32 = 0x8287B908;
    'dispatch: loop {
        match pc {
            0x8287B908 => {
    //   block [0x8287B908..0x8287B9BC)
	// 8287B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287B90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287B910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287B914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287B918: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287B91C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287B920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287B924: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8287B928: 4809B471  bl 0x82916d98
	ctx.lr = 0x8287B92C;
	sub_82916D98(ctx, base);
	// 8287B92C: 807E0070  lwz r3, 0x70(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 8287B930: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8287B934: 396000D0  li r11, 0xd0
	ctx.r[11].s64 = 208;
	// 8287B938: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8287B93C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287B940: 13E958C7  vcmpequd (lvx128) v31, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287B9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287B9C0 size=148
    let mut pc: u32 = 0x8287B9C0;
    'dispatch: loop {
        match pc {
            0x8287B9C0 => {
    //   block [0x8287B9C0..0x8287BA54)
	// 8287B9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287B9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287B9C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287B9CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287B9D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287B9D4: 480DCB0D  bl 0x829584e0
	ctx.lr = 0x8287B9D8;
	sub_829584E0(ctx, base);
	// 8287B9D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287B9DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287B9E0: 387F0184  addi r3, r31, 0x184
	ctx.r[3].s64 = ctx.r[31].s64 + 388;
	// 8287B9E4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287B9E8: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8287B9EC: D01F0174  stfs f0, 0x174(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 8287B9F0: D01F0178  stfs f0, 0x178(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), tmp.u32 ) };
	// 8287B9F4: D1BF017C  stfs f13, 0x17c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), tmp.u32 ) };
	// 8287B9F8: D1BF0180  stfs f13, 0x180(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 8287B9FC: 480DC1FD  bl 0x82957bf8
	ctx.lr = 0x8287BA00;
	sub_82957BF8(ctx, base);
	// 8287BA00: 387F0194  addi r3, r31, 0x194
	ctx.r[3].s64 = ctx.r[31].s64 + 404;
	// 8287BA04: 480DC1F5  bl 0x82957bf8
	ctx.lr = 0x8287BA08;
	sub_82957BF8(ctx, base);
	// 8287BA08: 387F01A4  addi r3, r31, 0x1a4
	ctx.r[3].s64 = ctx.r[31].s64 + 420;
	// 8287BA0C: 480DC1ED  bl 0x82957bf8
	ctx.lr = 0x8287BA10;
	sub_82957BF8(ctx, base);
	// 8287BA10: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BA14: 387F01B8  addi r3, r31, 0x1b8
	ctx.r[3].s64 = ctx.r[31].s64 + 440;
	// 8287BA18: C00B89AC  lfs f0, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287BA1C: D01F01B4  stfs f0, 0x1b4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), tmp.u32 ) };
	// 8287BA20: 480D9DA1  bl 0x829557c0
	ctx.lr = 0x8287BA24;
	sub_829557C0(ctx, base);
	// 8287BA24: 387F01D4  addi r3, r31, 0x1d4
	ctx.r[3].s64 = ctx.r[31].s64 + 468;
	// 8287BA28: 480D9D99  bl 0x829557c0
	ctx.lr = 0x8287BA2C;
	sub_829557C0(ctx, base);
	// 8287BA2C: 387F01F0  addi r3, r31, 0x1f0
	ctx.r[3].s64 = ctx.r[31].s64 + 496;
	// 8287BA30: 480D9DC9  bl 0x829557f8
	ctx.lr = 0x8287BA34;
	sub_829557F8(ctx, base);
	// 8287BA34: 387F0224  addi r3, r31, 0x224
	ctx.r[3].s64 = ctx.r[31].s64 + 548;
	// 8287BA38: 480D9DC1  bl 0x829557f8
	ctx.lr = 0x8287BA3C;
	sub_829557F8(ctx, base);
	// 8287BA3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287BA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287BA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287BA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287BA4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287BA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287BA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287BA58 size=104
    let mut pc: u32 = 0x8287BA58;
    'dispatch: loop {
        match pc {
            0x8287BA58 => {
    //   block [0x8287BA58..0x8287BAC0)
	// 8287BA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287BA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287BA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287BA64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287BA68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287BA6C: 480DCCDD  bl 0x82958748
	ctx.lr = 0x8287BA70;
	sub_82958748(ctx, base);
	// 8287BA70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287BA74: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8287BA78: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8287BA7C: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 8287BA80: C18B08A8  lfs f12, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8287BA84: C1AAA1C4  lfs f13, -0x5e3c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8287BA88: C009D72C  lfs f0, -0x28d4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-10452 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287BA8C: D19F00D4  stfs f12, 0xd4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 8287BA90: D1BF00D8  stfs f13, 0xd8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 8287BA94: D1BF00DC  stfs f13, 0xdc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 8287BA98: D01F00E0  stfs f0, 0xe0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8287BA9C: D01F00E4  stfs f0, 0xe4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8287BAA0: D01F00E8  stfs f0, 0xe8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8287BAA4: 480D9CF5  bl 0x82955798
	ctx.lr = 0x8287BAA8;
	sub_82955798(ctx, base);
	// 8287BAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287BAAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287BAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287BAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287BAB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287BABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287BAC0 size=1928
    let mut pc: u32 = 0x8287BAC0;
    'dispatch: loop {
        match pc {
            0x8287BAC0 => {
    //   block [0x8287BAC0..0x8287C248)
	// 8287BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287BAC4: 4892C6A5  bl 0x831a8168
	ctx.lr = 0x8287BAC8;
	sub_831A8130(ctx, base);
	// 8287BAC8: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 8287BACC: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8287BAD0: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8287BAD4: 9421FDD0  stwu r1, -0x230(r1)
	ea = ctx.r[1].u32.wrapping_add(-560 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287BAD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287BADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287BAE0: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 8287BAE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287BAE8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BAEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287BAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8287BAF4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8287BAF8: 419A0024  beq cr6, 0x8287bb1c
	if ctx.cr[6].eq {
	pc = 0x8287BB1C; continue 'dispatch;
	}
	// 8287BAFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287BB00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287BB04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BB08: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287BB0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287BB10: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287BB14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BB18: 4082FFE8  bne 0x8287bb00
	if !ctx.cr[0].eq {
	pc = 0x8287BB00; continue 'dispatch;
	}
	// 8287BB1C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8287BB20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287BB24: 480DCCF5  bl 0x82958818
	ctx.lr = 0x8287BB28;
	sub_82958818(ctx, base);
	// 8287BB28: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BB2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BB30: 388B3D90  addi r4, r11, 0x3d90
	ctx.r[4].s64 = ctx.r[11].s64 + 15760;
	// 8287BB34: 48577ED5  bl 0x82df3a08
	ctx.lr = 0x8287BB38;
	sub_82DF3A08(ctx, base);
	// 8287BB38: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BB3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BB40: 388B3D88  addi r4, r11, 0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + 15752;
	// 8287BB44: 48577EC5  bl 0x82df3a08
	ctx.lr = 0x8287BB48;
	sub_82DF3A08(ctx, base);
	// 8287BB48: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8287BB4C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287BB50: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BB54: 4BD4E9AD  bl 0x825ca500
	ctx.lr = 0x8287BB58;
	sub_825CA500(ctx, base);
	// 8287BB58: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287BB5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BB60: 485778C9  bl 0x82df3428
	ctx.lr = 0x8287BB64;
	sub_82DF3428(ctx, base);
	// 8287BB64: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BB68: 485778C1  bl 0x82df3428
	ctx.lr = 0x8287BB6C;
	sub_82DF3428(ctx, base);
	// 8287BB6C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BB70: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BB74: 388B3D74  addi r4, r11, 0x3d74
	ctx.r[4].s64 = ctx.r[11].s64 + 15732;
	// 8287BB78: 48577E91  bl 0x82df3a08
	ctx.lr = 0x8287BB7C;
	sub_82DF3A08(ctx, base);
	// 8287BB7C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BB80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BB84: 388B3D58  addi r4, r11, 0x3d58
	ctx.r[4].s64 = ctx.r[11].s64 + 15704;
	// 8287BB88: 48577E81  bl 0x82df3a08
	ctx.lr = 0x8287BB8C;
	sub_82DF3A08(ctx, base);
	// 8287BB8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287BB90: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287BB94: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8287BB98: 38BE0170  addi r5, r30, 0x170
	ctx.r[5].s64 = ctx.r[30].s64 + 368;
	// 8287BB9C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8287BBA0: C3AB9F7C  lfs f29, -0x6084(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8287BBA4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8287BBA8: C3CA08A8  lfs f30, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8287BBAC: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8287BBB0: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8287BBB4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8287BBB8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287BBBC: 4BD276ED  bl 0x825a32a8
	ctx.lr = 0x8287BBC0;
	sub_825A32A8(ctx, base);
	// 8287BBC0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287BBC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287BBC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287BBCC: 4BD25BA5  bl 0x825a1770
	ctx.lr = 0x8287BBD0;
	sub_825A1770(ctx, base);
	// 8287BBD0: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8287BBD4: 48577855  bl 0x82df3428
	ctx.lr = 0x8287BBD8;
	sub_82DF3428(ctx, base);
	// 8287BBD8: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8287BBDC: 4BA4D0DD  bl 0x822c8cb8
	ctx.lr = 0x8287BBE0;
	sub_822C8CB8(ctx, base);
	// 8287BBE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BBE4: 48577845  bl 0x82df3428
	ctx.lr = 0x8287BBE8;
	sub_82DF3428(ctx, base);
	// 8287BBE8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BBEC: 4857783D  bl 0x82df3428
	ctx.lr = 0x8287BBF0;
	sub_82DF3428(ctx, base);
	// 8287BBF0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BBF4: 4BD4E0DD  bl 0x825c9cd0
	ctx.lr = 0x8287BBF8;
	sub_825C9CD0(ctx, base);
	// 8287BBF8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BBFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BC00: 388B3D50  addi r4, r11, 0x3d50
	ctx.r[4].s64 = ctx.r[11].s64 + 15696;
	// 8287BC04: 48577E05  bl 0x82df3a08
	ctx.lr = 0x8287BC08;
	sub_82DF3A08(ctx, base);
	// 8287BC08: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8287BC0C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BC10: 388BC8D0  addi r4, r11, -0x3730
	ctx.r[4].s64 = ctx.r[11].s64 + -14128;
	// 8287BC14: 48577DF5  bl 0x82df3a08
	ctx.lr = 0x8287BC18;
	sub_82DF3A08(ctx, base);
	// 8287BC18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8287BC1C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8287BC20: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BC24: 4BD4E8DD  bl 0x825ca500
	ctx.lr = 0x8287BC28;
	sub_825CA500(ctx, base);
	// 8287BC28: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287BC2C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BC30: 485777F9  bl 0x82df3428
	ctx.lr = 0x8287BC34;
	sub_82DF3428(ctx, base);
	// 8287BC34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BC38: 485777F1  bl 0x82df3428
	ctx.lr = 0x8287BC3C;
	sub_82DF3428(ctx, base);
	// 8287BC3C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BC40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BC44: 388B3D30  addi r4, r11, 0x3d30
	ctx.r[4].s64 = ctx.r[11].s64 + 15664;
	// 8287BC48: 48577DC1  bl 0x82df3a08
	ctx.lr = 0x8287BC4C;
	sub_82DF3A08(ctx, base);
	// 8287BC4C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BC50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BC54: 388B3D1C  addi r4, r11, 0x3d1c
	ctx.r[4].s64 = ctx.r[11].s64 + 15644;
	// 8287BC58: 48577DB1  bl 0x82df3a08
	ctx.lr = 0x8287BC5C;
	sub_82DF3A08(ctx, base);
	// 8287BC5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287BC60: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8287BC64: C06B964C  lfs f3, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8287BC68: 38BE0174  addi r5, r30, 0x174
	ctx.r[5].s64 = ctx.r[30].s64 + 372;
	// 8287BC6C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287BC70: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287BC74: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8287BC78: C04A9A8C  lfs f2, -0x6574(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8287BC7C: 4BD2762D  bl 0x825a32a8
	ctx.lr = 0x8287BC80;
	sub_825A32A8(ctx, base);
	// 8287BC80: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287BC84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287BC88: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BC8C: 4BD25AE5  bl 0x825a1770
	ctx.lr = 0x8287BC90;
	sub_825A1770(ctx, base);
	// 8287BC90: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8287BC94: 48577795  bl 0x82df3428
	ctx.lr = 0x8287BC98;
	sub_82DF3428(ctx, base);
	// 8287BC98: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8287BC9C: 4BA4D01D  bl 0x822c8cb8
	ctx.lr = 0x8287BCA0;
	sub_822C8CB8(ctx, base);
	// 8287BCA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BCA4: 48577785  bl 0x82df3428
	ctx.lr = 0x8287BCA8;
	sub_82DF3428(ctx, base);
	// 8287BCA8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BCAC: 4857777D  bl 0x82df3428
	ctx.lr = 0x8287BCB0;
	sub_82DF3428(ctx, base);
	// 8287BCB0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BCB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BCB8: 388B3D04  addi r4, r11, 0x3d04
	ctx.r[4].s64 = ctx.r[11].s64 + 15620;
	// 8287BCBC: 48577D4D  bl 0x82df3a08
	ctx.lr = 0x8287BCC0;
	sub_82DF3A08(ctx, base);
	// 8287BCC0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BCC4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BCC8: 388B3CEC  addi r4, r11, 0x3cec
	ctx.r[4].s64 = ctx.r[11].s64 + 15596;
	// 8287BCCC: 48577D3D  bl 0x82df3a08
	ctx.lr = 0x8287BCD0;
	sub_82DF3A08(ctx, base);
	// 8287BCD0: 38BE0178  addi r5, r30, 0x178
	ctx.r[5].s64 = ctx.r[30].s64 + 376;
	// 8287BCD4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287BCD8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8287BCDC: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 8287BCE0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287BCE4: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8287BCE8: 4BD275C1  bl 0x825a32a8
	ctx.lr = 0x8287BCEC;
	sub_825A32A8(ctx, base);
	// 8287BCEC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287BCF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287BCF4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BCF8: 4BD25A79  bl 0x825a1770
	ctx.lr = 0x8287BCFC;
	sub_825A1770(ctx, base);
	// 8287BCFC: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 8287BD00: 48577729  bl 0x82df3428
	ctx.lr = 0x8287BD04;
	sub_82DF3428(ctx, base);
	// 8287BD04: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 8287BD08: 4BA4CFB1  bl 0x822c8cb8
	ctx.lr = 0x8287BD0C;
	sub_822C8CB8(ctx, base);
	// 8287BD0C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BD10: 48577719  bl 0x82df3428
	ctx.lr = 0x8287BD14;
	sub_82DF3428(ctx, base);
	// 8287BD14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BD18: 48577711  bl 0x82df3428
	ctx.lr = 0x8287BD1C;
	sub_82DF3428(ctx, base);
	// 8287BD1C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BD20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BD24: 388B3CD4  addi r4, r11, 0x3cd4
	ctx.r[4].s64 = ctx.r[11].s64 + 15572;
	// 8287BD28: 48577CE1  bl 0x82df3a08
	ctx.lr = 0x8287BD2C;
	sub_82DF3A08(ctx, base);
	// 8287BD2C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BD30: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BD34: 388B3CC8  addi r4, r11, 0x3cc8
	ctx.r[4].s64 = ctx.r[11].s64 + 15560;
	// 8287BD38: 48577CD1  bl 0x82df3a08
	ctx.lr = 0x8287BD3C;
	sub_82DF3A08(ctx, base);
	// 8287BD3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8287BD40: 38BE017C  addi r5, r30, 0x17c
	ctx.r[5].s64 = ctx.r[30].s64 + 380;
	// 8287BD44: FC60F090  fmr f3, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8287BD48: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287BD4C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287BD50: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8287BD54: C3AB6218  lfs f29, 0x6218(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8287BD58: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287BD5C: 4BD2754D  bl 0x825a32a8
	ctx.lr = 0x8287BD60;
	sub_825A32A8(ctx, base);
	// 8287BD60: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287BD64: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BD68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287BD6C: 4BD25A05  bl 0x825a1770
	ctx.lr = 0x8287BD70;
	sub_825A1770(ctx, base);
	// 8287BD70: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8287BD74: 485776B5  bl 0x82df3428
	ctx.lr = 0x8287BD78;
	sub_82DF3428(ctx, base);
	// 8287BD78: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8287BD7C: 4BA4CF3D  bl 0x822c8cb8
	ctx.lr = 0x8287BD80;
	sub_822C8CB8(ctx, base);
	// 8287BD80: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BD84: 485776A5  bl 0x82df3428
	ctx.lr = 0x8287BD88;
	sub_82DF3428(ctx, base);
	// 8287BD88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BD8C: 4857769D  bl 0x82df3428
	ctx.lr = 0x8287BD90;
	sub_82DF3428(ctx, base);
	// 8287BD90: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BD94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BD98: 388B3CB0  addi r4, r11, 0x3cb0
	ctx.r[4].s64 = ctx.r[11].s64 + 15536;
	// 8287BD9C: 48577C6D  bl 0x82df3a08
	ctx.lr = 0x8287BDA0;
	sub_82DF3A08(ctx, base);
	// 8287BDA0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BDA4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BDA8: 388B3CA4  addi r4, r11, 0x3ca4
	ctx.r[4].s64 = ctx.r[11].s64 + 15524;
	// 8287BDAC: 48577C5D  bl 0x82df3a08
	ctx.lr = 0x8287BDB0;
	sub_82DF3A08(ctx, base);
	// 8287BDB0: 38BE0180  addi r5, r30, 0x180
	ctx.r[5].s64 = ctx.r[30].s64 + 384;
	// 8287BDB4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287BDB8: FC60F090  fmr f3, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8287BDBC: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8287BDC0: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287BDC4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287BDC8: 4BD274E1  bl 0x825a32a8
	ctx.lr = 0x8287BDCC;
	sub_825A32A8(ctx, base);
	// 8287BDCC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287BDD0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BDD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287BDD8: 4BD25999  bl 0x825a1770
	ctx.lr = 0x8287BDDC;
	sub_825A1770(ctx, base);
	// 8287BDDC: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 8287BDE0: 48577649  bl 0x82df3428
	ctx.lr = 0x8287BDE4;
	sub_82DF3428(ctx, base);
	// 8287BDE4: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 8287BDE8: 4BA4CED1  bl 0x822c8cb8
	ctx.lr = 0x8287BDEC;
	sub_822C8CB8(ctx, base);
	// 8287BDEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BDF0: 48577639  bl 0x82df3428
	ctx.lr = 0x8287BDF4;
	sub_82DF3428(ctx, base);
	// 8287BDF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BDF8: 48577631  bl 0x82df3428
	ctx.lr = 0x8287BDFC;
	sub_82DF3428(ctx, base);
	// 8287BDFC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BE00: 4BD4DED1  bl 0x825c9cd0
	ctx.lr = 0x8287BE04;
	sub_825C9CD0(ctx, base);
	// 8287BE04: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BE08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BE0C: 388B3C98  addi r4, r11, 0x3c98
	ctx.r[4].s64 = ctx.r[11].s64 + 15512;
	// 8287BE10: 48577BF9  bl 0x82df3a08
	ctx.lr = 0x8287BE14;
	sub_82DF3A08(ctx, base);
	// 8287BE14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287BE18: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BE1C: 388B5D2C  addi r4, r11, 0x5d2c
	ctx.r[4].s64 = ctx.r[11].s64 + 23852;
	// 8287BE20: 48577BE9  bl 0x82df3a08
	ctx.lr = 0x8287BE24;
	sub_82DF3A08(ctx, base);
	// 8287BE24: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BE28: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BE2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287BE30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287BE34: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287BE38: 419A0024  beq cr6, 0x8287be5c
	if ctx.cr[6].eq {
	pc = 0x8287BE5C; continue 'dispatch;
	}
	// 8287BE3C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287BE40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287BE44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BE48: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287BE4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287BE50: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287BE54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BE58: 4082FFE8  bne 0x8287be40
	if !ctx.cr[0].eq {
	pc = 0x8287BE40; continue 'dispatch;
	}
	// 8287BE5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287BE60: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287BE64: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BE68: 387E0184  addi r3, r30, 0x184
	ctx.r[3].s64 = ctx.r[30].s64 + 388;
	// 8287BE6C: 480DBDA5  bl 0x82957c10
	ctx.lr = 0x8287BE70;
	sub_82957C10(ctx, base);
	// 8287BE70: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BE74: 485775B5  bl 0x82df3428
	ctx.lr = 0x8287BE78;
	sub_82DF3428(ctx, base);
	// 8287BE78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BE7C: 485775AD  bl 0x82df3428
	ctx.lr = 0x8287BE80;
	sub_82DF3428(ctx, base);
	// 8287BE80: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BE84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BE88: 388B3C8C  addi r4, r11, 0x3c8c
	ctx.r[4].s64 = ctx.r[11].s64 + 15500;
	// 8287BE8C: 48577B7D  bl 0x82df3a08
	ctx.lr = 0x8287BE90;
	sub_82DF3A08(ctx, base);
	// 8287BE90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287BE94: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BE98: 388B7C94  addi r4, r11, 0x7c94
	ctx.r[4].s64 = ctx.r[11].s64 + 31892;
	// 8287BE9C: 48577B6D  bl 0x82df3a08
	ctx.lr = 0x8287BEA0;
	sub_82DF3A08(ctx, base);
	// 8287BEA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BEA4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BEA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287BEAC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287BEB0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287BEB4: 419A0024  beq cr6, 0x8287bed8
	if ctx.cr[6].eq {
	pc = 0x8287BED8; continue 'dispatch;
	}
	// 8287BEB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287BEBC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287BEC0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BEC4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287BEC8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287BECC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287BED0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BED4: 4082FFE8  bne 0x8287bebc
	if !ctx.cr[0].eq {
	pc = 0x8287BEBC; continue 'dispatch;
	}
	// 8287BED8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287BEDC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287BEE0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BEE4: 387E0194  addi r3, r30, 0x194
	ctx.r[3].s64 = ctx.r[30].s64 + 404;
	// 8287BEE8: 480DBD29  bl 0x82957c10
	ctx.lr = 0x8287BEEC;
	sub_82957C10(ctx, base);
	// 8287BEEC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BEF0: 48577539  bl 0x82df3428
	ctx.lr = 0x8287BEF4;
	sub_82DF3428(ctx, base);
	// 8287BEF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BEF8: 48577531  bl 0x82df3428
	ctx.lr = 0x8287BEFC;
	sub_82DF3428(ctx, base);
	// 8287BEFC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BF00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BF04: 388B3C80  addi r4, r11, 0x3c80
	ctx.r[4].s64 = ctx.r[11].s64 + 15488;
	// 8287BF08: 48577B01  bl 0x82df3a08
	ctx.lr = 0x8287BF0C;
	sub_82DF3A08(ctx, base);
	// 8287BF0C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BF10: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BF14: 388B3C74  addi r4, r11, 0x3c74
	ctx.r[4].s64 = ctx.r[11].s64 + 15476;
	// 8287BF18: 48577AF1  bl 0x82df3a08
	ctx.lr = 0x8287BF1C;
	sub_82DF3A08(ctx, base);
	// 8287BF1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BF20: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BF24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287BF28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287BF2C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287BF30: 419A0024  beq cr6, 0x8287bf54
	if ctx.cr[6].eq {
	pc = 0x8287BF54; continue 'dispatch;
	}
	// 8287BF34: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287BF38: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287BF3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BF40: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287BF44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287BF48: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287BF4C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287BF50: 4082FFE8  bne 0x8287bf38
	if !ctx.cr[0].eq {
	pc = 0x8287BF38; continue 'dispatch;
	}
	// 8287BF54: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287BF58: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287BF5C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287BF60: 387E01A4  addi r3, r30, 0x1a4
	ctx.r[3].s64 = ctx.r[30].s64 + 420;
	// 8287BF64: 480DBCAD  bl 0x82957c10
	ctx.lr = 0x8287BF68;
	sub_82957C10(ctx, base);
	// 8287BF68: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BF6C: 485774BD  bl 0x82df3428
	ctx.lr = 0x8287BF70;
	sub_82DF3428(ctx, base);
	// 8287BF70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BF74: 485774B5  bl 0x82df3428
	ctx.lr = 0x8287BF78;
	sub_82DF3428(ctx, base);
	// 8287BF78: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BF7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BF80: 388B3C68  addi r4, r11, 0x3c68
	ctx.r[4].s64 = ctx.r[11].s64 + 15464;
	// 8287BF84: 48577A85  bl 0x82df3a08
	ctx.lr = 0x8287BF88;
	sub_82DF3A08(ctx, base);
	// 8287BF88: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BF8C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BF90: 388B3C5C  addi r4, r11, 0x3c5c
	ctx.r[4].s64 = ctx.r[11].s64 + 15452;
	// 8287BF94: 48577A75  bl 0x82df3a08
	ctx.lr = 0x8287BF98;
	sub_82DF3A08(ctx, base);
	// 8287BF98: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8287BF9C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8287BFA0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287BFA4: 4BD4E55D  bl 0x825ca500
	ctx.lr = 0x8287BFA8;
	sub_825CA500(ctx, base);
	// 8287BFA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287BFAC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287BFB0: 48577479  bl 0x82df3428
	ctx.lr = 0x8287BFB4;
	sub_82DF3428(ctx, base);
	// 8287BFB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287BFB8: 48577471  bl 0x82df3428
	ctx.lr = 0x8287BFBC;
	sub_82DF3428(ctx, base);
	// 8287BFBC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BFC0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287BFC4: 388B3C44  addi r4, r11, 0x3c44
	ctx.r[4].s64 = ctx.r[11].s64 + 15428;
	// 8287BFC8: 48577A41  bl 0x82df3a08
	ctx.lr = 0x8287BFCC;
	sub_82DF3A08(ctx, base);
	// 8287BFCC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287BFD0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287BFD4: 388B3C2C  addi r4, r11, 0x3c2c
	ctx.r[4].s64 = ctx.r[11].s64 + 15404;
	// 8287BFD8: 48577A31  bl 0x82df3a08
	ctx.lr = 0x8287BFDC;
	sub_82DF3A08(ctx, base);
	// 8287BFDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287BFE0: 38BE01B4  addi r5, r30, 0x1b4
	ctx.r[5].s64 = ctx.r[30].s64 + 436;
	// 8287BFE4: FC40E890  fmr f2, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287BFE8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287BFEC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287BFF0: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 8287BFF4: C06B9528  lfs f3, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8287BFF8: 4BD272B1  bl 0x825a32a8
	ctx.lr = 0x8287BFFC;
	sub_825A32A8(ctx, base);
	// 8287BFFC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C000: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C004: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C008: 4BD25769  bl 0x825a1770
	ctx.lr = 0x8287C00C;
	sub_825A1770(ctx, base);
	// 8287C00C: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 8287C010: 48577419  bl 0x82df3428
	ctx.lr = 0x8287C014;
	sub_82DF3428(ctx, base);
	// 8287C014: 386101C8  addi r3, r1, 0x1c8
	ctx.r[3].s64 = ctx.r[1].s64 + 456;
	// 8287C018: 4BA4CCA1  bl 0x822c8cb8
	ctx.lr = 0x8287C01C;
	sub_822C8CB8(ctx, base);
	// 8287C01C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C020: 48577409  bl 0x82df3428
	ctx.lr = 0x8287C024;
	sub_82DF3428(ctx, base);
	// 8287C024: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C028: 48577401  bl 0x82df3428
	ctx.lr = 0x8287C02C;
	sub_82DF3428(ctx, base);
	// 8287C02C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C030: 4BD4DCA1  bl 0x825c9cd0
	ctx.lr = 0x8287C034;
	sub_825C9CD0(ctx, base);
	// 8287C034: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C03C: 388B3C10  addi r4, r11, 0x3c10
	ctx.r[4].s64 = ctx.r[11].s64 + 15376;
	// 8287C040: 485779C9  bl 0x82df3a08
	ctx.lr = 0x8287C044;
	sub_82DF3A08(ctx, base);
	// 8287C044: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C048: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C04C: 388BA2C4  addi r4, r11, -0x5d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -23868;
	// 8287C050: 485779B9  bl 0x82df3a08
	ctx.lr = 0x8287C054;
	sub_82DF3A08(ctx, base);
	// 8287C054: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C058: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C05C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287C060: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287C064: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287C068: 419A0024  beq cr6, 0x8287c08c
	if ctx.cr[6].eq {
	pc = 0x8287C08C; continue 'dispatch;
	}
	// 8287C06C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287C070: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C074: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C078: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C07C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C080: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C084: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C088: 4082FFE8  bne 0x8287c070
	if !ctx.cr[0].eq {
	pc = 0x8287C070; continue 'dispatch;
	}
	// 8287C08C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287C090: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287C094: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C098: 387E01B8  addi r3, r30, 0x1b8
	ctx.r[3].s64 = ctx.r[30].s64 + 440;
	// 8287C09C: 480D97DD  bl 0x82955878
	ctx.lr = 0x8287C0A0;
	sub_82955878(ctx, base);
	// 8287C0A0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C0A4: 48577385  bl 0x82df3428
	ctx.lr = 0x8287C0A8;
	sub_82DF3428(ctx, base);
	// 8287C0A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C0AC: 4857737D  bl 0x82df3428
	ctx.lr = 0x8287C0B0;
	sub_82DF3428(ctx, base);
	// 8287C0B0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C0B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C0B8: 388B3BF4  addi r4, r11, 0x3bf4
	ctx.r[4].s64 = ctx.r[11].s64 + 15348;
	// 8287C0BC: 4857794D  bl 0x82df3a08
	ctx.lr = 0x8287C0C0;
	sub_82DF3A08(ctx, base);
	// 8287C0C0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C0C4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C0C8: 388B3B6C  addi r4, r11, 0x3b6c
	ctx.r[4].s64 = ctx.r[11].s64 + 15212;
	// 8287C0CC: 4857793D  bl 0x82df3a08
	ctx.lr = 0x8287C0D0;
	sub_82DF3A08(ctx, base);
	// 8287C0D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C0D4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C0D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287C0DC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287C0E0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287C0E4: 419A0024  beq cr6, 0x8287c108
	if ctx.cr[6].eq {
	pc = 0x8287C108; continue 'dispatch;
	}
	// 8287C0E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287C0EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C0F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C0F4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C0F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C0FC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C100: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C104: 4082FFE8  bne 0x8287c0ec
	if !ctx.cr[0].eq {
	pc = 0x8287C0EC; continue 'dispatch;
	}
	// 8287C108: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287C10C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287C110: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C114: 387E01D4  addi r3, r30, 0x1d4
	ctx.r[3].s64 = ctx.r[30].s64 + 468;
	// 8287C118: 480D9761  bl 0x82955878
	ctx.lr = 0x8287C11C;
	sub_82955878(ctx, base);
	// 8287C11C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C120: 48577309  bl 0x82df3428
	ctx.lr = 0x8287C124;
	sub_82DF3428(ctx, base);
	// 8287C124: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C128: 48577301  bl 0x82df3428
	ctx.lr = 0x8287C12C;
	sub_82DF3428(ctx, base);
	// 8287C12C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C134: 388B3BD8  addi r4, r11, 0x3bd8
	ctx.r[4].s64 = ctx.r[11].s64 + 15320;
	// 8287C138: 485778D1  bl 0x82df3a08
	ctx.lr = 0x8287C13C;
	sub_82DF3A08(ctx, base);
	// 8287C13C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C140: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C144: 388B3B88  addi r4, r11, 0x3b88
	ctx.r[4].s64 = ctx.r[11].s64 + 15240;
	// 8287C148: 485778C1  bl 0x82df3a08
	ctx.lr = 0x8287C14C;
	sub_82DF3A08(ctx, base);
	// 8287C14C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C150: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C154: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287C158: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287C15C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287C160: 419A0024  beq cr6, 0x8287c184
	if ctx.cr[6].eq {
	pc = 0x8287C184; continue 'dispatch;
	}
	// 8287C164: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287C168: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C16C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C170: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C174: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C178: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C17C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C180: 4082FFE8  bne 0x8287c168
	if !ctx.cr[0].eq {
	pc = 0x8287C168; continue 'dispatch;
	}
	// 8287C184: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287C188: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287C18C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C190: 387E01F0  addi r3, r30, 0x1f0
	ctx.r[3].s64 = ctx.r[30].s64 + 496;
	// 8287C194: 480D9A55  bl 0x82955be8
	ctx.lr = 0x8287C198;
	sub_82955BE8(ctx, base);
	// 8287C198: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C19C: 4857728D  bl 0x82df3428
	ctx.lr = 0x8287C1A0;
	sub_82DF3428(ctx, base);
	// 8287C1A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C1A4: 48577285  bl 0x82df3428
	ctx.lr = 0x8287C1A8;
	sub_82DF3428(ctx, base);
	// 8287C1A8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C1AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C1B0: 388B3BBC  addi r4, r11, 0x3bbc
	ctx.r[4].s64 = ctx.r[11].s64 + 15292;
	// 8287C1B4: 48577855  bl 0x82df3a08
	ctx.lr = 0x8287C1B8;
	sub_82DF3A08(ctx, base);
	// 8287C1B8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C1BC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C1C0: 388B3B78  addi r4, r11, 0x3b78
	ctx.r[4].s64 = ctx.r[11].s64 + 15224;
	// 8287C1C4: 48577845  bl 0x82df3a08
	ctx.lr = 0x8287C1C8;
	sub_82DF3A08(ctx, base);
	// 8287C1C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C1CC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C1D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287C1D4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287C1D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287C1DC: 419A0024  beq cr6, 0x8287c200
	if ctx.cr[6].eq {
	pc = 0x8287C200; continue 'dispatch;
	}
	// 8287C1E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287C1E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C1E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C1EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C1F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C1F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C1F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C1FC: 4082FFE8  bne 0x8287c1e4
	if !ctx.cr[0].eq {
	pc = 0x8287C1E4; continue 'dispatch;
	}
	// 8287C200: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8287C204: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 8287C208: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C20C: 387E0224  addi r3, r30, 0x224
	ctx.r[3].s64 = ctx.r[30].s64 + 548;
	// 8287C210: 480D99D9  bl 0x82955be8
	ctx.lr = 0x8287C214;
	sub_82955BE8(ctx, base);
	// 8287C214: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8287C218: 48577211  bl 0x82df3428
	ctx.lr = 0x8287C21C;
	sub_82DF3428(ctx, base);
	// 8287C21C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C220: 48577209  bl 0x82df3428
	ctx.lr = 0x8287C224;
	sub_82DF3428(ctx, base);
	// 8287C224: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287C22C: 419A0008  beq cr6, 0x8287c234
	if ctx.cr[6].eq {
	pc = 0x8287C234; continue 'dispatch;
	}
	// 8287C230: 4BA44661  bl 0x822c0890
	ctx.lr = 0x8287C234;
	sub_822C0890(ctx, base);
	// 8287C234: 38210230  addi r1, r1, 0x230
	ctx.r[1].s64 = ctx.r[1].s64 + 560;
	// 8287C238: CBA1FFC0  lfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8287C23C: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8287C240: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8287C244: 4892BF74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287C248 size=776
    let mut pc: u32 = 0x8287C248;
    'dispatch: loop {
        match pc {
            0x8287C248 => {
    //   block [0x8287C248..0x8287C550)
	// 8287C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287C24C: 4892BF1D  bl 0x831a8168
	ctx.lr = 0x8287C250;
	sub_831A8130(ctx, base);
	// 8287C250: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287C254: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8287C258: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8287C25C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287C260: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8287C264: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8287C268: 419A02E0  beq cr6, 0x8287c548
	if ctx.cr[6].eq {
	pc = 0x8287C548; continue 'dispatch;
	}
	// 8287C26C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C274: 388B12B0  addi r4, r11, 0x12b0
	ctx.r[4].s64 = ctx.r[11].s64 + 4784;
	// 8287C278: 48577791  bl 0x82df3a08
	ctx.lr = 0x8287C27C;
	sub_82DF3A08(ctx, base);
	// 8287C27C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8287C280: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C284: 388BBE80  addi r4, r11, -0x4180
	ctx.r[4].s64 = ctx.r[11].s64 + -16768;
	// 8287C288: 48577781  bl 0x82df3a08
	ctx.lr = 0x8287C28C;
	sub_82DF3A08(ctx, base);
	// 8287C28C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8287C290: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8287C294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287C298: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8287C29C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C2A0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8287C2A4: 4BD318ED  bl 0x825adb90
	ctx.lr = 0x8287C2A8;
	sub_825ADB90(ctx, base);
	// 8287C2A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C2AC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C2B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C2B4: 4BD25B05  bl 0x825a1db8
	ctx.lr = 0x8287C2B8;
	sub_825A1DB8(ctx, base);
	// 8287C2B8: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8287C2BC: 4857716D  bl 0x82df3428
	ctx.lr = 0x8287C2C0;
	sub_82DF3428(ctx, base);
	// 8287C2C0: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8287C2C4: 4BA4C9F5  bl 0x822c8cb8
	ctx.lr = 0x8287C2C8;
	sub_822C8CB8(ctx, base);
	// 8287C2C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C2CC: 4857715D  bl 0x82df3428
	ctx.lr = 0x8287C2D0;
	sub_82DF3428(ctx, base);
	// 8287C2D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C2D4: 48577155  bl 0x82df3428
	ctx.lr = 0x8287C2D8;
	sub_82DF3428(ctx, base);
	// 8287C2D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8287C2DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C2E0: 388BBE9C  addi r4, r11, -0x4164
	ctx.r[4].s64 = ctx.r[11].s64 + -16740;
	// 8287C2E4: 48577725  bl 0x82df3a08
	ctx.lr = 0x8287C2E8;
	sub_82DF3A08(ctx, base);
	// 8287C2E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8287C2EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C2F0: 388B4858  addi r4, r11, 0x4858
	ctx.r[4].s64 = ctx.r[11].s64 + 18520;
	// 8287C2F4: 48577715  bl 0x82df3a08
	ctx.lr = 0x8287C2F8;
	sub_82DF3A08(ctx, base);
	// 8287C2F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8287C2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8287C300: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287C304: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 8287C308: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C30C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8287C310: 4BD31881  bl 0x825adb90
	ctx.lr = 0x8287C314;
	sub_825ADB90(ctx, base);
	// 8287C314: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C318: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C31C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C320: 4BD25A99  bl 0x825a1db8
	ctx.lr = 0x8287C324;
	sub_825A1DB8(ctx, base);
	// 8287C324: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8287C328: 48577101  bl 0x82df3428
	ctx.lr = 0x8287C32C;
	sub_82DF3428(ctx, base);
	// 8287C32C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8287C330: 4BA4C989  bl 0x822c8cb8
	ctx.lr = 0x8287C334;
	sub_822C8CB8(ctx, base);
	// 8287C334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C338: 485770F1  bl 0x82df3428
	ctx.lr = 0x8287C33C;
	sub_82DF3428(ctx, base);
	// 8287C33C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C340: 485770E9  bl 0x82df3428
	ctx.lr = 0x8287C344;
	sub_82DF3428(ctx, base);
	// 8287C344: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C348: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287C34C: 388B3DE0  addi r4, r11, 0x3de0
	ctx.r[4].s64 = ctx.r[11].s64 + 15840;
	// 8287C350: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 8287C354: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8287C358: 4BA44081  bl 0x822c03d8
	ctx.lr = 0x8287C35C;
	sub_822C03D8(ctx, base);
	// 8287C35C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8287C360: 41820034  beq 0x8287c394
	if ctx.cr[0].eq {
	pc = 0x8287C394; continue 'dispatch;
	}
	// 8287C364: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C36C: 388B3DD4  addi r4, r11, 0x3dd4
	ctx.r[4].s64 = ctx.r[11].s64 + 15828;
	// 8287C370: 48577699  bl 0x82df3a08
	ctx.lr = 0x8287C374;
	sub_82DF3A08(ctx, base);
	// 8287C374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287C378: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 8287C37C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287C384: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8287C388: 4BD30089  bl 0x825ac410
	ctx.lr = 0x8287C38C;
	sub_825AC410(ctx, base);
	// 8287C38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287C390: 48000008  b 0x8287c398
	pc = 0x8287C398; continue 'dispatch;
	// 8287C394: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287C398: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8287C39C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287C3A0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8287C3A4: 4BC77015  bl 0x824f33b8
	ctx.lr = 0x8287C3A8;
	sub_824F33B8(ctx, base);
	// 8287C3A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287C3AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287C3B0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8287C3B4: 4BA43C4D  bl 0x822c0000
	ctx.lr = 0x8287C3B8;
	sub_822C0000(ctx, base);
	// 8287C3B8: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287C3BC: 4182000C  beq 0x8287c3c8
	if ctx.cr[0].eq {
	pc = 0x8287C3C8; continue 'dispatch;
	}
	// 8287C3C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C3C4: 48577065  bl 0x82df3428
	ctx.lr = 0x8287C3C8;
	sub_82DF3428(ctx, base);
	// 8287C3C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C3CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C3D0: 388B3DCC  addi r4, r11, 0x3dcc
	ctx.r[4].s64 = ctx.r[11].s64 + 15820;
	// 8287C3D4: 48577635  bl 0x82df3a08
	ctx.lr = 0x8287C3D8;
	sub_82DF3A08(ctx, base);
	// 8287C3D8: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8287C3DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8287C3E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287C3E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287C3E8: 4BD2F9D9  bl 0x825abdc0
	ctx.lr = 0x8287C3EC;
	sub_825ABDC0(ctx, base);
	// 8287C3EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C3F0: 48577039  bl 0x82df3428
	ctx.lr = 0x8287C3F4;
	sub_82DF3428(ctx, base);
	// 8287C3F4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C3F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C3FC: 388B3DC4  addi r4, r11, 0x3dc4
	ctx.r[4].s64 = ctx.r[11].s64 + 15812;
	// 8287C400: 48577609  bl 0x82df3a08
	ctx.lr = 0x8287C404;
	sub_82DF3A08(ctx, base);
	// 8287C404: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8287C408: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287C40C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287C410: 4BD2F9B1  bl 0x825abdc0
	ctx.lr = 0x8287C414;
	sub_825ABDC0(ctx, base);
	// 8287C414: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C418: 48577011  bl 0x82df3428
	ctx.lr = 0x8287C41C;
	sub_82DF3428(ctx, base);
	// 8287C41C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C424: 388B3DB4  addi r4, r11, 0x3db4
	ctx.r[4].s64 = ctx.r[11].s64 + 15796;
	// 8287C428: 485775E1  bl 0x82df3a08
	ctx.lr = 0x8287C42C;
	sub_82DF3A08(ctx, base);
	// 8287C42C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8287C430: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287C434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287C438: 4BD2F989  bl 0x825abdc0
	ctx.lr = 0x8287C43C;
	sub_825ABDC0(ctx, base);
	// 8287C43C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C440: 48576FE9  bl 0x82df3428
	ctx.lr = 0x8287C444;
	sub_82DF3428(ctx, base);
	// 8287C444: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C448: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C44C: 388B3DA4  addi r4, r11, 0x3da4
	ctx.r[4].s64 = ctx.r[11].s64 + 15780;
	// 8287C450: 485775B9  bl 0x82df3a08
	ctx.lr = 0x8287C454;
	sub_82DF3A08(ctx, base);
	// 8287C454: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8287C458: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8287C45C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287C460: 4BD2F961  bl 0x825abdc0
	ctx.lr = 0x8287C464;
	sub_825ABDC0(ctx, base);
	// 8287C464: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C468: 48576FC1  bl 0x82df3428
	ctx.lr = 0x8287C46C;
	sub_82DF3428(ctx, base);
	// 8287C46C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C470: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C474: 388B3D98  addi r4, r11, 0x3d98
	ctx.r[4].s64 = ctx.r[11].s64 + 15768;
	// 8287C478: 48577591  bl 0x82df3a08
	ctx.lr = 0x8287C47C;
	sub_82DF3A08(ctx, base);
	// 8287C47C: 83A1005C  lwz r29, 0x5c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287C480: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8287C484: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8287C488: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 8287C48C: 419A0024  beq cr6, 0x8287c4b0
	if ctx.cr[6].eq {
	pc = 0x8287C4B0; continue 'dispatch;
	}
	// 8287C490: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 8287C494: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C498: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C49C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C4A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C4A4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C4A8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C4AC: 4082FFE8  bne 0x8287c494
	if !ctx.cr[0].eq {
	pc = 0x8287C494; continue 'dispatch;
	}
	// 8287C4B0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8287C4B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C4B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C4BC: 4BD25C55  bl 0x825a2110
	ctx.lr = 0x8287C4C0;
	sub_825A2110(ctx, base);
	// 8287C4C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C4C4: 48576F65  bl 0x82df3428
	ctx.lr = 0x8287C4C8;
	sub_82DF3428(ctx, base);
	// 8287C4C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C4CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C4D0: 388B9D24  addi r4, r11, -0x62dc
	ctx.r[4].s64 = ctx.r[11].s64 + -25308;
	// 8287C4D4: 48577535  bl 0x82df3a08
	ctx.lr = 0x8287C4D8;
	sub_82DF3A08(ctx, base);
	// 8287C4D8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C4DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C4E0: 388B9B58  addi r4, r11, -0x64a8
	ctx.r[4].s64 = ctx.r[11].s64 + -25768;
	// 8287C4E4: 48577525  bl 0x82df3a08
	ctx.lr = 0x8287C4E8;
	sub_82DF3A08(ctx, base);
	// 8287C4E8: 3CE00001  lis r7, 1
	ctx.r[7].s64 = 65536;
	// 8287C4EC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8287C4F0: 60E7869F  ori r7, r7, 0x869f
	ctx.r[7].u64 = ctx.r[7].u64 | 34463;
	// 8287C4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287C4F8: 38BE000C  addi r5, r30, 0xc
	ctx.r[5].s64 = ctx.r[30].s64 + 12;
	// 8287C4FC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C500: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8287C504: 4BD3168D  bl 0x825adb90
	ctx.lr = 0x8287C508;
	sub_825ADB90(ctx, base);
	// 8287C508: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C50C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C510: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C514: 4BD258A5  bl 0x825a1db8
	ctx.lr = 0x8287C518;
	sub_825A1DB8(ctx, base);
	// 8287C518: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8287C51C: 48576F0D  bl 0x82df3428
	ctx.lr = 0x8287C520;
	sub_82DF3428(ctx, base);
	// 8287C520: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8287C524: 4BA4C795  bl 0x822c8cb8
	ctx.lr = 0x8287C528;
	sub_822C8CB8(ctx, base);
	// 8287C528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C52C: 48576EFD  bl 0x82df3428
	ctx.lr = 0x8287C530;
	sub_82DF3428(ctx, base);
	// 8287C530: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C534: 48576EF5  bl 0x82df3428
	ctx.lr = 0x8287C538;
	sub_82DF3428(ctx, base);
	// 8287C538: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8287C53C: 419A000C  beq cr6, 0x8287c548
	if ctx.cr[6].eq {
	pc = 0x8287C548; continue 'dispatch;
	}
	// 8287C540: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287C544: 4BA4434D  bl 0x822c0890
	ctx.lr = 0x8287C548;
	sub_822C0890(ctx, base);
	// 8287C548: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8287C54C: 4892BC6C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287C550 size=1172
    let mut pc: u32 = 0x8287C550;
    'dispatch: loop {
        match pc {
            0x8287C550 => {
    //   block [0x8287C550..0x8287C9E4)
	// 8287C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287C554: 4892BC15  bl 0x831a8168
	ctx.lr = 0x8287C558;
	sub_831A8130(ctx, base);
	// 8287C558: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 8287C55C: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8287C560: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8287C564: 9421FDD0  stwu r1, -0x230(r1)
	ea = ctx.r[1].u32.wrapping_add(-560 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287C568: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287C56C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287C570: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 8287C574: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287C578: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C57C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287C580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8287C584: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8287C588: 419A0024  beq cr6, 0x8287c5ac
	if ctx.cr[6].eq {
	pc = 0x8287C5AC; continue 'dispatch;
	}
	// 8287C58C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287C590: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C594: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C598: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C59C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C5A0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C5A4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C5A8: 4082FFE8  bne 0x8287c590
	if !ctx.cr[0].eq {
	pc = 0x8287C590; continue 'dispatch;
	}
	// 8287C5AC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287C5B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287C5B4: 480DCFCD  bl 0x82959580
	ctx.lr = 0x8287C5B8;
	sub_82959580(ctx, base);
	// 8287C5B8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C5BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C5C0: 388B3D50  addi r4, r11, 0x3d50
	ctx.r[4].s64 = ctx.r[11].s64 + 15696;
	// 8287C5C4: 48577445  bl 0x82df3a08
	ctx.lr = 0x8287C5C8;
	sub_82DF3A08(ctx, base);
	// 8287C5C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8287C5CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C5D0: 388BC8D0  addi r4, r11, -0x3730
	ctx.r[4].s64 = ctx.r[11].s64 + -14128;
	// 8287C5D4: 48577435  bl 0x82df3a08
	ctx.lr = 0x8287C5D8;
	sub_82DF3A08(ctx, base);
	// 8287C5D8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8287C5DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C5E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C5E4: 4BD4DF1D  bl 0x825ca500
	ctx.lr = 0x8287C5E8;
	sub_825CA500(ctx, base);
	// 8287C5E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287C5EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C5F0: 48576E39  bl 0x82df3428
	ctx.lr = 0x8287C5F4;
	sub_82DF3428(ctx, base);
	// 8287C5F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C5F8: 48576E31  bl 0x82df3428
	ctx.lr = 0x8287C5FC;
	sub_82DF3428(ctx, base);
	// 8287C5FC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C600: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C604: 388B3F04  addi r4, r11, 0x3f04
	ctx.r[4].s64 = ctx.r[11].s64 + 16132;
	// 8287C608: 48577401  bl 0x82df3a08
	ctx.lr = 0x8287C60C;
	sub_82DF3A08(ctx, base);
	// 8287C60C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C610: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C614: 388B3EF0  addi r4, r11, 0x3ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 16112;
	// 8287C618: 485773F1  bl 0x82df3a08
	ctx.lr = 0x8287C61C;
	sub_82DF3A08(ctx, base);
	// 8287C61C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287C620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8287C624: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8287C628: 38BE00D4  addi r5, r30, 0xd4
	ctx.r[5].s64 = ctx.r[30].s64 + 212;
	// 8287C62C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287C630: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8287C634: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8287C638: C3AA6218  lfs f29, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8287C63C: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8287C640: C3C908A4  lfs f30, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8287C644: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287C648: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8287C64C: 4BD26C5D  bl 0x825a32a8
	ctx.lr = 0x8287C650;
	sub_825A32A8(ctx, base);
	// 8287C650: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C654: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8287C658: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C65C: 4BD25115  bl 0x825a1770
	ctx.lr = 0x8287C660;
	sub_825A1770(ctx, base);
	// 8287C660: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 8287C664: 48576DC5  bl 0x82df3428
	ctx.lr = 0x8287C668;
	sub_82DF3428(ctx, base);
	// 8287C668: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 8287C66C: 4BA4C64D  bl 0x822c8cb8
	ctx.lr = 0x8287C670;
	sub_822C8CB8(ctx, base);
	// 8287C670: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C674: 48576DB5  bl 0x82df3428
	ctx.lr = 0x8287C678;
	sub_82DF3428(ctx, base);
	// 8287C678: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C67C: 48576DAD  bl 0x82df3428
	ctx.lr = 0x8287C680;
	sub_82DF3428(ctx, base);
	// 8287C680: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C684: 4BD4D64D  bl 0x825c9cd0
	ctx.lr = 0x8287C688;
	sub_825C9CD0(ctx, base);
	// 8287C688: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287C68C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C690: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 8287C694: 48577375  bl 0x82df3a08
	ctx.lr = 0x8287C698;
	sub_82DF3A08(ctx, base);
	// 8287C698: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287C69C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C6A0: 388B08E0  addi r4, r11, 0x8e0
	ctx.r[4].s64 = ctx.r[11].s64 + 2272;
	// 8287C6A4: 48577365  bl 0x82df3a08
	ctx.lr = 0x8287C6A8;
	sub_82DF3A08(ctx, base);
	// 8287C6A8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8287C6AC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287C6B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C6B4: 4BD4DE4D  bl 0x825ca500
	ctx.lr = 0x8287C6B8;
	sub_825CA500(ctx, base);
	// 8287C6B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287C6BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C6C0: 48576D69  bl 0x82df3428
	ctx.lr = 0x8287C6C4;
	sub_82DF3428(ctx, base);
	// 8287C6C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C6C8: 48576D61  bl 0x82df3428
	ctx.lr = 0x8287C6CC;
	sub_82DF3428(ctx, base);
	// 8287C6CC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C6D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C6D4: 388B1E0C  addi r4, r11, 0x1e0c
	ctx.r[4].s64 = ctx.r[11].s64 + 7692;
	// 8287C6D8: 48577331  bl 0x82df3a08
	ctx.lr = 0x8287C6DC;
	sub_82DF3A08(ctx, base);
	// 8287C6DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C6E0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C6E4: 388B1DFC  addi r4, r11, 0x1dfc
	ctx.r[4].s64 = ctx.r[11].s64 + 7676;
	// 8287C6E8: 48577321  bl 0x82df3a08
	ctx.lr = 0x8287C6EC;
	sub_82DF3A08(ctx, base);
	// 8287C6EC: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8287C6F0: 38BE00D8  addi r5, r30, 0xd8
	ctx.r[5].s64 = ctx.r[30].s64 + 216;
	// 8287C6F4: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287C6F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C6FC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8287C700: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 8287C704: 4BD26BA5  bl 0x825a32a8
	ctx.lr = 0x8287C708;
	sub_825A32A8(ctx, base);
	// 8287C708: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C70C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C710: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C714: 4BD2505D  bl 0x825a1770
	ctx.lr = 0x8287C718;
	sub_825A1770(ctx, base);
	// 8287C718: 38610128  addi r3, r1, 0x128
	ctx.r[3].s64 = ctx.r[1].s64 + 296;
	// 8287C71C: 48576D0D  bl 0x82df3428
	ctx.lr = 0x8287C720;
	sub_82DF3428(ctx, base);
	// 8287C720: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 8287C724: 4BA4C595  bl 0x822c8cb8
	ctx.lr = 0x8287C728;
	sub_822C8CB8(ctx, base);
	// 8287C728: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C72C: 48576CFD  bl 0x82df3428
	ctx.lr = 0x8287C730;
	sub_82DF3428(ctx, base);
	// 8287C730: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C734: 48576CF5  bl 0x82df3428
	ctx.lr = 0x8287C738;
	sub_82DF3428(ctx, base);
	// 8287C738: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C73C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C740: 388B1DE4  addi r4, r11, 0x1de4
	ctx.r[4].s64 = ctx.r[11].s64 + 7652;
	// 8287C744: 485772C5  bl 0x82df3a08
	ctx.lr = 0x8287C748;
	sub_82DF3A08(ctx, base);
	// 8287C748: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C74C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C750: 388B1DD4  addi r4, r11, 0x1dd4
	ctx.r[4].s64 = ctx.r[11].s64 + 7636;
	// 8287C754: 485772B5  bl 0x82df3a08
	ctx.lr = 0x8287C758;
	sub_82DF3A08(ctx, base);
	// 8287C758: 38BE00DC  addi r5, r30, 0xdc
	ctx.r[5].s64 = ctx.r[30].s64 + 220;
	// 8287C75C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C760: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8287C764: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 8287C768: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8287C76C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287C770: 4BD26B39  bl 0x825a32a8
	ctx.lr = 0x8287C774;
	sub_825A32A8(ctx, base);
	// 8287C774: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C778: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C77C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C780: 4BD24FF1  bl 0x825a1770
	ctx.lr = 0x8287C784;
	sub_825A1770(ctx, base);
	// 8287C784: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 8287C788: 48576CA1  bl 0x82df3428
	ctx.lr = 0x8287C78C;
	sub_82DF3428(ctx, base);
	// 8287C78C: 38610188  addi r3, r1, 0x188
	ctx.r[3].s64 = ctx.r[1].s64 + 392;
	// 8287C790: 4BA4C529  bl 0x822c8cb8
	ctx.lr = 0x8287C794;
	sub_822C8CB8(ctx, base);
	// 8287C794: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C798: 48576C91  bl 0x82df3428
	ctx.lr = 0x8287C79C;
	sub_82DF3428(ctx, base);
	// 8287C79C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C7A0: 48576C89  bl 0x82df3428
	ctx.lr = 0x8287C7A4;
	sub_82DF3428(ctx, base);
	// 8287C7A4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C7A8: 4BD4D529  bl 0x825c9cd0
	ctx.lr = 0x8287C7AC;
	sub_825C9CD0(ctx, base);
	// 8287C7AC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C7B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C7B4: 388B3C68  addi r4, r11, 0x3c68
	ctx.r[4].s64 = ctx.r[11].s64 + 15464;
	// 8287C7B8: 48577251  bl 0x82df3a08
	ctx.lr = 0x8287C7BC;
	sub_82DF3A08(ctx, base);
	// 8287C7BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C7C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C7C4: 388B3C5C  addi r4, r11, 0x3c5c
	ctx.r[4].s64 = ctx.r[11].s64 + 15452;
	// 8287C7C8: 48577241  bl 0x82df3a08
	ctx.lr = 0x8287C7CC;
	sub_82DF3A08(ctx, base);
	// 8287C7CC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8287C7D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8287C7D4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C7D8: 4BD4DD29  bl 0x825ca500
	ctx.lr = 0x8287C7DC;
	sub_825CA500(ctx, base);
	// 8287C7DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287C7E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C7E4: 48576C45  bl 0x82df3428
	ctx.lr = 0x8287C7E8;
	sub_82DF3428(ctx, base);
	// 8287C7E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C7EC: 48576C3D  bl 0x82df3428
	ctx.lr = 0x8287C7F0;
	sub_82DF3428(ctx, base);
	// 8287C7F0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C7F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C7F8: 388B3ED4  addi r4, r11, 0x3ed4
	ctx.r[4].s64 = ctx.r[11].s64 + 16084;
	// 8287C7FC: 4857720D  bl 0x82df3a08
	ctx.lr = 0x8287C800;
	sub_82DF3A08(ctx, base);
	// 8287C800: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C804: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C808: 388B3EC4  addi r4, r11, 0x3ec4
	ctx.r[4].s64 = ctx.r[11].s64 + 16068;
	// 8287C80C: 485771FD  bl 0x82df3a08
	ctx.lr = 0x8287C810;
	sub_82DF3A08(ctx, base);
	// 8287C810: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8287C814: 38BE00E0  addi r5, r30, 0xe0
	ctx.r[5].s64 = ctx.r[30].s64 + 224;
	// 8287C818: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8287C81C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C820: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8287C824: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8287C828: C3AB9A8C  lfs f29, -0x6574(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8287C82C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287C830: 4BD26A79  bl 0x825a32a8
	ctx.lr = 0x8287C834;
	sub_825A32A8(ctx, base);
	// 8287C834: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C838: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C83C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C840: 4BD24F31  bl 0x825a1770
	ctx.lr = 0x8287C844;
	sub_825A1770(ctx, base);
	// 8287C844: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 8287C848: 48576BE1  bl 0x82df3428
	ctx.lr = 0x8287C84C;
	sub_82DF3428(ctx, base);
	// 8287C84C: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8287C850: 4BA4C469  bl 0x822c8cb8
	ctx.lr = 0x8287C854;
	sub_822C8CB8(ctx, base);
	// 8287C854: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C858: 48576BD1  bl 0x82df3428
	ctx.lr = 0x8287C85C;
	sub_82DF3428(ctx, base);
	// 8287C85C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C860: 48576BC9  bl 0x82df3428
	ctx.lr = 0x8287C864;
	sub_82DF3428(ctx, base);
	// 8287C864: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C868: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C86C: 388B3EA8  addi r4, r11, 0x3ea8
	ctx.r[4].s64 = ctx.r[11].s64 + 16040;
	// 8287C870: 48577199  bl 0x82df3a08
	ctx.lr = 0x8287C874;
	sub_82DF3A08(ctx, base);
	// 8287C874: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C878: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C87C: 388B3E94  addi r4, r11, 0x3e94
	ctx.r[4].s64 = ctx.r[11].s64 + 16020;
	// 8287C880: 48577189  bl 0x82df3a08
	ctx.lr = 0x8287C884;
	sub_82DF3A08(ctx, base);
	// 8287C884: 38BE00E4  addi r5, r30, 0xe4
	ctx.r[5].s64 = ctx.r[30].s64 + 228;
	// 8287C888: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C88C: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8287C890: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 8287C894: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287C898: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8287C89C: 4BD26A0D  bl 0x825a32a8
	ctx.lr = 0x8287C8A0;
	sub_825A32A8(ctx, base);
	// 8287C8A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C8A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C8A8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C8AC: 4BD24EC5  bl 0x825a1770
	ctx.lr = 0x8287C8B0;
	sub_825A1770(ctx, base);
	// 8287C8B0: 38610168  addi r3, r1, 0x168
	ctx.r[3].s64 = ctx.r[1].s64 + 360;
	// 8287C8B4: 48576B75  bl 0x82df3428
	ctx.lr = 0x8287C8B8;
	sub_82DF3428(ctx, base);
	// 8287C8B8: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 8287C8BC: 4BA4C3FD  bl 0x822c8cb8
	ctx.lr = 0x8287C8C0;
	sub_822C8CB8(ctx, base);
	// 8287C8C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C8C4: 48576B65  bl 0x82df3428
	ctx.lr = 0x8287C8C8;
	sub_82DF3428(ctx, base);
	// 8287C8C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C8CC: 48576B5D  bl 0x82df3428
	ctx.lr = 0x8287C8D0;
	sub_82DF3428(ctx, base);
	// 8287C8D0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C8D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C8D8: 388B3E78  addi r4, r11, 0x3e78
	ctx.r[4].s64 = ctx.r[11].s64 + 15992;
	// 8287C8DC: 4857712D  bl 0x82df3a08
	ctx.lr = 0x8287C8E0;
	sub_82DF3A08(ctx, base);
	// 8287C8E0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C8E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C8E8: 388B3E64  addi r4, r11, 0x3e64
	ctx.r[4].s64 = ctx.r[11].s64 + 15972;
	// 8287C8EC: 4857711D  bl 0x82df3a08
	ctx.lr = 0x8287C8F0;
	sub_82DF3A08(ctx, base);
	// 8287C8F0: 38BE00E8  addi r5, r30, 0xe8
	ctx.r[5].s64 = ctx.r[30].s64 + 232;
	// 8287C8F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287C8F8: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8287C8FC: 386101B0  addi r3, r1, 0x1b0
	ctx.r[3].s64 = ctx.r[1].s64 + 432;
	// 8287C900: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8287C904: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8287C908: 4BD269A1  bl 0x825a32a8
	ctx.lr = 0x8287C90C;
	sub_825A32A8(ctx, base);
	// 8287C90C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287C910: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C914: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8287C918: 4BD24E59  bl 0x825a1770
	ctx.lr = 0x8287C91C;
	sub_825A1770(ctx, base);
	// 8287C91C: 386101E8  addi r3, r1, 0x1e8
	ctx.r[3].s64 = ctx.r[1].s64 + 488;
	// 8287C920: 48576B09  bl 0x82df3428
	ctx.lr = 0x8287C924;
	sub_82DF3428(ctx, base);
	// 8287C924: 386101C8  addi r3, r1, 0x1c8
	ctx.r[3].s64 = ctx.r[1].s64 + 456;
	// 8287C928: 4BA4C391  bl 0x822c8cb8
	ctx.lr = 0x8287C92C;
	sub_822C8CB8(ctx, base);
	// 8287C92C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287C930: 48576AF9  bl 0x82df3428
	ctx.lr = 0x8287C934;
	sub_82DF3428(ctx, base);
	// 8287C934: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287C938: 48576AF1  bl 0x82df3428
	ctx.lr = 0x8287C93C;
	sub_82DF3428(ctx, base);
	// 8287C93C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C940: 4BD4D391  bl 0x825c9cd0
	ctx.lr = 0x8287C944;
	sub_825C9CD0(ctx, base);
	// 8287C944: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287C948: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C94C: 388B3E54  addi r4, r11, 0x3e54
	ctx.r[4].s64 = ctx.r[11].s64 + 15956;
	// 8287C950: 485770B9  bl 0x82df3a08
	ctx.lr = 0x8287C954;
	sub_82DF3A08(ctx, base);
	// 8287C954: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8287C958: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C95C: 388B440C  addi r4, r11, 0x440c
	ctx.r[4].s64 = ctx.r[11].s64 + 17420;
	// 8287C960: 485770A9  bl 0x82df3a08
	ctx.lr = 0x8287C964;
	sub_82DF3A08(ctx, base);
	// 8287C964: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C968: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C96C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287C970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8287C974: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8287C978: 419A0024  beq cr6, 0x8287c99c
	if ctx.cr[6].eq {
	pc = 0x8287C99C; continue 'dispatch;
	}
	// 8287C97C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287C980: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287C984: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C988: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287C98C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287C990: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287C994: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287C998: 4082FFE8  bne 0x8287c980
	if !ctx.cr[0].eq {
	pc = 0x8287C980; continue 'dispatch;
	}
	// 8287C99C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8287C9A0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8287C9A4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287C9A8: 387E00EC  addi r3, r30, 0xec
	ctx.r[3].s64 = ctx.r[30].s64 + 236;
	// 8287C9AC: 480D99F5  bl 0x829563a0
	ctx.lr = 0x8287C9B0;
	sub_829563A0(ctx, base);
	// 8287C9B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287C9B4: 48576A75  bl 0x82df3428
	ctx.lr = 0x8287C9B8;
	sub_82DF3428(ctx, base);
	// 8287C9B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287C9BC: 48576A6D  bl 0x82df3428
	ctx.lr = 0x8287C9C0;
	sub_82DF3428(ctx, base);
	// 8287C9C0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287C9C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287C9C8: 419A0008  beq cr6, 0x8287c9d0
	if ctx.cr[6].eq {
	pc = 0x8287C9D0; continue 'dispatch;
	}
	// 8287C9CC: 4BA43EC5  bl 0x822c0890
	ctx.lr = 0x8287C9D0;
	sub_822C0890(ctx, base);
	// 8287C9D0: 38210230  addi r1, r1, 0x230
	ctx.r[1].s64 = ctx.r[1].s64 + 560;
	// 8287C9D4: CBA1FFC0  lfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8287C9D8: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8287C9DC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8287C9E0: 4892B7D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287C9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287C9E8 size=128
    let mut pc: u32 = 0x8287C9E8;
    'dispatch: loop {
        match pc {
            0x8287C9E8 => {
    //   block [0x8287C9E8..0x8287CA68)
	// 8287C9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287C9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287C9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287C9F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287C9F8: 8964001C  lbz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8287C9FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287CA00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CA04: 4182003C  beq 0x8287ca40
	if ctx.cr[0].eq {
	pc = 0x8287CA40; continue 'dispatch;
	}
	// 8287CA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CA0C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8287CA10: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8287CA14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CA18: 808A0B24  lwz r4, 0xb24(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2852 as u32) ) } as u64;
	// 8287CA1C: 48576FED  bl 0x82df3a08
	ctx.lr = 0x8287CA20;
	sub_82DF3A08(ctx, base);
	// 8287CA20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CA24: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8287CA28: 488D7BD9  bl 0x83154600
	ctx.lr = 0x8287CA2C;
	sub_83154600(ctx, base);
	// 8287CA2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287CA30: 4BF72D11  bl 0x827ef740
	ctx.lr = 0x8287CA34;
	sub_827EF740(ctx, base);
	// 8287CA34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CA38: 485769F1  bl 0x82df3428
	ctx.lr = 0x8287CA3C;
	sub_82DF3428(ctx, base);
	// 8287CA3C: 48000014  b 0x8287ca50
	pc = 0x8287CA50; continue 'dispatch;
	// 8287CA40: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8287CA44: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8287CA48: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8287CA4C: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8287CA50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8287CA54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CA60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CA68 size=196
    let mut pc: u32 = 0x8287CA68;
    'dispatch: loop {
        match pc {
            0x8287CA68 => {
    //   block [0x8287CA68..0x8287CB2C)
	// 8287CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CA70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287CA74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CA7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287CA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CA84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287CA88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287CA8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CA90: 4BA43EA9  bl 0x822c0938
	ctx.lr = 0x8287CA94;
	sub_822C0938(ctx, base);
	// 8287CA94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CA98: 41820028  beq 0x8287cac0
	if ctx.cr[0].eq {
	pc = 0x8287CAC0; continue 'dispatch;
	}
	// 8287CA9C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287CAA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287CAA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287CAA8: 392B3F84  addi r9, r11, 0x3f84
	ctx.r[9].s64 = ctx.r[11].s64 + 16260;
	// 8287CAAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287CAB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287CAB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287CAB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287CABC: 48000008  b 0x8287cac4
	pc = 0x8287CAC4; continue 'dispatch;
	// 8287CAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CAC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CAC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287CACC: 409A0044  bne cr6, 0x8287cb10
	if !ctx.cr[6].eq {
	pc = 0x8287CB10; continue 'dispatch;
	}
	// 8287CAD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287CAD4: 419A001C  beq cr6, 0x8287caf0
	if ctx.cr[6].eq {
	pc = 0x8287CAF0; continue 'dispatch;
	}
	// 8287CAD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CADC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287CAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CAE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287CAE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287CAEC: 4E800421  bctrl
	ctx.lr = 0x8287CAF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287CAF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287CAF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287CAF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CAFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287CB00: 816B95C8  lwz r11, -0x6a38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27192 as u32) ) } as u64;
	// 8287CB04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287CB08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287CB0C: 4BA434F5  bl 0x822c0000
	ctx.lr = 0x8287CB10;
	sub_822C0000(ctx, base);
	// 8287CB10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287CB14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CB20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287CB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CB30 size=196
    let mut pc: u32 = 0x8287CB30;
    'dispatch: loop {
        match pc {
            0x8287CB30 => {
    //   block [0x8287CB30..0x8287CBF4)
	// 8287CB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CB38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287CB3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CB40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CB44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287CB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CB4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287CB50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287CB54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CB58: 4BA43DE1  bl 0x822c0938
	ctx.lr = 0x8287CB5C;
	sub_822C0938(ctx, base);
	// 8287CB5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CB60: 41820028  beq 0x8287cb88
	if ctx.cr[0].eq {
	pc = 0x8287CB88; continue 'dispatch;
	}
	// 8287CB64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287CB68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287CB6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287CB70: 392B3F98  addi r9, r11, 0x3f98
	ctx.r[9].s64 = ctx.r[11].s64 + 16280;
	// 8287CB74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287CB78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287CB7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287CB80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287CB84: 48000008  b 0x8287cb8c
	pc = 0x8287CB8C; continue 'dispatch;
	// 8287CB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CB8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CB90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287CB94: 409A0044  bne cr6, 0x8287cbd8
	if !ctx.cr[6].eq {
	pc = 0x8287CBD8; continue 'dispatch;
	}
	// 8287CB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287CB9C: 419A001C  beq cr6, 0x8287cbb8
	if ctx.cr[6].eq {
	pc = 0x8287CBB8; continue 'dispatch;
	}
	// 8287CBA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CBA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287CBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CBAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CBB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287CBB4: 4E800421  bctrl
	ctx.lr = 0x8287CBB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287CBB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287CBBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287CBC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CBC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287CBC8: 816B95C8  lwz r11, -0x6a38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27192 as u32) ) } as u64;
	// 8287CBCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287CBD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287CBD4: 4BA4342D  bl 0x822c0000
	ctx.lr = 0x8287CBD8;
	sub_822C0000(ctx, base);
	// 8287CBD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287CBDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CBE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287CBEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CBF8 size=196
    let mut pc: u32 = 0x8287CBF8;
    'dispatch: loop {
        match pc {
            0x8287CBF8 => {
    //   block [0x8287CBF8..0x8287CCBC)
	// 8287CBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CC00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287CC04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CC0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287CC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CC14: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287CC18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287CC1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CC20: 4BA43D19  bl 0x822c0938
	ctx.lr = 0x8287CC24;
	sub_822C0938(ctx, base);
	// 8287CC24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CC28: 41820028  beq 0x8287cc50
	if ctx.cr[0].eq {
	pc = 0x8287CC50; continue 'dispatch;
	}
	// 8287CC2C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287CC30: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287CC34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287CC38: 392B3FAC  addi r9, r11, 0x3fac
	ctx.r[9].s64 = ctx.r[11].s64 + 16300;
	// 8287CC3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287CC40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287CC44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287CC48: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287CC4C: 48000008  b 0x8287cc54
	pc = 0x8287CC54; continue 'dispatch;
	// 8287CC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CC54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287CC5C: 409A0044  bne cr6, 0x8287cca0
	if !ctx.cr[6].eq {
	pc = 0x8287CCA0; continue 'dispatch;
	}
	// 8287CC60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287CC64: 419A001C  beq cr6, 0x8287cc80
	if ctx.cr[6].eq {
	pc = 0x8287CC80; continue 'dispatch;
	}
	// 8287CC68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CC6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287CC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CC74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CC78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287CC7C: 4E800421  bctrl
	ctx.lr = 0x8287CC80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287CC80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287CC84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287CC88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CC8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287CC90: 816B95C8  lwz r11, -0x6a38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27192 as u32) ) } as u64;
	// 8287CC94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287CC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287CC9C: 4BA43365  bl 0x822c0000
	ctx.lr = 0x8287CCA0;
	sub_822C0000(ctx, base);
	// 8287CCA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287CCA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CCB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287CCB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CCC0 size=196
    let mut pc: u32 = 0x8287CCC0;
    'dispatch: loop {
        match pc {
            0x8287CCC0 => {
    //   block [0x8287CCC0..0x8287CD84)
	// 8287CCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CCC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287CCCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CCD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CCD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287CCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CCDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287CCE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287CCE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CCE8: 4BA43C51  bl 0x822c0938
	ctx.lr = 0x8287CCEC;
	sub_822C0938(ctx, base);
	// 8287CCEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CCF0: 41820028  beq 0x8287cd18
	if ctx.cr[0].eq {
	pc = 0x8287CD18; continue 'dispatch;
	}
	// 8287CCF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287CCF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287CCFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287CD00: 392B3FC0  addi r9, r11, 0x3fc0
	ctx.r[9].s64 = ctx.r[11].s64 + 16320;
	// 8287CD04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287CD08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287CD0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287CD10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287CD14: 48000008  b 0x8287cd1c
	pc = 0x8287CD1C; continue 'dispatch;
	// 8287CD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CD1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287CD20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287CD24: 409A0044  bne cr6, 0x8287cd68
	if !ctx.cr[6].eq {
	pc = 0x8287CD68; continue 'dispatch;
	}
	// 8287CD28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287CD2C: 419A001C  beq cr6, 0x8287cd48
	if ctx.cr[6].eq {
	pc = 0x8287CD48; continue 'dispatch;
	}
	// 8287CD30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CD34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287CD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CD3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CD40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287CD44: 4E800421  bctrl
	ctx.lr = 0x8287CD48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287CD48: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287CD4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287CD50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CD54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287CD58: 816B95C8  lwz r11, -0x6a38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27192 as u32) ) } as u64;
	// 8287CD5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287CD60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287CD64: 4BA4329D  bl 0x822c0000
	ctx.lr = 0x8287CD68;
	sub_822C0000(ctx, base);
	// 8287CD68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287CD6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CD78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287CD7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CD88 size=104
    let mut pc: u32 = 0x8287CD88;
    'dispatch: loop {
        match pc {
            0x8287CD88 => {
    //   block [0x8287CD88..0x8287CDF0)
	// 8287CD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CD8C: 4892B3E1  bl 0x831a816c
	ctx.lr = 0x8287CD90;
	sub_831A8130(ctx, base);
	// 8287CD90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CD94: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8287CD98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287CD9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287CDA0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287CDA4: 41820034  beq 0x8287cdd8
	if ctx.cr[0].eq {
	pc = 0x8287CDD8; continue 'dispatch;
	}
	// 8287CDA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CDAC: 4892CBDD  bl 0x831a9988
	ctx.lr = 0x8287CDB0;
	sub_831A9988(ctx, base);
	// 8287CDB0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287CDB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287CDB8: 386B9740  addi r3, r11, -0x68c0
	ctx.r[3].s64 = ctx.r[11].s64 + -26816;
	// 8287CDBC: 4892B33D  bl 0x831a80f8
	ctx.lr = 0x8287CDC0;
	sub_831A80F8(ctx, base);
	// 8287CDC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287CDC4: 41820014  beq 0x8287cdd8
	if ctx.cr[0].eq {
	pc = 0x8287CDD8; continue 'dispatch;
	}
	// 8287CDC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287CDCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287CDD0: 4BFFFC19  bl 0x8287c9e8
	ctx.lr = 0x8287CDD4;
	sub_8287C9E8(ctx, base);
	// 8287CDD4: 48000014  b 0x8287cde8
	pc = 0x8287CDE8; continue 'dispatch;
	// 8287CDD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8287CDDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287CDE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287CDE4: 489016A5  bl 0x8317e488
	ctx.lr = 0x8287CDE8;
	sub_8317E488(ctx, base);
	// 8287CDE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CDEC: 4892B3D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CDF0 size=124
    let mut pc: u32 = 0x8287CDF0;
    'dispatch: loop {
        match pc {
            0x8287CDF0 => {
    //   block [0x8287CDF0..0x8287CE6C)
	// 8287CDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CDF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CDFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CE00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287CE04: 48000E25  bl 0x8287dc28
	ctx.lr = 0x8287CE08;
	sub_8287DC28(ctx, base);
	// 8287CE08: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287CE0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287CE10: 394A3FD4  addi r10, r10, 0x3fd4
	ctx.r[10].s64 = ctx.r[10].s64 + 16340;
	// 8287CE14: 3D20832C  lis r9, -0x7cd4
	ctx.r[9].s64 = -2094268416;
	// 8287CE18: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8287CE1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CE20: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8287CE24: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8287CE28: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8287CE2C: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8287CE30: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8287CE34: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 8287CE38: 808995C0  lwz r4, -0x6a40(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27200 as u32) ) } as u64;
	// 8287CE3C: 48576BCD  bl 0x82df3a08
	ctx.lr = 0x8287CE40;
	sub_82DF3A08(ctx, base);
	// 8287CE40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CE44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287CE48: 485DC8A1  bl 0x82e596e8
	ctx.lr = 0x8287CE4C;
	sub_82E596E8(ctx, base);
	// 8287CE4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287CE50: 485765D9  bl 0x82df3428
	ctx.lr = 0x8287CE54;
	sub_82DF3428(ctx, base);
	// 8287CE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CE58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CE70 size=148
    let mut pc: u32 = 0x8287CE70;
    'dispatch: loop {
        match pc {
            0x8287CE70 => {
    //   block [0x8287CE70..0x8287CF04)
	// 8287CE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CE78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CE7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CE80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287CE84: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8287CE88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287CE8C: 419A003C  beq cr6, 0x8287cec8
	if ctx.cr[6].eq {
	pc = 0x8287CEC8; continue 'dispatch;
	}
	// 8287CE90: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 8287CE94: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 8287CE98: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287CE9C: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 8287CEA0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8287CEA4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287CEA8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287CEAC: 4082FFE8  bne 0x8287ce94
	if !ctx.cr[0].eq {
	pc = 0x8287CE94; continue 'dispatch;
	}
	// 8287CEB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287CEB4: 409A0014  bne cr6, 0x8287cec8
	if !ctx.cr[6].eq {
	pc = 0x8287CEC8; continue 'dispatch;
	}
	// 8287CEB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287CEBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8287CEC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287CEC4: 4E800421  bctrl
	ctx.lr = 0x8287CEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287CEC8: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8287CECC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287CED0: 419A0008  beq cr6, 0x8287ced8
	if ctx.cr[6].eq {
	pc = 0x8287CED8; continue 'dispatch;
	}
	// 8287CED4: 4BA439BD  bl 0x822c0890
	ctx.lr = 0x8287CED8;
	sub_822C0890(ctx, base);
	// 8287CED8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8287CEDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287CEE0: 419A0008  beq cr6, 0x8287cee8
	if ctx.cr[6].eq {
	pc = 0x8287CEE8; continue 'dispatch;
	}
	// 8287CEE4: 4BA439AD  bl 0x822c0890
	ctx.lr = 0x8287CEE8;
	sub_822C0890(ctx, base);
	// 8287CEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CEEC: 485DF69D  bl 0x82e5c588
	ctx.lr = 0x8287CEF0;
	sub_82E5C588(ctx, base);
	// 8287CEF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287CEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CEFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CF08 size=76
    let mut pc: u32 = 0x8287CF08;
    'dispatch: loop {
        match pc {
            0x8287CF08 => {
    //   block [0x8287CF08..0x8287CF54)
	// 8287CF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287CF10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287CF14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287CF18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287CF20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8287CF24: 4BFFFF4D  bl 0x8287ce70
	ctx.lr = 0x8287CF28;
	sub_8287CE70(ctx, base);
	// 8287CF28: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287CF2C: 4182000C  beq 0x8287cf38
	if ctx.cr[0].eq {
	pc = 0x8287CF38; continue 'dispatch;
	}
	// 8287CF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CF34: 485754A5  bl 0x82df23d8
	ctx.lr = 0x8287CF38;
	sub_82DF23D8(ctx, base);
	// 8287CF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287CF3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287CF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287CF48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287CF4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287CF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CF58 size=112
    let mut pc: u32 = 0x8287CF58;
    'dispatch: loop {
        match pc {
            0x8287CF58 => {
    //   block [0x8287CF58..0x8287CFC8)
	// 8287CF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CF5C: 4892B211  bl 0x831a816c
	ctx.lr = 0x8287CF60;
	sub_831A8130(ctx, base);
	// 8287CF60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CF64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287CF68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287CF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287CF70: 388B3FF8  addi r4, r11, 0x3ff8
	ctx.r[4].s64 = ctx.r[11].s64 + 16376;
	// 8287CF74: 38A00112  li r5, 0x112
	ctx.r[5].s64 = 274;
	// 8287CF78: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 8287CF7C: 4857546D  bl 0x82df23e8
	ctx.lr = 0x8287CF80;
	sub_82DF23E8(ctx, base);
	// 8287CF80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CF84: 41820010  beq 0x8287cf94
	if ctx.cr[0].eq {
	pc = 0x8287CF94; continue 'dispatch;
	}
	// 8287CF88: 4BFFFE69  bl 0x8287cdf0
	ctx.lr = 0x8287CF8C;
	sub_8287CDF0(ctx, base);
	// 8287CF8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287CF90: 48000008  b 0x8287cf98
	pc = 0x8287CF98; continue 'dispatch;
	// 8287CF94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287CF98: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287CF9C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287CFA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287CFA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287CFA8: 4BFFFAC1  bl 0x8287ca68
	ctx.lr = 0x8287CFAC;
	sub_8287CA68(ctx, base);
	// 8287CFAC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287CFB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287CFB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287CFB8: 4BA43049  bl 0x822c0000
	ctx.lr = 0x8287CFBC;
	sub_822C0000(ctx, base);
	// 8287CFBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287CFC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287CFC4: 4892B1F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287CFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287CFC8 size=112
    let mut pc: u32 = 0x8287CFC8;
    'dispatch: loop {
        match pc {
            0x8287CFC8 => {
    //   block [0x8287CFC8..0x8287D038)
	// 8287CFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287CFCC: 4892B1A1  bl 0x831a816c
	ctx.lr = 0x8287CFD0;
	sub_831A8130(ctx, base);
	// 8287CFD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287CFD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287CFD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287CFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287CFE0: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8287CFE4: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 8287CFE8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8287CFEC: 485753FD  bl 0x82df23e8
	ctx.lr = 0x8287CFF0;
	sub_82DF23E8(ctx, base);
	// 8287CFF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287CFF4: 41820010  beq 0x8287d004
	if ctx.cr[0].eq {
	pc = 0x8287D004; continue 'dispatch;
	}
	// 8287CFF8: 48098A61  bl 0x82915a58
	ctx.lr = 0x8287CFFC;
	sub_82915A58(ctx, base);
	// 8287CFFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287D000: 48000008  b 0x8287d008
	pc = 0x8287D008; continue 'dispatch;
	// 8287D004: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287D008: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287D00C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287D010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287D014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287D018: 4BFFFB19  bl 0x8287cb30
	ctx.lr = 0x8287D01C;
	sub_8287CB30(ctx, base);
	// 8287D01C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287D020: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287D024: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287D028: 4BA42FD9  bl 0x822c0000
	ctx.lr = 0x8287D02C;
	sub_822C0000(ctx, base);
	// 8287D02C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287D030: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287D034: 4892B188  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287D038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287D038 size=120
    let mut pc: u32 = 0x8287D038;
    'dispatch: loop {
        match pc {
            0x8287D038 => {
    //   block [0x8287D038..0x8287D0B0)
	// 8287D038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287D03C: 4892B131  bl 0x831a816c
	ctx.lr = 0x8287D040;
	sub_831A8130(ctx, base);
	// 8287D040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287D044: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287D048: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287D04C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287D050: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287D054: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8287D058: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8287D05C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 8287D060: 48575389  bl 0x82df23e8
	ctx.lr = 0x8287D064;
	sub_82DF23E8(ctx, base);
	// 8287D064: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287D068: 41820014  beq 0x8287d07c
	if ctx.cr[0].eq {
	pc = 0x8287D07C; continue 'dispatch;
	}
	// 8287D06C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287D070: 48098FF9  bl 0x82916068
	ctx.lr = 0x8287D074;
	sub_82916068(ctx, base);
	// 8287D074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287D078: 48000008  b 0x8287d080
	pc = 0x8287D080; continue 'dispatch;
	// 8287D07C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287D080: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287D084: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287D088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287D08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287D090: 4BFFFB69  bl 0x8287cbf8
	ctx.lr = 0x8287D094;
	sub_8287CBF8(ctx, base);
	// 8287D094: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287D098: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287D09C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287D0A0: 4BA42F61  bl 0x822c0000
	ctx.lr = 0x8287D0A4;
	sub_822C0000(ctx, base);
	// 8287D0A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287D0A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287D0AC: 4892B110  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287D0B0 size=120
    let mut pc: u32 = 0x8287D0B0;
    'dispatch: loop {
        match pc {
            0x8287D0B0 => {
    //   block [0x8287D0B0..0x8287D128)
	// 8287D0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287D0B4: 4892B0B9  bl 0x831a816c
	ctx.lr = 0x8287D0B8;
	sub_831A8130(ctx, base);
	// 8287D0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287D0BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287D0C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287D0C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287D0C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287D0CC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8287D0D0: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8287D0D4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8287D0D8: 48575311  bl 0x82df23e8
	ctx.lr = 0x8287D0DC;
	sub_82DF23E8(ctx, base);
	// 8287D0DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287D0E0: 41820014  beq 0x8287d0f4
	if ctx.cr[0].eq {
	pc = 0x8287D0F4; continue 'dispatch;
	}
	// 8287D0E4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287D0E8: 480989B1  bl 0x82915a98
	ctx.lr = 0x8287D0EC;
	sub_82915A98(ctx, base);
	// 8287D0EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287D0F0: 48000008  b 0x8287d0f8
	pc = 0x8287D0F8; continue 'dispatch;
	// 8287D0F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287D0F8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287D0FC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287D100: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287D104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287D108: 4BFFFBB9  bl 0x8287ccc0
	ctx.lr = 0x8287D10C;
	sub_8287CCC0(ctx, base);
	// 8287D10C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287D110: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287D114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287D118: 4BA42EE9  bl 0x822c0000
	ctx.lr = 0x8287D11C;
	sub_822C0000(ctx, base);
	// 8287D11C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287D120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287D124: 4892B098  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287D128 size=264
    let mut pc: u32 = 0x8287D128;
    'dispatch: loop {
        match pc {
            0x8287D128 => {
    //   block [0x8287D128..0x8287D230)
	// 8287D128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287D12C: 4892B041  bl 0x831a816c
	ctx.lr = 0x8287D130;
	sub_831A8130(ctx, base);
	// 8287D130: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287D134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287D138: 488D74C9  bl 0x83154600
	ctx.lr = 0x8287D13C;
	sub_83154600(ctx, base);
	// 8287D13C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287D140: 48099629  bl 0x82916768
	ctx.lr = 0x8287D144;
	sub_82916768(ctx, base);
	// 8287D144: 4808C445  bl 0x82909588
	ctx.lr = 0x8287D148;
	sub_82909588(ctx, base);
	// 8287D148: 480C3361  bl 0x829404a8
	ctx.lr = 0x8287D14C;
	sub_829404A8(ctx, base);
	// 8287D14C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287D150: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287D154: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287D158: 4BF726F9  bl 0x827ef850
	ctx.lr = 0x8287D15C;
	sub_827EF850(ctx, base);
	// 8287D15C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8287D160: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8287D164: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287D168: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287D16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287D170: C02B029C  lfs f1, 0x29c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(668 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287D174: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8287D178: 4E800421  bctrl
	ctx.lr = 0x8287D17C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287D17C: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8287D180: 389F0074  addi r4, r31, 0x74
	ctx.r[4].s64 = ctx.r[31].s64 + 116;
	// 8287D184: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287D188: 419A00A0  beq cr6, 0x8287d228
	if ctx.cr[6].eq {
	pc = 0x8287D228; continue 'dispatch;
	}
	// 8287D18C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287D190: 4BFFFF21  bl 0x8287d0b0
	ctx.lr = 0x8287D194;
	sub_8287D0B0(ctx, base);
	// 8287D194: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287D198: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8287D19C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287D1A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287D1A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287D1A8: 419A0024  beq cr6, 0x8287d1cc
	if ctx.cr[6].eq {
	pc = 0x8287D1CC; continue 'dispatch;
	}
	// 8287D1AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8287D1B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287D1B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287D1B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287D1BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287D1C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287D1C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287D1C8: 4082FFE8  bne 0x8287d1b0
	if !ctx.cr[0].eq {
	pc = 0x8287D1B0; continue 'dispatch;
	}
	// 8287D1CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287D1D0: 488D7431  bl 0x83154600
	ctx.lr = 0x8287D1D4;
	sub_83154600(ctx, base);
	// 8287D1D4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287D1D8: 83FF0070  lwz r31, 0x70(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8287D1DC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8287D1E0: 3BCB3FF8  addi r30, r11, 0x3ff8
	ctx.r[30].s64 = ctx.r[11].s64 + 16376;
	// 8287D1E4: 4BF70635  bl 0x827ed818
	ctx.lr = 0x8287D1E8;
	sub_827ED818(ctx, base);
	// 8287D1E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287D1EC: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 8287D1F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8287D1F4: 38A00109  li r5, 0x109
	ctx.r[5].s64 = 265;
	// 8287D1F8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8287D1FC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8287D200: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287D204: 485DB83D  bl 0x82e58a40
	ctx.lr = 0x8287D208;
	sub_82E58A40(ctx, base);
	// 8287D208: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8287D20C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287D210: 419A0008  beq cr6, 0x8287d218
	if ctx.cr[6].eq {
	pc = 0x8287D218; continue 'dispatch;
	}
	// 8287D214: 4BA4367D  bl 0x822c0890
	ctx.lr = 0x8287D218;
	sub_822C0890(ctx, base);
	// 8287D218: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287D21C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287D220: 419A0008  beq cr6, 0x8287d228
	if ctx.cr[6].eq {
	pc = 0x8287D228; continue 'dispatch;
	}
	// 8287D224: 4BA4366D  bl 0x822c0890
	ctx.lr = 0x8287D228;
	sub_822C0890(ctx, base);
	// 8287D228: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8287D22C: 4892AF90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287D230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287D230 size=400
    let mut pc: u32 = 0x8287D230;
    'dispatch: loop {
        match pc {
            0x8287D230 => {
    //   block [0x8287D230..0x8287D3C0)
	// 8287D230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287D234: 4892AF29  bl 0x831a815c
	ctx.lr = 0x8287D238;
	sub_831A8130(ctx, base);
	// 8287D238: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287D23C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287D240: 488D73C1  bl 0x83154600
	ctx.lr = 0x8287D244;
	sub_83154600(ctx, base);
	// 8287D244: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287D248: 389F0074  addi r4, r31, 0x74
	ctx.r[4].s64 = ctx.r[31].s64 + 116;
	// 8287D24C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287D250: 4BFFFDE9  bl 0x8287d038
	ctx.lr = 0x8287D254;
	sub_8287D038(ctx, base);
	// 8287D254: 83610054  lwz r27, 0x54(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8287D258: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8287D25C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8287D260: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 8287D264: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 8287D268: 419A0024  beq cr6, 0x8287d28c
	if ctx.cr[6].eq {
	pc = 0x8287D28C; continue 'dispatch;
	}
	// 8287D26C: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 8287D270: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287D274: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287D278: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287D27C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287D280: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287D284: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287D288: 4082FFE8  bne 0x8287d270
	if !ctx.cr[0].eq {
	pc = 0x8287D270; continue 'dispatch;
	}
	// 8287D28C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287D290: 83DF0070  lwz r30, 0x70(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8287D294: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287D298: 3B4B3FF8  addi r26, r11, 0x3ff8
	ctx.r[26].s64 = ctx.r[11].s64 + 16376;
	// 8287D29C: 3B210058  addi r25, r1, 0x58
	ctx.r[25].s64 = ctx.r[1].s64 + 88;
	// 8287D2A0: 4BF70579  bl 0x827ed818
	ctx.lr = 0x8287D2A4;
	sub_827ED818(ctx, base);
	// 8287D2A4: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 8287D2A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8287D2AC: 38A0011B  li r5, 0x11b
	ctx.r[5].s64 = 283;
	// 8287D2B0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8287D2B4: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8287D2B8: 485D9D31  bl 0x82e56fe8
	ctx.lr = 0x8287D2BC;
	sub_82E56FE8(ctx, base);
	// 8287D2BC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287D2C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287D2C4: 419A0008  beq cr6, 0x8287d2cc
	if ctx.cr[6].eq {
	pc = 0x8287D2CC; continue 'dispatch;
	}
	// 8287D2C8: 4BA435C9  bl 0x822c0890
	ctx.lr = 0x8287D2CC;
	sub_822C0890(ctx, base);
	// 8287D2CC: 897C0040  lbz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 8287D2D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287D2D4: 418200A8  beq 0x8287d37c
	if ctx.cr[0].eq {
	pc = 0x8287D37C; continue 'dispatch;
	}
	// 8287D2D8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8287D2DC: 3BDF0080  addi r30, r31, 0x80
	ctx.r[30].s64 = ctx.r[31].s64 + 128;
	// 8287D2E0: 13FC58C7  vcmpequd (lvx128) v31, v28, v11
	tmp.u32 = ctx.r[28].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287D3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287D3C0 size=600
    let mut pc: u32 = 0x8287D3C0;
    'dispatch: loop {
        match pc {
            0x8287D3C0 => {
    //   block [0x8287D3C0..0x8287D618)
	// 8287D3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287D3C4: 4892AD8D  bl 0x831a8150
	ctx.lr = 0x8287D3C8;
	sub_831A8130(ctx, base);
	// 8287D3C8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287D3CC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8287D3D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287D3D4: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 8287D3D8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8287D3DC: 488D7225  bl 0x83154600
	ctx.lr = 0x8287D3E0;
	sub_83154600(ctx, base);
	// 8287D3E0: 48099389  bl 0x82916768
	ctx.lr = 0x8287D3E4;
	sub_82916768(ctx, base);
	// 8287D3E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287D3E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287D3EC: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8287D3F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287D3F4: 4E800421  bctrl
	ctx.lr = 0x8287D3F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287D3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287D3FC: 488D7205  bl 0x83154600
	ctx.lr = 0x8287D400;
	sub_83154600(ctx, base);
	// 8287D400: 48099369  bl 0x82916768
	ctx.lr = 0x8287D404;
	sub_82916768(ctx, base);
	// 8287D404: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287D408: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287D40C: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 8287D410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287D414: 4E800421  bctrl
	ctx.lr = 0x8287D418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287D418: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8287D41C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287D420: 388BDDCC  addi r4, r11, -0x2234
	ctx.r[4].s64 = ctx.r[11].s64 + -8756;
	// 8287D424: 4814B335  bl 0x829c8758
	ctx.lr = 0x8287D428;
	sub_829C8758(ctx, base);
	// 8287D428: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8287D42C: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8287D430: 4878BB89  bl 0x83008fb8
	ctx.lr = 0x8287D434;
	sub_83008FB8(ctx, base);
	// 8287D434: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 8287D438: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287D43C: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 8287D440: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 8287D444: 931F0064  stw r24, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 8287D448: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 8287D44C: 931F0074  stw r24, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[24].u32 ) };
	// 8287D450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287D454: C00A3F7C  lfs f0, 0x3f7c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16252 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287D458: D01F0068  stfs f0, 0x68(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8287D45C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8287D618 size=1552
    let mut pc: u32 = 0x8287D618;
    'dispatch: loop {
        match pc {
            0x8287D618 => {
    //   block [0x8287D618..0x8287DC28)
	// 8287D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287D61C: 4892AB39  bl 0x831a8154
	ctx.lr = 0x8287D620;
	sub_831A8130(ctx, base);
	// 8287D620: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8287D624: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287DC28 size=72
    let mut pc: u32 = 0x8287DC28;
    'dispatch: loop {
        match pc {
            0x8287DC28 => {
    //   block [0x8287DC28..0x8287DC70)
	// 8287DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DC30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DC34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DC38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287DC3C: 485DECED  bl 0x82e5c928
	ctx.lr = 0x8287DC40;
	sub_82E5C928(ctx, base);
	// 8287DC40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287DC44: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287DC48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287DC4C: 394A4080  addi r10, r10, 0x4080
	ctx.r[10].s64 = ctx.r[10].s64 + 16512;
	// 8287DC50: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287DC54: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8287DC58: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8287DC5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287DC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287DC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287DC68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287DC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287DC70 size=96
    let mut pc: u32 = 0x8287DC70;
    'dispatch: loop {
        match pc {
            0x8287DC70 => {
    //   block [0x8287DC70..0x8287DCD0)
	// 8287DC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DC78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DC7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DC80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287DC84: 4BFFFFA5  bl 0x8287dc28
	ctx.lr = 0x8287DC88;
	sub_8287DC28(ctx, base);
	// 8287DC88: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287DC8C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287DC90: 396B40B4  addi r11, r11, 0x40b4
	ctx.r[11].s64 = ctx.r[11].s64 + 16564;
	// 8287DC94: 388A40A4  addi r4, r10, 0x40a4
	ctx.r[4].s64 = ctx.r[10].s64 + 16548;
	// 8287DC98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287DC9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DCA0: 48575D69  bl 0x82df3a08
	ctx.lr = 0x8287DCA4;
	sub_82DF3A08(ctx, base);
	// 8287DCA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287DCA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287DCAC: 485DBA3D  bl 0x82e596e8
	ctx.lr = 0x8287DCB0;
	sub_82E596E8(ctx, base);
	// 8287DCB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DCB4: 48575775  bl 0x82df3428
	ctx.lr = 0x8287DCB8;
	sub_82DF3428(ctx, base);
	// 8287DCB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287DCBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287DCC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287DCC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287DCC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287DCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287DCD0 size=116
    let mut pc: u32 = 0x8287DCD0;
    'dispatch: loop {
        match pc {
            0x8287DCD0 => {
    //   block [0x8287DCD0..0x8287DD44)
	// 8287DCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DCD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DCDC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DCE0: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8287DCE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287DCE8: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8287DCEC: 409A0010  bne cr6, 0x8287dcfc
	if !ctx.cr[6].eq {
	pc = 0x8287DCFC; continue 'dispatch;
	}
	// 8287DCF0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287DCF4: 808B0B08  lwz r4, 0xb08(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2824 as u32) ) } as u64;
	// 8287DCF8: 48000014  b 0x8287dd0c
	pc = 0x8287DD0C; continue 'dispatch;
	// 8287DCFC: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 8287DD00: 409A0030  bne cr6, 0x8287dd30
	if !ctx.cr[6].eq {
	pc = 0x8287DD30; continue 'dispatch;
	}
	// 8287DD04: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287DD08: 808B0B10  lwz r4, 0xb10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2832 as u32) ) } as u64;
	// 8287DD0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DD10: 48575CF9  bl 0x82df3a08
	ctx.lr = 0x8287DD14;
	sub_82DF3A08(ctx, base);
	// 8287DD14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287DD18: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8287DD1C: 488D68E5  bl 0x83154600
	ctx.lr = 0x8287DD20;
	sub_83154600(ctx, base);
	// 8287DD20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287DD24: 4BF71A1D  bl 0x827ef740
	ctx.lr = 0x8287DD28;
	sub_827EF740(ctx, base);
	// 8287DD28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DD2C: 485756FD  bl 0x82df3428
	ctx.lr = 0x8287DD30;
	sub_82DF3428(ctx, base);
	// 8287DD30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287DD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287DD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287DD3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287DD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287DD48 size=60
    let mut pc: u32 = 0x8287DD48;
    'dispatch: loop {
        match pc {
            0x8287DD48 => {
    //   block [0x8287DD48..0x8287DD84)
	// 8287DD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DD50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DD54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DD58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287DD5C: 4BFFFECD  bl 0x8287dc28
	ctx.lr = 0x8287DD60;
	sub_8287DC28(ctx, base);
	// 8287DD60: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287DD64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287DD68: 396B40E0  addi r11, r11, 0x40e0
	ctx.r[11].s64 = ctx.r[11].s64 + 16608;
	// 8287DD6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287DD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287DD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287DD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287DD7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287DD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287DD88 size=160
    let mut pc: u32 = 0x8287DD88;
    'dispatch: loop {
        match pc {
            0x8287DD88 => {
    //   block [0x8287DD88..0x8287DE28)
	// 8287DD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DD90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287DD94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DD9C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287DDA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287DDA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DDA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8287DDAC: 808B0114  lwz r4, 0x114(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8287DDB0: 48575C59  bl 0x82df3a08
	ctx.lr = 0x8287DDB4;
	sub_82DF3A08(ctx, base);
	// 8287DDB4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8287DDB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287DDBC: 4BAAEA85  bl 0x8232c840
	ctx.lr = 0x8287DDC0;
	sub_8232C840(ctx, base);
	// 8287DDC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8287DDC4: 4BFEF18D  bl 0x8286cf50
	ctx.lr = 0x8287DDC8;
	sub_8286CF50(ctx, base);
	// 8287DDC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DDCC: 4857565D  bl 0x82df3428
	ctx.lr = 0x8287DDD0;
	sub_82DF3428(ctx, base);
	// 8287DDD0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287DDD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287DDD8: 808B0B14  lwz r4, 0xb14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2836 as u32) ) } as u64;
	// 8287DDDC: 48575C2D  bl 0x82df3a08
	ctx.lr = 0x8287DDE0;
	sub_82DF3A08(ctx, base);
	// 8287DDE0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287DDE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8287DDE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287DDEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287DDF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287DDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8287DDF8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287DDFC: 485E0C95  bl 0x82e5ea90
	ctx.lr = 0x8287DE00;
	sub_82E5EA90(ctx, base);
	// 8287DE00: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287DE04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287DE08: 419A0008  beq cr6, 0x8287de10
	if ctx.cr[6].eq {
	pc = 0x8287DE10; continue 'dispatch;
	}
	// 8287DE0C: 4BA42A85  bl 0x822c0890
	ctx.lr = 0x8287DE10;
	sub_822C0890(ctx, base);
	// 8287DE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8287DE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287DE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287DE1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287DE20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287DE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8287DE28 size=224
    let mut pc: u32 = 0x8287DE28;
    'dispatch: loop {
        match pc {
            0x8287DE28 => {
    //   block [0x8287DE28..0x8287DF08)
	// 8287DE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DE30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287DE34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DE38: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287DF08 size=196
    let mut pc: u32 = 0x8287DF08;
    'dispatch: loop {
        match pc {
            0x8287DF08 => {
    //   block [0x8287DF08..0x8287DFCC)
	// 8287DF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DF10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287DF14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DF18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DF1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287DF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287DF24: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287DF28: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287DF2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287DF30: 4BA42A09  bl 0x822c0938
	ctx.lr = 0x8287DF34;
	sub_822C0938(ctx, base);
	// 8287DF34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287DF38: 41820028  beq 0x8287df60
	if ctx.cr[0].eq {
	pc = 0x8287DF60; continue 'dispatch;
	}
	// 8287DF3C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287DF40: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287DF44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287DF48: 392B410C  addi r9, r11, 0x410c
	ctx.r[9].s64 = ctx.r[11].s64 + 16652;
	// 8287DF4C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287DF50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287DF54: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287DF58: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287DF5C: 48000008  b 0x8287df64
	pc = 0x8287DF64; continue 'dispatch;
	// 8287DF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287DF64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287DF68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287DF6C: 409A0044  bne cr6, 0x8287dfb0
	if !ctx.cr[6].eq {
	pc = 0x8287DFB0; continue 'dispatch;
	}
	// 8287DF70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287DF74: 419A001C  beq cr6, 0x8287df90
	if ctx.cr[6].eq {
	pc = 0x8287DF90; continue 'dispatch;
	}
	// 8287DF78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287DF7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287DF80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287DF84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287DF88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287DF8C: 4E800421  bctrl
	ctx.lr = 0x8287DF90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287DF90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287DF94: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287DF98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287DF9C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287DFA0: 816B98BC  lwz r11, -0x6744(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26436 as u32) ) } as u64;
	// 8287DFA4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287DFA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287DFAC: 4BA42055  bl 0x822c0000
	ctx.lr = 0x8287DFB0;
	sub_822C0000(ctx, base);
	// 8287DFB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287DFB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287DFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287DFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287DFC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287DFC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287DFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287DFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287DFD0 size=108
    let mut pc: u32 = 0x8287DFD0;
    'dispatch: loop {
        match pc {
            0x8287DFD0 => {
    //   block [0x8287DFD0..0x8287E03C)
	// 8287DFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287DFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287DFD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287DFDC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287DFE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287DFE4: 4BFFFC45  bl 0x8287dc28
	ctx.lr = 0x8287DFE8;
	sub_8287DC28(ctx, base);
	// 8287DFE8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287DFEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287DFF0: 394A4120  addi r10, r10, 0x4120
	ctx.r[10].s64 = ctx.r[10].s64 + 16672;
	// 8287DFF4: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 8287DFF8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8287DFFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E000: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8287E004: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8287E008: 80890AF0  lwz r4, 0xaf0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2800 as u32) ) } as u64;
	// 8287E00C: 485759FD  bl 0x82df3a08
	ctx.lr = 0x8287E010;
	sub_82DF3A08(ctx, base);
	// 8287E010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E014: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E018: 485DB6D1  bl 0x82e596e8
	ctx.lr = 0x8287E01C;
	sub_82E596E8(ctx, base);
	// 8287E01C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E020: 48575409  bl 0x82df3428
	ctx.lr = 0x8287E024;
	sub_82DF3428(ctx, base);
	// 8287E024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E028: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8287E040 size=32
    let mut pc: u32 = 0x8287E040;
    'dispatch: loop {
        match pc {
            0x8287E040 => {
    //   block [0x8287E040..0x8287E060)
	// 8287E040: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287E044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8287E048: 394B0064  addi r10, r11, 0x64
	ctx.r[10].s64 = ctx.r[11].s64 + 100;
	// 8287E04C: 912B0064  stw r9, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 8287E050: 806B0068  lwz r3, 0x68(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8287E054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287E058: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8287E05C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8287E060 size=8
    let mut pc: u32 = 0x8287E060;
    'dispatch: loop {
        match pc {
            0x8287E060 => {
    //   block [0x8287E060..0x8287E068)
	// 8287E060: 4BA42830  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 8287E064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E068 size=112
    let mut pc: u32 = 0x8287E068;
    'dispatch: loop {
        match pc {
            0x8287E068 => {
    //   block [0x8287E068..0x8287E0D8)
	// 8287E068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E06C: 4892A101  bl 0x831a816c
	ctx.lr = 0x8287E070;
	sub_831A8130(ctx, base);
	// 8287E070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E074: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E078: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287E07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287E080: 388B4148  addi r4, r11, 0x4148
	ctx.r[4].s64 = ctx.r[11].s64 + 16712;
	// 8287E084: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 8287E088: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 8287E08C: 4857435D  bl 0x82df23e8
	ctx.lr = 0x8287E090;
	sub_82DF23E8(ctx, base);
	// 8287E090: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287E094: 41820010  beq 0x8287e0a4
	if ctx.cr[0].eq {
	pc = 0x8287E0A4; continue 'dispatch;
	}
	// 8287E098: 4BFFFF39  bl 0x8287dfd0
	ctx.lr = 0x8287E09C;
	sub_8287DFD0(ctx, base);
	// 8287E09C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E0A0: 48000008  b 0x8287e0a8
	pc = 0x8287E0A8; continue 'dispatch;
	// 8287E0A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287E0A8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287E0AC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287E0B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E0B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E0B8: 4BFFFE51  bl 0x8287df08
	ctx.lr = 0x8287E0BC;
	sub_8287DF08(ctx, base);
	// 8287E0BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287E0C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E0C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E0C8: 4BA41F39  bl 0x822c0000
	ctx.lr = 0x8287E0CC;
	sub_822C0000(ctx, base);
	// 8287E0CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287E0D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E0D4: 4892A0E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287E0D8 size=192
    let mut pc: u32 = 0x8287E0D8;
    'dispatch: loop {
        match pc {
            0x8287E0D8 => {
    //   block [0x8287E0D8..0x8287E198)
	// 8287E0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E0DC: 4892A08D  bl 0x831a8168
	ctx.lr = 0x8287E0E0;
	sub_831A8130(ctx, base);
	// 8287E0E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E0E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8287E0E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287E0EC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8287E0F0: 488D6511  bl 0x83154600
	ctx.lr = 0x8287E0F4;
	sub_83154600(ctx, base);
	// 8287E0F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E0F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287E0FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E100: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E104: 4BF7174D  bl 0x827ef850
	ctx.lr = 0x8287E108;
	sub_827EF850(ctx, base);
	// 8287E108: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8287E10C: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8287E110: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287E114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E118: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287E11C: C02B029C  lfs f1, 0x29c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(668 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287E120: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8287E124: 4E800421  bctrl
	ctx.lr = 0x8287E128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E128: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287E130: 388B4148  addi r4, r11, 0x4148
	ctx.r[4].s64 = ctx.r[11].s64 + 16712;
	// 8287E134: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 8287E138: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8287E13C: 4BA4229D  bl 0x822c03d8
	ctx.lr = 0x8287E140;
	sub_822C03D8(ctx, base);
	// 8287E140: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8287E144: 41820030  beq 0x8287e174
	if ctx.cr[0].eq {
	pc = 0x8287E174; continue 'dispatch;
	}
	// 8287E148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E14C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8287E150: 48098619  bl 0x82916768
	ctx.lr = 0x8287E154;
	sub_82916768(ctx, base);
	// 8287E154: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287E158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E15C: 4BC913BD  bl 0x8250f518
	ctx.lr = 0x8287E160;
	sub_8250F518(ctx, base);
	// 8287E160: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287E164: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E168: 4BC12F89  bl 0x824910f0
	ctx.lr = 0x8287E16C;
	sub_824910F0(ctx, base);
	// 8287E16C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287E170: 48000008  b 0x8287e178
	pc = 0x8287E178; continue 'dispatch;
	// 8287E174: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287E178: 387C0064  addi r3, r28, 0x64
	ctx.r[3].s64 = ctx.r[28].s64 + 100;
	// 8287E17C: 4BA8C565  bl 0x8230a6e0
	ctx.lr = 0x8287E180;
	sub_8230A6E0(ctx, base);
	// 8287E180: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E184: 4182000C  beq 0x8287e190
	if ctx.cr[0].eq {
	pc = 0x8287E190; continue 'dispatch;
	}
	// 8287E188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E18C: 48573B05  bl 0x82df1c90
	ctx.lr = 0x8287E190;
	sub_82DF1C90(ctx, base);
	// 8287E190: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8287E194: 4892A024  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287E198 size=140
    let mut pc: u32 = 0x8287E198;
    'dispatch: loop {
        match pc {
            0x8287E198 => {
    //   block [0x8287E198..0x8287E224)
	// 8287E198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287E1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E1AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E1B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8287E1B4: 897F0064  lbz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8287E1B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287E1BC: 41820018  beq 0x8287e1d4
	if ctx.cr[0].eq {
	pc = 0x8287E1D4; continue 'dispatch;
	}
	// 8287E1C0: 485DB5A1  bl 0x82e59760
	ctx.lr = 0x8287E1C4;
	sub_82E59760(ctx, base);
	// 8287E1C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287E1C8: C00B9524  lfs f0, -0x6adc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287E1CC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8287E1D0: 4099003C  ble cr6, 0x8287e20c
	if !ctx.cr[6].gt {
	pc = 0x8287E20C; continue 'dispatch;
	}
	// 8287E1D4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8287E1D8: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 8287E1DC: 409A0030  bne cr6, 0x8287e20c
	if !ctx.cr[6].eq {
	pc = 0x8287E20C; continue 'dispatch;
	}
	// 8287E1E0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287E1E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E1E8: 808B0B10  lwz r4, 0xb10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2832 as u32) ) } as u64;
	// 8287E1EC: 4857581D  bl 0x82df3a08
	ctx.lr = 0x8287E1F0;
	sub_82DF3A08(ctx, base);
	// 8287E1F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E1F4: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8287E1F8: 488D6409  bl 0x83154600
	ctx.lr = 0x8287E1FC;
	sub_83154600(ctx, base);
	// 8287E1FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E200: 4BF71541  bl 0x827ef740
	ctx.lr = 0x8287E204;
	sub_827EF740(ctx, base);
	// 8287E204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E208: 48575221  bl 0x82df3428
	ctx.lr = 0x8287E20C;
	sub_82DF3428(ctx, base);
	// 8287E20C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287E21C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E228 size=96
    let mut pc: u32 = 0x8287E228;
    'dispatch: loop {
        match pc {
            0x8287E228 => {
    //   block [0x8287E228..0x8287E288)
	// 8287E228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E234: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E238: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E23C: 4BFFF9ED  bl 0x8287dc28
	ctx.lr = 0x8287E240;
	sub_8287DC28(ctx, base);
	// 8287E240: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E244: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 8287E248: 396B4294  addi r11, r11, 0x4294
	ctx.r[11].s64 = ctx.r[11].s64 + 17044;
	// 8287E24C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E250: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287E254: 808A95AC  lwz r4, -0x6a54(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27220 as u32) ) } as u64;
	// 8287E258: 485757B1  bl 0x82df3a08
	ctx.lr = 0x8287E25C;
	sub_82DF3A08(ctx, base);
	// 8287E25C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E260: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E264: 485DB485  bl 0x82e596e8
	ctx.lr = 0x8287E268;
	sub_82E596E8(ctx, base);
	// 8287E268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E26C: 485751BD  bl 0x82df3428
	ctx.lr = 0x8287E270;
	sub_82DF3428(ctx, base);
	// 8287E270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E288 size=220
    let mut pc: u32 = 0x8287E288;
    'dispatch: loop {
        match pc {
            0x8287E288 => {
    //   block [0x8287E288..0x8287E364)
	// 8287E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E28C: 48929EE1  bl 0x831a816c
	ctx.lr = 0x8287E290;
	sub_831A8130(ctx, base);
	// 8287E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287E298: 488D6369  bl 0x83154600
	ctx.lr = 0x8287E29C;
	sub_83154600(ctx, base);
	// 8287E29C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E2A0: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287E2A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E2A8: 4E800421  bctrl
	ctx.lr = 0x8287E2AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E2AC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E2B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E2B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E2B8: 808B9524  lwz r4, -0x6adc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) } as u64;
	// 8287E2BC: 4857574D  bl 0x82df3a08
	ctx.lr = 0x8287E2C0;
	sub_82DF3A08(ctx, base);
	// 8287E2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E2C4: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8287E2C8: 4BF6BFE1  bl 0x827ea2a8
	ctx.lr = 0x8287E2CC;
	sub_827EA2A8(ctx, base);
	// 8287E2CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287E2D0: 48575039  bl 0x82df3308
	ctx.lr = 0x8287E2D4;
	sub_82DF3308(ctx, base);
	// 8287E2D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287E2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E2DC: 4857514D  bl 0x82df3428
	ctx.lr = 0x8287E2E0;
	sub_82DF3428(ctx, base);
	// 8287E2E0: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E2E4: 4082003C  bne 0x8287e320
	if !ctx.cr[0].eq {
	pc = 0x8287E320; continue 'dispatch;
	}
	// 8287E2E8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E2EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E2F0: 808B9528  lwz r4, -0x6ad8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) } as u64;
	// 8287E2F4: 48575715  bl 0x82df3a08
	ctx.lr = 0x8287E2F8;
	sub_82DF3A08(ctx, base);
	// 8287E2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E2FC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8287E300: 4BF6BFA9  bl 0x827ea2a8
	ctx.lr = 0x8287E304;
	sub_827EA2A8(ctx, base);
	// 8287E304: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287E308: 48575001  bl 0x82df3308
	ctx.lr = 0x8287E30C;
	sub_82DF3308(ctx, base);
	// 8287E30C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287E310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E314: 48575115  bl 0x82df3428
	ctx.lr = 0x8287E318;
	sub_82DF3428(ctx, base);
	// 8287E318: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E31C: 41820040  beq 0x8287e35c
	if ctx.cr[0].eq {
	pc = 0x8287E35C; continue 'dispatch;
	}
	// 8287E320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E324: 4BF6BFED  bl 0x827ea310
	ctx.lr = 0x8287E328;
	sub_827EA310(ctx, base);
	// 8287E328: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E32C: 41820030  beq 0x8287e35c
	if ctx.cr[0].eq {
	pc = 0x8287E35C; continue 'dispatch;
	}
	// 8287E330: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287E334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E338: 808B0B24  lwz r4, 0xb24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) } as u64;
	// 8287E33C: 485756CD  bl 0x82df3a08
	ctx.lr = 0x8287E340;
	sub_82DF3A08(ctx, base);
	// 8287E340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E344: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8287E348: 488D62B9  bl 0x83154600
	ctx.lr = 0x8287E34C;
	sub_83154600(ctx, base);
	// 8287E34C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E350: 4BF713F1  bl 0x827ef740
	ctx.lr = 0x8287E354;
	sub_827EF740(ctx, base);
	// 8287E354: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E358: 485750D1  bl 0x82df3428
	ctx.lr = 0x8287E35C;
	sub_82DF3428(ctx, base);
	// 8287E35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8287E360: 48929E5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E368 size=96
    let mut pc: u32 = 0x8287E368;
    'dispatch: loop {
        match pc {
            0x8287E368 => {
    //   block [0x8287E368..0x8287E3C8)
	// 8287E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E370: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E374: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E378: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E37C: 4BFFF8AD  bl 0x8287dc28
	ctx.lr = 0x8287E380;
	sub_8287DC28(ctx, base);
	// 8287E380: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E384: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 8287E388: 396B42BC  addi r11, r11, 0x42bc
	ctx.r[11].s64 = ctx.r[11].s64 + 17084;
	// 8287E38C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E390: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287E394: 808A95B0  lwz r4, -0x6a50(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27216 as u32) ) } as u64;
	// 8287E398: 48575671  bl 0x82df3a08
	ctx.lr = 0x8287E39C;
	sub_82DF3A08(ctx, base);
	// 8287E39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E3A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E3A4: 485DB345  bl 0x82e596e8
	ctx.lr = 0x8287E3A8;
	sub_82E596E8(ctx, base);
	// 8287E3A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E3AC: 4857507D  bl 0x82df3428
	ctx.lr = 0x8287E3B0;
	sub_82DF3428(ctx, base);
	// 8287E3B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E3B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E3C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E3C8 size=220
    let mut pc: u32 = 0x8287E3C8;
    'dispatch: loop {
        match pc {
            0x8287E3C8 => {
    //   block [0x8287E3C8..0x8287E4A4)
	// 8287E3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E3CC: 48929DA1  bl 0x831a816c
	ctx.lr = 0x8287E3D0;
	sub_831A8130(ctx, base);
	// 8287E3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E3D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287E3D8: 488D6229  bl 0x83154600
	ctx.lr = 0x8287E3DC;
	sub_83154600(ctx, base);
	// 8287E3DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E3E0: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287E3E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E3E8: 4E800421  bctrl
	ctx.lr = 0x8287E3EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E3EC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E3F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E3F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E3F8: 808B951C  lwz r4, -0x6ae4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27364 as u32) ) } as u64;
	// 8287E3FC: 4857560D  bl 0x82df3a08
	ctx.lr = 0x8287E400;
	sub_82DF3A08(ctx, base);
	// 8287E400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E404: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8287E408: 4BF6BEA1  bl 0x827ea2a8
	ctx.lr = 0x8287E40C;
	sub_827EA2A8(ctx, base);
	// 8287E40C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287E410: 48574EF9  bl 0x82df3308
	ctx.lr = 0x8287E414;
	sub_82DF3308(ctx, base);
	// 8287E414: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287E418: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E41C: 4857500D  bl 0x82df3428
	ctx.lr = 0x8287E420;
	sub_82DF3428(ctx, base);
	// 8287E420: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E424: 4082003C  bne 0x8287e460
	if !ctx.cr[0].eq {
	pc = 0x8287E460; continue 'dispatch;
	}
	// 8287E428: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E42C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E430: 808B9520  lwz r4, -0x6ae0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27360 as u32) ) } as u64;
	// 8287E434: 485755D5  bl 0x82df3a08
	ctx.lr = 0x8287E438;
	sub_82DF3A08(ctx, base);
	// 8287E438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E43C: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 8287E440: 4BF6BE69  bl 0x827ea2a8
	ctx.lr = 0x8287E444;
	sub_827EA2A8(ctx, base);
	// 8287E444: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287E448: 48574EC1  bl 0x82df3308
	ctx.lr = 0x8287E44C;
	sub_82DF3308(ctx, base);
	// 8287E44C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287E450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E454: 48574FD5  bl 0x82df3428
	ctx.lr = 0x8287E458;
	sub_82DF3428(ctx, base);
	// 8287E458: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E45C: 41820040  beq 0x8287e49c
	if ctx.cr[0].eq {
	pc = 0x8287E49C; continue 'dispatch;
	}
	// 8287E460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E464: 4BF6BEAD  bl 0x827ea310
	ctx.lr = 0x8287E468;
	sub_827EA310(ctx, base);
	// 8287E468: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E46C: 41820030  beq 0x8287e49c
	if ctx.cr[0].eq {
	pc = 0x8287E49C; continue 'dispatch;
	}
	// 8287E470: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287E474: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E478: 808B0B24  lwz r4, 0xb24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) } as u64;
	// 8287E47C: 4857558D  bl 0x82df3a08
	ctx.lr = 0x8287E480;
	sub_82DF3A08(ctx, base);
	// 8287E480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E484: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8287E488: 488D6179  bl 0x83154600
	ctx.lr = 0x8287E48C;
	sub_83154600(ctx, base);
	// 8287E48C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E490: 4BF712B1  bl 0x827ef740
	ctx.lr = 0x8287E494;
	sub_827EF740(ctx, base);
	// 8287E494: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E498: 48574F91  bl 0x82df3428
	ctx.lr = 0x8287E49C;
	sub_82DF3428(ctx, base);
	// 8287E49C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8287E4A0: 48929D1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287E4A8 size=168
    let mut pc: u32 = 0x8287E4A8;
    'dispatch: loop {
        match pc {
            0x8287E4A8 => {
    //   block [0x8287E4A8..0x8287E550)
	// 8287E4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E4B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E4B4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8287E4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E4BC: 488D6145  bl 0x83154600
	ctx.lr = 0x8287E4C0;
	sub_83154600(ctx, base);
	// 8287E4C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E4C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E4C8: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287E4CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E4D0: 4E800421  bctrl
	ctx.lr = 0x8287E4D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E4D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287E4D8: C3EB08A8  lfs f31, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8287E4DC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287E4E0: 4BF6BE89  bl 0x827ea368
	ctx.lr = 0x8287E4E4;
	sub_827EA368(ctx, base);
	// 8287E4E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287E4E8: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8287E4EC: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8287E4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E4F4: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8287E4F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E4FC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287E500: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8287E504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E508: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287E50C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E510: 4E800421  bctrl
	ctx.lr = 0x8287E514;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E518: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8287E51C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287E520: 48098B01  bl 0x82917020
	ctx.lr = 0x8287E524;
	sub_82917020(ctx, base);
	// 8287E524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E528: 4BF6F2F1  bl 0x827ed818
	ctx.lr = 0x8287E52C;
	sub_827ED818(ctx, base);
	// 8287E52C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287E530: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8287E534: 4BFF6715  bl 0x82874c48
	ctx.lr = 0x8287E538;
	sub_82874C48(ctx, base);
	// 8287E538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8287E53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E544: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287E548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E550 size=96
    let mut pc: u32 = 0x8287E550;
    'dispatch: loop {
        match pc {
            0x8287E550 => {
    //   block [0x8287E550..0x8287E5B0)
	// 8287E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E55C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E564: 4BFFF6C5  bl 0x8287dc28
	ctx.lr = 0x8287E568;
	sub_8287DC28(ctx, base);
	// 8287E568: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E56C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 8287E570: 396B42E4  addi r11, r11, 0x42e4
	ctx.r[11].s64 = ctx.r[11].s64 + 17124;
	// 8287E574: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E578: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287E57C: 808A95B4  lwz r4, -0x6a4c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27212 as u32) ) } as u64;
	// 8287E580: 48575489  bl 0x82df3a08
	ctx.lr = 0x8287E584;
	sub_82DF3A08(ctx, base);
	// 8287E584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E588: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E58C: 485DB15D  bl 0x82e596e8
	ctx.lr = 0x8287E590;
	sub_82E596E8(ctx, base);
	// 8287E590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E594: 48574E95  bl 0x82df3428
	ctx.lr = 0x8287E598;
	sub_82DF3428(ctx, base);
	// 8287E598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E59C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E5A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E5B0 size=180
    let mut pc: u32 = 0x8287E5B0;
    'dispatch: loop {
        match pc {
            0x8287E5B0 => {
    //   block [0x8287E5B0..0x8287E664)
	// 8287E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E5B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E5BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E5C0: 488D6041  bl 0x83154600
	ctx.lr = 0x8287E5C4;
	sub_83154600(ctx, base);
	// 8287E5C4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E5C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E5CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E5D0: 808B9510  lwz r4, -0x6af0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27376 as u32) ) } as u64;
	// 8287E5D4: 48575435  bl 0x82df3a08
	ctx.lr = 0x8287E5D8;
	sub_82DF3A08(ctx, base);
	// 8287E5D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E5DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E5E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E5E4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8287E5E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E5EC: 4E800421  bctrl
	ctx.lr = 0x8287E5F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E5F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E5F4: 48574E35  bl 0x82df3428
	ctx.lr = 0x8287E5F8;
	sub_82DF3428(ctx, base);
	// 8287E5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E5FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287E600: 48013F89  bl 0x82892588
	ctx.lr = 0x8287E604;
	sub_82892588(ctx, base);
	// 8287E604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E608: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287E60C: 48013F95  bl 0x828925a0
	ctx.lr = 0x8287E610;
	sub_828925A0(ctx, base);
	// 8287E610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E614: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8287E618: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287E61C: 48098A05  bl 0x82917020
	ctx.lr = 0x8287E620;
	sub_82917020(ctx, base);
	// 8287E620: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287E624: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E628: 388B5D2C  addi r4, r11, 0x5d2c
	ctx.r[4].s64 = ctx.r[11].s64 + 23852;
	// 8287E62C: 485753DD  bl 0x82df3a08
	ctx.lr = 0x8287E630;
	sub_82DF3A08(ctx, base);
	// 8287E630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E634: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E63C: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8287E640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E644: 4E800421  bctrl
	ctx.lr = 0x8287E648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E64C: 48574DDD  bl 0x82df3428
	ctx.lr = 0x8287E650;
	sub_82DF3428(ctx, base);
	// 8287E650: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E65C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E668 size=80
    let mut pc: u32 = 0x8287E668;
    'dispatch: loop {
        match pc {
            0x8287E668 => {
    //   block [0x8287E668..0x8287E6B8)
	// 8287E668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E67C: 488D5F85  bl 0x83154600
	ctx.lr = 0x8287E680;
	sub_83154600(ctx, base);
	// 8287E680: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287E684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8287E688: 48098999  bl 0x82917020
	ctx.lr = 0x8287E68C;
	sub_82917020(ctx, base);
	// 8287E68C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E690: 488D5F71  bl 0x83154600
	ctx.lr = 0x8287E694;
	sub_83154600(ctx, base);
	// 8287E694: 4BF6F185  bl 0x827ed818
	ctx.lr = 0x8287E698;
	sub_827ED818(ctx, base);
	// 8287E698: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287E69C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8287E6A0: 4BFF65A9  bl 0x82874c48
	ctx.lr = 0x8287E6A4;
	sub_82874C48(ctx, base);
	// 8287E6A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287E6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E6B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E6B8 size=96
    let mut pc: u32 = 0x8287E6B8;
    'dispatch: loop {
        match pc {
            0x8287E6B8 => {
    //   block [0x8287E6B8..0x8287E718)
	// 8287E6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E6C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E6C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E6C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E6CC: 4BFFF55D  bl 0x8287dc28
	ctx.lr = 0x8287E6D0;
	sub_8287DC28(ctx, base);
	// 8287E6D0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E6D4: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 8287E6D8: 396B430C  addi r11, r11, 0x430c
	ctx.r[11].s64 = ctx.r[11].s64 + 17164;
	// 8287E6DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E6E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287E6E4: 808A95B8  lwz r4, -0x6a48(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27208 as u32) ) } as u64;
	// 8287E6E8: 48575321  bl 0x82df3a08
	ctx.lr = 0x8287E6EC;
	sub_82DF3A08(ctx, base);
	// 8287E6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E6F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E6F4: 485DAFF5  bl 0x82e596e8
	ctx.lr = 0x8287E6F8;
	sub_82E596E8(ctx, base);
	// 8287E6F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E6FC: 48574D2D  bl 0x82df3428
	ctx.lr = 0x8287E700;
	sub_82DF3428(ctx, base);
	// 8287E700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287E718 size=104
    let mut pc: u32 = 0x8287E718;
    'dispatch: loop {
        match pc {
            0x8287E718 => {
    //   block [0x8287E718..0x8287E780)
	// 8287E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E724: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E72C: 4BFFF4FD  bl 0x8287dc28
	ctx.lr = 0x8287E730;
	sub_8287DC28(ctx, base);
	// 8287E730: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287E734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8287E738: 396B4334  addi r11, r11, 0x4334
	ctx.r[11].s64 = ctx.r[11].s64 + 17204;
	// 8287E73C: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8287E740: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8287E744: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287E748: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E74C: 808A0B28  lwz r4, 0xb28(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2856 as u32) ) } as u64;
	// 8287E750: 485752B9  bl 0x82df3a08
	ctx.lr = 0x8287E754;
	sub_82DF3A08(ctx, base);
	// 8287E754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E758: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E75C: 485DAF8D  bl 0x82e596e8
	ctx.lr = 0x8287E760;
	sub_82E596E8(ctx, base);
	// 8287E760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E764: 48574CC5  bl 0x82df3428
	ctx.lr = 0x8287E768;
	sub_82DF3428(ctx, base);
	// 8287E768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287E770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287E780 size=292
    let mut pc: u32 = 0x8287E780;
    'dispatch: loop {
        match pc {
            0x8287E780 => {
    //   block [0x8287E780..0x8287E8A4)
	// 8287E780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287E788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287E78C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287E790: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8287E794: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287E79C: 488D5E65  bl 0x83154600
	ctx.lr = 0x8287E7A0;
	sub_83154600(ctx, base);
	// 8287E7A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E7A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E7A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E7AC: 808B958C  lwz r4, -0x6a74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27252 as u32) ) } as u64;
	// 8287E7B0: 48575259  bl 0x82df3a08
	ctx.lr = 0x8287E7B4;
	sub_82DF3A08(ctx, base);
	// 8287E7B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E7BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287E7C0: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8287E7C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E7C8: 4E800421  bctrl
	ctx.lr = 0x8287E7CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E7CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E7D0: 48574C59  bl 0x82df3428
	ctx.lr = 0x8287E7D4;
	sub_82DF3428(ctx, base);
	// 8287E7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E7D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287E7DC: 48013DAD  bl 0x82892588
	ctx.lr = 0x8287E7E0;
	sub_82892588(ctx, base);
	// 8287E7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E7E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287E7E8: 48013DB9  bl 0x828925a0
	ctx.lr = 0x8287E7EC;
	sub_828925A0(ctx, base);
	// 8287E7EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287E7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E7F4: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8287E7F8: 4BFFD049  bl 0x8287b840
	ctx.lr = 0x8287E7FC;
	sub_8287B840(ctx, base);
	// 8287E7FC: C0030088  lfs f0, 0x88(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287E800: D01E0060  stfs f0, 0x60(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8287E804: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E80C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8287E810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E814: 4E800421  bctrl
	ctx.lr = 0x8287E818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E818: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287E81C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287E820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E824: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287E828: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8287E82C: 40990014  ble cr6, 0x8287e840
	if !ctx.cr[6].gt {
	pc = 0x8287E840; continue 'dispatch;
	}
	// 8287E830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8287E834: 480987ED  bl 0x82917020
	ctx.lr = 0x8287E838;
	sub_82917020(ctx, base);
	// 8287E838: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8287E83C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E840: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8287E844: 480987DD  bl 0x82917020
	ctx.lr = 0x8287E848;
	sub_82917020(ctx, base);
	// 8287E848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E84C: 48097F1D  bl 0x82916768
	ctx.lr = 0x8287E850;
	sub_82916768(ctx, base);
	// 8287E850: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E854: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 8287E858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E85C: 4E800421  bctrl
	ctx.lr = 0x8287E860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E860: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287E864: 41820024  beq 0x8287e888
	if ctx.cr[0].eq {
	pc = 0x8287E888; continue 'dispatch;
	}
	// 8287E868: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E86C: C3E30054  lfs f31, 0x54(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8287E870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E874: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287E878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E87C: 4E800421  bctrl
	ctx.lr = 0x8287E880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E880: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8287E884: 4BF6BAE5  bl 0x827ea368
	ctx.lr = 0x8287E888;
	sub_827EA368(ctx, base);
	// 8287E888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8287E88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287E890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287E894: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8287E898: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287E89C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287E8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287E8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287E8A8 size=388
    let mut pc: u32 = 0x8287E8A8;
    'dispatch: loop {
        match pc {
            0x8287E8A8 => {
    //   block [0x8287E8A8..0x8287EA2C)
	// 8287E8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287E8AC: 489298C1  bl 0x831a816c
	ctx.lr = 0x8287E8B0;
	sub_831A8130(ctx, base);
	// 8287E8B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8287E8B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287E8B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287E8BC: 488D5D45  bl 0x83154600
	ctx.lr = 0x8287E8C0;
	sub_83154600(ctx, base);
	// 8287E8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E8C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E8C8: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287E8CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E8D0: 4E800421  bctrl
	ctx.lr = 0x8287E8D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E8D4: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8287E8D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287E8DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8287E8E0: 419800F0  blt cr6, 0x8287e9d0
	if ctx.cr[6].lt {
	pc = 0x8287E9D0; continue 'dispatch;
	}
	// 8287E8E4: 419A0048  beq cr6, 0x8287e92c
	if ctx.cr[6].eq {
	pc = 0x8287E92C; continue 'dispatch;
	}
	// 8287E8E8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8287E8EC: 40980134  bge cr6, 0x8287ea20
	if !ctx.cr[6].lt {
	pc = 0x8287EA20; continue 'dispatch;
	}
	// 8287E8F0: 4BF6BA21  bl 0x827ea310
	ctx.lr = 0x8287E8F4;
	sub_827EA310(ctx, base);
	// 8287E8F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287E8F8: 41820128  beq 0x8287ea20
	if ctx.cr[0].eq {
	pc = 0x8287EA20; continue 'dispatch;
	}
	// 8287E8FC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287E900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E904: 808B0B24  lwz r4, 0xb24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) } as u64;
	// 8287E908: 48575101  bl 0x82df3a08
	ctx.lr = 0x8287E90C;
	sub_82DF3A08(ctx, base);
	// 8287E90C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E910: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 8287E914: 488D5CED  bl 0x83154600
	ctx.lr = 0x8287E918;
	sub_83154600(ctx, base);
	// 8287E918: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E91C: 4BF70E25  bl 0x827ef740
	ctx.lr = 0x8287E920;
	sub_827EF740(ctx, base);
	// 8287E920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287E924: 48574B05  bl 0x82df3428
	ctx.lr = 0x8287E928;
	sub_82DF3428(ctx, base);
	// 8287E928: 480000F8  b 0x8287ea20
	pc = 0x8287EA20; continue 'dispatch;
	// 8287E92C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E934: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8287E938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E93C: 4E800421  bctrl
	ctx.lr = 0x8287E940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E940: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287E944: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8287E948: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8287E94C: 41990030  bgt cr6, 0x8287e97c
	if ctx.cr[6].gt {
	pc = 0x8287E97C; continue 'dispatch;
	}
	// 8287E950: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287E954: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287E958: 808B0B2C  lwz r4, 0xb2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2860 as u32) ) } as u64;
	// 8287E95C: 485750AD  bl 0x82df3a08
	ctx.lr = 0x8287E960;
	sub_82DF3A08(ctx, base);
	// 8287E960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E964: 3BE10054  addi r31, r1, 0x54
	ctx.r[31].s64 = ctx.r[1].s64 + 84;
	// 8287E968: 488D5C99  bl 0x83154600
	ctx.lr = 0x8287E96C;
	sub_83154600(ctx, base);
	// 8287E96C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E970: 4BF70DD1  bl 0x827ef740
	ctx.lr = 0x8287E974;
	sub_827EF740(ctx, base);
	// 8287E974: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287E978: 4BFFFFAC  b 0x8287e924
	pc = 0x8287E924; continue 'dispatch;
	// 8287E97C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287E980: 485DADF9  bl 0x82e59778
	ctx.lr = 0x8287E984;
	sub_82E59778(ctx, base);
	// 8287E984: C01E0060  lfs f0, 0x60(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287E988: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8287E98C: D01E0060  stfs f0, 0x60(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8287E990: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8287E994: 4098008C  bge cr6, 0x8287ea20
	if !ctx.cr[6].lt {
	pc = 0x8287EA20; continue 'dispatch;
	}
	// 8287E998: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287E99C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287E9A0: 808BF460  lwz r4, -0xba0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2976 as u32) ) } as u64;
	// 8287E9A4: 48575065  bl 0x82df3a08
	ctx.lr = 0x8287E9A8;
	sub_82DF3A08(ctx, base);
	// 8287E9A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287E9AC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8287E9B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287E9B4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8287E9B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287E9BC: 4E800421  bctrl
	ctx.lr = 0x8287E9C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287E9C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287E9C4: 48574A65  bl 0x82df3428
	ctx.lr = 0x8287E9C8;
	sub_82DF3428(ctx, base);
	// 8287E9C8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8287E9CC: 48000050  b 0x8287ea1c
	pc = 0x8287EA1C; continue 'dispatch;
	// 8287E9D0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287E9D4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8287E9D8: 808B958C  lwz r4, -0x6a74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27252 as u32) ) } as u64;
	// 8287E9DC: 4857502D  bl 0x82df3a08
	ctx.lr = 0x8287E9E0;
	sub_82DF3A08(ctx, base);
	// 8287E9E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287E9E4: 3BE1005C  addi r31, r1, 0x5c
	ctx.r[31].s64 = ctx.r[1].s64 + 92;
	// 8287E9E8: 4BF6B8C1  bl 0x827ea2a8
	ctx.lr = 0x8287E9EC;
	sub_827EA2A8(ctx, base);
	// 8287E9EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287E9F0: 48574919  bl 0x82df3308
	ctx.lr = 0x8287E9F4;
	sub_82DF3308(ctx, base);
	// 8287E9F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287E9F8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8287E9FC: 48574A2D  bl 0x82df3428
	ctx.lr = 0x8287EA00;
	sub_82DF3428(ctx, base);
	// 8287EA00: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287EA04: 4182001C  beq 0x8287ea20
	if ctx.cr[0].eq {
	pc = 0x8287EA20; continue 'dispatch;
	}
	// 8287EA08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287EA0C: 4BF6B905  bl 0x827ea310
	ctx.lr = 0x8287EA10;
	sub_827EA310(ctx, base);
	// 8287EA10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287EA14: 4182000C  beq 0x8287ea20
	if ctx.cr[0].eq {
	pc = 0x8287EA20; continue 'dispatch;
	}
	// 8287EA18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8287EA1C: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8287EA20: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8287EA24: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8287EA28: 48929794  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EA30 size=96
    let mut pc: u32 = 0x8287EA30;
    'dispatch: loop {
        match pc {
            0x8287EA30 => {
    //   block [0x8287EA30..0x8287EA90)
	// 8287EA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EA38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EA3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EA40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287EA44: 4BFFF1E5  bl 0x8287dc28
	ctx.lr = 0x8287EA48;
	sub_8287DC28(ctx, base);
	// 8287EA48: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EA4C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8287EA50: 396B435C  addi r11, r11, 0x435c
	ctx.r[11].s64 = ctx.r[11].s64 + 17244;
	// 8287EA54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EA58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EA5C: 808A0B34  lwz r4, 0xb34(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2868 as u32) ) } as u64;
	// 8287EA60: 48574FA9  bl 0x82df3a08
	ctx.lr = 0x8287EA64;
	sub_82DF3A08(ctx, base);
	// 8287EA64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EA68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287EA6C: 485DAC7D  bl 0x82e596e8
	ctx.lr = 0x8287EA70;
	sub_82E596E8(ctx, base);
	// 8287EA70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EA74: 485749B5  bl 0x82df3428
	ctx.lr = 0x8287EA78;
	sub_82DF3428(ctx, base);
	// 8287EA78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EA7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EA88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EA90 size=44
    let mut pc: u32 = 0x8287EA90;
    'dispatch: loop {
        match pc {
            0x8287EA90 => {
    //   block [0x8287EA90..0x8287EABC)
	// 8287EA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EA9C: 488D5B65  bl 0x83154600
	ctx.lr = 0x8287EAA0;
	sub_83154600(ctx, base);
	// 8287EAA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8287EAA4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287EAA8: 48098579  bl 0x82917020
	ctx.lr = 0x8287EAAC;
	sub_82917020(ctx, base);
	// 8287EAAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8287EAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EAC0 size=96
    let mut pc: u32 = 0x8287EAC0;
    'dispatch: loop {
        match pc {
            0x8287EAC0 => {
    //   block [0x8287EAC0..0x8287EB20)
	// 8287EAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EAC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EACC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EAD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287EAD4: 4BFFF155  bl 0x8287dc28
	ctx.lr = 0x8287EAD8;
	sub_8287DC28(ctx, base);
	// 8287EAD8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EADC: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8287EAE0: 396B4384  addi r11, r11, 0x4384
	ctx.r[11].s64 = ctx.r[11].s64 + 17284;
	// 8287EAE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EAE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EAEC: 808A0B3C  lwz r4, 0xb3c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2876 as u32) ) } as u64;
	// 8287EAF0: 48574F19  bl 0x82df3a08
	ctx.lr = 0x8287EAF4;
	sub_82DF3A08(ctx, base);
	// 8287EAF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EAF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287EAFC: 485DABED  bl 0x82e596e8
	ctx.lr = 0x8287EB00;
	sub_82E596E8(ctx, base);
	// 8287EB00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EB04: 48574925  bl 0x82df3428
	ctx.lr = 0x8287EB08;
	sub_82DF3428(ctx, base);
	// 8287EB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EB0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EB18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EB20 size=116
    let mut pc: u32 = 0x8287EB20;
    'dispatch: loop {
        match pc {
            0x8287EB20 => {
    //   block [0x8287EB20..0x8287EB94)
	// 8287EB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EB28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287EB2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EB30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EB34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287EB38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8287EB3C: 4BFFF0ED  bl 0x8287dc28
	ctx.lr = 0x8287EB40;
	sub_8287DC28(ctx, base);
	// 8287EB40: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EB44: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 8287EB48: 396B43AC  addi r11, r11, 0x43ac
	ctx.r[11].s64 = ctx.r[11].s64 + 17324;
	// 8287EB4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EB50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EB54: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8287EB58: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8287EB5C: 808A95A8  lwz r4, -0x6a58(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27224 as u32) ) } as u64;
	// 8287EB60: 48574EA9  bl 0x82df3a08
	ctx.lr = 0x8287EB64;
	sub_82DF3A08(ctx, base);
	// 8287EB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EB68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287EB6C: 485DAB7D  bl 0x82e596e8
	ctx.lr = 0x8287EB70;
	sub_82E596E8(ctx, base);
	// 8287EB70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EB74: 485748B5  bl 0x82df3428
	ctx.lr = 0x8287EB78;
	sub_82DF3428(ctx, base);
	// 8287EB78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EB7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EB88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287EB8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287EB98 size=264
    let mut pc: u32 = 0x8287EB98;
    'dispatch: loop {
        match pc {
            0x8287EB98 => {
    //   block [0x8287EB98..0x8287ECA0)
	// 8287EB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EBA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EBA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EBA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287EBAC: 488D5A55  bl 0x83154600
	ctx.lr = 0x8287EBB0;
	sub_83154600(ctx, base);
	// 8287EBB0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8287EBB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287EBB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287EBBC: 409A0068  bne cr6, 0x8287ec24
	if !ctx.cr[6].eq {
	pc = 0x8287EC24; continue 'dispatch;
	}
	// 8287EBC0: 48097BA9  bl 0x82916768
	ctx.lr = 0x8287EBC4;
	sub_82916768(ctx, base);
	// 8287EBC4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287EBC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EBCC: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287EBD0: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287EBD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EBD8: 4E800421  bctrl
	ctx.lr = 0x8287EBDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EBDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8287EBE0: C00B7BC8  lfs f0, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287EBE4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8287EBE8: 40980010  bge cr6, 0x8287ebf8
	if !ctx.cr[6].lt {
	pc = 0x8287EBF8; continue 'dispatch;
	}
	// 8287EBEC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EBF0: 808B9584  lwz r4, -0x6a7c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) } as u64;
	// 8287EBF4: 48000038  b 0x8287ec2c
	pc = 0x8287EC2C; continue 'dispatch;
	// 8287EBF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8287EBFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EC00: C00BC664  lfs f0, -0x399c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287EC04: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8287EC08: 40980010  bge cr6, 0x8287ec18
	if !ctx.cr[6].lt {
	pc = 0x8287EC18; continue 'dispatch;
	}
	// 8287EC0C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EC10: 808B9588  lwz r4, -0x6a78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27256 as u32) ) } as u64;
	// 8287EC14: 4800001C  b 0x8287ec30
	pc = 0x8287EC30; continue 'dispatch;
	// 8287EC18: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EC1C: 808B9580  lwz r4, -0x6a80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27264 as u32) ) } as u64;
	// 8287EC20: 48000010  b 0x8287ec30
	pc = 0x8287EC30; continue 'dispatch;
	// 8287EC24: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EC28: 808B9580  lwz r4, -0x6a80(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27264 as u32) ) } as u64;
	// 8287EC2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EC30: 48574DD9  bl 0x82df3a08
	ctx.lr = 0x8287EC34;
	sub_82DF3A08(ctx, base);
	// 8287EC34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EC3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287EC40: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8287EC44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EC48: 4E800421  bctrl
	ctx.lr = 0x8287EC4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EC4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EC50: 485747D9  bl 0x82df3428
	ctx.lr = 0x8287EC54;
	sub_82DF3428(ctx, base);
	// 8287EC54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EC58: 48097B11  bl 0x82916768
	ctx.lr = 0x8287EC5C;
	sub_82916768(ctx, base);
	// 8287EC5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EC60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287EC64: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8287EC68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EC6C: 4E800421  bctrl
	ctx.lr = 0x8287EC70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EC74: 48097AF5  bl 0x82916768
	ctx.lr = 0x8287EC78;
	sub_82916768(ctx, base);
	// 8287EC78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EC7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287EC80: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 8287EC84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EC88: 4E800421  bctrl
	ctx.lr = 0x8287EC8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EC8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EC98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287ECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287ECA0 size=196
    let mut pc: u32 = 0x8287ECA0;
    'dispatch: loop {
        match pc {
            0x8287ECA0 => {
    //   block [0x8287ECA0..0x8287ED64)
	// 8287ECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287ECA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287ECA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287ECAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287ECB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287ECB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287ECB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287ECBC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287ECC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287ECC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287ECC8: 4BA41C71  bl 0x822c0938
	ctx.lr = 0x8287ECCC;
	sub_822C0938(ctx, base);
	// 8287ECCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287ECD0: 41820028  beq 0x8287ecf8
	if ctx.cr[0].eq {
	pc = 0x8287ECF8; continue 'dispatch;
	}
	// 8287ECD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287ECD8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287ECDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287ECE0: 392B41CC  addi r9, r11, 0x41cc
	ctx.r[9].s64 = ctx.r[11].s64 + 16844;
	// 8287ECE4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287ECE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287ECEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287ECF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287ECF4: 48000008  b 0x8287ecfc
	pc = 0x8287ECFC; continue 'dispatch;
	// 8287ECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287ECFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287ED00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287ED04: 409A0044  bne cr6, 0x8287ed48
	if !ctx.cr[6].eq {
	pc = 0x8287ED48; continue 'dispatch;
	}
	// 8287ED08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287ED0C: 419A001C  beq cr6, 0x8287ed28
	if ctx.cr[6].eq {
	pc = 0x8287ED28; continue 'dispatch;
	}
	// 8287ED10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287ED14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287ED18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287ED1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287ED20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287ED24: 4E800421  bctrl
	ctx.lr = 0x8287ED28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287ED28: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287ED2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287ED30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287ED34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287ED38: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287ED3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287ED40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287ED44: 4BA412BD  bl 0x822c0000
	ctx.lr = 0x8287ED48;
	sub_822C0000(ctx, base);
	// 8287ED48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287ED4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287ED50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287ED54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287ED58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287ED5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287ED60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287ED68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287ED68 size=196
    let mut pc: u32 = 0x8287ED68;
    'dispatch: loop {
        match pc {
            0x8287ED68 => {
    //   block [0x8287ED68..0x8287EE2C)
	// 8287ED68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287ED6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287ED70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287ED74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287ED78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287ED7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287ED80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287ED84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287ED88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287ED8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287ED90: 4BA41BA9  bl 0x822c0938
	ctx.lr = 0x8287ED94;
	sub_822C0938(ctx, base);
	// 8287ED94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287ED98: 41820028  beq 0x8287edc0
	if ctx.cr[0].eq {
	pc = 0x8287EDC0; continue 'dispatch;
	}
	// 8287ED9C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EDA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287EDA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287EDA8: 392B41E0  addi r9, r11, 0x41e0
	ctx.r[9].s64 = ctx.r[11].s64 + 16864;
	// 8287EDAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287EDB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287EDB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287EDB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287EDBC: 48000008  b 0x8287edc4
	pc = 0x8287EDC4; continue 'dispatch;
	// 8287EDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287EDC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EDC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287EDCC: 409A0044  bne cr6, 0x8287ee10
	if !ctx.cr[6].eq {
	pc = 0x8287EE10; continue 'dispatch;
	}
	// 8287EDD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287EDD4: 419A001C  beq cr6, 0x8287edf0
	if ctx.cr[6].eq {
	pc = 0x8287EDF0; continue 'dispatch;
	}
	// 8287EDD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EDDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287EDE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EDE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287EDE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EDEC: 4E800421  bctrl
	ctx.lr = 0x8287EDF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EDF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EDF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287EDF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EDFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287EE00: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287EE04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287EE08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287EE0C: 4BA411F5  bl 0x822c0000
	ctx.lr = 0x8287EE10;
	sub_822C0000(ctx, base);
	// 8287EE10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287EE14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EE20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287EE24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EE30 size=196
    let mut pc: u32 = 0x8287EE30;
    'dispatch: loop {
        match pc {
            0x8287EE30 => {
    //   block [0x8287EE30..0x8287EEF4)
	// 8287EE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EE38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287EE3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EE40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EE44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287EE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287EE4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287EE50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287EE54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EE58: 4BA41AE1  bl 0x822c0938
	ctx.lr = 0x8287EE5C;
	sub_822C0938(ctx, base);
	// 8287EE5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287EE60: 41820028  beq 0x8287ee88
	if ctx.cr[0].eq {
	pc = 0x8287EE88; continue 'dispatch;
	}
	// 8287EE64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EE68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287EE6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287EE70: 392B41F4  addi r9, r11, 0x41f4
	ctx.r[9].s64 = ctx.r[11].s64 + 16884;
	// 8287EE74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287EE78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287EE7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287EE80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287EE84: 48000008  b 0x8287ee8c
	pc = 0x8287EE8C; continue 'dispatch;
	// 8287EE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287EE8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EE90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287EE94: 409A0044  bne cr6, 0x8287eed8
	if !ctx.cr[6].eq {
	pc = 0x8287EED8; continue 'dispatch;
	}
	// 8287EE98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287EE9C: 419A001C  beq cr6, 0x8287eeb8
	if ctx.cr[6].eq {
	pc = 0x8287EEB8; continue 'dispatch;
	}
	// 8287EEA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EEA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287EEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EEAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287EEB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EEB4: 4E800421  bctrl
	ctx.lr = 0x8287EEB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EEB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EEBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287EEC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EEC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287EEC8: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287EECC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287EED0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287EED4: 4BA4112D  bl 0x822c0000
	ctx.lr = 0x8287EED8;
	sub_822C0000(ctx, base);
	// 8287EED8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287EEDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EEE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287EEEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EEF8 size=196
    let mut pc: u32 = 0x8287EEF8;
    'dispatch: loop {
        match pc {
            0x8287EEF8 => {
    //   block [0x8287EEF8..0x8287EFBC)
	// 8287EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287EF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EF0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287EF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287EF14: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287EF18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287EF1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EF20: 4BA41A19  bl 0x822c0938
	ctx.lr = 0x8287EF24;
	sub_822C0938(ctx, base);
	// 8287EF24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287EF28: 41820028  beq 0x8287ef50
	if ctx.cr[0].eq {
	pc = 0x8287EF50; continue 'dispatch;
	}
	// 8287EF2C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EF30: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287EF34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287EF38: 392B4208  addi r9, r11, 0x4208
	ctx.r[9].s64 = ctx.r[11].s64 + 16904;
	// 8287EF3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287EF40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287EF44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287EF48: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287EF4C: 48000008  b 0x8287ef54
	pc = 0x8287EF54; continue 'dispatch;
	// 8287EF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287EF54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EF58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287EF5C: 409A0044  bne cr6, 0x8287efa0
	if !ctx.cr[6].eq {
	pc = 0x8287EFA0; continue 'dispatch;
	}
	// 8287EF60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287EF64: 419A001C  beq cr6, 0x8287ef80
	if ctx.cr[6].eq {
	pc = 0x8287EF80; continue 'dispatch;
	}
	// 8287EF68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287EF6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287EF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287EF74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287EF78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287EF7C: 4E800421  bctrl
	ctx.lr = 0x8287EF80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287EF80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287EF84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287EF88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287EF8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287EF90: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287EF94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287EF98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287EF9C: 4BA41065  bl 0x822c0000
	ctx.lr = 0x8287EFA0;
	sub_822C0000(ctx, base);
	// 8287EFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287EFA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287EFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287EFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287EFB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287EFB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287EFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287EFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287EFC0 size=196
    let mut pc: u32 = 0x8287EFC0;
    'dispatch: loop {
        match pc {
            0x8287EFC0 => {
    //   block [0x8287EFC0..0x8287F084)
	// 8287EFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287EFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287EFC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287EFCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287EFD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287EFD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287EFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287EFDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287EFE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287EFE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287EFE8: 4BA41951  bl 0x822c0938
	ctx.lr = 0x8287EFEC;
	sub_822C0938(ctx, base);
	// 8287EFEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287EFF0: 41820028  beq 0x8287f018
	if ctx.cr[0].eq {
	pc = 0x8287F018; continue 'dispatch;
	}
	// 8287EFF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287EFF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287EFFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287F000: 392B421C  addi r9, r11, 0x421c
	ctx.r[9].s64 = ctx.r[11].s64 + 16924;
	// 8287F004: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287F008: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F00C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287F010: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287F014: 48000008  b 0x8287f01c
	pc = 0x8287F01C; continue 'dispatch;
	// 8287F018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F01C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F020: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287F024: 409A0044  bne cr6, 0x8287f068
	if !ctx.cr[6].eq {
	pc = 0x8287F068; continue 'dispatch;
	}
	// 8287F028: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287F02C: 419A001C  beq cr6, 0x8287f048
	if ctx.cr[6].eq {
	pc = 0x8287F048; continue 'dispatch;
	}
	// 8287F030: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F034: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F03C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F044: 4E800421  bctrl
	ctx.lr = 0x8287F048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F048: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F04C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287F050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F054: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287F058: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287F05C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287F060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287F064: 4BA40F9D  bl 0x822c0000
	ctx.lr = 0x8287F068;
	sub_822C0000(ctx, base);
	// 8287F068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287F06C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287F074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287F078: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287F07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287F080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F088 size=196
    let mut pc: u32 = 0x8287F088;
    'dispatch: loop {
        match pc {
            0x8287F088 => {
    //   block [0x8287F088..0x8287F14C)
	// 8287F088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F09C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F0A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287F0A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287F0AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F0B0: 4BA41889  bl 0x822c0938
	ctx.lr = 0x8287F0B4;
	sub_822C0938(ctx, base);
	// 8287F0B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287F0B8: 41820028  beq 0x8287f0e0
	if ctx.cr[0].eq {
	pc = 0x8287F0E0; continue 'dispatch;
	}
	// 8287F0BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287F0C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287F0C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287F0C8: 392B4230  addi r9, r11, 0x4230
	ctx.r[9].s64 = ctx.r[11].s64 + 16944;
	// 8287F0CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287F0D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F0D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287F0D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287F0DC: 48000008  b 0x8287f0e4
	pc = 0x8287F0E4; continue 'dispatch;
	// 8287F0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F0E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F0E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287F0EC: 409A0044  bne cr6, 0x8287f130
	if !ctx.cr[6].eq {
	pc = 0x8287F130; continue 'dispatch;
	}
	// 8287F0F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287F0F4: 419A001C  beq cr6, 0x8287f110
	if ctx.cr[6].eq {
	pc = 0x8287F110; continue 'dispatch;
	}
	// 8287F0F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F0FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F104: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F10C: 4E800421  bctrl
	ctx.lr = 0x8287F110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F110: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F114: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287F118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F11C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287F120: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287F124: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287F128: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287F12C: 4BA40ED5  bl 0x822c0000
	ctx.lr = 0x8287F130;
	sub_822C0000(ctx, base);
	// 8287F130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287F134: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287F13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287F140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287F144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287F148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F150 size=196
    let mut pc: u32 = 0x8287F150;
    'dispatch: loop {
        match pc {
            0x8287F150 => {
    //   block [0x8287F150..0x8287F214)
	// 8287F150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F15C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F164: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F16C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287F170: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287F174: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F178: 4BA417C1  bl 0x822c0938
	ctx.lr = 0x8287F17C;
	sub_822C0938(ctx, base);
	// 8287F17C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287F180: 41820028  beq 0x8287f1a8
	if ctx.cr[0].eq {
	pc = 0x8287F1A8; continue 'dispatch;
	}
	// 8287F184: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287F188: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287F18C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287F190: 392B4244  addi r9, r11, 0x4244
	ctx.r[9].s64 = ctx.r[11].s64 + 16964;
	// 8287F194: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287F198: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F19C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287F1A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287F1A4: 48000008  b 0x8287f1ac
	pc = 0x8287F1AC; continue 'dispatch;
	// 8287F1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F1AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F1B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287F1B4: 409A0044  bne cr6, 0x8287f1f8
	if !ctx.cr[6].eq {
	pc = 0x8287F1F8; continue 'dispatch;
	}
	// 8287F1B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287F1BC: 419A001C  beq cr6, 0x8287f1d8
	if ctx.cr[6].eq {
	pc = 0x8287F1D8; continue 'dispatch;
	}
	// 8287F1C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F1C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F1CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F1D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F1D4: 4E800421  bctrl
	ctx.lr = 0x8287F1D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F1D8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F1DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287F1E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F1E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287F1E8: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287F1EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287F1F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287F1F4: 4BA40E0D  bl 0x822c0000
	ctx.lr = 0x8287F1F8;
	sub_822C0000(ctx, base);
	// 8287F1F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287F1FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287F204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287F208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287F20C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287F210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F218 size=196
    let mut pc: u32 = 0x8287F218;
    'dispatch: loop {
        match pc {
            0x8287F218 => {
    //   block [0x8287F218..0x8287F2DC)
	// 8287F218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F22C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F234: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287F238: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287F23C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F240: 4BA416F9  bl 0x822c0938
	ctx.lr = 0x8287F244;
	sub_822C0938(ctx, base);
	// 8287F244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287F248: 41820028  beq 0x8287f270
	if ctx.cr[0].eq {
	pc = 0x8287F270; continue 'dispatch;
	}
	// 8287F24C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287F250: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287F254: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287F258: 392B4258  addi r9, r11, 0x4258
	ctx.r[9].s64 = ctx.r[11].s64 + 16984;
	// 8287F25C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287F260: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F264: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287F268: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287F26C: 48000008  b 0x8287f274
	pc = 0x8287F274; continue 'dispatch;
	// 8287F270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F274: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287F27C: 409A0044  bne cr6, 0x8287f2c0
	if !ctx.cr[6].eq {
	pc = 0x8287F2C0; continue 'dispatch;
	}
	// 8287F280: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287F284: 419A001C  beq cr6, 0x8287f2a0
	if ctx.cr[6].eq {
	pc = 0x8287F2A0; continue 'dispatch;
	}
	// 8287F288: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F28C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F294: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F29C: 4E800421  bctrl
	ctx.lr = 0x8287F2A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F2A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F2A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287F2A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F2AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287F2B0: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287F2B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287F2B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287F2BC: 4BA40D45  bl 0x822c0000
	ctx.lr = 0x8287F2C0;
	sub_822C0000(ctx, base);
	// 8287F2C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287F2C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287F2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287F2D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287F2D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287F2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F2E0 size=196
    let mut pc: u32 = 0x8287F2E0;
    'dispatch: loop {
        match pc {
            0x8287F2E0 => {
    //   block [0x8287F2E0..0x8287F3A4)
	// 8287F2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F2E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F2EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F2F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F2F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F2FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287F300: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287F304: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F308: 4BA41631  bl 0x822c0938
	ctx.lr = 0x8287F30C;
	sub_822C0938(ctx, base);
	// 8287F30C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287F310: 41820028  beq 0x8287f338
	if ctx.cr[0].eq {
	pc = 0x8287F338; continue 'dispatch;
	}
	// 8287F314: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287F318: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287F31C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287F320: 392B426C  addi r9, r11, 0x426c
	ctx.r[9].s64 = ctx.r[11].s64 + 17004;
	// 8287F324: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287F328: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F32C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287F330: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287F334: 48000008  b 0x8287f33c
	pc = 0x8287F33C; continue 'dispatch;
	// 8287F338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F33C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F340: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287F344: 409A0044  bne cr6, 0x8287f388
	if !ctx.cr[6].eq {
	pc = 0x8287F388; continue 'dispatch;
	}
	// 8287F348: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287F34C: 419A001C  beq cr6, 0x8287f368
	if ctx.cr[6].eq {
	pc = 0x8287F368; continue 'dispatch;
	}
	// 8287F350: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F354: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F35C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F364: 4E800421  bctrl
	ctx.lr = 0x8287F368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F368: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F36C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287F370: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F374: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287F378: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287F37C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287F380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287F384: 4BA40C7D  bl 0x822c0000
	ctx.lr = 0x8287F388;
	sub_822C0000(ctx, base);
	// 8287F388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287F38C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287F398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287F39C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287F3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F3A8 size=196
    let mut pc: u32 = 0x8287F3A8;
    'dispatch: loop {
        match pc {
            0x8287F3A8 => {
    //   block [0x8287F3A8..0x8287F46C)
	// 8287F3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F3B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F3B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F3BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F3C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8287F3C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287F3CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F3D0: 4BA41569  bl 0x822c0938
	ctx.lr = 0x8287F3D4;
	sub_822C0938(ctx, base);
	// 8287F3D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287F3D8: 41820028  beq 0x8287f400
	if ctx.cr[0].eq {
	pc = 0x8287F400; continue 'dispatch;
	}
	// 8287F3DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287F3E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8287F3E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8287F3E8: 392B4280  addi r9, r11, 0x4280
	ctx.r[9].s64 = ctx.r[11].s64 + 17024;
	// 8287F3EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8287F3F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F3F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8287F3F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8287F3FC: 48000008  b 0x8287f404
	pc = 0x8287F404; continue 'dispatch;
	// 8287F400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8287F404: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287F408: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287F40C: 409A0044  bne cr6, 0x8287f450
	if !ctx.cr[6].eq {
	pc = 0x8287F450; continue 'dispatch;
	}
	// 8287F410: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8287F414: 419A001C  beq cr6, 0x8287f430
	if ctx.cr[6].eq {
	pc = 0x8287F430; continue 'dispatch;
	}
	// 8287F418: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F41C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F424: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F42C: 4E800421  bctrl
	ctx.lr = 0x8287F430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F430: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F434: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8287F438: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F43C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 8287F440: 816B994C  lwz r11, -0x66b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26292 as u32) ) } as u64;
	// 8287F444: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8287F448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8287F44C: 4BA40BB5  bl 0x822c0000
	ctx.lr = 0x8287F450;
	sub_822C0000(ctx, base);
	// 8287F450: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287F454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287F45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287F460: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8287F464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287F468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F470 size=108
    let mut pc: u32 = 0x8287F470;
    'dispatch: loop {
        match pc {
            0x8287F470 => {
    //   block [0x8287F470..0x8287F4DC)
	// 8287F470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F474: 48928CF9  bl 0x831a816c
	ctx.lr = 0x8287F478;
	sub_831A8130(ctx, base);
	// 8287F478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F47C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8287F480: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287F484: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8287F488: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287F48C: 41820038  beq 0x8287f4c4
	if ctx.cr[0].eq {
	pc = 0x8287F4C4; continue 'dispatch;
	}
	// 8287F490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F494: 4892A4F5  bl 0x831a9988
	ctx.lr = 0x8287F498;
	sub_831A9988(ctx, base);
	// 8287F498: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8287F49C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287F4A0: 386B6BDC  addi r3, r11, 0x6bdc
	ctx.r[3].s64 = ctx.r[11].s64 + 27612;
	// 8287F4A4: 48928C55  bl 0x831a80f8
	ctx.lr = 0x8287F4A8;
	sub_831A80F8(ctx, base);
	// 8287F4A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287F4AC: 41820018  beq 0x8287f4c4
	if ctx.cr[0].eq {
	pc = 0x8287F4C4; continue 'dispatch;
	}
	// 8287F4B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287F4B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287F4B8: 4BFFECE1  bl 0x8287e198
	ctx.lr = 0x8287F4BC;
	sub_8287E198(ctx, base);
	// 8287F4BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8287F4C0: 48000014  b 0x8287f4d4
	pc = 0x8287F4D4; continue 'dispatch;
	// 8287F4C4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8287F4C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287F4CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287F4D0: 4BFED821  bl 0x8286ccf0
	ctx.lr = 0x8287F4D4;
	sub_8286CCF0(ctx, base);
	// 8287F4D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287F4D8: 48928CE4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287F4E0 size=540
    let mut pc: u32 = 0x8287F4E0;
    'dispatch: loop {
        match pc {
            0x8287F4E0 => {
    //   block [0x8287F4E0..0x8287F6FC)
	// 8287F4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F4E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F4EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F4F0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8287F4F4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F4F8: 488D5109  bl 0x83154600
	ctx.lr = 0x8287F4FC;
	sub_83154600(ctx, base);
	// 8287F4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287F500: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8287F504: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287F508: 4BFFC351  bl 0x8287b858
	ctx.lr = 0x8287F50C;
	sub_8287B858(ctx, base);
	// 8287F50C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F510: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F518: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F51C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F520: 4E800421  bctrl
	ctx.lr = 0x8287F524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F524: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F528: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287F700 size=556
    let mut pc: u32 = 0x8287F700;
    'dispatch: loop {
        match pc {
            0x8287F700 => {
    //   block [0x8287F700..0x8287F92C)
	// 8287F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F708: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8287F70C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F710: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8287F714: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F718: 488D4EE9  bl 0x83154600
	ctx.lr = 0x8287F71C;
	sub_83154600(ctx, base);
	// 8287F71C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287F720: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F724: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8287F728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F72C: 4E800421  bctrl
	ctx.lr = 0x8287F730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F730: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287F734: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8287F738: 4BFFC121  bl 0x8287b858
	ctx.lr = 0x8287F73C;
	sub_8287B858(ctx, base);
	// 8287F73C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F740: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287F744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F748: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287F74C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F750: 4E800421  bctrl
	ctx.lr = 0x8287F754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F754: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F758: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287F930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287F930 size=224
    let mut pc: u32 = 0x8287F930;
    'dispatch: loop {
        match pc {
            0x8287F930 => {
    //   block [0x8287F930..0x8287FA10)
	// 8287F930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287F934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287F938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287F93C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287F940: 488D4CC1  bl 0x83154600
	ctx.lr = 0x8287F944;
	sub_83154600(ctx, base);
	// 8287F944: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287F948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287F94C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F950: 808B9550  lwz r4, -0x6ab0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27312 as u32) ) } as u64;
	// 8287F954: 485740B5  bl 0x82df3a08
	ctx.lr = 0x8287F958;
	sub_82DF3A08(ctx, base);
	// 8287F958: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F95C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F960: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287F964: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8287F968: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F96C: 4E800421  bctrl
	ctx.lr = 0x8287F970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F970: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287F974: 48573AB5  bl 0x82df3428
	ctx.lr = 0x8287F978;
	sub_82DF3428(ctx, base);
	// 8287F978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F97C: 48096DED  bl 0x82916768
	ctx.lr = 0x8287F980;
	sub_82916768(ctx, base);
	// 8287F980: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F984: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287F988: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8287F98C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F990: 4E800421  bctrl
	ctx.lr = 0x8287F994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F998: 48096DD1  bl 0x82916768
	ctx.lr = 0x8287F99C;
	sub_82916768(ctx, base);
	// 8287F99C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F9A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287F9A4: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 8287F9A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F9AC: 4E800421  bctrl
	ctx.lr = 0x8287F9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F9B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F9B4: 48096DB5  bl 0x82916768
	ctx.lr = 0x8287F9B8;
	sub_82916768(ctx, base);
	// 8287F9B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8287F9BC: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287F9C0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8287F9C4: 38AA43D0  addi r5, r10, 0x43d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17360;
	// 8287F9C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287F9CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287F9D0: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8287F9D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287F9D8: 4E800421  bctrl
	ctx.lr = 0x8287F9DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287F9DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287F9E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287F9E4: 419A0008  beq cr6, 0x8287f9ec
	if ctx.cr[6].eq {
	pc = 0x8287F9EC; continue 'dispatch;
	}
	// 8287F9E8: 4BA40EA9  bl 0x822c0890
	ctx.lr = 0x8287F9EC;
	sub_822C0890(ctx, base);
	// 8287F9EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8287F9F0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8287F9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287F9F8: 48097629  bl 0x82917020
	ctx.lr = 0x8287F9FC;
	sub_82917020(ctx, base);
	// 8287F9FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287FA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287FA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287FA08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287FA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287FA10 size=96
    let mut pc: u32 = 0x8287FA10;
    'dispatch: loop {
        match pc {
            0x8287FA10 => {
    //   block [0x8287FA10..0x8287FA70)
	// 8287FA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8287FA18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8287FA1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FA20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FA24: 4BFFE325  bl 0x8287dd48
	ctx.lr = 0x8287FA28;
	sub_8287DD48(ctx, base);
	// 8287FA28: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287FA2C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 8287FA30: 396B43EC  addi r11, r11, 0x43ec
	ctx.r[11].s64 = ctx.r[11].s64 + 17388;
	// 8287FA34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287FA38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8287FA3C: 808A0B30  lwz r4, 0xb30(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2864 as u32) ) } as u64;
	// 8287FA40: 48573FC9  bl 0x82df3a08
	ctx.lr = 0x8287FA44;
	sub_82DF3A08(ctx, base);
	// 8287FA44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FA48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287FA4C: 485D9C9D  bl 0x82e596e8
	ctx.lr = 0x8287FA50;
	sub_82E596E8(ctx, base);
	// 8287FA50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287FA54: 485739D5  bl 0x82df3428
	ctx.lr = 0x8287FA58;
	sub_82DF3428(ctx, base);
	// 8287FA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FA5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287FA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8287FA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8287FA68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8287FA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287FA70 size=364
    let mut pc: u32 = 0x8287FA70;
    'dispatch: loop {
        match pc {
            0x8287FA70 => {
    //   block [0x8287FA70..0x8287FBDC)
	// 8287FA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FA74: 489286F5  bl 0x831a8168
	ctx.lr = 0x8287FA78;
	sub_831A8130(ctx, base);
	// 8287FA78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FA7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287FA80: 488D4B81  bl 0x83154600
	ctx.lr = 0x8287FA84;
	sub_83154600(ctx, base);
	// 8287FA84: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8287FA88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FA8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287FA90: 808B9594  lwz r4, -0x6a6c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27244 as u32) ) } as u64;
	// 8287FA94: 48573F75  bl 0x82df3a08
	ctx.lr = 0x8287FA98;
	sub_82DF3A08(ctx, base);
	// 8287FA98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FA9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FAA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287FAA4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8287FAA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287FAAC: 4E800421  bctrl
	ctx.lr = 0x8287FAB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287FAB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287FAB4: 48573975  bl 0x82df3428
	ctx.lr = 0x8287FAB8;
	sub_82DF3428(ctx, base);
	// 8287FAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FABC: 48096CAD  bl 0x82916768
	ctx.lr = 0x8287FAC0;
	sub_82916768(ctx, base);
	// 8287FAC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FAC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8287FAC8: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8287FACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287FAD0: 4E800421  bctrl
	ctx.lr = 0x8287FAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287FAD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FAD8: 48096C91  bl 0x82916768
	ctx.lr = 0x8287FADC;
	sub_82916768(ctx, base);
	// 8287FADC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FAE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8287FAE4: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 8287FAE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287FAEC: 4E800421  bctrl
	ctx.lr = 0x8287FAF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287FAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FAF4: 48096C75  bl 0x82916768
	ctx.lr = 0x8287FAF8;
	sub_82916768(ctx, base);
	// 8287FAF8: 48089A91  bl 0x82909588
	ctx.lr = 0x8287FAFC;
	sub_82909588(ctx, base);
	// 8287FAFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FB00: 480C09A9  bl 0x829404a8
	ctx.lr = 0x8287FB04;
	sub_829404A8(ctx, base);
	// 8287FB04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287FB08: 480C24E1  bl 0x82941fe8
	ctx.lr = 0x8287FB0C;
	sub_82941FE8(ctx, base);
	// 8287FB0C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287FB10: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 8287FB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287FB18: 388A4418  addi r4, r10, 0x4418
	ctx.r[4].s64 = ctx.r[10].s64 + 17432;
	// 8287FB1C: 38A00065  li r5, 0x65
	ctx.r[5].s64 = 101;
	// 8287FB20: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8287FB24: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8287FB28: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8287FB2C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8287FB30: 4BA408A9  bl 0x822c03d8
	ctx.lr = 0x8287FB34;
	sub_822C03D8(ctx, base);
	// 8287FB34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287FB38: 41820018  beq 0x8287fb50
	if ctx.cr[0].eq {
	pc = 0x8287FB50; continue 'dispatch;
	}
	// 8287FB3C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8287FB40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287FB44: 480C24C5  bl 0x82942008
	ctx.lr = 0x8287FB48;
	sub_82942008(ctx, base);
	// 8287FB48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287FB4C: 48000008  b 0x8287fb54
	pc = 0x8287FB54; continue 'dispatch;
	// 8287FB50: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8287FB54: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8287FB58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8287FB5C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287FB60: 4BFEE2D1  bl 0x8286de30
	ctx.lr = 0x8287FB64;
	sub_8286DE30(ctx, base);
	// 8287FB64: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8287FB68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8287FB6C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8287FB70: 4BA40491  bl 0x822c0000
	ctx.lr = 0x8287FB74;
	sub_822C0000(ctx, base);
	// 8287FB74: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8287FB78: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8287FB7C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8287FB80: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8287FB84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8287FB88: 419A0024  beq cr6, 0x8287fbac
	if ctx.cr[6].eq {
	pc = 0x8287FBAC; continue 'dispatch;
	}
	// 8287FB8C: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8287FB90: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8287FB94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287FB98: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8287FB9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8287FBA0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8287FBA4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8287FBA8: 4082FFE8  bne 0x8287fb90
	if !ctx.cr[0].eq {
	pc = 0x8287FB90; continue 'dispatch;
	}
	// 8287FBAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287FBB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287FBB4: 480C08FD  bl 0x829404b0
	ctx.lr = 0x8287FBB8;
	sub_829404B0(ctx, base);
	// 8287FBB8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8287FBBC: 419A000C  beq cr6, 0x8287fbc8
	if ctx.cr[6].eq {
	pc = 0x8287FBC8; continue 'dispatch;
	}
	// 8287FBC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FBC4: 4BA40CCD  bl 0x822c0890
	ctx.lr = 0x8287FBC8;
	sub_822C0890(ctx, base);
	// 8287FBC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FBCC: 48096F35  bl 0x82916b00
	ctx.lr = 0x8287FBD0;
	sub_82916B00(ctx, base);
	// 8287FBD0: 987C0064  stb r3, 0x64(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(100 as u32), ctx.r[3].u8 ) };
	// 8287FBD4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8287FBD8: 489285E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8287FBE0 size=648
    let mut pc: u32 = 0x8287FBE0;
    'dispatch: loop {
        match pc {
            0x8287FBE0 => {
    //   block [0x8287FBE0..0x8287FE68)
	// 8287FBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FBE4: 48928585  bl 0x831a8168
	ctx.lr = 0x8287FBE8;
	sub_831A8130(ctx, base);
	// 8287FBE8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 8287FBEC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8287FBF0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FBF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8287FBF8: 488D4A09  bl 0x83154600
	ctx.lr = 0x8287FBFC;
	sub_83154600(ctx, base);
	// 8287FBFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FC00: 48096B69  bl 0x82916768
	ctx.lr = 0x8287FC04;
	sub_82916768(ctx, base);
	// 8287FC04: 48089985  bl 0x82909588
	ctx.lr = 0x8287FC08;
	sub_82909588(ctx, base);
	// 8287FC08: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8287FC0C: 480C0845  bl 0x82940450
	ctx.lr = 0x8287FC10;
	sub_82940450(ctx, base);
	// 8287FC10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FC14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8287FC18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287FC1C: 480C2AB5  bl 0x829426d0
	ctx.lr = 0x8287FC20;
	sub_829426D0(ctx, base);
	// 8287FC20: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FC24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FC28: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8287FC2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287FC30: 4E800421  bctrl
	ctx.lr = 0x8287FC34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287FC34: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8287FC38: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8287FC3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8287FC40: 419A000C  beq cr6, 0x8287fc4c
	if ctx.cr[6].eq {
	pc = 0x8287FC4C; continue 'dispatch;
	}
	// 8287FC44: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8287FC48: 4BA40C49  bl 0x822c0890
	ctx.lr = 0x8287FC4C;
	sub_822C0890(ctx, base);
	// 8287FC4C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287FC50: 4182002C  beq 0x8287fc7c
	if ctx.cr[0].eq {
	pc = 0x8287FC7C; continue 'dispatch;
	}
	// 8287FC54: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287FC58: 808B0B24  lwz r4, 0xb24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2852 as u32) ) } as u64;
	// 8287FC5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287FC60: 48573DA9  bl 0x82df3a08
	ctx.lr = 0x8287FC64;
	sub_82DF3A08(ctx, base);
	// 8287FC64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8287FC68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FC6C: 4BF6FAD5  bl 0x827ef740
	ctx.lr = 0x8287FC70;
	sub_827EF740(ctx, base);
	// 8287FC70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8287FC74: 485737B5  bl 0x82df3428
	ctx.lr = 0x8287FC78;
	sub_82DF3428(ctx, base);
	// 8287FC78: 480001E0  b 0x8287fe58
	pc = 0x8287FE58; continue 'dispatch;
	// 8287FC7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8287FC80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8287FC84: 480C2A4D  bl 0x829426d0
	ctx.lr = 0x8287FC88;
	sub_829426D0(ctx, base);
	// 8287FC88: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FC8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FC90: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8287FC94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287FC98: 4E800421  bctrl
	ctx.lr = 0x8287FC9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287FC9C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8287FCA0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8287FCA4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8287FCA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8287FCAC: 557DDFFE  rlwinm r29, r11, 0x1b, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8287FCB0: 419A0008  beq cr6, 0x8287fcb8
	if ctx.cr[6].eq {
	pc = 0x8287FCB8; continue 'dispatch;
	}
	// 8287FCB4: 4BA40BDD  bl 0x822c0890
	ctx.lr = 0x8287FCB8;
	sub_822C0890(ctx, base);
	// 8287FCB8: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8287FCBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FCC0: 41820024  beq 0x8287fce4
	if ctx.cr[0].eq {
	pc = 0x8287FCE4; continue 'dispatch;
	}
	// 8287FCC4: 48096AA5  bl 0x82916768
	ctx.lr = 0x8287FCC8;
	sub_82916768(ctx, base);
	// 8287FCC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8287FCCC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8287FCD0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8287FCD4: 4808F0A5  bl 0x8290ed78
	ctx.lr = 0x8287FCD8;
	sub_8290ED78(ctx, base);
	// 8287FCD8: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 8287FCDC: 808B0B10  lwz r4, 0xb10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2832 as u32) ) } as u64;
	// 8287FCE0: 4BFFFF7C  b 0x8287fc5c
	pc = 0x8287FC5C; continue 'dispatch;
	// 8287FCE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8287FCE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8287FCEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8287FCF0: 4E800421  bctrl
	ctx.lr = 0x8287FCF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8287FCF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FCF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8287FCFC: 48096A6D  bl 0x82916768
	ctx.lr = 0x8287FD00;
	sub_82916768(ctx, base);
	// 8287FD00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8287FD04: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8287FD08: 48089951  bl 0x82909658
	ctx.lr = 0x8287FD0C;
	sub_82909658(ctx, base);
	// 8287FD0C: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8287FD10: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8287FD14: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287FE68 size=112
    let mut pc: u32 = 0x8287FE68;
    'dispatch: loop {
        match pc {
            0x8287FE68 => {
    //   block [0x8287FE68..0x8287FED8)
	// 8287FE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FE6C: 48928301  bl 0x831a816c
	ctx.lr = 0x8287FE70;
	sub_831A8130(ctx, base);
	// 8287FE70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FE74: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287FE78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287FE80: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 8287FE84: 38A000DC  li r5, 0xdc
	ctx.r[5].s64 = 220;
	// 8287FE88: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 8287FE8C: 4857255D  bl 0x82df23e8
	ctx.lr = 0x8287FE90;
	sub_82DF23E8(ctx, base);
	// 8287FE90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287FE94: 41820010  beq 0x8287fea4
	if ctx.cr[0].eq {
	pc = 0x8287FEA4; continue 'dispatch;
	}
	// 8287FE98: 4BFFFB79  bl 0x8287fa10
	ctx.lr = 0x8287FE9C;
	sub_8287FA10(ctx, base);
	// 8287FE9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FEA0: 48000008  b 0x8287fea8
	pc = 0x8287FEA8; continue 'dispatch;
	// 8287FEA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287FEA8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287FEAC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287FEB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287FEB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FEB8: 4BFFEDE9  bl 0x8287eca0
	ctx.lr = 0x8287FEBC;
	sub_8287ECA0(ctx, base);
	// 8287FEBC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287FEC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287FEC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FEC8: 4BA40139  bl 0x822c0000
	ctx.lr = 0x8287FECC;
	sub_822C0000(ctx, base);
	// 8287FECC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287FED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287FED4: 489282E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287FED8 size=112
    let mut pc: u32 = 0x8287FED8;
    'dispatch: loop {
        match pc {
            0x8287FED8 => {
    //   block [0x8287FED8..0x8287FF48)
	// 8287FED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FEDC: 48928291  bl 0x831a816c
	ctx.lr = 0x8287FEE0;
	sub_831A8130(ctx, base);
	// 8287FEE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FEE4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287FEE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287FEF0: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 8287FEF4: 38A00135  li r5, 0x135
	ctx.r[5].s64 = 309;
	// 8287FEF8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8287FEFC: 485724ED  bl 0x82df23e8
	ctx.lr = 0x8287FF00;
	sub_82DF23E8(ctx, base);
	// 8287FF00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287FF04: 41820010  beq 0x8287ff14
	if ctx.cr[0].eq {
	pc = 0x8287FF14; continue 'dispatch;
	}
	// 8287FF08: 4BFFE321  bl 0x8287e228
	ctx.lr = 0x8287FF0C;
	sub_8287E228(ctx, base);
	// 8287FF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FF10: 48000008  b 0x8287ff18
	pc = 0x8287FF18; continue 'dispatch;
	// 8287FF14: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287FF18: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287FF1C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287FF20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287FF24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FF28: 4BFFEE41  bl 0x8287ed68
	ctx.lr = 0x8287FF2C;
	sub_8287ED68(ctx, base);
	// 8287FF2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287FF30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287FF34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FF38: 4BA400C9  bl 0x822c0000
	ctx.lr = 0x8287FF3C;
	sub_822C0000(ctx, base);
	// 8287FF3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287FF40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287FF44: 48928278  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287FF48 size=112
    let mut pc: u32 = 0x8287FF48;
    'dispatch: loop {
        match pc {
            0x8287FF48 => {
    //   block [0x8287FF48..0x8287FFB8)
	// 8287FF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FF4C: 48928221  bl 0x831a816c
	ctx.lr = 0x8287FF50;
	sub_831A8130(ctx, base);
	// 8287FF50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FF54: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287FF58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287FF60: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 8287FF64: 38A0018F  li r5, 0x18f
	ctx.r[5].s64 = 399;
	// 8287FF68: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8287FF6C: 4857247D  bl 0x82df23e8
	ctx.lr = 0x8287FF70;
	sub_82DF23E8(ctx, base);
	// 8287FF70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287FF74: 41820010  beq 0x8287ff84
	if ctx.cr[0].eq {
	pc = 0x8287FF84; continue 'dispatch;
	}
	// 8287FF78: 4BFFE3F1  bl 0x8287e368
	ctx.lr = 0x8287FF7C;
	sub_8287E368(ctx, base);
	// 8287FF7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FF80: 48000008  b 0x8287ff88
	pc = 0x8287FF88; continue 'dispatch;
	// 8287FF84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287FF88: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287FF8C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8287FF90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287FF94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FF98: 4BFFEE99  bl 0x8287ee30
	ctx.lr = 0x8287FF9C;
	sub_8287EE30(ctx, base);
	// 8287FF9C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8287FFA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8287FFA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8287FFA8: 4BA40059  bl 0x822c0000
	ctx.lr = 0x8287FFAC;
	sub_822C0000(ctx, base);
	// 8287FFAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8287FFB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8287FFB4: 48928208  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8287FFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8287FFB8 size=112
    let mut pc: u32 = 0x8287FFB8;
    'dispatch: loop {
        match pc {
            0x8287FFB8 => {
    //   block [0x8287FFB8..0x82880028)
	// 8287FFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8287FFBC: 489281B1  bl 0x831a816c
	ctx.lr = 0x8287FFC0;
	sub_831A8130(ctx, base);
	// 8287FFC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8287FFC4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8287FFC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8287FFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8287FFD0: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 8287FFD4: 38A001C9  li r5, 0x1c9
	ctx.r[5].s64 = 457;
	// 8287FFD8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8287FFDC: 4857240D  bl 0x82df23e8
	ctx.lr = 0x8287FFE0;
	sub_82DF23E8(ctx, base);
	// 8287FFE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8287FFE4: 41820010  beq 0x8287fff4
	if ctx.cr[0].eq {
	pc = 0x8287FFF4; continue 'dispatch;
	}
	// 8287FFE8: 4BFFE569  bl 0x8287e550
	ctx.lr = 0x8287FFEC;
	sub_8287E550(ctx, base);
	// 8287FFEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8287FFF0: 48000008  b 0x8287fff8
	pc = 0x8287FFF8; continue 'dispatch;
	// 8287FFF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8287FFF8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8287FFFC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82880000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880008: 4BFFEEF1  bl 0x8287eef8
	ctx.lr = 0x8288000C;
	sub_8287EEF8(ctx, base);
	// 8288000C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82880010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880018: 4BA3FFE9  bl 0x822c0000
	ctx.lr = 0x8288001C;
	sub_822C0000(ctx, base);
	// 8288001C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82880024: 48928198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880028 size=112
    let mut pc: u32 = 0x82880028;
    'dispatch: loop {
        match pc {
            0x82880028 => {
    //   block [0x82880028..0x82880098)
	// 82880028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288002C: 48928141  bl 0x831a816c
	ctx.lr = 0x82880030;
	sub_831A8130(ctx, base);
	// 82880030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880034: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82880038: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8288003C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82880040: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 82880044: 38A0020B  li r5, 0x20b
	ctx.r[5].s64 = 523;
	// 82880048: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 8288004C: 4857239D  bl 0x82df23e8
	ctx.lr = 0x82880050;
	sub_82DF23E8(ctx, base);
	// 82880050: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82880054: 41820010  beq 0x82880064
	if ctx.cr[0].eq {
	pc = 0x82880064; continue 'dispatch;
	}
	// 82880058: 4BFFE661  bl 0x8287e6b8
	ctx.lr = 0x8288005C;
	sub_8287E6B8(ctx, base);
	// 8288005C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880060: 48000008  b 0x82880068
	pc = 0x82880068; continue 'dispatch;
	// 82880064: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82880068: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8288006C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82880070: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880078: 4BFFEF49  bl 0x8287efc0
	ctx.lr = 0x8288007C;
	sub_8287EFC0(ctx, base);
	// 8288007C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82880080: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880088: 4BA3FF79  bl 0x822c0000
	ctx.lr = 0x8288008C;
	sub_822C0000(ctx, base);
	// 8288008C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82880094: 48928128  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880098 size=112
    let mut pc: u32 = 0x82880098;
    'dispatch: loop {
        match pc {
            0x82880098 => {
    //   block [0x82880098..0x82880108)
	// 82880098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288009C: 489280D1  bl 0x831a816c
	ctx.lr = 0x828800A0;
	sub_831A8130(ctx, base);
	// 828800A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828800A4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828800A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828800AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828800B0: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 828800B4: 38A00283  li r5, 0x283
	ctx.r[5].s64 = 643;
	// 828800B8: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 828800BC: 4857232D  bl 0x82df23e8
	ctx.lr = 0x828800C0;
	sub_82DF23E8(ctx, base);
	// 828800C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828800C4: 41820010  beq 0x828800d4
	if ctx.cr[0].eq {
	pc = 0x828800D4; continue 'dispatch;
	}
	// 828800C8: 4BFFE651  bl 0x8287e718
	ctx.lr = 0x828800CC;
	sub_8287E718(ctx, base);
	// 828800CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828800D0: 48000008  b 0x828800d8
	pc = 0x828800D8; continue 'dispatch;
	// 828800D4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828800D8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828800DC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 828800E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828800E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828800E8: 4BFFEFA1  bl 0x8287f088
	ctx.lr = 0x828800EC;
	sub_8287F088(ctx, base);
	// 828800EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828800F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828800F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828800F8: 4BA3FF09  bl 0x822c0000
	ctx.lr = 0x828800FC;
	sub_822C0000(ctx, base);
	// 828800FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82880104: 489280B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82880108 size=416
    let mut pc: u32 = 0x82880108;
    'dispatch: loop {
        match pc {
            0x82880108 => {
    //   block [0x82880108..0x828802A8)
	// 82880108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82880110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82880114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82880118: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828802A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828802A8 size=112
    let mut pc: u32 = 0x828802A8;
    'dispatch: loop {
        match pc {
            0x828802A8 => {
    //   block [0x828802A8..0x82880318)
	// 828802A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828802AC: 48927EC1  bl 0x831a816c
	ctx.lr = 0x828802B0;
	sub_831A8130(ctx, base);
	// 828802B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828802B4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828802B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828802BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828802C0: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 828802C4: 38A002D1  li r5, 0x2d1
	ctx.r[5].s64 = 721;
	// 828802C8: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 828802CC: 4857211D  bl 0x82df23e8
	ctx.lr = 0x828802D0;
	sub_82DF23E8(ctx, base);
	// 828802D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828802D4: 41820010  beq 0x828802e4
	if ctx.cr[0].eq {
	pc = 0x828802E4; continue 'dispatch;
	}
	// 828802D8: 4BFFE759  bl 0x8287ea30
	ctx.lr = 0x828802DC;
	sub_8287EA30(ctx, base);
	// 828802DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828802E0: 48000008  b 0x828802e8
	pc = 0x828802E8; continue 'dispatch;
	// 828802E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828802E8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828802EC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 828802F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828802F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828802F8: 4BFFEE59  bl 0x8287f150
	ctx.lr = 0x828802FC;
	sub_8287F150(ctx, base);
	// 828802FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82880300: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880308: 4BA3FCF9  bl 0x822c0000
	ctx.lr = 0x8288030C;
	sub_822C0000(ctx, base);
	// 8288030C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880310: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82880314: 48927EA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880318 size=108
    let mut pc: u32 = 0x82880318;
    'dispatch: loop {
        match pc {
            0x82880318 => {
    //   block [0x82880318..0x82880384)
	// 82880318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288031C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82880320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82880324: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8288032C: 4BFFD8FD  bl 0x8287dc28
	ctx.lr = 0x82880330;
	sub_8287DC28(ctx, base);
	// 82880330: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 82880334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82880338: 394A44B0  addi r10, r10, 0x44b0
	ctx.r[10].s64 = ctx.r[10].s64 + 17584;
	// 8288033C: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82880340: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82880344: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880348: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8288034C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82880350: 80890B38  lwz r4, 0xb38(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2872 as u32) ) } as u64;
	// 82880354: 485736B5  bl 0x82df3a08
	ctx.lr = 0x82880358;
	sub_82DF3A08(ctx, base);
	// 82880358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288035C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880360: 485D9389  bl 0x82e596e8
	ctx.lr = 0x82880364;
	sub_82E596E8(ctx, base);
	// 82880364: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880368: 485730C1  bl 0x82df3428
	ctx.lr = 0x8288036C;
	sub_82DF3428(ctx, base);
	// 8288036C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880370: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82880374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82880378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8288037C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82880380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82880388 size=584
    let mut pc: u32 = 0x82880388;
    'dispatch: loop {
        match pc {
            0x82880388 => {
    //   block [0x82880388..0x828805D0)
	// 82880388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288038C: 48927DDD  bl 0x831a8168
	ctx.lr = 0x82880390;
	sub_831A8130(ctx, base);
	// 82880390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880398: 488D4269  bl 0x83154600
	ctx.lr = 0x8288039C;
	sub_83154600(ctx, base);
	// 8288039C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828803A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828803A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828803A8: 808B953C  lwz r4, -0x6ac4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27332 as u32) ) } as u64;
	// 828803AC: 4857365D  bl 0x82df3a08
	ctx.lr = 0x828803B0;
	sub_82DF3A08(ctx, base);
	// 828803B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 828803B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828803B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828803BC: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 828803C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828803C4: 4E800421  bctrl
	ctx.lr = 0x828803C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828803C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828803CC: 4857305D  bl 0x82df3428
	ctx.lr = 0x828803D0;
	sub_82DF3428(ctx, base);
	// 828803D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828803D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828803D8: 480121B1  bl 0x82892588
	ctx.lr = 0x828803DC;
	sub_82892588(ctx, base);
	// 828803DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828803E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 828803E4: 480121BD  bl 0x828925a0
	ctx.lr = 0x828803E8;
	sub_828925A0(ctx, base);
	// 828803E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828803EC: 4BFFB455  bl 0x8287b840
	ctx.lr = 0x828803F0;
	sub_8287B840(ctx, base);
	// 828803F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828803F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828803F8: 4BFFB499  bl 0x8287b890
	ctx.lr = 0x828803FC;
	sub_8287B890(ctx, base);
	// 828803FC: C01D0180  lfs f0, 0x180(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(384 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82880400: C1BD017C  lfs f13, 0x17c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(380 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82880404: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880408: EC01683A  fmadds f0, f1, f0, f13
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8288040C: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82880410: D01F006C  stfs f0, 0x6c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82880414: 48096355  bl 0x82916768
	ctx.lr = 0x82880418;
	sub_82916768(ctx, base);
	// 82880418: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288041C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82880420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880424: 4E800421  bctrl
	ctx.lr = 0x82880428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8288042C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880430: 4BFFB411  bl 0x8287b840
	ctx.lr = 0x82880434;
	sub_8287B840(ctx, base);
	// 82880434: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82880438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288043C: 480BF955  bl 0x8293fd90
	ctx.lr = 0x82880440;
	sub_8293FD90(ctx, base);
	// 82880440: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82880444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82880448: 3BAB4418  addi r29, r11, 0x4418
	ctx.r[29].s64 = ctx.r[11].s64 + 17432;
	// 8288044C: 38A002FB  li r5, 0x2fb
	ctx.r[5].s64 = 763;
	// 82880450: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82880454: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82880458: 4BA3FF81  bl 0x822c03d8
	ctx.lr = 0x8288045C;
	sub_822C03D8(ctx, base);
	// 8288045C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82880460: 41820018  beq 0x82880478
	if ctx.cr[0].eq {
	pc = 0x82880478; continue 'dispatch;
	}
	// 82880464: 38BC013C  addi r5, r28, 0x13c
	ctx.r[5].s64 = ctx.r[28].s64 + 316;
	// 82880468: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288046C: 480C0B95  bl 0x82941000
	ctx.lr = 0x82880470;
	sub_82941000(ctx, base);
	// 82880470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880474: 48000008  b 0x8288047c
	pc = 0x8288047C; continue 'dispatch;
	// 82880478: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8288047C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82880480: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82880484: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82880488: 4BFF18A1  bl 0x82871d28
	ctx.lr = 0x8288048C;
	sub_82871D28(ctx, base);
	// 8288048C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82880490: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82880494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82880498: 4BA3FB69  bl 0x822c0000
	ctx.lr = 0x8288049C;
	sub_822C0000(ctx, base);
	// 8288049C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828804A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828804A4: 480BF9FD  bl 0x8293fea0
	ctx.lr = 0x828804A8;
	sub_8293FEA0(ctx, base);
	// 828804A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 828804AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828804B0: 38A002FD  li r5, 0x2fd
	ctx.r[5].s64 = 765;
	// 828804B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828804B8: 4BA3FF21  bl 0x822c03d8
	ctx.lr = 0x828804BC;
	sub_822C03D8(ctx, base);
	// 828804BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828804C0: 41820018  beq 0x828804d8
	if ctx.cr[0].eq {
	pc = 0x828804D8; continue 'dispatch;
	}
	// 828804C4: 38BC0148  addi r5, r28, 0x148
	ctx.r[5].s64 = ctx.r[28].s64 + 328;
	// 828804C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828804CC: 480C2AA5  bl 0x82942f70
	ctx.lr = 0x828804D0;
	sub_82942F70(ctx, base);
	// 828804D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828804D4: 48000008  b 0x828804dc
	pc = 0x828804DC; continue 'dispatch;
	// 828804D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 828804DC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 828804E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828804E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828804E8: 4BFF1909  bl 0x82871df0
	ctx.lr = 0x828804EC;
	sub_82871DF0(ctx, base);
	// 828804EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 828804F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828804F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828804F8: 4BA3FB09  bl 0x822c0000
	ctx.lr = 0x828804FC;
	sub_822C0000(ctx, base);
	// 828804FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880504: 480BF99D  bl 0x8293fea0
	ctx.lr = 0x82880508;
	sub_8293FEA0(ctx, base);
	// 82880508: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8288050C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82880510: 38A002FF  li r5, 0x2ff
	ctx.r[5].s64 = 767;
	// 82880514: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82880518: 4BA3FEC1  bl 0x822c03d8
	ctx.lr = 0x8288051C;
	sub_822C03D8(ctx, base);
	// 8288051C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82880520: 41820018  beq 0x82880538
	if ctx.cr[0].eq {
	pc = 0x82880538; continue 'dispatch;
	}
	// 82880524: 38BC0150  addi r5, r28, 0x150
	ctx.r[5].s64 = ctx.r[28].s64 + 336;
	// 82880528: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288052C: 480C2E5D  bl 0x82943388
	ctx.lr = 0x82880530;
	sub_82943388(ctx, base);
	// 82880530: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880534: 48000008  b 0x8288053c
	pc = 0x8288053C; continue 'dispatch;
	// 82880538: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8288053C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82880540: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82880544: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82880548: 4BFF1971  bl 0x82871eb8
	ctx.lr = 0x8288054C;
	sub_82871EB8(ctx, base);
	// 8288054C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82880550: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82880554: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82880558: 4BA3FAA9  bl 0x822c0000
	ctx.lr = 0x8288055C;
	sub_822C0000(ctx, base);
	// 8288055C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880564: 480BF93D  bl 0x8293fea0
	ctx.lr = 0x82880568;
	sub_8293FEA0(ctx, base);
	// 82880568: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8288056C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82880570: 38A00301  li r5, 0x301
	ctx.r[5].s64 = 769;
	// 82880574: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82880578: 4BA3FE61  bl 0x822c03d8
	ctx.lr = 0x8288057C;
	sub_822C03D8(ctx, base);
	// 8288057C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82880580: 41820018  beq 0x82880598
	if ctx.cr[0].eq {
	pc = 0x82880598; continue 'dispatch;
	}
	// 82880584: 38BC0160  addi r5, r28, 0x160
	ctx.r[5].s64 = ctx.r[28].s64 + 352;
	// 82880588: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288058C: 480C2395  bl 0x82942920
	ctx.lr = 0x82880590;
	sub_82942920(ctx, base);
	// 82880590: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880594: 48000008  b 0x8288059c
	pc = 0x8288059C; continue 'dispatch;
	// 82880598: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8288059C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 828805A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828805A4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828805A8: 4BFF19D9  bl 0x82871f80
	ctx.lr = 0x828805AC;
	sub_82871F80(ctx, base);
	// 828805AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 828805B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828805B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828805B8: 4BA3FA49  bl 0x822c0000
	ctx.lr = 0x828805BC;
	sub_822C0000(ctx, base);
	// 828805BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828805C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828805C4: 480BF8DD  bl 0x8293fea0
	ctx.lr = 0x828805C8;
	sub_8293FEA0(ctx, base);
	// 828805C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828805CC: 48927BEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828805D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828805D0 size=444
    let mut pc: u32 = 0x828805D0;
    'dispatch: loop {
        match pc {
            0x828805D0 => {
    //   block [0x828805D0..0x8288078C)
	// 828805D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828805D4: 48927B95  bl 0x831a8168
	ctx.lr = 0x828805D8;
	sub_831A8130(ctx, base);
	// 828805D8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828805DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828805E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 828805E4: 488D401D  bl 0x83154600
	ctx.lr = 0x828805E8;
	sub_83154600(ctx, base);
	// 828805E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828805EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828805F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828805F4: 4BF6F25D  bl 0x827ef850
	ctx.lr = 0x828805F8;
	sub_827EF850(ctx, base);
	// 828805F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828805FC: 4809616D  bl 0x82916768
	ctx.lr = 0x82880600;
	sub_82916768(ctx, base);
	// 82880600: 480888C9  bl 0x82908ec8
	ctx.lr = 0x82880604;
	sub_82908EC8(ctx, base);
	// 82880604: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880608: 3B830010  addi r28, r3, 0x10
	ctx.r[28].s64 = ctx.r[3].s64 + 16;
	// 8288060C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880610: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82880614: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880618: 4E800421  bctrl
	ctx.lr = 0x8288061C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8288061C: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82880620: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82880624: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880790 size=112
    let mut pc: u32 = 0x82880790;
    'dispatch: loop {
        match pc {
            0x82880790 => {
    //   block [0x82880790..0x82880800)
	// 82880790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880794: 489279D9  bl 0x831a816c
	ctx.lr = 0x82880798;
	sub_831A8130(ctx, base);
	// 82880798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288079C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828807A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828807A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828807A8: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 828807AC: 38A003B3  li r5, 0x3b3
	ctx.r[5].s64 = 947;
	// 828807B0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 828807B4: 48571C35  bl 0x82df23e8
	ctx.lr = 0x828807B8;
	sub_82DF23E8(ctx, base);
	// 828807B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828807BC: 41820010  beq 0x828807cc
	if ctx.cr[0].eq {
	pc = 0x828807CC; continue 'dispatch;
	}
	// 828807C0: 4BFFFB59  bl 0x82880318
	ctx.lr = 0x828807C4;
	sub_82880318(ctx, base);
	// 828807C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828807C8: 48000008  b 0x828807d0
	pc = 0x828807D0; continue 'dispatch;
	// 828807CC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828807D0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828807D4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 828807D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828807DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828807E0: 4BFFEA39  bl 0x8287f218
	ctx.lr = 0x828807E4;
	sub_8287F218(ctx, base);
	// 828807E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828807E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828807EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828807F0: 4BA3F811  bl 0x822c0000
	ctx.lr = 0x828807F4;
	sub_822C0000(ctx, base);
	// 828807F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828807F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828807FC: 489279C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880800 size=112
    let mut pc: u32 = 0x82880800;
    'dispatch: loop {
        match pc {
            0x82880800 => {
    //   block [0x82880800..0x82880870)
	// 82880800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880804: 48927969  bl 0x831a816c
	ctx.lr = 0x82880808;
	sub_831A8130(ctx, base);
	// 82880808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288080C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82880810: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82880814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82880818: 388B4418  addi r4, r11, 0x4418
	ctx.r[4].s64 = ctx.r[11].s64 + 17432;
	// 8288081C: 38A003F4  li r5, 0x3f4
	ctx.r[5].s64 = 1012;
	// 82880820: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82880824: 48571BC5  bl 0x82df23e8
	ctx.lr = 0x82880828;
	sub_82DF23E8(ctx, base);
	// 82880828: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288082C: 41820010  beq 0x8288083c
	if ctx.cr[0].eq {
	pc = 0x8288083C; continue 'dispatch;
	}
	// 82880830: 4BFFE291  bl 0x8287eac0
	ctx.lr = 0x82880834;
	sub_8287EAC0(ctx, base);
	// 82880834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880838: 48000008  b 0x82880840
	pc = 0x82880840; continue 'dispatch;
	// 8288083C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82880840: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82880844: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82880848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288084C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880850: 4BFFEA91  bl 0x8287f2e0
	ctx.lr = 0x82880854;
	sub_8287F2E0(ctx, base);
	// 82880854: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82880858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288085C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880860: 4BA3F7A1  bl 0x822c0000
	ctx.lr = 0x82880864;
	sub_822C0000(ctx, base);
	// 82880864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880868: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288086C: 48927950  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880870 size=120
    let mut pc: u32 = 0x82880870;
    'dispatch: loop {
        match pc {
            0x82880870 => {
    //   block [0x82880870..0x828808E8)
	// 82880870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880874: 489278F9  bl 0x831a816c
	ctx.lr = 0x82880878;
	sub_831A8130(ctx, base);
	// 82880878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288087C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82880880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82880884: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82880888: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8288088C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82880890: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 82880894: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82880898: 48571B51  bl 0x82df23e8
	ctx.lr = 0x8288089C;
	sub_82DF23E8(ctx, base);
	// 8288089C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828808A0: 41820014  beq 0x828808b4
	if ctx.cr[0].eq {
	pc = 0x828808B4; continue 'dispatch;
	}
	// 828808A4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828808A8: 480D4A19  bl 0x829552c0
	ctx.lr = 0x828808AC;
	sub_829552C0(ctx, base);
	// 828808AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828808B0: 48000008  b 0x828808b8
	pc = 0x828808B8; continue 'dispatch;
	// 828808B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828808B8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828808BC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 828808C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828808C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828808C8: 4BFFEAE1  bl 0x8287f3a8
	ctx.lr = 0x828808CC;
	sub_8287F3A8(ctx, base);
	// 828808CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828808D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828808D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828808D8: 4BA3F729  bl 0x822c0000
	ctx.lr = 0x828808DC;
	sub_822C0000(ctx, base);
	// 828808DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828808E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828808E4: 489278D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828808E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828808E8 size=332
    let mut pc: u32 = 0x828808E8;
    'dispatch: loop {
        match pc {
            0x828808E8 => {
    //   block [0x828808E8..0x82880A34)
	// 828808E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828808EC: 4892787D  bl 0x831a8168
	ctx.lr = 0x828808F0;
	sub_831A8130(ctx, base);
	// 828808F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828808F4: 488D3D0D  bl 0x83154600
	ctx.lr = 0x828808F8;
	sub_83154600(ctx, base);
	// 828808F8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828808FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880904: 808B9518  lwz r4, -0x6ae8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27368 as u32) ) } as u64;
	// 82880908: 48573101  bl 0x82df3a08
	ctx.lr = 0x8288090C;
	sub_82DF3A08(ctx, base);
	// 8288090C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880914: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880918: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8288091C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880920: 4E800421  bctrl
	ctx.lr = 0x82880924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880928: 48572B01  bl 0x82df3428
	ctx.lr = 0x8288092C;
	sub_82DF3428(ctx, base);
	// 8288092C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880930: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82880934: 48011C55  bl 0x82892588
	ctx.lr = 0x82880938;
	sub_82892588(ctx, base);
	// 82880938: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8288093C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82880940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880944: 480966DD  bl 0x82917020
	ctx.lr = 0x82880948;
	sub_82917020(ctx, base);
	// 82880948: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8288094C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880950: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82880954: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82880958: 4BFFFF19  bl 0x82880870
	ctx.lr = 0x8288095C;
	sub_82880870(ctx, base);
	// 8288095C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880960: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82880964: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82880968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8288096C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82880970: 419A0024  beq cr6, 0x82880994
	if ctx.cr[6].eq {
	pc = 0x82880994; continue 'dispatch;
	}
	// 82880974: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82880978: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8288097C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82880980: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82880984: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82880988: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8288098C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82880990: 4082FFE8  bne 0x82880978
	if !ctx.cr[0].eq {
	pc = 0x82880978; continue 'dispatch;
	}
	// 82880994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880998: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 8288099C: 48095DCD  bl 0x82916768
	ctx.lr = 0x828809A0;
	sub_82916768(ctx, base);
	// 828809A0: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 828809A4: 48788615  bl 0x83008fb8
	ctx.lr = 0x828809A8;
	sub_83008FB8(ctx, base);
	// 828809A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828809AC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828809B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828809B4: 3B8B4418  addi r28, r11, 0x4418
	ctx.r[28].s64 = ctx.r[11].s64 + 17432;
	// 828809B8: 48095DB1  bl 0x82916768
	ctx.lr = 0x828809BC;
	sub_82916768(ctx, base);
	// 828809BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 828809C0: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 828809C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 828809C8: 38A001E3  li r5, 0x1e3
	ctx.r[5].s64 = 483;
	// 828809CC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 828809D0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 828809D4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828809D8: 485D8069  bl 0x82e58a40
	ctx.lr = 0x828809DC;
	sub_82E58A40(ctx, base);
	// 828809DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828809E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828809E4: 419A0008  beq cr6, 0x828809ec
	if ctx.cr[6].eq {
	pc = 0x828809EC; continue 'dispatch;
	}
	// 828809E8: 4BA3FEA9  bl 0x822c0890
	ctx.lr = 0x828809EC;
	sub_822C0890(ctx, base);
	// 828809EC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 828809F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828809F4: 419A0008  beq cr6, 0x828809fc
	if ctx.cr[6].eq {
	pc = 0x828809FC; continue 'dispatch;
	}
	// 828809F8: 4BA3FE99  bl 0x822c0890
	ctx.lr = 0x828809FC;
	sub_822C0890(ctx, base);
	// 828809FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82880A00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880A04: 388B7C94  addi r4, r11, 0x7c94
	ctx.r[4].s64 = ctx.r[11].s64 + 31892;
	// 82880A08: 48573001  bl 0x82df3a08
	ctx.lr = 0x82880A0C;
	sub_82DF3A08(ctx, base);
	// 82880A0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880A10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880A18: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82880A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880A20: 4E800421  bctrl
	ctx.lr = 0x82880A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880A24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880A28: 48572A01  bl 0x82df3428
	ctx.lr = 0x82880A2C;
	sub_82DF3428(ctx, base);
	// 82880A2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82880A30: 48927788  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82880A38 size=248
    let mut pc: u32 = 0x82880A38;
    'dispatch: loop {
        match pc {
            0x82880A38 => {
    //   block [0x82880A38..0x82880B30)
	// 82880A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880A3C: 4892772D  bl 0x831a8168
	ctx.lr = 0x82880A40;
	sub_831A8130(ctx, base);
	// 82880A40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880A44: 488D3BBD  bl 0x83154600
	ctx.lr = 0x82880A48;
	sub_83154600(ctx, base);
	// 82880A48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82880A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880A50: 48011B51  bl 0x828925a0
	ctx.lr = 0x82880A54;
	sub_828925A0(ctx, base);
	// 82880A54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880A58: 4BF6CDC1  bl 0x827ed818
	ctx.lr = 0x82880A5C;
	sub_827ED818(ctx, base);
	// 82880A5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82880A60: 4BFF76E1  bl 0x82878140
	ctx.lr = 0x82880A64;
	sub_82878140(ctx, base);
	// 82880A64: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82880A68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880A6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82880A70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82880A74: 4BFFFDFD  bl 0x82880870
	ctx.lr = 0x82880A78;
	sub_82880870(ctx, base);
	// 82880A78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880A7C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82880A80: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82880A84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82880A88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82880A8C: 419A0024  beq cr6, 0x82880ab0
	if ctx.cr[6].eq {
	pc = 0x82880AB0; continue 'dispatch;
	}
	// 82880A90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82880A94: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82880A98: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82880A9C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82880AA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82880AA4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82880AA8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82880AAC: 4082FFE8  bne 0x82880a94
	if !ctx.cr[0].eq {
	pc = 0x82880A94; continue 'dispatch;
	}
	// 82880AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880AB4: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 82880AB8: 48095CB1  bl 0x82916768
	ctx.lr = 0x82880ABC;
	sub_82916768(ctx, base);
	// 82880ABC: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82880AC0: 487884F9  bl 0x83008fb8
	ctx.lr = 0x82880AC4;
	sub_83008FB8(ctx, base);
	// 82880AC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82880AC8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82880ACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880AD0: 3B8B4418  addi r28, r11, 0x4418
	ctx.r[28].s64 = ctx.r[11].s64 + 17432;
	// 82880AD4: 48095C95  bl 0x82916768
	ctx.lr = 0x82880AD8;
	sub_82916768(ctx, base);
	// 82880AD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82880ADC: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82880AE0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82880AE4: 38A00201  li r5, 0x201
	ctx.r[5].s64 = 513;
	// 82880AE8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82880AEC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82880AF0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82880AF4: 485D7F4D  bl 0x82e58a40
	ctx.lr = 0x82880AF8;
	sub_82E58A40(ctx, base);
	// 82880AF8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82880AFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82880B00: 419A0008  beq cr6, 0x82880b08
	if ctx.cr[6].eq {
	pc = 0x82880B08; continue 'dispatch;
	}
	// 82880B04: 4BA3FD8D  bl 0x822c0890
	ctx.lr = 0x82880B08;
	sub_822C0890(ctx, base);
	// 82880B08: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82880B0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82880B10: 419A0008  beq cr6, 0x82880b18
	if ctx.cr[6].eq {
	pc = 0x82880B18; continue 'dispatch;
	}
	// 82880B14: 4BA3FD7D  bl 0x822c0890
	ctx.lr = 0x82880B18;
	sub_822C0890(ctx, base);
	// 82880B18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82880B1C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82880B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880B24: 480964FD  bl 0x82917020
	ctx.lr = 0x82880B28;
	sub_82917020(ctx, base);
	// 82880B28: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82880B2C: 4892768C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880B30 size=160
    let mut pc: u32 = 0x82880B30;
    'dispatch: loop {
        match pc {
            0x82880B30 => {
    //   block [0x82880B30..0x82880BD0)
	// 82880B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880B34: 48927635  bl 0x831a8168
	ctx.lr = 0x82880B38;
	sub_831A8130(ctx, base);
	// 82880B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880B3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82880B40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880B44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82880B48: 57BC063F  clrlwi. r28, r29, 0x18
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82880B4C: 41820034  beq 0x82880b80
	if ctx.cr[0].eq {
	pc = 0x82880B80; continue 'dispatch;
	}
	// 82880B50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880B54: 48928E35  bl 0x831a9988
	ctx.lr = 0x82880B58;
	sub_831A9988(ctx, base);
	// 82880B58: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82880B5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82880B60: 386B3984  addi r3, r11, 0x3984
	ctx.r[3].s64 = ctx.r[11].s64 + 14724;
	// 82880B64: 48927595  bl 0x831a80f8
	ctx.lr = 0x82880B68;
	sub_831A80F8(ctx, base);
	// 82880B68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82880B6C: 41820014  beq 0x82880b80
	if ctx.cr[0].eq {
	pc = 0x82880B80; continue 'dispatch;
	}
	// 82880B70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880B78: 4BFFFA59  bl 0x828805d0
	ctx.lr = 0x82880B7C;
	sub_828805D0(ctx, base);
	// 82880B7C: 4800004C  b 0x82880bc8
	pc = 0x82880BC8; continue 'dispatch;
	// 82880B80: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82880B84: 419A0034  beq cr6, 0x82880bb8
	if ctx.cr[6].eq {
	pc = 0x82880BB8; continue 'dispatch;
	}
	// 82880B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880B8C: 48928DFD  bl 0x831a9988
	ctx.lr = 0x82880B90;
	sub_831A9988(ctx, base);
	// 82880B90: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82880B94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82880B98: 386BF378  addi r3, r11, -0xc88
	ctx.r[3].s64 = ctx.r[11].s64 + -3208;
	// 82880B9C: 4892755D  bl 0x831a80f8
	ctx.lr = 0x82880BA0;
	sub_831A80F8(ctx, base);
	// 82880BA0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82880BA4: 41820014  beq 0x82880bb8
	if ctx.cr[0].eq {
	pc = 0x82880BB8; continue 'dispatch;
	}
	// 82880BA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880BAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880BB0: 4BF54F21  bl 0x827d5ad0
	ctx.lr = 0x82880BB4;
	sub_827D5AD0(ctx, base);
	// 82880BB4: 48000014  b 0x82880bc8
	pc = 0x82880BC8; continue 'dispatch;
	// 82880BB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82880BBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82880BC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880BC4: 488FD8C5  bl 0x8317e488
	ctx.lr = 0x82880BC8;
	sub_8317E488(ctx, base);
	// 82880BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82880BCC: 489275EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880BD0 size=180
    let mut pc: u32 = 0x82880BD0;
    'dispatch: loop {
        match pc {
            0x82880BD0 => {
    //   block [0x82880BD0..0x82880C84)
	// 82880BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880BD4: 48927599  bl 0x831a816c
	ctx.lr = 0x82880BD8;
	sub_831A8130(ctx, base);
	// 82880BD8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82880BDC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880BE4: 488D3A1D  bl 0x83154600
	ctx.lr = 0x82880BE8;
	sub_83154600(ctx, base);
	// 82880BE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880BEC: 48095B7D  bl 0x82916768
	ctx.lr = 0x82880BF0;
	sub_82916768(ctx, base);
	// 82880BF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82880BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880BF8: 485D8B81  bl 0x82e59778
	ctx.lr = 0x82880BFC;
	sub_82E59778(ctx, base);
	// 82880BFC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880C00: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82880C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880C08: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82880C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880C10: 4E800421  bctrl
	ctx.lr = 0x82880C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880C14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82880C18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880C1C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82880C20: 480BF011  bl 0x8293fc30
	ctx.lr = 0x82880C24;
	sub_8293FC30(ctx, base);
	// 82880C24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880C28: 4BFFAC19  bl 0x8287b840
	ctx.lr = 0x82880C2C;
	sub_8287B840(ctx, base);
	// 82880C2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82880C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880C34: 4BFFAC25  bl 0x8287b858
	ctx.lr = 0x82880C38;
	sub_8287B858(ctx, base);
	// 82880C38: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880C3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880C40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82880C44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880C48: 4E800421  bctrl
	ctx.lr = 0x82880C4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880C4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880C50: 48095B19  bl 0x82916768
	ctx.lr = 0x82880C54;
	sub_82916768(ctx, base);
	// 82880C54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880C58: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82880C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880C60: 4E800421  bctrl
	ctx.lr = 0x82880C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880C64: 480BE835  bl 0x8293f498
	ctx.lr = 0x82880C68;
	sub_8293F498(ctx, base);
	// 82880C68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82880C6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82880C70: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 82880C74: 4BB5337D  bl 0x823d3ff0
	ctx.lr = 0x82880C78;
	sub_823D3FF0(ctx, base);
	// 82880C78: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82880C7C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82880C80: 4892753C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82880C88 size=428
    let mut pc: u32 = 0x82880C88;
    'dispatch: loop {
        match pc {
            0x82880C88 => {
    //   block [0x82880C88..0x82880E34)
	// 82880C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880C8C: 489274DD  bl 0x831a8168
	ctx.lr = 0x82880C90;
	sub_831A8130(ctx, base);
	// 82880C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880C94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880C98: 488D3969  bl 0x83154600
	ctx.lr = 0x82880C9C;
	sub_83154600(ctx, base);
	// 82880C9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82880CA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880CA4: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82880CA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880CAC: 4E800421  bctrl
	ctx.lr = 0x82880CB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880CB0: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82880CB4: 3BDF0068  addi r30, r31, 0x68
	ctx.r[30].s64 = ctx.r[31].s64 + 104;
	// 82880CB8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82880CBC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82880CC0: 41820010  beq 0x82880cd0
	if ctx.cr[0].eq {
	pc = 0x82880CD0; continue 'dispatch;
	}
	// 82880CC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82880CC8: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880CCC: 48000108  b 0x82880dd4
	pc = 0x82880DD4; continue 'dispatch;
	// 82880CD0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82880CD4: 556AD7FF  rlwinm. r10, r11, 0x1a, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880CD8: 408200A8  bne 0x82880d80
	if !ctx.cr[0].eq {
	pc = 0x82880D80; continue 'dispatch;
	}
	// 82880CDC: 556AF7FF  rlwinm. r10, r11, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880CE0: 408200A0  bne 0x82880d80
	if !ctx.cr[0].eq {
	pc = 0x82880D80; continue 'dispatch;
	}
	// 82880CE4: 556ADFFF  rlwinm. r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880CE8: 40820074  bne 0x82880d5c
	if !ctx.cr[0].eq {
	pc = 0x82880D5C; continue 'dispatch;
	}
	// 82880CEC: 556AFFFF  rlwinm. r10, r11, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880CF0: 4082006C  bne 0x82880d5c
	if !ctx.cr[0].eq {
	pc = 0x82880D5C; continue 'dispatch;
	}
	// 82880CF4: 556AC7FF  rlwinm. r10, r11, 0x18, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880CF8: 40820040  bne 0x82880d38
	if !ctx.cr[0].eq {
	pc = 0x82880D38; continue 'dispatch;
	}
	// 82880CFC: 556AE7FF  rlwinm. r10, r11, 0x1c, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880D00: 40820038  bne 0x82880d38
	if !ctx.cr[0].eq {
	pc = 0x82880D38; continue 'dispatch;
	}
	// 82880D04: 556ACFFF  rlwinm. r10, r11, 0x19, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82880D08: 4082000C  bne 0x82880d14
	if !ctx.cr[0].eq {
	pc = 0x82880D14; continue 'dispatch;
	}
	// 82880D0C: 556BEFFF  rlwinm. r11, r11, 0x1d, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82880D10: 418200C4  beq 0x82880dd4
	if ctx.cr[0].eq {
	pc = 0x82880DD4; continue 'dispatch;
	}
	// 82880D14: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82880D18: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82880D1C: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D20: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D24: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82880D28: 556B066E  rlwinm r11, r11, 0, 0x19, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D2C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D30: 808A954C  lwz r4, -0x6ab4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27316 as u32) ) } as u64;
	// 82880D34: 4800006C  b 0x82880da0
	pc = 0x82880DA0; continue 'dispatch;
	// 82880D38: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82880D3C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82880D40: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D44: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D48: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82880D4C: 556B062C  rlwinm r11, r11, 0, 0x18, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D50: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D54: 808A9548  lwz r4, -0x6ab8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27320 as u32) ) } as u64;
	// 82880D58: 48000048  b 0x82880da0
	pc = 0x82880DA0; continue 'dispatch;
	// 82880D5C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82880D60: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82880D64: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D68: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D6C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82880D70: 556B06F2  rlwinm r11, r11, 0, 0x1b, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D74: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D78: 808A9544  lwz r4, -0x6abc(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27324 as u32) ) } as u64;
	// 82880D7C: 48000024  b 0x82880da0
	pc = 0x82880DA0; continue 'dispatch;
	// 82880D80: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82880D84: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82880D88: 556B07B8  rlwinm r11, r11, 0, 0x1e, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D8C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D90: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82880D94: 556B06B0  rlwinm r11, r11, 0, 0x1a, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880D98: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82880D9C: 808A9540  lwz r4, -0x6ac0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27328 as u32) ) } as u64;
	// 82880DA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880DA4: 48572C65  bl 0x82df3a08
	ctx.lr = 0x82880DA8;
	sub_82DF3A08(ctx, base);
	// 82880DA8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880DAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880DB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880DB4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82880DB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880DBC: 4E800421  bctrl
	ctx.lr = 0x82880DC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880DC4: 48572665  bl 0x82df3428
	ctx.lr = 0x82880DC8;
	sub_82DF3428(ctx, base);
	// 82880DC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880DCC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82880DD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82880DD4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880DD8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82880DDC: 41820050  beq 0x82880e2c
	if ctx.cr[0].eq {
	pc = 0x82880E2C; continue 'dispatch;
	}
	// 82880DE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82880DE4: 4BF6952D  bl 0x827ea310
	ctx.lr = 0x82880DE8;
	sub_827EA310(ctx, base);
	// 82880DE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82880DEC: 41820040  beq 0x82880e2c
	if ctx.cr[0].eq {
	pc = 0x82880E2C; continue 'dispatch;
	}
	// 82880DF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82880DF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880DF8: 808B953C  lwz r4, -0x6ac4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27332 as u32) ) } as u64;
	// 82880DFC: 48572C0D  bl 0x82df3a08
	ctx.lr = 0x82880E00;
	sub_82DF3A08(ctx, base);
	// 82880E00: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880E04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880E08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82880E0C: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82880E10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880E14: 4E800421  bctrl
	ctx.lr = 0x82880E18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880E18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880E1C: 4857260D  bl 0x82df3428
	ctx.lr = 0x82880E20;
	sub_82DF3428(ctx, base);
	// 82880E20: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880E24: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82880E28: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82880E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82880E30: 48927388  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82880E38 size=300
    let mut pc: u32 = 0x82880E38;
    'dispatch: loop {
        match pc {
            0x82880E38 => {
    //   block [0x82880E38..0x82880F64)
	// 82880E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880E3C: 4892732D  bl 0x831a8168
	ctx.lr = 0x82880E40;
	sub_831A8130(ctx, base);
	// 82880E40: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82880E44: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880E48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880E4C: 488D37B5  bl 0x83154600
	ctx.lr = 0x82880E50;
	sub_83154600(ctx, base);
	// 82880E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880E54: 48095915  bl 0x82916768
	ctx.lr = 0x82880E58;
	sub_82916768(ctx, base);
	// 82880E58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880E5C: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82880E60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880E64: 4E800421  bctrl
	ctx.lr = 0x82880E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880E68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880E6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82880E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880E74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82880E78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880E7C: 4E800421  bctrl
	ctx.lr = 0x82880E80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880E80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82880E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82880E88: 480958E1  bl 0x82916768
	ctx.lr = 0x82880E8C;
	sub_82916768(ctx, base);
	// 82880E8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82880E90: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82880E94: 480887C5  bl 0x82909658
	ctx.lr = 0x82880E98;
	sub_82909658(ctx, base);
	// 82880E98: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82880E9C: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82880EA0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82880F68 size=140
    let mut pc: u32 = 0x82880F68;
    'dispatch: loop {
        match pc {
            0x82880F68 => {
    //   block [0x82880F68..0x82880FF4)
	// 82880F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82880F70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82880F74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82880F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82880F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82880F80: 488D3681  bl 0x83154600
	ctx.lr = 0x82880F84;
	sub_83154600(ctx, base);
	// 82880F84: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82880F88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82880F8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880F90: 808BF400  lwz r4, -0xc00(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3072 as u32) ) } as u64;
	// 82880F94: 48572A75  bl 0x82df3a08
	ctx.lr = 0x82880F98;
	sub_82DF3A08(ctx, base);
	// 82880F98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82880F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880FA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82880FA4: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82880FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82880FAC: 4E800421  bctrl
	ctx.lr = 0x82880FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82880FB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82880FB4: 48572475  bl 0x82df3428
	ctx.lr = 0x82880FB8;
	sub_82DF3428(ctx, base);
	// 82880FB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82880FBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880FC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82880FC4: C00BA1C4  lfs f0, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82880FC8: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82880FCC: 480115BD  bl 0x82892588
	ctx.lr = 0x82880FD0;
	sub_82892588(ctx, base);
	// 82880FD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82880FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82880FD8: 480115C9  bl 0x828925a0
	ctx.lr = 0x82880FDC;
	sub_828925A0(ctx, base);
	// 82880FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82880FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82880FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82880FE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82880FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82880FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82880FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82880FF8 size=116
    let mut pc: u32 = 0x82880FF8;
    'dispatch: loop {
        match pc {
            0x82880FF8 => {
    //   block [0x82880FF8..0x8288106C)
	// 82880FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82880FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82881000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82881004: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8288100C: 485D876D  bl 0x82e59778
	ctx.lr = 0x82881010;
	sub_82E59778(ctx, base);
	// 82881010: C01F0060  lfs f0, 0x60(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82881014: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82881018: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8288101C: D1BF0060  stfs f13, 0x60(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82881020: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82881024: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82881028: 40980030  bge cr6, 0x82881058
	if !ctx.cr[6].lt {
	pc = 0x82881058; continue 'dispatch;
	}
	// 8288102C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82881030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881034: 808B0AF4  lwz r4, 0xaf4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2804 as u32) ) } as u64;
	// 82881038: 485729D1  bl 0x82df3a08
	ctx.lr = 0x8288103C;
	sub_82DF3A08(ctx, base);
	// 8288103C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881040: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82881044: 488D35BD  bl 0x83154600
	ctx.lr = 0x82881048;
	sub_83154600(ctx, base);
	// 82881048: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288104C: 4BF6E6F5  bl 0x827ef740
	ctx.lr = 0x82881050;
	sub_827EF740(ctx, base);
	// 82881050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881054: 485723D5  bl 0x82df3428
	ctx.lr = 0x82881058;
	sub_82DF3428(ctx, base);
	// 82881058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288105C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82881060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82881064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82881068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881070 size=96
    let mut pc: u32 = 0x82881070;
    'dispatch: loop {
        match pc {
            0x82881070 => {
    //   block [0x82881070..0x828810D0)
	// 82881070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82881078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8288107C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881084: 4BFFCBED  bl 0x8287dc70
	ctx.lr = 0x82881088;
	sub_8287DC70(ctx, base);
	// 82881088: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288108C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82881090: 396B4514  addi r11, r11, 0x4514
	ctx.r[11].s64 = ctx.r[11].s64 + 17684;
	// 82881094: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881098: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8288109C: 808A0AF4  lwz r4, 0xaf4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2804 as u32) ) } as u64;
	// 828810A0: 48572969  bl 0x82df3a08
	ctx.lr = 0x828810A4;
	sub_82DF3A08(ctx, base);
	// 828810A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828810A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828810AC: 485D863D  bl 0x82e596e8
	ctx.lr = 0x828810B0;
	sub_82E596E8(ctx, base);
	// 828810B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828810B4: 48572375  bl 0x82df3428
	ctx.lr = 0x828810B8;
	sub_82DF3428(ctx, base);
	// 828810B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828810BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828810C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828810C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828810C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828810CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828810D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828810D0 size=200
    let mut pc: u32 = 0x828810D0;
    'dispatch: loop {
        match pc {
            0x828810D0 => {
    //   block [0x828810D0..0x82881198)
	// 828810D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828810D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828810D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828810DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828810E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828810E4: 488D351D  bl 0x83154600
	ctx.lr = 0x828810E8;
	sub_83154600(ctx, base);
	// 828810E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828810EC: 4BF6C72D  bl 0x827ed818
	ctx.lr = 0x828810F0;
	sub_827ED818(ctx, base);
	// 828810F0: 4BFF2921  bl 0x82873a10
	ctx.lr = 0x828810F4;
	sub_82873A10(ctx, base);
	// 828810F4: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 828810F8: 409A002C  bne cr6, 0x82881124
	if !ctx.cr[6].eq {
	pc = 0x82881124; continue 'dispatch;
	}
	// 828810FC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82881100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881104: 808B95C0  lwz r4, -0x6a40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27200 as u32) ) } as u64;
	// 82881108: 48572901  bl 0x82df3a08
	ctx.lr = 0x8288110C;
	sub_82DF3A08(ctx, base);
	// 8288110C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82881110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881114: 4BF6E62D  bl 0x827ef740
	ctx.lr = 0x82881118;
	sub_827EF740(ctx, base);
	// 82881118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288111C: 4857230D  bl 0x82df3428
	ctx.lr = 0x82881120;
	sub_82DF3428(ctx, base);
	// 82881120: 48000060  b 0x82881180
	pc = 0x82881180; continue 'dispatch;
	// 82881124: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82881128: 409A0058  bne cr6, 0x82881180
	if !ctx.cr[6].eq {
	pc = 0x82881180; continue 'dispatch;
	}
	// 8288112C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82881130: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82881134: 48095C65  bl 0x82916d98
	ctx.lr = 0x82881138;
	sub_82916D98(ctx, base);
	// 82881138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288113C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82881140: 4BF6C6D9  bl 0x827ed818
	ctx.lr = 0x82881144;
	sub_827ED818(ctx, base);
	// 82881144: 4BFF28D5  bl 0x82873a18
	ctx.lr = 0x82881148;
	sub_82873A18(ctx, base);
	// 82881148: 907E007C  stw r3, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 8288114C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82881150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881154: 808B0B20  lwz r4, 0xb20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2848 as u32) ) } as u64;
	// 82881158: 485728B1  bl 0x82df3a08
	ctx.lr = 0x8288115C;
	sub_82DF3A08(ctx, base);
	// 8288115C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82881160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881164: 4BF6E5DD  bl 0x827ef740
	ctx.lr = 0x82881168;
	sub_827EF740(ctx, base);
	// 82881168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288116C: 485722BD  bl 0x82df3428
	ctx.lr = 0x82881170;
	sub_82DF3428(ctx, base);
	// 82881170: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82881174: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82881178: 419A0008  beq cr6, 0x82881180
	if ctx.cr[6].eq {
	pc = 0x82881180; continue 'dispatch;
	}
	// 8288117C: 4BA3F715  bl 0x822c0890
	ctx.lr = 0x82881180;
	sub_822C0890(ctx, base);
	// 82881180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82881184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82881188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8288118C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82881190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82881194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881198 size=96
    let mut pc: u32 = 0x82881198;
    'dispatch: loop {
        match pc {
            0x82881198 => {
    //   block [0x82881198..0x828811F8)
	// 82881198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288119C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828811A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828811A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828811A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828811AC: 4BFFCAC5  bl 0x8287dc70
	ctx.lr = 0x828811B0;
	sub_8287DC70(ctx, base);
	// 828811B0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828811B4: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 828811B8: 396B4540  addi r11, r11, 0x4540
	ctx.r[11].s64 = ctx.r[11].s64 + 17728;
	// 828811BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828811C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828811C4: 808A0B18  lwz r4, 0xb18(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2840 as u32) ) } as u64;
	// 828811C8: 48572841  bl 0x82df3a08
	ctx.lr = 0x828811CC;
	sub_82DF3A08(ctx, base);
	// 828811CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828811D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828811D4: 485D8515  bl 0x82e596e8
	ctx.lr = 0x828811D8;
	sub_82E596E8(ctx, base);
	// 828811D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828811DC: 4857224D  bl 0x82df3428
	ctx.lr = 0x828811E0;
	sub_82DF3428(ctx, base);
	// 828811E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828811E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828811E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828811EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828811F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828811F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828811F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828811F8 size=196
    let mut pc: u32 = 0x828811F8;
    'dispatch: loop {
        match pc {
            0x828811F8 => {
    //   block [0x828811F8..0x828812BC)
	// 828811F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828811FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82881200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82881204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82881208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288120C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82881210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82881214: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82881218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8288121C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82881220: 4BA3F719  bl 0x822c0938
	ctx.lr = 0x82881224;
	sub_822C0938(ctx, base);
	// 82881224: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82881228: 41820028  beq 0x82881250
	if ctx.cr[0].eq {
	pc = 0x82881250; continue 'dispatch;
	}
	// 8288122C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881230: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82881234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82881238: 392B44D8  addi r9, r11, 0x44d8
	ctx.r[9].s64 = ctx.r[11].s64 + 17624;
	// 8288123C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82881240: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82881244: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82881248: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8288124C: 48000008  b 0x82881254
	pc = 0x82881254; continue 'dispatch;
	// 82881250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82881254: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82881258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8288125C: 409A0044  bne cr6, 0x828812a0
	if !ctx.cr[6].eq {
	pc = 0x828812A0; continue 'dispatch;
	}
	// 82881260: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82881264: 419A001C  beq cr6, 0x82881280
	if ctx.cr[6].eq {
	pc = 0x82881280; continue 'dispatch;
	}
	// 82881268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288126C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82881270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881274: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82881278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288127C: 4E800421  bctrl
	ctx.lr = 0x82881280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881280: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82881284: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82881288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288128C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82881290: 816B9F10  lwz r11, -0x60f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 82881294: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82881298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288129C: 4BA3ED65  bl 0x822c0000
	ctx.lr = 0x828812A0;
	sub_822C0000(ctx, base);
	// 828812A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828812A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828812A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828812AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828812B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828812B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828812B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828812C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828812C0 size=196
    let mut pc: u32 = 0x828812C0;
    'dispatch: loop {
        match pc {
            0x828812C0 => {
    //   block [0x828812C0..0x82881384)
	// 828812C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828812C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828812C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828812CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828812D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828812D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828812D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828812DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828812E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828812E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828812E8: 4BA3F651  bl 0x822c0938
	ctx.lr = 0x828812EC;
	sub_822C0938(ctx, base);
	// 828812EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828812F0: 41820028  beq 0x82881318
	if ctx.cr[0].eq {
	pc = 0x82881318; continue 'dispatch;
	}
	// 828812F4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828812F8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 828812FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82881300: 392B44EC  addi r9, r11, 0x44ec
	ctx.r[9].s64 = ctx.r[11].s64 + 17644;
	// 82881304: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82881308: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8288130C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82881310: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82881314: 48000008  b 0x8288131c
	pc = 0x8288131C; continue 'dispatch;
	// 82881318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8288131C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82881320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82881324: 409A0044  bne cr6, 0x82881368
	if !ctx.cr[6].eq {
	pc = 0x82881368; continue 'dispatch;
	}
	// 82881328: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8288132C: 419A001C  beq cr6, 0x82881348
	if ctx.cr[6].eq {
	pc = 0x82881348; continue 'dispatch;
	}
	// 82881330: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881334: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82881338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288133C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82881340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881344: 4E800421  bctrl
	ctx.lr = 0x82881348;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881348: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8288134C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82881350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881354: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82881358: 816B9F10  lwz r11, -0x60f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 8288135C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82881360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82881364: 4BA3EC9D  bl 0x822c0000
	ctx.lr = 0x82881368;
	sub_822C0000(ctx, base);
	// 82881368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8288136C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82881370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82881374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82881378: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8288137C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82881380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881388 size=196
    let mut pc: u32 = 0x82881388;
    'dispatch: loop {
        match pc {
            0x82881388 => {
    //   block [0x82881388..0x8288144C)
	// 82881388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288138C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82881390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82881394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82881398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288139C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828813A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828813A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828813A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828813AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828813B0: 4BA3F589  bl 0x822c0938
	ctx.lr = 0x828813B4;
	sub_822C0938(ctx, base);
	// 828813B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828813B8: 41820028  beq 0x828813e0
	if ctx.cr[0].eq {
	pc = 0x828813E0; continue 'dispatch;
	}
	// 828813BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828813C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 828813C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 828813C8: 392B4500  addi r9, r11, 0x4500
	ctx.r[9].s64 = ctx.r[11].s64 + 17664;
	// 828813CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 828813D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 828813D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 828813D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 828813DC: 48000008  b 0x828813e4
	pc = 0x828813E4; continue 'dispatch;
	// 828813E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828813E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828813E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828813EC: 409A0044  bne cr6, 0x82881430
	if !ctx.cr[6].eq {
	pc = 0x82881430; continue 'dispatch;
	}
	// 828813F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828813F4: 419A001C  beq cr6, 0x82881410
	if ctx.cr[6].eq {
	pc = 0x82881410; continue 'dispatch;
	}
	// 828813F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828813FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82881400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881404: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82881408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288140C: 4E800421  bctrl
	ctx.lr = 0x82881410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881410: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82881414: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82881418: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288141C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82881420: 816B9F10  lwz r11, -0x60f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24816 as u32) ) } as u64;
	// 82881424: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82881428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288142C: 4BA3EBD5  bl 0x822c0000
	ctx.lr = 0x82881430;
	sub_822C0000(ctx, base);
	// 82881430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881434: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82881438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288143C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82881440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82881444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82881448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881450 size=96
    let mut pc: u32 = 0x82881450;
    'dispatch: loop {
        match pc {
            0x82881450 => {
    //   block [0x82881450..0x828814B0)
	// 82881450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82881458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8288145C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881464: 4BFFC80D  bl 0x8287dc70
	ctx.lr = 0x82881468;
	sub_8287DC70(ctx, base);
	// 82881468: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288146C: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82881470: 396B456C  addi r11, r11, 0x456c
	ctx.r[11].s64 = ctx.r[11].s64 + 17772;
	// 82881474: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881478: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8288147C: 808A0AF4  lwz r4, 0xaf4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2804 as u32) ) } as u64;
	// 82881480: 48572589  bl 0x82df3a08
	ctx.lr = 0x82881484;
	sub_82DF3A08(ctx, base);
	// 82881484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881488: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288148C: 485D825D  bl 0x82e596e8
	ctx.lr = 0x82881490;
	sub_82E596E8(ctx, base);
	// 82881490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881494: 48571F95  bl 0x82df3428
	ctx.lr = 0x82881498;
	sub_82DF3428(ctx, base);
	// 82881498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288149C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828814A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828814A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828814A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828814AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828814B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828814B0 size=444
    let mut pc: u32 = 0x828814B0;
    'dispatch: loop {
        match pc {
            0x828814B0 => {
    //   block [0x828814B0..0x8288166C)
	// 828814B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828814B4: 48926CB9  bl 0x831a816c
	ctx.lr = 0x828814B8;
	sub_831A8130(ctx, base);
	// 828814B8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 828814BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828814C0: 488D3141  bl 0x83154600
	ctx.lr = 0x828814C4;
	sub_83154600(ctx, base);
	// 828814C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828814C8: 4BF6C351  bl 0x827ed818
	ctx.lr = 0x828814CC;
	sub_827ED818(ctx, base);
	// 828814CC: 4BFF2545  bl 0x82873a10
	ctx.lr = 0x828814D0;
	sub_82873A10(ctx, base);
	// 828814D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828814D4: 40820100  bne 0x828815d4
	if !ctx.cr[0].eq {
	pc = 0x828815D4; continue 'dispatch;
	}
	// 828814D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828814DC: 4809528D  bl 0x82916768
	ctx.lr = 0x828814E0;
	sub_82916768(ctx, base);
	// 828814E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828814E4: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 828814E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828814EC: 4E800421  bctrl
	ctx.lr = 0x828814F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828814F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828814F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828814F8: C1BE007C  lfs f13, 0x7c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 828814FC: C01E0080  lfs f0, 0x80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82881500: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82881504: C1BE0078  lfs f13, 0x78(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82881508: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8288150C: 4809525D  bl 0x82916768
	ctx.lr = 0x82881510;
	sub_82916768(ctx, base);
	// 82881510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881514: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82881518: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8288151C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881520: 4E800421  bctrl
	ctx.lr = 0x82881524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881524: C01E0078  lfs f0, 0x78(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82881528: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8288152C: 40980010  bge cr6, 0x8288153c
	if !ctx.cr[6].lt {
	pc = 0x8288153C; continue 'dispatch;
	}
	// 82881530: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82881534: 808B0AFC  lwz r4, 0xafc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2812 as u32) ) } as u64;
	// 82881538: 4800010C  b 0x82881644
	pc = 0x82881644; continue 'dispatch;
	// 8288153C: EC010028  fsubs f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 82881540: C1BE007C  lfs f13, 0x7c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82881544: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82881548: 40980074  bge cr6, 0x828815bc
	if !ctx.cr[6].lt {
	pc = 0x828815BC; continue 'dispatch;
	}
	// 8288154C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82881550: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82881554: 48095845  bl 0x82916d98
	ctx.lr = 0x82881558;
	sub_82916D98(ctx, base);
	// 82881558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288155C: 4809520D  bl 0x82916768
	ctx.lr = 0x82881560;
	sub_82916768(ctx, base);
	// 82881560: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82881564: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881568: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8288156C: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82881570: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82881574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881578: 4E800421  bctrl
	ctx.lr = 0x8288157C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8288157C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82881580: C1BE00C4  lfs f13, 0xc4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82881584: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82881588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288158C: C00BD7BC  lfs f0, -0x2844(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82881590: EC01683A  fmadds f0, f1, f0, f13
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82881594: D01D0074  stfs f0, 0x74(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82881598: 808A0AF8  lwz r4, 0xaf8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2808 as u32) ) } as u64;
	// 8288159C: 4857246D  bl 0x82df3a08
	ctx.lr = 0x828815A0;
	sub_82DF3A08(ctx, base);
	// 828815A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828815A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828815A8: 4BF6E199  bl 0x827ef740
	ctx.lr = 0x828815AC;
	sub_827EF740(ctx, base);
	// 828815AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828815B0: 48571E79  bl 0x82df3428
	ctx.lr = 0x828815B4;
	sub_82DF3428(ctx, base);
	// 828815B4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828815B8: 4800006C  b 0x82881624
	pc = 0x82881624; continue 'dispatch;
	// 828815BC: EC006828  fsubs f0, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 828815C0: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 828815C4: C1BE0080  lfs f13, 0x80(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 828815C8: 808B0B18  lwz r4, 0xb18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2840 as u32) ) } as u64;
	// 828815CC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 828815D0: 48000074  b 0x82881644
	pc = 0x82881644; continue 'dispatch;
	// 828815D4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 828815D8: 409A005C  bne cr6, 0x82881634
	if !ctx.cr[6].eq {
	pc = 0x82881634; continue 'dispatch;
	}
	// 828815DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828815E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828815E4: 480957B5  bl 0x82916d98
	ctx.lr = 0x828815E8;
	sub_82916D98(ctx, base);
	// 828815E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828815EC: 83C10060  lwz r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 828815F0: 4BF6C229  bl 0x827ed818
	ctx.lr = 0x828815F4;
	sub_827ED818(ctx, base);
	// 828815F4: 4BFF2425  bl 0x82873a18
	ctx.lr = 0x828815F8;
	sub_82873A18(ctx, base);
	// 828815F8: 907E007C  stw r3, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 828815FC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82881600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881604: 808B0B00  lwz r4, 0xb00(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2816 as u32) ) } as u64;
	// 82881608: 48572401  bl 0x82df3a08
	ctx.lr = 0x8288160C;
	sub_82DF3A08(ctx, base);
	// 8288160C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82881610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881614: 4BF6E12D  bl 0x827ef740
	ctx.lr = 0x82881618;
	sub_827EF740(ctx, base);
	// 82881618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288161C: 48571E0D  bl 0x82df3428
	ctx.lr = 0x82881620;
	sub_82DF3428(ctx, base);
	// 82881620: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82881624: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82881628: 419A0038  beq cr6, 0x82881660
	if ctx.cr[6].eq {
	pc = 0x82881660; continue 'dispatch;
	}
	// 8288162C: 4BA3F265  bl 0x822c0890
	ctx.lr = 0x82881630;
	sub_822C0890(ctx, base);
	// 82881630: 48000030  b 0x82881660
	pc = 0x82881660; continue 'dispatch;
	// 82881634: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 82881638: 409A0028  bne cr6, 0x82881660
	if !ctx.cr[6].eq {
	pc = 0x82881660; continue 'dispatch;
	}
	// 8288163C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82881640: 808B95C0  lwz r4, -0x6a40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27200 as u32) ) } as u64;
	// 82881644: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881648: 485723C1  bl 0x82df3a08
	ctx.lr = 0x8288164C;
	sub_82DF3A08(ctx, base);
	// 8288164C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82881650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881654: 4BF6E0ED  bl 0x827ef740
	ctx.lr = 0x82881658;
	sub_827EF740(ctx, base);
	// 82881658: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288165C: 48571DCD  bl 0x82df3428
	ctx.lr = 0x82881660;
	sub_82DF3428(ctx, base);
	// 82881660: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82881664: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82881668: 48926B54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881670 size=112
    let mut pc: u32 = 0x82881670;
    'dispatch: loop {
        match pc {
            0x82881670 => {
    //   block [0x82881670..0x828816E0)
	// 82881670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881674: 48926AF9  bl 0x831a816c
	ctx.lr = 0x82881678;
	sub_831A8130(ctx, base);
	// 82881678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288167C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881680: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82881684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82881688: 388B4598  addi r4, r11, 0x4598
	ctx.r[4].s64 = ctx.r[11].s64 + 17816;
	// 8288168C: 38A0007A  li r5, 0x7a
	ctx.r[5].s64 = 122;
	// 82881690: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82881694: 48570D55  bl 0x82df23e8
	ctx.lr = 0x82881698;
	sub_82DF23E8(ctx, base);
	// 82881698: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288169C: 41820010  beq 0x828816ac
	if ctx.cr[0].eq {
	pc = 0x828816AC; continue 'dispatch;
	}
	// 828816A0: 4BFFFDB1  bl 0x82881450
	ctx.lr = 0x828816A4;
	sub_82881450(ctx, base);
	// 828816A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828816A8: 48000008  b 0x828816b0
	pc = 0x828816B0; continue 'dispatch;
	// 828816AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828816B0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 828816B4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 828816B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828816BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828816C0: 4BFFFB39  bl 0x828811f8
	ctx.lr = 0x828816C4;
	sub_828811F8(ctx, base);
	// 828816C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828816C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828816CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828816D0: 4BA3E931  bl 0x822c0000
	ctx.lr = 0x828816D4;
	sub_822C0000(ctx, base);
	// 828816D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828816D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828816DC: 48926AE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828816E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828816E0 size=112
    let mut pc: u32 = 0x828816E0;
    'dispatch: loop {
        match pc {
            0x828816E0 => {
    //   block [0x828816E0..0x82881750)
	// 828816E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828816E4: 48926A89  bl 0x831a816c
	ctx.lr = 0x828816E8;
	sub_831A8130(ctx, base);
	// 828816E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828816EC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828816F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828816F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828816F8: 388B4598  addi r4, r11, 0x4598
	ctx.r[4].s64 = ctx.r[11].s64 + 17816;
	// 828816FC: 38A000B1  li r5, 0xb1
	ctx.r[5].s64 = 177;
	// 82881700: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82881704: 48570CE5  bl 0x82df23e8
	ctx.lr = 0x82881708;
	sub_82DF23E8(ctx, base);
	// 82881708: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288170C: 41820010  beq 0x8288171c
	if ctx.cr[0].eq {
	pc = 0x8288171C; continue 'dispatch;
	}
	// 82881710: 4BFFF961  bl 0x82881070
	ctx.lr = 0x82881714;
	sub_82881070(ctx, base);
	// 82881714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881718: 48000008  b 0x82881720
	pc = 0x82881720; continue 'dispatch;
	// 8288171C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82881720: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82881724: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82881728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288172C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881730: 4BFFFB91  bl 0x828812c0
	ctx.lr = 0x82881734;
	sub_828812C0(ctx, base);
	// 82881734: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82881738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288173C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881740: 4BA3E8C1  bl 0x822c0000
	ctx.lr = 0x82881744;
	sub_822C0000(ctx, base);
	// 82881744: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82881748: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288174C: 48926A70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881750 size=112
    let mut pc: u32 = 0x82881750;
    'dispatch: loop {
        match pc {
            0x82881750 => {
    //   block [0x82881750..0x828817C0)
	// 82881750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881754: 48926A19  bl 0x831a816c
	ctx.lr = 0x82881758;
	sub_831A8130(ctx, base);
	// 82881758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288175C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881760: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82881764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82881768: 388B4598  addi r4, r11, 0x4598
	ctx.r[4].s64 = ctx.r[11].s64 + 17816;
	// 8288176C: 38A0012D  li r5, 0x12d
	ctx.r[5].s64 = 301;
	// 82881770: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82881774: 48570C75  bl 0x82df23e8
	ctx.lr = 0x82881778;
	sub_82DF23E8(ctx, base);
	// 82881778: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288177C: 41820010  beq 0x8288178c
	if ctx.cr[0].eq {
	pc = 0x8288178C; continue 'dispatch;
	}
	// 82881780: 4BFFFA19  bl 0x82881198
	ctx.lr = 0x82881784;
	sub_82881198(ctx, base);
	// 82881784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881788: 48000008  b 0x82881790
	pc = 0x82881790; continue 'dispatch;
	// 8288178C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82881790: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82881794: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82881798: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288179C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828817A0: 4BFFFBE9  bl 0x82881388
	ctx.lr = 0x828817A4;
	sub_82881388(ctx, base);
	// 828817A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828817A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828817AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828817B0: 4BA3E851  bl 0x822c0000
	ctx.lr = 0x828817B4;
	sub_822C0000(ctx, base);
	// 828817B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828817B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828817BC: 48926A00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828817C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828817C0 size=148
    let mut pc: u32 = 0x828817C0;
    'dispatch: loop {
        match pc {
            0x828817C0 => {
    //   block [0x828817C0..0x82881854)
	// 828817C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828817C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828817C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828817CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828817D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828817D4: 4BFFC455  bl 0x8287dc28
	ctx.lr = 0x828817D8;
	sub_8287DC28(ctx, base);
	// 828817D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 828817DC: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828817E0: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 828817E4: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 828817E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828817EC: 394A4630  addi r10, r10, 0x4630
	ctx.r[10].s64 = ctx.r[10].s64 + 17968;
	// 828817F0: 38E00090  li r7, 0x90
	ctx.r[7].s64 = 144;
	// 828817F4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828817F8: 3CC0832C  lis r6, -0x7cd4
	ctx.r[6].s64 = -2094268416;
	// 828817FC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82881800: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82881804: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881808: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8288180C: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82881810: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82881858 size=448
    let mut pc: u32 = 0x82881858;
    'dispatch: loop {
        match pc {
            0x82881858 => {
    //   block [0x82881858..0x82881A18)
	// 82881858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288185C: 48926909  bl 0x831a8164
	ctx.lr = 0x82881860;
	sub_831A8130(ctx, base);
	// 82881860: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881868: 488D2D99  bl 0x83154600
	ctx.lr = 0x8288186C;
	sub_83154600(ctx, base);
	// 8288186C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82881870: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881874: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82881878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288187C: 4E800421  bctrl
	ctx.lr = 0x82881880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881880: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82881884: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82881888: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8288188C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82881890: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881A18 size=196
    let mut pc: u32 = 0x82881A18;
    'dispatch: loop {
        match pc {
            0x82881A18 => {
    //   block [0x82881A18..0x82881ADC)
	// 82881A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82881A20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82881A24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82881A28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881A2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82881A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82881A34: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82881A38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82881A3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82881A40: 4BA3EEF9  bl 0x822c0938
	ctx.lr = 0x82881A44;
	sub_822C0938(ctx, base);
	// 82881A44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82881A48: 41820028  beq 0x82881a70
	if ctx.cr[0].eq {
	pc = 0x82881A70; continue 'dispatch;
	}
	// 82881A4C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881A50: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82881A54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82881A58: 392B461C  addi r9, r11, 0x461c
	ctx.r[9].s64 = ctx.r[11].s64 + 17948;
	// 82881A5C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82881A60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82881A64: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82881A68: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82881A6C: 48000008  b 0x82881a74
	pc = 0x82881A74; continue 'dispatch;
	// 82881A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82881A74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82881A78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82881A7C: 409A0044  bne cr6, 0x82881ac0
	if !ctx.cr[6].eq {
	pc = 0x82881AC0; continue 'dispatch;
	}
	// 82881A80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82881A84: 419A001C  beq cr6, 0x82881aa0
	if ctx.cr[6].eq {
	pc = 0x82881AA0; continue 'dispatch;
	}
	// 82881A88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881A8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82881A90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881A94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82881A98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881A9C: 4E800421  bctrl
	ctx.lr = 0x82881AA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881AA0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82881AA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82881AA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881AAC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82881AB0: 816BA0C8  lwz r11, -0x5f38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24376 as u32) ) } as u64;
	// 82881AB4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82881AB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82881ABC: 4BA3E545  bl 0x822c0000
	ctx.lr = 0x82881AC0;
	sub_822C0000(ctx, base);
	// 82881AC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881AC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82881AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82881ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82881AD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82881AD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82881AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82881AE0 size=112
    let mut pc: u32 = 0x82881AE0;
    'dispatch: loop {
        match pc {
            0x82881AE0 => {
    //   block [0x82881AE0..0x82881B50)
	// 82881AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881AE4: 48926689  bl 0x831a816c
	ctx.lr = 0x82881AE8;
	sub_831A8130(ctx, base);
	// 82881AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881AEC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881AF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82881AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82881AF8: 388B4658  addi r4, r11, 0x4658
	ctx.r[4].s64 = ctx.r[11].s64 + 18008;
	// 82881AFC: 38A000A8  li r5, 0xa8
	ctx.r[5].s64 = 168;
	// 82881B00: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 82881B04: 485708E5  bl 0x82df23e8
	ctx.lr = 0x82881B08;
	sub_82DF23E8(ctx, base);
	// 82881B08: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82881B0C: 41820010  beq 0x82881b1c
	if ctx.cr[0].eq {
	pc = 0x82881B1C; continue 'dispatch;
	}
	// 82881B10: 4BFFFCB1  bl 0x828817c0
	ctx.lr = 0x82881B14;
	sub_828817C0(ctx, base);
	// 82881B14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881B18: 48000008  b 0x82881b20
	pc = 0x82881B20; continue 'dispatch;
	// 82881B1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82881B20: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82881B24: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82881B28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82881B2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881B30: 4BFFFEE9  bl 0x82881a18
	ctx.lr = 0x82881B34;
	sub_82881A18(ctx, base);
	// 82881B34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82881B38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82881B3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881B40: 4BA3E4C1  bl 0x822c0000
	ctx.lr = 0x82881B44;
	sub_822C0000(ctx, base);
	// 82881B44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82881B48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82881B4C: 48926670  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82881B50 size=540
    let mut pc: u32 = 0x82881B50;
    'dispatch: loop {
        match pc {
            0x82881B50 => {
    //   block [0x82881B50..0x82881D6C)
	// 82881B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881B54: 48926609  bl 0x831a815c
	ctx.lr = 0x82881B58;
	sub_831A8130(ctx, base);
	// 82881B58: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881B5C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82881B60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82881B64: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82881B68: 388BDDCC  addi r4, r11, -0x2234
	ctx.r[4].s64 = ctx.r[11].s64 + -8756;
	// 82881B6C: 48146BED  bl 0x829c8758
	ctx.lr = 0x82881B70;
	sub_829C8758(ctx, base);
	// 82881B70: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82881B74: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82881B78: 48787441  bl 0x83008fb8
	ctx.lr = 0x82881B7C;
	sub_83008FB8(ctx, base);
	// 82881B7C: 907E006C  stw r3, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82881B80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881B84: 488D2A7D  bl 0x83154600
	ctx.lr = 0x82881B88;
	sub_83154600(ctx, base);
	// 82881B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881B8C: 3BBE0070  addi r29, r30, 0x70
	ctx.r[29].s64 = ctx.r[30].s64 + 112;
	// 82881B90: 4BF6BC89  bl 0x827ed818
	ctx.lr = 0x82881B94;
	sub_827ED818(ctx, base);
	// 82881B94: 8163039C  lwz r11, 0x39c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(924 as u32) ) } as u64;
	// 82881B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881B9C: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82881BA0: 48094BC9  bl 0x82916768
	ctx.lr = 0x82881BA4;
	sub_82916768(ctx, base);
	// 82881BA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881BA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82881BAC: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82881BB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881BB4: 4E800421  bctrl
	ctx.lr = 0x82881BB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881BBC: 48094BAD  bl 0x82916768
	ctx.lr = 0x82881BC0;
	sub_82916768(ctx, base);
	// 82881BC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881BC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82881BC8: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 82881BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881BD0: 4E800421  bctrl
	ctx.lr = 0x82881BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881BD4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82881BD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881BDC: 808B0BF0  lwz r4, 0xbf0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3056 as u32) ) } as u64;
	// 82881BE0: 48571E29  bl 0x82df3a08
	ctx.lr = 0x82881BE4;
	sub_82DF3A08(ctx, base);
	// 82881BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881BE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82881BEC: 4BF6DB85  bl 0x827ef770
	ctx.lr = 0x82881BF0;
	sub_827EF770(ctx, base);
	// 82881BF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881BF4: 48571835  bl 0x82df3428
	ctx.lr = 0x82881BF8;
	sub_82DF3428(ctx, base);
	// 82881BF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82881BFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881C00: 808B9598  lwz r4, -0x6a68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27240 as u32) ) } as u64;
	// 82881C04: 48571E05  bl 0x82df3a08
	ctx.lr = 0x82881C08;
	sub_82DF3A08(ctx, base);
	// 82881C08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881C0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82881C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881C14: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82881C18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881C1C: 4E800421  bctrl
	ctx.lr = 0x82881C20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881C24: 48571805  bl 0x82df3428
	ctx.lr = 0x82881C28;
	sub_82DF3428(ctx, base);
	// 82881C28: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82881C2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881C30: 4BFFB409  bl 0x8287d038
	ctx.lr = 0x82881C34;
	sub_8287D038(ctx, base);
	// 82881C34: 83810054  lwz r28, 0x54(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82881C38: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82881C3C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82881C40: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82881C44: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82881C48: 419A0024  beq cr6, 0x82881c6c
	if ctx.cr[6].eq {
	pc = 0x82881C6C; continue 'dispatch;
	}
	// 82881C4C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82881C50: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82881C54: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82881C58: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82881C5C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82881C60: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82881C64: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82881C68: 4082FFE8  bne 0x82881c50
	if !ctx.cr[0].eq {
	pc = 0x82881C50; continue 'dispatch;
	}
	// 82881C6C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881C70: 837E006C  lwz r27, 0x6c(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82881C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881C78: 3B4B4658  addi r26, r11, 0x4658
	ctx.r[26].s64 = ctx.r[11].s64 + 18008;
	// 82881C7C: 3B210060  addi r25, r1, 0x60
	ctx.r[25].s64 = ctx.r[1].s64 + 96;
	// 82881C80: 48094AE9  bl 0x82916768
	ctx.lr = 0x82881C84;
	sub_82916768(ctx, base);
	// 82881C84: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82881C88: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82881C8C: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82881C90: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82881C94: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82881C98: 485D5351  bl 0x82e56fe8
	ctx.lr = 0x82881C9C;
	sub_82E56FE8(ctx, base);
	// 82881C9C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82881CA0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82881CA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82881CA8: 419A000C  beq cr6, 0x82881cb4
	if ctx.cr[6].eq {
	pc = 0x82881CB4; continue 'dispatch;
	}
	// 82881CAC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82881CB0: 4BA3EBE1  bl 0x822c0890
	ctx.lr = 0x82881CB4;
	sub_822C0890(ctx, base);
	// 82881CB4: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82881CB8: 41820068  beq 0x82881d20
	if ctx.cr[0].eq {
	pc = 0x82881D20; continue 'dispatch;
	}
	// 82881CBC: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82881CC0: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 82881CC4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82881CC8: 39000090  li r8, 0x90
	ctx.r[8].s64 = 144;
	// 82881CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82881CD0: 13FD58C7  vcmpequd (lvx128) v31, v29, v11
	tmp.u32 = ctx.r[29].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82881D70 size=240
    let mut pc: u32 = 0x82881D70;
    'dispatch: loop {
        match pc {
            0x82881D70 => {
    //   block [0x82881D70..0x82881E60)
	// 82881D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881D74: 489263F9  bl 0x831a816c
	ctx.lr = 0x82881D78;
	sub_831A8130(ctx, base);
	// 82881D78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881D7C: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82881D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82881D84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881D88: 808B0BD8  lwz r4, 0xbd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3032 as u32) ) } as u64;
	// 82881D8C: 48571C7D  bl 0x82df3a08
	ctx.lr = 0x82881D90;
	sub_82DF3A08(ctx, base);
	// 82881D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881D94: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82881D98: 488D2869  bl 0x83154600
	ctx.lr = 0x82881D9C;
	sub_83154600(ctx, base);
	// 82881D9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82881DA0: 4BF6D9D1  bl 0x827ef770
	ctx.lr = 0x82881DA4;
	sub_827EF770(ctx, base);
	// 82881DA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82881DA8: 48571681  bl 0x82df3428
	ctx.lr = 0x82881DAC;
	sub_82DF3428(ctx, base);
	// 82881DAC: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82881DB0: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 82881DB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82881DB8: 419A00A0  beq cr6, 0x82881e58
	if ctx.cr[6].eq {
	pc = 0x82881E58; continue 'dispatch;
	}
	// 82881DBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82881DC0: 4BFFB2F1  bl 0x8287d0b0
	ctx.lr = 0x82881DC4;
	sub_8287D0B0(ctx, base);
	// 82881DC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881DC8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82881DCC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82881DD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82881DD4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82881DD8: 419A0024  beq cr6, 0x82881dfc
	if ctx.cr[6].eq {
	pc = 0x82881DFC; continue 'dispatch;
	}
	// 82881DDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82881DE0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82881DE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82881DE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82881DEC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82881DF0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82881DF4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82881DF8: 4082FFE8  bne 0x82881de0
	if !ctx.cr[0].eq {
	pc = 0x82881DE0; continue 'dispatch;
	}
	// 82881DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881E00: 488D2801  bl 0x83154600
	ctx.lr = 0x82881E04;
	sub_83154600(ctx, base);
	// 82881E04: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881E08: 83FF006C  lwz r31, 0x6c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82881E0C: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 82881E10: 3BCB4658  addi r30, r11, 0x4658
	ctx.r[30].s64 = ctx.r[11].s64 + 18008;
	// 82881E14: 4BF6BA05  bl 0x827ed818
	ctx.lr = 0x82881E18;
	sub_827ED818(ctx, base);
	// 82881E18: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82881E1C: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82881E20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82881E24: 38A0009F  li r5, 0x9f
	ctx.r[5].s64 = 159;
	// 82881E28: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82881E2C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82881E30: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82881E34: 485D6C0D  bl 0x82e58a40
	ctx.lr = 0x82881E38;
	sub_82E58A40(ctx, base);
	// 82881E38: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82881E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82881E40: 419A0008  beq cr6, 0x82881e48
	if ctx.cr[6].eq {
	pc = 0x82881E48; continue 'dispatch;
	}
	// 82881E44: 4BA3EA4D  bl 0x822c0890
	ctx.lr = 0x82881E48;
	sub_822C0890(ctx, base);
	// 82881E48: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82881E4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82881E50: 419A0008  beq cr6, 0x82881e58
	if ctx.cr[6].eq {
	pc = 0x82881E58; continue 'dispatch;
	}
	// 82881E54: 4BA3EA3D  bl 0x822c0890
	ctx.lr = 0x82881E58;
	sub_822C0890(ctx, base);
	// 82881E58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82881E5C: 48926360  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82881E60 size=364
    let mut pc: u32 = 0x82881E60;
    'dispatch: loop {
        match pc {
            0x82881E60 => {
    //   block [0x82881E60..0x82881FCC)
	// 82881E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881E64: 489262FD  bl 0x831a8160
	ctx.lr = 0x82881E68;
	sub_831A8130(ctx, base);
	// 82881E68: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881E6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82881E70: 488D2791  bl 0x83154600
	ctx.lr = 0x82881E74;
	sub_83154600(ctx, base);
	// 82881E74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82881E78: 389D0070  addi r4, r29, 0x70
	ctx.r[4].s64 = ctx.r[29].s64 + 112;
	// 82881E7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82881E80: 4BFFB1B9  bl 0x8287d038
	ctx.lr = 0x82881E84;
	sub_8287D038(ctx, base);
	// 82881E84: 83810064  lwz r28, 0x64(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82881E88: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82881E8C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82881E90: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82881E94: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82881E98: 419A0024  beq cr6, 0x82881ebc
	if ctx.cr[6].eq {
	pc = 0x82881EBC; continue 'dispatch;
	}
	// 82881E9C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82881EA0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82881EA4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82881EA8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82881EAC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82881EB0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82881EB4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82881EB8: 4082FFE8  bne 0x82881ea0
	if !ctx.cr[0].eq {
	pc = 0x82881EA0; continue 'dispatch;
	}
	// 82881EBC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82881EC0: 83BD006C  lwz r29, 0x6c(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 82881EC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881EC8: 3B6B4658  addi r27, r11, 0x4658
	ctx.r[27].s64 = ctx.r[11].s64 + 18008;
	// 82881ECC: 3B410050  addi r26, r1, 0x50
	ctx.r[26].s64 = ctx.r[1].s64 + 80;
	// 82881ED0: 48094899  bl 0x82916768
	ctx.lr = 0x82881ED4;
	sub_82916768(ctx, base);
	// 82881ED4: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82881ED8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82881EDC: 38A000CB  li r5, 0xcb
	ctx.r[5].s64 = 203;
	// 82881EE0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82881EE4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82881EE8: 485D5101  bl 0x82e56fe8
	ctx.lr = 0x82881EEC;
	sub_82E56FE8(ctx, base);
	// 82881EEC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82881EF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82881EF4: 419A0008  beq cr6, 0x82881efc
	if ctx.cr[6].eq {
	pc = 0x82881EFC; continue 'dispatch;
	}
	// 82881EF8: 4BA3E999  bl 0x822c0890
	ctx.lr = 0x82881EFC;
	sub_822C0890(ctx, base);
	// 82881EFC: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82881F00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82881F04: 418200A8  beq 0x82881fac
	if ctx.cr[0].eq {
	pc = 0x82881FAC; continue 'dispatch;
	}
	// 82881F08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881F0C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82881F10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881F14: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82881F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881F1C: 4E800421  bctrl
	ctx.lr = 0x82881F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881F20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82881F24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82881F28: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82881F2C: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82881F30: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82881F34: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82881F38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82881F3C: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82881F40: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82881F44: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82881F48: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82881F4C: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82881F50: C19F0038  lfs f12, 0x38(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82881F54: FD806050  fneg f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82881F58: C17F0034  lfs f11, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82881F5C: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 82881F60: C15F0030  lfs f10, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82881F64: FD405050  fneg f10, f10
	ctx.f[10].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 82881F68: C13F003C  lfs f9, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82881F6C: FD204850  fneg f9, f9
	ctx.f[9].u64 = ctx.f[9].u64 ^ 0x8000_0000_0000_0000u64;
	// 82881F70: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82881F74: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82881F78: D1410090  stfs f10, 0x90(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82881F7C: D1A10088  stfs f13, 0x88(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82881F80: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82881F84: D121009C  stfs f9, 0x9c(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82881F88: D1810098  stfs f12, 0x98(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82881F8C: D1610094  stfs f11, 0x94(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82881F90: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82881F94: 485FB1E5  bl 0x82e7d178
	ctx.lr = 0x82881F98;
	sub_82E7D178(ctx, base);
	// 82881F98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82881F9C: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82881FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82881FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82881FA8: 4E800421  bctrl
	ctx.lr = 0x82881FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82881FAC: 8BFF0040  lbz r31, 0x40(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82881FB0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82881FB4: 419A000C  beq cr6, 0x82881fc0
	if ctx.cr[6].eq {
	pc = 0x82881FC0; continue 'dispatch;
	}
	// 82881FB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82881FBC: 4BA3E8D5  bl 0x822c0890
	ctx.lr = 0x82881FC0;
	sub_822C0890(ctx, base);
	// 82881FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82881FC4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82881FC8: 489261E8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82881FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82881FD0 size=580
    let mut pc: u32 = 0x82881FD0;
    'dispatch: loop {
        match pc {
            0x82881FD0 => {
    //   block [0x82881FD0..0x82882214)
	// 82881FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82881FD4: 48926199  bl 0x831a816c
	ctx.lr = 0x82881FD8;
	sub_831A8130(ctx, base);
	// 82881FD8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82881FDC: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82881FE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82881FE4: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82881FE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82881FEC: 41980204  blt cr6, 0x828821f0
	if ctx.cr[6].lt {
	pc = 0x828821F0; continue 'dispatch;
	}
	// 82881FF0: 419A003C  beq cr6, 0x8288202c
	if ctx.cr[6].eq {
	pc = 0x8288202C; continue 'dispatch;
	}
	// 82881FF4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82881FF8: 40980210  bge cr6, 0x82882208
	if !ctx.cr[6].lt {
	pc = 0x82882208; continue 'dispatch;
	}
	// 82881FFC: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 82882000: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82882004: 808B0AF4  lwz r4, 0xaf4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2804 as u32) ) } as u64;
	// 82882008: 48571A01  bl 0x82df3a08
	ctx.lr = 0x8288200C;
	sub_82DF3A08(ctx, base);
	// 8288200C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882010: 3BE10054  addi r31, r1, 0x54
	ctx.r[31].s64 = ctx.r[1].s64 + 84;
	// 82882014: 488D25ED  bl 0x83154600
	ctx.lr = 0x82882018;
	sub_83154600(ctx, base);
	// 82882018: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288201C: 4BF6D725  bl 0x827ef740
	ctx.lr = 0x82882020;
	sub_827EF740(ctx, base);
	// 82882020: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82882024: 48571405  bl 0x82df3428
	ctx.lr = 0x82882028;
	sub_82DF3428(ctx, base);
	// 82882028: 480001E0  b 0x82882208
	pc = 0x82882208; continue 'dispatch;
	// 8288202C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882030: 4BFFFE31  bl 0x82881e60
	ctx.lr = 0x82882034;
	sub_82881E60(ctx, base);
	// 82882034: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882038: 418201B0  beq 0x828821e8
	if ctx.cr[0].eq {
	pc = 0x828821E8; continue 'dispatch;
	}
	// 8288203C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882040: 488D25C1  bl 0x83154600
	ctx.lr = 0x82882044;
	sub_83154600(ctx, base);
	// 82882044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882048: 4BF6B7D1  bl 0x827ed818
	ctx.lr = 0x8288204C;
	sub_827ED818(ctx, base);
	// 8288204C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882050: 816B01A0  lwz r11, 0x1a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(416 as u32) ) } as u64;
	// 82882054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882058: 4E800421  bctrl
	ctx.lr = 0x8288205C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8288205C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882064: 485D7715  bl 0x82e59778
	ctx.lr = 0x82882068;
	sub_82E59778(ctx, base);
	// 82882068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288206C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82882070: 4BF6D711  bl 0x827ef780
	ctx.lr = 0x82882074;
	sub_827EF780(ctx, base);
	// 82882074: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82882078: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8288207C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82882080: 4BF683C9  bl 0x827ea448
	ctx.lr = 0x82882084;
	sub_827EA448(ctx, base);
	// 82882084: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82882088: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8288208C: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 82882090: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82882094: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82882098: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8288209C: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 828820A0: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 828820A4: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 828820A8: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 828820AC: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 828820B0: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 828820B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828820B8: 13885C07  vcmpneb. (lvlx128) v28, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882218 size=196
    let mut pc: u32 = 0x82882218;
    'dispatch: loop {
        match pc {
            0x82882218 => {
    //   block [0x82882218..0x828822DC)
	// 82882218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82882220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82882224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82882228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288222C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82882230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82882234: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82882238: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8288223C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882240: 4BA3E6F9  bl 0x822c0938
	ctx.lr = 0x82882244;
	sub_822C0938(ctx, base);
	// 82882244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82882248: 41820028  beq 0x82882270
	if ctx.cr[0].eq {
	pc = 0x82882270; continue 'dispatch;
	}
	// 8288224C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882250: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82882254: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82882258: 392B46E0  addi r9, r11, 0x46e0
	ctx.r[9].s64 = ctx.r[11].s64 + 18144;
	// 8288225C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82882260: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82882264: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82882268: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8288226C: 48000008  b 0x82882274
	pc = 0x82882274; continue 'dispatch;
	// 82882270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82882274: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8288227C: 409A0044  bne cr6, 0x828822c0
	if !ctx.cr[6].eq {
	pc = 0x828822C0; continue 'dispatch;
	}
	// 82882280: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82882284: 419A001C  beq cr6, 0x828822a0
	if ctx.cr[6].eq {
	pc = 0x828822A0; continue 'dispatch;
	}
	// 82882288: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288228C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82882290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882294: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82882298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288229C: 4E800421  bctrl
	ctx.lr = 0x828822A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828822A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828822A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 828822A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828822AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 828822B0: 816BA164  lwz r11, -0x5e9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24220 as u32) ) } as u64;
	// 828822B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828822B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828822BC: 4BA3DD45  bl 0x822c0000
	ctx.lr = 0x828822C0;
	sub_822C0000(ctx, base);
	// 828822C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828822C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828822C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828822CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828822D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828822D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828822D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828822E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828822E0 size=196
    let mut pc: u32 = 0x828822E0;
    'dispatch: loop {
        match pc {
            0x828822E0 => {
    //   block [0x828822E0..0x828823A4)
	// 828822E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828822E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828822E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828822EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828822F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828822F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828822F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828822FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82882300: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82882304: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882308: 4BA3E631  bl 0x822c0938
	ctx.lr = 0x8288230C;
	sub_822C0938(ctx, base);
	// 8288230C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82882310: 41820028  beq 0x82882338
	if ctx.cr[0].eq {
	pc = 0x82882338; continue 'dispatch;
	}
	// 82882314: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882318: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8288231C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82882320: 392B46F4  addi r9, r11, 0x46f4
	ctx.r[9].s64 = ctx.r[11].s64 + 18164;
	// 82882324: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82882328: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8288232C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82882330: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82882334: 48000008  b 0x8288233c
	pc = 0x8288233C; continue 'dispatch;
	// 82882338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8288233C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882340: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82882344: 409A0044  bne cr6, 0x82882388
	if !ctx.cr[6].eq {
	pc = 0x82882388; continue 'dispatch;
	}
	// 82882348: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8288234C: 419A001C  beq cr6, 0x82882368
	if ctx.cr[6].eq {
	pc = 0x82882368; continue 'dispatch;
	}
	// 82882350: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882354: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82882358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288235C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82882360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882364: 4E800421  bctrl
	ctx.lr = 0x82882368;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882368: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8288236C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82882370: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882374: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82882378: 816BA164  lwz r11, -0x5e9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24220 as u32) ) } as u64;
	// 8288237C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82882380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82882384: 4BA3DC7D  bl 0x822c0000
	ctx.lr = 0x82882388;
	sub_822C0000(ctx, base);
	// 82882388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8288238C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82882390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82882394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82882398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8288239C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828823A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828823A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828823A8 size=108
    let mut pc: u32 = 0x828823A8;
    'dispatch: loop {
        match pc {
            0x828823A8 => {
    //   block [0x828823A8..0x82882414)
	// 828823A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828823AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828823B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828823B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828823B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828823BC: 4BFFB86D  bl 0x8287dc28
	ctx.lr = 0x828823C0;
	sub_8287DC28(ctx, base);
	// 828823C0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828823C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828823C8: 394A4708  addi r10, r10, 0x4708
	ctx.r[10].s64 = ctx.r[10].s64 + 18184;
	// 828823CC: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 828823D0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 828823D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828823D8: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828823DC: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 828823E0: 80890B24  lwz r4, 0xb24(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2852 as u32) ) } as u64;
	// 828823E4: 48571625  bl 0x82df3a08
	ctx.lr = 0x828823E8;
	sub_82DF3A08(ctx, base);
	// 828823E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828823EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828823F0: 485D72F9  bl 0x82e596e8
	ctx.lr = 0x828823F4;
	sub_82E596E8(ctx, base);
	// 828823F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828823F8: 48571031  bl 0x82df3428
	ctx.lr = 0x828823FC;
	sub_82DF3428(ctx, base);
	// 828823FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82882404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82882408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8288240C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82882410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82882418 size=696
    let mut pc: u32 = 0x82882418;
    'dispatch: loop {
        match pc {
            0x82882418 => {
    //   block [0x82882418..0x828826D0)
	// 82882418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288241C: 48925D41  bl 0x831a815c
	ctx.lr = 0x82882420;
	sub_831A8130(ctx, base);
	// 82882420: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882424: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82882428: 488D21D9  bl 0x83154600
	ctx.lr = 0x8288242C;
	sub_83154600(ctx, base);
	// 8288242C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82882430: 48094339  bl 0x82916768
	ctx.lr = 0x82882434;
	sub_82916768(ctx, base);
	// 82882434: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82882438: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8288243C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82882440: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82882444: 4808C935  bl 0x8290ed78
	ctx.lr = 0x82882448;
	sub_8290ED78(ctx, base);
	// 82882448: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288244C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882450: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82882454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882458: 4E800421  bctrl
	ctx.lr = 0x8288245C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8288245C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82882460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882464: 808B9558  lwz r4, -0x6aa8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27304 as u32) ) } as u64;
	// 82882468: 485715A1  bl 0x82df3a08
	ctx.lr = 0x8288246C;
	sub_82DF3A08(ctx, base);
	// 8288246C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82882474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882478: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8288247C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882480: 4E800421  bctrl
	ctx.lr = 0x82882484;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882488: 48570FA1  bl 0x82df3428
	ctx.lr = 0x8288248C;
	sub_82DF3428(ctx, base);
	// 8288248C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82882490: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82882494: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82882498: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288249C: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 828824A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828824A4: 4E800421  bctrl
	ctx.lr = 0x828824A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828824A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 828824AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 828824B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828824B4: C00B7BC4  lfs f0, 0x7bc4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31684 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828824B8: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 828824BC: D01B0060  stfs f0, 0x60(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 828824C0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 828824C4: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 828824C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828824CC: 4E800421  bctrl
	ctx.lr = 0x828824D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828824D0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 828824D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 828824D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 828824DC: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 828824E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828824E4: 4E800421  bctrl
	ctx.lr = 0x828824E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828824E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 828824EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 828824F0: 93BB0064  stw r29, 0x64(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 828824F4: 93BB0068  stw r29, 0x68(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 828824F8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 828824FC: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82882500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882504: 4E800421  bctrl
	ctx.lr = 0x82882508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882508: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288250C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882510: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82882514: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82882518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288251C: 4E800421  bctrl
	ctx.lr = 0x82882520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882520: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82882524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882528: 480BD869  bl 0x8293fd90
	ctx.lr = 0x8288252C;
	sub_8293FD90(ctx, base);
	// 8288252C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882530: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82882534: 3B4B4768  addi r26, r11, 0x4768
	ctx.r[26].s64 = ctx.r[11].s64 + 18280;
	// 82882538: 38A00062  li r5, 0x62
	ctx.r[5].s64 = 98;
	// 8288253C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82882540: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82882544: 4BA3DE95  bl 0x822c03d8
	ctx.lr = 0x82882548;
	sub_822C03D8(ctx, base);
	// 82882548: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288254C: 41820018  beq 0x82882564
	if ctx.cr[0].eq {
	pc = 0x82882564; continue 'dispatch;
	}
	// 82882550: 38B9013C  addi r5, r25, 0x13c
	ctx.r[5].s64 = ctx.r[25].s64 + 316;
	// 82882554: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82882558: 480BEAA9  bl 0x82941000
	ctx.lr = 0x8288255C;
	sub_82941000(ctx, base);
	// 8288255C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82882560: 48000008  b 0x82882568
	pc = 0x82882568; continue 'dispatch;
	// 82882564: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82882568: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8288256C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82882570: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82882574: 4BFEF7B5  bl 0x82871d28
	ctx.lr = 0x82882578;
	sub_82871D28(ctx, base);
	// 82882578: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8288257C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82882580: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82882584: 4BA3DA7D  bl 0x822c0000
	ctx.lr = 0x82882588;
	sub_822C0000(ctx, base);
	// 82882588: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288258C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882590: 480BD911  bl 0x8293fea0
	ctx.lr = 0x82882594;
	sub_8293FEA0(ctx, base);
	// 82882594: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82882598: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8288259C: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 828825A0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828825A4: 4BA3DE35  bl 0x822c03d8
	ctx.lr = 0x828825A8;
	sub_822C03D8(ctx, base);
	// 828825A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828825AC: 41820018  beq 0x828825c4
	if ctx.cr[0].eq {
	pc = 0x828825C4; continue 'dispatch;
	}
	// 828825B0: 38B90148  addi r5, r25, 0x148
	ctx.r[5].s64 = ctx.r[25].s64 + 328;
	// 828825B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828825B8: 480C09B9  bl 0x82942f70
	ctx.lr = 0x828825BC;
	sub_82942F70(ctx, base);
	// 828825BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 828825C0: 48000008  b 0x828825c8
	pc = 0x828825C8; continue 'dispatch;
	// 828825C4: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 828825C8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 828825CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 828825D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828825D4: 4BFEF81D  bl 0x82871df0
	ctx.lr = 0x828825D8;
	sub_82871DF0(ctx, base);
	// 828825D8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 828825DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 828825E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828825E4: 4BA3DA1D  bl 0x822c0000
	ctx.lr = 0x828825E8;
	sub_822C0000(ctx, base);
	// 828825E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828825EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828825F0: 480BD8B1  bl 0x8293fea0
	ctx.lr = 0x828825F4;
	sub_8293FEA0(ctx, base);
	// 828825F4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 828825F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828825FC: 38A00066  li r5, 0x66
	ctx.r[5].s64 = 102;
	// 82882600: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82882604: 4BA3DDD5  bl 0x822c03d8
	ctx.lr = 0x82882608;
	sub_822C03D8(ctx, base);
	// 82882608: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288260C: 41820018  beq 0x82882624
	if ctx.cr[0].eq {
	pc = 0x82882624; continue 'dispatch;
	}
	// 82882610: 38B90150  addi r5, r25, 0x150
	ctx.r[5].s64 = ctx.r[25].s64 + 336;
	// 82882614: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82882618: 480C0D71  bl 0x82943388
	ctx.lr = 0x8288261C;
	sub_82943388(ctx, base);
	// 8288261C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82882620: 48000008  b 0x82882628
	pc = 0x82882628; continue 'dispatch;
	// 82882624: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82882628: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8288262C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82882630: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82882634: 4BFEF885  bl 0x82871eb8
	ctx.lr = 0x82882638;
	sub_82871EB8(ctx, base);
	// 82882638: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8288263C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82882640: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82882644: 4BA3D9BD  bl 0x822c0000
	ctx.lr = 0x82882648;
	sub_822C0000(ctx, base);
	// 82882648: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288264C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882650: 480BD851  bl 0x8293fea0
	ctx.lr = 0x82882654;
	sub_8293FEA0(ctx, base);
	// 82882654: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82882658: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8288265C: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 82882660: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82882664: 4BA3DD75  bl 0x822c03d8
	ctx.lr = 0x82882668;
	sub_822C03D8(ctx, base);
	// 82882668: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8288266C: 41820014  beq 0x82882680
	if ctx.cr[0].eq {
	pc = 0x82882680; continue 'dispatch;
	}
	// 82882670: 38B90160  addi r5, r25, 0x160
	ctx.r[5].s64 = ctx.r[25].s64 + 352;
	// 82882674: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82882678: 480C02A9  bl 0x82942920
	ctx.lr = 0x8288267C;
	sub_82942920(ctx, base);
	// 8288267C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882680: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82882684: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82882688: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8288268C: 4BFEF8F5  bl 0x82871f80
	ctx.lr = 0x82882690;
	sub_82871F80(ctx, base);
	// 82882690: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82882694: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82882698: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8288269C: 4BA3D965  bl 0x822c0000
	ctx.lr = 0x828826A0;
	sub_822C0000(ctx, base);
	// 828826A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828826A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828826A8: 480BD7F9  bl 0x8293fea0
	ctx.lr = 0x828826AC;
	sub_8293FEA0(ctx, base);
	// 828826AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 828826B0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 828826B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828826B8: 48094969  bl 0x82917020
	ctx.lr = 0x828826BC;
	sub_82917020(ctx, base);
	// 828826BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828826C0: 48094441  bl 0x82916b00
	ctx.lr = 0x828826C4;
	sub_82916B00(ctx, base);
	// 828826C4: 987B006C  stb r3, 0x6c(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(108 as u32), ctx.r[3].u8 ) };
	// 828826C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 828826CC: 48925AE0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828826D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828826D0 size=112
    let mut pc: u32 = 0x828826D0;
    'dispatch: loop {
        match pc {
            0x828826D0 => {
    //   block [0x828826D0..0x82882740)
	// 828826D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828826D4: 48925A99  bl 0x831a816c
	ctx.lr = 0x828826D8;
	sub_831A8130(ctx, base);
	// 828826D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828826DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828826E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 828826E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 828826E8: 388B4768  addi r4, r11, 0x4768
	ctx.r[4].s64 = ctx.r[11].s64 + 18280;
	// 828826EC: 38A00124  li r5, 0x124
	ctx.r[5].s64 = 292;
	// 828826F0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 828826F4: 4856FCF5  bl 0x82df23e8
	ctx.lr = 0x828826F8;
	sub_82DF23E8(ctx, base);
	// 828826F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828826FC: 41820010  beq 0x8288270c
	if ctx.cr[0].eq {
	pc = 0x8288270C; continue 'dispatch;
	}
	// 82882700: 4BFFFCA9  bl 0x828823a8
	ctx.lr = 0x82882704;
	sub_828823A8(ctx, base);
	// 82882704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882708: 48000008  b 0x82882710
	pc = 0x82882710; continue 'dispatch;
	// 8288270C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82882710: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82882714: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82882718: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288271C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882720: 4BFFFAF9  bl 0x82882218
	ctx.lr = 0x82882724;
	sub_82882218(ctx, base);
	// 82882724: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82882728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288272C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882730: 4BA3D8D1  bl 0x822c0000
	ctx.lr = 0x82882734;
	sub_822C0000(ctx, base);
	// 82882734: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82882738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288273C: 48925A80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882740 size=124
    let mut pc: u32 = 0x82882740;
    'dispatch: loop {
        match pc {
            0x82882740 => {
    //   block [0x82882740..0x828827BC)
	// 82882740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882744: 48925A29  bl 0x831a816c
	ctx.lr = 0x82882748;
	sub_831A8130(ctx, base);
	// 82882748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288274C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882750: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82882758: 388B4768  addi r4, r11, 0x4768
	ctx.r[4].s64 = ctx.r[11].s64 + 18280;
	// 8288275C: 38A00144  li r5, 0x144
	ctx.r[5].s64 = 324;
	// 82882760: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82882764: 4856FC85  bl 0x82df23e8
	ctx.lr = 0x82882768;
	sub_82DF23E8(ctx, base);
	// 82882768: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8288276C: 4182001C  beq 0x82882788
	if ctx.cr[0].eq {
	pc = 0x82882788; continue 'dispatch;
	}
	// 82882770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882774: 4BFFFC35  bl 0x828823a8
	ctx.lr = 0x82882778;
	sub_828823A8(ctx, base);
	// 82882778: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288277C: 396B4738  addi r11, r11, 0x4738
	ctx.r[11].s64 = ctx.r[11].s64 + 18232;
	// 82882780: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882784: 48000008  b 0x8288278c
	pc = 0x8288278C; continue 'dispatch;
	// 82882788: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8288278C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82882790: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82882794: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82882798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8288279C: 4BFFFB45  bl 0x828822e0
	ctx.lr = 0x828827A0;
	sub_828822E0(ctx, base);
	// 828827A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828827A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828827A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828827AC: 4BA3D855  bl 0x822c0000
	ctx.lr = 0x828827B0;
	sub_822C0000(ctx, base);
	// 828827B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 828827B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828827B8: 48925A04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828827C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828827C0 size=412
    let mut pc: u32 = 0x828827C0;
    'dispatch: loop {
        match pc {
            0x828827C0 => {
    //   block [0x828827C0..0x8288295C)
	// 828827C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828827C4: 489259A9  bl 0x831a816c
	ctx.lr = 0x828827C8;
	sub_831A8130(ctx, base);
	// 828827C8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 828827CC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828827D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828827D4: 488D1E2D  bl 0x83154600
	ctx.lr = 0x828827D8;
	sub_83154600(ctx, base);
	// 828827D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828827DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828827E0: 485D6F99  bl 0x82e59778
	ctx.lr = 0x828827E4;
	sub_82E59778(ctx, base);
	// 828827E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828827E8: C01F00C0  lfs f0, 0xc0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 828827EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828827F0: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 828827F4: D01F00C0  stfs f0, 0xc0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 828827F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 828827FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882800: 4E800421  bctrl
	ctx.lr = 0x82882804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882804: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882808: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8288280C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82882810: 4BFF9049  bl 0x8287b858
	ctx.lr = 0x82882814;
	sub_8287B858(ctx, base);
	// 82882814: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82882818: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8288281C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82882820: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82882824: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82882828: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882960 size=476
    let mut pc: u32 = 0x82882960;
    'dispatch: loop {
        match pc {
            0x82882960 => {
    //   block [0x82882960..0x82882B3C)
	// 82882960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882964: 48925805  bl 0x831a8168
	ctx.lr = 0x82882968;
	sub_831A8130(ctx, base);
	// 82882968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288296C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82882970: 488D1C91  bl 0x83154600
	ctx.lr = 0x82882974;
	sub_83154600(ctx, base);
	// 82882974: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882978: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288297C: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82882980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882984: 4E800421  bctrl
	ctx.lr = 0x82882988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882988: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8288298C: 3BFE0068  addi r31, r30, 0x68
	ctx.r[31].s64 = ctx.r[30].s64 + 104;
	// 82882990: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882994: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82882998: 41820010  beq 0x828829a8
	if ctx.cr[0].eq {
	pc = 0x828829A8; continue 'dispatch;
	}
	// 8288299C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828829A0: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828829A4: 48000138  b 0x82882adc
	pc = 0x82882ADC; continue 'dispatch;
	// 828829A8: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 828829AC: 556AD7FF  rlwinm. r10, r11, 0x1a, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 828829B0: 4182001C  beq 0x828829cc
	if ctx.cr[0].eq {
	pc = 0x828829CC; continue 'dispatch;
	}
	// 828829B4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 828829B8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 828829BC: 556B06B0  rlwinm r11, r11, 0, 0x1a, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 828829C0: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828829C4: 808A9560  lwz r4, -0x6aa0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27296 as u32) ) } as u64;
	// 828829C8: 480000E0  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 828829CC: 556ADFFF  rlwinm. r10, r11, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 828829D0: 4182001C  beq 0x828829ec
	if ctx.cr[0].eq {
	pc = 0x828829EC; continue 'dispatch;
	}
	// 828829D4: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 828829D8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 828829DC: 556B06F2  rlwinm r11, r11, 0, 0x1b, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 828829E0: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828829E4: 808A9568  lwz r4, -0x6a98(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27288 as u32) ) } as u64;
	// 828829E8: 480000C0  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 828829EC: 556AF7FF  rlwinm. r10, r11, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 828829F0: 4182001C  beq 0x82882a0c
	if ctx.cr[0].eq {
	pc = 0x82882A0C; continue 'dispatch;
	}
	// 828829F4: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 828829F8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 828829FC: 556B07B8  rlwinm r11, r11, 0, 0x1e, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882A00: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82882A04: 808A9564  lwz r4, -0x6a9c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27292 as u32) ) } as u64;
	// 82882A08: 480000A0  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 82882A0C: 556AFFFF  rlwinm. r10, r11, 0x1f, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82882A10: 4182001C  beq 0x82882a2c
	if ctx.cr[0].eq {
	pc = 0x82882A2C; continue 'dispatch;
	}
	// 82882A14: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82882A18: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82882A1C: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882A20: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82882A24: 808A956C  lwz r4, -0x6a94(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27284 as u32) ) } as u64;
	// 82882A28: 48000080  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 82882A2C: 556AC7FF  rlwinm. r10, r11, 0x18, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82882A30: 4182001C  beq 0x82882a4c
	if ctx.cr[0].eq {
	pc = 0x82882A4C; continue 'dispatch;
	}
	// 82882A34: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82882A38: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82882A3C: 556B062C  rlwinm r11, r11, 0, 0x18, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882A40: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82882A44: 808A9570  lwz r4, -0x6a90(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27280 as u32) ) } as u64;
	// 82882A48: 48000060  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 82882A4C: 556AE7FF  rlwinm. r10, r11, 0x1c, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82882A50: 4182001C  beq 0x82882a6c
	if ctx.cr[0].eq {
	pc = 0x82882A6C; continue 'dispatch;
	}
	// 82882A54: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82882A58: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82882A5C: 556B0734  rlwinm r11, r11, 0, 0x1c, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882A60: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82882A64: 808A9574  lwz r4, -0x6a8c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27276 as u32) ) } as u64;
	// 82882A68: 48000040  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 82882A6C: 556ACFFF  rlwinm. r10, r11, 0x19, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82882A70: 4182001C  beq 0x82882a8c
	if ctx.cr[0].eq {
	pc = 0x82882A8C; continue 'dispatch;
	}
	// 82882A74: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82882A78: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82882A7C: 556B066E  rlwinm r11, r11, 0, 0x19, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882A80: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82882A84: 808A9578  lwz r4, -0x6a88(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27272 as u32) ) } as u64;
	// 82882A88: 48000020  b 0x82882aa8
	pc = 0x82882AA8; continue 'dispatch;
	// 82882A8C: 556BEFFF  rlwinm. r11, r11, 0x1d, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882A90: 4182004C  beq 0x82882adc
	if ctx.cr[0].eq {
	pc = 0x82882ADC; continue 'dispatch;
	}
	// 82882A94: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82882A98: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 82882A9C: 556B0776  rlwinm r11, r11, 0, 0x1d, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882AA0: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82882AA4: 808A957C  lwz r4, -0x6a84(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27268 as u32) ) } as u64;
	// 82882AA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882AAC: 48570F5D  bl 0x82df3a08
	ctx.lr = 0x82882AB0;
	sub_82DF3A08(ctx, base);
	// 82882AB0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882AB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82882AB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82882ABC: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82882AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882AC4: 4E800421  bctrl
	ctx.lr = 0x82882AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882ACC: 4857095D  bl 0x82df3428
	ctx.lr = 0x82882AD0;
	sub_82DF3428(ctx, base);
	// 82882AD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882AD4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82882AD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882ADC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882AE0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882AE4: 41820050  beq 0x82882b34
	if ctx.cr[0].eq {
	pc = 0x82882B34; continue 'dispatch;
	}
	// 82882AE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82882AEC: 4BF67825  bl 0x827ea310
	ctx.lr = 0x82882AF0;
	sub_827EA310(ctx, base);
	// 82882AF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882AF4: 41820040  beq 0x82882b34
	if ctx.cr[0].eq {
	pc = 0x82882B34; continue 'dispatch;
	}
	// 82882AF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82882AFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882B00: 808B9558  lwz r4, -0x6aa8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27304 as u32) ) } as u64;
	// 82882B04: 48570F05  bl 0x82df3a08
	ctx.lr = 0x82882B08;
	sub_82DF3A08(ctx, base);
	// 82882B08: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882B0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82882B10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82882B14: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82882B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882B1C: 4E800421  bctrl
	ctx.lr = 0x82882B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882B20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882B24: 48570905  bl 0x82df3428
	ctx.lr = 0x82882B28;
	sub_82DF3428(ctx, base);
	// 82882B28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882B2C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882B30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82882B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82882B38: 48925680  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82882B40 size=204
    let mut pc: u32 = 0x82882B40;
    'dispatch: loop {
        match pc {
            0x82882B40 => {
    //   block [0x82882B40..0x82882C0C)
	// 82882B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82882B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82882B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82882B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882B58: 4805F461  bl 0x828e1fb8
	ctx.lr = 0x82882B5C;
	sub_828E1FB8(ctx, base);
	// 82882B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882B60: 488D1AA1  bl 0x83154600
	ctx.lr = 0x82882B64;
	sub_83154600(ctx, base);
	// 82882B64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82882B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882B6C: 485D6C0D  bl 0x82e59778
	ctx.lr = 0x82882B70;
	sub_82E59778(ctx, base);
	// 82882B70: C01E00C4  lfs f0, 0xc4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82882B74: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82882B78: D01E00C4  stfs f0, 0xc4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82882B7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82882B80: 4BF6AC99  bl 0x827ed818
	ctx.lr = 0x82882B84;
	sub_827ED818(ctx, base);
	// 82882B84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882B88: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 82882B8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882B90: 4E800421  bctrl
	ctx.lr = 0x82882B94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882B94: C01E00C4  lfs f0, 0xc4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82882B98: C1A301B4  lfs f13, 0x1b4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(436 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82882B9C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82882BA0: 40990054  ble cr6, 0x82882bf4
	if !ctx.cr[6].gt {
	pc = 0x82882BF4; continue 'dispatch;
	}
	// 82882BA4: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82882BA8: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882BAC: 41820048  beq 0x82882bf4
	if ctx.cr[0].eq {
	pc = 0x82882BF4; continue 'dispatch;
	}
	// 82882BB0: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82882BB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82882BB8: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 82882BBC: 556B07FA  rlwinm r11, r11, 0, 0x1f, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82882BC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882BC4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82882BC8: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82882BCC: D01E00C4  stfs f0, 0xc4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82882BD0: 80890AF4  lwz r4, 0xaf4(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2804 as u32) ) } as u64;
	// 82882BD4: 48570E35  bl 0x82df3a08
	ctx.lr = 0x82882BD8;
	sub_82DF3A08(ctx, base);
	// 82882BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882BDC: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82882BE0: 488D1A21  bl 0x83154600
	ctx.lr = 0x82882BE4;
	sub_83154600(ctx, base);
	// 82882BE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82882BE8: 4BF6CB59  bl 0x827ef740
	ctx.lr = 0x82882BEC;
	sub_827EF740(ctx, base);
	// 82882BEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882BF0: 48570839  bl 0x82df3428
	ctx.lr = 0x82882BF4;
	sub_82DF3428(ctx, base);
	// 82882BF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82882BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82882BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82882C00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82882C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82882C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882C10 size=12
    let mut pc: u32 = 0x82882C10;
    'dispatch: loop {
        match pc {
            0x82882C10 => {
    //   block [0x82882C10..0x82882C1C)
	// 82882C10: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82882C14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82882C18: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882C1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882C1C size=8
    let mut pc: u32 = 0x82882C1C;
    'dispatch: loop {
        match pc {
            0x82882C1C => {
    //   block [0x82882C1C..0x82882C24)
	// 82882C1C: 480106BC  b 0x828932d8
	sub_828932D8(ctx, base);
	return;
	// 82882C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882C28 size=204
    let mut pc: u32 = 0x82882C28;
    'dispatch: loop {
        match pc {
            0x82882C28 => {
    //   block [0x82882C28..0x82882CF4)
	// 82882C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882C2C: 48925541  bl 0x831a816c
	ctx.lr = 0x82882C30;
	sub_831A8130(ctx, base);
	// 82882C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882C34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882C38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82882C3C: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82882C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882C44: 4E800421  bctrl
	ctx.lr = 0x82882C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882C48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82882C4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82882C50: 388B5D2C  addi r4, r11, 0x5d2c
	ctx.r[4].s64 = ctx.r[11].s64 + 23852;
	// 82882C54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882C58: 48570DB1  bl 0x82df3a08
	ctx.lr = 0x82882C5C;
	sub_82DF3A08(ctx, base);
	// 82882C5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82882C60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882C64: 485706A5  bl 0x82df3308
	ctx.lr = 0x82882C68;
	sub_82DF3308(ctx, base);
	// 82882C68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882C6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882C70: 485707B9  bl 0x82df3428
	ctx.lr = 0x82882C74;
	sub_82DF3428(ctx, base);
	// 82882C74: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882C78: 40820070  bne 0x82882ce8
	if !ctx.cr[0].eq {
	pc = 0x82882CE8; continue 'dispatch;
	}
	// 82882C7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82882C80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882C84: 388B7C94  addi r4, r11, 0x7c94
	ctx.r[4].s64 = ctx.r[11].s64 + 31892;
	// 82882C88: 48570D81  bl 0x82df3a08
	ctx.lr = 0x82882C8C;
	sub_82DF3A08(ctx, base);
	// 82882C8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82882C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882C94: 48570675  bl 0x82df3308
	ctx.lr = 0x82882C98;
	sub_82DF3308(ctx, base);
	// 82882C98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82882C9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882CA0: 48570789  bl 0x82df3428
	ctx.lr = 0x82882CA4;
	sub_82DF3428(ctx, base);
	// 82882CA4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882CA8: 4182000C  beq 0x82882cb4
	if ctx.cr[0].eq {
	pc = 0x82882CB4; continue 'dispatch;
	}
	// 82882CAC: 387E0198  addi r3, r30, 0x198
	ctx.r[3].s64 = ctx.r[30].s64 + 408;
	// 82882CB0: 4800003C  b 0x82882cec
	pc = 0x82882CEC; continue 'dispatch;
	// 82882CB4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882CB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882CBC: 388B4918  addi r4, r11, 0x4918
	ctx.r[4].s64 = ctx.r[11].s64 + 18712;
	// 82882CC0: 48570D49  bl 0x82df3a08
	ctx.lr = 0x82882CC4;
	sub_82DF3A08(ctx, base);
	// 82882CC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82882CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882CCC: 4857063D  bl 0x82df3308
	ctx.lr = 0x82882CD0;
	sub_82DF3308(ctx, base);
	// 82882CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882CD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882CD8: 48570751  bl 0x82df3428
	ctx.lr = 0x82882CDC;
	sub_82DF3428(ctx, base);
	// 82882CDC: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82882CE0: 387E01A8  addi r3, r30, 0x1a8
	ctx.r[3].s64 = ctx.r[30].s64 + 424;
	// 82882CE4: 40820008  bne 0x82882cec
	if !ctx.cr[0].eq {
	pc = 0x82882CEC; continue 'dispatch;
	}
	// 82882CE8: 387E0188  addi r3, r30, 0x188
	ctx.r[3].s64 = ctx.r[30].s64 + 392;
	// 82882CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82882CF0: 489254CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882CF8 size=16
    let mut pc: u32 = 0x82882CF8;
    'dispatch: loop {
        match pc {
            0x82882CF8 => {
    //   block [0x82882CF8..0x82882D08)
	// 82882CF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882CFC: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82882D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882D04: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882D08 size=16
    let mut pc: u32 = 0x82882D08;
    'dispatch: loop {
        match pc {
            0x82882D08 => {
    //   block [0x82882D08..0x82882D18)
	// 82882D08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882D0C: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 82882D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882D14: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882D18 size=48
    let mut pc: u32 = 0x82882D18;
    'dispatch: loop {
        match pc {
            0x82882D18 => {
    //   block [0x82882D18..0x82882D48)
	// 82882D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82882D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82882D28: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 82882D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82882D30: 4E800421  bctrl
	ctx.lr = 0x82882D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82882D34: 386300AC  addi r3, r3, 0xac
	ctx.r[3].s64 = ctx.r[3].s64 + 172;
	// 82882D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82882D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82882D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82882D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882D48 size=12
    let mut pc: u32 = 0x82882D48;
    'dispatch: loop {
        match pc {
            0x82882D48 => {
    //   block [0x82882D48..0x82882D54)
	// 82882D48: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882D4C: 386BFCCC  addi r3, r11, -0x334
	ctx.r[3].s64 = ctx.r[11].s64 + -820;
	// 82882D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882D58 size=4
    let mut pc: u32 = 0x82882D58;
    'dispatch: loop {
        match pc {
            0x82882D58 => {
    //   block [0x82882D58..0x82882D5C)
	// 82882D58: 4BC8FE80  b 0x82512bd8
	sub_82512BD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882D60 size=8
    let mut pc: u32 = 0x82882D60;
    'dispatch: loop {
        match pc {
            0x82882D60 => {
    //   block [0x82882D60..0x82882D68)
	// 82882D60: 806303E8  lwz r3, 0x3e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1000 as u32) ) } as u64;
	// 82882D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882D68 size=12
    let mut pc: u32 = 0x82882D68;
    'dispatch: loop {
        match pc {
            0x82882D68 => {
    //   block [0x82882D68..0x82882D74)
	// 82882D68: 816303D0  lwz r11, 0x3d0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(976 as u32) ) } as u64;
	// 82882D6C: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82882D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882D78 size=136
    let mut pc: u32 = 0x82882D78;
    'dispatch: loop {
        match pc {
            0x82882D78 => {
    //   block [0x82882D78..0x82882E00)
	// 82882D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82882D80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82882D84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82882D88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882D8C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82882D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882D98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82882D9C: 808BA288  lwz r4, -0x5d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23928 as u32) ) } as u64;
	// 82882DA0: 48570C69  bl 0x82df3a08
	ctx.lr = 0x82882DA4;
	sub_82DF3A08(ctx, base);
	// 82882DA4: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 82882DA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882DB0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882DB4: 4808B09D  bl 0x8290de50
	ctx.lr = 0x82882DB8;
	sub_8290DE50(ctx, base);
	// 82882DB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882DBC: 4857066D  bl 0x82df3428
	ctx.lr = 0x82882DC0;
	sub_82DF3428(ctx, base);
	// 82882DC0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882DC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882DC8: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 82882DCC: 48570C3D  bl 0x82df3a08
	ctx.lr = 0x82882DD0;
	sub_82DF3A08(ctx, base);
	// 82882DD0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882DD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882DDC: 4808B14D  bl 0x8290df28
	ctx.lr = 0x82882DE0;
	sub_8290DF28(ctx, base);
	// 82882DE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882DE4: 48570645  bl 0x82df3428
	ctx.lr = 0x82882DE8;
	sub_82DF3428(ctx, base);
	// 82882DE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82882DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82882DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82882DF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82882DF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82882DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882E00 size=12
    let mut pc: u32 = 0x82882E00;
    'dispatch: loop {
        match pc {
            0x82882E00 => {
    //   block [0x82882E00..0x82882E0C)
	// 82882E00: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882E04: 386BFCD8  addi r3, r11, -0x328
	ctx.r[3].s64 = ctx.r[11].s64 + -808;
	// 82882E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882E10 size=24
    let mut pc: u32 = 0x82882E10;
    'dispatch: loop {
        match pc {
            0x82882E10 => {
    //   block [0x82882E10..0x82882E28)
	// 82882E10: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 82882E14: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882E18: 396BFDE0  addi r11, r11, -0x220
	ctx.r[11].s64 = ctx.r[11].s64 + -544;
	// 82882E1C: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 82882E20: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82882E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882E28 size=152
    let mut pc: u32 = 0x82882E28;
    'dispatch: loop {
        match pc {
            0x82882E28 => {
    //   block [0x82882E28..0x82882EC0)
	// 82882E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882E2C: 48925341  bl 0x831a816c
	ctx.lr = 0x82882E30;
	sub_831A8130(ctx, base);
	// 82882E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882E34: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82882E38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882E3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882E40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82882E44: 3BABA288  addi r29, r11, -0x5d78
	ctx.r[29].s64 = ctx.r[11].s64 + -23928;
	// 82882E48: 808BA288  lwz r4, -0x5d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23928 as u32) ) } as u64;
	// 82882E4C: 48570BBD  bl 0x82df3a08
	ctx.lr = 0x82882E50;
	sub_82DF3A08(ctx, base);
	// 82882E50: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 82882E54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882E5C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882E60: 4808AFF1  bl 0x8290de50
	ctx.lr = 0x82882E64;
	sub_8290DE50(ctx, base);
	// 82882E64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882E68: 485705C1  bl 0x82df3428
	ctx.lr = 0x82882E6C;
	sub_82DF3428(ctx, base);
	// 82882E6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882E70: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82882E74: 48570B95  bl 0x82df3a08
	ctx.lr = 0x82882E78;
	sub_82DF3A08(ctx, base);
	// 82882E78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882E80: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882E84: 4808AFCD  bl 0x8290de50
	ctx.lr = 0x82882E88;
	sub_8290DE50(ctx, base);
	// 82882E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882E8C: 4857059D  bl 0x82df3428
	ctx.lr = 0x82882E90;
	sub_82DF3428(ctx, base);
	// 82882E90: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882E94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882E98: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 82882E9C: 48570B6D  bl 0x82df3a08
	ctx.lr = 0x82882EA0;
	sub_82DF3A08(ctx, base);
	// 82882EA0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882EA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882EAC: 4808B07D  bl 0x8290df28
	ctx.lr = 0x82882EB0;
	sub_8290DF28(ctx, base);
	// 82882EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882EB4: 48570575  bl 0x82df3428
	ctx.lr = 0x82882EB8;
	sub_82DF3428(ctx, base);
	// 82882EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82882EBC: 48925300  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82882EC0 size=52
    let mut pc: u32 = 0x82882EC0;
    'dispatch: loop {
        match pc {
            0x82882EC0 => {
    //   block [0x82882EC0..0x82882EF4)
	// 82882EC0: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82882EC4: C1A30410  lfs f13, 0x410(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1040 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82882EC8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82882ECC: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82882ED0: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82882ED4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82882ED8: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82882EDC: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82882EE0: EDAD6028  fsubs f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82882EE4: D1A30410  stfs f13, 0x410(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1040 as u32), tmp.u32 ) };
	// 82882EE8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82882EEC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82882EF0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882EF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882EF4 size=8
    let mut pc: u32 = 0x82882EF4;
    'dispatch: loop {
        match pc {
            0x82882EF4 => {
    //   block [0x82882EF4..0x82882EFC)
	// 82882EF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82882EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882F00 size=12
    let mut pc: u32 = 0x82882F00;
    'dispatch: loop {
        match pc {
            0x82882F00 => {
    //   block [0x82882F00..0x82882F0C)
	// 82882F00: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882F04: 386B1370  addi r3, r11, 0x1370
	ctx.r[3].s64 = ctx.r[11].s64 + 4976;
	// 82882F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882F10 size=24
    let mut pc: u32 = 0x82882F10;
    'dispatch: loop {
        match pc {
            0x82882F10 => {
    //   block [0x82882F10..0x82882F28)
	// 82882F10: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 82882F14: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882F18: 396B1478  addi r11, r11, 0x1478
	ctx.r[11].s64 = ctx.r[11].s64 + 5240;
	// 82882F1C: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 82882F20: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82882F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882F28 size=152
    let mut pc: u32 = 0x82882F28;
    'dispatch: loop {
        match pc {
            0x82882F28 => {
    //   block [0x82882F28..0x82882FC0)
	// 82882F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882F2C: 48925241  bl 0x831a816c
	ctx.lr = 0x82882F30;
	sub_831A8130(ctx, base);
	// 82882F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882F34: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82882F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882F3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882F40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82882F44: 3BABA288  addi r29, r11, -0x5d78
	ctx.r[29].s64 = ctx.r[11].s64 + -23928;
	// 82882F48: 808BA288  lwz r4, -0x5d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23928 as u32) ) } as u64;
	// 82882F4C: 48570ABD  bl 0x82df3a08
	ctx.lr = 0x82882F50;
	sub_82DF3A08(ctx, base);
	// 82882F50: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 82882F54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882F58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882F5C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882F60: 4808AEF1  bl 0x8290de50
	ctx.lr = 0x82882F64;
	sub_8290DE50(ctx, base);
	// 82882F64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882F68: 485704C1  bl 0x82df3428
	ctx.lr = 0x82882F6C;
	sub_82DF3428(ctx, base);
	// 82882F6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882F70: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82882F74: 48570A95  bl 0x82df3a08
	ctx.lr = 0x82882F78;
	sub_82DF3A08(ctx, base);
	// 82882F78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882F80: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882F84: 4808AECD  bl 0x8290de50
	ctx.lr = 0x82882F88;
	sub_8290DE50(ctx, base);
	// 82882F88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882F8C: 4857049D  bl 0x82df3428
	ctx.lr = 0x82882F90;
	sub_82DF3428(ctx, base);
	// 82882F90: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82882F94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882F98: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 82882F9C: 48570A6D  bl 0x82df3a08
	ctx.lr = 0x82882FA0;
	sub_82DF3A08(ctx, base);
	// 82882FA0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82882FA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82882FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82882FAC: 4808AF7D  bl 0x8290df28
	ctx.lr = 0x82882FB0;
	sub_8290DF28(ctx, base);
	// 82882FB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82882FB4: 48570475  bl 0x82df3428
	ctx.lr = 0x82882FB8;
	sub_82DF3428(ctx, base);
	// 82882FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82882FBC: 48925200  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882FC0 size=12
    let mut pc: u32 = 0x82882FC0;
    'dispatch: loop {
        match pc {
            0x82882FC0 => {
    //   block [0x82882FC0..0x82882FCC)
	// 82882FC0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882FC4: 386B2A08  addi r3, r11, 0x2a08
	ctx.r[3].s64 = ctx.r[11].s64 + 10760;
	// 82882FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82882FD0 size=24
    let mut pc: u32 = 0x82882FD0;
    'dispatch: loop {
        match pc {
            0x82882FD0 => {
    //   block [0x82882FD0..0x82882FE8)
	// 82882FD0: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 82882FD4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82882FD8: 396B2B10  addi r11, r11, 0x2b10
	ctx.r[11].s64 = ctx.r[11].s64 + 11024;
	// 82882FDC: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 82882FE0: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82882FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82882FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82882FE8 size=152
    let mut pc: u32 = 0x82882FE8;
    'dispatch: loop {
        match pc {
            0x82882FE8 => {
    //   block [0x82882FE8..0x82883080)
	// 82882FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82882FEC: 48925181  bl 0x831a816c
	ctx.lr = 0x82882FF0;
	sub_831A8130(ctx, base);
	// 82882FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82882FF4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82882FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82882FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883000: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82883004: 3BABA288  addi r29, r11, -0x5d78
	ctx.r[29].s64 = ctx.r[11].s64 + -23928;
	// 82883008: 808BA288  lwz r4, -0x5d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23928 as u32) ) } as u64;
	// 8288300C: 485709FD  bl 0x82df3a08
	ctx.lr = 0x82883010;
	sub_82DF3A08(ctx, base);
	// 82883010: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 82883014: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288301C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883020: 4808AE31  bl 0x8290de50
	ctx.lr = 0x82883024;
	sub_8290DE50(ctx, base);
	// 82883024: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883028: 48570401  bl 0x82df3428
	ctx.lr = 0x8288302C;
	sub_82DF3428(ctx, base);
	// 8288302C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883030: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82883034: 485709D5  bl 0x82df3a08
	ctx.lr = 0x82883038;
	sub_82DF3A08(ctx, base);
	// 82883038: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8288303C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883040: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883044: 4808AE0D  bl 0x8290de50
	ctx.lr = 0x82883048;
	sub_8290DE50(ctx, base);
	// 82883048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288304C: 485703DD  bl 0x82df3428
	ctx.lr = 0x82883050;
	sub_82DF3428(ctx, base);
	// 82883050: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883058: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 8288305C: 485709AD  bl 0x82df3a08
	ctx.lr = 0x82883060;
	sub_82DF3A08(ctx, base);
	// 82883060: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883064: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288306C: 4808AEBD  bl 0x8290df28
	ctx.lr = 0x82883070;
	sub_8290DF28(ctx, base);
	// 82883070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883074: 485703B5  bl 0x82df3428
	ctx.lr = 0x82883078;
	sub_82DF3428(ctx, base);
	// 82883078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8288307C: 48925140  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883080 size=12
    let mut pc: u32 = 0x82883080;
    'dispatch: loop {
        match pc {
            0x82883080 => {
    //   block [0x82883080..0x8288308C)
	// 82883080: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82883084: 386B40A0  addi r3, r11, 0x40a0
	ctx.r[3].s64 = ctx.r[11].s64 + 16544;
	// 82883088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883090 size=24
    let mut pc: u32 = 0x82883090;
    'dispatch: loop {
        match pc {
            0x82883090 => {
    //   block [0x82883090..0x828830A8)
	// 82883090: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 82883094: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82883098: 396B41A8  addi r11, r11, 0x41a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16808;
	// 8288309C: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 828830A0: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 828830A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828830A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828830A8 size=188
    let mut pc: u32 = 0x828830A8;
    'dispatch: loop {
        match pc {
            0x828830A8 => {
    //   block [0x828830A8..0x82883164)
	// 828830A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828830AC: 489250C1  bl 0x831a816c
	ctx.lr = 0x828830B0;
	sub_831A8130(ctx, base);
	// 828830B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828830B4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828830B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828830BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828830C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828830C4: 3BABA288  addi r29, r11, -0x5d78
	ctx.r[29].s64 = ctx.r[11].s64 + -23928;
	// 828830C8: 808BA288  lwz r4, -0x5d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23928 as u32) ) } as u64;
	// 828830CC: 4857093D  bl 0x82df3a08
	ctx.lr = 0x828830D0;
	sub_82DF3A08(ctx, base);
	// 828830D0: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 828830D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828830D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828830DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828830E0: 4808AD71  bl 0x8290de50
	ctx.lr = 0x828830E4;
	sub_8290DE50(ctx, base);
	// 828830E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828830E8: 48570341  bl 0x82df3428
	ctx.lr = 0x828830EC;
	sub_82DF3428(ctx, base);
	// 828830EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828830F0: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 828830F4: 48570915  bl 0x82df3a08
	ctx.lr = 0x828830F8;
	sub_82DF3A08(ctx, base);
	// 828830F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828830FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883100: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883104: 4808AD4D  bl 0x8290de50
	ctx.lr = 0x82883108;
	sub_8290DE50(ctx, base);
	// 82883108: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288310C: 4857031D  bl 0x82df3428
	ctx.lr = 0x82883110;
	sub_82DF3428(ctx, base);
	// 82883110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883114: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82883118: 485708F1  bl 0x82df3a08
	ctx.lr = 0x8288311C;
	sub_82DF3A08(ctx, base);
	// 8288311C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883124: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883128: 4808AD29  bl 0x8290de50
	ctx.lr = 0x8288312C;
	sub_8290DE50(ctx, base);
	// 8288312C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883130: 485702F9  bl 0x82df3428
	ctx.lr = 0x82883134;
	sub_82DF3428(ctx, base);
	// 82883134: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883138: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288313C: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 82883140: 485708C9  bl 0x82df3a08
	ctx.lr = 0x82883144;
	sub_82DF3A08(ctx, base);
	// 82883144: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883148: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8288314C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883150: 4808ADD9  bl 0x8290df28
	ctx.lr = 0x82883154;
	sub_8290DF28(ctx, base);
	// 82883154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883158: 485702D1  bl 0x82df3428
	ctx.lr = 0x8288315C;
	sub_82DF3428(ctx, base);
	// 8288315C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82883160: 4892505C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883168 size=12
    let mut pc: u32 = 0x82883168;
    'dispatch: loop {
        match pc {
            0x82883168 => {
    //   block [0x82883168..0x82883174)
	// 82883168: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 8288316C: 386B5738  addi r3, r11, 0x5738
	ctx.r[3].s64 = ctx.r[11].s64 + 22328;
	// 82883170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883178 size=24
    let mut pc: u32 = 0x82883178;
    'dispatch: loop {
        match pc {
            0x82883178 => {
    //   block [0x82883178..0x82883190)
	// 82883178: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 8288317C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82883180: 396B5840  addi r11, r11, 0x5840
	ctx.r[11].s64 = ctx.r[11].s64 + 22592;
	// 82883184: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 82883188: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8288318C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883190 size=188
    let mut pc: u32 = 0x82883190;
    'dispatch: loop {
        match pc {
            0x82883190 => {
    //   block [0x82883190..0x8288324C)
	// 82883190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883194: 48924FD9  bl 0x831a816c
	ctx.lr = 0x82883198;
	sub_831A8130(ctx, base);
	// 82883198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288319C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828831A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828831A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828831A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828831AC: 3BABA288  addi r29, r11, -0x5d78
	ctx.r[29].s64 = ctx.r[11].s64 + -23928;
	// 828831B0: 808BA288  lwz r4, -0x5d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23928 as u32) ) } as u64;
	// 828831B4: 48570855  bl 0x82df3a08
	ctx.lr = 0x828831B8;
	sub_82DF3A08(ctx, base);
	// 828831B8: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 828831BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828831C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828831C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828831C8: 4808AC89  bl 0x8290de50
	ctx.lr = 0x828831CC;
	sub_8290DE50(ctx, base);
	// 828831CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828831D0: 48570259  bl 0x82df3428
	ctx.lr = 0x828831D4;
	sub_82DF3428(ctx, base);
	// 828831D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828831D8: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 828831DC: 4857082D  bl 0x82df3a08
	ctx.lr = 0x828831E0;
	sub_82DF3A08(ctx, base);
	// 828831E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828831E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828831E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828831EC: 4808AC65  bl 0x8290de50
	ctx.lr = 0x828831F0;
	sub_8290DE50(ctx, base);
	// 828831F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828831F4: 48570235  bl 0x82df3428
	ctx.lr = 0x828831F8;
	sub_82DF3428(ctx, base);
	// 828831F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828831FC: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82883200: 48570809  bl 0x82df3a08
	ctx.lr = 0x82883204;
	sub_82DF3A08(ctx, base);
	// 82883204: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288320C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883210: 4808AC41  bl 0x8290de50
	ctx.lr = 0x82883214;
	sub_8290DE50(ctx, base);
	// 82883214: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883218: 48570211  bl 0x82df3428
	ctx.lr = 0x8288321C;
	sub_82DF3428(ctx, base);
	// 8288321C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883224: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 82883228: 485707E1  bl 0x82df3a08
	ctx.lr = 0x8288322C;
	sub_82DF3A08(ctx, base);
	// 8288322C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883238: 4808ACF1  bl 0x8290df28
	ctx.lr = 0x8288323C;
	sub_8290DF28(ctx, base);
	// 8288323C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883240: 485701E9  bl 0x82df3428
	ctx.lr = 0x82883244;
	sub_82DF3428(ctx, base);
	// 82883244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82883248: 48924F74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883250 size=12
    let mut pc: u32 = 0x82883250;
    'dispatch: loop {
        match pc {
            0x82883250 => {
    //   block [0x82883250..0x8288325C)
	// 82883250: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82883254: 386B6DD0  addi r3, r11, 0x6dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 28112;
	// 82883258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883260 size=24
    let mut pc: u32 = 0x82883260;
    'dispatch: loop {
        match pc {
            0x82883260 => {
    //   block [0x82883260..0x82883278)
	// 82883260: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 82883264: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82883268: 396B6ED8  addi r11, r11, 0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + 28376;
	// 8288326C: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 82883270: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82883274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883278 size=140
    let mut pc: u32 = 0x82883278;
    'dispatch: loop {
        match pc {
            0x82883278 => {
    //   block [0x82883278..0x82883304)
	// 82883278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288328C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82883294: 396BA288  addi r11, r11, -0x5d78
	ctx.r[11].s64 = ctx.r[11].s64 + -23928;
	// 82883298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288329C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 828832A0: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 828832A4: 48570765  bl 0x82df3a08
	ctx.lr = 0x828832A8;
	sub_82DF3A08(ctx, base);
	// 828832A8: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 828832AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828832B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828832B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828832B8: 4808AB99  bl 0x8290de50
	ctx.lr = 0x828832BC;
	sub_8290DE50(ctx, base);
	// 828832BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828832C0: 48570169  bl 0x82df3428
	ctx.lr = 0x828832C4;
	sub_82DF3428(ctx, base);
	// 828832C4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828832C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828832CC: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 828832D0: 48570739  bl 0x82df3a08
	ctx.lr = 0x828832D4;
	sub_82DF3A08(ctx, base);
	// 828832D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828832D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 828832DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828832E0: 4808AC49  bl 0x8290df28
	ctx.lr = 0x828832E4;
	sub_8290DF28(ctx, base);
	// 828832E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828832E8: 48570141  bl 0x82df3428
	ctx.lr = 0x828832EC;
	sub_82DF3428(ctx, base);
	// 828832EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828832F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828832F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828832F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828832FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883308 size=12
    let mut pc: u32 = 0x82883308;
    'dispatch: loop {
        match pc {
            0x82883308 => {
    //   block [0x82883308..0x82883314)
	// 82883308: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 8288330C: 386B8468  addi r3, r11, -0x7b98
	ctx.r[3].s64 = ctx.r[11].s64 + -31640;
	// 82883310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883318 size=24
    let mut pc: u32 = 0x82883318;
    'dispatch: loop {
        match pc {
            0x82883318 => {
    //   block [0x82883318..0x82883330)
	// 82883318: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 8288331C: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 82883320: 396B8570  addi r11, r11, -0x7a90
	ctx.r[11].s64 = ctx.r[11].s64 + -31376;
	// 82883324: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 82883328: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8288332C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883330 size=140
    let mut pc: u32 = 0x82883330;
    'dispatch: loop {
        match pc {
            0x82883330 => {
    //   block [0x82883330..0x828833BC)
	// 82883330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8288333C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883340: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883344: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8288334C: 396BA288  addi r11, r11, -0x5d78
	ctx.r[11].s64 = ctx.r[11].s64 + -23928;
	// 82883350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883354: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82883358: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8288335C: 485706AD  bl 0x82df3a08
	ctx.lr = 0x82883360;
	sub_82DF3A08(ctx, base);
	// 82883360: 3BFFFCA0  addi r31, r31, -0x360
	ctx.r[31].s64 = ctx.r[31].s64 + -864;
	// 82883364: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288336C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883370: 4808AAE1  bl 0x8290de50
	ctx.lr = 0x82883374;
	sub_8290DE50(ctx, base);
	// 82883374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883378: 485700B1  bl 0x82df3428
	ctx.lr = 0x8288337C;
	sub_82DF3428(ctx, base);
	// 8288337C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883384: 388B4920  addi r4, r11, 0x4920
	ctx.r[4].s64 = ctx.r[11].s64 + 18720;
	// 82883388: 48570681  bl 0x82df3a08
	ctx.lr = 0x8288338C;
	sub_82DF3A08(ctx, base);
	// 8288338C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82883390: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82883394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883398: 4808AB91  bl 0x8290df28
	ctx.lr = 0x8288339C;
	sub_8290DF28(ctx, base);
	// 8288339C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828833A0: 48570089  bl 0x82df3428
	ctx.lr = 0x828833A4;
	sub_82DF3428(ctx, base);
	// 828833A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828833A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828833AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828833B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828833B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828833B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828833C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828833C0 size=12
    let mut pc: u32 = 0x828833C0;
    'dispatch: loop {
        match pc {
            0x828833C0 => {
    //   block [0x828833C0..0x828833CC)
	// 828833C0: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 828833C4: 386B9B00  addi r3, r11, -0x6500
	ctx.r[3].s64 = ctx.r[11].s64 + -25856;
	// 828833C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828833D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828833D0 size=24
    let mut pc: u32 = 0x828833D0;
    'dispatch: loop {
        match pc {
            0x828833D0 => {
    //   block [0x828833D0..0x828833E8)
	// 828833D0: 814303E4  lwz r10, 0x3e4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(996 as u32) ) } as u64;
	// 828833D4: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 828833D8: 396B9C08  addi r11, r11, -0x63f8
	ctx.r[11].s64 = ctx.r[11].s64 + -25592;
	// 828833DC: 1D4A0228  mulli r10, r10, 0x228
	ctx.r[10].s64 = ctx.r[10].s64 * 552;
	// 828833E0: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 828833E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828833E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828833E8 size=116
    let mut pc: u32 = 0x828833E8;
    'dispatch: loop {
        match pc {
            0x828833E8 => {
    //   block [0x828833E8..0x8288345C)
	// 828833E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828833EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828833F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828833F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828833F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828833FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883400: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883404: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82883408: 409A000C  bne cr6, 0x82883414
	if !ctx.cr[6].eq {
	pc = 0x82883414; continue 'dispatch;
	}
	// 8288340C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883410: 48000030  b 0x82883440
	pc = 0x82883440; continue 'dispatch;
	// 82883414: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82883418: 419A0024  beq cr6, 0x8288343c
	if ctx.cr[6].eq {
	pc = 0x8288343C; continue 'dispatch;
	}
	// 8288341C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883420: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883424: 388BA6D0  addi r4, r11, -0x5930
	ctx.r[4].s64 = ctx.r[11].s64 + -22832;
	// 82883428: 48924CD1  bl 0x831a80f8
	ctx.lr = 0x8288342C;
	sub_831A80F8(ctx, base);
	// 8288342C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82883430: 4182000C  beq 0x8288343c
	if ctx.cr[0].eq {
	pc = 0x8288343C; continue 'dispatch;
	}
	// 82883434: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82883438: 4800000C  b 0x82883444
	pc = 0x82883444; continue 'dispatch;
	// 8288343C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883440: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288344C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883460 size=136
    let mut pc: u32 = 0x82883460;
    'dispatch: loop {
        match pc {
            0x82883460 => {
    //   block [0x82883460..0x828834E8)
	// 82883460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8288346C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883474: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883478: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8288347C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82883480: 409A0020  bne cr6, 0x828834a0
	if !ctx.cr[6].eq {
	pc = 0x828834A0; continue 'dispatch;
	}
	// 82883484: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883488: 419A0048  beq cr6, 0x828834d0
	if ctx.cr[6].eq {
	pc = 0x828834D0; continue 'dispatch;
	}
	// 8288348C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82883490: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82883494: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82883498: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8288349C: 48000034  b 0x828834d0
	pc = 0x828834D0; continue 'dispatch;
	// 828834A0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 828834A4: 419A002C  beq cr6, 0x828834d0
	if ctx.cr[6].eq {
	pc = 0x828834D0; continue 'dispatch;
	}
	// 828834A8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828834AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828834B0: 388BA720  addi r4, r11, -0x58e0
	ctx.r[4].s64 = ctx.r[11].s64 + -22752;
	// 828834B4: 48924C45  bl 0x831a80f8
	ctx.lr = 0x828834B8;
	sub_831A80F8(ctx, base);
	// 828834B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828834BC: 4182000C  beq 0x828834c8
	if ctx.cr[0].eq {
	pc = 0x828834C8; continue 'dispatch;
	}
	// 828834C0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 828834C4: 4800000C  b 0x828834d0
	pc = 0x828834D0; continue 'dispatch;
	// 828834C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828834CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828834D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828834D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828834D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828834DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828834E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828834E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828834E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828834E8 size=136
    let mut pc: u32 = 0x828834E8;
    'dispatch: loop {
        match pc {
            0x828834E8 => {
    //   block [0x828834E8..0x82883570)
	// 828834E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828834EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828834F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828834F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828834F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828834FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883500: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883504: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82883508: 409A0020  bne cr6, 0x82883528
	if !ctx.cr[6].eq {
	pc = 0x82883528; continue 'dispatch;
	}
	// 8288350C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883510: 419A0048  beq cr6, 0x82883558
	if ctx.cr[6].eq {
	pc = 0x82883558; continue 'dispatch;
	}
	// 82883514: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82883518: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8288351C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82883520: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82883524: 48000034  b 0x82883558
	pc = 0x82883558; continue 'dispatch;
	// 82883528: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8288352C: 419A002C  beq cr6, 0x82883558
	if ctx.cr[6].eq {
	pc = 0x82883558; continue 'dispatch;
	}
	// 82883530: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883534: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883538: 388BA7F8  addi r4, r11, -0x5808
	ctx.r[4].s64 = ctx.r[11].s64 + -22536;
	// 8288353C: 48924BBD  bl 0x831a80f8
	ctx.lr = 0x82883540;
	sub_831A80F8(ctx, base);
	// 82883540: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82883544: 4182000C  beq 0x82883550
	if ctx.cr[0].eq {
	pc = 0x82883550; continue 'dispatch;
	}
	// 82883548: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8288354C: 4800000C  b 0x82883558
	pc = 0x82883558; continue 'dispatch;
	// 82883550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883554: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883558: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288355C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883564: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8288356C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883570 size=136
    let mut pc: u32 = 0x82883570;
    'dispatch: loop {
        match pc {
            0x82883570 => {
    //   block [0x82883570..0x828835F8)
	// 82883570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8288357C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883588: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8288358C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82883590: 409A0020  bne cr6, 0x828835b0
	if !ctx.cr[6].eq {
	pc = 0x828835B0; continue 'dispatch;
	}
	// 82883594: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883598: 419A0048  beq cr6, 0x828835e0
	if ctx.cr[6].eq {
	pc = 0x828835E0; continue 'dispatch;
	}
	// 8288359C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 828835A0: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 828835A4: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 828835A8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 828835AC: 48000034  b 0x828835e0
	pc = 0x828835E0; continue 'dispatch;
	// 828835B0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 828835B4: 419A002C  beq cr6, 0x828835e0
	if ctx.cr[6].eq {
	pc = 0x828835E0; continue 'dispatch;
	}
	// 828835B8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828835BC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828835C0: 388BA8D8  addi r4, r11, -0x5728
	ctx.r[4].s64 = ctx.r[11].s64 + -22312;
	// 828835C4: 48924B35  bl 0x831a80f8
	ctx.lr = 0x828835C8;
	sub_831A80F8(ctx, base);
	// 828835C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828835CC: 4182000C  beq 0x828835d8
	if ctx.cr[0].eq {
	pc = 0x828835D8; continue 'dispatch;
	}
	// 828835D0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 828835D4: 4800000C  b 0x828835e0
	pc = 0x828835E0; continue 'dispatch;
	// 828835D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828835DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828835E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828835E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828835E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828835EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828835F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828835F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828835F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828835F8 size=136
    let mut pc: u32 = 0x828835F8;
    'dispatch: loop {
        match pc {
            0x828835F8 => {
    //   block [0x828835F8..0x82883680)
	// 828835F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828835FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288360C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883610: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883614: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82883618: 409A0020  bne cr6, 0x82883638
	if !ctx.cr[6].eq {
	pc = 0x82883638; continue 'dispatch;
	}
	// 8288361C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883620: 419A0048  beq cr6, 0x82883668
	if ctx.cr[6].eq {
	pc = 0x82883668; continue 'dispatch;
	}
	// 82883624: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82883628: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8288362C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82883630: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82883634: 48000034  b 0x82883668
	pc = 0x82883668; continue 'dispatch;
	// 82883638: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8288363C: 419A002C  beq cr6, 0x82883668
	if ctx.cr[6].eq {
	pc = 0x82883668; continue 'dispatch;
	}
	// 82883640: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883644: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883648: 388BA9D0  addi r4, r11, -0x5630
	ctx.r[4].s64 = ctx.r[11].s64 + -22064;
	// 8288364C: 48924AAD  bl 0x831a80f8
	ctx.lr = 0x82883650;
	sub_831A80F8(ctx, base);
	// 82883650: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82883654: 4182000C  beq 0x82883660
	if ctx.cr[0].eq {
	pc = 0x82883660; continue 'dispatch;
	}
	// 82883658: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8288365C: 4800000C  b 0x82883668
	pc = 0x82883668; continue 'dispatch;
	// 82883660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883664: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883668: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288366C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883674: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8288367C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883680 size=136
    let mut pc: u32 = 0x82883680;
    'dispatch: loop {
        match pc {
            0x82883680 => {
    //   block [0x82883680..0x82883708)
	// 82883680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8288368C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883698: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8288369C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 828836A0: 409A0020  bne cr6, 0x828836c0
	if !ctx.cr[6].eq {
	pc = 0x828836C0; continue 'dispatch;
	}
	// 828836A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828836A8: 419A0048  beq cr6, 0x828836f0
	if ctx.cr[6].eq {
	pc = 0x828836F0; continue 'dispatch;
	}
	// 828836AC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 828836B0: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 828836B4: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 828836B8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 828836BC: 48000034  b 0x828836f0
	pc = 0x828836F0; continue 'dispatch;
	// 828836C0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 828836C4: 419A002C  beq cr6, 0x828836f0
	if ctx.cr[6].eq {
	pc = 0x828836F0; continue 'dispatch;
	}
	// 828836C8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828836CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828836D0: 388BAA90  addi r4, r11, -0x5570
	ctx.r[4].s64 = ctx.r[11].s64 + -21872;
	// 828836D4: 48924A25  bl 0x831a80f8
	ctx.lr = 0x828836D8;
	sub_831A80F8(ctx, base);
	// 828836D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828836DC: 4182000C  beq 0x828836e8
	if ctx.cr[0].eq {
	pc = 0x828836E8; continue 'dispatch;
	}
	// 828836E0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 828836E4: 4800000C  b 0x828836f0
	pc = 0x828836F0; continue 'dispatch;
	// 828836E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828836EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828836F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828836F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828836F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828836FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883708 size=136
    let mut pc: u32 = 0x82883708;
    'dispatch: loop {
        match pc {
            0x82883708 => {
    //   block [0x82883708..0x82883790)
	// 82883708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288371C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883720: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883724: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82883728: 409A0020  bne cr6, 0x82883748
	if !ctx.cr[6].eq {
	pc = 0x82883748; continue 'dispatch;
	}
	// 8288372C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883730: 419A0048  beq cr6, 0x82883778
	if ctx.cr[6].eq {
	pc = 0x82883778; continue 'dispatch;
	}
	// 82883734: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82883738: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8288373C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82883740: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82883744: 48000034  b 0x82883778
	pc = 0x82883778; continue 'dispatch;
	// 82883748: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8288374C: 419A002C  beq cr6, 0x82883778
	if ctx.cr[6].eq {
	pc = 0x82883778; continue 'dispatch;
	}
	// 82883750: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883754: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883758: 388BAB50  addi r4, r11, -0x54b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21680;
	// 8288375C: 4892499D  bl 0x831a80f8
	ctx.lr = 0x82883760;
	sub_831A80F8(ctx, base);
	// 82883760: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82883764: 4182000C  beq 0x82883770
	if ctx.cr[0].eq {
	pc = 0x82883770; continue 'dispatch;
	}
	// 82883768: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8288376C: 4800000C  b 0x82883778
	pc = 0x82883778; continue 'dispatch;
	// 82883770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883774: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883778: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288377C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883784: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8288378C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883790 size=16
    let mut pc: u32 = 0x82883790;
    'dispatch: loop {
        match pc {
            0x82883790 => {
    //   block [0x82883790..0x828837A0)
	// 82883790: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883794: 816C01CC  lwz r11, 0x1cc(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(460 as u32) ) } as u64;
	// 82883798: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288379C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828837A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828837A0 size=16
    let mut pc: u32 = 0x828837A0;
    'dispatch: loop {
        match pc {
            0x828837A0 => {
    //   block [0x828837A0..0x828837B0)
	// 828837A0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828837A4: 816C01D0  lwz r11, 0x1d0(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(464 as u32) ) } as u64;
	// 828837A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828837AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828837B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828837B0 size=16
    let mut pc: u32 = 0x828837B0;
    'dispatch: loop {
        match pc {
            0x828837B0 => {
    //   block [0x828837B0..0x828837C0)
	// 828837B0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828837B4: 816C01D4  lwz r11, 0x1d4(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(468 as u32) ) } as u64;
	// 828837B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828837BC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828837C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828837C0 size=16
    let mut pc: u32 = 0x828837C0;
    'dispatch: loop {
        match pc {
            0x828837C0 => {
    //   block [0x828837C0..0x828837D0)
	// 828837C0: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828837C4: 816C01D8  lwz r11, 0x1d8(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(472 as u32) ) } as u64;
	// 828837C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828837CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828837D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828837D0 size=76
    let mut pc: u32 = 0x828837D0;
    'dispatch: loop {
        match pc {
            0x828837D0 => {
    //   block [0x828837D0..0x8288381C)
	// 828837D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828837D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828837D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828837DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828837E0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828837E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828837E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828837EC: 388B4A18  addi r4, r11, 0x4a18
	ctx.r[4].s64 = ctx.r[11].s64 + 18968;
	// 828837F0: 48570219  bl 0x82df3a08
	ctx.lr = 0x828837F4;
	sub_82DF3A08(ctx, base);
	// 828837F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828837F8: 807F01CC  lwz r3, 0x1cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 828837FC: 4BF66B7D  bl 0x827ea378
	ctx.lr = 0x82883800;
	sub_827EA378(ctx, base);
	// 82883800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883804: 4856FC25  bl 0x82df3428
	ctx.lr = 0x82883808;
	sub_82DF3428(ctx, base);
	// 82883808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288380C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883820 size=76
    let mut pc: u32 = 0x82883820;
    'dispatch: loop {
        match pc {
            0x82883820 => {
    //   block [0x82883820..0x8288386C)
	// 82883820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8288382C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883830: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82883838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288383C: 388B4A28  addi r4, r11, 0x4a28
	ctx.r[4].s64 = ctx.r[11].s64 + 18984;
	// 82883840: 485701C9  bl 0x82df3a08
	ctx.lr = 0x82883844;
	sub_82DF3A08(ctx, base);
	// 82883844: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82883848: 807F01CC  lwz r3, 0x1cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 8288384C: 4BF66B2D  bl 0x827ea378
	ctx.lr = 0x82883850;
	sub_827EA378(ctx, base);
	// 82883850: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883854: 4856FBD5  bl 0x82df3428
	ctx.lr = 0x82883858;
	sub_82DF3428(ctx, base);
	// 82883858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8288385C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883870 size=188
    let mut pc: u32 = 0x82883870;
    'dispatch: loop {
        match pc {
            0x82883870 => {
    //   block [0x82883870..0x8288392C)
	// 82883870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8288387C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82883884: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82883888: 409A0024  bne cr6, 0x828838ac
	if !ctx.cr[6].eq {
	pc = 0x828838AC; continue 'dispatch;
	}
	// 8288388C: 807F0364  lwz r3, 0x364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(868 as u32) ) } as u64;
	// 82883890: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883894: 4800E94D  bl 0x828921e0
	ctx.lr = 0x82883898;
	sub_828921E0(ctx, base);
	// 82883898: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8288389C: 807F0364  lwz r3, 0x364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(868 as u32) ) } as u64;
	// 828838A0: 4800E9B1  bl 0x82892250
	ctx.lr = 0x828838A4;
	sub_82892250(ctx, base);
	// 828838A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828838A8: 4800005C  b 0x82883904
	pc = 0x82883904; continue 'dispatch;
	// 828838AC: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 828838B0: 409A0018  bne cr6, 0x828838c8
	if !ctx.cr[6].eq {
	pc = 0x828838C8; continue 'dispatch;
	}
	// 828838B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 828838B8: 807F0364  lwz r3, 0x364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(868 as u32) ) } as u64;
	// 828838BC: 4800E925  bl 0x828921e0
	ctx.lr = 0x828838C0;
	sub_828921E0(ctx, base);
	// 828838C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828838C4: 4BFFFFD8  b 0x8288389c
	pc = 0x8288389C; continue 'dispatch;
	// 828838C8: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 828838CC: 409A000C  bne cr6, 0x828838d8
	if !ctx.cr[6].eq {
	pc = 0x828838D8; continue 'dispatch;
	}
	// 828838D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828838D4: 4BFFFFE4  b 0x828838b8
	pc = 0x828838B8; continue 'dispatch;
	// 828838D8: 2B040004  cmplwi cr6, r4, 4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4 as u32, &mut ctx.xer);
	// 828838DC: 419AFFB0  beq cr6, 0x8288388c
	if ctx.cr[6].eq {
	pc = 0x8288388C; continue 'dispatch;
	}
	// 828838E0: 2B040005  cmplwi cr6, r4, 5
	ctx.cr[6].compare_u32(ctx.r[4].u32, 5 as u32, &mut ctx.xer);
	// 828838E4: 807F0364  lwz r3, 0x364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(868 as u32) ) } as u64;
	// 828838E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 828838EC: 419AFFA8  beq cr6, 0x82883894
	if ctx.cr[6].eq {
	pc = 0x82883894; continue 'dispatch;
	}
	// 828838F0: 4800E8F1  bl 0x828921e0
	ctx.lr = 0x828838F4;
	sub_828921E0(ctx, base);
	// 828838F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 828838F8: 807F0364  lwz r3, 0x364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(868 as u32) ) } as u64;
	// 828838FC: 4800E955  bl 0x82892250
	ctx.lr = 0x82883900;
	sub_82892250(ctx, base);
	// 82883900: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82883904: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288390C: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 82883910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883914: 4E800421  bctrl
	ctx.lr = 0x82883918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8288391C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883924: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82883930 size=176
    let mut pc: u32 = 0x82883930;
    'dispatch: loop {
        match pc {
            0x82883930 => {
    //   block [0x82883930..0x828839E0)
	// 82883930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883934: 48924839  bl 0x831a816c
	ctx.lr = 0x82883938;
	sub_831A8130(ctx, base);
	// 82883938: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288393C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82883940: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82883944: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 82883948: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 8288394C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82883950: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82883954: C00BBA80  lfs f0, -0x4580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82883958: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8288395C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82883960: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82883964: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82883968: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8288396C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82883970: C1A90008  lfs f13, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82883974: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883978: C189000C  lfs f12, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8288397C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82883980: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82883984: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82883988: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8288398C: 809F03F0  lwz r4, 0x3f0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1008 as u32) ) } as u64;
	// 82883990: D181005C  stfs f12, 0x5c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828839E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828839E0 size=176
    let mut pc: u32 = 0x828839E0;
    'dispatch: loop {
        match pc {
            0x828839E0 => {
    //   block [0x828839E0..0x82883A90)
	// 828839E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828839E4: 48924789  bl 0x831a816c
	ctx.lr = 0x828839E8;
	sub_831A8130(ctx, base);
	// 828839E8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828839EC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 828839F0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 828839F4: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 828839F8: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 828839FC: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82883A00: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82883A04: C00BBA80  lfs f0, -0x4580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82883A08: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82883A0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82883A10: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82883A14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82883A18: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82883A1C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82883A20: C1A90008  lfs f13, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82883A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883A28: C189000C  lfs f12, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82883A2C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82883A30: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82883A34: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82883A38: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82883A3C: 809F03F4  lwz r4, 0x3f4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1012 as u32) ) } as u64;
	// 82883A40: D181005C  stfs f12, 0x5c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883A90 size=20
    let mut pc: u32 = 0x82883A90;
    'dispatch: loop {
        match pc {
            0x82883A90 => {
    //   block [0x82883A90..0x82883AA4)
	// 82883A90: 80630260  lwz r3, 0x260(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(608 as u32) ) } as u64;
	// 82883A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82883A98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82883A9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883AA0: 4808F4D8  b 0x82912f78
	sub_82912F78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883AA8 size=12
    let mut pc: u32 = 0x82883AA8;
    'dispatch: loop {
        match pc {
            0x82883AA8 => {
    //   block [0x82883AA8..0x82883AB4)
	// 82883AA8: 80630364  lwz r3, 0x364(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(868 as u32) ) } as u64;
	// 82883AAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82883AB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883AB4 size=16
    let mut pc: u32 = 0x82883AB4;
    'dispatch: loop {
        match pc {
            0x82883AB4 => {
    //   block [0x82883AB4..0x82883AC4)
	// 82883AB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883AB8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82883ABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883AC0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82883AC4 size=4
    let mut pc: u32 = 0x82883AC4;
    'dispatch: loop {
        match pc {
            0x82883AC4 => {
    //   block [0x82883AC4..0x82883AC8)
	// 82883AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883AC8 size=196
    let mut pc: u32 = 0x82883AC8;
    'dispatch: loop {
        match pc {
            0x82883AC8 => {
    //   block [0x82883AC8..0x82883B8C)
	// 82883AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883ADC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883AE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883AE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883AEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883AF0: 4BA3CE49  bl 0x822c0938
	ctx.lr = 0x82883AF4;
	sub_822C0938(ctx, base);
	// 82883AF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883AF8: 41820028  beq 0x82883b20
	if ctx.cr[0].eq {
	pc = 0x82883B20; continue 'dispatch;
	}
	// 82883AFC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883B00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883B04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883B08: 392B4940  addi r9, r11, 0x4940
	ctx.r[9].s64 = ctx.r[11].s64 + 18752;
	// 82883B0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883B10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883B14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883B18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883B1C: 48000008  b 0x82883b24
	pc = 0x82883B24; continue 'dispatch;
	// 82883B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883B24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883B2C: 409A0044  bne cr6, 0x82883b70
	if !ctx.cr[6].eq {
	pc = 0x82883B70; continue 'dispatch;
	}
	// 82883B30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883B34: 419A001C  beq cr6, 0x82883b50
	if ctx.cr[6].eq {
	pc = 0x82883B50; continue 'dispatch;
	}
	// 82883B38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883B3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883B44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883B4C: 4E800421  bctrl
	ctx.lr = 0x82883B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883B50: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883B54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82883B58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883B5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82883B60: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82883B64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82883B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82883B6C: 4BA3C495  bl 0x822c0000
	ctx.lr = 0x82883B70;
	sub_822C0000(ctx, base);
	// 82883B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883B80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883B90 size=196
    let mut pc: u32 = 0x82883B90;
    'dispatch: loop {
        match pc {
            0x82883B90 => {
    //   block [0x82883B90..0x82883C54)
	// 82883B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883BA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883BA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883BAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883BB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883BB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883BB8: 4BA3CD81  bl 0x822c0938
	ctx.lr = 0x82883BBC;
	sub_822C0938(ctx, base);
	// 82883BBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883BC0: 41820028  beq 0x82883be8
	if ctx.cr[0].eq {
	pc = 0x82883BE8; continue 'dispatch;
	}
	// 82883BC4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883BC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883BD0: 392B4954  addi r9, r11, 0x4954
	ctx.r[9].s64 = ctx.r[11].s64 + 18772;
	// 82883BD4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883BD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883BDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883BE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883BE4: 48000008  b 0x82883bec
	pc = 0x82883BEC; continue 'dispatch;
	// 82883BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883BEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883BF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883BF4: 409A0044  bne cr6, 0x82883c38
	if !ctx.cr[6].eq {
	pc = 0x82883C38; continue 'dispatch;
	}
	// 82883BF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883BFC: 419A001C  beq cr6, 0x82883c18
	if ctx.cr[6].eq {
	pc = 0x82883C18; continue 'dispatch;
	}
	// 82883C00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883C04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883C0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883C14: 4E800421  bctrl
	ctx.lr = 0x82883C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883C18: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883C1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82883C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883C24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82883C28: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82883C2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82883C30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82883C34: 4BA3C3CD  bl 0x822c0000
	ctx.lr = 0x82883C38;
	sub_822C0000(ctx, base);
	// 82883C38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883C48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883C58 size=196
    let mut pc: u32 = 0x82883C58;
    'dispatch: loop {
        match pc {
            0x82883C58 => {
    //   block [0x82883C58..0x82883D1C)
	// 82883C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883C60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883C64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883C68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883C6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883C74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883C78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883C7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883C80: 4BA3CCB9  bl 0x822c0938
	ctx.lr = 0x82883C84;
	sub_822C0938(ctx, base);
	// 82883C84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883C88: 41820028  beq 0x82883cb0
	if ctx.cr[0].eq {
	pc = 0x82883CB0; continue 'dispatch;
	}
	// 82883C8C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883C90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883C94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883C98: 392B4968  addi r9, r11, 0x4968
	ctx.r[9].s64 = ctx.r[11].s64 + 18792;
	// 82883C9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883CA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883CA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883CA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883CAC: 48000008  b 0x82883cb4
	pc = 0x82883CB4; continue 'dispatch;
	// 82883CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883CB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883CB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883CBC: 409A0044  bne cr6, 0x82883d00
	if !ctx.cr[6].eq {
	pc = 0x82883D00; continue 'dispatch;
	}
	// 82883CC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883CC4: 419A001C  beq cr6, 0x82883ce0
	if ctx.cr[6].eq {
	pc = 0x82883CE0; continue 'dispatch;
	}
	// 82883CC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883CCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883CD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883CD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883CDC: 4E800421  bctrl
	ctx.lr = 0x82883CE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883CE0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883CE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82883CE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883CEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82883CF0: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82883CF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82883CF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82883CFC: 4BA3C305  bl 0x822c0000
	ctx.lr = 0x82883D00;
	sub_822C0000(ctx, base);
	// 82883D00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883D04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883D10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883D14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883D20 size=196
    let mut pc: u32 = 0x82883D20;
    'dispatch: loop {
        match pc {
            0x82883D20 => {
    //   block [0x82883D20..0x82883DE4)
	// 82883D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883D28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883D2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883D34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883D3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883D40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883D44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883D48: 4BA3CBF1  bl 0x822c0938
	ctx.lr = 0x82883D4C;
	sub_822C0938(ctx, base);
	// 82883D4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883D50: 41820028  beq 0x82883d78
	if ctx.cr[0].eq {
	pc = 0x82883D78; continue 'dispatch;
	}
	// 82883D54: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883D58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883D5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883D60: 392B497C  addi r9, r11, 0x497c
	ctx.r[9].s64 = ctx.r[11].s64 + 18812;
	// 82883D64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883D68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883D6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883D70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883D74: 48000008  b 0x82883d7c
	pc = 0x82883D7C; continue 'dispatch;
	// 82883D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883D7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883D80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883D84: 409A0044  bne cr6, 0x82883dc8
	if !ctx.cr[6].eq {
	pc = 0x82883DC8; continue 'dispatch;
	}
	// 82883D88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883D8C: 419A001C  beq cr6, 0x82883da8
	if ctx.cr[6].eq {
	pc = 0x82883DA8; continue 'dispatch;
	}
	// 82883D90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883D94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883D98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883D9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883DA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883DA4: 4E800421  bctrl
	ctx.lr = 0x82883DA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883DA8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883DAC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82883DB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883DB4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82883DB8: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82883DBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82883DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82883DC4: 4BA3C23D  bl 0x822c0000
	ctx.lr = 0x82883DC8;
	sub_822C0000(ctx, base);
	// 82883DC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883DCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883DD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883DDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883DE8 size=196
    let mut pc: u32 = 0x82883DE8;
    'dispatch: loop {
        match pc {
            0x82883DE8 => {
    //   block [0x82883DE8..0x82883EAC)
	// 82883DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883DF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883DFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883E04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883E08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883E0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883E10: 4BA3CB29  bl 0x822c0938
	ctx.lr = 0x82883E14;
	sub_822C0938(ctx, base);
	// 82883E14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883E18: 41820028  beq 0x82883e40
	if ctx.cr[0].eq {
	pc = 0x82883E40; continue 'dispatch;
	}
	// 82883E1C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883E20: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883E24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883E28: 392B4990  addi r9, r11, 0x4990
	ctx.r[9].s64 = ctx.r[11].s64 + 18832;
	// 82883E2C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883E30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883E34: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883E38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883E3C: 48000008  b 0x82883e44
	pc = 0x82883E44; continue 'dispatch;
	// 82883E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883E44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883E48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883E4C: 409A0044  bne cr6, 0x82883e90
	if !ctx.cr[6].eq {
	pc = 0x82883E90; continue 'dispatch;
	}
	// 82883E50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883E54: 419A001C  beq cr6, 0x82883e70
	if ctx.cr[6].eq {
	pc = 0x82883E70; continue 'dispatch;
	}
	// 82883E58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883E5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883E64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883E68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883E6C: 4E800421  bctrl
	ctx.lr = 0x82883E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883E70: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883E74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82883E78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883E7C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82883E80: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82883E84: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82883E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82883E8C: 4BA3C175  bl 0x822c0000
	ctx.lr = 0x82883E90;
	sub_822C0000(ctx, base);
	// 82883E90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883EA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883EA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883EB0 size=196
    let mut pc: u32 = 0x82883EB0;
    'dispatch: loop {
        match pc {
            0x82883EB0 => {
    //   block [0x82883EB0..0x82883F74)
	// 82883EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883EC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883ECC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883ED0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883ED4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883ED8: 4BA3CA61  bl 0x822c0938
	ctx.lr = 0x82883EDC;
	sub_822C0938(ctx, base);
	// 82883EDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883EE0: 41820028  beq 0x82883f08
	if ctx.cr[0].eq {
	pc = 0x82883F08; continue 'dispatch;
	}
	// 82883EE4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883EE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883EEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883EF0: 392B49A4  addi r9, r11, 0x49a4
	ctx.r[9].s64 = ctx.r[11].s64 + 18852;
	// 82883EF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883EF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883EFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883F00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883F04: 48000008  b 0x82883f0c
	pc = 0x82883F0C; continue 'dispatch;
	// 82883F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883F0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883F10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883F14: 409A0044  bne cr6, 0x82883f58
	if !ctx.cr[6].eq {
	pc = 0x82883F58; continue 'dispatch;
	}
	// 82883F18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883F1C: 419A001C  beq cr6, 0x82883f38
	if ctx.cr[6].eq {
	pc = 0x82883F38; continue 'dispatch;
	}
	// 82883F20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883F24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883F2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883F34: 4E800421  bctrl
	ctx.lr = 0x82883F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82883F38: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82883F3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82883F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82883F44: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82883F48: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82883F4C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82883F50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82883F54: 4BA3C0AD  bl 0x822c0000
	ctx.lr = 0x82883F58;
	sub_822C0000(ctx, base);
	// 82883F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82883F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82883F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82883F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82883F68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82883F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82883F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82883F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82883F78 size=196
    let mut pc: u32 = 0x82883F78;
    'dispatch: loop {
        match pc {
            0x82883F78 => {
    //   block [0x82883F78..0x8288403C)
	// 82883F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82883F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82883F80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82883F84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82883F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82883F8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82883F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883F94: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82883F98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82883F9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883FA0: 4BA3C999  bl 0x822c0938
	ctx.lr = 0x82883FA4;
	sub_822C0938(ctx, base);
	// 82883FA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82883FA8: 41820028  beq 0x82883fd0
	if ctx.cr[0].eq {
	pc = 0x82883FD0; continue 'dispatch;
	}
	// 82883FAC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82883FB0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82883FB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82883FB8: 392B49B8  addi r9, r11, 0x49b8
	ctx.r[9].s64 = ctx.r[11].s64 + 18872;
	// 82883FBC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82883FC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82883FC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82883FC8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82883FCC: 48000008  b 0x82883fd4
	pc = 0x82883FD4; continue 'dispatch;
	// 82883FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82883FD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82883FD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82883FDC: 409A0044  bne cr6, 0x82884020
	if !ctx.cr[6].eq {
	pc = 0x82884020; continue 'dispatch;
	}
	// 82883FE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82883FE4: 419A001C  beq cr6, 0x82884000
	if ctx.cr[6].eq {
	pc = 0x82884000; continue 'dispatch;
	}
	// 82883FE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883FEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82883FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82883FF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82883FF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82883FFC: 4E800421  bctrl
	ctx.lr = 0x82884000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884000: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82884004: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82884008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288400C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82884010: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82884014: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82884018: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288401C: 4BA3BFE5  bl 0x822c0000
	ctx.lr = 0x82884020;
	sub_822C0000(ctx, base);
	// 82884020: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82884024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82884028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288402C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82884030: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82884034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884040 size=196
    let mut pc: u32 = 0x82884040;
    'dispatch: loop {
        match pc {
            0x82884040 => {
    //   block [0x82884040..0x82884104)
	// 82884040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8288404C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82884050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884054: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82884058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8288405C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82884060: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82884064: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82884068: 4BA3C8D1  bl 0x822c0938
	ctx.lr = 0x8288406C;
	sub_822C0938(ctx, base);
	// 8288406C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82884070: 41820028  beq 0x82884098
	if ctx.cr[0].eq {
	pc = 0x82884098; continue 'dispatch;
	}
	// 82884074: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884078: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8288407C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82884080: 392B49CC  addi r9, r11, 0x49cc
	ctx.r[9].s64 = ctx.r[11].s64 + 18892;
	// 82884084: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82884088: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8288408C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82884090: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82884094: 48000008  b 0x8288409c
	pc = 0x8288409C; continue 'dispatch;
	// 82884098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8288409C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828840A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828840A4: 409A0044  bne cr6, 0x828840e8
	if !ctx.cr[6].eq {
	pc = 0x828840E8; continue 'dispatch;
	}
	// 828840A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 828840AC: 419A001C  beq cr6, 0x828840c8
	if ctx.cr[6].eq {
	pc = 0x828840C8; continue 'dispatch;
	}
	// 828840B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828840B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 828840B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828840BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828840C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828840C4: 4E800421  bctrl
	ctx.lr = 0x828840C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828840C8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828840CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 828840D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828840D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 828840D8: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 828840DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828840E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828840E4: 4BA3BF1D  bl 0x822c0000
	ctx.lr = 0x828840E8;
	sub_822C0000(ctx, base);
	// 828840E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828840EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828840F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828840F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828840F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828840FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884108 size=196
    let mut pc: u32 = 0x82884108;
    'dispatch: loop {
        match pc {
            0x82884108 => {
    //   block [0x82884108..0x828841CC)
	// 82884108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288410C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82884114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82884118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288411C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82884120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82884124: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82884128: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8288412C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82884130: 4BA3C809  bl 0x822c0938
	ctx.lr = 0x82884134;
	sub_822C0938(ctx, base);
	// 82884134: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82884138: 41820028  beq 0x82884160
	if ctx.cr[0].eq {
	pc = 0x82884160; continue 'dispatch;
	}
	// 8288413C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884140: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82884144: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82884148: 392B49E0  addi r9, r11, 0x49e0
	ctx.r[9].s64 = ctx.r[11].s64 + 18912;
	// 8288414C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82884150: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82884154: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82884158: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8288415C: 48000008  b 0x82884164
	pc = 0x82884164; continue 'dispatch;
	// 82884160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82884164: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82884168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8288416C: 409A0044  bne cr6, 0x828841b0
	if !ctx.cr[6].eq {
	pc = 0x828841B0; continue 'dispatch;
	}
	// 82884170: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82884174: 419A001C  beq cr6, 0x82884190
	if ctx.cr[6].eq {
	pc = 0x82884190; continue 'dispatch;
	}
	// 82884178: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288417C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82884180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884184: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884188: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288418C: 4E800421  bctrl
	ctx.lr = 0x82884190;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884190: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82884194: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82884198: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288419C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 828841A0: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 828841A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828841A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828841AC: 4BA3BE55  bl 0x822c0000
	ctx.lr = 0x828841B0;
	sub_822C0000(ctx, base);
	// 828841B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828841B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828841B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828841BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828841C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828841C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828841C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828841D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828841D0 size=196
    let mut pc: u32 = 0x828841D0;
    'dispatch: loop {
        match pc {
            0x828841D0 => {
    //   block [0x828841D0..0x82884294)
	// 828841D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828841D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828841D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828841DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828841E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828841E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828841E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828841EC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828841F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828841F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828841F8: 4BA3C741  bl 0x822c0938
	ctx.lr = 0x828841FC;
	sub_822C0938(ctx, base);
	// 828841FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82884200: 41820028  beq 0x82884228
	if ctx.cr[0].eq {
	pc = 0x82884228; continue 'dispatch;
	}
	// 82884204: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884208: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8288420C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82884210: 392B49F4  addi r9, r11, 0x49f4
	ctx.r[9].s64 = ctx.r[11].s64 + 18932;
	// 82884214: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82884218: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8288421C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82884220: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82884224: 48000008  b 0x8288422c
	pc = 0x8288422C; continue 'dispatch;
	// 82884228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8288422C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82884230: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884234: 409A0044  bne cr6, 0x82884278
	if !ctx.cr[6].eq {
	pc = 0x82884278; continue 'dispatch;
	}
	// 82884238: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8288423C: 419A001C  beq cr6, 0x82884258
	if ctx.cr[6].eq {
	pc = 0x82884258; continue 'dispatch;
	}
	// 82884240: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884244: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82884248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288424C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884250: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82884254: 4E800421  bctrl
	ctx.lr = 0x82884258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884258: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 8288425C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82884260: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884264: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82884268: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 8288426C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82884270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82884274: 4BA3BD8D  bl 0x822c0000
	ctx.lr = 0x82884278;
	sub_822C0000(ctx, base);
	// 82884278: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8288427C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82884280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82884284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82884288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8288428C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884298 size=196
    let mut pc: u32 = 0x82884298;
    'dispatch: loop {
        match pc {
            0x82884298 => {
    //   block [0x82884298..0x8288435C)
	// 82884298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828842A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828842A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828842A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828842AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828842B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828842B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 828842B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 828842BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828842C0: 4BA3C679  bl 0x822c0938
	ctx.lr = 0x828842C4;
	sub_822C0938(ctx, base);
	// 828842C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 828842C8: 41820028  beq 0x828842f0
	if ctx.cr[0].eq {
	pc = 0x828842F0; continue 'dispatch;
	}
	// 828842CC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828842D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 828842D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 828842D8: 392B4A08  addi r9, r11, 0x4a08
	ctx.r[9].s64 = ctx.r[11].s64 + 18952;
	// 828842DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 828842E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 828842E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 828842E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 828842EC: 48000008  b 0x828842f4
	pc = 0x828842F4; continue 'dispatch;
	// 828842F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828842F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828842F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828842FC: 409A0044  bne cr6, 0x82884340
	if !ctx.cr[6].eq {
	pc = 0x82884340; continue 'dispatch;
	}
	// 82884300: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82884304: 419A001C  beq cr6, 0x82884320
	if ctx.cr[6].eq {
	pc = 0x82884320; continue 'dispatch;
	}
	// 82884308: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288430C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82884310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884314: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288431C: 4E800421  bctrl
	ctx.lr = 0x82884320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884320: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82884324: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82884328: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288432C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82884330: 816BA2A8  lwz r11, -0x5d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23896 as u32) ) } as u64;
	// 82884334: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82884338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288433C: 4BA3BCC5  bl 0x822c0000
	ctx.lr = 0x82884340;
	sub_822C0000(ctx, base);
	// 82884340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82884344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82884348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288434C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82884350: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82884354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884360 size=72
    let mut pc: u32 = 0x82884360;
    'dispatch: loop {
        match pc {
            0x82884360 => {
    //   block [0x82884360..0x828843A8)
	// 82884360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288436C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82884370: 419A001C  beq cr6, 0x8288438c
	if ctx.cr[6].eq {
	pc = 0x8288438C; continue 'dispatch;
	}
	// 82884374: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82884378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8288437C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82884380: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82884384: 4BFFF065  bl 0x828833e8
	ctx.lr = 0x82884388;
	sub_828833E8(ctx, base);
	// 82884388: 48000010  b 0x82884398
	pc = 0x82884398; continue 'dispatch;
	// 8288438C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82884390: 396BA6D0  addi r11, r11, -0x5930
	ctx.r[11].s64 = ctx.r[11].s64 + -22832;
	// 82884394: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82884398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8288439C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828843A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828843A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828843A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828843A8 size=268
    let mut pc: u32 = 0x828843A8;
    'dispatch: loop {
        match pc {
            0x828843A8 => {
    //   block [0x828843A8..0x828844B4)
	// 828843A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828843AC: 48923DBD  bl 0x831a8168
	ctx.lr = 0x828843B0;
	sub_831A8130(ctx, base);
	// 828843B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828843B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828843B8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828843BC: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 828843C0: 396B4ABC  addi r11, r11, 0x4abc
	ctx.r[11].s64 = ctx.r[11].s64 + 19132;
	// 828843C4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 828843C8: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 828843CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828843D0: 807F03D0  lwz r3, 0x3d0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(976 as u32) ) } as u64;
	// 828843D4: 394A4AA4  addi r10, r10, 0x4aa4
	ctx.r[10].s64 = ctx.r[10].s64 + 19108;
	// 828843D8: 39294A88  addi r9, r9, 0x4a88
	ctx.r[9].s64 = ctx.r[9].s64 + 19080;
	// 828843DC: 39684A3C  addi r11, r8, 0x4a3c
	ctx.r[11].s64 = ctx.r[8].s64 + 19004;
	// 828843E0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 828843E4: 3B9F0360  addi r28, r31, 0x360
	ctx.r[28].s64 = ctx.r[31].s64 + 864;
	// 828843E8: 913F0208  stw r9, 0x208(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(520 as u32), ctx.r[9].u32 ) };
	// 828843EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828843F0: 917F0360  stw r11, 0x360(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(864 as u32), ctx.r[11].u32 ) };
	// 828843F4: 419A0008  beq cr6, 0x828843fc
	if ctx.cr[6].eq {
	pc = 0x828843FC; continue 'dispatch;
	}
	// 828843F8: 4BA3BE71  bl 0x822c0268
	ctx.lr = 0x828843FC;
	sub_822C0268(ctx, base);
	// 828843FC: 807F0400  lwz r3, 0x400(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1024 as u32) ) } as u64;
	// 82884400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884404: 419A0018  beq cr6, 0x8288441c
	if ctx.cr[6].eq {
	pc = 0x8288441C; continue 'dispatch;
	}
	// 82884408: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288440C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82884410: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82884418: 4E800421  bctrl
	ctx.lr = 0x8288441C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8288441C: 387F03DC  addi r3, r31, 0x3dc
	ctx.r[3].s64 = ctx.r[31].s64 + 988;
	// 82884420: 4856F009  bl 0x82df3428
	ctx.lr = 0x82884424;
	sub_82DF3428(ctx, base);
	// 82884424: 807F03C8  lwz r3, 0x3c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(968 as u32) ) } as u64;
	// 82884428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288442C: 419A0008  beq cr6, 0x82884434
	if ctx.cr[6].eq {
	pc = 0x82884434; continue 'dispatch;
	}
	// 82884430: 4BA3C461  bl 0x822c0890
	ctx.lr = 0x82884434;
	sub_822C0890(ctx, base);
	// 82884434: 807F03C0  lwz r3, 0x3c0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(960 as u32) ) } as u64;
	// 82884438: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288443C: 419A0008  beq cr6, 0x82884444
	if ctx.cr[6].eq {
	pc = 0x82884444; continue 'dispatch;
	}
	// 82884440: 4BA3C451  bl 0x822c0890
	ctx.lr = 0x82884444;
	sub_822C0890(ctx, base);
	// 82884444: 807F03B8  lwz r3, 0x3b8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 82884448: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288444C: 419A0008  beq cr6, 0x82884454
	if ctx.cr[6].eq {
	pc = 0x82884454; continue 'dispatch;
	}
	// 82884450: 4BA3C441  bl 0x822c0890
	ctx.lr = 0x82884454;
	sub_822C0890(ctx, base);
	// 82884454: 807F03B0  lwz r3, 0x3b0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(944 as u32) ) } as u64;
	// 82884458: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288445C: 419A0008  beq cr6, 0x82884464
	if ctx.cr[6].eq {
	pc = 0x82884464; continue 'dispatch;
	}
	// 82884460: 4BA3C431  bl 0x822c0890
	ctx.lr = 0x82884464;
	sub_822C0890(ctx, base);
	// 82884464: 397F03AC  addi r11, r31, 0x3ac
	ctx.r[11].s64 = ctx.r[31].s64 + 940;
	// 82884468: 3BA00007  li r29, 7
	ctx.r[29].s64 = 7;
	// 8288446C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82884470: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82884474: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884478: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288447C: 419A0008  beq cr6, 0x82884484
	if ctx.cr[6].eq {
	pc = 0x82884484; continue 'dispatch;
	}
	// 82884480: 4BA3C411  bl 0x822c0890
	ctx.lr = 0x82884484;
	sub_822C0890(ctx, base);
	// 82884484: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82884488: 4080FFE8  bge 0x82884470
	if !ctx.cr[0].lt {
	pc = 0x82884470; continue 'dispatch;
	}
	// 8288448C: 807F0368  lwz r3, 0x368(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(872 as u32) ) } as u64;
	// 82884490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884494: 419A0008  beq cr6, 0x8288449c
	if ctx.cr[6].eq {
	pc = 0x8288449C; continue 'dispatch;
	}
	// 82884498: 4BA3C3F9  bl 0x822c0890
	ctx.lr = 0x8288449C;
	sub_822C0890(ctx, base);
	// 8288449C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 828844A0: 4BF231B9  bl 0x827a7658
	ctx.lr = 0x828844A4;
	sub_827A7658(ctx, base);
	// 828844A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828844A8: 4808C7E9  bl 0x82910c90
	ctx.lr = 0x828844AC;
	sub_82910C90(ctx, base);
	// 828844AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 828844B0: 48923D08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828844B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828844B8 size=8
    let mut pc: u32 = 0x828844B8;
    'dispatch: loop {
        match pc {
            0x828844B8 => {
    //   block [0x828844B8..0x828844C0)
	// 828844B8: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 828844BC: 480022CC  b 0x82886788
	sub_82886788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828844C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828844C0 size=8
    let mut pc: u32 = 0x828844C0;
    'dispatch: loop {
        match pc {
            0x828844C0 => {
    //   block [0x828844C0..0x828844C8)
	// 828844C0: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 828844C4: 480022C4  b 0x82886788
	sub_82886788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828844C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828844C8 size=8
    let mut pc: u32 = 0x828844C8;
    'dispatch: loop {
        match pc {
            0x828844C8 => {
    //   block [0x828844C8..0x828844D0)
	// 828844C8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 828844CC: 480022BC  b 0x82886788
	sub_82886788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828844D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828844D0 size=100
    let mut pc: u32 = 0x828844D0;
    'dispatch: loop {
        match pc {
            0x828844D0 => {
    //   block [0x828844D0..0x82884534)
	// 828844D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828844D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828844D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828844DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828844E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828844E4: 2F050005  cmpwi cr6, r5, 5
	ctx.cr[6].compare_i32(ctx.r[5].s32, 5, &mut ctx.xer);
	// 828844E8: 419A0028  beq cr6, 0x82884510
	if ctx.cr[6].eq {
	pc = 0x82884510; continue 'dispatch;
	}
	// 828844EC: 2F050009  cmpwi cr6, r5, 9
	ctx.cr[6].compare_i32(ctx.r[5].s32, 9, &mut ctx.xer);
	// 828844F0: 419A0014  beq cr6, 0x82884504
	if ctx.cr[6].eq {
	pc = 0x82884504; continue 'dispatch;
	}
	// 828844F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 828844F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828844FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82884500: 48000020  b 0x82884520
	pc = 0x82884520; continue 'dispatch;
	// 82884504: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884508: 38AB4CA8  addi r5, r11, 0x4ca8
	ctx.r[5].s64 = ctx.r[11].s64 + 19624;
	// 8288450C: 4800000C  b 0x82884518
	pc = 0x82884518; continue 'dispatch;
	// 82884510: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884514: 38AB4C98  addi r5, r11, 0x4c98
	ctx.r[5].s64 = ctx.r[11].s64 + 19608;
	// 82884518: 4BC8DC61  bl 0x82512178
	ctx.lr = 0x8288451C;
	sub_82512178(ctx, base);
	// 8288451C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82884524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82884528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8288452C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884538 size=92
    let mut pc: u32 = 0x82884538;
    'dispatch: loop {
        match pc {
            0x82884538 => {
    //   block [0x82884538..0x82884594)
	// 82884538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288453C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82884544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884548: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8288454C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82884550: 419A001C  beq cr6, 0x8288456c
	if ctx.cr[6].eq {
	pc = 0x8288456C; continue 'dispatch;
	}
	// 82884554: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82884558: 419A0014  beq cr6, 0x8288456c
	if ctx.cr[6].eq {
	pc = 0x8288456C; continue 'dispatch;
	}
	// 8288455C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82884560: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82884564: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82884568: 48000014  b 0x8288457c
	pc = 0x8288457C; continue 'dispatch;
	// 8288456C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884574: 38AB4CBC  addi r5, r11, 0x4cbc
	ctx.r[5].s64 = ctx.r[11].s64 + 19644;
	// 82884578: 4BC8D9C1  bl 0x82511f38
	ctx.lr = 0x8288457C;
	sub_82511F38(ctx, base);
	// 8288457C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82884584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82884588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8288458C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82884598 size=788
    let mut pc: u32 = 0x82884598;
    'dispatch: loop {
        match pc {
            0x82884598 => {
    //   block [0x82884598..0x828848AC)
	// 82884598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828845A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828845A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828845A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828845AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828845B0: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 828845B4: 418202AC  beq 0x82884860
	if ctx.cr[0].eq {
	pc = 0x82884860; continue 'dispatch;
	}
	// 828845B8: 48084831  bl 0x82908de8
	ctx.lr = 0x828845BC;
	sub_82908DE8(ctx, base);
	// 828845BC: 2B030007  cmplwi cr6, r3, 7
	ctx.cr[6].compare_u32(ctx.r[3].u32, 7 as u32, &mut ctx.xer);
	// 828845C0: 419902D4  bgt cr6, 0x82884894
	if ctx.cr[6].gt {
	pc = 0x82884894; continue 'dispatch;
	}
	// 828845C4: 3D808208  lis r12, -0x7df8
	ctx.r[12].s64 = -2113404928;
	// 828845C8: 398C48F8  addi r12, r12, 0x48f8
	ctx.r[12].s64 = ctx.r[12].s64 + 18680;
	// 828845CC: 7C0C18AE  lbzx r0, r12, r3
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 828845D0: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 828845D4: 3D808288  lis r12, -0x7d78
	ctx.r[12].s64 = -2105016320;
	// 828845D8: 398C45EC  addi r12, r12, 0x45ec
	ctx.r[12].s64 = ctx.r[12].s64 + 17900;
	// 828845DC: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 828845E0: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 828845E4: 60000000  nop
	// 828845E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 828845EC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828845F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 828845F4: 388B4D18  addi r4, r11, 0x4d18
	ctx.r[4].s64 = ctx.r[11].s64 + 19736;
	// 828845F8: 4856F411  bl 0x82df3a08
	ctx.lr = 0x828845FC;
	sub_82DF3A08(ctx, base);
	// 828845FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82884600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884604: 388B3568  addi r4, r11, 0x3568
	ctx.r[4].s64 = ctx.r[11].s64 + 13672;
	// 82884608: 4856F401  bl 0x82df3a08
	ctx.lr = 0x8288460C;
	sub_82DF3A08(ctx, base);
	// 8288460C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82884610: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82884614: 4BF67FB5  bl 0x827ec5c8
	ctx.lr = 0x82884618;
	sub_827EC5C8(ctx, base);
	// 82884618: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8288461C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82884620: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82884624: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 82884628: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8288462C: 48590A05  bl 0x82e15030
	ctx.lr = 0x82884630;
	sub_82E15030(ctx, base);
	// 82884630: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82884634: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82884638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8288463C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82884640: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82884644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82884648: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8288464C: 4BF67BBD  bl 0x827ec208
	ctx.lr = 0x82884650;
	sub_827EC208(ctx, base);
	// 82884650: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82884654: 907F03F8  stw r3, 0x3f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1016 as u32), ctx.r[3].u32 ) };
	// 82884658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8288465C: 419A000C  beq cr6, 0x82884668
	if ctx.cr[6].eq {
	pc = 0x82884668; continue 'dispatch;
	}
	// 82884660: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82884664: 4BA3C22D  bl 0x822c0890
	ctx.lr = 0x82884668;
	sub_822C0890(ctx, base);
	// 82884668: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8288466C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884670: 419A0008  beq cr6, 0x82884678
	if ctx.cr[6].eq {
	pc = 0x82884678; continue 'dispatch;
	}
	// 82884674: 4BA3C21D  bl 0x822c0890
	ctx.lr = 0x82884678;
	sub_822C0890(ctx, base);
	// 82884678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8288467C: 4856EDAD  bl 0x82df3428
	ctx.lr = 0x82884680;
	sub_82DF3428(ctx, base);
	// 82884680: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82884684: 480001D4  b 0x82884858
	pc = 0x82884858; continue 'dispatch;
	// 82884688: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288468C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82884690: 388B4D00  addi r4, r11, 0x4d00
	ctx.r[4].s64 = ctx.r[11].s64 + 19712;
	// 82884694: 4856F375  bl 0x82df3a08
	ctx.lr = 0x82884698;
	sub_82DF3A08(ctx, base);
	// 82884698: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8288469C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 828846A0: 388B3568  addi r4, r11, 0x3568
	ctx.r[4].s64 = ctx.r[11].s64 + 13672;
	// 828846A4: 4856F365  bl 0x82df3a08
	ctx.lr = 0x828846A8;
	sub_82DF3A08(ctx, base);
	// 828846A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828846AC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 828846B0: 4BF67F19  bl 0x827ec5c8
	ctx.lr = 0x828846B4;
	sub_827EC5C8(ctx, base);
	// 828846B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 828846B8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 828846BC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 828846C0: 3BC1005C  addi r30, r1, 0x5c
	ctx.r[30].s64 = ctx.r[1].s64 + 92;
	// 828846C4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 828846C8: 48590969  bl 0x82e15030
	ctx.lr = 0x828846CC;
	sub_82E15030(ctx, base);
	// 828846CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 828846D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828846D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828846D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 828846DC: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 828846E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 828846E4: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 828846E8: 4BF67B21  bl 0x827ec208
	ctx.lr = 0x828846EC;
	sub_827EC208(ctx, base);
	// 828846EC: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 828846F0: 907F03F8  stw r3, 0x3f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1016 as u32), ctx.r[3].u32 ) };
	// 828846F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828846F8: 419A000C  beq cr6, 0x82884704
	if ctx.cr[6].eq {
	pc = 0x82884704; continue 'dispatch;
	}
	// 828846FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82884700: 4BA3C191  bl 0x822c0890
	ctx.lr = 0x82884704;
	sub_822C0890(ctx, base);
	// 82884704: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 82884708: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288470C: 419A0008  beq cr6, 0x82884714
	if ctx.cr[6].eq {
	pc = 0x82884714; continue 'dispatch;
	}
	// 82884710: 4BA3C181  bl 0x822c0890
	ctx.lr = 0x82884714;
	sub_822C0890(ctx, base);
	// 82884714: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82884718: 4856ED11  bl 0x82df3428
	ctx.lr = 0x8288471C;
	sub_82DF3428(ctx, base);
	// 8288471C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82884720: 48000138  b 0x82884858
	pc = 0x82884858; continue 'dispatch;
	// 82884724: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884728: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8288472C: 388B4CE8  addi r4, r11, 0x4ce8
	ctx.r[4].s64 = ctx.r[11].s64 + 19688;
	// 82884730: 4856F2D9  bl 0x82df3a08
	ctx.lr = 0x82884734;
	sub_82DF3A08(ctx, base);
	// 82884734: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82884738: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8288473C: 388B3568  addi r4, r11, 0x3568
	ctx.r[4].s64 = ctx.r[11].s64 + 13672;
	// 82884740: 4856F2C9  bl 0x82df3a08
	ctx.lr = 0x82884744;
	sub_82DF3A08(ctx, base);
	// 82884744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82884748: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8288474C: 4BF67E7D  bl 0x827ec5c8
	ctx.lr = 0x82884750;
	sub_827EC5C8(ctx, base);
	// 82884750: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82884754: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82884758: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8288475C: 3BC10064  addi r30, r1, 0x64
	ctx.r[30].s64 = ctx.r[1].s64 + 100;
	// 82884760: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884764: 485908CD  bl 0x82e15030
	ctx.lr = 0x82884768;
	sub_82E15030(ctx, base);
	// 82884768: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8288476C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82884770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884774: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82884778: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8288477C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82884780: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82884784: 4BF67A85  bl 0x827ec208
	ctx.lr = 0x82884788;
	sub_827EC208(ctx, base);
	// 82884788: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8288478C: 907F03F8  stw r3, 0x3f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1016 as u32), ctx.r[3].u32 ) };
	// 82884790: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884794: 419A000C  beq cr6, 0x828847a0
	if ctx.cr[6].eq {
	pc = 0x828847A0; continue 'dispatch;
	}
	// 82884798: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8288479C: 4BA3C0F5  bl 0x822c0890
	ctx.lr = 0x828847A0;
	sub_822C0890(ctx, base);
	// 828847A0: 8061009C  lwz r3, 0x9c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 828847A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828847A8: 419A0008  beq cr6, 0x828847b0
	if ctx.cr[6].eq {
	pc = 0x828847B0; continue 'dispatch;
	}
	// 828847AC: 4BA3C0E5  bl 0x822c0890
	ctx.lr = 0x828847B0;
	sub_822C0890(ctx, base);
	// 828847B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 828847B4: 4856EC75  bl 0x82df3428
	ctx.lr = 0x828847B8;
	sub_82DF3428(ctx, base);
	// 828847B8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 828847BC: 4800009C  b 0x82884858
	pc = 0x82884858; continue 'dispatch;
	// 828847C0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828847C4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 828847C8: 388B4CD0  addi r4, r11, 0x4cd0
	ctx.r[4].s64 = ctx.r[11].s64 + 19664;
	// 828847CC: 4856F23D  bl 0x82df3a08
	ctx.lr = 0x828847D0;
	sub_82DF3A08(ctx, base);
	// 828847D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 828847D4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 828847D8: 388B3568  addi r4, r11, 0x3568
	ctx.r[4].s64 = ctx.r[11].s64 + 13672;
	// 828847DC: 4856F22D  bl 0x82df3a08
	ctx.lr = 0x828847E0;
	sub_82DF3A08(ctx, base);
	// 828847E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 828847E4: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 828847E8: 4BF67DE1  bl 0x827ec5c8
	ctx.lr = 0x828847EC;
	sub_827EC5C8(ctx, base);
	// 828847EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 828847F0: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 828847F4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 828847F8: 3BC1006C  addi r30, r1, 0x6c
	ctx.r[30].s64 = ctx.r[1].s64 + 108;
	// 828847FC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884800: 48590831  bl 0x82e15030
	ctx.lr = 0x82884804;
	sub_82E15030(ctx, base);
	// 82884804: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82884808: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8288480C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884810: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82884814: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82884818: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8288481C: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82884820: 4BF679E9  bl 0x827ec208
	ctx.lr = 0x82884824;
	sub_827EC208(ctx, base);
	// 82884824: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82884828: 907F03F8  stw r3, 0x3f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1016 as u32), ctx.r[3].u32 ) };
	// 8288482C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884830: 419A000C  beq cr6, 0x8288483c
	if ctx.cr[6].eq {
	pc = 0x8288483C; continue 'dispatch;
	}
	// 82884834: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82884838: 4BA3C059  bl 0x822c0890
	ctx.lr = 0x8288483C;
	sub_822C0890(ctx, base);
	// 8288483C: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82884840: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884844: 419A0008  beq cr6, 0x8288484c
	if ctx.cr[6].eq {
	pc = 0x8288484C; continue 'dispatch;
	}
	// 82884848: 4BA3C049  bl 0x822c0890
	ctx.lr = 0x8288484C;
	sub_822C0890(ctx, base);
	// 8288484C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82884850: 4856EBD9  bl 0x82df3428
	ctx.lr = 0x82884854;
	sub_82DF3428(ctx, base);
	// 82884854: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82884858: 4856EBD1  bl 0x82df3428
	ctx.lr = 0x8288485C;
	sub_82DF3428(ctx, base);
	// 8288485C: 48000038  b 0x82884894
	pc = 0x82884894; continue 'dispatch;
	// 82884860: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82884864: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82884868: 4BC8AC61  bl 0x8250f4c8
	ctx.lr = 0x8288486C;
	sub_8250F4C8(ctx, base);
	// 8288486C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884870: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884874: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82884878: 409A0008  bne cr6, 0x82884880
	if !ctx.cr[6].eq {
	pc = 0x82884880; continue 'dispatch;
	}
	// 8288487C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82884880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884884: 80BF03F8  lwz r5, 0x3f8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1016 as u32) ) } as u64;
	// 82884888: 4BF67A09  bl 0x827ec290
	ctx.lr = 0x8288488C;
	sub_827EC290(ctx, base);
	// 8288488C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82884890: 4856D401  bl 0x82df1c90
	ctx.lr = 0x82884894;
	sub_82DF1C90(ctx, base);
	// 82884894: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82884898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828848A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 828848A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828848A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828848B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x828848B0 size=232
    let mut pc: u32 = 0x828848B0;
    'dispatch: loop {
        match pc {
            0x828848B0 => {
    //   block [0x828848B0..0x82884998)
	// 828848B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828848B4: 489238B5  bl 0x831a8168
	ctx.lr = 0x828848B8;
	sub_831A8130(ctx, base);
	// 828848B8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 828848BC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828848C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 828848C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 828848C8: 4BF67899  bl 0x827ec160
	ctx.lr = 0x828848CC;
	sub_827EC160(ctx, base);
	// 828848CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 828848D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828848D4: 4BF6AF7D  bl 0x827ef850
	ctx.lr = 0x828848D8;
	sub_827EF850(ctx, base);
	// 828848D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828848DC: 4BF67885  bl 0x827ec160
	ctx.lr = 0x828848E0;
	sub_827EC160(ctx, base);
	// 828848E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 828848E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 828848E8: 480845E1  bl 0x82908ec8
	ctx.lr = 0x828848EC;
	sub_82908EC8(ctx, base);
	// 828848EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 828848F0: 3B830010  addi r28, r3, 0x10
	ctx.r[28].s64 = ctx.r[3].s64 + 16;
	// 828848F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828848F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 828848FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82884900: 4E800421  bctrl
	ctx.lr = 0x82884904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884904: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82884908: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8288490C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884998 size=364
    let mut pc: u32 = 0x82884998;
    'dispatch: loop {
        match pc {
            0x82884998 => {
    //   block [0x82884998..0x82884B04)
	// 82884998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288499C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828849A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 828849A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828849A8: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884B08 size=328
    let mut pc: u32 = 0x82884B08;
    'dispatch: loop {
        match pc {
            0x82884B08 => {
    //   block [0x82884B08..0x82884C50)
	// 82884B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82884B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82884B18: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82884C50 size=196
    let mut pc: u32 = 0x82884C50;
    'dispatch: loop {
        match pc {
            0x82884C50 => {
    //   block [0x82884C50..0x82884D14)
	// 82884C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884C54: 48923519  bl 0x831a816c
	ctx.lr = 0x82884C58;
	sub_831A8130(ctx, base);
	// 82884C58: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82884C60: 817F0400  lwz r11, 0x400(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1024 as u32) ) } as u64;
	// 82884C64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884C68: 419A00A4  beq cr6, 0x82884d0c
	if ctx.cr[6].eq {
	pc = 0x82884D0C; continue 'dispatch;
	}
	// 82884C6C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82884C70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884C74: 808BE760  lwz r4, -0x18a0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6304 as u32) ) } as u64;
	// 82884C78: 4856ED91  bl 0x82df3a08
	ctx.lr = 0x82884C7C;
	sub_82DF3A08(ctx, base);
	// 82884C7C: 807F01CC  lwz r3, 0x1cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 82884C80: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82884C84: 4BF65625  bl 0x827ea2a8
	ctx.lr = 0x82884C88;
	sub_827EA2A8(ctx, base);
	// 82884C88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82884C8C: 4856E67D  bl 0x82df3308
	ctx.lr = 0x82884C90;
	sub_82DF3308(ctx, base);
	// 82884C90: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82884C94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884C98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82884C9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82884CA0: 697E0001  xori r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u64 ^ 1;
	// 82884CA4: 4856E785  bl 0x82df3428
	ctx.lr = 0x82884CA8;
	sub_82DF3428(ctx, base);
	// 82884CA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82884CAC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82884CB0: 480849A9  bl 0x82909658
	ctx.lr = 0x82884CB4;
	sub_82909658(ctx, base);
	// 82884CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884CB8: 4818E9C9  bl 0x82a13680
	ctx.lr = 0x82884CBC;
	sub_82A13680(ctx, base);
	// 82884CBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82884CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82884CC4: 4818E9C5  bl 0x82a13688
	ctx.lr = 0x82884CC8;
	sub_82A13688(ctx, base);
	// 82884CC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82884CCC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82884CD0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82884CD4: 4BA82AA5  bl 0x82307778
	ctx.lr = 0x82884CD8;
	sub_82307778(ctx, base);
	// 82884CD8: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82884CDC: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 82884CE0: 485F6D89  bl 0x82e7ba68
	ctx.lr = 0x82884CE4;
	sub_82E7BA68(ctx, base);
	// 82884CE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82884CE8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82884CEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82884CF0: 485F6FD9  bl 0x82e7bcc8
	ctx.lr = 0x82884CF4;
	sub_82E7BCC8(ctx, base);
	// 82884CF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82884CF8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82884CFC: 807F0400  lwz r3, 0x400(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1024 as u32) ) } as u64;
	// 82884D00: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82884D04: C02B0A90  lfs f1, 0xa90(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2704 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82884D08: 480F84D9  bl 0x8297d1e0
	ctx.lr = 0x82884D0C;
	sub_8297D1E0(ctx, base);
	// 82884D0C: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82884D10: 489234AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884D18 size=8
    let mut pc: u32 = 0x82884D18;
    'dispatch: loop {
        match pc {
            0x82884D18 => {
    //   block [0x82884D18..0x82884D20)
	// 82884D18: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82884D1C: 48002304  b 0x82887020
	sub_82887020(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884D20 size=8
    let mut pc: u32 = 0x82884D20;
    'dispatch: loop {
        match pc {
            0x82884D20 => {
    //   block [0x82884D20..0x82884D28)
	// 82884D20: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 82884D24: 480022FC  b 0x82887020
	sub_82887020(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884D28 size=8
    let mut pc: u32 = 0x82884D28;
    'dispatch: loop {
        match pc {
            0x82884D28 => {
    //   block [0x82884D28..0x82884D30)
	// 82884D28: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82884D2C: 480022F4  b 0x82887020
	sub_82887020(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884D30 size=232
    let mut pc: u32 = 0x82884D30;
    'dispatch: loop {
        match pc {
            0x82884D30 => {
    //   block [0x82884D30..0x82884E18)
	// 82884D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884D34: 48923431  bl 0x831a8164
	ctx.lr = 0x82884D38;
	sub_831A8130(ctx, base);
	// 82884D38: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884D3C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82884D40: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82884D44: 3BABFDE0  addi r29, r11, -0x220
	ctx.r[29].s64 = ctx.r[11].s64 + -544;
	// 82884D48: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884D4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82884D50: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82884D54: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 82884D58: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82884D5C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82884D60: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82884D64: 48923D75  bl 0x831a8ad8
	ctx.lr = 0x82884D68;
	sub_831A8AD8(ctx, base);
	// 82884D68: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82884D6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884D70: 4856EC99  bl 0x82df3a08
	ctx.lr = 0x82884D74;
	sub_82DF3A08(ctx, base);
	// 82884D74: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884D78: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82884D7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82884D80: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884D84: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82884D88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82884D8C: 4E800421  bctrl
	ctx.lr = 0x82884D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884D90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884D94: 4856E695  bl 0x82df3428
	ctx.lr = 0x82884D98;
	sub_82DF3428(ctx, base);
	// 82884D98: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82884D9C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82884DA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884DA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82884DA8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82884DAC: 419A0024  beq cr6, 0x82884dd0
	if ctx.cr[6].eq {
	pc = 0x82884DD0; continue 'dispatch;
	}
	// 82884DB0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82884DB4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82884DB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884DBC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82884DC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82884DC4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82884DC8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884DCC: 4082FFE8  bne 0x82884db4
	if !ctx.cr[0].eq {
	pc = 0x82884DB4; continue 'dispatch;
	}
	// 82884DD0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82884DD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82884DD8: 4800DDC9  bl 0x82892ba0
	ctx.lr = 0x82884DDC;
	sub_82892BA0(ctx, base);
	// 82884DDC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82884DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884DE4: 419A0008  beq cr6, 0x82884dec
	if ctx.cr[6].eq {
	pc = 0x82884DEC; continue 'dispatch;
	}
	// 82884DE8: 4BA3BAA9  bl 0x822c0890
	ctx.lr = 0x82884DEC;
	sub_822C0890(ctx, base);
	// 82884DEC: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 82884DF0: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 82884DF4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82884DF8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82884DFC: 4198FF5C  blt cr6, 0x82884d58
	if ctx.cr[6].lt {
	pc = 0x82884D58; continue 'dispatch;
	}
	// 82884E00: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82884E04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884E08: 419A0008  beq cr6, 0x82884e10
	if ctx.cr[6].eq {
	pc = 0x82884E10; continue 'dispatch;
	}
	// 82884E0C: 4BA3BA85  bl 0x822c0890
	ctx.lr = 0x82884E10;
	sub_822C0890(ctx, base);
	// 82884E10: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82884E14: 489233A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884E18 size=128
    let mut pc: u32 = 0x82884E18;
    'dispatch: loop {
        match pc {
            0x82884E18 => {
    //   block [0x82884E18..0x82884E98)
	// 82884E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82884E24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884E28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82884E2C: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82884E30: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884E34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884E38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82884E3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82884E40: 419A0024  beq cr6, 0x82884e64
	if ctx.cr[6].eq {
	pc = 0x82884E64; continue 'dispatch;
	}
	// 82884E44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82884E48: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82884E4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884E50: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82884E54: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82884E58: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82884E5C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884E60: 4082FFE8  bne 0x82884e48
	if !ctx.cr[0].eq {
	pc = 0x82884E48; continue 'dispatch;
	}
	// 82884E64: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82884E68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82884E6C: 386BFCD8  addi r3, r11, -0x328
	ctx.r[3].s64 = ctx.r[11].s64 + -808;
	// 82884E70: 4800E749  bl 0x828935b8
	ctx.lr = 0x82884E74;
	sub_828935B8(ctx, base);
	// 82884E74: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884E78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884E7C: 419A0008  beq cr6, 0x82884e84
	if ctx.cr[6].eq {
	pc = 0x82884E84; continue 'dispatch;
	}
	// 82884E80: 4BA3BA11  bl 0x822c0890
	ctx.lr = 0x82884E84;
	sub_822C0890(ctx, base);
	// 82884E84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82884E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82884E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82884E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82884E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884E98 size=8
    let mut pc: u32 = 0x82884E98;
    'dispatch: loop {
        match pc {
            0x82884E98 => {
    //   block [0x82884E98..0x82884EA0)
	// 82884E98: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82884E9C: 48002314  b 0x828871b0
	sub_828871B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884EA0 size=8
    let mut pc: u32 = 0x82884EA0;
    'dispatch: loop {
        match pc {
            0x82884EA0 => {
    //   block [0x82884EA0..0x82884EA8)
	// 82884EA0: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 82884EA4: 4800230C  b 0x828871b0
	sub_828871B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82884EA8 size=8
    let mut pc: u32 = 0x82884EA8;
    'dispatch: loop {
        match pc {
            0x82884EA8 => {
    //   block [0x82884EA8..0x82884EB0)
	// 82884EA8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82884EAC: 48002304  b 0x828871b0
	sub_828871B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884EB0 size=232
    let mut pc: u32 = 0x82884EB0;
    'dispatch: loop {
        match pc {
            0x82884EB0 => {
    //   block [0x82884EB0..0x82884F98)
	// 82884EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884EB4: 489232B1  bl 0x831a8164
	ctx.lr = 0x82884EB8;
	sub_831A8130(ctx, base);
	// 82884EB8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884EBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82884EC0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82884EC4: 3BAB1478  addi r29, r11, 0x1478
	ctx.r[29].s64 = ctx.r[11].s64 + 5240;
	// 82884EC8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82884ECC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82884ED0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82884ED4: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 82884ED8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82884EDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82884EE0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82884EE4: 48923BF5  bl 0x831a8ad8
	ctx.lr = 0x82884EE8;
	sub_831A8AD8(ctx, base);
	// 82884EE8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82884EEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884EF0: 4856EB19  bl 0x82df3a08
	ctx.lr = 0x82884EF4;
	sub_82DF3A08(ctx, base);
	// 82884EF4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884EF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82884EFC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82884F00: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884F04: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82884F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82884F0C: 4E800421  bctrl
	ctx.lr = 0x82884F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82884F10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82884F14: 4856E515  bl 0x82df3428
	ctx.lr = 0x82884F18;
	sub_82DF3428(ctx, base);
	// 82884F18: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82884F1C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82884F20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884F24: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82884F28: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82884F2C: 419A0024  beq cr6, 0x82884f50
	if ctx.cr[6].eq {
	pc = 0x82884F50; continue 'dispatch;
	}
	// 82884F30: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82884F34: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82884F38: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884F3C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82884F40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82884F44: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82884F48: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884F4C: 4082FFE8  bne 0x82884f34
	if !ctx.cr[0].eq {
	pc = 0x82884F34; continue 'dispatch;
	}
	// 82884F50: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82884F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82884F58: 4800DC49  bl 0x82892ba0
	ctx.lr = 0x82884F5C;
	sub_82892BA0(ctx, base);
	// 82884F5C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82884F60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884F64: 419A0008  beq cr6, 0x82884f6c
	if ctx.cr[6].eq {
	pc = 0x82884F6C; continue 'dispatch;
	}
	// 82884F68: 4BA3B929  bl 0x822c0890
	ctx.lr = 0x82884F6C;
	sub_822C0890(ctx, base);
	// 82884F6C: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 82884F70: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 82884F74: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82884F78: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82884F7C: 4198FF5C  blt cr6, 0x82884ed8
	if ctx.cr[6].lt {
	pc = 0x82884ED8; continue 'dispatch;
	}
	// 82884F80: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82884F84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884F88: 419A0008  beq cr6, 0x82884f90
	if ctx.cr[6].eq {
	pc = 0x82884F90; continue 'dispatch;
	}
	// 82884F8C: 4BA3B905  bl 0x822c0890
	ctx.lr = 0x82884F90;
	sub_822C0890(ctx, base);
	// 82884F90: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82884F94: 48923220  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82884F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82884F98 size=128
    let mut pc: u32 = 0x82884F98;
    'dispatch: loop {
        match pc {
            0x82884F98 => {
    //   block [0x82884F98..0x82885018)
	// 82884F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82884F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82884FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82884FA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82884FA8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82884FAC: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82884FB0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884FB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82884FB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82884FBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82884FC0: 419A0024  beq cr6, 0x82884fe4
	if ctx.cr[6].eq {
	pc = 0x82884FE4; continue 'dispatch;
	}
	// 82884FC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82884FC8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82884FCC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884FD0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82884FD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82884FD8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82884FDC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82884FE0: 4082FFE8  bne 0x82884fc8
	if !ctx.cr[0].eq {
	pc = 0x82884FC8; continue 'dispatch;
	}
	// 82884FE4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82884FE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82884FEC: 386B1370  addi r3, r11, 0x1370
	ctx.r[3].s64 = ctx.r[11].s64 + 4976;
	// 82884FF0: 4800E5C9  bl 0x828935b8
	ctx.lr = 0x82884FF4;
	sub_828935B8(ctx, base);
	// 82884FF4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82884FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82884FFC: 419A0008  beq cr6, 0x82885004
	if ctx.cr[6].eq {
	pc = 0x82885004; continue 'dispatch;
	}
	// 82885000: 4BA3B891  bl 0x822c0890
	ctx.lr = 0x82885004;
	sub_822C0890(ctx, base);
	// 82885004: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82885008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288500C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82885014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885018 size=8
    let mut pc: u32 = 0x82885018;
    'dispatch: loop {
        match pc {
            0x82885018 => {
    //   block [0x82885018..0x82885020)
	// 82885018: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8288501C: 48002324  b 0x82887340
	sub_82887340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885020 size=8
    let mut pc: u32 = 0x82885020;
    'dispatch: loop {
        match pc {
            0x82885020 => {
    //   block [0x82885020..0x82885028)
	// 82885020: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82885024: 4800231C  b 0x82887340
	sub_82887340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885028 size=8
    let mut pc: u32 = 0x82885028;
    'dispatch: loop {
        match pc {
            0x82885028 => {
    //   block [0x82885028..0x82885030)
	// 82885028: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 8288502C: 48002314  b 0x82887340
	sub_82887340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885030 size=232
    let mut pc: u32 = 0x82885030;
    'dispatch: loop {
        match pc {
            0x82885030 => {
    //   block [0x82885030..0x82885118)
	// 82885030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885034: 48923131  bl 0x831a8164
	ctx.lr = 0x82885038;
	sub_831A8130(ctx, base);
	// 82885038: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288503C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82885040: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82885044: 3BAB2B10  addi r29, r11, 0x2b10
	ctx.r[29].s64 = ctx.r[11].s64 + 11024;
	// 82885048: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288504C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82885050: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82885054: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 82885058: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8288505C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82885060: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82885064: 48923A75  bl 0x831a8ad8
	ctx.lr = 0x82885068;
	sub_831A8AD8(ctx, base);
	// 82885068: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8288506C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885070: 4856E999  bl 0x82df3a08
	ctx.lr = 0x82885074;
	sub_82DF3A08(ctx, base);
	// 82885074: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885078: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8288507C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82885080: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885084: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82885088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288508C: 4E800421  bctrl
	ctx.lr = 0x82885090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82885090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885094: 4856E395  bl 0x82df3428
	ctx.lr = 0x82885098;
	sub_82DF3428(ctx, base);
	// 82885098: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8288509C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 828850A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828850A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828850A8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828850AC: 419A0024  beq cr6, 0x828850d0
	if ctx.cr[6].eq {
	pc = 0x828850D0; continue 'dispatch;
	}
	// 828850B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828850B4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828850B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828850BC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828850C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828850C4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828850C8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828850CC: 4082FFE8  bne 0x828850b4
	if !ctx.cr[0].eq {
	pc = 0x828850B4; continue 'dispatch;
	}
	// 828850D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 828850D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828850D8: 4800DAC9  bl 0x82892ba0
	ctx.lr = 0x828850DC;
	sub_82892BA0(ctx, base);
	// 828850DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828850E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828850E4: 419A0008  beq cr6, 0x828850ec
	if ctx.cr[6].eq {
	pc = 0x828850EC; continue 'dispatch;
	}
	// 828850E8: 4BA3B7A9  bl 0x822c0890
	ctx.lr = 0x828850EC;
	sub_822C0890(ctx, base);
	// 828850EC: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 828850F0: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 828850F4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 828850F8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 828850FC: 4198FF5C  blt cr6, 0x82885058
	if ctx.cr[6].lt {
	pc = 0x82885058; continue 'dispatch;
	}
	// 82885100: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82885104: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885108: 419A0008  beq cr6, 0x82885110
	if ctx.cr[6].eq {
	pc = 0x82885110; continue 'dispatch;
	}
	// 8288510C: 4BA3B785  bl 0x822c0890
	ctx.lr = 0x82885110;
	sub_822C0890(ctx, base);
	// 82885110: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82885114: 489230A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885118 size=128
    let mut pc: u32 = 0x82885118;
    'dispatch: loop {
        match pc {
            0x82885118 => {
    //   block [0x82885118..0x82885198)
	// 82885118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288511C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885120: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82885124: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885128: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8288512C: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82885130: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82885138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288513C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82885140: 419A0024  beq cr6, 0x82885164
	if ctx.cr[6].eq {
	pc = 0x82885164; continue 'dispatch;
	}
	// 82885144: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82885148: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8288514C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885150: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82885154: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82885158: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8288515C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885160: 4082FFE8  bne 0x82885148
	if !ctx.cr[0].eq {
	pc = 0x82885148; continue 'dispatch;
	}
	// 82885164: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82885168: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288516C: 386B2A08  addi r3, r11, 0x2a08
	ctx.r[3].s64 = ctx.r[11].s64 + 10760;
	// 82885170: 4800E449  bl 0x828935b8
	ctx.lr = 0x82885174;
	sub_828935B8(ctx, base);
	// 82885174: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288517C: 419A0008  beq cr6, 0x82885184
	if ctx.cr[6].eq {
	pc = 0x82885184; continue 'dispatch;
	}
	// 82885180: 4BA3B711  bl 0x822c0890
	ctx.lr = 0x82885184;
	sub_822C0890(ctx, base);
	// 82885184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82885188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288518C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82885194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885198 size=8
    let mut pc: u32 = 0x82885198;
    'dispatch: loop {
        match pc {
            0x82885198 => {
    //   block [0x82885198..0x828851A0)
	// 82885198: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8288519C: 48002324  b 0x828874c0
	sub_828874C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828851A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828851A0 size=8
    let mut pc: u32 = 0x828851A0;
    'dispatch: loop {
        match pc {
            0x828851A0 => {
    //   block [0x828851A0..0x828851A8)
	// 828851A0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 828851A4: 4800231C  b 0x828874c0
	sub_828874C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828851A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828851A8 size=8
    let mut pc: u32 = 0x828851A8;
    'dispatch: loop {
        match pc {
            0x828851A8 => {
    //   block [0x828851A8..0x828851B0)
	// 828851A8: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 828851AC: 48002314  b 0x828874c0
	sub_828874C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828851B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828851B0 size=232
    let mut pc: u32 = 0x828851B0;
    'dispatch: loop {
        match pc {
            0x828851B0 => {
    //   block [0x828851B0..0x82885298)
	// 828851B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828851B4: 48922FB1  bl 0x831a8164
	ctx.lr = 0x828851B8;
	sub_831A8130(ctx, base);
	// 828851B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828851BC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828851C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 828851C4: 3BAB41A8  addi r29, r11, 0x41a8
	ctx.r[29].s64 = ctx.r[11].s64 + 16808;
	// 828851C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828851CC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828851D0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 828851D4: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 828851D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 828851DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828851E0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 828851E4: 489238F5  bl 0x831a8ad8
	ctx.lr = 0x828851E8;
	sub_831A8AD8(ctx, base);
	// 828851E8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 828851EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828851F0: 4856E819  bl 0x82df3a08
	ctx.lr = 0x828851F4;
	sub_82DF3A08(ctx, base);
	// 828851F4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 828851F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828851FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82885200: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885204: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82885208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288520C: 4E800421  bctrl
	ctx.lr = 0x82885210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82885210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885214: 4856E215  bl 0x82df3428
	ctx.lr = 0x82885218;
	sub_82DF3428(ctx, base);
	// 82885218: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8288521C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82885220: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82885224: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82885228: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8288522C: 419A0024  beq cr6, 0x82885250
	if ctx.cr[6].eq {
	pc = 0x82885250; continue 'dispatch;
	}
	// 82885230: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82885234: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82885238: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8288523C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82885240: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82885244: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82885248: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8288524C: 4082FFE8  bne 0x82885234
	if !ctx.cr[0].eq {
	pc = 0x82885234; continue 'dispatch;
	}
	// 82885250: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82885254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82885258: 4800D949  bl 0x82892ba0
	ctx.lr = 0x8288525C;
	sub_82892BA0(ctx, base);
	// 8288525C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82885260: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885264: 419A0008  beq cr6, 0x8288526c
	if ctx.cr[6].eq {
	pc = 0x8288526C; continue 'dispatch;
	}
	// 82885268: 4BA3B629  bl 0x822c0890
	ctx.lr = 0x8288526C;
	sub_822C0890(ctx, base);
	// 8288526C: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 82885270: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 82885274: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82885278: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8288527C: 4198FF5C  blt cr6, 0x828851d8
	if ctx.cr[6].lt {
	pc = 0x828851D8; continue 'dispatch;
	}
	// 82885280: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82885284: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885288: 419A0008  beq cr6, 0x82885290
	if ctx.cr[6].eq {
	pc = 0x82885290; continue 'dispatch;
	}
	// 8288528C: 4BA3B605  bl 0x822c0890
	ctx.lr = 0x82885290;
	sub_822C0890(ctx, base);
	// 82885290: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82885294: 48922F20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885298 size=128
    let mut pc: u32 = 0x82885298;
    'dispatch: loop {
        match pc {
            0x82885298 => {
    //   block [0x82885298..0x82885318)
	// 82885298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828852A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828852A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828852A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828852AC: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 828852B0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828852B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828852B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828852BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828852C0: 419A0024  beq cr6, 0x828852e4
	if ctx.cr[6].eq {
	pc = 0x828852E4; continue 'dispatch;
	}
	// 828852C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828852C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828852CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828852D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828852D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828852D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828852DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828852E0: 4082FFE8  bne 0x828852c8
	if !ctx.cr[0].eq {
	pc = 0x828852C8; continue 'dispatch;
	}
	// 828852E4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828852E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828852EC: 386B40A0  addi r3, r11, 0x40a0
	ctx.r[3].s64 = ctx.r[11].s64 + 16544;
	// 828852F0: 4800E2C9  bl 0x828935b8
	ctx.lr = 0x828852F4;
	sub_828935B8(ctx, base);
	// 828852F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828852F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828852FC: 419A0008  beq cr6, 0x82885304
	if ctx.cr[6].eq {
	pc = 0x82885304; continue 'dispatch;
	}
	// 82885300: 4BA3B591  bl 0x822c0890
	ctx.lr = 0x82885304;
	sub_822C0890(ctx, base);
	// 82885304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82885308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82885314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885318 size=8
    let mut pc: u32 = 0x82885318;
    'dispatch: loop {
        match pc {
            0x82885318 => {
    //   block [0x82885318..0x82885320)
	// 82885318: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8288531C: 48002334  b 0x82887650
	sub_82887650(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885320 size=8
    let mut pc: u32 = 0x82885320;
    'dispatch: loop {
        match pc {
            0x82885320 => {
    //   block [0x82885320..0x82885328)
	// 82885320: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82885324: 4800232C  b 0x82887650
	sub_82887650(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885328 size=8
    let mut pc: u32 = 0x82885328;
    'dispatch: loop {
        match pc {
            0x82885328 => {
    //   block [0x82885328..0x82885330)
	// 82885328: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 8288532C: 48002324  b 0x82887650
	sub_82887650(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885330 size=232
    let mut pc: u32 = 0x82885330;
    'dispatch: loop {
        match pc {
            0x82885330 => {
    //   block [0x82885330..0x82885418)
	// 82885330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885334: 48922E31  bl 0x831a8164
	ctx.lr = 0x82885338;
	sub_831A8130(ctx, base);
	// 82885338: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288533C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82885340: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82885344: 3BAB5840  addi r29, r11, 0x5840
	ctx.r[29].s64 = ctx.r[11].s64 + 22592;
	// 82885348: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288534C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82885350: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82885354: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 82885358: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8288535C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82885360: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82885364: 48923775  bl 0x831a8ad8
	ctx.lr = 0x82885368;
	sub_831A8AD8(ctx, base);
	// 82885368: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8288536C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885370: 4856E699  bl 0x82df3a08
	ctx.lr = 0x82885374;
	sub_82DF3A08(ctx, base);
	// 82885374: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885378: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8288537C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82885380: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885384: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82885388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288538C: 4E800421  bctrl
	ctx.lr = 0x82885390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82885390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885394: 4856E095  bl 0x82df3428
	ctx.lr = 0x82885398;
	sub_82DF3428(ctx, base);
	// 82885398: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8288539C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 828853A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828853A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828853A8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828853AC: 419A0024  beq cr6, 0x828853d0
	if ctx.cr[6].eq {
	pc = 0x828853D0; continue 'dispatch;
	}
	// 828853B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828853B4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828853B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828853BC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828853C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828853C4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828853C8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828853CC: 4082FFE8  bne 0x828853b4
	if !ctx.cr[0].eq {
	pc = 0x828853B4; continue 'dispatch;
	}
	// 828853D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 828853D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828853D8: 4800D7C9  bl 0x82892ba0
	ctx.lr = 0x828853DC;
	sub_82892BA0(ctx, base);
	// 828853DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828853E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828853E4: 419A0008  beq cr6, 0x828853ec
	if ctx.cr[6].eq {
	pc = 0x828853EC; continue 'dispatch;
	}
	// 828853E8: 4BA3B4A9  bl 0x822c0890
	ctx.lr = 0x828853EC;
	sub_822C0890(ctx, base);
	// 828853EC: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 828853F0: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 828853F4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 828853F8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 828853FC: 4198FF5C  blt cr6, 0x82885358
	if ctx.cr[6].lt {
	pc = 0x82885358; continue 'dispatch;
	}
	// 82885400: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82885404: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885408: 419A0008  beq cr6, 0x82885410
	if ctx.cr[6].eq {
	pc = 0x82885410; continue 'dispatch;
	}
	// 8288540C: 4BA3B485  bl 0x822c0890
	ctx.lr = 0x82885410;
	sub_822C0890(ctx, base);
	// 82885410: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82885414: 48922DA0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885418 size=128
    let mut pc: u32 = 0x82885418;
    'dispatch: loop {
        match pc {
            0x82885418 => {
    //   block [0x82885418..0x82885498)
	// 82885418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82885424: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885428: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8288542C: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82885430: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82885438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288543C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82885440: 419A0024  beq cr6, 0x82885464
	if ctx.cr[6].eq {
	pc = 0x82885464; continue 'dispatch;
	}
	// 82885444: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82885448: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8288544C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885450: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82885454: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82885458: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8288545C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885460: 4082FFE8  bne 0x82885448
	if !ctx.cr[0].eq {
	pc = 0x82885448; continue 'dispatch;
	}
	// 82885464: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 82885468: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288546C: 386B5738  addi r3, r11, 0x5738
	ctx.r[3].s64 = ctx.r[11].s64 + 22328;
	// 82885470: 4800E149  bl 0x828935b8
	ctx.lr = 0x82885474;
	sub_828935B8(ctx, base);
	// 82885474: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885478: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288547C: 419A0008  beq cr6, 0x82885484
	if ctx.cr[6].eq {
	pc = 0x82885484; continue 'dispatch;
	}
	// 82885480: 4BA3B411  bl 0x822c0890
	ctx.lr = 0x82885484;
	sub_822C0890(ctx, base);
	// 82885484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82885488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288548C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82885494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885498 size=8
    let mut pc: u32 = 0x82885498;
    'dispatch: loop {
        match pc {
            0x82885498 => {
    //   block [0x82885498..0x828854A0)
	// 82885498: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 8288549C: 48002344  b 0x828877e0
	sub_828877E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828854A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828854A0 size=8
    let mut pc: u32 = 0x828854A0;
    'dispatch: loop {
        match pc {
            0x828854A0 => {
    //   block [0x828854A0..0x828854A8)
	// 828854A0: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 828854A4: 4800233C  b 0x828877e0
	sub_828877E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828854A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828854A8 size=8
    let mut pc: u32 = 0x828854A8;
    'dispatch: loop {
        match pc {
            0x828854A8 => {
    //   block [0x828854A8..0x828854B0)
	// 828854A8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 828854AC: 48002334  b 0x828877e0
	sub_828877E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828854B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828854B0 size=232
    let mut pc: u32 = 0x828854B0;
    'dispatch: loop {
        match pc {
            0x828854B0 => {
    //   block [0x828854B0..0x82885598)
	// 828854B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828854B4: 48922CB1  bl 0x831a8164
	ctx.lr = 0x828854B8;
	sub_831A8130(ctx, base);
	// 828854B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828854BC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828854C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 828854C4: 3BAB6ED8  addi r29, r11, 0x6ed8
	ctx.r[29].s64 = ctx.r[11].s64 + 28376;
	// 828854C8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828854CC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828854D0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 828854D4: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 828854D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 828854DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 828854E0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 828854E4: 489235F5  bl 0x831a8ad8
	ctx.lr = 0x828854E8;
	sub_831A8AD8(ctx, base);
	// 828854E8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 828854EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828854F0: 4856E519  bl 0x82df3a08
	ctx.lr = 0x828854F4;
	sub_82DF3A08(ctx, base);
	// 828854F4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 828854F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 828854FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82885500: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885504: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82885508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288550C: 4E800421  bctrl
	ctx.lr = 0x82885510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82885510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885514: 4856DF15  bl 0x82df3428
	ctx.lr = 0x82885518;
	sub_82DF3428(ctx, base);
	// 82885518: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8288551C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82885520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82885524: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82885528: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8288552C: 419A0024  beq cr6, 0x82885550
	if ctx.cr[6].eq {
	pc = 0x82885550; continue 'dispatch;
	}
	// 82885530: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82885534: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82885538: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8288553C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82885540: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82885544: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82885548: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8288554C: 4082FFE8  bne 0x82885534
	if !ctx.cr[0].eq {
	pc = 0x82885534; continue 'dispatch;
	}
	// 82885550: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82885554: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82885558: 4800D649  bl 0x82892ba0
	ctx.lr = 0x8288555C;
	sub_82892BA0(ctx, base);
	// 8288555C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82885560: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885564: 419A0008  beq cr6, 0x8288556c
	if ctx.cr[6].eq {
	pc = 0x8288556C; continue 'dispatch;
	}
	// 82885568: 4BA3B329  bl 0x822c0890
	ctx.lr = 0x8288556C;
	sub_822C0890(ctx, base);
	// 8288556C: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 82885570: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 82885574: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82885578: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8288557C: 4198FF5C  blt cr6, 0x828854d8
	if ctx.cr[6].lt {
	pc = 0x828854D8; continue 'dispatch;
	}
	// 82885580: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82885584: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885588: 419A0008  beq cr6, 0x82885590
	if ctx.cr[6].eq {
	pc = 0x82885590; continue 'dispatch;
	}
	// 8288558C: 4BA3B305  bl 0x822c0890
	ctx.lr = 0x82885590;
	sub_822C0890(ctx, base);
	// 82885590: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82885594: 48922C20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885598 size=128
    let mut pc: u32 = 0x82885598;
    'dispatch: loop {
        match pc {
            0x82885598 => {
    //   block [0x82885598..0x82885618)
	// 82885598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288559C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828855A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828855A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828855A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828855AC: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 828855B0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828855B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828855B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828855BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828855C0: 419A0024  beq cr6, 0x828855e4
	if ctx.cr[6].eq {
	pc = 0x828855E4; continue 'dispatch;
	}
	// 828855C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828855C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828855CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828855D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828855D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828855D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828855DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828855E0: 4082FFE8  bne 0x828855c8
	if !ctx.cr[0].eq {
	pc = 0x828855C8; continue 'dispatch;
	}
	// 828855E4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 828855E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 828855EC: 386B6DD0  addi r3, r11, 0x6dd0
	ctx.r[3].s64 = ctx.r[11].s64 + 28112;
	// 828855F0: 4800DFC9  bl 0x828935b8
	ctx.lr = 0x828855F4;
	sub_828935B8(ctx, base);
	// 828855F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 828855F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828855FC: 419A0008  beq cr6, 0x82885604
	if ctx.cr[6].eq {
	pc = 0x82885604; continue 'dispatch;
	}
	// 82885600: 4BA3B291  bl 0x822c0890
	ctx.lr = 0x82885604;
	sub_822C0890(ctx, base);
	// 82885604: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82885608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288560C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82885614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82885618 size=28
    let mut pc: u32 = 0x82885618;
    'dispatch: loop {
        match pc {
            0x82885618 => {
    //   block [0x82885618..0x82885634)
	// 82885618: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8288561C: C1A30410  lfs f13, 0x410(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1040 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82885620: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82885624: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82885628: 4199000C  bgt cr6, 0x82885634
	if ctx.cr[6].gt {
		sub_82885634(ctx, base);
		return;
	}
	// 8288562C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82885630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885634 size=4
    let mut pc: u32 = 0x82885634;
    'dispatch: loop {
        match pc {
            0x82885634 => {
    //   block [0x82885634..0x82885638)
	// 82885634: 4BFFF27C  b 0x828848b0
	sub_828848B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885638 size=8
    let mut pc: u32 = 0x82885638;
    'dispatch: loop {
        match pc {
            0x82885638 => {
    //   block [0x82885638..0x82885640)
	// 82885638: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8288563C: 48002324  b 0x82887960
	sub_82887960(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885640 size=8
    let mut pc: u32 = 0x82885640;
    'dispatch: loop {
        match pc {
            0x82885640 => {
    //   block [0x82885640..0x82885648)
	// 82885640: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 82885644: 4800231C  b 0x82887960
	sub_82887960(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82885648 size=8
    let mut pc: u32 = 0x82885648;
    'dispatch: loop {
        match pc {
            0x82885648 => {
    //   block [0x82885648..0x82885650)
	// 82885648: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 8288564C: 48002314  b 0x82887960
	sub_82887960(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885650 size=232
    let mut pc: u32 = 0x82885650;
    'dispatch: loop {
        match pc {
            0x82885650 => {
    //   block [0x82885650..0x82885738)
	// 82885650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885654: 48922B11  bl 0x831a8164
	ctx.lr = 0x82885658;
	sub_831A8130(ctx, base);
	// 82885658: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288565C: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 82885660: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82885664: 3BAB8570  addi r29, r11, -0x7a90
	ctx.r[29].s64 = ctx.r[11].s64 + -31376;
	// 82885668: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 8288566C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82885670: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82885674: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 82885678: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8288567C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82885680: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82885684: 48923455  bl 0x831a8ad8
	ctx.lr = 0x82885688;
	sub_831A8AD8(ctx, base);
	// 82885688: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8288568C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885690: 4856E379  bl 0x82df3a08
	ctx.lr = 0x82885694;
	sub_82DF3A08(ctx, base);
	// 82885694: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885698: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8288569C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 828856A0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 828856A4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 828856A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 828856AC: 4E800421  bctrl
	ctx.lr = 0x828856B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 828856B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 828856B4: 4856DD75  bl 0x82df3428
	ctx.lr = 0x828856B8;
	sub_82DF3428(ctx, base);
	// 828856B8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 828856BC: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 828856C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828856C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 828856C8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 828856CC: 419A0024  beq cr6, 0x828856f0
	if ctx.cr[6].eq {
	pc = 0x828856F0; continue 'dispatch;
	}
	// 828856D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828856D4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828856D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828856DC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828856E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828856E4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828856E8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828856EC: 4082FFE8  bne 0x828856d4
	if !ctx.cr[0].eq {
	pc = 0x828856D4; continue 'dispatch;
	}
	// 828856F0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 828856F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 828856F8: 4800D4A9  bl 0x82892ba0
	ctx.lr = 0x828856FC;
	sub_82892BA0(ctx, base);
	// 828856FC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82885700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885704: 419A0008  beq cr6, 0x8288570c
	if ctx.cr[6].eq {
	pc = 0x8288570C; continue 'dispatch;
	}
	// 82885708: 4BA3B189  bl 0x822c0890
	ctx.lr = 0x8288570C;
	sub_822C0890(ctx, base);
	// 8288570C: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 82885710: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 82885714: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82885718: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8288571C: 4198FF5C  blt cr6, 0x82885678
	if ctx.cr[6].lt {
	pc = 0x82885678; continue 'dispatch;
	}
	// 82885720: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82885724: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885728: 419A0008  beq cr6, 0x82885730
	if ctx.cr[6].eq {
	pc = 0x82885730; continue 'dispatch;
	}
	// 8288572C: 4BA3B165  bl 0x822c0890
	ctx.lr = 0x82885730;
	sub_822C0890(ctx, base);
	// 82885730: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82885734: 48922A80  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885738 size=128
    let mut pc: u32 = 0x82885738;
    'dispatch: loop {
        match pc {
            0x82885738 => {
    //   block [0x82885738..0x828857B8)
	// 82885738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82885744: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885748: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8288574C: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82885750: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82885758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8288575C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82885760: 419A0024  beq cr6, 0x82885784
	if ctx.cr[6].eq {
	pc = 0x82885784; continue 'dispatch;
	}
	// 82885764: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82885768: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8288576C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885770: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82885774: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82885778: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8288577C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885780: 4082FFE8  bne 0x82885768
	if !ctx.cr[0].eq {
	pc = 0x82885768; continue 'dispatch;
	}
	// 82885784: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 82885788: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288578C: 386B8468  addi r3, r11, -0x7b98
	ctx.r[3].s64 = ctx.r[11].s64 + -31640;
	// 82885790: 4800DE29  bl 0x828935b8
	ctx.lr = 0x82885794;
	sub_828935B8(ctx, base);
	// 82885794: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885798: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288579C: 419A0008  beq cr6, 0x828857a4
	if ctx.cr[6].eq {
	pc = 0x828857A4; continue 'dispatch;
	}
	// 828857A0: 4BA3B0F1  bl 0x822c0890
	ctx.lr = 0x828857A4;
	sub_822C0890(ctx, base);
	// 828857A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 828857A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828857AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828857B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 828857B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828857B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828857B8 size=8
    let mut pc: u32 = 0x828857B8;
    'dispatch: loop {
        match pc {
            0x828857B8 => {
    //   block [0x828857B8..0x828857C0)
	// 828857B8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 828857BC: 4800232C  b 0x82887ae8
	sub_82887AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828857C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828857C0 size=8
    let mut pc: u32 = 0x828857C0;
    'dispatch: loop {
        match pc {
            0x828857C0 => {
    //   block [0x828857C0..0x828857C8)
	// 828857C0: 3863FDF8  addi r3, r3, -0x208
	ctx.r[3].s64 = ctx.r[3].s64 + -520;
	// 828857C4: 48002324  b 0x82887ae8
	sub_82887AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828857C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x828857C8 size=8
    let mut pc: u32 = 0x828857C8;
    'dispatch: loop {
        match pc {
            0x828857C8 => {
    //   block [0x828857C8..0x828857D0)
	// 828857C8: 3863FCA0  addi r3, r3, -0x360
	ctx.r[3].s64 = ctx.r[3].s64 + -864;
	// 828857CC: 4800231C  b 0x82887ae8
	sub_82887AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828857D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828857D0 size=232
    let mut pc: u32 = 0x828857D0;
    'dispatch: loop {
        match pc {
            0x828857D0 => {
    //   block [0x828857D0..0x828858B8)
	// 828857D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828857D4: 48922991  bl 0x831a8164
	ctx.lr = 0x828857D8;
	sub_831A8130(ctx, base);
	// 828857D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828857DC: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 828857E0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 828857E4: 3BAB9C08  addi r29, r11, -0x63f8
	ctx.r[29].s64 = ctx.r[11].s64 + -25592;
	// 828857E8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 828857EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 828857F0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 828857F4: 3B6B0F7C  addi r27, r11, 0xf7c
	ctx.r[27].s64 = ctx.r[11].s64 + 3964;
	// 828857F8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 828857FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82885800: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82885804: 489232D5  bl 0x831a8ad8
	ctx.lr = 0x82885808;
	sub_831A8AD8(ctx, base);
	// 82885808: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8288580C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885810: 4856E1F9  bl 0x82df3a08
	ctx.lr = 0x82885814;
	sub_82DF3A08(ctx, base);
	// 82885814: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885818: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8288581C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82885820: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885824: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82885828: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8288582C: 4E800421  bctrl
	ctx.lr = 0x82885830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82885830: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82885834: 4856DBF5  bl 0x82df3428
	ctx.lr = 0x82885838;
	sub_82DF3428(ctx, base);
	// 82885838: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8288583C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82885840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82885844: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82885848: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8288584C: 419A0024  beq cr6, 0x82885870
	if ctx.cr[6].eq {
	pc = 0x82885870; continue 'dispatch;
	}
	// 82885850: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82885854: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82885858: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8288585C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82885860: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82885864: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82885868: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8288586C: 4082FFE8  bne 0x82885854
	if !ctx.cr[0].eq {
	pc = 0x82885854; continue 'dispatch;
	}
	// 82885870: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82885874: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82885878: 4800D329  bl 0x82892ba0
	ctx.lr = 0x8288587C;
	sub_82892BA0(ctx, base);
	// 8288587C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82885880: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82885884: 419A0008  beq cr6, 0x8288588c
	if ctx.cr[6].eq {
	pc = 0x8288588C; continue 'dispatch;
	}
	// 82885888: 4BA3B009  bl 0x822c0890
	ctx.lr = 0x8288588C;
	sub_822C0890(ctx, base);
	// 8288588C: 3BDE0228  addi r30, r30, 0x228
	ctx.r[30].s64 = ctx.r[30].s64 + 552;
	// 82885890: 397D1590  addi r11, r29, 0x1590
	ctx.r[11].s64 = ctx.r[29].s64 + 5520;
	// 82885894: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82885898: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8288589C: 4198FF5C  blt cr6, 0x828857f8
	if ctx.cr[6].lt {
	pc = 0x828857F8; continue 'dispatch;
	}
	// 828858A0: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 828858A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 828858A8: 419A0008  beq cr6, 0x828858b0
	if ctx.cr[6].eq {
	pc = 0x828858B0; continue 'dispatch;
	}
	// 828858AC: 4BA3AFE5  bl 0x822c0890
	ctx.lr = 0x828858B0;
	sub_822C0890(ctx, base);
	// 828858B0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 828858B4: 48922900  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828858B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828858B8 size=128
    let mut pc: u32 = 0x828858B8;
    'dispatch: loop {
        match pc {
            0x828858B8 => {
    //   block [0x828858B8..0x82885938)
	// 828858B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828858BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828858C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 828858C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828858C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 828858CC: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 828858D0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 828858D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 828858D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 828858DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 828858E0: 419A0024  beq cr6, 0x82885904
	if ctx.cr[6].eq {
	pc = 0x82885904; continue 'dispatch;
	}
	// 828858E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 828858E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 828858EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 828858F0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 828858F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 828858F8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 828858FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82885900: 4082FFE8  bne 0x828858e8
	if !ctx.cr[0].eq {
	pc = 0x828858E8; continue 'dispatch;
	}
	// 82885904: 3D608337  lis r11, -0x7cc9
	ctx.r[11].s64 = -2093547520;
	// 82885908: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8288590C: 386B9B00  addi r3, r11, -0x6500
	ctx.r[3].s64 = ctx.r[11].s64 + -25856;
	// 82885910: 4800DCA9  bl 0x828935b8
	ctx.lr = 0x82885914;
	sub_828935B8(ctx, base);
	// 82885914: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82885918: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8288591C: 419A0008  beq cr6, 0x82885924
	if ctx.cr[6].eq {
	pc = 0x82885924; continue 'dispatch;
	}
	// 82885920: 4BA3AF71  bl 0x822c0890
	ctx.lr = 0x82885924;
	sub_822C0890(ctx, base);
	// 82885924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82885928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8288592C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885930: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82885934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885938 size=72
    let mut pc: u32 = 0x82885938;
    'dispatch: loop {
        match pc {
            0x82885938 => {
    //   block [0x82885938..0x82885980)
	// 82885938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8288593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885944: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82885948: 419A001C  beq cr6, 0x82885964
	if ctx.cr[6].eq {
	pc = 0x82885964; continue 'dispatch;
	}
	// 8288594C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82885950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82885954: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82885958: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8288595C: 4BFFDB05  bl 0x82883460
	ctx.lr = 0x82885960;
	sub_82883460(ctx, base);
	// 82885960: 48000010  b 0x82885970
	pc = 0x82885970; continue 'dispatch;
	// 82885964: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82885968: 396BA720  addi r11, r11, -0x58e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22752;
	// 8288596C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82885970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82885974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82885978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8288597C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885980 size=72
    let mut pc: u32 = 0x82885980;
    'dispatch: loop {
        match pc {
            0x82885980 => {
    //   block [0x82885980..0x828859C8)
	// 82885980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8288598C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82885990: 419A001C  beq cr6, 0x828859ac
	if ctx.cr[6].eq {
	pc = 0x828859AC; continue 'dispatch;
	}
	// 82885994: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82885998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8288599C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 828859A0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828859A4: 4BFFDB45  bl 0x828834e8
	ctx.lr = 0x828859A8;
	sub_828834E8(ctx, base);
	// 828859A8: 48000010  b 0x828859b8
	pc = 0x828859B8; continue 'dispatch;
	// 828859AC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828859B0: 396BA7F8  addi r11, r11, -0x5808
	ctx.r[11].s64 = ctx.r[11].s64 + -22536;
	// 828859B4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 828859B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 828859BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 828859C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 828859C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_828859C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x828859C8 size=72
    let mut pc: u32 = 0x828859C8;
    'dispatch: loop {
        match pc {
            0x828859C8 => {
    //   block [0x828859C8..0x82885A10)
	// 828859C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 828859CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 828859D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 828859D4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 828859D8: 419A001C  beq cr6, 0x828859f4
	if ctx.cr[6].eq {
	pc = 0x828859F4; continue 'dispatch;
	}
	// 828859DC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 828859E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 828859E4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 828859E8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 828859EC: 4BFFDB85  bl 0x82883570
	ctx.lr = 0x828859F0;
	sub_82883570(ctx, base);
	// 828859F0: 48000010  b 0x82885a00
	pc = 0x82885A00; continue 'dispatch;
	// 828859F4: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 828859F8: 396BA8D8  addi r11, r11, -0x5728
	ctx.r[11].s64 = ctx.r[11].s64 + -22312;
	// 828859FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82885A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82885A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82885A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885A10 size=72
    let mut pc: u32 = 0x82885A10;
    'dispatch: loop {
        match pc {
            0x82885A10 => {
    //   block [0x82885A10..0x82885A58)
	// 82885A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885A18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885A1C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82885A20: 419A001C  beq cr6, 0x82885a3c
	if ctx.cr[6].eq {
	pc = 0x82885A3C; continue 'dispatch;
	}
	// 82885A24: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82885A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82885A2C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82885A30: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82885A34: 4BFFDBC5  bl 0x828835f8
	ctx.lr = 0x82885A38;
	sub_828835F8(ctx, base);
	// 82885A38: 48000010  b 0x82885a48
	pc = 0x82885A48; continue 'dispatch;
	// 82885A3C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82885A40: 396BA9D0  addi r11, r11, -0x5630
	ctx.r[11].s64 = ctx.r[11].s64 + -22064;
	// 82885A44: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82885A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82885A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82885A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885A58 size=72
    let mut pc: u32 = 0x82885A58;
    'dispatch: loop {
        match pc {
            0x82885A58 => {
    //   block [0x82885A58..0x82885AA0)
	// 82885A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885A64: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82885A68: 419A001C  beq cr6, 0x82885a84
	if ctx.cr[6].eq {
	pc = 0x82885A84; continue 'dispatch;
	}
	// 82885A6C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82885A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82885A74: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82885A78: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82885A7C: 4BFFDC05  bl 0x82883680
	ctx.lr = 0x82885A80;
	sub_82883680(ctx, base);
	// 82885A80: 48000010  b 0x82885a90
	pc = 0x82885A90; continue 'dispatch;
	// 82885A84: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82885A88: 396BAA90  addi r11, r11, -0x5570
	ctx.r[11].s64 = ctx.r[11].s64 + -21872;
	// 82885A8C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82885A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82885A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82885A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82885AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82885AA0 size=72
    let mut pc: u32 = 0x82885AA0;
    'dispatch: loop {
        match pc {
            0x82885AA0 => {
    //   block [0x82885AA0..0x82885AE8)
	// 82885AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82885AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82885AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82885AAC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82885AB0: 419A001C  beq cr6, 0x82885acc
	if ctx.cr[6].eq {
	pc = 0x82885ACC; continue 'dispatch;
	}
	// 82885AB4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82885AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82885ABC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82885AC0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82885AC4: 4BFFDC45  bl 0x82883708
	ctx.lr = 0x82885AC8;
	sub_82883708(ctx, base);
	// 82885AC8: 48000010  b 0x82885ad8
	pc = 0x82885AD8; continue 'dispatch;
	// 82885ACC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 82885AD0: 396BAB50  addi r11, r11, -0x54b0
	ctx.r[11].s64 = ctx.r[11].s64 + -21680;
	// 82885AD4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82885AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82885ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82885AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82885AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


