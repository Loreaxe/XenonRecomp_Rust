pub fn sub_827F5058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F5058 size=8
    let mut pc: u32 = 0x827F5058;
    'dispatch: loop {
        match pc {
            0x827F5058 => {
    //   block [0x827F5058..0x827F5060)
	// 827F5058: FF006800  fcmpu cr6, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 827F505C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5060 size=96
    let mut pc: u32 = 0x827F5060;
    'dispatch: loop {
        match pc {
            0x827F5060 => {
    //   block [0x827F5060..0x827F50C0)
	// 827F5060: 8144005C  lwz r10, 0x5c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F5064: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F50C0 size=48
    let mut pc: u32 = 0x827F50C0;
    'dispatch: loop {
        match pc {
            0x827F50C0 => {
    //   block [0x827F50C0..0x827F50F0)
	// 827F50C0: 3900FFE0  li r8, -0x20
	ctx.r[8].s64 = -32;
	// 827F50C4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F50F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F50F0 size=4
    let mut pc: u32 = 0x827F50F0;
    'dispatch: loop {
        match pc {
            0x827F50F0 => {
    //   block [0x827F50F0..0x827F50F4)
	// 827F50F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F50F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F50F8 size=532
    let mut pc: u32 = 0x827F50F8;
    'dispatch: loop {
        match pc {
            0x827F50F8 => {
    //   block [0x827F50F8..0x827F530C)
	// 827F50F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F50FC: 489B3069  bl 0x831a8164
	ctx.lr = 0x827F5100;
	sub_831A8130(ctx, base);
	// 827F5100: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 827F5104: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 827F5108: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F510C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5110: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 827F5114: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F5118: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F511C: 4BFF8EC5  bl 0x827edfe0
	ctx.lr = 0x827F5120;
	sub_827EDFE0(ctx, base);
	// 827F5120: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F5124: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F5128: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F512C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F5130: 4E800421  bctrl
	ctx.lr = 0x827F5134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F5134: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F5138: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827F513C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F5140: 409A0074  bne cr6, 0x827f51b4
	if !ctx.cr[6].eq {
	pc = 0x827F51B4; continue 'dispatch;
	}
	// 827F5144: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F5148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F514C: 4BFFF7FD  bl 0x827f4948
	ctx.lr = 0x827F5150;
	sub_827F4948(ctx, base);
	// 827F5150: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F5158: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F515C: 388B638C  addi r4, r11, 0x638c
	ctx.r[4].s64 = ctx.r[11].s64 + 25484;
	// 827F5160: 38A00063  li r5, 0x63
	ctx.r[5].s64 = 99;
	// 827F5164: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 827F5168: 4BACB271  bl 0x822c03d8
	ctx.lr = 0x827F516C;
	sub_822C03D8(ctx, base);
	// 827F516C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827F5170: 41820020  beq 0x827f5190
	if ctx.cr[0].eq {
	pc = 0x827F5190; continue 'dispatch;
	}
	// 827F5174: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F5178: 4BFF86A1  bl 0x827ed818
	ctx.lr = 0x827F517C;
	sub_827ED818(ctx, base);
	// 827F517C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5184: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F5188: 4BFFE229  bl 0x827f33b0
	ctx.lr = 0x827F518C;
	sub_827F33B0(ctx, base);
	// 827F518C: 48000008  b 0x827f5194
	pc = 0x827F5194; continue 'dispatch;
	// 827F5190: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F5194: 83DF0040  lwz r30, 0x40(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F5198: 907F0040  stw r3, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 827F519C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F51A0: 419A0014  beq cr6, 0x827f51b4
	if ctx.cr[6].eq {
	pc = 0x827F51B4; continue 'dispatch;
	}
	// 827F51A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F51A8: 4BFFDED9  bl 0x827f3080
	ctx.lr = 0x827F51AC;
	sub_827F3080(ctx, base);
	// 827F51AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F51B0: 4BACB0B9  bl 0x822c0268
	ctx.lr = 0x827F51B4;
	sub_822C0268(ctx, base);
	// 827F51B4: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F51B8: 4BFFDEC1  bl 0x827f3078
	ctx.lr = 0x827F51BC;
	sub_827F3078(ctx, base);
	// 827F51BC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F51C0: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827F51C4: 3B8B6910  addi r28, r11, 0x6910
	ctx.r[28].s64 = ctx.r[11].s64 + 26896;
	// 827F51C8: 40820070  bne 0x827f5238
	if !ctx.cr[0].eq {
	pc = 0x827F5238; continue 'dispatch;
	}
	// 827F51CC: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F51D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F51D4: 419A0064  beq cr6, 0x827f5238
	if ctx.cr[6].eq {
	pc = 0x827F5238; continue 'dispatch;
	}
	// 827F51D8: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 827F51DC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F51E0: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F51E4: 41820054  beq 0x827f5238
	if ctx.cr[0].eq {
	pc = 0x827F5238; continue 'dispatch;
	}
	// 827F51E8: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 827F51EC: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F51F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827F51F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F51F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5310 size=80
    let mut pc: u32 = 0x827F5310;
    'dispatch: loop {
        match pc {
            0x827F5310 => {
    //   block [0x827F5310..0x827F5360)
	// 827F5310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F531C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5324: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 827F5328: 4BFFD431  bl 0x827f2758
	ctx.lr = 0x827F532C;
	sub_827F2758(ctx, base);
	// 827F532C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F5330: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F5334: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827F5338: 39200070  li r9, 0x70
	ctx.r[9].s64 = 112;
	// 827F533C: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5340: D01F0080  stfs f0, 0x80(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 827F5344: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5360 size=112
    let mut pc: u32 = 0x827F5360;
    'dispatch: loop {
        match pc {
            0x827F5360 => {
    //   block [0x827F5360..0x827F53D0)
	// 827F5360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F536C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5378: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F537C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 827F5380: 396B63D4  addi r11, r11, 0x63d4
	ctx.r[11].s64 = ctx.r[11].s64 + 25556;
	// 827F5384: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5388: 4BC74C79  bl 0x8246a000
	ctx.lr = 0x827F538C;
	sub_8246A000(ctx, base);
	// 827F538C: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 827F5390: 4BDCAD11  bl 0x825c00a0
	ctx.lr = 0x827F5394;
	sub_825C00A0(ctx, base);
	// 827F5394: 83DF0040  lwz r30, 0x40(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F5398: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F539C: 419A0014  beq cr6, 0x827f53b0
	if ctx.cr[6].eq {
	pc = 0x827F53B0; continue 'dispatch;
	}
	// 827F53A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F53A4: 4BFFDCDD  bl 0x827f3080
	ctx.lr = 0x827F53A8;
	sub_827F3080(ctx, base);
	// 827F53A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F53AC: 4BACAEBD  bl 0x822c0268
	ctx.lr = 0x827F53B0;
	sub_822C0268(ctx, base);
	// 827F53B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F53B4: 4BFFF125  bl 0x827f44d8
	ctx.lr = 0x827F53B8;
	sub_827F44D8(ctx, base);
	// 827F53B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F53BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F53C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F53C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F53C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F53CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F53D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F53D0 size=8
    let mut pc: u32 = 0x827F53D0;
    'dispatch: loop {
        match pc {
            0x827F53D0 => {
    //   block [0x827F53D0..0x827F53D8)
	// 827F53D0: 38630070  addi r3, r3, 0x70
	ctx.r[3].s64 = ctx.r[3].s64 + 112;
	// 827F53D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F53D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F53D8 size=76
    let mut pc: u32 = 0x827F53D8;
    'dispatch: loop {
        match pc {
            0x827F53D8 => {
    //   block [0x827F53D8..0x827F5424)
	// 827F53D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F53DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F53E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F53E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F53E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F53EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F53F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F53F4: 4BFFFF6D  bl 0x827f5360
	ctx.lr = 0x827F53F8;
	sub_827F5360(ctx, base);
	// 827F53F8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F53FC: 4182000C  beq 0x827f5408
	if ctx.cr[0].eq {
	pc = 0x827F5408; continue 'dispatch;
	}
	// 827F5400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5404: 485FCFD5  bl 0x82df23d8
	ctx.lr = 0x827F5408;
	sub_82DF23D8(ctx, base);
	// 827F5408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F540C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5418: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F541C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5428 size=72
    let mut pc: u32 = 0x827F5428;
    'dispatch: loop {
        match pc {
            0x827F5428 => {
    //   block [0x827F5428..0x827F5470)
	// 827F5428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F542C: 489B2D41  bl 0x831a816c
	ctx.lr = 0x827F5430;
	sub_831A8130(ctx, base);
	// 827F5430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5434: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F5438: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F543C: 3BFE0058  addi r31, r30, 0x58
	ctx.r[31].s64 = ctx.r[30].s64 + 88;
	// 827F5440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5444: 4BFFD315  bl 0x827f2758
	ctx.lr = 0x827F5448;
	sub_827F2758(ctx, base);
	// 827F5448: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F544C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5450: 4BFFD791  bl 0x827f2be0
	ctx.lr = 0x827F5454;
	sub_827F2BE0(ctx, base);
	// 827F5454: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F5458: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 827F545C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827F5460: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5470 size=136
    let mut pc: u32 = 0x827F5470;
    'dispatch: loop {
        match pc {
            0x827F5470 => {
    //   block [0x827F5470..0x827F54F8)
	// 827F5470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F547C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5484: 4BFFF10D  bl 0x827f4590
	ctx.lr = 0x827F5488;
	sub_827F4590(ctx, base);
	// 827F5488: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F548C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 827F5490: 392B63D4  addi r9, r11, 0x63d4
	ctx.r[9].s64 = ctx.r[11].s64 + 25556;
	// 827F5494: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F5498: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F549C: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 827F54A0: 39200070  li r9, 0x70
	ctx.r[9].s64 = 112;
	// 827F54A4: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 827F54A8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827F54AC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 827F54B0: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 827F54B4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 827F54B8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F54BC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F54C0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827F54C4: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 827F54C8: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F54CC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827F54D0: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F54F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F54F8 size=124
    let mut pc: u32 = 0x827F54F8;
    'dispatch: loop {
        match pc {
            0x827F54F8 => {
    //   block [0x827F54F8..0x827F5574)
	// 827F54F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F54FC: 489B2C71  bl 0x831a816c
	ctx.lr = 0x827F5500;
	sub_831A8130(ctx, base);
	// 827F5500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F550C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F5510: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 827F5514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5518: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F551C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 827F5520: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F5524: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827F5528: 4BFFD0A1  bl 0x827f25c8
	ctx.lr = 0x827F552C;
	sub_827F25C8(ctx, base);
	// 827F552C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5530: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F5534: 4BFFD15D  bl 0x827f2690
	ctx.lr = 0x827F5538;
	sub_827F2690(ctx, base);
	// 827F5538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F553C: 4BFFCEDD  bl 0x827f2418
	ctx.lr = 0x827F5540;
	sub_827F2418(ctx, base);
	// 827F5540: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F5544: 41820024  beq 0x827f5568
	if ctx.cr[0].eq {
	pc = 0x827F5568; continue 'dispatch;
	}
	// 827F5548: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F554C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5550: 4BFFDA49  bl 0x827f2f98
	ctx.lr = 0x827F5554;
	sub_827F2F98(ctx, base);
	// 827F5554: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F555C: 4BFFD685  bl 0x827f2be0
	ctx.lr = 0x827F5560;
	sub_827F2BE0(ctx, base);
	// 827F5560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5564: 4BC74A9D  bl 0x8246a000
	ctx.lr = 0x827F5568;
	sub_8246A000(ctx, base);
	// 827F5568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F556C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F5570: 489B2C4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5578 size=64
    let mut pc: u32 = 0x827F5578;
    'dispatch: loop {
        match pc {
            0x827F5578 => {
    //   block [0x827F5578..0x827F55B8)
	// 827F5578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F557C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5584: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F558C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F5590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5594: 48675F6D  bl 0x82e6b500
	ctx.lr = 0x827F5598;
	sub_82E6B500(ctx, base);
	// 827F5598: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F559C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 827F55A0: 4BFFD8B9  bl 0x827f2e58
	ctx.lr = 0x827F55A4;
	sub_827F2E58(ctx, base);
	// 827F55A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F55A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F55AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F55B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F55B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F55B8 size=192
    let mut pc: u32 = 0x827F55B8;
    'dispatch: loop {
        match pc {
            0x827F55B8 => {
    //   block [0x827F55B8..0x827F5678)
	// 827F55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F55BC: 489B2BA9  bl 0x831a8164
	ctx.lr = 0x827F55C0;
	sub_831A8130(ctx, base);
	// 827F55C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F55C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F55C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F55CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F55D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827F55D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F55D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F55DC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 827F55E0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F55E4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827F55E8: 4BFFA1B1  bl 0x827ef798
	ctx.lr = 0x827F55EC;
	sub_827EF798(ctx, base);
	// 827F55EC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827F55F0: 41820044  beq 0x827f5634
	if ctx.cr[0].eq {
	pc = 0x827F5634; continue 'dispatch;
	}
	// 827F55F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F55F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F55FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F5600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F5604: 4E800421  bctrl
	ctx.lr = 0x827F5608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F5608: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827F560C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827F5610: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F5614: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5618: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 827F561C: 4BFFFEDD  bl 0x827f54f8
	ctx.lr = 0x827F5620;
	sub_827F54F8(ctx, base);
	// 827F5620: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5628: 4BFFD5B9  bl 0x827f2be0
	ctx.lr = 0x827F562C;
	sub_827F2BE0(ctx, base);
	// 827F562C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5630: 4BC749D1  bl 0x8246a000
	ctx.lr = 0x827F5634;
	sub_8246A000(ctx, base);
	// 827F5634: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F5638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F563C: 419A0014  beq cr6, 0x827f5650
	if ctx.cr[6].eq {
	pc = 0x827F5650; continue 'dispatch;
	}
	// 827F5640: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F5644: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F5648: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F564C: 40820020  bne 0x827f566c
	if !ctx.cr[0].eq {
	pc = 0x827F566C; continue 'dispatch;
	}
	// 827F5650: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F5654: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827F5658: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F565C: 48675EA5  bl 0x82e6b500
	ctx.lr = 0x827F5660;
	sub_82E6B500(ctx, base);
	// 827F5660: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5668: 4BFFD7F1  bl 0x827f2e58
	ctx.lr = 0x827F566C;
	sub_827F2E58(ctx, base);
	// 827F566C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5670: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F5674: 489B2B40  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5678 size=80
    let mut pc: u32 = 0x827F5678;
    'dispatch: loop {
        match pc {
            0x827F5678 => {
    //   block [0x827F5678..0x827F56C8)
	// 827F5678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F567C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F568C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5690: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F5694: 396B6444  addi r11, r11, 0x6444
	ctx.r[11].s64 = ctx.r[11].s64 + 25668;
	// 827F5698: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F569C: 4BFFAE65  bl 0x827f0500
	ctx.lr = 0x827F56A0;
	sub_827F0500(ctx, base);
	// 827F56A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F56A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F56A8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F56AC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F56B0: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F56B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F56B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F56BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F56C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F56C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F56C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F56C8 size=16
    let mut pc: u32 = 0x827F56C8;
    'dispatch: loop {
        match pc {
            0x827F56C8 => {
    //   block [0x827F56C8..0x827F56D8)
	// 827F56C8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F56CC: 396B6444  addi r11, r11, 0x6444
	ctx.r[11].s64 = ctx.r[11].s64 + 25668;
	// 827F56D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F56D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F56D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F56D8 size=20
    let mut pc: u32 = 0x827F56D8;
    'dispatch: loop {
        match pc {
            0x827F56D8 => {
    //   block [0x827F56D8..0x827F56EC)
	// 827F56D8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 827F56DC: C00BE830  lfs f0, -0x17d0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F56E0: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 827F56E4: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827F56E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F56F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F56F0 size=8
    let mut pc: u32 = 0x827F56F0;
    'dispatch: loop {
        match pc {
            0x827F56F0 => {
    //   block [0x827F56F0..0x827F56F8)
	// 827F56F0: D0230020  stfs f1, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F56F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F56F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F56F8 size=24
    let mut pc: u32 = 0x827F56F8;
    'dispatch: loop {
        match pc {
            0x827F56F8 => {
    //   block [0x827F56F8..0x827F5710)
	// 827F56F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F56FC: C1A30020  lfs f13, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F5700: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F5704: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5708: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827F570C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F5710 size=8
    let mut pc: u32 = 0x827F5710;
    'dispatch: loop {
        match pc {
            0x827F5710 => {
    //   block [0x827F5710..0x827F5718)
	// 827F5710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F5714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5718 size=16
    let mut pc: u32 = 0x827F5718;
    'dispatch: loop {
        match pc {
            0x827F5718 => {
    //   block [0x827F5718..0x827F5728)
	// 827F5718: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F571C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5720: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F5724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5728 size=8
    let mut pc: u32 = 0x827F5728;
    'dispatch: loop {
        match pc {
            0x827F5728 => {
    //   block [0x827F5728..0x827F5730)
	// 827F5728: C0230024  lfs f1, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F572C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5730 size=8
    let mut pc: u32 = 0x827F5730;
    'dispatch: loop {
        match pc {
            0x827F5730 => {
    //   block [0x827F5730..0x827F5738)
	// 827F5730: D0230024  stfs f1, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F5734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5738 size=76
    let mut pc: u32 = 0x827F5738;
    'dispatch: loop {
        match pc {
            0x827F5738 => {
    //   block [0x827F5738..0x827F5784)
	// 827F5738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5748: C0440008  lfs f2, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 827F574C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5750: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F5754: 489B5865  bl 0x831aafb8
	ctx.lr = 0x827F5758;
	sub_831AAFB8(ctx, base);
	// 827F5758: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F575C: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827F5760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5764: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F5768: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F576C: 4E800421  bctrl
	ctx.lr = 0x827F5770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F5770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F5774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F577C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5788 size=92
    let mut pc: u32 = 0x827F5788;
    'dispatch: loop {
        match pc {
            0x827F5788 => {
    //   block [0x827F5788..0x827F57E4)
	// 827F5788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F578C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5794: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F579C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F57A0: 486866F9  bl 0x82e7be98
	ctx.lr = 0x827F57A4;
	sub_82E7BE98(ctx, base);
	// 827F57A4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F57A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F57AC: 38ABCA90  addi r5, r11, -0x3570
	ctx.r[5].s64 = ctx.r[11].s64 + -13680;
	// 827F57B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F57B4: 48686515  bl 0x82e7bcc8
	ctx.lr = 0x827F57B8;
	sub_82E7BCC8(ctx, base);
	// 827F57B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F57BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F57C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F57C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F57C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F57CC: 4E800421  bctrl
	ctx.lr = 0x827F57D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F57D0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F57D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F57D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F57DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F57E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F57E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F57E8 size=436
    let mut pc: u32 = 0x827F57E8;
    'dispatch: loop {
        match pc {
            0x827F57E8 => {
    //   block [0x827F57E8..0x827F599C)
	// 827F57E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F57EC: 489B2981  bl 0x831a816c
	ctx.lr = 0x827F57F0;
	sub_831A8130(ctx, base);
	// 827F57F0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827F57F4: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F57F8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F57FC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F5800: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F5804: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F5808: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F580C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F5810: 48686689  bl 0x82e7be98
	ctx.lr = 0x827F5814;
	sub_82E7BE98(ctx, base);
	// 827F5814: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F5818: C11F0020  lfs f8, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 827F581C: C0DF0024  lfs f6, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 827F5820: FD604090  fmr f11, f8
	ctx.f[11].f64 = ctx.f[8].f64;
	// 827F5824: C0BF0010  lfs f5, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 827F5828: FDA03090  fmr f13, f6
	ctx.f[13].f64 = ctx.f[6].f64;
	// 827F582C: C15F0014  lfs f10, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 827F5830: C12B08A4  lfs f9, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 827F5834: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5838: FF084800  fcmpu cr6, f8, f9
	ctx.cr[6].compare_f64(ctx.f[8].f64, ctx.f[9].f64);
	// 827F583C: 40980014  bge cr6, 0x827f5850
	if !ctx.cr[6].lt {
	pc = 0x827F5850; continue 'dispatch;
	}
	// 827F5840: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F5844: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F5848: FD405050  fneg f10, f10
	ctx.f[10].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F584C: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F5850: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5854: C18B630C  lfs f12, 0x630c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F5858: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 827F585C: 40990028  ble cr6, 0x827f5884
	if !ctx.cr[6].gt {
	pc = 0x827F5884; continue 'dispatch;
	}
	// 827F5860: ED8A07F2  fmuls f12, f10, f31
	ctx.f[12].f64 = (((ctx.f[10].f64 * ctx.f[31].f64) as f32) as f64);
	// 827F5864: FCE06A10  fabs f7, f13
	ctx.f[7].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 827F5868: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 827F586C: FF076000  fcmpu cr6, f7, f12
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[12].f64);
	// 827F5870: 41990014  bgt cr6, 0x827f5884
	if ctx.cr[6].gt {
	pc = 0x827F5884; continue 'dispatch;
	}
	// 827F5874: D13F0020  stfs f9, 0x20(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F5878: FFE04090  fmr f31, f8
	ctx.f[31].f64 = ctx.f[8].f64;
	// 827F587C: D13F0024  stfs f9, 0x24(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F5880: 48000054  b 0x827f58d4
	pc = 0x827F58D4; continue 'dispatch;
	// 827F5884: ED860024  fdivs f12, f6, f0
	ctx.f[12].f64 = ((ctx.f[6].f64 / ctx.f[0].f64) as f32) as f64;
	// 827F5888: FF0C4800  fcmpu cr6, f12, f9
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[9].f64);
	// 827F588C: 40990034  ble cr6, 0x827f58c0
	if !ctx.cr[6].gt {
	pc = 0x827F58C0; continue 'dispatch;
	}
	// 827F5890: EC8C0032  fmuls f4, f12, f0
	ctx.f[4].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 827F5894: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F5898: C0EB9450  lfs f7, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 827F589C: ECE469FA  fmadds f7, f4, f7, f13
	ctx.f[7].f64 = (((ctx.f[4].f64 * ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64);
	// 827F58A0: ED870332  fmuls f12, f7, f12
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 827F58A4: FF0B6000  fcmpu cr6, f11, f12
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[12].f64);
	// 827F58A8: 4098000C  bge cr6, 0x827f58b4
	if !ctx.cr[6].lt {
	pc = 0x827F58B4; continue 'dispatch;
	}
	// 827F58AC: FD400050  fneg f10, f0
	ctx.f[10].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F58B0: 48000010  b 0x827f58c0
	pc = 0x827F58C0; continue 'dispatch;
	// 827F58B4: FF056800  fcmpu cr6, f5, f13
	ctx.cr[6].compare_f64(ctx.f[5].f64, ctx.f[13].f64);
	// 827F58B8: 40980008  bge cr6, 0x827f58c0
	if !ctx.cr[6].lt {
	pc = 0x827F58C0; continue 'dispatch;
	}
	// 827F58BC: FD404890  fmr f10, f9
	ctx.f[10].f64 = ctx.f[9].f64;
	// 827F58C0: EC0A37FA  fmadds f0, f10, f31, f6
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[31].f64 + ctx.f[6].f64) as f32) as f64);
	// 827F58C4: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F58C8: EFE007F2  fmuls f31, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 827F58CC: EC08F828  fsubs f0, f8, f31
	ctx.f[0].f64 = (((ctx.f[8].f64 - ctx.f[31].f64) as f32) as f64);
	// 827F58D0: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F58D4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F58D8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827F58DC: 38ABCA80  addi r5, r11, -0x3580
	ctx.r[5].s64 = ctx.r[11].s64 + -13696;
	// 827F58E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F58E4: 486863E5  bl 0x82e7bcc8
	ctx.lr = 0x827F58E8;
	sub_82E7BCC8(ctx, base);
	// 827F58E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F58EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F58F0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F58F4: 4868734D  bl 0x82e7cc40
	ctx.lr = 0x827F58F8;
	sub_82E7CC40(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F59A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F59A0 size=152
    let mut pc: u32 = 0x827F59A0;
    'dispatch: loop {
        match pc {
            0x827F59A0 => {
    //   block [0x827F59A0..0x827F5A38)
	// 827F59A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F59A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F59A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F59AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F59B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F59B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F59B8: 4895EC49  bl 0x83154600
	ctx.lr = 0x827F59BC;
	sub_83154600(ctx, base);
	// 827F59BC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F59C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F59C4: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F59C8: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 827F59CC: 388B5F2C  addi r4, r11, 0x5f2c
	ctx.r[4].s64 = ctx.r[11].s64 + 24364;
	// 827F59D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F59D4: 3BEA5F24  addi r31, r10, 0x5f24
	ctx.r[31].s64 = ctx.r[10].s64 + 24356;
	// 827F59D8: 485FE2C9  bl 0x82df3ca0
	ctx.lr = 0x827F59DC;
	sub_82DF3CA0(ctx, base);
	// 827F59DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F59E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F59E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F59E8: 485FE311  bl 0x82df3cf8
	ctx.lr = 0x827F59EC;
	sub_82DF3CF8(ctx, base);
	// 827F59EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F59F0: 485FDA39  bl 0x82df3428
	ctx.lr = 0x827F59F4;
	sub_82DF3428(ctx, base);
	// 827F59F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F59F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F59FC: 4BFF8025  bl 0x827eda20
	ctx.lr = 0x827F5A00;
	sub_827EDA20(ctx, base);
	// 827F5A00: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 827F5A04: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F5A08: 409A0010  bne cr6, 0x827f5a18
	if !ctx.cr[6].eq {
	pc = 0x827F5A18; continue 'dispatch;
	}
	// 827F5A0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F5A10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5A14: 4BFF7FD5  bl 0x827ed9e8
	ctx.lr = 0x827F5A18;
	sub_827ED9E8(ctx, base);
	// 827F5A18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5A1C: 485FDA0D  bl 0x82df3428
	ctx.lr = 0x827F5A20;
	sub_82DF3428(ctx, base);
	// 827F5A20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5A2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F5A30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5A38 size=152
    let mut pc: u32 = 0x827F5A38;
    'dispatch: loop {
        match pc {
            0x827F5A38 => {
    //   block [0x827F5A38..0x827F5AD0)
	// 827F5A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F5A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5A50: 4895EBB1  bl 0x83154600
	ctx.lr = 0x827F5A54;
	sub_83154600(ctx, base);
	// 827F5A54: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5A58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F5A5C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F5A60: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 827F5A64: 388B5F2C  addi r4, r11, 0x5f2c
	ctx.r[4].s64 = ctx.r[11].s64 + 24364;
	// 827F5A68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F5A6C: 3BEA5D94  addi r31, r10, 0x5d94
	ctx.r[31].s64 = ctx.r[10].s64 + 23956;
	// 827F5A70: 485FE231  bl 0x82df3ca0
	ctx.lr = 0x827F5A74;
	sub_82DF3CA0(ctx, base);
	// 827F5A74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5A78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5A7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F5A80: 485FE279  bl 0x82df3cf8
	ctx.lr = 0x827F5A84;
	sub_82DF3CF8(ctx, base);
	// 827F5A84: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F5A88: 485FD9A1  bl 0x82df3428
	ctx.lr = 0x827F5A8C;
	sub_82DF3428(ctx, base);
	// 827F5A8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F5A90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5A94: 4BFF7F8D  bl 0x827eda20
	ctx.lr = 0x827F5A98;
	sub_827EDA20(ctx, base);
	// 827F5A98: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 827F5A9C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F5AA0: 409A0010  bne cr6, 0x827f5ab0
	if !ctx.cr[6].eq {
	pc = 0x827F5AB0; continue 'dispatch;
	}
	// 827F5AA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F5AA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5AAC: 4BFF7F3D  bl 0x827ed9e8
	ctx.lr = 0x827F5AB0;
	sub_827ED9E8(ctx, base);
	// 827F5AB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5AB4: 485FD975  bl 0x82df3428
	ctx.lr = 0x827F5AB8;
	sub_82DF3428(ctx, base);
	// 827F5AB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5AC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F5AC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5AD0 size=152
    let mut pc: u32 = 0x827F5AD0;
    'dispatch: loop {
        match pc {
            0x827F5AD0 => {
    //   block [0x827F5AD0..0x827F5B68)
	// 827F5AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F5ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5AE8: 4895EB19  bl 0x83154600
	ctx.lr = 0x827F5AEC;
	sub_83154600(ctx, base);
	// 827F5AEC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5AF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F5AF4: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F5AF8: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 827F5AFC: 388B5F2C  addi r4, r11, 0x5f2c
	ctx.r[4].s64 = ctx.r[11].s64 + 24364;
	// 827F5B00: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F5B04: 3BEA5F1C  addi r31, r10, 0x5f1c
	ctx.r[31].s64 = ctx.r[10].s64 + 24348;
	// 827F5B08: 485FE199  bl 0x82df3ca0
	ctx.lr = 0x827F5B0C;
	sub_82DF3CA0(ctx, base);
	// 827F5B0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5B10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5B14: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F5B18: 485FE1E1  bl 0x82df3cf8
	ctx.lr = 0x827F5B1C;
	sub_82DF3CF8(ctx, base);
	// 827F5B1C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F5B20: 485FD909  bl 0x82df3428
	ctx.lr = 0x827F5B24;
	sub_82DF3428(ctx, base);
	// 827F5B24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F5B28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5B2C: 4BFF7EF5  bl 0x827eda20
	ctx.lr = 0x827F5B30;
	sub_827EDA20(ctx, base);
	// 827F5B30: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 827F5B34: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F5B38: 409A0010  bne cr6, 0x827f5b48
	if !ctx.cr[6].eq {
	pc = 0x827F5B48; continue 'dispatch;
	}
	// 827F5B3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F5B40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5B44: 4BFF7EA5  bl 0x827ed9e8
	ctx.lr = 0x827F5B48;
	sub_827ED9E8(ctx, base);
	// 827F5B48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5B4C: 485FD8DD  bl 0x82df3428
	ctx.lr = 0x827F5B50;
	sub_82DF3428(ctx, base);
	// 827F5B50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5B5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F5B60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5B68 size=100
    let mut pc: u32 = 0x827F5B68;
    'dispatch: loop {
        match pc {
            0x827F5B68 => {
    //   block [0x827F5B68..0x827F5BCC)
	// 827F5B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5B6C: 489B25FD  bl 0x831a8168
	ctx.lr = 0x827F5B70;
	sub_831A8130(ctx, base);
	// 827F5B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5B78: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F5B7C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F5B80: 48666DA9  bl 0x82e5c928
	ctx.lr = 0x827F5B84;
	sub_82E5C928(ctx, base);
	// 827F5B84: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5B88: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
	// 827F5B8C: 396B6464  addi r11, r11, 0x6464
	ctx.r[11].s64 = ctx.r[11].s64 + 25700;
	// 827F5B90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5B94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5B98: 485FD559  bl 0x82df30f0
	ctx.lr = 0x827F5B9C;
	sub_82DF30F0(ctx, base);
	// 827F5B9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5BA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F5BA4: 485FE02D  bl 0x82df3bd0
	ctx.lr = 0x827F5BA8;
	sub_82DF3BD0(ctx, base);
	// 827F5BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5BAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F5BB0: 48663B39  bl 0x82e596e8
	ctx.lr = 0x827F5BB4;
	sub_82E596E8(ctx, base);
	// 827F5BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5BB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F5BBC: 4885D595  bl 0x83053150
	ctx.lr = 0x827F5BC0;
	sub_83053150(ctx, base);
	// 827F5BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F5BC8: 489B25F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5BD0 size=88
    let mut pc: u32 = 0x827F5BD0;
    'dispatch: loop {
        match pc {
            0x827F5BD0 => {
    //   block [0x827F5BD0..0x827F5C28)
	// 827F5BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5BD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F5BDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5BE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5BE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F5BEC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 827F5BF0: 485FD839  bl 0x82df3428
	ctx.lr = 0x827F5BF4;
	sub_82DF3428(ctx, base);
	// 827F5BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5BF8: 48666991  bl 0x82e5c588
	ctx.lr = 0x827F5BFC;
	sub_82E5C588(ctx, base);
	// 827F5BFC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F5C00: 4182000C  beq 0x827f5c0c
	if ctx.cr[0].eq {
	pc = 0x827F5C0C; continue 'dispatch;
	}
	// 827F5C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5C08: 485FC7D1  bl 0x82df23d8
	ctx.lr = 0x827F5C0C;
	sub_82DF23D8(ctx, base);
	// 827F5C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5C10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5C1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F5C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5C28 size=104
    let mut pc: u32 = 0x827F5C28;
    'dispatch: loop {
        match pc {
            0x827F5C28 => {
    //   block [0x827F5C28..0x827F5C90)
	// 827F5C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5C2C: 489B2541  bl 0x831a816c
	ctx.lr = 0x827F5C30;
	sub_831A8130(ctx, base);
	// 827F5C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5C38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F5C3C: 48666CED  bl 0x82e5c928
	ctx.lr = 0x827F5C40;
	sub_82E5C928(ctx, base);
	// 827F5C40: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5C44: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
	// 827F5C48: 396B648C  addi r11, r11, 0x648c
	ctx.r[11].s64 = ctx.r[11].s64 + 25740;
	// 827F5C4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5C50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5C54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F5C58: 485FDFA9  bl 0x82df3c00
	ctx.lr = 0x827F5C5C;
	sub_82DF3C00(ctx, base);
	// 827F5C5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5C60: 485FD551  bl 0x82df31b0
	ctx.lr = 0x827F5C64;
	sub_82DF31B0(ctx, base);
	// 827F5C64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5C68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5C6C: 485FDD9D  bl 0x82df3a08
	ctx.lr = 0x827F5C70;
	sub_82DF3A08(ctx, base);
	// 827F5C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5C74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F5C78: 48663A71  bl 0x82e596e8
	ctx.lr = 0x827F5C7C;
	sub_82E596E8(ctx, base);
	// 827F5C7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5C80: 485FD7A9  bl 0x82df3428
	ctx.lr = 0x827F5C84;
	sub_82DF3428(ctx, base);
	// 827F5C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F5C8C: 489B2530  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5C90 size=100
    let mut pc: u32 = 0x827F5C90;
    'dispatch: loop {
        match pc {
            0x827F5C90 => {
    //   block [0x827F5C90..0x827F5CF4)
	// 827F5C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F5C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5CA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5CA8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5CAC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 827F5CB0: 396B648C  addi r11, r11, 0x648c
	ctx.r[11].s64 = ctx.r[11].s64 + 25740;
	// 827F5CB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F5CB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5CBC: 485FD76D  bl 0x82df3428
	ctx.lr = 0x827F5CC0;
	sub_82DF3428(ctx, base);
	// 827F5CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5CC4: 486668C5  bl 0x82e5c588
	ctx.lr = 0x827F5CC8;
	sub_82E5C588(ctx, base);
	// 827F5CC8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F5CCC: 4182000C  beq 0x827f5cd8
	if ctx.cr[0].eq {
	pc = 0x827F5CD8; continue 'dispatch;
	}
	// 827F5CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5CD4: 485FC705  bl 0x82df23d8
	ctx.lr = 0x827F5CD8;
	sub_82DF23D8(ctx, base);
	// 827F5CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5CDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5CE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F5CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5CF8 size=92
    let mut pc: u32 = 0x827F5CF8;
    'dispatch: loop {
        match pc {
            0x827F5CF8 => {
    //   block [0x827F5CF8..0x827F5D54)
	// 827F5CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5D00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5D04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5D08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5D0C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 827F5D10: 485FD4A1  bl 0x82df31b0
	ctx.lr = 0x827F5D14;
	sub_82DF31B0(ctx, base);
	// 827F5D14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F5D18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5D1C: 485FDCED  bl 0x82df3a08
	ctx.lr = 0x827F5D20;
	sub_82DF3A08(ctx, base);
	// 827F5D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5D24: 4895E8DD  bl 0x83154600
	ctx.lr = 0x827F5D28;
	sub_83154600(ctx, base);
	// 827F5D28: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 827F5D2C: 4BFF9A55  bl 0x827ef780
	ctx.lr = 0x827F5D30;
	sub_827EF780(ctx, base);
	// 827F5D30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F5D34: 4BFF47AD  bl 0x827ea4e0
	ctx.lr = 0x827F5D38;
	sub_827EA4E0(ctx, base);
	// 827F5D38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5D3C: 485FD6ED  bl 0x82df3428
	ctx.lr = 0x827F5D40;
	sub_82DF3428(ctx, base);
	// 827F5D40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5D58 size=80
    let mut pc: u32 = 0x827F5D58;
    'dispatch: loop {
        match pc {
            0x827F5D58 => {
    //   block [0x827F5D58..0x827F5DA8)
	// 827F5D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5D6C: 4895E895  bl 0x83154600
	ctx.lr = 0x827F5D70;
	sub_83154600(ctx, base);
	// 827F5D70: 4BFF9A11  bl 0x827ef780
	ctx.lr = 0x827F5D74;
	sub_827EF780(ctx, base);
	// 827F5D74: 4BFF459D  bl 0x827ea310
	ctx.lr = 0x827F5D78;
	sub_827EA310(ctx, base);
	// 827F5D78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F5D7C: 41820018  beq 0x827f5d94
	if ctx.cr[0].eq {
	pc = 0x827F5D94; continue 'dispatch;
	}
	// 827F5D80: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F5D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F5D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5D8C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F5D90: 48666CA9  bl 0x82e5ca38
	ctx.lr = 0x827F5D94;
	sub_82E5CA38(ctx, base);
	// 827F5D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F5D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5DA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5DA8 size=196
    let mut pc: u32 = 0x827F5DA8;
    'dispatch: loop {
        match pc {
            0x827F5DA8 => {
    //   block [0x827F5DA8..0x827F5E6C)
	// 827F5DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5DB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F5DB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5DB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5DBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F5DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F5DC4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F5DC8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F5DCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5DD0: 4BACAB69  bl 0x822c0938
	ctx.lr = 0x827F5DD4;
	sub_822C0938(ctx, base);
	// 827F5DD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F5DD8: 41820028  beq 0x827f5e00
	if ctx.cr[0].eq {
	pc = 0x827F5E00; continue 'dispatch;
	}
	// 827F5DDC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5DE0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F5DE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F5DE8: 392B64B4  addi r9, r11, 0x64b4
	ctx.r[9].s64 = ctx.r[11].s64 + 25780;
	// 827F5DEC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F5DF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F5DF4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F5DF8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F5DFC: 48000008  b 0x827f5e04
	pc = 0x827F5E04; continue 'dispatch;
	// 827F5E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F5E04: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F5E0C: 409A0044  bne cr6, 0x827f5e50
	if !ctx.cr[6].eq {
	pc = 0x827F5E50; continue 'dispatch;
	}
	// 827F5E10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F5E14: 419A001C  beq cr6, 0x827f5e30
	if ctx.cr[6].eq {
	pc = 0x827F5E30; continue 'dispatch;
	}
	// 827F5E18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F5E1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F5E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F5E24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F5E28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F5E2C: 4E800421  bctrl
	ctx.lr = 0x827F5E30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F5E30: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F5E34: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F5E38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F5E3C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F5E40: 816BCBB0  lwz r11, -0x3450(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13392 as u32) ) } as u64;
	// 827F5E44: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F5E48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F5E4C: 4BACA1B5  bl 0x822c0000
	ctx.lr = 0x827F5E50;
	sub_822C0000(ctx, base);
	// 827F5E50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5E54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F5E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F5E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F5E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F5E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F5E70 size=120
    let mut pc: u32 = 0x827F5E70;
    'dispatch: loop {
        match pc {
            0x827F5E70 => {
    //   block [0x827F5E70..0x827F5EE8)
	// 827F5E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5E74: 489B22F9  bl 0x831a816c
	ctx.lr = 0x827F5E78;
	sub_831A8130(ctx, base);
	// 827F5E78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5E7C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5E80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F5E84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F5E88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F5E8C: 388B64C8  addi r4, r11, 0x64c8
	ctx.r[4].s64 = ctx.r[11].s64 + 25800;
	// 827F5E90: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 827F5E94: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 827F5E98: 485FC551  bl 0x82df23e8
	ctx.lr = 0x827F5E9C;
	sub_82DF23E8(ctx, base);
	// 827F5E9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F5EA0: 41820014  beq 0x827f5eb4
	if ctx.cr[0].eq {
	pc = 0x827F5EB4; continue 'dispatch;
	}
	// 827F5EA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F5EA8: 4BFFFD81  bl 0x827f5c28
	ctx.lr = 0x827F5EAC;
	sub_827F5C28(ctx, base);
	// 827F5EAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5EB0: 48000008  b 0x827f5eb8
	pc = 0x827F5EB8; continue 'dispatch;
	// 827F5EB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F5EB8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F5EBC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827F5EC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F5EC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5EC8: 4BFFFEE1  bl 0x827f5da8
	ctx.lr = 0x827F5ECC;
	sub_827F5DA8(ctx, base);
	// 827F5ECC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F5ED0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F5ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F5ED8: 4BACA129  bl 0x822c0000
	ctx.lr = 0x827F5EDC;
	sub_822C0000(ctx, base);
	// 827F5EDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F5EE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F5EE4: 489B22D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5EE8 size=48
    let mut pc: u32 = 0x827F5EE8;
    'dispatch: loop {
        match pc {
            0x827F5EE8 => {
    //   block [0x827F5EE8..0x827F5F18)
	// 827F5EE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F5EEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F5EF0: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5EF4: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 827F5EF8: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5EFC: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F5F00: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827F5F04: 40980014  bge cr6, 0x827f5f18
	if !ctx.cr[6].lt {
		sub_827F5F18(ctx, base);
		return;
	}
	// 827F5F08: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5F0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F5F10: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F5F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F5F18 size=8
    let mut pc: u32 = 0x827F5F18;
    'dispatch: loop {
        match pc {
            0x827F5F18 => {
    //   block [0x827F5F18..0x827F5F20)
	// 827F5F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F5F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5F20 size=24
    let mut pc: u32 = 0x827F5F20;
    'dispatch: loop {
        match pc {
            0x827F5F20 => {
    //   block [0x827F5F20..0x827F5F38)
	// 827F5F20: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F5F24: D0230004  stfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F5F28: D0430008  stfs f2, 8(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F5F2C: 396B6514  addi r11, r11, 0x6514
	ctx.r[11].s64 = ctx.r[11].s64 + 25876;
	// 827F5F30: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F5F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5F38 size=40
    let mut pc: u32 = 0x827F5F38;
    'dispatch: loop {
        match pc {
            0x827F5F38 => {
    //   block [0x827F5F38..0x827F5F60)
	// 827F5F38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F5F3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F5F40: 39200064  li r9, 0x64
	ctx.r[9].s64 = 100;
	// 827F5F44: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 827F5F48: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F5F4C: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827F5F50: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F5F54: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 827F5F58: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827F5F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F5F60 size=216
    let mut pc: u32 = 0x827F5F60;
    'dispatch: loop {
        match pc {
            0x827F5F60 => {
    //   block [0x827F5F60..0x827F6038)
	// 827F5F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F5F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F5F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F5F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F5F70: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 827F5F74: 489B2AFD  bl 0x831a8a70
	ctx.lr = 0x827F5F78;
	sub_831A8A40(ctx, base);
	// 827F5F78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F5F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F5F80: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F5F84: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 827F5F88: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 827F5F8C: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 827F5F90: FF602890  fmr f27, f5
	ctx.f[27].f64 = ctx.f[5].f64;
	// 827F5F94: FF403090  fmr f26, f6
	ctx.f[26].f64 = ctx.f[6].f64;
	// 827F5F98: 4BFFB859  bl 0x827f17f0
	ctx.lr = 0x827F5F9C;
	sub_827F17F0(ctx, base);
	// 827F5F9C: D3FF0020  stfs f31, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F5FA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F5FA4: D3DF0024  stfs f30, 0x24(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F5FA8: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F5FAC: D3BF0028  stfs f29, 0x28(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 827F5FB0: 388B5D2C  addi r4, r11, 0x5d2c
	ctx.r[4].s64 = ctx.r[11].s64 + 23852;
	// 827F5FB4: D39F002C  stfs f28, 0x2c(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 827F5FB8: D37F0030  stfs f27, 0x30(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 827F5FBC: D35F0034  stfs f26, 0x34(r31)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827F5FC0: 485FDA49  bl 0x82df3a08
	ctx.lr = 0x827F5FC4;
	sub_82DF3A08(ctx, base);
	// 827F5FC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F5FC8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 827F5FCC: 3BCB9BC9  addi r30, r11, -0x6437
	ctx.r[30].s64 = ctx.r[11].s64 + -25655;
	// 827F5FD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F5FD4: 485FDA35  bl 0x82df3a08
	ctx.lr = 0x827F5FD8;
	sub_82DF3A08(ctx, base);
	// 827F5FD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F5FDC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 827F5FE0: 485FDA29  bl 0x82df3a08
	ctx.lr = 0x827F5FE4;
	sub_82DF3A08(ctx, base);
	// 827F5FE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F5FE8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 827F5FEC: 485FDA1D  bl 0x82df3a08
	ctx.lr = 0x827F5FF0;
	sub_82DF3A08(ctx, base);
	// 827F5FF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F5FF4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 827F5FF8: 485FDA11  bl 0x82df3a08
	ctx.lr = 0x827F5FFC;
	sub_82DF3A08(ctx, base);
	// 827F5FFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6000: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 827F6004: 485FDA05  bl 0x82df3a08
	ctx.lr = 0x827F6008;
	sub_82DF3A08(ctx, base);
	// 827F6008: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 827F600C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6010: 485FD9F9  bl 0x82df3a08
	ctx.lr = 0x827F6014;
	sub_82DF3A08(ctx, base);
	// 827F6014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6018: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F601C: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 827F6020: 489B2A9D  bl 0x831a8abc
	ctx.lr = 0x827F6024;
	sub_831A8A8C(ctx, base);
	// 827F6024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F602C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F6030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F6038 size=184
    let mut pc: u32 = 0x827F6038;
    'dispatch: loop {
        match pc {
            0x827F6038 => {
    //   block [0x827F6038..0x827F60F0)
	// 827F6038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F603C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F6044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F604C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F6050: 4BFFB7A1  bl 0x827f17f0
	ctx.lr = 0x827F6054;
	sub_827F17F0(ctx, base);
	// 827F6054: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F6058: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827F605C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F6060: 388A5D2C  addi r4, r10, 0x5d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 23852;
	// 827F6064: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F6068: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F606C: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F6070: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 827F6074: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 827F6078: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 827F607C: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827F6080: 485FD989  bl 0x82df3a08
	ctx.lr = 0x827F6084;
	sub_82DF3A08(ctx, base);
	// 827F6084: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F6088: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 827F608C: 3BCB9BC9  addi r30, r11, -0x6437
	ctx.r[30].s64 = ctx.r[11].s64 + -25655;
	// 827F6090: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6094: 485FD975  bl 0x82df3a08
	ctx.lr = 0x827F6098;
	sub_82DF3A08(ctx, base);
	// 827F6098: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F609C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 827F60A0: 485FD969  bl 0x82df3a08
	ctx.lr = 0x827F60A4;
	sub_82DF3A08(ctx, base);
	// 827F60A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F60A8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 827F60AC: 485FD95D  bl 0x82df3a08
	ctx.lr = 0x827F60B0;
	sub_82DF3A08(ctx, base);
	// 827F60B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F60B4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 827F60B8: 485FD951  bl 0x82df3a08
	ctx.lr = 0x827F60BC;
	sub_82DF3A08(ctx, base);
	// 827F60BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F60C0: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 827F60C4: 485FD945  bl 0x82df3a08
	ctx.lr = 0x827F60C8;
	sub_82DF3A08(ctx, base);
	// 827F60C8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 827F60CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F60D0: 485FD939  bl 0x82df3a08
	ctx.lr = 0x827F60D4;
	sub_82DF3A08(ctx, base);
	// 827F60D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F60D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F60DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F60E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F60E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F60E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F60EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F60F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F60F0 size=504
    let mut pc: u32 = 0x827F60F0;
    'dispatch: loop {
        match pc {
            0x827F60F0 => {
    //   block [0x827F60F0..0x827F62E8)
	// 827F60F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F60F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F60F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F60FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6100: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6104: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F6108: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F610C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6110: 419A01C0  beq cr6, 0x827f62d0
	if ctx.cr[6].eq {
	pc = 0x827F62D0; continue 'dispatch;
	}
	// 827F6114: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6118: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F611C: 388B6558  addi r4, r11, 0x6558
	ctx.r[4].s64 = ctx.r[11].s64 + 25944;
	// 827F6120: 485FD8E9  bl 0x82df3a08
	ctx.lr = 0x827F6124;
	sub_82DF3A08(ctx, base);
	// 827F6124: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F612C: 388B654C  addi r4, r11, 0x654c
	ctx.r[4].s64 = ctx.r[11].s64 + 25932;
	// 827F6130: 485FD8D9  bl 0x82df3a08
	ctx.lr = 0x827F6134;
	sub_82DF3A08(ctx, base);
	// 827F6134: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827F6138: 38E00064  li r7, 0x64
	ctx.r[7].s64 = 100;
	// 827F613C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 827F6140: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827F6144: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F6148: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F614C: 4BDAD2CD  bl 0x825a3418
	ctx.lr = 0x827F6150;
	sub_825A3418(ctx, base);
	// 827F6150: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F6154: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F6158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F615C: 4BDABB05  bl 0x825a1c60
	ctx.lr = 0x827F6160;
	sub_825A1C60(ctx, base);
	// 827F6160: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 827F6164: 485FD2C5  bl 0x82df3428
	ctx.lr = 0x827F6168;
	sub_82DF3428(ctx, base);
	// 827F6168: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827F616C: 4BAD2B4D  bl 0x822c8cb8
	ctx.lr = 0x827F6170;
	sub_822C8CB8(ctx, base);
	// 827F6170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6174: 485FD2B5  bl 0x82df3428
	ctx.lr = 0x827F6178;
	sub_82DF3428(ctx, base);
	// 827F6178: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F617C: 485FD2AD  bl 0x82df3428
	ctx.lr = 0x827F6180;
	sub_82DF3428(ctx, base);
	// 827F6180: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6184: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6188: 388B653C  addi r4, r11, 0x653c
	ctx.r[4].s64 = ctx.r[11].s64 + 25916;
	// 827F618C: 485FD87D  bl 0x82df3a08
	ctx.lr = 0x827F6190;
	sub_82DF3A08(ctx, base);
	// 827F6190: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6194: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F6198: 388B6534  addi r4, r11, 0x6534
	ctx.r[4].s64 = ctx.r[11].s64 + 25908;
	// 827F619C: 485FD86D  bl 0x82df3a08
	ctx.lr = 0x827F61A0;
	sub_82DF3A08(ctx, base);
	// 827F61A0: 39000064  li r8, 0x64
	ctx.r[8].s64 = 100;
	// 827F61A4: 38E02710  li r7, 0x2710
	ctx.r[7].s64 = 10000;
	// 827F61A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F61AC: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 827F61B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F61B4: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827F61B8: 4BDAD261  bl 0x825a3418
	ctx.lr = 0x827F61BC;
	sub_825A3418(ctx, base);
	// 827F61BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F61C0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F61C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F61C8: 4BDABD11  bl 0x825a1ed8
	ctx.lr = 0x827F61CC;
	sub_825A1ED8(ctx, base);
	// 827F61CC: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 827F61D0: 485FD259  bl 0x82df3428
	ctx.lr = 0x827F61D4;
	sub_82DF3428(ctx, base);
	// 827F61D4: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 827F61D8: 4BAD2AE1  bl 0x822c8cb8
	ctx.lr = 0x827F61DC;
	sub_822C8CB8(ctx, base);
	// 827F61DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F61E0: 485FD249  bl 0x82df3428
	ctx.lr = 0x827F61E4;
	sub_82DF3428(ctx, base);
	// 827F61E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F61E8: 485FD241  bl 0x82df3428
	ctx.lr = 0x827F61EC;
	sub_82DF3428(ctx, base);
	// 827F61EC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F61F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F61F4: 388B6524  addi r4, r11, 0x6524
	ctx.r[4].s64 = ctx.r[11].s64 + 25892;
	// 827F61F8: 485FD811  bl 0x82df3a08
	ctx.lr = 0x827F61FC;
	sub_82DF3A08(ctx, base);
	// 827F61FC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6200: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F6204: 388B6518  addi r4, r11, 0x6518
	ctx.r[4].s64 = ctx.r[11].s64 + 25880;
	// 827F6208: 485FD801  bl 0x82df3a08
	ctx.lr = 0x827F620C;
	sub_82DF3A08(ctx, base);
	// 827F620C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827F6210: 38E02710  li r7, 0x2710
	ctx.r[7].s64 = 10000;
	// 827F6214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F6218: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 827F621C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F6220: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827F6224: 4BDAD1F5  bl 0x825a3418
	ctx.lr = 0x827F6228;
	sub_825A3418(ctx, base);
	// 827F6228: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F622C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F6230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6234: 4BDABCA5  bl 0x825a1ed8
	ctx.lr = 0x827F6238;
	sub_825A1ED8(ctx, base);
	// 827F6238: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 827F623C: 485FD1ED  bl 0x82df3428
	ctx.lr = 0x827F6240;
	sub_82DF3428(ctx, base);
	// 827F6240: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827F6244: 4BAD2A75  bl 0x822c8cb8
	ctx.lr = 0x827F6248;
	sub_822C8CB8(ctx, base);
	// 827F6248: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F624C: 485FD1DD  bl 0x82df3428
	ctx.lr = 0x827F6250;
	sub_82DF3428(ctx, base);
	// 827F6250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6254: 485FD1D5  bl 0x82df3428
	ctx.lr = 0x827F6258;
	sub_82DF3428(ctx, base);
	// 827F6258: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F625C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6260: 388B0454  addi r4, r11, 0x454
	ctx.r[4].s64 = ctx.r[11].s64 + 1108;
	// 827F6264: 485FD7A5  bl 0x82df3a08
	ctx.lr = 0x827F6268;
	sub_82DF3A08(ctx, base);
	// 827F6268: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827F626C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F6270: 388BCCA4  addi r4, r11, -0x335c
	ctx.r[4].s64 = ctx.r[11].s64 + -13148;
	// 827F6274: 485FD795  bl 0x82df3a08
	ctx.lr = 0x827F6278;
	sub_82DF3A08(ctx, base);
	// 827F6278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F627C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 827F6280: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F6284: 38BE000C  addi r5, r30, 0xc
	ctx.r[5].s64 = ctx.r[30].s64 + 12;
	// 827F6288: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F628C: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827F6290: C06B964C  lfs f3, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 827F6294: C04A9A8C  lfs f2, -0x6574(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 827F6298: C02908A4  lfs f1, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F629C: 4BDAD00D  bl 0x825a32a8
	ctx.lr = 0x827F62A0;
	sub_825A32A8(ctx, base);
	// 827F62A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F62A4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F62A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F62AC: 4BDAB4C5  bl 0x825a1770
	ctx.lr = 0x827F62B0;
	sub_825A1770(ctx, base);
	// 827F62B0: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 827F62B4: 485FD175  bl 0x82df3428
	ctx.lr = 0x827F62B8;
	sub_82DF3428(ctx, base);
	// 827F62B8: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 827F62BC: 4BAD29FD  bl 0x822c8cb8
	ctx.lr = 0x827F62C0;
	sub_822C8CB8(ctx, base);
	// 827F62C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F62C4: 485FD165  bl 0x82df3428
	ctx.lr = 0x827F62C8;
	sub_82DF3428(ctx, base);
	// 827F62C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F62CC: 485FD15D  bl 0x82df3428
	ctx.lr = 0x827F62D0;
	sub_82DF3428(ctx, base);
	// 827F62D0: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 827F62D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F62D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F62DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F62E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F62E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F62E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F62E8 size=88
    let mut pc: u32 = 0x827F62E8;
    'dispatch: loop {
        match pc {
            0x827F62E8 => {
    //   block [0x827F62E8..0x827F6340)
	// 827F62E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F62EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F62F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F62F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F62F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F62FC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 827F6300: 4BAD7491  bl 0x822cd790
	ctx.lr = 0x827F6304;
	sub_822CD790(ctx, base);
	// 827F6304: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 827F6308: 4BFFB491  bl 0x827f1798
	ctx.lr = 0x827F630C;
	sub_827F1798(ctx, base);
	// 827F630C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 827F6310: 4BB05999  bl 0x822fbca8
	ctx.lr = 0x827F6314;
	sub_822FBCA8(ctx, base);
	// 827F6314: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 827F6318: 485FD111  bl 0x82df3428
	ctx.lr = 0x827F631C;
	sub_82DF3428(ctx, base);
	// 827F631C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F6320: 485FD109  bl 0x82df3428
	ctx.lr = 0x827F6324;
	sub_82DF3428(ctx, base);
	// 827F6324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6328: 485FD101  bl 0x82df3428
	ctx.lr = 0x827F632C;
	sub_82DF3428(ctx, base);
	// 827F632C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F6330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F633C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6340 size=100
    let mut pc: u32 = 0x827F6340;
    'dispatch: loop {
        match pc {
            0x827F6340 => {
    //   block [0x827F6340..0x827F63A4)
	// 827F6340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6348: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F634C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F6358: 4BFFB621  bl 0x827f1978
	ctx.lr = 0x827F635C;
	sub_827F1978(ctx, base);
	// 827F635C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F6360: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 827F6364: 3BCB9BC9  addi r30, r11, -0x6437
	ctx.r[30].s64 = ctx.r[11].s64 + -25655;
	// 827F6368: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F636C: 485FD69D  bl 0x82df3a08
	ctx.lr = 0x827F6370;
	sub_82DF3A08(ctx, base);
	// 827F6370: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6374: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 827F6378: 485FD691  bl 0x82df3a08
	ctx.lr = 0x827F637C;
	sub_82DF3A08(ctx, base);
	// 827F637C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F6380: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6384: 485FD685  bl 0x82df3a08
	ctx.lr = 0x827F6388;
	sub_82DF3A08(ctx, base);
	// 827F6388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F638C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F6390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F639C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F63A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F63A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F63A8 size=136
    let mut pc: u32 = 0x827F63A8;
    'dispatch: loop {
        match pc {
            0x827F63A8 => {
    //   block [0x827F63A8..0x827F6430)
	// 827F63A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F63AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F63B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F63B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F63B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F63BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F63C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F63C4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827F63C8: 409A0020  bne cr6, 0x827f63e8
	if !ctx.cr[6].eq {
	pc = 0x827F63E8; continue 'dispatch;
	}
	// 827F63CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F63D0: 419A0048  beq cr6, 0x827f6418
	if ctx.cr[6].eq {
	pc = 0x827F6418; continue 'dispatch;
	}
	// 827F63D4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827F63D8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 827F63DC: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 827F63E0: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827F63E4: 48000034  b 0x827f6418
	pc = 0x827F6418; continue 'dispatch;
	// 827F63E8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827F63EC: 419A002C  beq cr6, 0x827f6418
	if ctx.cr[6].eq {
	pc = 0x827F6418; continue 'dispatch;
	}
	// 827F63F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F63F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F63F8: 388BCE20  addi r4, r11, -0x31e0
	ctx.r[4].s64 = ctx.r[11].s64 + -12768;
	// 827F63FC: 489B1CFD  bl 0x831a80f8
	ctx.lr = 0x827F6400;
	sub_831A80F8(ctx, base);
	// 827F6400: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F6404: 4182000C  beq 0x827f6410
	if ctx.cr[0].eq {
	pc = 0x827F6410; continue 'dispatch;
	}
	// 827F6408: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827F640C: 4800000C  b 0x827f6418
	pc = 0x827F6418; continue 'dispatch;
	// 827F6410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F6414: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F6428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F642C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6430 size=136
    let mut pc: u32 = 0x827F6430;
    'dispatch: loop {
        match pc {
            0x827F6430 => {
    //   block [0x827F6430..0x827F64B8)
	// 827F6430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F643C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6444: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F6448: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F644C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827F6450: 409A0020  bne cr6, 0x827f6470
	if !ctx.cr[6].eq {
	pc = 0x827F6470; continue 'dispatch;
	}
	// 827F6454: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6458: 419A0048  beq cr6, 0x827f64a0
	if ctx.cr[6].eq {
	pc = 0x827F64A0; continue 'dispatch;
	}
	// 827F645C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827F6460: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 827F6464: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 827F6468: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827F646C: 48000034  b 0x827f64a0
	pc = 0x827F64A0; continue 'dispatch;
	// 827F6470: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827F6474: 419A002C  beq cr6, 0x827f64a0
	if ctx.cr[6].eq {
	pc = 0x827F64A0; continue 'dispatch;
	}
	// 827F6478: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F647C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6480: 388BCEE8  addi r4, r11, -0x3118
	ctx.r[4].s64 = ctx.r[11].s64 + -12568;
	// 827F6484: 489B1C75  bl 0x831a80f8
	ctx.lr = 0x827F6488;
	sub_831A80F8(ctx, base);
	// 827F6488: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F648C: 4182000C  beq 0x827f6498
	if ctx.cr[0].eq {
	pc = 0x827F6498; continue 'dispatch;
	}
	// 827F6490: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827F6494: 4800000C  b 0x827f64a0
	pc = 0x827F64A0; continue 'dispatch;
	// 827F6498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F649C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F64A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F64A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F64A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F64AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F64B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F64B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F64B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F64B8 size=196
    let mut pc: u32 = 0x827F64B8;
    'dispatch: loop {
        match pc {
            0x827F64B8 => {
    //   block [0x827F64B8..0x827F657C)
	// 827F64B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F64BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F64C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F64C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F64C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F64CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F64D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F64D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F64D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F64DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F64E0: 4BACA459  bl 0x822c0938
	ctx.lr = 0x827F64E4;
	sub_822C0938(ctx, base);
	// 827F64E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F64E8: 41820028  beq 0x827f6510
	if ctx.cr[0].eq {
	pc = 0x827F6510; continue 'dispatch;
	}
	// 827F64EC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F64F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F64F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F64F8: 392B658C  addi r9, r11, 0x658c
	ctx.r[9].s64 = ctx.r[11].s64 + 25996;
	// 827F64FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F6500: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F6504: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F6508: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F650C: 48000008  b 0x827f6514
	pc = 0x827F6514; continue 'dispatch;
	// 827F6510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F6514: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6518: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F651C: 409A0044  bne cr6, 0x827f6560
	if !ctx.cr[6].eq {
	pc = 0x827F6560; continue 'dispatch;
	}
	// 827F6520: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6524: 419A001C  beq cr6, 0x827f6540
	if ctx.cr[6].eq {
	pc = 0x827F6540; continue 'dispatch;
	}
	// 827F6528: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F652C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F6530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6534: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6538: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F653C: 4E800421  bctrl
	ctx.lr = 0x827F6540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F6540: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F6544: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F6548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F654C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F6550: 816BCD4C  lwz r11, -0x32b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12980 as u32) ) } as u64;
	// 827F6554: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F6558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F655C: 4BAC9AA5  bl 0x822c0000
	ctx.lr = 0x827F6560;
	sub_822C0000(ctx, base);
	// 827F6560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F6564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F6568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F656C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F6574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6580 size=120
    let mut pc: u32 = 0x827F6580;
    'dispatch: loop {
        match pc {
            0x827F6580 => {
    //   block [0x827F6580..0x827F65F8)
	// 827F6580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F658C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F6594: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 827F6598: 485FCE91  bl 0x82df3428
	ctx.lr = 0x827F659C;
	sub_82DF3428(ctx, base);
	// 827F659C: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 827F65A0: 485FCE89  bl 0x82df3428
	ctx.lr = 0x827F65A4;
	sub_82DF3428(ctx, base);
	// 827F65A4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 827F65A8: 485FCE81  bl 0x82df3428
	ctx.lr = 0x827F65AC;
	sub_82DF3428(ctx, base);
	// 827F65AC: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 827F65B0: 485FCE79  bl 0x82df3428
	ctx.lr = 0x827F65B4;
	sub_82DF3428(ctx, base);
	// 827F65B4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 827F65B8: 485FCE71  bl 0x82df3428
	ctx.lr = 0x827F65BC;
	sub_82DF3428(ctx, base);
	// 827F65BC: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 827F65C0: 485FCE69  bl 0x82df3428
	ctx.lr = 0x827F65C4;
	sub_82DF3428(ctx, base);
	// 827F65C4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F65C8: 485FCE61  bl 0x82df3428
	ctx.lr = 0x827F65CC;
	sub_82DF3428(ctx, base);
	// 827F65CC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827F65D0: 485FCE59  bl 0x82df3428
	ctx.lr = 0x827F65D4;
	sub_82DF3428(ctx, base);
	// 827F65D4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 827F65D8: 485FCE51  bl 0x82df3428
	ctx.lr = 0x827F65DC;
	sub_82DF3428(ctx, base);
	// 827F65DC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 827F65E0: 485FCE49  bl 0x82df3428
	ctx.lr = 0x827F65E4;
	sub_82DF3428(ctx, base);
	// 827F65E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F65E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F65EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F65F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F65F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F65F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F65F8 size=72
    let mut pc: u32 = 0x827F65F8;
    'dispatch: loop {
        match pc {
            0x827F65F8 => {
    //   block [0x827F65F8..0x827F6640)
	// 827F65F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F65FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6604: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827F6608: 419A001C  beq cr6, 0x827f6624
	if ctx.cr[6].eq {
	pc = 0x827F6624; continue 'dispatch;
	}
	// 827F660C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F6610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F6614: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827F6618: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F661C: 4BFFFD8D  bl 0x827f63a8
	ctx.lr = 0x827F6620;
	sub_827F63A8(ctx, base);
	// 827F6620: 48000010  b 0x827f6630
	pc = 0x827F6630; continue 'dispatch;
	// 827F6624: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F6628: 396BCE20  addi r11, r11, -0x31e0
	ctx.r[11].s64 = ctx.r[11].s64 + -12768;
	// 827F662C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F6634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F663C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6640 size=72
    let mut pc: u32 = 0x827F6640;
    'dispatch: loop {
        match pc {
            0x827F6640 => {
    //   block [0x827F6640..0x827F6688)
	// 827F6640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F664C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827F6650: 419A001C  beq cr6, 0x827f666c
	if ctx.cr[6].eq {
	pc = 0x827F666C; continue 'dispatch;
	}
	// 827F6654: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F6658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F665C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827F6660: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F6664: 4BFFFDCD  bl 0x827f6430
	ctx.lr = 0x827F6668;
	sub_827F6430(ctx, base);
	// 827F6668: 48000010  b 0x827f6678
	pc = 0x827F6678; continue 'dispatch;
	// 827F666C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F6670: 396BCEE8  addi r11, r11, -0x3118
	ctx.r[11].s64 = ctx.r[11].s64 + -12568;
	// 827F6674: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F667C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6688 size=140
    let mut pc: u32 = 0x827F6688;
    'dispatch: loop {
        match pc {
            0x827F6688 => {
    //   block [0x827F6688..0x827F6714)
	// 827F6688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6694: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6698: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F669C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 827F66A0: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F66A4: 3BE50004  addi r31, r5, 4
	ctx.r[31].s64 = ctx.r[5].s64 + 4;
	// 827F66A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F66AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F66B0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 827F66B4: 419A0024  beq cr6, 0x827f66d8
	if ctx.cr[6].eq {
	pc = 0x827F66D8; continue 'dispatch;
	}
	// 827F66B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F66BC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827F66C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F66C4: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827F66C8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827F66CC: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F66D0: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F66D4: 4082FFE8  bne 0x827f66bc
	if !ctx.cr[0].eq {
	pc = 0x827F66BC; continue 'dispatch;
	}
	// 827F66D8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F66DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F66E0: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F66E4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827F66E8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 827F66EC: 4E800421  bctrl
	ctx.lr = 0x827F66F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F66F0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F66F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F66F8: 419A0008  beq cr6, 0x827f6700
	if ctx.cr[6].eq {
	pc = 0x827F6700; continue 'dispatch;
	}
	// 827F66FC: 4BACA195  bl 0x822c0890
	ctx.lr = 0x827F6700;
	sub_822C0890(ctx, base);
	// 827F6700: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F6704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F670C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6718 size=88
    let mut pc: u32 = 0x827F6718;
    'dispatch: loop {
        match pc {
            0x827F6718 => {
    //   block [0x827F6718..0x827F6770)
	// 827F6718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F671C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F6724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F672C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F6730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F6734: 48662F6D  bl 0x82e596a0
	ctx.lr = 0x827F6738;
	sub_82E596A0(ctx, base);
	// 827F6738: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F673C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 827F6740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F6744: 394A65C8  addi r10, r10, 0x65c8
	ctx.r[10].s64 = ctx.r[10].s64 + 26056;
	// 827F6748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F674C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F6750: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 827F6754: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 827F6758: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6764: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F6768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F676C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6770 size=88
    let mut pc: u32 = 0x827F6770;
    'dispatch: loop {
        match pc {
            0x827F6770 => {
    //   block [0x827F6770..0x827F67C8)
	// 827F6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F677C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F6788: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F678C: 48662F15  bl 0x82e596a0
	ctx.lr = 0x827F6790;
	sub_82E596A0(ctx, base);
	// 827F6790: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F6794: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 827F6798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F679C: 394A65D0  addi r10, r10, 0x65d0
	ctx.r[10].s64 = ctx.r[10].s64 + 26064;
	// 827F67A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F67A4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F67A8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 827F67AC: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 827F67B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F67B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F67B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F67BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F67C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F67C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F67C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F67C8 size=112
    let mut pc: u32 = 0x827F67C8;
    'dispatch: loop {
        match pc {
            0x827F67C8 => {
    //   block [0x827F67C8..0x827F6838)
	// 827F67C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F67CC: 489B19A1  bl 0x831a816c
	ctx.lr = 0x827F67D0;
	sub_831A8130(ctx, base);
	// 827F67D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F67D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F67D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F67DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F67E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F67E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F67E8: 4BCFFBF9  bl 0x824f63e0
	ctx.lr = 0x827F67EC;
	sub_824F63E0(ctx, base);
	// 827F67EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F67F0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 827F67F4: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F67F8: 419A0020  beq cr6, 0x827f6818
	if ctx.cr[6].eq {
	pc = 0x827F6818; continue 'dispatch;
	}
	// 827F67FC: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 827F6800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F6804: 485FCA35  bl 0x82df3238
	ctx.lr = 0x827F6808;
	sub_82DF3238(ctx, base);
	// 827F6808: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F680C: 4082000C  bne 0x827f6818
	if !ctx.cr[0].eq {
	pc = 0x827F6818; continue 'dispatch;
	}
	// 827F6810: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F6814: 48000010  b 0x827f6824
	pc = 0x827F6824; continue 'dispatch;
	// 827F6818: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F681C: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 827F6820: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 827F6824: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6828: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F682C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F6834: 489B1988  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6838 size=108
    let mut pc: u32 = 0x827F6838;
    'dispatch: loop {
        match pc {
            0x827F6838 => {
    //   block [0x827F6838..0x827F68A4)
	// 827F6838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F683C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6844: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6848: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 827F684C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 827F6850: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F6854: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F685C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F6860: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 827F6864: 419A0024  beq cr6, 0x827f6888
	if ctx.cr[6].eq {
	pc = 0x827F6888; continue 'dispatch;
	}
	// 827F6868: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F686C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 827F6870: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6874: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 827F6878: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 827F687C: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F6880: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6884: 4082FFE8  bne 0x827f686c
	if !ctx.cr[0].eq {
	pc = 0x827F686C; continue 'dispatch;
	}
	// 827F6888: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F688C: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6890: 4BFFFDF9  bl 0x827f6688
	ctx.lr = 0x827F6894;
	sub_827F6688(ctx, base);
	// 827F6894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F6898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F689C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F68A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F68A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F68A8 size=236
    let mut pc: u32 = 0x827F68A8;
    'dispatch: loop {
        match pc {
            0x827F68A8 => {
    //   block [0x827F68A8..0x827F6994)
	// 827F68A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F68AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F68B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F68B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F68B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F68BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F68C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F68C4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827F68C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F68CC: 485FD13D  bl 0x82df3a08
	ctx.lr = 0x827F68D0;
	sub_82DF3A08(ctx, base);
	// 827F68D0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F68D4: 389E00CC  addi r4, r30, 0xcc
	ctx.r[4].s64 = ctx.r[30].s64 + 204;
	// 827F68D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F68DC: 4BFFFEED  bl 0x827f67c8
	ctx.lr = 0x827F68E0;
	sub_827F67C8(ctx, base);
	// 827F68E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F68E4: 485FCB45  bl 0x82df3428
	ctx.lr = 0x827F68E8;
	sub_82DF3428(ctx, base);
	// 827F68E8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F68EC: 815E00D0  lwz r10, 0xd0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(208 as u32) ) } as u64;
	// 827F68F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F68F4: 419A0040  beq cr6, 0x827f6934
	if ctx.cr[6].eq {
	pc = 0x827F6934; continue 'dispatch;
	}
	// 827F68F8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F68FC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F6900: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F6904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F6908: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F690C: 419A006C  beq cr6, 0x827f6978
	if ctx.cr[6].eq {
	pc = 0x827F6978; continue 'dispatch;
	}
	// 827F6910: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F6914: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F6918: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F691C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F6920: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F6924: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F6928: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F692C: 4082FFE8  bne 0x827f6914
	if !ctx.cr[0].eq {
	pc = 0x827F6914; continue 'dispatch;
	}
	// 827F6930: 48000048  b 0x827f6978
	pc = 0x827F6978; continue 'dispatch;
	// 827F6934: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827F6938: 394BAD44  addi r10, r11, -0x52bc
	ctx.r[10].s64 = ctx.r[11].s64 + -21180;
	// 827F693C: 816BAD44  lwz r11, -0x52bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21180 as u32) ) } as u64;
	// 827F6940: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6944: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F6948: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F694C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F6950: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F6954: 419A0024  beq cr6, 0x827f6978
	if ctx.cr[6].eq {
	pc = 0x827F6978; continue 'dispatch;
	}
	// 827F6958: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F695C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F6960: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6964: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F6968: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F696C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F6970: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6974: 4082FFE8  bne 0x827f695c
	if !ctx.cr[0].eq {
	pc = 0x827F695C; continue 'dispatch;
	}
	// 827F6978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F697C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F6980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6988: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F698C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6998 size=100
    let mut pc: u32 = 0x827F6998;
    'dispatch: loop {
        match pc {
            0x827F6998 => {
    //   block [0x827F6998..0x827F69FC)
	// 827F6998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F69A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F69A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F69A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F69AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F69B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F69B4: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F69B8: 4BFFFEF1  bl 0x827f68a8
	ctx.lr = 0x827F69BC;
	sub_827F68A8(ctx, base);
	// 827F69BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F69C0: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 827F69C4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F69C8: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827F69CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F69D0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 827F69D4: 4BACDA8D  bl 0x822c4460
	ctx.lr = 0x827F69D8;
	sub_822C4460(ctx, base);
	// 827F69D8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F69DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F69E0: 419A0008  beq cr6, 0x827f69e8
	if ctx.cr[6].eq {
	pc = 0x827F69E8; continue 'dispatch;
	}
	// 827F69E4: 4BAC9EAD  bl 0x822c0890
	ctx.lr = 0x827F69E8;
	sub_822C0890(ctx, base);
	// 827F69E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F69EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F69F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F69F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F69F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6A00 size=188
    let mut pc: u32 = 0x827F6A00;
    'dispatch: loop {
        match pc {
            0x827F6A00 => {
    //   block [0x827F6A00..0x827F6ABC)
	// 827F6A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6A08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F6A0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6A14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F6A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F6A1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F6A20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F6A24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6A28: 4BAC9F11  bl 0x822c0938
	ctx.lr = 0x827F6A2C;
	sub_822C0938(ctx, base);
	// 827F6A2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F6A30: 41820028  beq 0x827f6a58
	if ctx.cr[0].eq {
	pc = 0x827F6A58; continue 'dispatch;
	}
	// 827F6A34: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6A38: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F6A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F6A40: 392B65B4  addi r9, r11, 0x65b4
	ctx.r[9].s64 = ctx.r[11].s64 + 26036;
	// 827F6A44: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F6A48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F6A4C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F6A50: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F6A54: 48000008  b 0x827f6a5c
	pc = 0x827F6A5C; continue 'dispatch;
	// 827F6A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F6A5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F6A60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F6A64: 409A003C  bne cr6, 0x827f6aa0
	if !ctx.cr[6].eq {
	pc = 0x827F6AA0; continue 'dispatch;
	}
	// 827F6A68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6A6C: 419A0014  beq cr6, 0x827f6a80
	if ctx.cr[6].eq {
	pc = 0x827F6A80; continue 'dispatch;
	}
	// 827F6A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6A74: 4BFFFB0D  bl 0x827f6580
	ctx.lr = 0x827F6A78;
	sub_827F6580(ctx, base);
	// 827F6A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6A7C: 4BAC97ED  bl 0x822c0268
	ctx.lr = 0x827F6A80;
	sub_822C0268(ctx, base);
	// 827F6A80: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F6A84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F6A88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6A8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F6A90: 816BCD4C  lwz r11, -0x32b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12980 as u32) ) } as u64;
	// 827F6A94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F6A98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F6A9C: 4BAC9565  bl 0x822c0000
	ctx.lr = 0x827F6AA0;
	sub_822C0000(ctx, base);
	// 827F6AA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F6AA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F6AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6AB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F6AB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6AC0 size=64
    let mut pc: u32 = 0x827F6AC0;
    'dispatch: loop {
        match pc {
            0x827F6AC0 => {
    //   block [0x827F6AC0..0x827F6B00)
	// 827F6AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6AC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6ACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6AD0: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F6AD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6AD8: 419A0014  beq cr6, 0x827f6aec
	if ctx.cr[6].eq {
	pc = 0x827F6AEC; continue 'dispatch;
	}
	// 827F6ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6AE0: 4BFFFAA1  bl 0x827f6580
	ctx.lr = 0x827F6AE4;
	sub_827F6580(ctx, base);
	// 827F6AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6AE8: 4BAC9781  bl 0x822c0268
	ctx.lr = 0x827F6AEC;
	sub_822C0268(ctx, base);
	// 827F6AEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F6AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6B00 size=164
    let mut pc: u32 = 0x827F6B00;
    'dispatch: loop {
        match pc {
            0x827F6B00 => {
    //   block [0x827F6B00..0x827F6BA4)
	// 827F6B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6B04: 489B1665  bl 0x831a8168
	ctx.lr = 0x827F6B08;
	sub_831A8130(ctx, base);
	// 827F6B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6B0C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827F6B10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F6B14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F6B18: 57BC063F  clrlwi. r28, r29, 0x18
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 827F6B1C: 41820038  beq 0x827f6b54
	if ctx.cr[0].eq {
	pc = 0x827F6B54; continue 'dispatch;
	}
	// 827F6B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6B24: 489B2E65  bl 0x831a9988
	ctx.lr = 0x827F6B28;
	sub_831A9988(ctx, base);
	// 827F6B28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F6B2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F6B30: 386BCFF0  addi r3, r11, -0x3010
	ctx.r[3].s64 = ctx.r[11].s64 + -12304;
	// 827F6B34: 489B15C5  bl 0x831a80f8
	ctx.lr = 0x827F6B38;
	sub_831A80F8(ctx, base);
	// 827F6B38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F6B3C: 41820018  beq 0x827f6b54
	if ctx.cr[0].eq {
	pc = 0x827F6B54; continue 'dispatch;
	}
	// 827F6B40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F6B44: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827F6B48: 4BFFFE51  bl 0x827f6998
	ctx.lr = 0x827F6B4C;
	sub_827F6998(ctx, base);
	// 827F6B4C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F6B50: 4800004C  b 0x827f6b9c
	pc = 0x827F6B9C; continue 'dispatch;
	// 827F6B54: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827F6B58: 419A0034  beq cr6, 0x827f6b8c
	if ctx.cr[6].eq {
	pc = 0x827F6B8C; continue 'dispatch;
	}
	// 827F6B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6B60: 489B2E29  bl 0x831a9988
	ctx.lr = 0x827F6B64;
	sub_831A9988(ctx, base);
	// 827F6B64: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F6B68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F6B6C: 386BCFC4  addi r3, r11, -0x303c
	ctx.r[3].s64 = ctx.r[11].s64 + -12348;
	// 827F6B70: 489B1589  bl 0x831a80f8
	ctx.lr = 0x827F6B74;
	sub_831A80F8(ctx, base);
	// 827F6B74: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F6B78: 41820014  beq 0x827f6b8c
	if ctx.cr[0].eq {
	pc = 0x827F6B8C; continue 'dispatch;
	}
	// 827F6B7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F6B80: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827F6B84: 4818109D  bl 0x82977c20
	ctx.lr = 0x827F6B88;
	sub_82977C20(ctx, base);
	// 827F6B88: 4BFFFFC4  b 0x827f6b4c
	pc = 0x827F6B4C; continue 'dispatch;
	// 827F6B8C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827F6B90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F6B94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F6B98: 4BFFAE59  bl 0x827f19f0
	ctx.lr = 0x827F6B9C;
	sub_827F19F0(ctx, base);
	// 827F6B9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F6BA0: 489B1618  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6BA8 size=96
    let mut pc: u32 = 0x827F6BA8;
    'dispatch: loop {
        match pc {
            0x827F6BA8 => {
    //   block [0x827F6BA8..0x827F6C08)
	// 827F6BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F6BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F6BB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6BB8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F6BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F6BC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F6BC4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F6BC8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 827F6BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827F6BD0: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827F6BD4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 827F6BD8: 38650008  addi r3, r5, 8
	ctx.r[3].s64 = ctx.r[5].s64 + 8;
	// 827F6BDC: 88810050  lbz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F6BE0: 4BFFFC59  bl 0x827f6838
	ctx.lr = 0x827F6BE4;
	sub_827F6838(ctx, base);
	// 827F6BE4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F6BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6BEC: 419A0008  beq cr6, 0x827f6bf4
	if ctx.cr[6].eq {
	pc = 0x827F6BF4; continue 'dispatch;
	}
	// 827F6BF0: 4BAC9CA1  bl 0x822c0890
	ctx.lr = 0x827F6BF4;
	sub_822C0890(ctx, base);
	// 827F6BF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F6BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F6BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F6C00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F6C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6C08 size=128
    let mut pc: u32 = 0x827F6C08;
    'dispatch: loop {
        match pc {
            0x827F6C08 => {
    //   block [0x827F6C08..0x827F6C88)
	// 827F6C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6C0C: 489B1561  bl 0x831a816c
	ctx.lr = 0x827F6C10;
	sub_831A8130(ctx, base);
	// 827F6C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6C14: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827F6C18: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827F6C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F6C20: 3BEBAD28  addi r31, r11, -0x52d8
	ctx.r[31].s64 = ctx.r[11].s64 + -21208;
	// 827F6C24: 816AAD30  lwz r11, -0x52d0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21200 as u32) ) } as u64;
	// 827F6C28: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827F6C2C: 40820024  bne 0x827f6c50
	if !ctx.cr[0].eq {
	pc = 0x827F6C50; continue 'dispatch;
	}
	// 827F6C30: 3D2082BC  lis r9, -0x7d44
	ctx.r[9].s64 = -2101608448;
	// 827F6C34: 3D00827F  lis r8, -0x7d81
	ctx.r[8].s64 = -2105606144;
	// 827F6C38: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827F6C3C: 3929AD78  addi r9, r9, -0x5288
	ctx.r[9].s64 = ctx.r[9].s64 + -21128;
	// 827F6C40: 39086640  addi r8, r8, 0x6640
	ctx.r[8].s64 = ctx.r[8].s64 + 26176;
	// 827F6C44: 916AAD30  stw r11, -0x52d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21200 as u32), ctx.r[11].u32 ) };
	// 827F6C48: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827F6C4C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827F6C50: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F6C54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F6C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6C5C: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 827F6C60: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827F6C64: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F6C68: 4BE5D959  bl 0x826545c0
	ctx.lr = 0x827F6C6C;
	sub_826545C0(ctx, base);
	// 827F6C6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F6C70: 4182000C  beq 0x827f6c7c
	if ctx.cr[0].eq {
	pc = 0x827F6C7C; continue 'dispatch;
	}
	// 827F6C74: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F6C78: 48000008  b 0x827f6c80
	pc = 0x827F6C80; continue 'dispatch;
	// 827F6C7C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827F6C80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F6C84: 489B1538  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6C88 size=288
    let mut pc: u32 = 0x827F6C88;
    'dispatch: loop {
        match pc {
            0x827F6C88 => {
    //   block [0x827F6C88..0x827F6DA8)
	// 827F6C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6C8C: 489B14E1  bl 0x831a816c
	ctx.lr = 0x827F6C90;
	sub_831A8130(ctx, base);
	// 827F6C90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6C94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F6C98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F6C9C: 3BAB9BC9  addi r29, r11, -0x6437
	ctx.r[29].s64 = ctx.r[11].s64 + -25655;
	// 827F6CA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F6CA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F6CA8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827F6CAC: 485FCD5D  bl 0x82df3a08
	ctx.lr = 0x827F6CB0;
	sub_82DF3A08(ctx, base);
	// 827F6CB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F6CB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6CB8: 485FCD51  bl 0x82df3a08
	ctx.lr = 0x827F6CBC;
	sub_82DF3A08(ctx, base);
	// 827F6CBC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6CC0: 4BDE30D1  bl 0x825d9d90
	ctx.lr = 0x827F6CC4;
	sub_825D9D90(ctx, base);
	// 827F6CC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827F6CC8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 827F6CCC: 388B4C9C  addi r4, r11, 0x4c9c
	ctx.r[4].s64 = ctx.r[11].s64 + 19612;
	// 827F6CD0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6CD4: 4BDE313D  bl 0x825d9e10
	ctx.lr = 0x827F6CD8;
	sub_825D9E10(ctx, base);
	// 827F6CD8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F6CDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F6CE0: 388B65D4  addi r4, r11, 0x65d4
	ctx.r[4].s64 = ctx.r[11].s64 + 26068;
	// 827F6CE4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6CE8: 4BDE3129  bl 0x825d9e10
	ctx.lr = 0x827F6CEC;
	sub_825D9E10(ctx, base);
	// 827F6CEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6CF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6CF4: 4BDE3115  bl 0x825d9e08
	ctx.lr = 0x827F6CF8;
	sub_825D9E08(ctx, base);
	// 827F6CF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F6CFC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F6D00: 485FCF01  bl 0x82df3c00
	ctx.lr = 0x827F6D04;
	sub_82DF3C00(ctx, base);
	// 827F6D04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F6D08: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F6D0C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827F6D10: 485FCEF1  bl 0x82df3c00
	ctx.lr = 0x827F6D14;
	sub_82DF3C00(ctx, base);
	// 827F6D14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F6D18: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F6D1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F6D20: 485FCEE1  bl 0x82df3c00
	ctx.lr = 0x827F6D24;
	sub_82DF3C00(ctx, base);
	// 827F6D24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F6D28: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 827F6D2C: 485FCED5  bl 0x82df3c00
	ctx.lr = 0x827F6D30;
	sub_82DF3C00(ctx, base);
	// 827F6D30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F6D34: 485FC6F5  bl 0x82df3428
	ctx.lr = 0x827F6D38;
	sub_82DF3428(ctx, base);
	// 827F6D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F6D3C: 485FC6ED  bl 0x82df3428
	ctx.lr = 0x827F6D40;
	sub_82DF3428(ctx, base);
	// 827F6D40: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827F6D44: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F6D48: 485FCEB9  bl 0x82df3c00
	ctx.lr = 0x827F6D4C;
	sub_82DF3C00(ctx, base);
	// 827F6D4C: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 827F6D50: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 827F6D54: 485FCEAD  bl 0x82df3c00
	ctx.lr = 0x827F6D58;
	sub_82DF3C00(ctx, base);
	// 827F6D58: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 827F6D5C: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 827F6D60: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827F6D64: 4BB04985  bl 0x822fb6e8
	ctx.lr = 0x827F6D68;
	sub_822FB6E8(ctx, base);
	// 827F6D68: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 827F6D6C: 485FC6BD  bl 0x82df3428
	ctx.lr = 0x827F6D70;
	sub_82DF3428(ctx, base);
	// 827F6D70: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F6D74: 485FC6B5  bl 0x82df3428
	ctx.lr = 0x827F6D78;
	sub_82DF3428(ctx, base);
	// 827F6D78: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 827F6D7C: 485FC6AD  bl 0x82df3428
	ctx.lr = 0x827F6D80;
	sub_82DF3428(ctx, base);
	// 827F6D80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F6D84: 485FC6A5  bl 0x82df3428
	ctx.lr = 0x827F6D88;
	sub_82DF3428(ctx, base);
	// 827F6D88: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6D8C: 4BDE303D  bl 0x825d9dc8
	ctx.lr = 0x827F6D90;
	sub_825D9DC8(ctx, base);
	// 827F6D90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6D94: 485FC695  bl 0x82df3428
	ctx.lr = 0x827F6D98;
	sub_82DF3428(ctx, base);
	// 827F6D98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F6D9C: 485FC68D  bl 0x82df3428
	ctx.lr = 0x827F6DA0;
	sub_82DF3428(ctx, base);
	// 827F6DA0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F6DA4: 489B1418  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6DA8 size=228
    let mut pc: u32 = 0x827F6DA8;
    'dispatch: loop {
        match pc {
            0x827F6DA8 => {
    //   block [0x827F6DA8..0x827F6E8C)
	// 827F6DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6DAC: 489B13BD  bl 0x831a8168
	ctx.lr = 0x827F6DB0;
	sub_831A8130(ctx, base);
	// 827F6DB0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6DB4: 83E50004  lwz r31, 4(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F6DB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F6DBC: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6DC0: 3BC50004  addi r30, r5, 4
	ctx.r[30].s64 = ctx.r[5].s64 + 4;
	// 827F6DC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6DC8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 827F6DCC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827F6DD0: 419A0024  beq cr6, 0x827f6df4
	if ctx.cr[6].eq {
	pc = 0x827F6DF4; continue 'dispatch;
	}
	// 827F6DD4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827F6DD8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F6DDC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6DE0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F6DE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F6DE8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F6DEC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6DF0: 4082FFE8  bne 0x827f6dd8
	if !ctx.cr[0].eq {
	pc = 0x827F6DD8; continue 'dispatch;
	}
	// 827F6DF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6DF8: 485FCE09  bl 0x82df3c00
	ctx.lr = 0x827F6DFC;
	sub_82DF3C00(ctx, base);
	// 827F6DFC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F6E00: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827F6E04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F6E08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6E0C: 48294A05  bl 0x82a8b810
	ctx.lr = 0x827F6E10;
	sub_82A8B810(ctx, base);
	// 827F6E10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F6E14: 485FC615  bl 0x82df3428
	ctx.lr = 0x827F6E18;
	sub_82DF3428(ctx, base);
	// 827F6E18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6E1C: 419A000C  beq cr6, 0x827f6e28
	if ctx.cr[6].eq {
	pc = 0x827F6E28; continue 'dispatch;
	}
	// 827F6E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6E24: 4BAC9A6D  bl 0x822c0890
	ctx.lr = 0x827F6E28;
	sub_822C0890(ctx, base);
	// 827F6E28: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827F6E2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F6E30: 4838E981  bl 0x82b857b0
	ctx.lr = 0x827F6E34;
	sub_82B857B0(ctx, base);
	// 827F6E34: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827F6E38: 389D00C0  addi r4, r29, 0xc0
	ctx.r[4].s64 = ctx.r[29].s64 + 192;
	// 827F6E3C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F6E40: 4838FB81  bl 0x82b869c0
	ctx.lr = 0x827F6E44;
	sub_82B869C0(ctx, base);
	// 827F6E44: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 827F6E48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6E4C: 419A0008  beq cr6, 0x827f6e54
	if ctx.cr[6].eq {
	pc = 0x827F6E54; continue 'dispatch;
	}
	// 827F6E50: 4BAC9A41  bl 0x822c0890
	ctx.lr = 0x827F6E54;
	sub_822C0890(ctx, base);
	// 827F6E54: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F6E58: 485FC5D1  bl 0x82df3428
	ctx.lr = 0x827F6E5C;
	sub_82DF3428(ctx, base);
	// 827F6E5C: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 827F6E60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6E64: 419A0008  beq cr6, 0x827f6e6c
	if ctx.cr[6].eq {
	pc = 0x827F6E6C; continue 'dispatch;
	}
	// 827F6E68: 4BAC9A29  bl 0x822c0890
	ctx.lr = 0x827F6E6C;
	sub_822C0890(ctx, base);
	// 827F6E6C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6E70: 485FC5B9  bl 0x82df3428
	ctx.lr = 0x827F6E74;
	sub_82DF3428(ctx, base);
	// 827F6E74: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6E78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6E7C: 419A0008  beq cr6, 0x827f6e84
	if ctx.cr[6].eq {
	pc = 0x827F6E84; continue 'dispatch;
	}
	// 827F6E80: 4BAC9A11  bl 0x822c0890
	ctx.lr = 0x827F6E84;
	sub_822C0890(ctx, base);
	// 827F6E84: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F6E88: 489B1330  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6E90 size=228
    let mut pc: u32 = 0x827F6E90;
    'dispatch: loop {
        match pc {
            0x827F6E90 => {
    //   block [0x827F6E90..0x827F6F74)
	// 827F6E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6E94: 489B12D5  bl 0x831a8168
	ctx.lr = 0x827F6E98;
	sub_831A8130(ctx, base);
	// 827F6E98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6E9C: 83E50004  lwz r31, 4(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F6EA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F6EA4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6EA8: 3BC50004  addi r30, r5, 4
	ctx.r[30].s64 = ctx.r[5].s64 + 4;
	// 827F6EAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6EB0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 827F6EB4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827F6EB8: 419A0024  beq cr6, 0x827f6edc
	if ctx.cr[6].eq {
	pc = 0x827F6EDC; continue 'dispatch;
	}
	// 827F6EBC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827F6EC0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F6EC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6EC8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F6ECC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F6ED0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F6ED4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F6ED8: 4082FFE8  bne 0x827f6ec0
	if !ctx.cr[0].eq {
	pc = 0x827F6EC0; continue 'dispatch;
	}
	// 827F6EDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F6EE0: 485FCD21  bl 0x82df3c00
	ctx.lr = 0x827F6EE4;
	sub_82DF3C00(ctx, base);
	// 827F6EE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F6EE8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827F6EEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F6EF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6EF4: 4829491D  bl 0x82a8b810
	ctx.lr = 0x827F6EF8;
	sub_82A8B810(ctx, base);
	// 827F6EF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F6EFC: 485FC52D  bl 0x82df3428
	ctx.lr = 0x827F6F00;
	sub_82DF3428(ctx, base);
	// 827F6F00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F6F04: 419A000C  beq cr6, 0x827f6f10
	if ctx.cr[6].eq {
	pc = 0x827F6F10; continue 'dispatch;
	}
	// 827F6F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6F0C: 4BAC9985  bl 0x822c0890
	ctx.lr = 0x827F6F10;
	sub_822C0890(ctx, base);
	// 827F6F10: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827F6F14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F6F18: 4838E899  bl 0x82b857b0
	ctx.lr = 0x827F6F1C;
	sub_82B857B0(ctx, base);
	// 827F6F1C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827F6F20: 389D00CC  addi r4, r29, 0xcc
	ctx.r[4].s64 = ctx.r[29].s64 + 204;
	// 827F6F24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F6F28: 4838FA99  bl 0x82b869c0
	ctx.lr = 0x827F6F2C;
	sub_82B869C0(ctx, base);
	// 827F6F2C: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 827F6F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6F34: 419A0008  beq cr6, 0x827f6f3c
	if ctx.cr[6].eq {
	pc = 0x827F6F3C; continue 'dispatch;
	}
	// 827F6F38: 4BAC9959  bl 0x822c0890
	ctx.lr = 0x827F6F3C;
	sub_822C0890(ctx, base);
	// 827F6F3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F6F40: 485FC4E9  bl 0x82df3428
	ctx.lr = 0x827F6F44;
	sub_82DF3428(ctx, base);
	// 827F6F44: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 827F6F48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6F4C: 419A0008  beq cr6, 0x827f6f54
	if ctx.cr[6].eq {
	pc = 0x827F6F54; continue 'dispatch;
	}
	// 827F6F50: 4BAC9941  bl 0x822c0890
	ctx.lr = 0x827F6F54;
	sub_822C0890(ctx, base);
	// 827F6F54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F6F58: 485FC4D1  bl 0x82df3428
	ctx.lr = 0x827F6F5C;
	sub_82DF3428(ctx, base);
	// 827F6F5C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F6F60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F6F64: 419A0008  beq cr6, 0x827f6f6c
	if ctx.cr[6].eq {
	pc = 0x827F6F6C; continue 'dispatch;
	}
	// 827F6F68: 4BAC9929  bl 0x822c0890
	ctx.lr = 0x827F6F6C;
	sub_822C0890(ctx, base);
	// 827F6F6C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F6F70: 489B1248  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6F78 size=128
    let mut pc: u32 = 0x827F6F78;
    'dispatch: loop {
        match pc {
            0x827F6F78 => {
    //   block [0x827F6F78..0x827F6FF8)
	// 827F6F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6F7C: 489B11F1  bl 0x831a816c
	ctx.lr = 0x827F6F80;
	sub_831A8130(ctx, base);
	// 827F6F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F6F84: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827F6F88: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827F6F8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F6F90: 3BEBAD34  addi r31, r11, -0x52cc
	ctx.r[31].s64 = ctx.r[11].s64 + -21196;
	// 827F6F94: 816AAD3C  lwz r11, -0x52c4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21188 as u32) ) } as u64;
	// 827F6F98: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827F6F9C: 40820024  bne 0x827f6fc0
	if !ctx.cr[0].eq {
	pc = 0x827F6FC0; continue 'dispatch;
	}
	// 827F6FA0: 3D20827F  lis r9, -0x7d81
	ctx.r[9].s64 = -2105606144;
	// 827F6FA4: 3D00827F  lis r8, -0x7d81
	ctx.r[8].s64 = -2105606144;
	// 827F6FA8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827F6FAC: 39296BA8  addi r9, r9, 0x6ba8
	ctx.r[9].s64 = ctx.r[9].s64 + 27560;
	// 827F6FB0: 390865F8  addi r8, r8, 0x65f8
	ctx.r[8].s64 = ctx.r[8].s64 + 26104;
	// 827F6FB4: 916AAD3C  stw r11, -0x52c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21188 as u32), ctx.r[11].u32 ) };
	// 827F6FB8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827F6FBC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827F6FC0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F6FC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F6FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F6FCC: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 827F6FD0: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827F6FD4: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F6FD8: 4BE5D5E9  bl 0x826545c0
	ctx.lr = 0x827F6FDC;
	sub_826545C0(ctx, base);
	// 827F6FDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F6FE0: 4182000C  beq 0x827f6fec
	if ctx.cr[0].eq {
	pc = 0x827F6FEC; continue 'dispatch;
	}
	// 827F6FE4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F6FE8: 48000008  b 0x827f6ff0
	pc = 0x827F6FF0; continue 'dispatch;
	// 827F6FEC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827F6FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F6FF4: 489B11C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F6FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F6FF8 size=128
    let mut pc: u32 = 0x827F6FF8;
    'dispatch: loop {
        match pc {
            0x827F6FF8 => {
    //   block [0x827F6FF8..0x827F7078)
	// 827F6FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F6FFC: 489B1171  bl 0x831a816c
	ctx.lr = 0x827F7000;
	sub_831A8130(ctx, base);
	// 827F7000: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F7008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F700C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F7010: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827F7014: 4BDE2D7D  bl 0x825d9d90
	ctx.lr = 0x827F7018;
	sub_825D9D90(ctx, base);
	// 827F7018: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 827F701C: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 827F7020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7024: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 827F7028: 394A6C88  addi r10, r10, 0x6c88
	ctx.r[10].s64 = ctx.r[10].s64 + 27784;
	// 827F702C: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 827F7030: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F7034: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827F7038: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 827F703C: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 827F7040: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827F7044: 4BFFFBC5  bl 0x827f6c08
	ctx.lr = 0x827F7048;
	sub_827F6C08(ctx, base);
	// 827F7048: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F704C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 827F7050: 388B50BC  addi r4, r11, 0x50bc
	ctx.r[4].s64 = ctx.r[11].s64 + 20668;
	// 827F7054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7058: 4BDE2E89  bl 0x825d9ee0
	ctx.lr = 0x827F705C;
	sub_825D9EE0(ctx, base);
	// 827F705C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F7060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7064: 4BDE2DA5  bl 0x825d9e08
	ctx.lr = 0x827F7068;
	sub_825D9E08(ctx, base);
	// 827F7068: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F706C: 4BDE2D5D  bl 0x825d9dc8
	ctx.lr = 0x827F7070;
	sub_825D9DC8(ctx, base);
	// 827F7070: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F7074: 489B1148  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7078 size=600
    let mut pc: u32 = 0x827F7078;
    'dispatch: loop {
        match pc {
            0x827F7078 => {
    //   block [0x827F7078..0x827F72D0)
	// 827F7078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F707C: 489B10ED  bl 0x831a8168
	ctx.lr = 0x827F7080;
	sub_831A8130(ctx, base);
	// 827F7080: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7084: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7088: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F708C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F7090: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7094: 388B6658  addi r4, r11, 0x6658
	ctx.r[4].s64 = ctx.r[11].s64 + 26200;
	// 827F7098: 38A000A9  li r5, 0xa9
	ctx.r[5].s64 = 169;
	// 827F709C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 827F70A0: 4BAC9339  bl 0x822c03d8
	ctx.lr = 0x827F70A4;
	sub_822C03D8(ctx, base);
	// 827F70A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F70A8: 41820010  beq 0x827f70b8
	if ctx.cr[0].eq {
	pc = 0x827F70B8; continue 'dispatch;
	}
	// 827F70AC: 4BFFEF8D  bl 0x827f6038
	ctx.lr = 0x827F70B0;
	sub_827F6038(ctx, base);
	// 827F70B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F70B4: 48000008  b 0x827f70bc
	pc = 0x827F70BC; continue 'dispatch;
	// 827F70B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F70BC: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 827F70C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F70C4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 827F70C8: 4BFFF939  bl 0x827f6a00
	ctx.lr = 0x827F70CC;
	sub_827F6A00(ctx, base);
	// 827F70CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F70D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F70D4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 827F70D8: 4BAC8F29  bl 0x822c0000
	ctx.lr = 0x827F70DC;
	sub_822C0000(ctx, base);
	// 827F70DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F70E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F70E4: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 827F70E8: 485FC921  bl 0x82df3a08
	ctx.lr = 0x827F70EC;
	sub_82DF3A08(ctx, base);
	// 827F70EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F70F0: 4BDE2CA1  bl 0x825d9d90
	ctx.lr = 0x827F70F4;
	sub_825D9D90(ctx, base);
	// 827F70F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827F70F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F70FC: 388BDBAC  addi r4, r11, -0x2454
	ctx.r[4].s64 = ctx.r[11].s64 + -9300;
	// 827F7100: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7104: 4BDE2D0D  bl 0x825d9e10
	ctx.lr = 0x827F7108;
	sub_825D9E10(ctx, base);
	// 827F7108: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827F710C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7110: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 827F7114: 388B664C  addi r4, r11, 0x664c
	ctx.r[4].s64 = ctx.r[11].s64 + 26188;
	// 827F7118: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F711C: 4BDE2CF5  bl 0x825d9e10
	ctx.lr = 0x827F7120;
	sub_825D9E10(ctx, base);
	// 827F7120: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7124: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 827F7128: 388B4148  addi r4, r11, 0x4148
	ctx.r[4].s64 = ctx.r[11].s64 + 16712;
	// 827F712C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7130: 4BDE2CE1  bl 0x825d9e10
	ctx.lr = 0x827F7134;
	sub_825D9E10(ctx, base);
	// 827F7134: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 827F7138: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F713C: 388BA14C  addi r4, r11, -0x5eb4
	ctx.r[4].s64 = ctx.r[11].s64 + -24244;
	// 827F7140: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7144: 4BDE2D05  bl 0x825d9e48
	ctx.lr = 0x827F7148;
	sub_825D9E48(ctx, base);
	// 827F7148: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F714C: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 827F7150: 388B6640  addi r4, r11, 0x6640
	ctx.r[4].s64 = ctx.r[11].s64 + 26176;
	// 827F7154: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7158: 4BDE2CD1  bl 0x825d9e28
	ctx.lr = 0x827F715C;
	sub_825D9E28(ctx, base);
	// 827F715C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F7160: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 827F7164: 388BEB64  addi r4, r11, -0x149c
	ctx.r[4].s64 = ctx.r[11].s64 + -5276;
	// 827F7168: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F716C: 4BDE2CBD  bl 0x825d9e28
	ctx.lr = 0x827F7170;
	sub_825D9E28(ctx, base);
	// 827F7170: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7174: 38BF0024  addi r5, r31, 0x24
	ctx.r[5].s64 = ctx.r[31].s64 + 36;
	// 827F7178: 388B6630  addi r4, r11, 0x6630
	ctx.r[4].s64 = ctx.r[11].s64 + 26160;
	// 827F717C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7180: 4BDE2CA9  bl 0x825d9e28
	ctx.lr = 0x827F7184;
	sub_825D9E28(ctx, base);
	// 827F7184: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7188: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 827F718C: 388B6620  addi r4, r11, 0x6620
	ctx.r[4].s64 = ctx.r[11].s64 + 26144;
	// 827F7190: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7194: 4BDE2C95  bl 0x825d9e28
	ctx.lr = 0x827F7198;
	sub_825D9E28(ctx, base);
	// 827F7198: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F719C: 38BF002C  addi r5, r31, 0x2c
	ctx.r[5].s64 = ctx.r[31].s64 + 44;
	// 827F71A0: 388B6614  addi r4, r11, 0x6614
	ctx.r[4].s64 = ctx.r[11].s64 + 26132;
	// 827F71A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F71A8: 4BDE2C81  bl 0x825d9e28
	ctx.lr = 0x827F71AC;
	sub_825D9E28(ctx, base);
	// 827F71AC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F71B0: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 827F71B4: 388B660C  addi r4, r11, 0x660c
	ctx.r[4].s64 = ctx.r[11].s64 + 26124;
	// 827F71B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F71BC: 4BDE2C6D  bl 0x825d9e28
	ctx.lr = 0x827F71C0;
	sub_825D9E28(ctx, base);
	// 827F71C0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F71C4: 38BF0034  addi r5, r31, 0x34
	ctx.r[5].s64 = ctx.r[31].s64 + 52;
	// 827F71C8: 388B4640  addi r4, r11, 0x4640
	ctx.r[4].s64 = ctx.r[11].s64 + 17984;
	// 827F71CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F71D0: 4BDE2C59  bl 0x825d9e28
	ctx.lr = 0x827F71D4;
	sub_825D9E28(ctx, base);
	// 827F71D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827F71D8: 38BF0038  addi r5, r31, 0x38
	ctx.r[5].s64 = ctx.r[31].s64 + 56;
	// 827F71DC: 388B168C  addi r4, r11, 0x168c
	ctx.r[4].s64 = ctx.r[11].s64 + 5772;
	// 827F71E0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F71E4: 4BDE2C2D  bl 0x825d9e10
	ctx.lr = 0x827F71E8;
	sub_825D9E10(ctx, base);
	// 827F71E8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F71EC: 38BF0044  addi r5, r31, 0x44
	ctx.r[5].s64 = ctx.r[31].s64 + 68;
	// 827F71F0: 388B6600  addi r4, r11, 0x6600
	ctx.r[4].s64 = ctx.r[11].s64 + 26112;
	// 827F71F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F71F8: 4BDE2C19  bl 0x825d9e10
	ctx.lr = 0x827F71FC;
	sub_825D9E10(ctx, base);
	// 827F71FC: 38BF0040  addi r5, r31, 0x40
	ctx.r[5].s64 = ctx.r[31].s64 + 64;
	// 827F7200: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7204: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7208: 388B65F8  addi r4, r11, 0x65f8
	ctx.r[4].s64 = ctx.r[11].s64 + 26104;
	// 827F720C: 4BDE2C05  bl 0x825d9e10
	ctx.lr = 0x827F7210;
	sub_825D9E10(ctx, base);
	// 827F7210: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7214: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 827F7218: 388B65E8  addi r4, r11, 0x65e8
	ctx.r[4].s64 = ctx.r[11].s64 + 26088;
	// 827F721C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7220: 4BDE2BF1  bl 0x825d9e10
	ctx.lr = 0x827F7224;
	sub_825D9E10(ctx, base);
	// 827F7224: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7228: 38BF004C  addi r5, r31, 0x4c
	ctx.r[5].s64 = ctx.r[31].s64 + 76;
	// 827F722C: 388B65DC  addi r4, r11, 0x65dc
	ctx.r[4].s64 = ctx.r[11].s64 + 26076;
	// 827F7230: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7234: 4BDE2BDD  bl 0x825d9e10
	ctx.lr = 0x827F7238;
	sub_825D9E10(ctx, base);
	// 827F7238: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F723C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7240: 4BDE2BC9  bl 0x825d9e08
	ctx.lr = 0x827F7244;
	sub_825D9E08(ctx, base);
	// 827F7244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7248: 485FC961  bl 0x82df3ba8
	ctx.lr = 0x827F724C;
	sub_82DF3BA8(ctx, base);
	// 827F724C: 83C10064  lwz r30, 0x64(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F7250: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F7254: 40820044  bne 0x827f7298
	if !ctx.cr[0].eq {
	pc = 0x827F7298; continue 'dispatch;
	}
	// 827F7258: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 827F725C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F7260: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 827F7264: 419A0024  beq cr6, 0x827f7288
	if ctx.cr[6].eq {
	pc = 0x827F7288; continue 'dispatch;
	}
	// 827F7268: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 827F726C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F7270: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F7274: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F7278: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F727C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F7280: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F7284: 4082FFE8  bne 0x827f726c
	if !ctx.cr[0].eq {
	pc = 0x827F726C; continue 'dispatch;
	}
	// 827F7288: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827F728C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F7290: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F7294: 4BFFFBFD  bl 0x827f6e90
	ctx.lr = 0x827F7298;
	sub_827F6E90(ctx, base);
	// 827F7298: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F729C: 4BDE2B2D  bl 0x825d9dc8
	ctx.lr = 0x827F72A0;
	sub_825D9DC8(ctx, base);
	// 827F72A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F72A4: 485FC185  bl 0x82df3428
	ctx.lr = 0x827F72A8;
	sub_82DF3428(ctx, base);
	// 827F72A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F72AC: 419A000C  beq cr6, 0x827f72b8
	if ctx.cr[6].eq {
	pc = 0x827F72B8; continue 'dispatch;
	}
	// 827F72B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F72B4: 4BAC95DD  bl 0x822c0890
	ctx.lr = 0x827F72B8;
	sub_822C0890(ctx, base);
	// 827F72B8: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F72BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F72C0: 419A0008  beq cr6, 0x827f72c8
	if ctx.cr[6].eq {
	pc = 0x827F72C8; continue 'dispatch;
	}
	// 827F72C4: 4BAC95CD  bl 0x822c0890
	ctx.lr = 0x827F72C8;
	sub_822C0890(ctx, base);
	// 827F72C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827F72CC: 489B0EEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F72D0 size=88
    let mut pc: u32 = 0x827F72D0;
    'dispatch: loop {
        match pc {
            0x827F72D0 => {
    //   block [0x827F72D0..0x827F7328)
	// 827F72D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F72D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F72D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F72DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F72E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F72E4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F72E8: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F72EC: 396B66B4  addi r11, r11, 0x66b4
	ctx.r[11].s64 = ctx.r[11].s64 + 26292;
	// 827F72F0: 394A66A0  addi r10, r10, 0x66a0
	ctx.r[10].s64 = ctx.r[10].s64 + 26272;
	// 827F72F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F72F8: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 827F72FC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F7300: 4BDB8E11  bl 0x825b0110
	ctx.lr = 0x827F7304;
	sub_825B0110(ctx, base);
	// 827F7304: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827F7308: 4BDB8E09  bl 0x825b0110
	ctx.lr = 0x827F730C;
	sub_825B0110(ctx, base);
	// 827F730C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7310: 4BFFA731  bl 0x827f1a40
	ctx.lr = 0x827F7314;
	sub_827F1A40(ctx, base);
	// 827F7314: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F7318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F731C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7320: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F7324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F7328 size=8
    let mut pc: u32 = 0x827F7328;
    'dispatch: loop {
        match pc {
            0x827F7328 => {
    //   block [0x827F7328..0x827F7330)
	// 827F7328: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827F732C: 48000004  b 0x827f7330
	sub_827F7330(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7330 size=76
    let mut pc: u32 = 0x827F7330;
    'dispatch: loop {
        match pc {
            0x827F7330 => {
    //   block [0x827F7330..0x827F737C)
	// 827F7330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F7338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F733C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F7340: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F7348: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F734C: 4BFFFF85  bl 0x827f72d0
	ctx.lr = 0x827F7350;
	sub_827F72D0(ctx, base);
	// 827F7350: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F7354: 4182000C  beq 0x827f7360
	if ctx.cr[0].eq {
	pc = 0x827F7360; continue 'dispatch;
	}
	// 827F7358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F735C: 485FB07D  bl 0x82df23d8
	ctx.lr = 0x827F7360;
	sub_82DF23D8(ctx, base);
	// 827F7360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7364: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F7368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7370: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F7374: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F7378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7380 size=104
    let mut pc: u32 = 0x827F7380;
    'dispatch: loop {
        match pc {
            0x827F7380 => {
    //   block [0x827F7380..0x827F73E8)
	// 827F7380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F7388: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F738C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F7390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F7398: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F739C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F73A0: 485FC089  bl 0x82df3428
	ctx.lr = 0x827F73A4;
	sub_82DF3428(ctx, base);
	// 827F73A4: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 827F73A8: 485FC081  bl 0x82df3428
	ctx.lr = 0x827F73AC;
	sub_82DF3428(ctx, base);
	// 827F73AC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 827F73B0: 485FC079  bl 0x82df3428
	ctx.lr = 0x827F73B4;
	sub_82DF3428(ctx, base);
	// 827F73B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F73B8: 4BFFEF31  bl 0x827f62e8
	ctx.lr = 0x827F73BC;
	sub_827F62E8(ctx, base);
	// 827F73BC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F73C0: 4182000C  beq 0x827f73cc
	if ctx.cr[0].eq {
	pc = 0x827F73CC; continue 'dispatch;
	}
	// 827F73C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F73C8: 4BAC8EA1  bl 0x822c0268
	ctx.lr = 0x827F73CC;
	sub_822C0268(ctx, base);
	// 827F73CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F73D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F73D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F73D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F73DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F73E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F73E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F73E8 size=184
    let mut pc: u32 = 0x827F73E8;
    'dispatch: loop {
        match pc {
            0x827F73E8 => {
    //   block [0x827F73E8..0x827F74A0)
	// 827F73E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F73EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F73F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F73F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F73F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F73FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F7400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7404: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F7408: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F740C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7410: 4BAC9529  bl 0x822c0938
	ctx.lr = 0x827F7414;
	sub_822C0938(ctx, base);
	// 827F7414: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F7418: 41820028  beq 0x827f7440
	if ctx.cr[0].eq {
	pc = 0x827F7440; continue 'dispatch;
	}
	// 827F741C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7420: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F7424: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F7428: 392B65A0  addi r9, r11, 0x65a0
	ctx.r[9].s64 = ctx.r[11].s64 + 26016;
	// 827F742C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F7430: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F7434: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F7438: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F743C: 48000008  b 0x827f7444
	pc = 0x827F7444; continue 'dispatch;
	// 827F7440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7444: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7448: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F744C: 409A0038  bne cr6, 0x827f7484
	if !ctx.cr[6].eq {
	pc = 0x827F7484; continue 'dispatch;
	}
	// 827F7450: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F7454: 419A0010  beq cr6, 0x827f7464
	if ctx.cr[6].eq {
	pc = 0x827F7464; continue 'dispatch;
	}
	// 827F7458: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F745C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7460: 4BFFFF21  bl 0x827f7380
	ctx.lr = 0x827F7464;
	sub_827F7380(ctx, base);
	// 827F7464: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F7468: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F746C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7470: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F7474: 816BCD4C  lwz r11, -0x32b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12980 as u32) ) } as u64;
	// 827F7478: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F747C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F7480: 4BAC8B81  bl 0x822c0000
	ctx.lr = 0x827F7484;
	sub_822C0000(ctx, base);
	// 827F7484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F7488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F748C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F7490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F7498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F749C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F74A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F74A0 size=12
    let mut pc: u32 = 0x827F74A0;
    'dispatch: loop {
        match pc {
            0x827F74A0 => {
    //   block [0x827F74A0..0x827F74AC)
	// 827F74A0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F74A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F74A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F74AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F74AC size=8
    let mut pc: u32 = 0x827F74AC;
    'dispatch: loop {
        match pc {
            0x827F74AC => {
    //   block [0x827F74AC..0x827F74B4)
	// 827F74AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F74B0: 4BFFFED0  b 0x827f7380
	sub_827F7380(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F74B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F74B4 size=4
    let mut pc: u32 = 0x827F74B4;
    'dispatch: loop {
        match pc {
            0x827F74B4 => {
    //   block [0x827F74B4..0x827F74B8)
	// 827F74B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F74B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F74B8 size=488
    let mut pc: u32 = 0x827F74B8;
    'dispatch: loop {
        match pc {
            0x827F74B8 => {
    //   block [0x827F74B8..0x827F76A0)
	// 827F74B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F74BC: 489B0CAD  bl 0x831a8168
	ctx.lr = 0x827F74C0;
	sub_831A8130(ctx, base);
	// 827F74C0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F74C4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F74C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F74CC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827F74D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F74D4: 388B6658  addi r4, r11, 0x6658
	ctx.r[4].s64 = ctx.r[11].s64 + 26200;
	// 827F74D8: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 827F74DC: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 827F74E0: 4BAC8EF9  bl 0x822c03d8
	ctx.lr = 0x827F74E4;
	sub_822C03D8(ctx, base);
	// 827F74E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F74E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F74EC: 41820010  beq 0x827f74fc
	if ctx.cr[0].eq {
	pc = 0x827F74FC; continue 'dispatch;
	}
	// 827F74F0: 4BFFEE51  bl 0x827f6340
	ctx.lr = 0x827F74F4;
	sub_827F6340(ctx, base);
	// 827F74F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F74F8: 48000008  b 0x827f7500
	pc = 0x827F7500; continue 'dispatch;
	// 827F74FC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 827F7500: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 827F7504: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F7508: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 827F750C: 4BFFFEDD  bl 0x827f73e8
	ctx.lr = 0x827F7510;
	sub_827F73E8(ctx, base);
	// 827F7510: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F7514: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F7518: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 827F751C: 4BAC8AE5  bl 0x822c0000
	ctx.lr = 0x827F7520;
	sub_822C0000(ctx, base);
	// 827F7520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F7524: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7528: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 827F752C: 485FC4DD  bl 0x82df3a08
	ctx.lr = 0x827F7530;
	sub_82DF3A08(ctx, base);
	// 827F7530: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7534: 4BDE285D  bl 0x825d9d90
	ctx.lr = 0x827F7538;
	sub_825D9D90(ctx, base);
	// 827F7538: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827F753C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F7540: 388BDBAC  addi r4, r11, -0x2454
	ctx.r[4].s64 = ctx.r[11].s64 + -9300;
	// 827F7544: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7548: 4BDE28C9  bl 0x825d9e10
	ctx.lr = 0x827F754C;
	sub_825D9E10(ctx, base);
	// 827F754C: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827F7550: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7554: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F7558: 388B671C  addi r4, r11, 0x671c
	ctx.r[4].s64 = ctx.r[11].s64 + 26396;
	// 827F755C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7560: 4BDE28B1  bl 0x825d9e10
	ctx.lr = 0x827F7564;
	sub_825D9E10(ctx, base);
	// 827F7564: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7568: 38BF0004  addi r5, r31, 4
	ctx.r[5].s64 = ctx.r[31].s64 + 4;
	// 827F756C: 388B670C  addi r4, r11, 0x670c
	ctx.r[4].s64 = ctx.r[11].s64 + 26380;
	// 827F7570: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7574: 4BDE289D  bl 0x825d9e10
	ctx.lr = 0x827F7578;
	sub_825D9E10(ctx, base);
	// 827F7578: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 827F757C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827F7580: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 827F7584: 396B6FF8  addi r11, r11, 0x6ff8
	ctx.r[11].s64 = ctx.r[11].s64 + 28664;
	// 827F7588: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 827F758C: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 827F7590: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 827F7594: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 827F7598: E8A10070  ld r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 827F759C: 93C10080  stw r30, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 827F75A0: 4BFFF669  bl 0x827f6c08
	ctx.lr = 0x827F75A4;
	sub_827F6C08(ctx, base);
	// 827F75A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 827F75A8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 827F75AC: 388BE1C4  addi r4, r11, -0x1e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -7740;
	// 827F75B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F75B4: 4BDE292D  bl 0x825d9ee0
	ctx.lr = 0x827F75B8;
	sub_825D9EE0(ctx, base);
	// 827F75B8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F75BC: 38BF0034  addi r5, r31, 0x34
	ctx.r[5].s64 = ctx.r[31].s64 + 52;
	// 827F75C0: 388B2D38  addi r4, r11, 0x2d38
	ctx.r[4].s64 = ctx.r[11].s64 + 11576;
	// 827F75C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F75C8: 4BDE2849  bl 0x825d9e10
	ctx.lr = 0x827F75CC;
	sub_825D9E10(ctx, base);
	// 827F75CC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F75D0: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 827F75D4: 388B6700  addi r4, r11, 0x6700
	ctx.r[4].s64 = ctx.r[11].s64 + 26368;
	// 827F75D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F75DC: 4BDE2835  bl 0x825d9e10
	ctx.lr = 0x827F75E0;
	sub_825D9E10(ctx, base);
	// 827F75E0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F75E4: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 827F75E8: 388B66F0  addi r4, r11, 0x66f0
	ctx.r[4].s64 = ctx.r[11].s64 + 26352;
	// 827F75EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F75F0: 4BDE2821  bl 0x825d9e10
	ctx.lr = 0x827F75F4;
	sub_825D9E10(ctx, base);
	// 827F75F4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F75F8: 38BF0038  addi r5, r31, 0x38
	ctx.r[5].s64 = ctx.r[31].s64 + 56;
	// 827F75FC: 388B66DC  addi r4, r11, 0x66dc
	ctx.r[4].s64 = ctx.r[11].s64 + 26332;
	// 827F7600: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7604: 4BDE280D  bl 0x825d9e10
	ctx.lr = 0x827F7608;
	sub_825D9E10(ctx, base);
	// 827F7608: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F760C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7610: 4BDE27F9  bl 0x825d9e08
	ctx.lr = 0x827F7614;
	sub_825D9E08(ctx, base);
	// 827F7614: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7618: 485FC591  bl 0x82df3ba8
	ctx.lr = 0x827F761C;
	sub_82DF3BA8(ctx, base);
	// 827F761C: 83C10064  lwz r30, 0x64(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F7620: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F7624: 40820044  bne 0x827f7668
	if !ctx.cr[0].eq {
	pc = 0x827F7668; continue 'dispatch;
	}
	// 827F7628: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 827F762C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F7630: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 827F7634: 419A0024  beq cr6, 0x827f7658
	if ctx.cr[6].eq {
	pc = 0x827F7658; continue 'dispatch;
	}
	// 827F7638: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 827F763C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F7640: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F7644: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F7648: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F764C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F7650: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F7654: 4082FFE8  bne 0x827f763c
	if !ctx.cr[0].eq {
	pc = 0x827F763C; continue 'dispatch;
	}
	// 827F7658: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 827F765C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F7660: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F7664: 4BFFF745  bl 0x827f6da8
	ctx.lr = 0x827F7668;
	sub_827F6DA8(ctx, base);
	// 827F7668: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F766C: 4BDE275D  bl 0x825d9dc8
	ctx.lr = 0x827F7670;
	sub_825D9DC8(ctx, base);
	// 827F7670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7674: 485FBDB5  bl 0x82df3428
	ctx.lr = 0x827F7678;
	sub_82DF3428(ctx, base);
	// 827F7678: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F767C: 419A000C  beq cr6, 0x827f7688
	if ctx.cr[6].eq {
	pc = 0x827F7688; continue 'dispatch;
	}
	// 827F7680: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F7684: 4BAC920D  bl 0x822c0890
	ctx.lr = 0x827F7688;
	sub_822C0890(ctx, base);
	// 827F7688: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F768C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F7690: 419A0008  beq cr6, 0x827f7698
	if ctx.cr[6].eq {
	pc = 0x827F7698; continue 'dispatch;
	}
	// 827F7694: 4BAC91FD  bl 0x822c0890
	ctx.lr = 0x827F7698;
	sub_822C0890(ctx, base);
	// 827F7698: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827F769C: 489B0B1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F76A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F76A0 size=208
    let mut pc: u32 = 0x827F76A0;
    'dispatch: loop {
        match pc {
            0x827F76A0 => {
    //   block [0x827F76A0..0x827F7770)
	// 827F76A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F76A4: 489B0AC5  bl 0x831a8168
	ctx.lr = 0x827F76A8;
	sub_831A8130(ctx, base);
	// 827F76A8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F76AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F76B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F76B4: 4BFFA345  bl 0x827f19f8
	ctx.lr = 0x827F76B8;
	sub_827F19F8(ctx, base);
	// 827F76B8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F76BC: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F76C0: 396B66B4  addi r11, r11, 0x66b4
	ctx.r[11].s64 = ctx.r[11].s64 + 26292;
	// 827F76C4: 394A66A0  addi r10, r10, 0x66a0
	ctx.r[10].s64 = ctx.r[10].s64 + 26272;
	// 827F76C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F76CC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827F76D0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F76D4: 4BFA4525  bl 0x8279bbf8
	ctx.lr = 0x827F76D8;
	sub_8279BBF8(ctx, base);
	// 827F76D8: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 827F76DC: 4BFA451D  bl 0x8279bbf8
	ctx.lr = 0x827F76E0;
	sub_8279BBF8(ctx, base);
	// 827F76E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 827F76E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F76E8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 827F76EC: 396B74B8  addi r11, r11, 0x74b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29880;
	// 827F76F0: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 827F76F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F76F8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 827F76FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F7700: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827F7704: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 827F7708: 4BFFF871  bl 0x827f6f78
	ctx.lr = 0x827F770C;
	sub_827F6F78(ctx, base);
	// 827F770C: 3F80832B  lis r28, -0x7cd5
	ctx.r[28].s64 = -2094333952;
	// 827F7710: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F7714: 397CCD44  addi r11, r28, -0x32bc
	ctx.r[11].s64 = ctx.r[28].s64 + -12988;
	// 827F7718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F771C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 827F7720: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7724: 4BFFA46D  bl 0x827f1b90
	ctx.lr = 0x827F7728;
	sub_827F1B90(ctx, base);
	// 827F7728: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 827F772C: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 827F7730: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F7734: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 827F7738: 396B7078  addi r11, r11, 0x7078
	ctx.r[11].s64 = ctx.r[11].s64 + 28792;
	// 827F773C: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 827F7740: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 827F7744: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F7748: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 827F774C: 4BFFF82D  bl 0x827f6f78
	ctx.lr = 0x827F7750;
	sub_827F6F78(ctx, base);
	// 827F7750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7754: 80BCCD44  lwz r5, -0x32bc(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-12988 as u32) ) } as u64;
	// 827F7758: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 827F775C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F7760: 4BFFA431  bl 0x827f1b90
	ctx.lr = 0x827F7764;
	sub_827F1B90(ctx, base);
	// 827F7764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7768: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F776C: 489B0A4C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7770 size=120
    let mut pc: u32 = 0x827F7770;
    'dispatch: loop {
        match pc {
            0x827F7770 => {
    //   block [0x827F7770..0x827F77E8)
	// 827F7770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7774: 489B09F9  bl 0x831a816c
	ctx.lr = 0x827F7778;
	sub_831A8130(ctx, base);
	// 827F7778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F777C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7780: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F7784: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F7788: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F778C: 388B6658  addi r4, r11, 0x6658
	ctx.r[4].s64 = ctx.r[11].s64 + 26200;
	// 827F7790: 38A00036  li r5, 0x36
	ctx.r[5].s64 = 54;
	// 827F7794: 386000D8  li r3, 0xd8
	ctx.r[3].s64 = 216;
	// 827F7798: 485FAC51  bl 0x82df23e8
	ctx.lr = 0x827F779C;
	sub_82DF23E8(ctx, base);
	// 827F779C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F77A0: 41820014  beq 0x827f77b4
	if ctx.cr[0].eq {
	pc = 0x827F77B4; continue 'dispatch;
	}
	// 827F77A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F77A8: 4BFFFEF9  bl 0x827f76a0
	ctx.lr = 0x827F77AC;
	sub_827F76A0(ctx, base);
	// 827F77AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F77B0: 48000008  b 0x827f77b8
	pc = 0x827F77B8; continue 'dispatch;
	// 827F77B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F77B8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F77BC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827F77C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F77C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F77C8: 4BFFECF1  bl 0x827f64b8
	ctx.lr = 0x827F77CC;
	sub_827F64B8(ctx, base);
	// 827F77CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F77D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F77D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F77D8: 4BAC8829  bl 0x822c0000
	ctx.lr = 0x827F77DC;
	sub_822C0000(ctx, base);
	// 827F77DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F77E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F77E4: 489B09D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F77E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F77E8 size=92
    let mut pc: u32 = 0x827F77E8;
    'dispatch: loop {
        match pc {
            0x827F77E8 => {
    //   block [0x827F77E8..0x827F7844)
	// 827F77E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F77EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F77F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F77F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F77F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F77FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F7800: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827F7804: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7808: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F780C: 419A0020  beq cr6, 0x827f782c
	if ctx.cr[6].eq {
	pc = 0x827F782C; continue 'dispatch;
	}
	// 827F7810: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F7814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7818: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F781C: 4BAD94A5  bl 0x822d0cc0
	ctx.lr = 0x827F7820;
	sub_822D0CC0(ctx, base);
	// 827F7820: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F7824: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7828: 4BAD88C1  bl 0x822d00e8
	ctx.lr = 0x827F782C;
	sub_822D00E8(ctx, base);
	// 827F782C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F7830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F7834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7838: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F783C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F7840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F7848 size=108
    let mut pc: u32 = 0x827F7848;
    'dispatch: loop {
        match pc {
            0x827F7848 => {
    //   block [0x827F7848..0x827F78B4)
	// 827F7848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F784C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F7850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F7854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F7858: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827F785C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7860: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F7864: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827F7868: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F786C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F7870: 419A0028  beq cr6, 0x827f7898
	if ctx.cr[6].eq {
	pc = 0x827F7898; continue 'dispatch;
	}
	// 827F7874: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F7878: C3E50004  lfs f31, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F787C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7880: C02B9450  lfs f1, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F7884: 4BAD943D  bl 0x822d0cc0
	ctx.lr = 0x827F7888;
	sub_822D0CC0(ctx, base);
	// 827F7888: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F788C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7890: 4BAD8859  bl 0x822d00e8
	ctx.lr = 0x827F7894;
	sub_822D00E8(ctx, base);
	// 827F7894: D3FE0004  stfs f31, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F7898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F789C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F78A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F78A4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827F78A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F78AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F78B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F78B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F78B8 size=220
    let mut pc: u32 = 0x827F78B8;
    'dispatch: loop {
        match pc {
            0x827F78B8 => {
    //   block [0x827F78B8..0x827F7994)
	// 827F78B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F78BC: 489B08A9  bl 0x831a8164
	ctx.lr = 0x827F78C0;
	sub_831A8130(ctx, base);
	// 827F78C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F78C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F78C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F78CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F78D0: 4BAECF79  bl 0x822e4848
	ctx.lr = 0x827F78D4;
	sub_822E4848(ctx, base);
	// 827F78D4: 8381005C  lwz r28, 0x5c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F78D8: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F78DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827F78E0: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 827F78E4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827F78E8: 419A0024  beq cr6, 0x827f790c
	if ctx.cr[6].eq {
	pc = 0x827F790C; continue 'dispatch;
	}
	// 827F78EC: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 827F78F0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F78F4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F78F8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F78FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F7900: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F7904: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F7908: 4082FFE8  bne 0x827f78f0
	if !ctx.cr[0].eq {
	pc = 0x827F78F0; continue 'dispatch;
	}
	// 827F790C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F7910: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 827F7914: 488116A5  bl 0x83008fb8
	ctx.lr = 0x827F7918;
	sub_83008FB8(ctx, base);
	// 827F7918: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F791C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827F7920: 388B672C  addi r4, r11, 0x672c
	ctx.r[4].s64 = ctx.r[11].s64 + 26412;
	// 827F7924: 38A000AC  li r5, 0xac
	ctx.r[5].s64 = 172;
	// 827F7928: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F792C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 827F7930: 4865F6B9  bl 0x82e56fe8
	ctx.lr = 0x827F7934;
	sub_82E56FE8(ctx, base);
	// 827F7934: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F7938: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F793C: 419A0008  beq cr6, 0x827f7944
	if ctx.cr[6].eq {
	pc = 0x827F7944; continue 'dispatch;
	}
	// 827F7940: 4BAC8F51  bl 0x822c0890
	ctx.lr = 0x827F7944;
	sub_822C0890(ctx, base);
	// 827F7944: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F7948: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F794C: 80BE0020  lwz r5, 0x20(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F7950: 4BAD7EE9  bl 0x822cf838
	ctx.lr = 0x827F7954;
	sub_822CF838(ctx, base);
	// 827F7954: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F7958: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F795C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F7960: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7964: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7968: 4BACCAF9  bl 0x822c4460
	ctx.lr = 0x827F796C;
	sub_822C4460(ctx, base);
	// 827F796C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F7970: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F7974: 419A0008  beq cr6, 0x827f797c
	if ctx.cr[6].eq {
	pc = 0x827F797C; continue 'dispatch;
	}
	// 827F7978: 4BAC8F19  bl 0x822c0890
	ctx.lr = 0x827F797C;
	sub_822C0890(ctx, base);
	// 827F797C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827F7980: 419A000C  beq cr6, 0x827f798c
	if ctx.cr[6].eq {
	pc = 0x827F798C; continue 'dispatch;
	}
	// 827F7984: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F7988: 4BAC8F09  bl 0x822c0890
	ctx.lr = 0x827F798C;
	sub_822C0890(ctx, base);
	// 827F798C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F7990: 489B0824  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7998 size=188
    let mut pc: u32 = 0x827F7998;
    'dispatch: loop {
        match pc {
            0x827F7998 => {
    //   block [0x827F7998..0x827F7A54)
	// 827F7998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F799C: 489B07C9  bl 0x831a8164
	ctx.lr = 0x827F79A0;
	sub_831A8130(ctx, base);
	// 827F79A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F79A4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F79A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F79AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F79B0: 4BF0B8D1  bl 0x82703280
	ctx.lr = 0x827F79B4;
	sub_82703280(ctx, base);
	// 827F79B4: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F79B8: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F79BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F79C0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 827F79C4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 827F79C8: 419A0024  beq cr6, 0x827f79ec
	if ctx.cr[6].eq {
	pc = 0x827F79EC; continue 'dispatch;
	}
	// 827F79CC: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827F79D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F79D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F79D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F79DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F79E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F79E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F79E8: 4082FFE8  bne 0x827f79d0
	if !ctx.cr[0].eq {
	pc = 0x827F79D0; continue 'dispatch;
	}
	// 827F79EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F79F0: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 827F79F4: 488115C5  bl 0x83008fb8
	ctx.lr = 0x827F79F8;
	sub_83008FB8(ctx, base);
	// 827F79F8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F79FC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827F7A00: 388B672C  addi r4, r11, 0x672c
	ctx.r[4].s64 = ctx.r[11].s64 + 26412;
	// 827F7A04: 38A000B7  li r5, 0xb7
	ctx.r[5].s64 = 183;
	// 827F7A08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F7A0C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 827F7A10: 4865F5D9  bl 0x82e56fe8
	ctx.lr = 0x827F7A14;
	sub_82E56FE8(ctx, base);
	// 827F7A14: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F7A18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F7A1C: 419A0008  beq cr6, 0x827f7a24
	if ctx.cr[6].eq {
	pc = 0x827F7A24; continue 'dispatch;
	}
	// 827F7A20: 4BAC8E71  bl 0x822c0890
	ctx.lr = 0x827F7A24;
	sub_822C0890(ctx, base);
	// 827F7A24: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F7A28: 397D0018  addi r11, r29, 0x18
	ctx.r[11].s64 = ctx.r[29].s64 + 24;
	// 827F7A2C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 827F7A30: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F7A34: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F7A38: 4BACCA29  bl 0x822c4460
	ctx.lr = 0x827F7A3C;
	sub_822C4460(ctx, base);
	// 827F7A3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F7A40: 419A000C  beq cr6, 0x827f7a4c
	if ctx.cr[6].eq {
	pc = 0x827F7A4C; continue 'dispatch;
	}
	// 827F7A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7A48: 4BAC8E49  bl 0x822c0890
	ctx.lr = 0x827F7A4C;
	sub_822C0890(ctx, base);
	// 827F7A4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827F7A50: 489B0764  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F7A58 size=292
    let mut pc: u32 = 0x827F7A58;
    'dispatch: loop {
        match pc {
            0x827F7A58 => {
    //   block [0x827F7A58..0x827F7B7C)
	// 827F7A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7A5C: 489B0701  bl 0x831a815c
	ctx.lr = 0x827F7A60;
	sub_831A8130(ctx, base);
	// 827F7A60: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7A64: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 827F7A68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7A6C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F7A70: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 827F7A74: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 827F7A78: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827F7A7C: 93B90004  stw r29, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 827F7A80: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 827F7A84: 93B90008  stw r29, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 827F7A88: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 827F7A8C: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 827F7A90: 3BF90004  addi r31, r25, 4
	ctx.r[31].s64 = ctx.r[25].s64 + 4;
	// 827F7A94: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 827F7A98: 486A8C99  bl 0x82ea0730
	ctx.lr = 0x827F7A9C;
	sub_82EA0730(ctx, base);
	// 827F7A9C: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 827F7AA0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 827F7AA4: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F7AA8: C02A6728  lfs f1, 0x6728(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26408 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F7AAC: 48722745  bl 0x82f1a1f0
	ctx.lr = 0x827F7AB0;
	sub_82F1A1F0(ctx, base);
	// 827F7AB0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 827F7AB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F7AB8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827F7ABC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F7AC0: 80AB679C  lwz r5, 0x679c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26524 as u32) ) } as u64;
	// 827F7AC4: 808A67A4  lwz r4, 0x67a4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26532 as u32) ) } as u64;
	// 827F7AC8: 4BAFFCD9  bl 0x822f77a0
	ctx.lr = 0x827F7ACC;
	sub_822F77A0(ctx, base);
	// 827F7ACC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F7AD0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F7AD4: 4BAED3ED  bl 0x822e4ec0
	ctx.lr = 0x827F7AD8;
	sub_822E4EC0(ctx, base);
	// 827F7AD8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F7ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7AE0: E89E0000  ld r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827F7AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F7AE8: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827F7AEC: 4BC9479D  bl 0x8248c288
	ctx.lr = 0x827F7AF0;
	sub_8248C288(ctx, base);
	// 827F7AF0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7AF4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 827F7AF8: 388B672C  addi r4, r11, 0x672c
	ctx.r[4].s64 = ctx.r[11].s64 + 26412;
	// 827F7AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7B00: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 827F7B04: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 827F7B08: 4BAC88D1  bl 0x822c03d8
	ctx.lr = 0x827F7B0C;
	sub_822C03D8(ctx, base);
	// 827F7B0C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827F7B10: 41820034  beq 0x827f7b44
	if ctx.cr[0].eq {
	pc = 0x827F7B44; continue 'dispatch;
	}
	// 827F7B14: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827F7B18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7B1C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 827F7B20: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 827F7B24: 4BD179F5  bl 0x8250f518
	ctx.lr = 0x827F7B28;
	sub_8250F518(ctx, base);
	// 827F7B28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F7B2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F7B30: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827F7B34: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 827F7B38: 4BC9A6B9  bl 0x824921f0
	ctx.lr = 0x827F7B3C;
	sub_824921F0(ctx, base);
	// 827F7B3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F7B40: 48000008  b 0x827f7b48
	pc = 0x827F7B48; continue 'dispatch;
	// 827F7B44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F7B48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7B4C: 4BAED29D  bl 0x822e4de8
	ctx.lr = 0x827F7B50;
	sub_822E4DE8(ctx, base);
	// 827F7B50: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F7B54: 4182000C  beq 0x827f7b60
	if ctx.cr[0].eq {
	pc = 0x827F7B60; continue 'dispatch;
	}
	// 827F7B58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F7B5C: 485FA135  bl 0x82df1c90
	ctx.lr = 0x827F7B60;
	sub_82DF1C90(ctx, base);
	// 827F7B60: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F7B64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F7B68: 419A0008  beq cr6, 0x827f7b70
	if ctx.cr[6].eq {
	pc = 0x827F7B70; continue 'dispatch;
	}
	// 827F7B6C: 4BAF06FD  bl 0x822e8268
	ctx.lr = 0x827F7B70;
	sub_822E8268(ctx, base);
	// 827F7B70: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 827F7B74: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F7B78: 489B0634  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7B80 size=116
    let mut pc: u32 = 0x827F7B80;
    'dispatch: loop {
        match pc {
            0x827F7B80 => {
    //   block [0x827F7B80..0x827F7BF4)
	// 827F7B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7B84: 489B05E5  bl 0x831a8168
	ctx.lr = 0x827F7B88;
	sub_831A8130(ctx, base);
	// 827F7B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7B8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F7B90: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F7B94: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7B98: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7B9C: 48000048  b 0x827f7be4
	pc = 0x827F7BE4; continue 'dispatch;
	// 827F7BA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F7BA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F7BA8: 419A0034  beq cr6, 0x827f7bdc
	if ctx.cr[6].eq {
	pc = 0x827F7BDC; continue 'dispatch;
	}
	// 827F7BAC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 827F7BB0: 4BFE2421  bl 0x827d9fd0
	ctx.lr = 0x827F7BB4;
	sub_827D9FD0(ctx, base);
	// 827F7BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F7BB8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F7BBC: 4BD19425  bl 0x82510fe0
	ctx.lr = 0x827F7BC0;
	sub_82510FE0(ctx, base);
	// 827F7BC0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F7BC4: 419A0018  beq cr6, 0x827f7bdc
	if ctx.cr[6].eq {
	pc = 0x827F7BDC; continue 'dispatch;
	}
	// 827F7BC8: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 827F7BCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F7BD0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827F7BD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F7BD8: 4832D219  bl 0x82b24df0
	ctx.lr = 0x827F7BDC;
	sub_82B24DF0(ctx, base);
	// 827F7BDC: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7BE0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7BE4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F7BE8: 409AFFB8  bne cr6, 0x827f7ba0
	if !ctx.cr[6].eq {
	pc = 0x827F7BA0; continue 'dispatch;
	}
	// 827F7BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F7BF0: 489B05C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7BF8 size=412
    let mut pc: u32 = 0x827F7BF8;
    'dispatch: loop {
        match pc {
            0x827F7BF8 => {
    //   block [0x827F7BF8..0x827F7D94)
	// 827F7BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7BFC: 489B055D  bl 0x831a8158
	ctx.lr = 0x827F7C00;
	sub_831A8130(ctx, base);
	// 827F7C00: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7C04: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827F7C08: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827F7C0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F7C10: 3BABAD4C  addi r29, r11, -0x52b4
	ctx.r[29].s64 = ctx.r[11].s64 + -21172;
	// 827F7C14: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 827F7C18: 816AAD54  lwz r11, -0x52ac(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21164 as u32) ) } as u64;
	// 827F7C1C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 827F7C20: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 827F7C24: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827F7C28: 4082007C  bne 0x827f7ca4
	if !ctx.cr[0].eq {
	pc = 0x827F7CA4; continue 'dispatch;
	}
	// 827F7C2C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827F7C30: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 827F7C34: 916AAD54  stw r11, -0x52ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21164 as u32), ctx.r[11].u32 ) };
	// 827F7C38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7C3C: 8089679C  lwz r4, 0x679c(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26524 as u32) ) } as u64;
	// 827F7C40: 4BAED2F1  bl 0x822e4f30
	ctx.lr = 0x827F7C44;
	sub_822E4F30(ctx, base);
	// 827F7C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F7C48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F7C4C: 4BAED275  bl 0x822e4ec0
	ctx.lr = 0x827F7C50;
	sub_822E4EC0(ctx, base);
	// 827F7C50: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F7C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7C58: E89F0000  ld r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 827F7C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F7C60: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827F7C64: 4BC94625  bl 0x8248c288
	ctx.lr = 0x827F7C68;
	sub_8248C288(ctx, base);
	// 827F7C68: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 827F7C6C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F7C70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F7C74: 808B67A4  lwz r4, 0x67a4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26532 as u32) ) } as u64;
	// 827F7C78: 4BAED2B9  bl 0x822e4f30
	ctx.lr = 0x827F7C7C;
	sub_822E4F30(ctx, base);
	// 827F7C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F7C80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7C84: 4BAED23D  bl 0x822e4ec0
	ctx.lr = 0x827F7C88;
	sub_822E4EC0(ctx, base);
	// 827F7C88: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F7C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F7C90: E89F0000  ld r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 827F7C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F7C98: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827F7C9C: 4BC945ED  bl 0x8248c288
	ctx.lr = 0x827F7CA0;
	sub_8248C288(ctx, base);
	// 827F7CA0: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 827F7CA4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F7CA8: 4BC6E831  bl 0x824664d8
	ctx.lr = 0x827F7CAC;
	sub_824664D8(ctx, base);
	// 827F7CAC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 827F7CB0: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 827F7CB4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7CB8: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 827F7CBC: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 827F7CC0: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 827F7CC4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 827F7CC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F7CCC: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 827F7CD0: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 827F7CD4: 419A0040  beq cr6, 0x827f7d14
	if ctx.cr[6].eq {
	pc = 0x827F7D14; continue 'dispatch;
	}
	// 827F7CD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827F7CDC: 4BC9A29D  bl 0x82491f78
	ctx.lr = 0x827F7CE0;
	sub_82491F78(ctx, base);
	// 827F7CE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F7CE4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827F7CE8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7CEC: 4BC9A265  bl 0x82491f50
	ctx.lr = 0x827F7CF0;
	sub_82491F50(ctx, base);
	// 827F7CF0: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 827F7CF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F7CF8: 419A001C  beq cr6, 0x827f7d14
	if ctx.cr[6].eq {
	pc = 0x827F7D14; continue 'dispatch;
	}
	// 827F7CFC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827F7D00: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827F7D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F7D08: 4BFFFE79  bl 0x827f7b80
	ctx.lr = 0x827F7D0C;
	sub_827F7B80(ctx, base);
	// 827F7D0C: 83810068  lwz r28, 0x68(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 827F7D10: 83E10064  lwz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F7D14: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 827F7D18: 419A0028  beq cr6, 0x827f7d40
	if ctx.cr[6].eq {
	pc = 0x827F7D40; continue 'dispatch;
	}
	// 827F7D1C: 573E103A  slwi r30, r25, 2
	ctx.r[30].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 827F7D20: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7D24: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 827F7D28: 4BC93DF1  bl 0x8248bb18
	ctx.lr = 0x827F7D2C;
	sub_8248BB18(ctx, base);
	// 827F7D2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F7D30: 40820018  bne 0x827f7d48
	if !ctx.cr[0].eq {
	pc = 0x827F7D48; continue 'dispatch;
	}
	// 827F7D34: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 827F7D38: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 827F7D3C: 409AFFE4  bne cr6, 0x827f7d20
	if !ctx.cr[6].eq {
	pc = 0x827F7D20; continue 'dispatch;
	}
	// 827F7D40: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 827F7D44: 48000024  b 0x827f7d68
	pc = 0x827F7D68; continue 'dispatch;
	// 827F7D48: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F7D4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7D50: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 827F7D54: 396BD048  addi r11, r11, -0x2fb8
	ctx.r[11].s64 = ctx.r[11].s64 + -12216;
	// 827F7D58: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 827F7D5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F7D60: 4E800421  bctrl
	ctx.lr = 0x827F7D64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F7D64: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 827F7D68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F7D6C: 4BC72295  bl 0x8246a000
	ctx.lr = 0x827F7D70;
	sub_8246A000(ctx, base);
	// 827F7D70: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F7D74: 4BC3090D  bl 0x82428680
	ctx.lr = 0x827F7D78;
	sub_82428680(ctx, base);
	// 827F7D78: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F7D7C: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 827F7D80: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F7D84: 485FA405  bl 0x82df2188
	ctx.lr = 0x827F7D88;
	sub_82DF2188(ctx, base);
	// 827F7D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7D8C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827F7D90: 489B0418  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F7D98 size=76
    let mut pc: u32 = 0x827F7D98;
    'dispatch: loop {
        match pc {
            0x827F7D98 => {
    //   block [0x827F7D98..0x827F7DE4)
	// 827F7D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7D9C: 489B03D1  bl 0x831a816c
	ctx.lr = 0x827F7DA0;
	sub_831A8130(ctx, base);
	// 827F7DA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7DA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F7DA8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F7DAC: 4BFFFE4D  bl 0x827f7bf8
	ctx.lr = 0x827F7DB0;
	sub_827F7BF8(ctx, base);
	// 827F7DB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F7DB4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F7DB8: 41820020  beq 0x827f7dd8
	if ctx.cr[0].eq {
	pc = 0x827F7DD8; continue 'dispatch;
	}
	// 827F7DBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F7DC0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7DC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F7DC8: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F7DCC: 4BAD86FD  bl 0x822d04c8
	ctx.lr = 0x827F7DD0;
	sub_822D04C8(ctx, base);
	// 827F7DD0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7DD4: 4BAD815D  bl 0x822cff30
	ctx.lr = 0x827F7DD8;
	sub_822CFF30(ctx, base);
	// 827F7DD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F7DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F7DE0: 489B03DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7DE8 size=196
    let mut pc: u32 = 0x827F7DE8;
    'dispatch: loop {
        match pc {
            0x827F7DE8 => {
    //   block [0x827F7DE8..0x827F7EAC)
	// 827F7DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F7DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F7DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F7DF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7DFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F7E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7E04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F7E08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F7E0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7E10: 4BAC8B29  bl 0x822c0938
	ctx.lr = 0x827F7E14;
	sub_822C0938(ctx, base);
	// 827F7E14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F7E18: 41820028  beq 0x827f7e40
	if ctx.cr[0].eq {
	pc = 0x827F7E40; continue 'dispatch;
	}
	// 827F7E1C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7E20: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F7E24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F7E28: 392B676C  addi r9, r11, 0x676c
	ctx.r[9].s64 = ctx.r[11].s64 + 26476;
	// 827F7E2C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F7E30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F7E34: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F7E38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F7E3C: 48000008  b 0x827f7e44
	pc = 0x827F7E44; continue 'dispatch;
	// 827F7E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7E44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7E48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F7E4C: 409A0044  bne cr6, 0x827f7e90
	if !ctx.cr[6].eq {
	pc = 0x827F7E90; continue 'dispatch;
	}
	// 827F7E50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F7E54: 419A001C  beq cr6, 0x827f7e70
	if ctx.cr[6].eq {
	pc = 0x827F7E70; continue 'dispatch;
	}
	// 827F7E58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7E5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F7E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7E64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7E68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F7E6C: 4E800421  bctrl
	ctx.lr = 0x827F7E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F7E70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F7E74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F7E78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7E7C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F7E80: 816BD050  lwz r11, -0x2fb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12208 as u32) ) } as u64;
	// 827F7E84: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F7E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F7E8C: 4BAC8175  bl 0x822c0000
	ctx.lr = 0x827F7E90;
	sub_822C0000(ctx, base);
	// 827F7E90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F7E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F7E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F7E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7EA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F7EA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F7EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F7EB0 size=196
    let mut pc: u32 = 0x827F7EB0;
    'dispatch: loop {
        match pc {
            0x827F7EB0 => {
    //   block [0x827F7EB0..0x827F7F74)
	// 827F7EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F7EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F7EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F7EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7EC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F7EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7ECC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F7ED0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F7ED4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7ED8: 4BAC8A61  bl 0x822c0938
	ctx.lr = 0x827F7EDC;
	sub_822C0938(ctx, base);
	// 827F7EDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F7EE0: 41820028  beq 0x827f7f08
	if ctx.cr[0].eq {
	pc = 0x827F7F08; continue 'dispatch;
	}
	// 827F7EE4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F7EE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F7EEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F7EF0: 392B6780  addi r9, r11, 0x6780
	ctx.r[9].s64 = ctx.r[11].s64 + 26496;
	// 827F7EF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F7EF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F7EFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F7F00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F7F04: 48000008  b 0x827f7f0c
	pc = 0x827F7F0C; continue 'dispatch;
	// 827F7F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F7F0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F7F10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F7F14: 409A0044  bne cr6, 0x827f7f58
	if !ctx.cr[6].eq {
	pc = 0x827F7F58; continue 'dispatch;
	}
	// 827F7F18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F7F1C: 419A001C  beq cr6, 0x827f7f38
	if ctx.cr[6].eq {
	pc = 0x827F7F38; continue 'dispatch;
	}
	// 827F7F20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7F24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F7F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F7F2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F7F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F7F34: 4E800421  bctrl
	ctx.lr = 0x827F7F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F7F38: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F7F3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F7F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7F44: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F7F48: 816BD050  lwz r11, -0x2fb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12208 as u32) ) } as u64;
	// 827F7F4C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F7F50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F7F54: 4BAC80AD  bl 0x822c0000
	ctx.lr = 0x827F7F58;
	sub_822C0000(ctx, base);
	// 827F7F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F7F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F7F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F7F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7F68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F7F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F7F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F7F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F7F78 size=140
    let mut pc: u32 = 0x827F7F78;
    'dispatch: loop {
        match pc {
            0x827F7F78 => {
    //   block [0x827F7F78..0x827F8004)
	// 827F7F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F7F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F7F80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F7F84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F7F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F7F8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F7F90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7F94: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F7F98: 485FBA71  bl 0x82df3a08
	ctx.lr = 0x827F7F9C;
	sub_82DF3A08(ctx, base);
	// 827F7F9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F7FA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F7FA4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F7FA8: 4BC9A361  bl 0x82492308
	ctx.lr = 0x827F7FAC;
	sub_82492308(ctx, base);
	// 827F7FAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F7FB0: 485FB479  bl 0x82df3428
	ctx.lr = 0x827F7FB4;
	sub_82DF3428(ctx, base);
	// 827F7FB4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F7FB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F7FBC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F7FC0: 419A002C  beq cr6, 0x827f7fec
	if ctx.cr[6].eq {
	pc = 0x827F7FEC; continue 'dispatch;
	}
	// 827F7FC4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F7FC8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F7FCC: 3D008207  lis r8, -0x7df9
	ctx.r[8].s64 = -2113470464;
	// 827F7FD0: 80CA0010  lwz r6, 0x10(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F7FD4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 827F7FD8: 38886790  addi r4, r8, 0x6790
	ctx.r[4].s64 = ctx.r[8].s64 + 26512;
	// 827F7FDC: 38A0007F  li r5, 0x7f
	ctx.r[5].s64 = 127;
	// 827F7FE0: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827F7FE4: C02908A4  lfs f1, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F7FE8: 48660A59  bl 0x82e58a40
	ctx.lr = 0x827F7FEC;
	sub_82E58A40(ctx, base);
	// 827F7FEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F7FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F7FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F7FF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F7FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8008 size=108
    let mut pc: u32 = 0x827F8008;
    'dispatch: loop {
        match pc {
            0x827F8008 => {
    //   block [0x827F8008..0x827F8074)
	// 827F8008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F800C: 489B0161  bl 0x831a816c
	ctx.lr = 0x827F8010;
	sub_831A8130(ctx, base);
	// 827F8010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8018: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F801C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F8020: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8024: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F8028: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F802C: 419A0040  beq cr6, 0x827f806c
	if ctx.cr[6].eq {
	pc = 0x827F806C; continue 'dispatch;
	}
	// 827F8030: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F8034: 3BCA6790  addi r30, r10, 0x6790
	ctx.r[30].s64 = ctx.r[10].s64 + 26512;
	// 827F8038: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F803C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F8040: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827F8044: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F8048: 38A0008C  li r5, 0x8c
	ctx.r[5].s64 = 140;
	// 827F804C: 386A0028  addi r3, r10, 0x28
	ctx.r[3].s64 = ctx.r[10].s64 + 40;
	// 827F8050: 4865EF99  bl 0x82e56fe8
	ctx.lr = 0x827F8054;
	sub_82E56FE8(ctx, base);
	// 827F8054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8058: 4BCF1039  bl 0x824e9090
	ctx.lr = 0x827F805C;
	sub_824E9090(ctx, base);
	// 827F805C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F8060: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F8064: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F8068: 409AFFD0  bne cr6, 0x827f8038
	if !ctx.cr[6].eq {
	pc = 0x827F8038; continue 'dispatch;
	}
	// 827F806C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F8070: 489B014C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8078 size=128
    let mut pc: u32 = 0x827F8078;
    'dispatch: loop {
        match pc {
            0x827F8078 => {
    //   block [0x827F8078..0x827F80F8)
	// 827F8078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F807C: 489B00F1  bl 0x831a816c
	ctx.lr = 0x827F8080;
	sub_831A8130(ctx, base);
	// 827F8080: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827F8084: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F808C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F8090: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F8094: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8098: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F809C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F80A0: 419A004C  beq cr6, 0x827f80ec
	if ctx.cr[6].eq {
	pc = 0x827F80EC; continue 'dispatch;
	}
	// 827F80A4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F80A8: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F80AC: 3BCA6790  addi r30, r10, 0x6790
	ctx.r[30].s64 = ctx.r[10].s64 + 26512;
	// 827F80B0: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F80B4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F80B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F80BC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827F80C0: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F80C4: 38A00098  li r5, 0x98
	ctx.r[5].s64 = 152;
	// 827F80C8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F80CC: 386A0028  addi r3, r10, 0x28
	ctx.r[3].s64 = ctx.r[10].s64 + 40;
	// 827F80D0: 48660971  bl 0x82e58a40
	ctx.lr = 0x827F80D4;
	sub_82E58A40(ctx, base);
	// 827F80D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F80D8: 4BCF0FB9  bl 0x824e9090
	ctx.lr = 0x827F80DC;
	sub_824E9090(ctx, base);
	// 827F80DC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F80E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F80E4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F80E8: 409AFFCC  bne cr6, 0x827f80b4
	if !ctx.cr[6].eq {
	pc = 0x827F80B4; continue 'dispatch;
	}
	// 827F80EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F80F0: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827F80F4: 489B00C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F80F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F80F8 size=120
    let mut pc: u32 = 0x827F80F8;
    'dispatch: loop {
        match pc {
            0x827F80F8 => {
    //   block [0x827F80F8..0x827F8170)
	// 827F80F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F80FC: 489B0071  bl 0x831a816c
	ctx.lr = 0x827F8100;
	sub_831A8130(ctx, base);
	// 827F8100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8104: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F8108: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F810C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F8110: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F8114: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 827F8118: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 827F811C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 827F8120: 485FA2C9  bl 0x82df23e8
	ctx.lr = 0x827F8124;
	sub_82DF23E8(ctx, base);
	// 827F8124: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F8128: 41820014  beq 0x827f813c
	if ctx.cr[0].eq {
	pc = 0x827F813C; continue 'dispatch;
	}
	// 827F812C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8130: 4835AD51  bl 0x82b52e80
	ctx.lr = 0x827F8134;
	sub_82B52E80(ctx, base);
	// 827F8134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8138: 48000008  b 0x827f8140
	pc = 0x827F8140; continue 'dispatch;
	// 827F813C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F8140: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F8144: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827F8148: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F814C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F8150: 4BFFFC99  bl 0x827f7de8
	ctx.lr = 0x827F8154;
	sub_827F7DE8(ctx, base);
	// 827F8154: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F8158: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F815C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F8160: 4BAC7EA1  bl 0x822c0000
	ctx.lr = 0x827F8164;
	sub_822C0000(ctx, base);
	// 827F8164: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F8168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F816C: 489B0050  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8170 size=120
    let mut pc: u32 = 0x827F8170;
    'dispatch: loop {
        match pc {
            0x827F8170 => {
    //   block [0x827F8170..0x827F81E8)
	// 827F8170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8174: 489AFFF9  bl 0x831a816c
	ctx.lr = 0x827F8178;
	sub_831A8130(ctx, base);
	// 827F8178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F817C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F8180: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F8184: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F8188: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F818C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 827F8190: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 827F8194: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 827F8198: 485FA251  bl 0x82df23e8
	ctx.lr = 0x827F819C;
	sub_82DF23E8(ctx, base);
	// 827F819C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F81A0: 41820014  beq 0x827f81b4
	if ctx.cr[0].eq {
	pc = 0x827F81B4; continue 'dispatch;
	}
	// 827F81A4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F81A8: 4835A749  bl 0x82b528f0
	ctx.lr = 0x827F81AC;
	sub_82B528F0(ctx, base);
	// 827F81AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F81B0: 48000008  b 0x827f81b8
	pc = 0x827F81B8; continue 'dispatch;
	// 827F81B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F81B8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F81BC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827F81C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F81C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F81C8: 4BFFFCE9  bl 0x827f7eb0
	ctx.lr = 0x827F81CC;
	sub_827F7EB0(ctx, base);
	// 827F81CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F81D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F81D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F81D8: 4BAC7E29  bl 0x822c0000
	ctx.lr = 0x827F81DC;
	sub_822C0000(ctx, base);
	// 827F81DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F81E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F81E4: 489AFFD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F81E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F81E8 size=292
    let mut pc: u32 = 0x827F81E8;
    'dispatch: loop {
        match pc {
            0x827F81E8 => {
    //   block [0x827F81E8..0x827F830C)
	// 827F81E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F81EC: 489AFF7D  bl 0x831a8168
	ctx.lr = 0x827F81F0;
	sub_831A8130(ctx, base);
	// 827F81F0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827F81F4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F81F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F81FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F8200: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 827F8204: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 827F8208: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F820C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8210: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F8214: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F8218: 419A00D0  beq cr6, 0x827f82e8
	if ctx.cr[6].eq {
	pc = 0x827F82E8; continue 'dispatch;
	}
	// 827F821C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F8220: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F8224: 3B8A6790  addi r28, r10, 0x6790
	ctx.r[28].s64 = ctx.r[10].s64 + 26512;
	// 827F8228: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F822C: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F8230: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F8234: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F8238: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 827F823C: 4BFFFF35  bl 0x827f8170
	ctx.lr = 0x827F8240;
	sub_827F8170(ctx, base);
	// 827F8240: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F8244: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827F8248: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F824C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8250: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827F8254: 4BACC20D  bl 0x822c4460
	ctx.lr = 0x827F8258;
	sub_822C4460(ctx, base);
	// 827F8258: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827F825C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F8260: 419A0008  beq cr6, 0x827f8268
	if ctx.cr[6].eq {
	pc = 0x827F8268; continue 'dispatch;
	}
	// 827F8264: 4BAC862D  bl 0x822c0890
	ctx.lr = 0x827F8268;
	sub_822C0890(ctx, base);
	// 827F8268: 83A1005C  lwz r29, 0x5c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F826C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F8270: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F8274: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 827F8278: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 827F827C: 419A0024  beq cr6, 0x827f82a0
	if ctx.cr[6].eq {
	pc = 0x827F82A0; continue 'dispatch;
	}
	// 827F8280: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 827F8284: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F8288: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F828C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F8290: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F8294: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F8298: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F829C: 4082FFE8  bne 0x827f8284
	if !ctx.cr[0].eq {
	pc = 0x827F8284; continue 'dispatch;
	}
	// 827F82A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F82A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F82A8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 827F82AC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F82B0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827F82B4: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 827F82B8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827F82BC: 48660785  bl 0x82e58a40
	ctx.lr = 0x827F82C0;
	sub_82E58A40(ctx, base);
	// 827F82C0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F82C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F82C8: 419A0008  beq cr6, 0x827f82d0
	if ctx.cr[6].eq {
	pc = 0x827F82D0; continue 'dispatch;
	}
	// 827F82CC: 4BAC85C5  bl 0x822c0890
	ctx.lr = 0x827F82D0;
	sub_822C0890(ctx, base);
	// 827F82D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F82D4: 4BCF0DBD  bl 0x824e9090
	ctx.lr = 0x827F82D8;
	sub_824E9090(ctx, base);
	// 827F82D8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F82DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F82E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F82E4: 409AFF48  bne cr6, 0x827f822c
	if !ctx.cr[6].eq {
	pc = 0x827F822C; continue 'dispatch;
	}
	// 827F82E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F82EC: 4BD3558D  bl 0x8252d878
	ctx.lr = 0x827F82F0;
	sub_8252D878(ctx, base);
	// 827F82F0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F82F4: 419A000C  beq cr6, 0x827f8300
	if ctx.cr[6].eq {
	pc = 0x827F8300; continue 'dispatch;
	}
	// 827F82F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F82FC: 4BAC8595  bl 0x822c0890
	ctx.lr = 0x827F8300;
	sub_822C0890(ctx, base);
	// 827F8300: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F8304: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827F8308: 489AFEB0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8310 size=248
    let mut pc: u32 = 0x827F8310;
    'dispatch: loop {
        match pc {
            0x827F8310 => {
    //   block [0x827F8310..0x827F8408)
	// 827F8310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8314: 489AFE59  bl 0x831a816c
	ctx.lr = 0x827F8318;
	sub_831A8130(ctx, base);
	// 827F8318: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F831C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8324: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F8328: 485FB6E1  bl 0x82df3a08
	ctx.lr = 0x827F832C;
	sub_82DF3A08(ctx, base);
	// 827F832C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F8330: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F8334: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F8338: 4BC99FD1  bl 0x82492308
	ctx.lr = 0x827F833C;
	sub_82492308(ctx, base);
	// 827F833C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8340: 485FB0E9  bl 0x82df3428
	ctx.lr = 0x827F8344;
	sub_82DF3428(ctx, base);
	// 827F8344: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F8348: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F834C: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F8350: 419A00AC  beq cr6, 0x827f83fc
	if ctx.cr[6].eq {
	pc = 0x827F83FC; continue 'dispatch;
	}
	// 827F8354: 83C50010  lwz r30, 0x10(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F8358: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F835C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8360: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 827F8364: 4BCF54CD  bl 0x824ed830
	ctx.lr = 0x827F8368;
	sub_824ED830(ctx, base);
	// 827F8368: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F836C: 419A0090  beq cr6, 0x827f83fc
	if ctx.cr[6].eq {
	pc = 0x827F83FC; continue 'dispatch;
	}
	// 827F8370: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827F8374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8378: 4BFFFDF9  bl 0x827f8170
	ctx.lr = 0x827F837C;
	sub_827F8170(ctx, base);
	// 827F837C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F8380: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 827F8384: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F8388: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F838C: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 827F8390: 419A0024  beq cr6, 0x827f83b4
	if ctx.cr[6].eq {
	pc = 0x827F83B4; continue 'dispatch;
	}
	// 827F8394: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 827F8398: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F839C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F83A0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F83A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F83A8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F83AC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F83B0: 4082FFE8  bne 0x827f8398
	if !ctx.cr[0].eq {
	pc = 0x827F8398; continue 'dispatch;
	}
	// 827F83B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F83B8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F83BC: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827F83C0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 827F83C4: 38896790  addi r4, r9, 0x6790
	ctx.r[4].s64 = ctx.r[9].s64 + 26512;
	// 827F83C8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827F83CC: 38A00057  li r5, 0x57
	ctx.r[5].s64 = 87;
	// 827F83D0: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F83D4: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827F83D8: 48660669  bl 0x82e58a40
	ctx.lr = 0x827F83DC;
	sub_82E58A40(ctx, base);
	// 827F83DC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F83E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F83E4: 419A0008  beq cr6, 0x827f83ec
	if ctx.cr[6].eq {
	pc = 0x827F83EC; continue 'dispatch;
	}
	// 827F83E8: 4BAC84A9  bl 0x822c0890
	ctx.lr = 0x827F83EC;
	sub_822C0890(ctx, base);
	// 827F83EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F83F0: 419A000C  beq cr6, 0x827f83fc
	if ctx.cr[6].eq {
	pc = 0x827F83FC; continue 'dispatch;
	}
	// 827F83F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F83F8: 4BAC8499  bl 0x822c0890
	ctx.lr = 0x827F83FC;
	sub_822C0890(ctx, base);
	// 827F83FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F8400: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827F8404: 489AFDB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8408 size=284
    let mut pc: u32 = 0x827F8408;
    'dispatch: loop {
        match pc {
            0x827F8408 => {
    //   block [0x827F8408..0x827F8524)
	// 827F8408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F840C: 489AFD5D  bl 0x831a8168
	ctx.lr = 0x827F8410;
	sub_831A8130(ctx, base);
	// 827F8410: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8418: 38650028  addi r3, r5, 0x28
	ctx.r[3].s64 = ctx.r[5].s64 + 40;
	// 827F841C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F8420: 48810B99  bl 0x83008fb8
	ctx.lr = 0x827F8424;
	sub_83008FB8(ctx, base);
	// 827F8424: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F8428: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F842C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827F8430: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F8434: 4BFFFCC5  bl 0x827f80f8
	ctx.lr = 0x827F8438;
	sub_827F80F8(ctx, base);
	// 827F8438: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F843C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 827F8440: 8381005C  lwz r28, 0x5c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F8444: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827F8448: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 827F844C: 419A0024  beq cr6, 0x827f8470
	if ctx.cr[6].eq {
	pc = 0x827F8470; continue 'dispatch;
	}
	// 827F8450: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 827F8454: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F8458: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F845C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F8460: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F8464: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F8468: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F846C: 4082FFE8  bne 0x827f8454
	if !ctx.cr[0].eq {
	pc = 0x827F8454; continue 'dispatch;
	}
	// 827F8470: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F8474: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F8478: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827F847C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 827F8480: 38896790  addi r4, r9, 0x6790
	ctx.r[4].s64 = ctx.r[9].s64 + 26512;
	// 827F8484: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827F8488: 38A0003A  li r5, 0x3a
	ctx.r[5].s64 = 58;
	// 827F848C: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F8490: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827F8494: 486605AD  bl 0x82e58a40
	ctx.lr = 0x827F8498;
	sub_82E58A40(ctx, base);
	// 827F8498: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F849C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F84A0: 419A0008  beq cr6, 0x827f84a8
	if ctx.cr[6].eq {
	pc = 0x827F84A8; continue 'dispatch;
	}
	// 827F84A4: 4BAC83ED  bl 0x822c0890
	ctx.lr = 0x827F84A8;
	sub_822C0890(ctx, base);
	// 827F84A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F84AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F84B0: 485FB559  bl 0x82df3a08
	ctx.lr = 0x827F84B4;
	sub_82DF3A08(ctx, base);
	// 827F84B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F84B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F84BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F84C0: 4BC99E49  bl 0x82492308
	ctx.lr = 0x827F84C4;
	sub_82492308(ctx, base);
	// 827F84C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F84C8: 485FAF61  bl 0x82df3428
	ctx.lr = 0x827F84CC;
	sub_82DF3428(ctx, base);
	// 827F84CC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F84D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F84D4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F84D8: 419A000C  beq cr6, 0x827f84e4
	if ctx.cr[6].eq {
	pc = 0x827F84E4; continue 'dispatch;
	}
	// 827F84DC: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 827F84E0: 4800002C  b 0x827f850c
	pc = 0x827F850C; continue 'dispatch;
	// 827F84E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F84E8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F84EC: 485FB51D  bl 0x82df3a08
	ctx.lr = 0x827F84F0;
	sub_82DF3A08(ctx, base);
	// 827F84F0: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 827F84F4: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 827F84F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F84FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F8500: 48386571  bl 0x82b7ea70
	ctx.lr = 0x827F8504;
	sub_82B7EA70(ctx, base);
	// 827F8504: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F8508: 485FAF21  bl 0x82df3428
	ctx.lr = 0x827F850C;
	sub_82DF3428(ctx, base);
	// 827F850C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827F8510: 419A000C  beq cr6, 0x827f851c
	if ctx.cr[6].eq {
	pc = 0x827F851C; continue 'dispatch;
	}
	// 827F8514: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F8518: 4BAC8379  bl 0x822c0890
	ctx.lr = 0x827F851C;
	sub_822C0890(ctx, base);
	// 827F851C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F8520: 489AFC98  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F8528 size=4
    let mut pc: u32 = 0x827F8528;
    'dispatch: loop {
        match pc {
            0x827F8528 => {
    //   block [0x827F8528..0x827F852C)
	// 827F8528: 4BFB4C68  b 0x827ad190
	sub_827AD190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8530 size=64
    let mut pc: u32 = 0x827F8530;
    'dispatch: loop {
        match pc {
            0x827F8530 => {
    //   block [0x827F8530..0x827F8570)
	// 827F8530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F853C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F8540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8548: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F854C: 4BB10EDD  bl 0x82309428
	ctx.lr = 0x827F8550;
	sub_82309428(ctx, base);
	// 827F8550: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 827F8554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8558: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F855C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8564: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F8568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F856C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8570 size=60
    let mut pc: u32 = 0x827F8570;
    'dispatch: loop {
        match pc {
            0x827F8570 => {
    //   block [0x827F8570..0x827F85AC)
	// 827F8570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F857C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8584: 4BFFCEED  bl 0x827f5470
	ctx.lr = 0x827F8588;
	sub_827F5470(ctx, base);
	// 827F8588: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F858C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8590: 396B67D4  addi r11, r11, 0x67d4
	ctx.r[11].s64 = ctx.r[11].s64 + 26580;
	// 827F8594: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F8598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F859C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F85A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F85A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F85A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F85B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F85B0 size=16
    let mut pc: u32 = 0x827F85B0;
    'dispatch: loop {
        match pc {
            0x827F85B0 => {
    //   block [0x827F85B0..0x827F85C0)
	// 827F85B0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F85B4: 396B67D4  addi r11, r11, 0x67d4
	ctx.r[11].s64 = ctx.r[11].s64 + 26580;
	// 827F85B8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F85BC: 4BFFCDA4  b 0x827f5360
	sub_827F5360(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F85C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F85C0 size=88
    let mut pc: u32 = 0x827F85C0;
    'dispatch: loop {
        match pc {
            0x827F85C0 => {
    //   block [0x827F85C0..0x827F8618)
	// 827F85C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F85C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F85C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F85CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F85D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F85D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F85D8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F85DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F85E0: 396B67D4  addi r11, r11, 0x67d4
	ctx.r[11].s64 = ctx.r[11].s64 + 26580;
	// 827F85E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F85E8: 4BFFCD79  bl 0x827f5360
	ctx.lr = 0x827F85EC;
	sub_827F5360(ctx, base);
	// 827F85EC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F85F0: 4182000C  beq 0x827f85fc
	if ctx.cr[0].eq {
	pc = 0x827F85FC; continue 'dispatch;
	}
	// 827F85F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F85F8: 485F9DE1  bl 0x82df23d8
	ctx.lr = 0x827F85FC;
	sub_82DF23D8(ctx, base);
	// 827F85FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F8604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F860C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F8610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8618 size=148
    let mut pc: u32 = 0x827F8618;
    'dispatch: loop {
        match pc {
            0x827F8618 => {
    //   block [0x827F8618..0x827F86AC)
	// 827F8618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F861C: 489AFB51  bl 0x831a816c
	ctx.lr = 0x827F8620;
	sub_831A8130(ctx, base);
	// 827F8620: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827F8624: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8628: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F862C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F8630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8634: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F8638: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F863C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8640: 4BFF5B81  bl 0x827ee1c0
	ctx.lr = 0x827F8644;
	sub_827EE1C0(ctx, base);
	// 827F8644: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F8648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F864C: 388B6910  addi r4, r11, 0x6910
	ctx.r[4].s64 = ctx.r[11].s64 + 26896;
	// 827F8650: 4BC2DAE1  bl 0x82426130
	ctx.lr = 0x827F8654;
	sub_82426130(ctx, base);
	// 827F8654: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F8658: 4182001C  beq 0x827f8674
	if ctx.cr[0].eq {
	pc = 0x827F8674; continue 'dispatch;
	}
	// 827F865C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827F8660: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F8664: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F8668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F866C: 4BFFC185  bl 0x827f47f0
	ctx.lr = 0x827F8670;
	sub_827F47F0(ctx, base);
	// 827F8670: 4800002C  b 0x827f869c
	pc = 0x827F869C; continue 'dispatch;
	// 827F8674: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8678: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827F867C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F8680: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F8684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F8688: 4E800421  bctrl
	ctx.lr = 0x827F868C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F868C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F8690: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827F8694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8698: 486836B1  bl 0x82e7bd48
	ctx.lr = 0x827F869C;
	sub_82E7BD48(ctx, base);
	// 827F869C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F86A0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827F86A4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827F86A8: 489AFB14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F86B0 size=8
    let mut pc: u32 = 0x827F86B0;
    'dispatch: loop {
        match pc {
            0x827F86B0 => {
    //   block [0x827F86B0..0x827F86B8)
	// 827F86B0: D0230090  stfs f1, 0x90(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F86B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F86B8 size=24
    let mut pc: u32 = 0x827F86B8;
    'dispatch: loop {
        match pc {
            0x827F86B8 => {
    //   block [0x827F86B8..0x827F86D0)
	// 827F86B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F86BC: C1A30090  lfs f13, 0x90(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F86C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F86C4: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F86C8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827F86CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F86D0 size=8
    let mut pc: u32 = 0x827F86D0;
    'dispatch: loop {
        match pc {
            0x827F86D0 => {
    //   block [0x827F86D0..0x827F86D8)
	// 827F86D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F86D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F86D8 size=16
    let mut pc: u32 = 0x827F86D8;
    'dispatch: loop {
        match pc {
            0x827F86D8 => {
    //   block [0x827F86D8..0x827F86E8)
	// 827F86D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F86DC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F86E0: D0030090  stfs f0, 0x90(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F86E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F86E8 size=8
    let mut pc: u32 = 0x827F86E8;
    'dispatch: loop {
        match pc {
            0x827F86E8 => {
    //   block [0x827F86E8..0x827F86F0)
	// 827F86E8: C0230094  lfs f1, 0x94(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F86EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F86F0 size=8
    let mut pc: u32 = 0x827F86F0;
    'dispatch: loop {
        match pc {
            0x827F86F0 => {
    //   block [0x827F86F0..0x827F86F8)
	// 827F86F0: D0230094  stfs f1, 0x94(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F86F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F86F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F86F8 size=8
    let mut pc: u32 = 0x827F86F8;
    'dispatch: loop {
        match pc {
            0x827F86F8 => {
    //   block [0x827F86F8..0x827F8700)
	// 827F86F8: 88630110  lbz r3, 0x110(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) } as u64;
	// 827F86FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8700 size=180
    let mut pc: u32 = 0x827F8700;
    'dispatch: loop {
        match pc {
            0x827F8700 => {
    //   block [0x827F8700..0x827F87B4)
	// 827F8700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F870C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8714: 4BFFFE5D  bl 0x827f8570
	ctx.lr = 0x827F8718;
	sub_827F8570(ctx, base);
	// 827F8718: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F871C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F8720: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 827F8724: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827F8728: 39296910  addi r9, r9, 0x6910
	ctx.r[9].s64 = ctx.r[9].s64 + 26896;
	// 827F872C: 38EA684C  addi r7, r10, 0x684c
	ctx.r[7].s64 = ctx.r[10].s64 + 26700;
	// 827F8730: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F8738: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F873C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 827F8740: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F8744: 995F00A0  stb r10, 0xa0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u8 ) };
	// 827F8748: C1A808A8  lfs f13, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F874C: D01F00B0  stfs f0, 0xb0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 827F8750: 397F00A0  addi r11, r31, 0xa0
	ctx.r[11].s64 = ctx.r[31].s64 + 160;
	// 827F8754: D1BF00B4  stfs f13, 0xb4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 827F8758: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 827F875C: D01F00B8  stfs f0, 0xb8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 827F8760: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 827F8764: D01F00BC  stfs f0, 0xbc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 827F8768: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827F876C: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F8770: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F87B8 size=88
    let mut pc: u32 = 0x827F87B8;
    'dispatch: loop {
        match pc {
            0x827F87B8 => {
    //   block [0x827F87B8..0x827F8810)
	// 827F87B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F87BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F87C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F87C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F87C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F87CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F87D0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F87D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F87D8: 396B684C  addi r11, r11, 0x684c
	ctx.r[11].s64 = ctx.r[11].s64 + 26700;
	// 827F87DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F87E0: 4BFFFDD1  bl 0x827f85b0
	ctx.lr = 0x827F87E4;
	sub_827F85B0(ctx, base);
	// 827F87E4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F87E8: 4182000C  beq 0x827f87f4
	if ctx.cr[0].eq {
	pc = 0x827F87F4; continue 'dispatch;
	}
	// 827F87EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F87F0: 485F9BE9  bl 0x82df23d8
	ctx.lr = 0x827F87F4;
	sub_82DF23D8(ctx, base);
	// 827F87F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F87F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F87FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8804: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F8808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F880C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8810 size=76
    let mut pc: u32 = 0x827F8810;
    'dispatch: loop {
        match pc {
            0x827F8810 => {
    //   block [0x827F8810..0x827F885C)
	// 827F8810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F881C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8820: C0440008  lfs f2, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 827F8824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8828: C0240000  lfs f1, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F882C: 489B278D  bl 0x831aafb8
	ctx.lr = 0x827F8830;
	sub_831AAFB8(ctx, base);
	// 827F8830: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8834: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827F8838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F883C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F8840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F8844: 4E800421  bctrl
	ctx.lr = 0x827F8848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F8848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F884C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8860 size=92
    let mut pc: u32 = 0x827F8860;
    'dispatch: loop {
        match pc {
            0x827F8860 => {
    //   block [0x827F8860..0x827F88BC)
	// 827F8860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F886C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8874: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F8878: 48683621  bl 0x82e7be98
	ctx.lr = 0x827F887C;
	sub_82E7BE98(ctx, base);
	// 827F887C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F8880: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F8884: 38ABCA90  addi r5, r11, -0x3570
	ctx.r[5].s64 = ctx.r[11].s64 + -13680;
	// 827F8888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F888C: 4868343D  bl 0x82e7bcc8
	ctx.lr = 0x827F8890;
	sub_82E7BCC8(ctx, base);
	// 827F8890: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8894: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F8898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F889C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F88A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F88A4: 4E800421  bctrl
	ctx.lr = 0x827F88A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F88A8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F88AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F88B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F88B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F88B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F88C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F88C0 size=380
    let mut pc: u32 = 0x827F88C0;
    'dispatch: loop {
        match pc {
            0x827F88C0 => {
    //   block [0x827F88C0..0x827F8A3C)
	// 827F88C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F88C4: 489AF8A1  bl 0x831a8164
	ctx.lr = 0x827F88C8;
	sub_831A8130(ctx, base);
	// 827F88C8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 827F88CC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F88D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F88D4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F88D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F88DC: D3E100DC  stfs f31, 0xdc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 827F88E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F88E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F88E8: 4BFF6E91  bl 0x827ef778
	ctx.lr = 0x827F88EC;
	sub_827EF778(ctx, base);
	// 827F88EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F88F0: 41820140  beq 0x827f8a30
	if ctx.cr[0].eq {
	pc = 0x827F8A30; continue 'dispatch;
	}
	// 827F88F4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F88F8: 8B7D00A0  lbz r27, 0xa0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(160 as u32) ) } as u64;
	// 827F88FC: 3BFD00A0  addi r31, r29, 0xa0
	ctx.r[31].s64 = ctx.r[29].s64 + 160;
	// 827F8900: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F8904: 394BCA80  addi r10, r11, -0x3580
	ctx.r[10].s64 = ctx.r[11].s64 + -13696;
	// 827F8908: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F890C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 827F8910: C00BCA80  lfs f0, -0x3580(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13696 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8914: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F8918: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F891C: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827F8920: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8924: FD800050  fneg f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8928: C1AA0008  lfs f13, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F892C: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8930: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8934: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8938: D1810064  stfs f12, 0x64(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827F893C: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827F8940: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827F8944: 482ED2F5  bl 0x82ae5c38
	ctx.lr = 0x827F8948;
	sub_82AE5C38(ctx, base);
	// 827F8948: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F894C: 4BFF70B5  bl 0x827efa00
	ctx.lr = 0x827F8950;
	sub_827EFA00(ctx, base);
	// 827F8950: 396100DC  addi r11, r1, 0xdc
	ctx.r[11].s64 = ctx.r[1].s64 + 220;
	// 827F8954: 895D00A0  lbz r10, 0xa0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(160 as u32) ) } as u64;
	// 827F8958: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F895C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F8960: 13C05C07  vcmpneb. (lvlx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F8964: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8A40 size=440
    let mut pc: u32 = 0x827F8A40;
    'dispatch: loop {
        match pc {
            0x827F8A40 => {
    //   block [0x827F8A40..0x827F8BF8)
	// 827F8A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8A44: 489AF725  bl 0x831a8168
	ctx.lr = 0x827F8A48;
	sub_831A8130(ctx, base);
	// 827F8A48: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827F8A4C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8A50: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F8A54: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F8A58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F8A5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F8A60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F8A64: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F8A68: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827F8A6C: 4868342D  bl 0x82e7be98
	ctx.lr = 0x827F8A70;
	sub_82E7BE98(ctx, base);
	// 827F8A70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F8A74: C11F0090  lfs f8, 0x90(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 827F8A78: C0DF0094  lfs f6, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 827F8A7C: FD604090  fmr f11, f8
	ctx.f[11].f64 = ctx.f[8].f64;
	// 827F8A80: C0BC000C  lfs f5, 0xc(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 827F8A84: FDA03090  fmr f13, f6
	ctx.f[13].f64 = ctx.f[6].f64;
	// 827F8A88: C15C0010  lfs f10, 0x10(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 827F8A8C: C12B08A4  lfs f9, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 827F8A90: C01C0014  lfs f0, 0x14(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8A94: FF084800  fcmpu cr6, f8, f9
	ctx.cr[6].compare_f64(ctx.f[8].f64, ctx.f[9].f64);
	// 827F8A98: 40980014  bge cr6, 0x827f8aac
	if !ctx.cr[6].lt {
	pc = 0x827F8AAC; continue 'dispatch;
	}
	// 827F8A9C: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8AA0: FD605850  fneg f11, f11
	ctx.f[11].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8AA4: FD405050  fneg f10, f10
	ctx.f[10].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8AA8: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8AAC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F8AB0: C18B630C  lfs f12, 0x630c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F8AB4: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 827F8AB8: 40990028  ble cr6, 0x827f8ae0
	if !ctx.cr[6].gt {
	pc = 0x827F8AE0; continue 'dispatch;
	}
	// 827F8ABC: ED8A07F2  fmuls f12, f10, f31
	ctx.f[12].f64 = (((ctx.f[10].f64 * ctx.f[31].f64) as f32) as f64);
	// 827F8AC0: FCE06A10  fabs f7, f13
	ctx.f[7].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 827F8AC4: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 827F8AC8: FF076000  fcmpu cr6, f7, f12
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[12].f64);
	// 827F8ACC: 41990014  bgt cr6, 0x827f8ae0
	if ctx.cr[6].gt {
	pc = 0x827F8AE0; continue 'dispatch;
	}
	// 827F8AD0: D13F0090  stfs f9, 0x90(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F8AD4: FFE04090  fmr f31, f8
	ctx.f[31].f64 = ctx.f[8].f64;
	// 827F8AD8: D13F0094  stfs f9, 0x94(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F8ADC: 48000054  b 0x827f8b30
	pc = 0x827F8B30; continue 'dispatch;
	// 827F8AE0: ED860024  fdivs f12, f6, f0
	ctx.f[12].f64 = ((ctx.f[6].f64 / ctx.f[0].f64) as f32) as f64;
	// 827F8AE4: FF0C4800  fcmpu cr6, f12, f9
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[9].f64);
	// 827F8AE8: 40990034  ble cr6, 0x827f8b1c
	if !ctx.cr[6].gt {
	pc = 0x827F8B1C; continue 'dispatch;
	}
	// 827F8AEC: EC8C0032  fmuls f4, f12, f0
	ctx.f[4].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 827F8AF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F8AF4: C0EB9450  lfs f7, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 827F8AF8: ECE469FA  fmadds f7, f4, f7, f13
	ctx.f[7].f64 = (((ctx.f[4].f64 * ctx.f[7].f64 + ctx.f[13].f64) as f32) as f64);
	// 827F8AFC: ED870332  fmuls f12, f7, f12
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[12].f64) as f32) as f64);
	// 827F8B00: FF0B6000  fcmpu cr6, f11, f12
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[12].f64);
	// 827F8B04: 4098000C  bge cr6, 0x827f8b10
	if !ctx.cr[6].lt {
	pc = 0x827F8B10; continue 'dispatch;
	}
	// 827F8B08: FD400050  fneg f10, f0
	ctx.f[10].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F8B0C: 48000010  b 0x827f8b1c
	pc = 0x827F8B1C; continue 'dispatch;
	// 827F8B10: FF056800  fcmpu cr6, f5, f13
	ctx.cr[6].compare_f64(ctx.f[5].f64, ctx.f[13].f64);
	// 827F8B14: 40980008  bge cr6, 0x827f8b1c
	if !ctx.cr[6].lt {
	pc = 0x827F8B1C; continue 'dispatch;
	}
	// 827F8B18: FD404890  fmr f10, f9
	ctx.f[10].f64 = ctx.f[9].f64;
	// 827F8B1C: EC0A37FA  fmadds f0, f10, f31, f6
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[31].f64 + ctx.f[6].f64) as f32) as f64);
	// 827F8B20: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F8B24: EFE007F2  fmuls f31, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 827F8B28: EC08F828  fsubs f0, f8, f31
	ctx.f[0].f64 = (((ctx.f[8].f64 - ctx.f[31].f64) as f32) as f64);
	// 827F8B2C: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F8B30: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F8B34: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827F8B38: 38ABCA80  addi r5, r11, -0x3580
	ctx.r[5].s64 = ctx.r[11].s64 + -13696;
	// 827F8B3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8B40: 48683189  bl 0x82e7bcc8
	ctx.lr = 0x827F8B44;
	sub_82E7BCC8(ctx, base);
	// 827F8B44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F8B48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F8B4C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F8B50: 486840F1  bl 0x82e7cc40
	ctx.lr = 0x827F8B54;
	sub_82E7CC40(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8BF8 size=300
    let mut pc: u32 = 0x827F8BF8;
    'dispatch: loop {
        match pc {
            0x827F8BF8 => {
    //   block [0x827F8BF8..0x827F8D24)
	// 827F8BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8BFC: 489AF571  bl 0x831a816c
	ctx.lr = 0x827F8C00;
	sub_831A8130(ctx, base);
	// 827F8C00: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827F8C04: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8C08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F8C0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F8C10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F8C14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8C18: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 827F8C1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F8C20: 4E800421  bctrl
	ctx.lr = 0x827F8C24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F8C24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F8C28: 418200A0  beq 0x827f8cc8
	if ctx.cr[0].eq {
	pc = 0x827F8CC8; continue 'dispatch;
	}
	// 827F8C2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F8C30: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F8C34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8C38: 4BFF4C79  bl 0x827ed8b0
	ctx.lr = 0x827F8C3C;
	sub_827ED8B0(ctx, base);
	// 827F8C3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F8C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8C44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F8C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F8C4C: 4E800421  bctrl
	ctx.lr = 0x827F8C50;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8D28 size=116
    let mut pc: u32 = 0x827F8D28;
    'dispatch: loop {
        match pc {
            0x827F8D28 => {
    //   block [0x827F8D28..0x827F8D9C)
	// 827F8D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F8D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8D38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8D3C: 4BFFF835  bl 0x827f8570
	ctx.lr = 0x827F8D40;
	sub_827F8570(ctx, base);
	// 827F8D40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F8D44: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F8D48: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 827F8D4C: 394A68BC  addi r10, r10, 0x68bc
	ctx.r[10].s64 = ctx.r[10].s64 + 26812;
	// 827F8D50: 3909BA80  addi r8, r9, -0x4580
	ctx.r[8].s64 = ctx.r[9].s64 + -17792;
	// 827F8D54: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8D58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F8D5C: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F8D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8D64: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F8D68: C009BA80  lfs f0, -0x4580(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8D6C: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827F8D70: C0080004  lfs f0, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8D74: D01F00A4  stfs f0, 0xa4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827F8D78: C0080008  lfs f0, 8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8D7C: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827F8D80: C008000C  lfs f0, 0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8D84: D01F00AC  stfs f0, 0xac(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 827F8D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F8D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F8DA0 size=88
    let mut pc: u32 = 0x827F8DA0;
    'dispatch: loop {
        match pc {
            0x827F8DA0 => {
    //   block [0x827F8DA0..0x827F8DF8)
	// 827F8DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8DA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F8DAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F8DB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8DB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8DB8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F8DBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F8DC0: 396B68BC  addi r11, r11, 0x68bc
	ctx.r[11].s64 = ctx.r[11].s64 + 26812;
	// 827F8DC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F8DC8: 4BFFF7E9  bl 0x827f85b0
	ctx.lr = 0x827F8DCC;
	sub_827F85B0(ctx, base);
	// 827F8DCC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F8DD0: 4182000C  beq 0x827f8ddc
	if ctx.cr[0].eq {
	pc = 0x827F8DDC; continue 'dispatch;
	}
	// 827F8DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8DD8: 485F9601  bl 0x82df23d8
	ctx.lr = 0x827F8DDC;
	sub_82DF23D8(ctx, base);
	// 827F8DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F8DE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F8DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8DEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F8DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8DF8 size=88
    let mut pc: u32 = 0x827F8DF8;
    'dispatch: loop {
        match pc {
            0x827F8DF8 => {
    //   block [0x827F8DF8..0x827F8E50)
	// 827F8DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8E00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F8E04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8E08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F8E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8E14: 388BCA90  addi r4, r11, -0x3570
	ctx.r[4].s64 = ctx.r[11].s64 + -13680;
	// 827F8E18: 48683E29  bl 0x82e7cc40
	ctx.lr = 0x827F8E1C;
	sub_82E7CC40(ctx, base);
	// 827F8E1C: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E20: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827F8E24: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E28: D01F00A4  stfs f0, 0xa4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827F8E2C: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E30: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827F8E34: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E38: D01F00AC  stfs f0, 0xac(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 827F8E3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F8E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8E50 size=88
    let mut pc: u32 = 0x827F8E50;
    'dispatch: loop {
        match pc {
            0x827F8E50 => {
    //   block [0x827F8E50..0x827F8EA8)
	// 827F8E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F8E5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8E60: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F8E64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8E68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F8E6C: 388BCA80  addi r4, r11, -0x3580
	ctx.r[4].s64 = ctx.r[11].s64 + -13696;
	// 827F8E70: 48683DD1  bl 0x82e7cc40
	ctx.lr = 0x827F8E74;
	sub_82E7CC40(ctx, base);
	// 827F8E74: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E78: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827F8E7C: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E80: D01F00A4  stfs f0, 0xa4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827F8E84: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E88: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827F8E8C: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8E90: D01F00AC  stfs f0, 0xac(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 827F8E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F8E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8EA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F8EA8 size=36
    let mut pc: u32 = 0x827F8EA8;
    'dispatch: loop {
        match pc {
            0x827F8EA8 => {
    //   block [0x827F8EA8..0x827F8ECC)
	// 827F8EA8: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8EAC: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827F8EB0: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8EB4: D00300A4  stfs f0, 0xa4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827F8EB8: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8EBC: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827F8EC0: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8EC4: D00300AC  stfs f0, 0xac(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 827F8EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F8ED0 size=160
    let mut pc: u32 = 0x827F8ED0;
    'dispatch: loop {
        match pc {
            0x827F8ED0 => {
    //   block [0x827F8ED0..0x827F8F70)
	// 827F8ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F8ED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F8EDC: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 827F8EE0: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827F8EE4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F8EE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F8EEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F8EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F8EF4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F8EF8: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 827F8EFC: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F8F00: C3CA08A8  lfs f30, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827F8F04: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827F8F08: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827F8F0C: D3C10058  stfs f30, 0x58(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827F8F10: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827F8F14: 48682F85  bl 0x82e7be98
	ctx.lr = 0x827F8F18;
	sub_82E7BE98(ctx, base);
	// 827F8F18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F8F1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F8F20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F8F24: 48682DA5  bl 0x82e7bcc8
	ctx.lr = 0x827F8F28;
	sub_82E7BCC8(ctx, base);
	// 827F8F28: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 827F8F2C: C1A10068  lfs f13, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F8F30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F8F34: C00BDFB0  lfs f0, -0x2050(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8F38: EC1E0028  fsubs f0, f30, f0
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 827F8F3C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827F8F40: 41980014  blt cr6, 0x827f8f54
	if ctx.cr[6].lt {
	pc = 0x827F8F54; continue 'dispatch;
	}
	// 827F8F44: C01F0094  lfs f0, 0x94(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8F48: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 827F8F4C: 409A0008  bne cr6, 0x827f8f54
	if !ctx.cr[6].eq {
	pc = 0x827F8F54; continue 'dispatch;
	}
	// 827F8F50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F8F54: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827F8F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F8F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F8F60: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827F8F64: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F8F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F8F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F8F70 size=56
    let mut pc: u32 = 0x827F8F70;
    'dispatch: loop {
        match pc {
            0x827F8F70 => {
    //   block [0x827F8F70..0x827F8FA8)
	// 827F8F70: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F8F74: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 827F8F78: 392ABA80  addi r9, r10, -0x4580
	ctx.r[9].s64 = ctx.r[10].s64 + -17792;
	// 827F8F7C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8F80: D0030094  stfs f0, 0x94(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F8F84: C00ABA80  lfs f0, -0x4580(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8F88: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827F8F8C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8F90: D00300A4  stfs f0, 0xa4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827F8F94: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8F98: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827F8F9C: C009000C  lfs f0, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F8FA0: D00300AC  stfs f0, 0xac(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 827F8FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F8FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F8FA8 size=656
    let mut pc: u32 = 0x827F8FA8;
    'dispatch: loop {
        match pc {
            0x827F8FA8 => {
    //   block [0x827F8FA8..0x827F9238)
	// 827F8FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F8FAC: 489AF1B9  bl 0x831a8164
	ctx.lr = 0x827F8FB0;
	sub_831A8130(ctx, base);
	// 827F8FB0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 827F8FB4: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9238 size=264
    let mut pc: u32 = 0x827F9238;
    'dispatch: loop {
        match pc {
            0x827F9238 => {
    //   block [0x827F9238..0x827F9340)
	// 827F9238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F923C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F9244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9248: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827F924C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9250: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F9254: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F9258: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F925C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9260: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 827F9264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F9268: 4E800421  bctrl
	ctx.lr = 0x827F926C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F926C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9270: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9274: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F9278: 418200A4  beq 0x827f931c
	if ctx.cr[0].eq {
	pc = 0x827F931C; continue 'dispatch;
	}
	// 827F927C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F9280: 4BFF4631  bl 0x827ed8b0
	ctx.lr = 0x827F9284;
	sub_827ED8B0(ctx, base);
	// 827F9284: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F928C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F9290: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F9294: 4E800421  bctrl
	ctx.lr = 0x827F9298;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9340 size=16
    let mut pc: u32 = 0x827F9340;
    'dispatch: loop {
        match pc {
            0x827F9340 => {
    //   block [0x827F9340..0x827F9350)
	// 827F9340: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F9344: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827F9348: 808B66F4  lwz r4, 0x66f4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26356 as u32) ) } as u64;
	// 827F934C: 4BC96054  b 0x8248f3a0
	sub_8248F3A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9350 size=8
    let mut pc: u32 = 0x827F9350;
    'dispatch: loop {
        match pc {
            0x827F9350 => {
    //   block [0x827F9350..0x827F9358)
	// 827F9350: 38630100  addi r3, r3, 0x100
	ctx.r[3].s64 = ctx.r[3].s64 + 256;
	// 827F9354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9358 size=8
    let mut pc: u32 = 0x827F9358;
    'dispatch: loop {
        match pc {
            0x827F9358 => {
    //   block [0x827F9358..0x827F9360)
	// 827F9358: 386300F0  addi r3, r3, 0xf0
	ctx.r[3].s64 = ctx.r[3].s64 + 240;
	// 827F935C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F9360 size=112
    let mut pc: u32 = 0x827F9360;
    'dispatch: loop {
        match pc {
            0x827F9360 => {
    //   block [0x827F9360..0x827F93D0)
	// 827F9360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F936C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827F9370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F937C: 4BD15FB5  bl 0x8250f330
	ctx.lr = 0x827F9380;
	sub_8250F330(ctx, base);
	// 827F9380: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9384: 4BCF04CD  bl 0x824e9850
	ctx.lr = 0x827F9388;
	sub_824E9850(ctx, base);
	// 827F9388: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F938C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F9390: C00B9450  lfs f0, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F9394: EFE10028  fsubs f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 827F9398: 485F88F9  bl 0x82df1c90
	ctx.lr = 0x827F939C;
	sub_82DF1C90(ctx, base);
	// 827F939C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F93A0: D3FF0000  stfs f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827F93A4: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F93A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F93AC: D3FF0008  stfs f31, 8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F93B0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F93B4: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827F93B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F93BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F93C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F93C4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F93C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F93CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F93D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F93D0 size=12
    let mut pc: u32 = 0x827F93D0;
    'dispatch: loop {
        match pc {
            0x827F93D0 => {
    //   block [0x827F93D0..0x827F93DC)
	// 827F93D0: 80630108  lwz r3, 0x108(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F93D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F93D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F93DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F93DC size=8
    let mut pc: u32 = 0x827F93DC;
    'dispatch: loop {
        match pc {
            0x827F93DC => {
    //   block [0x827F93DC..0x827F93E4)
	// 827F93DC: 4BC95FC4  b 0x8248f3a0
	sub_8248F3A0(ctx, base);
	return;
	// 827F93E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F93E8 size=216
    let mut pc: u32 = 0x827F93E8;
    'dispatch: loop {
        match pc {
            0x827F93E8 => {
    //   block [0x827F93E8..0x827F94C0)
	// 827F93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F93F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F93F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F93F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F93FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9400: 897F011C  lbz r11, 0x11c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 827F9404: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F9408: 41820064  beq 0x827f946c
	if ctx.cr[0].eq {
	pc = 0x827F946C; continue 'dispatch;
	}
	// 827F940C: 807F00F0  lwz r3, 0xf0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 827F9410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9414: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F9418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F941C: 4E800421  bctrl
	ctx.lr = 0x827F9420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F9420: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F9424: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F9428: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F942C: 486839DD  bl 0x82e7ce08
	ctx.lr = 0x827F9430;
	sub_82E7CE08(ctx, base);
	// 827F9430: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9438: 4BD186B9  bl 0x82511af0
	ctx.lr = 0x827F943C;
	sub_82511AF0(ctx, base);
	// 827F943C: C01E003C  lfs f0, 0x3c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F9440: C1BE0038  lfs f13, 0x38(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F9444: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F9448: C19E0034  lfs f12, 0x34(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F944C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9450: C17E0030  lfs f11, 0x30(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827F9454: D1610050  stfs f11, 0x50(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827F9458: D1810054  stfs f12, 0x54(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827F945C: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827F9460: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827F9464: 4BD1861D  bl 0x82511a80
	ctx.lr = 0x827F9468;
	sub_82511A80(ctx, base);
	// 827F9468: 48000040  b 0x827f94a8
	pc = 0x827F94A8; continue 'dispatch;
	// 827F946C: 809F0108  lwz r4, 0x108(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F9470: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F9474: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9478: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F947C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F9480: 4E800421  bctrl
	ctx.lr = 0x827F9484;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F9484: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F948C: 4BD185F5  bl 0x82511a80
	ctx.lr = 0x827F9490;
	sub_82511A80(ctx, base);
	// 827F9490: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F9494: 809F0108  lwz r4, 0x108(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F9498: 4BAF3AD9  bl 0x822ecf70
	ctx.lr = 0x827F949C;
	sub_822ECF70(ctx, base);
	// 827F949C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F94A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F94A4: 4BD1864D  bl 0x82511af0
	ctx.lr = 0x827F94A8;
	sub_82511AF0(ctx, base);
	// 827F94A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827F94AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F94B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F94B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F94B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F94BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F94C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F94C0 size=8
    let mut pc: u32 = 0x827F94C0;
    'dispatch: loop {
        match pc {
            0x827F94C0 => {
    //   block [0x827F94C0..0x827F94C8)
	// 827F94C0: 80630108  lwz r3, 0x108(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F94C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F94C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F94C8 size=12
    let mut pc: u32 = 0x827F94C8;
    'dispatch: loop {
        match pc {
            0x827F94C8 => {
    //   block [0x827F94C8..0x827F94D4)
	// 827F94C8: 80630108  lwz r3, 0x108(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F94CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F94D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F94D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F94D4 size=8
    let mut pc: u32 = 0x827F94D4;
    'dispatch: loop {
        match pc {
            0x827F94D4 => {
    //   block [0x827F94D4..0x827F94DC)
	// 827F94D4: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F94D8: 4BC95EC8  b 0x8248f3a0
	sub_8248F3A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F94DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F94DC size=4
    let mut pc: u32 = 0x827F94DC;
    'dispatch: loop {
        match pc {
            0x827F94DC => {
    //   block [0x827F94DC..0x827F94E0)
	// 827F94DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F94E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F94E0 size=80
    let mut pc: u32 = 0x827F94E0;
    'dispatch: loop {
        match pc {
            0x827F94E0 => {
    //   block [0x827F94E0..0x827F9530)
	// 827F94E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F94E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F94E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F94EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F94F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F94F4: 809F00E8  lwz r4, 0xe8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 827F94F8: 807F00F0  lwz r3, 0xf0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 827F94FC: 48619AD5  bl 0x82e12fd0
	ctx.lr = 0x827F9500;
	sub_82E12FD0(ctx, base);
	// 827F9500: 807F0108  lwz r3, 0x108(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F9504: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9508: 419A000C  beq cr6, 0x827f9514
	if ctx.cr[6].eq {
	pc = 0x827F9514; continue 'dispatch;
	}
	// 827F950C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F9510: 4BAF4529  bl 0x822eda38
	ctx.lr = 0x827F9514;
	sub_822EDA38(ctx, base);
	// 827F9514: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827F9518: 997F011C  stb r11, 0x11c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u8 ) };
	// 827F951C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F9520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F952C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F9530 size=632
    let mut pc: u32 = 0x827F9530;
    'dispatch: loop {
        match pc {
            0x827F9530 => {
    //   block [0x827F9530..0x827F97A8)
	// 827F9530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9534: 489AEC31  bl 0x831a8164
	ctx.lr = 0x827F9538;
	sub_831A8130(ctx, base);
	// 827F9538: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F953C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9540: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 827F9544: 48003A25  bl 0x827fcf68
	ctx.lr = 0x827F9548;
	sub_827FCF68(ctx, base);
	// 827F9548: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F954C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F9550: 388B6928  addi r4, r11, 0x6928
	ctx.r[4].s64 = ctx.r[11].s64 + 26920;
	// 827F9554: 38A00048  li r5, 0x48
	ctx.r[5].s64 = 72;
	// 827F9558: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 827F955C: 485F8E8D  bl 0x82df23e8
	ctx.lr = 0x827F9560;
	sub_82DF23E8(ctx, base);
	// 827F9560: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F9564: 41820018  beq 0x827f957c
	if ctx.cr[0].eq {
	pc = 0x827F957C; continue 'dispatch;
	}
	// 827F9568: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F956C: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 827F9570: 48619B81  bl 0x82e130f0
	ctx.lr = 0x827F9574;
	sub_82E130F0(ctx, base);
	// 827F9574: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9578: 48000008  b 0x827f9580
	pc = 0x827F9580; continue 'dispatch;
	// 827F957C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F9580: 3BDF00F0  addi r30, r31, 0xf0
	ctx.r[30].s64 = ctx.r[31].s64 + 240;
	// 827F9584: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F9588: 4BAE87C9  bl 0x822e1d50
	ctx.lr = 0x827F958C;
	sub_822E1D50(ctx, base);
	// 827F958C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9590: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F9594: 83BF00F0  lwz r29, 0xf0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 827F9598: 4BD18B81  bl 0x82512118
	ctx.lr = 0x827F959C;
	sub_82512118(ctx, base);
	// 827F959C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827F95A0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F95A4: 4861988D  bl 0x82e12e30
	ctx.lr = 0x827F95A8;
	sub_82E12E30(ctx, base);
	// 827F95A8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F95AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F95B0: 419A0008  beq cr6, 0x827f95b8
	if ctx.cr[6].eq {
	pc = 0x827F95B8; continue 'dispatch;
	}
	// 827F95B4: 4BAC72DD  bl 0x822c0890
	ctx.lr = 0x827F95B8;
	sub_822C0890(ctx, base);
	// 827F95B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F95BC: 48619155  bl 0x82e12710
	ctx.lr = 0x827F95C0;
	sub_82E12710(ctx, base);
	// 827F95C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F95C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F95C8: 80BB0000  lwz r5, 0(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F95CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F95D0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F95D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F95D8: 4E800421  bctrl
	ctx.lr = 0x827F95DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F95DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F95E0: 3BBF0100  addi r29, r31, 0x100
	ctx.r[29].s64 = ctx.r[31].s64 + 256;
	// 827F95E4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F95E8: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 827F95EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F95F0: 917F0100  stw r11, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 827F95F4: 4BACAE6D  bl 0x822c4460
	ctx.lr = 0x827F95F8;
	sub_822C4460(ctx, base);
	// 827F95F8: 817F0100  lwz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 827F95FC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F9600: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F9604: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9608: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F960C: 697C0001  xori r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u64 ^ 1;
	// 827F9610: 419A0008  beq cr6, 0x827f9618
	if ctx.cr[6].eq {
	pc = 0x827F9618; continue 'dispatch;
	}
	// 827F9614: 4BAC727D  bl 0x822c0890
	ctx.lr = 0x827F9618;
	sub_822C0890(ctx, base);
	// 827F9618: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F961C: 41820060  beq 0x827f967c
	if ctx.cr[0].eq {
	pc = 0x827F967C; continue 'dispatch;
	}
	// 827F9620: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9624: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9628: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F962C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F9630: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F9634: 419A0024  beq cr6, 0x827f9658
	if ctx.cr[6].eq {
	pc = 0x827F9658; continue 'dispatch;
	}
	// 827F9638: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F963C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F9640: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F9644: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F9648: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F964C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F9650: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F9654: 4082FFE8  bne 0x827f963c
	if !ctx.cr[0].eq {
	pc = 0x827F963C; continue 'dispatch;
	}
	// 827F9658: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F965C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9660: 4861BA51  bl 0x82e150b0
	ctx.lr = 0x827F9664;
	sub_82E150B0(ctx, base);
	// 827F9664: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F9668: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 827F966C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827F9670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9674: 808B7058  lwz r4, 0x7058(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28760 as u32) ) } as u64;
	// 827F9678: 4BD174E1  bl 0x82510b58
	ctx.lr = 0x827F967C;
	sub_82510B58(ctx, base);
	// 827F967C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9680: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9684: 80BB0000  lwz r5, 0(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9688: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F968C: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 827F9690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F9694: 4E800421  bctrl
	ctx.lr = 0x827F9698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F9698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F969C: 3BDF0108  addi r30, r31, 0x108
	ctx.r[30].s64 = ctx.r[31].s64 + 264;
	// 827F96A0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F96A4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 827F96A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F96AC: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 827F96B0: 4BACADB1  bl 0x822c4460
	ctx.lr = 0x827F96B4;
	sub_822C4460(ctx, base);
	// 827F96B4: 817F0108  lwz r11, 0x108(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F96B8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827F96BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F96C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F96C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F96C8: 697C0001  xori r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u64 ^ 1;
	// 827F96CC: 419A0008  beq cr6, 0x827f96d4
	if ctx.cr[6].eq {
	pc = 0x827F96D4; continue 'dispatch;
	}
	// 827F96D0: 4BAC71C1  bl 0x822c0890
	ctx.lr = 0x827F96D4;
	sub_822C0890(ctx, base);
	// 827F96D4: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F96D8: 418200A8  beq 0x827f9780
	if ctx.cr[0].eq {
	pc = 0x827F9780; continue 'dispatch;
	}
	// 827F96DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F96E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F96E4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827F96E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F96EC: 4E800421  bctrl
	ctx.lr = 0x827F96F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F96F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F96F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F96F8: 485FA311  bl 0x82df3a08
	ctx.lr = 0x827F96FC;
	sub_82DF3A08(ctx, base);
	// 827F96FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F9700: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F9704: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9708: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F970C: 4861B925  bl 0x82e15030
	ctx.lr = 0x827F9710;
	sub_82E15030(ctx, base);
	// 827F9710: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9714: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F9718: 4BAF4849  bl 0x822edf60
	ctx.lr = 0x827F971C;
	sub_822EDF60(ctx, base);
	// 827F971C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F9720: 485F9D09  bl 0x82df3428
	ctx.lr = 0x827F9724;
	sub_82DF3428(ctx, base);
	// 827F9724: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 827F9728: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F972C: 4BAF430D  bl 0x822eda38
	ctx.lr = 0x827F9730;
	sub_822EDA38(ctx, base);
	// 827F9730: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9734: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F9738: 4BFFFC29  bl 0x827f9360
	ctx.lr = 0x827F973C;
	sub_827F9360(ctx, base);
	// 827F973C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 827F9740: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 827F9744: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 827F9748: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F974C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 827F9750: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 827F9754: C00B89AC  lfs f0, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F9758: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827F975C: 13E05407  vcmpneb. (lvlx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F97A8 size=412
    let mut pc: u32 = 0x827F97A8;
    'dispatch: loop {
        match pc {
            0x827F97A8 => {
    //   block [0x827F97A8..0x827F9944)
	// 827F97A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F97AC: 489AE9C1  bl 0x831a816c
	ctx.lr = 0x827F97B0;
	sub_831A8130(ctx, base);
	// 827F97B0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F97B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F97B8: 817F0100  lwz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 827F97BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F97C0: 419A00B0  beq cr6, 0x827f9870
	if ctx.cr[6].eq {
	pc = 0x827F9870; continue 'dispatch;
	}
	// 827F97C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F97C8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827F97CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F97D0: 4E800421  bctrl
	ctx.lr = 0x827F97D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F97D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F97D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F97DC: 485FA22D  bl 0x82df3a08
	ctx.lr = 0x827F97E0;
	sub_82DF3A08(ctx, base);
	// 827F97E0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F97E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F97E8: 809F0100  lwz r4, 0x100(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 827F97EC: 4861B845  bl 0x82e15030
	ctx.lr = 0x827F97F0;
	sub_82E15030(ctx, base);
	// 827F97F0: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F97F4: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F97F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F97FC: 419A000C  beq cr6, 0x827f9808
	if ctx.cr[6].eq {
	pc = 0x827F9808; continue 'dispatch;
	}
	// 827F9800: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827F9804: 4BAC708D  bl 0x822c0890
	ctx.lr = 0x827F9808;
	sub_822C0890(ctx, base);
	// 827F9808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F980C: 485F9C1D  bl 0x82df3428
	ctx.lr = 0x827F9810;
	sub_82DF3428(ctx, base);
	// 827F9810: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F9814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F9818: 388B6928  addi r4, r11, 0x6928
	ctx.r[4].s64 = ctx.r[11].s64 + 26920;
	// 827F981C: 38A0010C  li r5, 0x10c
	ctx.r[5].s64 = 268;
	// 827F9820: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 827F9824: 485F8BC5  bl 0x82df23e8
	ctx.lr = 0x827F9828;
	sub_82DF23E8(ctx, base);
	// 827F9828: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827F982C: 41820038  beq 0x827f9864
	if ctx.cr[0].eq {
	pc = 0x827F9864; continue 'dispatch;
	}
	// 827F9830: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F9838: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F983C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F9840: 4E800421  bctrl
	ctx.lr = 0x827F9844;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F9844: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9848: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827F984C: 4868221D  bl 0x82e7ba68
	ctx.lr = 0x827F9850;
	sub_82E7BA68(ctx, base);
	// 827F9850: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9854: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F9858: 48619899  bl 0x82e130f0
	ctx.lr = 0x827F985C;
	sub_82E130F0(ctx, base);
	// 827F985C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9860: 48000008  b 0x827f9868
	pc = 0x827F9868; continue 'dispatch;
	// 827F9864: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F9868: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 827F986C: 4BAE84E5  bl 0x822e1d50
	ctx.lr = 0x827F9870;
	sub_822E1D50(ctx, base);
	// 827F9870: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9874: 83DF00F8  lwz r30, 0xf8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 827F9878: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F987C: 4BD1889D  bl 0x82512118
	ctx.lr = 0x827F9880;
	sub_82512118(ctx, base);
	// 827F9880: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F9884: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F9888: 486195A9  bl 0x82e12e30
	ctx.lr = 0x827F988C;
	sub_82E12E30(ctx, base);
	// 827F988C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F9890: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9894: 419A0008  beq cr6, 0x827f989c
	if ctx.cr[6].eq {
	pc = 0x827F989C; continue 'dispatch;
	}
	// 827F9898: 4BAC6FF9  bl 0x822c0890
	ctx.lr = 0x827F989C;
	sub_822C0890(ctx, base);
	// 827F989C: 809F00F8  lwz r4, 0xf8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 827F98A0: 807F00F0  lwz r3, 0xf0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 827F98A4: 4861972D  bl 0x82e12fd0
	ctx.lr = 0x827F98A8;
	sub_82E12FD0(ctx, base);
	// 827F98A8: 807F0108  lwz r3, 0x108(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F98AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F98B0: 419A0030  beq cr6, 0x827f98e0
	if ctx.cr[6].eq {
	pc = 0x827F98E0; continue 'dispatch;
	}
	// 827F98B4: 4BAF47FD  bl 0x822ee0b0
	ctx.lr = 0x827F98B8;
	sub_822EE0B0(ctx, base);
	// 827F98B8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F98BC: 809F0108  lwz r4, 0x108(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F98C0: 4BAF4A29  bl 0x822ee2e8
	ctx.lr = 0x827F98C4;
	sub_822EE2E8(ctx, base);
	// 827F98C4: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827F98C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F98CC: 419A0008  beq cr6, 0x827f98d4
	if ctx.cr[6].eq {
	pc = 0x827F98D4; continue 'dispatch;
	}
	// 827F98D0: 4BAC6FC1  bl 0x822c0890
	ctx.lr = 0x827F98D4;
	sub_822C0890(ctx, base);
	// 827F98D4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 827F98D8: 807F0108  lwz r3, 0x108(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F98DC: 4BAF415D  bl 0x822eda38
	ctx.lr = 0x827F98E0;
	sub_822EDA38(ctx, base);
	// 827F98E0: 809F0108  lwz r4, 0x108(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F98E4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827F98E8: 419A003C  beq cr6, 0x827f9924
	if ctx.cr[6].eq {
	pc = 0x827F9924; continue 'dispatch;
	}
	// 827F98EC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F98F0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F98F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F98F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F98FC: 4E800421  bctrl
	ctx.lr = 0x827F9900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F9900: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9908: 4BD18179  bl 0x82511a80
	ctx.lr = 0x827F990C;
	sub_82511A80(ctx, base);
	// 827F990C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F9910: 809F0108  lwz r4, 0x108(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F9914: 4BAF365D  bl 0x822ecf70
	ctx.lr = 0x827F9918;
	sub_822ECF70(ctx, base);
	// 827F9918: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F991C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9920: 4BD181D1  bl 0x82511af0
	ctx.lr = 0x827F9924;
	sub_82511AF0(ctx, base);
	// 827F9924: 807F0108  lwz r3, 0x108(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 827F9928: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F992C: 419A0008  beq cr6, 0x827f9934
	if ctx.cr[6].eq {
	pc = 0x827F9934; continue 'dispatch;
	}
	// 827F9930: 4BAED451  bl 0x822e6d80
	ctx.lr = 0x827F9934;
	sub_822E6D80(ctx, base);
	// 827F9934: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F9938: 997F011C  stb r11, 0x11c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u8 ) };
	// 827F993C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827F9940: 489AE87C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9948 size=116
    let mut pc: u32 = 0x827F9948;
    'dispatch: loop {
        match pc {
            0x827F9948 => {
    //   block [0x827F9948..0x827F99BC)
	// 827F9948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F994C: 489AE81D  bl 0x831a8168
	ctx.lr = 0x827F9950;
	sub_831A8130(ctx, base);
	// 827F9950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9954: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F9958: 83850000  lwz r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F995C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827F9960: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F9964: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9968: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F996C: 419A0040  beq cr6, 0x827f99ac
	if ctx.cr[6].eq {
	pc = 0x827F99AC; continue 'dispatch;
	}
	// 827F9970: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9974: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 827F9978: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F997C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F9980: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9984: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9988: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F998C: 4BACF32D  bl 0x822c8cb8
	ctx.lr = 0x827F9990;
	sub_822C8CB8(ctx, base);
	// 827F9990: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F9994: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9998: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F999C: 485F87ED  bl 0x82df2188
	ctx.lr = 0x827F99A0;
	sub_82DF2188(ctx, base);
	// 827F99A0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F99A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827F99A8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F99AC: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827F99B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F99B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F99B8: 489AE800  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F99C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F99C0 size=100
    let mut pc: u32 = 0x827F99C0;
    'dispatch: loop {
        match pc {
            0x827F99C0 => {
    //   block [0x827F99C0..0x827F9A24)
	// 827F99C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F99C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F99C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F99CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F99D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F99D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F99D8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F99DC: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F99E0: 48000024  b 0x827f9a04
	pc = 0x827F9A04; continue 'dispatch;
	// 827F99E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 827F99E8: 4BC59C99  bl 0x82453680
	ctx.lr = 0x827F99EC;
	sub_82453680(ctx, base);
	// 827F99EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F99F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F99F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F99F8: 4BFFFF51  bl 0x827f9948
	ctx.lr = 0x827F99FC;
	sub_827F9948(ctx, base);
	// 827F99FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9A00: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9A04: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F9A08: 409AFFDC  bne cr6, 0x827f99e4
	if !ctx.cr[6].eq {
	pc = 0x827F99E4; continue 'dispatch;
	}
	// 827F9A0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F9A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9A18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F9A1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F9A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9A28 size=12
    let mut pc: u32 = 0x827F9A28;
    'dispatch: loop {
        match pc {
            0x827F9A28 => {
    //   block [0x827F9A28..0x827F9A34)
	// 827F9A28: 8963011C  lbz r11, 0x11c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) } as u64;
	// 827F9A2C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F9A30: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9A34 size=8
    let mut pc: u32 = 0x827F9A34;
    'dispatch: loop {
        match pc {
            0x827F9A34 => {
    //   block [0x827F9A34..0x827F9A3C)
	// 827F9A34: 38830110  addi r4, r3, 0x110
	ctx.r[4].s64 = ctx.r[3].s64 + 272;
	// 827F9A38: 4BFFFF88  b 0x827f99c0
	sub_827F99C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9A3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9A3C size=4
    let mut pc: u32 = 0x827F9A3C;
    'dispatch: loop {
        match pc {
            0x827F9A3C => {
    //   block [0x827F9A3C..0x827F9A40)
	// 827F9A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9A40 size=276
    let mut pc: u32 = 0x827F9A40;
    'dispatch: loop {
        match pc {
            0x827F9A40 => {
    //   block [0x827F9A40..0x827F9B54)
	// 827F9A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9A44: 489AE725  bl 0x831a8168
	ctx.lr = 0x827F9A48;
	sub_831A8130(ctx, base);
	// 827F9A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9A4C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F9A50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F9A54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F9A58: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827F9A5C: 41820038  beq 0x827f9a94
	if ctx.cr[0].eq {
	pc = 0x827F9A94; continue 'dispatch;
	}
	// 827F9A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9A64: 489AFF25  bl 0x831a9988
	ctx.lr = 0x827F9A68;
	sub_831A9988(ctx, base);
	// 827F9A68: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F9A6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9A70: 386BD1DC  addi r3, r11, -0x2e24
	ctx.r[3].s64 = ctx.r[11].s64 + -11812;
	// 827F9A74: 489AE685  bl 0x831a80f8
	ctx.lr = 0x827F9A78;
	sub_831A80F8(ctx, base);
	// 827F9A78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9A7C: 41820018  beq 0x827f9a94
	if ctx.cr[0].eq {
	pc = 0x827F9A94; continue 'dispatch;
	}
	// 827F9A80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9A84: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827F9A88: 4BFFFA59  bl 0x827f94e0
	ctx.lr = 0x827F9A8C;
	sub_827F94E0(ctx, base);
	// 827F9A8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F9A90: 480000BC  b 0x827f9b4c
	pc = 0x827F9B4C; continue 'dispatch;
	// 827F9A94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F9A98: 419A00A4  beq cr6, 0x827f9b3c
	if ctx.cr[6].eq {
	pc = 0x827F9B3C; continue 'dispatch;
	}
	// 827F9A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9AA0: 489AFEE9  bl 0x831a9988
	ctx.lr = 0x827F9AA4;
	sub_831A9988(ctx, base);
	// 827F9AA4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F9AA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9AAC: 386BD1B0  addi r3, r11, -0x2e50
	ctx.r[3].s64 = ctx.r[11].s64 + -11856;
	// 827F9AB0: 489AE649  bl 0x831a80f8
	ctx.lr = 0x827F9AB4;
	sub_831A80F8(ctx, base);
	// 827F9AB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9AB8: 41820014  beq 0x827f9acc
	if ctx.cr[0].eq {
	pc = 0x827F9ACC; continue 'dispatch;
	}
	// 827F9ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9AC0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827F9AC4: 4BFFFCE5  bl 0x827f97a8
	ctx.lr = 0x827F9AC8;
	sub_827F97A8(ctx, base);
	// 827F9AC8: 4BFFFFC4  b 0x827f9a8c
	pc = 0x827F9A8C; continue 'dispatch;
	// 827F9ACC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F9AD0: 419A006C  beq cr6, 0x827f9b3c
	if ctx.cr[6].eq {
	pc = 0x827F9B3C; continue 'dispatch;
	}
	// 827F9AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9AD8: 489AFEB1  bl 0x831a9988
	ctx.lr = 0x827F9ADC;
	sub_831A9988(ctx, base);
	// 827F9ADC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 827F9AE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9AE4: 386B3644  addi r3, r11, 0x3644
	ctx.r[3].s64 = ctx.r[11].s64 + 13892;
	// 827F9AE8: 489AE611  bl 0x831a80f8
	ctx.lr = 0x827F9AEC;
	sub_831A80F8(ctx, base);
	// 827F9AEC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9AF0: 41820014  beq 0x827f9b04
	if ctx.cr[0].eq {
	pc = 0x827F9B04; continue 'dispatch;
	}
	// 827F9AF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9AF8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827F9AFC: 4BFFFF2D  bl 0x827f9a28
	ctx.lr = 0x827F9B00;
	sub_827F9A28(ctx, base);
	// 827F9B00: 4BFFFF8C  b 0x827f9a8c
	pc = 0x827F9A8C; continue 'dispatch;
	// 827F9B04: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F9B08: 419A0034  beq cr6, 0x827f9b3c
	if ctx.cr[6].eq {
	pc = 0x827F9B3C; continue 'dispatch;
	}
	// 827F9B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9B10: 489AFE79  bl 0x831a9988
	ctx.lr = 0x827F9B14;
	sub_831A9988(ctx, base);
	// 827F9B14: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F9B18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F9B1C: 386BD170  addi r3, r11, -0x2e90
	ctx.r[3].s64 = ctx.r[11].s64 + -11920;
	// 827F9B20: 489AE5D9  bl 0x831a80f8
	ctx.lr = 0x827F9B24;
	sub_831A80F8(ctx, base);
	// 827F9B24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9B28: 41820014  beq 0x827f9b3c
	if ctx.cr[0].eq {
	pc = 0x827F9B3C; continue 'dispatch;
	}
	// 827F9B2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9B30: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827F9B34: 4BFFF995  bl 0x827f94c8
	ctx.lr = 0x827F9B38;
	sub_827F94C8(ctx, base);
	// 827F9B38: 4BFFFF54  b 0x827f9a8c
	pc = 0x827F9A8C; continue 'dispatch;
	// 827F9B3C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827F9B40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F9B44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F9B48: 48003741  bl 0x827fd288
	ctx.lr = 0x827F9B4C;
	sub_827FD288(ctx, base);
	// 827F9B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F9B50: 489AE668  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9B58 size=184
    let mut pc: u32 = 0x827F9B58;
    'dispatch: loop {
        match pc {
            0x827F9B58 => {
    //   block [0x827F9B58..0x827F9C10)
	// 827F9B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9B60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9B64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9B68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9B6C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F9B70: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F9B74: 396B6994  addi r11, r11, 0x6994
	ctx.r[11].s64 = ctx.r[11].s64 + 27028;
	// 827F9B78: 394A697C  addi r10, r10, 0x697c
	ctx.r[10].s64 = ctx.r[10].s64 + 27004;
	// 827F9B7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F9B80: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 827F9B84: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F9B88: 4BD25201  bl 0x8251ed88
	ctx.lr = 0x827F9B8C;
	sub_8251ED88(ctx, base);
	// 827F9B8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F9B90: 809F0114  lwz r4, 0x114(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 827F9B94: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F9B98: 485F85F1  bl 0x82df2188
	ctx.lr = 0x827F9B9C;
	sub_82DF2188(ctx, base);
	// 827F9B9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F9BA0: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 827F9BA4: 807F010C  lwz r3, 0x10c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 827F9BA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9BAC: 419A0008  beq cr6, 0x827f9bb4
	if ctx.cr[6].eq {
	pc = 0x827F9BB4; continue 'dispatch;
	}
	// 827F9BB0: 4BAC6CE1  bl 0x822c0890
	ctx.lr = 0x827F9BB4;
	sub_822C0890(ctx, base);
	// 827F9BB4: 807F0104  lwz r3, 0x104(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 827F9BB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9BBC: 419A0008  beq cr6, 0x827f9bc4
	if ctx.cr[6].eq {
	pc = 0x827F9BC4; continue 'dispatch;
	}
	// 827F9BC0: 4BAC6CD1  bl 0x822c0890
	ctx.lr = 0x827F9BC4;
	sub_822C0890(ctx, base);
	// 827F9BC4: 807F00FC  lwz r3, 0xfc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 827F9BC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9BCC: 419A0008  beq cr6, 0x827f9bd4
	if ctx.cr[6].eq {
	pc = 0x827F9BD4; continue 'dispatch;
	}
	// 827F9BD0: 4BAC6CC1  bl 0x822c0890
	ctx.lr = 0x827F9BD4;
	sub_822C0890(ctx, base);
	// 827F9BD4: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 827F9BD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9BDC: 419A0008  beq cr6, 0x827f9be4
	if ctx.cr[6].eq {
	pc = 0x827F9BE4; continue 'dispatch;
	}
	// 827F9BE0: 4BAC6CB1  bl 0x822c0890
	ctx.lr = 0x827F9BE4;
	sub_822C0890(ctx, base);
	// 827F9BE4: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 827F9BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9BEC: 419A0008  beq cr6, 0x827f9bf4
	if ctx.cr[6].eq {
	pc = 0x827F9BF4; continue 'dispatch;
	}
	// 827F9BF0: 4BAC6CA1  bl 0x822c0890
	ctx.lr = 0x827F9BF4;
	sub_822C0890(ctx, base);
	// 827F9BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9BF8: 48003511  bl 0x827fd108
	ctx.lr = 0x827F9BFC;
	sub_827FD108(ctx, base);
	// 827F9BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F9C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9C08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F9C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9C10 size=8
    let mut pc: u32 = 0x827F9C10;
    'dispatch: loop {
        match pc {
            0x827F9C10 => {
    //   block [0x827F9C10..0x827F9C18)
	// 827F9C10: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827F9C14: 480000CC  b 0x827f9ce0
	sub_827F9CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9C18 size=200
    let mut pc: u32 = 0x827F9C18;
    'dispatch: loop {
        match pc {
            0x827F9C18 => {
    //   block [0x827F9C18..0x827F9CE0)
	// 827F9C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9C20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F9C24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9C28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9C30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F9C34: 48003485  bl 0x827fd0b8
	ctx.lr = 0x827F9C38;
	sub_827FD0B8(ctx, base);
	// 827F9C38: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F9C3C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F9C40: 396B6994  addi r11, r11, 0x6994
	ctx.r[11].s64 = ctx.r[11].s64 + 27028;
	// 827F9C44: 394A697C  addi r10, r10, 0x697c
	ctx.r[10].s64 = ctx.r[10].s64 + 27004;
	// 827F9C48: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F9C4C: 397F00E8  addi r11, r31, 0xe8
	ctx.r[11].s64 = ctx.r[31].s64 + 232;
	// 827F9C50: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F9C54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9C58: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 827F9C5C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F9C64: 917F00EC  stw r11, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 827F9C68: 419A0024  beq cr6, 0x827f9c8c
	if ctx.cr[6].eq {
	pc = 0x827F9C8C; continue 'dispatch;
	}
	// 827F9C6C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F9C70: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F9C74: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F9C78: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F9C7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F9C80: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F9C84: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F9C88: 4082FFE8  bne 0x827f9c70
	if !ctx.cr[0].eq {
	pc = 0x827F9C70; continue 'dispatch;
	}
	// 827F9C8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F9C90: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 827F9C94: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 827F9C98: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 827F9C9C: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 827F9CA0: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 827F9CA4: 93DF0100  stw r30, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[30].u32 ) };
	// 827F9CA8: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 827F9CAC: 93DF0108  stw r30, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[30].u32 ) };
	// 827F9CB0: 93DF010C  stw r30, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[30].u32 ) };
	// 827F9CB4: 4818FD5D  bl 0x82989a10
	ctx.lr = 0x827F9CB8;
	sub_82989A10(ctx, base);
	// 827F9CB8: 907F0114  stw r3, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[3].u32 ) };
	// 827F9CBC: 93DF0118  stw r30, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u32 ) };
	// 827F9CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9CC4: 9BDF011C  stb r30, 0x11c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[30].u8 ) };
	// 827F9CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F9CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9CD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F9CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F9CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9CE0 size=76
    let mut pc: u32 = 0x827F9CE0;
    'dispatch: loop {
        match pc {
            0x827F9CE0 => {
    //   block [0x827F9CE0..0x827F9D2C)
	// 827F9CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F9CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9CF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9CF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F9CFC: 4BFFFE5D  bl 0x827f9b58
	ctx.lr = 0x827F9D00;
	sub_827F9B58(ctx, base);
	// 827F9D00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9D04: 4182000C  beq 0x827f9d10
	if ctx.cr[0].eq {
	pc = 0x827F9D10; continue 'dispatch;
	}
	// 827F9D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9D0C: 485F86CD  bl 0x82df23d8
	ctx.lr = 0x827F9D10;
	sub_82DF23D8(ctx, base);
	// 827F9D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9D14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F9D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9D20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F9D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F9D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9D30 size=76
    let mut pc: u32 = 0x827F9D30;
    'dispatch: loop {
        match pc {
            0x827F9D30 => {
    //   block [0x827F9D30..0x827F9D7C)
	// 827F9D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9D34: 489AE439  bl 0x831a816c
	ctx.lr = 0x827F9D38;
	sub_831A8130(ctx, base);
	// 827F9D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9D3C: 83C30114  lwz r30, 0x114(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 827F9D40: 3BE30110  addi r31, r3, 0x110
	ctx.r[31].s64 = ctx.r[3].s64 + 272;
	// 827F9D44: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 827F9D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9D4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F9D50: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9D54: 481A1EFD  bl 0x8299bc50
	ctx.lr = 0x827F9D58;
	sub_8299BC50(ctx, base);
	// 827F9D58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F9D5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F9D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9D64: 4BD25095  bl 0x8251edf8
	ctx.lr = 0x827F9D68;
	sub_8251EDF8(ctx, base);
	// 827F9D68: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 827F9D6C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9D70: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827F9D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F9D78: 489AE444  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F9D80 size=68
    let mut pc: u32 = 0x827F9D80;
    'dispatch: loop {
        match pc {
            0x827F9D80 => {
    //   block [0x827F9D80..0x827F9DC4)
	// 827F9D80: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F9D84: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 827F9D88: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 827F9D8C: 3CE08208  lis r7, -0x7df8
	ctx.r[7].s64 = -2113404928;
	// 827F9D90: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 827F9D94: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F9D98: C1A9D7BC  lfs f13, -0x2844(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F9D9C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827F9DA0: C1889524  lfs f12, -0x6adc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F9DA4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827F9DA8: C16789AC  lfs f11, -0x7654(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827F9DAC: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827F9DB0: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F9DB4: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F9DB8: D1630014  stfs f11, 0x14(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 827F9DBC: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827F9DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9DC8 size=136
    let mut pc: u32 = 0x827F9DC8;
    'dispatch: loop {
        match pc {
            0x827F9DC8 => {
    //   block [0x827F9DC8..0x827F9E50)
	// 827F9DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9DD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F9DD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9DDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F9DE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F9DE4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827F9DE8: 409A0020  bne cr6, 0x827f9e08
	if !ctx.cr[6].eq {
	pc = 0x827F9E08; continue 'dispatch;
	}
	// 827F9DEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F9DF0: 419A0048  beq cr6, 0x827f9e38
	if ctx.cr[6].eq {
	pc = 0x827F9E38; continue 'dispatch;
	}
	// 827F9DF4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827F9DF8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 827F9DFC: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 827F9E00: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827F9E04: 48000034  b 0x827f9e38
	pc = 0x827F9E38; continue 'dispatch;
	// 827F9E08: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827F9E0C: 419A002C  beq cr6, 0x827f9e38
	if ctx.cr[6].eq {
	pc = 0x827F9E38; continue 'dispatch;
	}
	// 827F9E10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F9E14: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9E18: 388BD258  addi r4, r11, -0x2da8
	ctx.r[4].s64 = ctx.r[11].s64 + -11688;
	// 827F9E1C: 489AE2DD  bl 0x831a80f8
	ctx.lr = 0x827F9E20;
	sub_831A80F8(ctx, base);
	// 827F9E20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F9E24: 4182000C  beq 0x827f9e30
	if ctx.cr[0].eq {
	pc = 0x827F9E30; continue 'dispatch;
	}
	// 827F9E28: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827F9E2C: 4800000C  b 0x827f9e38
	pc = 0x827F9E38; continue 'dispatch;
	// 827F9E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F9E34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F9E38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F9E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9E44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F9E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F9E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9E50 size=96
    let mut pc: u32 = 0x827F9E50;
    'dispatch: loop {
        match pc {
            0x827F9E50 => {
    //   block [0x827F9E50..0x827F9EB0)
	// 827F9E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F9E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9E64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F9E68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F9E6C: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827F9E70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F9E74: 419A0018  beq cr6, 0x827f9e8c
	if ctx.cr[6].eq {
	pc = 0x827F9E8C; continue 'dispatch;
	}
	// 827F9E78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F9E7C: 4BFD54F5  bl 0x827cf370
	ctx.lr = 0x827F9E80;
	sub_827CF370(ctx, base);
	// 827F9E80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F9E84: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827F9E88: 4BFD5FB9  bl 0x827cfe40
	ctx.lr = 0x827F9E8C;
	sub_827CFE40(ctx, base);
	// 827F9E8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F9E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9E94: 4BD181D5  bl 0x82512068
	ctx.lr = 0x827F9E98;
	sub_82512068(ctx, base);
	// 827F9E98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F9E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9EA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F9EA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F9EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F9EB0 size=72
    let mut pc: u32 = 0x827F9EB0;
    'dispatch: loop {
        match pc {
            0x827F9EB0 => {
    //   block [0x827F9EB0..0x827F9EF8)
	// 827F9EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9EBC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827F9EC0: 419A001C  beq cr6, 0x827f9edc
	if ctx.cr[6].eq {
	pc = 0x827F9EDC; continue 'dispatch;
	}
	// 827F9EC4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F9EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F9ECC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827F9ED0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F9ED4: 4BFFFEF5  bl 0x827f9dc8
	ctx.lr = 0x827F9ED8;
	sub_827F9DC8(ctx, base);
	// 827F9ED8: 48000010  b 0x827f9ee8
	pc = 0x827F9EE8; continue 'dispatch;
	// 827F9EDC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F9EE0: 396BD258  addi r11, r11, -0x2da8
	ctx.r[11].s64 = ctx.r[11].s64 + -11688;
	// 827F9EE4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F9EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F9EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F9EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F9EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F9EF8 size=16
    let mut pc: u32 = 0x827F9EF8;
    'dispatch: loop {
        match pc {
            0x827F9EF8 => {
    //   block [0x827F9EF8..0x827F9F08)
	// 827F9EF8: 8124015C  lwz r9, 0x15c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(348 as u32) ) } as u64;
	// 827F9EFC: 39440158  addi r10, r4, 0x158
	ctx.r[10].s64 = ctx.r[4].s64 + 344;
	// 827F9F00: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9F04: 4800004C  b 0x827f9f50
	sub_827F9F08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F9F08 size=84
    let mut pc: u32 = 0x827F9F08;
    'dispatch: loop {
        match pc {
            0x827F9F08 => {
    //   block [0x827F9F08..0x827F9F5C)
	// 827F9F08: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F9F0C: C0030134  lfs f0, 0x134(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F9F10: 81090080  lwz r8, 0x80(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(128 as u32) ) } as u64;
	// 827F9F14: 9101FFE0  stw r8, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[8].u32 ) };
	// 827F9F18: 8901FFE0  lbz r8, -0x20(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 827F9F1C: F901FFE8  std r8, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[8].u64 ) };
	// 827F9F20: C9A1FFE8  lfd f13, -0x18(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F9F24: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 827F9F28: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 827F9F2C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827F9F30: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 827F9F34: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 827F9F38: 8901FFF7  lbz r8, -9(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-9 as u32) ) } as u64;
	// 827F9F3C: 9901FFE0  stb r8, -0x20(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[8].u8 ) };
	// 827F9F40: 8101FFE0  lwz r8, -0x20(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) } as u64;
	// 827F9F44: 91090080  stw r8, 0x80(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 827F9F48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F9F4C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F9F50: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 827F9F54: 409AFFB4  bne cr6, 0x827f9f08
	if !ctx.cr[6].eq {
	pc = 0x827F9F08; continue 'dispatch;
	}
	// 827F9F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F9F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F9F60 size=868
    let mut pc: u32 = 0x827F9F60;
    'dispatch: loop {
        match pc {
            0x827F9F60 => {
    //   block [0x827F9F60..0x827FA2C4)
	// 827F9F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F9F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F9F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F9F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F9F70: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 827F9F74: 489AEB05  bl 0x831a8a78
	ctx.lr = 0x827F9F78;
	sub_831A8A40(ctx, base);
	// 827F9F78: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F9F7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F9F80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F9F84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F9F88: 419A031C  beq cr6, 0x827fa2a4
	if ctx.cr[6].eq {
	pc = 0x827FA2A4; continue 'dispatch;
	}
	// 827F9F8C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F9F90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F9F94: 388B6AD4  addi r4, r11, 0x6ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 27348;
	// 827F9F98: 485F9A71  bl 0x82df3a08
	ctx.lr = 0x827F9F9C;
	sub_82DF3A08(ctx, base);
	// 827F9F9C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F9FA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F9FA4: 388B6AC4  addi r4, r11, 0x6ac4
	ctx.r[4].s64 = ctx.r[11].s64 + 27332;
	// 827F9FA8: 485F9A61  bl 0x82df3a08
	ctx.lr = 0x827F9FAC;
	sub_82DF3A08(ctx, base);
	// 827F9FAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F9FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827F9FB4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F9FB8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827F9FBC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F9FC0: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F9FC4: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827F9FC8: C3CA6218  lfs f30, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827F9FCC: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F9FD0: C3A908A4  lfs f29, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827F9FD4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F9FD8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F9FDC: 4BDA92CD  bl 0x825a32a8
	ctx.lr = 0x827F9FE0;
	sub_825A32A8(ctx, base);
	// 827F9FE0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F9FE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F9FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F9FEC: 4BDA7785  bl 0x825a1770
	ctx.lr = 0x827F9FF0;
	sub_825A1770(ctx, base);
	// 827F9FF0: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 827F9FF4: 485F9435  bl 0x82df3428
	ctx.lr = 0x827F9FF8;
	sub_82DF3428(ctx, base);
	// 827F9FF8: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 827F9FFC: 4BACECBD  bl 0x822c8cb8
	ctx.lr = 0x827FA000;
	sub_822C8CB8(ctx, base);
	// 827FA000: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA004: 485F9425  bl 0x82df3428
	ctx.lr = 0x827FA008;
	sub_82DF3428(ctx, base);
	// 827FA008: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA00C: 485F941D  bl 0x82df3428
	ctx.lr = 0x827FA010;
	sub_82DF3428(ctx, base);
	// 827FA010: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA018: 388B6AB4  addi r4, r11, 0x6ab4
	ctx.r[4].s64 = ctx.r[11].s64 + 27316;
	// 827FA01C: 485F99ED  bl 0x82df3a08
	ctx.lr = 0x827FA020;
	sub_82DF3A08(ctx, base);
	// 827FA020: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA024: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA028: 388B6AA4  addi r4, r11, 0x6aa4
	ctx.r[4].s64 = ctx.r[11].s64 + 27300;
	// 827FA02C: 485F99DD  bl 0x82df3a08
	ctx.lr = 0x827FA030;
	sub_82DF3A08(ctx, base);
	// 827FA030: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 827FA034: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA038: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827FA03C: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 827FA040: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827FA044: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FA048: 4BDA9261  bl 0x825a32a8
	ctx.lr = 0x827FA04C;
	sub_825A32A8(ctx, base);
	// 827FA04C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FA050: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827FA054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA058: 4BDA7719  bl 0x825a1770
	ctx.lr = 0x827FA05C;
	sub_825A1770(ctx, base);
	// 827FA05C: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 827FA060: 485F93C9  bl 0x82df3428
	ctx.lr = 0x827FA064;
	sub_82DF3428(ctx, base);
	// 827FA064: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 827FA068: 4BACEC51  bl 0x822c8cb8
	ctx.lr = 0x827FA06C;
	sub_822C8CB8(ctx, base);
	// 827FA06C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA070: 485F93B9  bl 0x82df3428
	ctx.lr = 0x827FA074;
	sub_82DF3428(ctx, base);
	// 827FA074: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA078: 485F93B1  bl 0x82df3428
	ctx.lr = 0x827FA07C;
	sub_82DF3428(ctx, base);
	// 827FA07C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA084: 388B6A98  addi r4, r11, 0x6a98
	ctx.r[4].s64 = ctx.r[11].s64 + 27288;
	// 827FA088: 485F9981  bl 0x82df3a08
	ctx.lr = 0x827FA08C;
	sub_82DF3A08(ctx, base);
	// 827FA08C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA090: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA094: 388B6A88  addi r4, r11, 0x6a88
	ctx.r[4].s64 = ctx.r[11].s64 + 27272;
	// 827FA098: 485F9971  bl 0x82df3a08
	ctx.lr = 0x827FA09C;
	sub_82DF3A08(ctx, base);
	// 827FA09C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FA0A0: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 827FA0A4: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827FA0A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA0AC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827FA0B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FA0B4: C38B9528  lfs f28, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 827FA0B8: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 827FA0BC: 4BDA91ED  bl 0x825a32a8
	ctx.lr = 0x827FA0C0;
	sub_825A32A8(ctx, base);
	// 827FA0C0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FA0C4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827FA0C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA0CC: 4BDA76A5  bl 0x825a1770
	ctx.lr = 0x827FA0D0;
	sub_825A1770(ctx, base);
	// 827FA0D0: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 827FA0D4: 485F9355  bl 0x82df3428
	ctx.lr = 0x827FA0D8;
	sub_82DF3428(ctx, base);
	// 827FA0D8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827FA0DC: 4BACEBDD  bl 0x822c8cb8
	ctx.lr = 0x827FA0E0;
	sub_822C8CB8(ctx, base);
	// 827FA0E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA0E4: 485F9345  bl 0x82df3428
	ctx.lr = 0x827FA0E8;
	sub_82DF3428(ctx, base);
	// 827FA0E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA0EC: 485F933D  bl 0x82df3428
	ctx.lr = 0x827FA0F0;
	sub_82DF3428(ctx, base);
	// 827FA0F0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA0F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA0F8: 388B6A78  addi r4, r11, 0x6a78
	ctx.r[4].s64 = ctx.r[11].s64 + 27256;
	// 827FA0FC: 485F990D  bl 0x82df3a08
	ctx.lr = 0x827FA100;
	sub_82DF3A08(ctx, base);
	// 827FA100: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA104: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA108: 388B6A64  addi r4, r11, 0x6a64
	ctx.r[4].s64 = ctx.r[11].s64 + 27236;
	// 827FA10C: 485F98FD  bl 0x82df3a08
	ctx.lr = 0x827FA110;
	sub_82DF3A08(ctx, base);
	// 827FA110: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827FA114: 38E003E8  li r7, 0x3e8
	ctx.r[7].s64 = 1000;
	// 827FA118: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FA11C: 38BE000C  addi r5, r30, 0xc
	ctx.r[5].s64 = ctx.r[30].s64 + 12;
	// 827FA120: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA124: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827FA128: 4BDA92F1  bl 0x825a3418
	ctx.lr = 0x827FA12C;
	sub_825A3418(ctx, base);
	// 827FA12C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FA130: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827FA134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA138: 4BDA7DA1  bl 0x825a1ed8
	ctx.lr = 0x827FA13C;
	sub_825A1ED8(ctx, base);
	// 827FA13C: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 827FA140: 485F92E9  bl 0x82df3428
	ctx.lr = 0x827FA144;
	sub_82DF3428(ctx, base);
	// 827FA144: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827FA148: 4BACEB71  bl 0x822c8cb8
	ctx.lr = 0x827FA14C;
	sub_822C8CB8(ctx, base);
	// 827FA14C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA150: 485F92D9  bl 0x82df3428
	ctx.lr = 0x827FA154;
	sub_82DF3428(ctx, base);
	// 827FA154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA158: 485F92D1  bl 0x82df3428
	ctx.lr = 0x827FA15C;
	sub_82DF3428(ctx, base);
	// 827FA15C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA164: 388B6A4C  addi r4, r11, 0x6a4c
	ctx.r[4].s64 = ctx.r[11].s64 + 27212;
	// 827FA168: 485F98A1  bl 0x82df3a08
	ctx.lr = 0x827FA16C;
	sub_82DF3A08(ctx, base);
	// 827FA16C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA170: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA174: 388B6A38  addi r4, r11, 0x6a38
	ctx.r[4].s64 = ctx.r[11].s64 + 27192;
	// 827FA178: 485F9891  bl 0x82df3a08
	ctx.lr = 0x827FA17C;
	sub_82DF3A08(ctx, base);
	// 827FA17C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827FA180: 38E003E8  li r7, 0x3e8
	ctx.r[7].s64 = 1000;
	// 827FA184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FA188: 38BE0010  addi r5, r30, 0x10
	ctx.r[5].s64 = ctx.r[30].s64 + 16;
	// 827FA18C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA190: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827FA194: 4BDA9285  bl 0x825a3418
	ctx.lr = 0x827FA198;
	sub_825A3418(ctx, base);
	// 827FA198: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FA19C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827FA1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA1A4: 4BDA7D35  bl 0x825a1ed8
	ctx.lr = 0x827FA1A8;
	sub_825A1ED8(ctx, base);
	// 827FA1A8: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 827FA1AC: 485F927D  bl 0x82df3428
	ctx.lr = 0x827FA1B0;
	sub_82DF3428(ctx, base);
	// 827FA1B0: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 827FA1B4: 4BACEB05  bl 0x822c8cb8
	ctx.lr = 0x827FA1B8;
	sub_822C8CB8(ctx, base);
	// 827FA1B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA1BC: 485F926D  bl 0x82df3428
	ctx.lr = 0x827FA1C0;
	sub_82DF3428(ctx, base);
	// 827FA1C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA1C4: 485F9265  bl 0x82df3428
	ctx.lr = 0x827FA1C8;
	sub_82DF3428(ctx, base);
	// 827FA1C8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA1CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA1D0: 388B6A20  addi r4, r11, 0x6a20
	ctx.r[4].s64 = ctx.r[11].s64 + 27168;
	// 827FA1D4: 485F9835  bl 0x82df3a08
	ctx.lr = 0x827FA1D8;
	sub_82DF3A08(ctx, base);
	// 827FA1D8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA1DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA1E0: 388B6A0C  addi r4, r11, 0x6a0c
	ctx.r[4].s64 = ctx.r[11].s64 + 27148;
	// 827FA1E4: 485F9825  bl 0x82df3a08
	ctx.lr = 0x827FA1E8;
	sub_82DF3A08(ctx, base);
	// 827FA1E8: 38BE0014  addi r5, r30, 0x14
	ctx.r[5].s64 = ctx.r[30].s64 + 20;
	// 827FA1EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA1F0: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827FA1F4: 38610160  addi r3, r1, 0x160
	ctx.r[3].s64 = ctx.r[1].s64 + 352;
	// 827FA1F8: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827FA1FC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FA200: 4BDA90A9  bl 0x825a32a8
	ctx.lr = 0x827FA204;
	sub_825A32A8(ctx, base);
	// 827FA204: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FA208: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827FA20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA210: 4BDA7561  bl 0x825a1770
	ctx.lr = 0x827FA214;
	sub_825A1770(ctx, base);
	// 827FA214: 38610198  addi r3, r1, 0x198
	ctx.r[3].s64 = ctx.r[1].s64 + 408;
	// 827FA218: 485F9211  bl 0x82df3428
	ctx.lr = 0x827FA21C;
	sub_82DF3428(ctx, base);
	// 827FA21C: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 827FA220: 4BACEA99  bl 0x822c8cb8
	ctx.lr = 0x827FA224;
	sub_822C8CB8(ctx, base);
	// 827FA224: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA228: 485F9201  bl 0x82df3428
	ctx.lr = 0x827FA22C;
	sub_82DF3428(ctx, base);
	// 827FA22C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA230: 485F91F9  bl 0x82df3428
	ctx.lr = 0x827FA234;
	sub_82DF3428(ctx, base);
	// 827FA234: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA23C: 388B69F4  addi r4, r11, 0x69f4
	ctx.r[4].s64 = ctx.r[11].s64 + 27124;
	// 827FA240: 485F97C9  bl 0x82df3a08
	ctx.lr = 0x827FA244;
	sub_82DF3A08(ctx, base);
	// 827FA244: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA248: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA24C: 388B69E0  addi r4, r11, 0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + 27104;
	// 827FA250: 485F97B9  bl 0x82df3a08
	ctx.lr = 0x827FA254;
	sub_82DF3A08(ctx, base);
	// 827FA254: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 827FA258: 38BE0018  addi r5, r30, 0x18
	ctx.r[5].s64 = ctx.r[30].s64 + 24;
	// 827FA25C: FC60E090  fmr f3, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[28].f64;
	// 827FA260: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA264: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FA268: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 827FA26C: C04B89AC  lfs f2, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 827FA270: 4BDA9039  bl 0x825a32a8
	ctx.lr = 0x827FA274;
	sub_825A32A8(ctx, base);
	// 827FA274: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FA278: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827FA27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA280: 4BDA74F1  bl 0x825a1770
	ctx.lr = 0x827FA284;
	sub_825A1770(ctx, base);
	// 827FA284: 38610218  addi r3, r1, 0x218
	ctx.r[3].s64 = ctx.r[1].s64 + 536;
	// 827FA288: 485F91A1  bl 0x82df3428
	ctx.lr = 0x827FA28C;
	sub_82DF3428(ctx, base);
	// 827FA28C: 386101F8  addi r3, r1, 0x1f8
	ctx.r[3].s64 = ctx.r[1].s64 + 504;
	// 827FA290: 4BACEA29  bl 0x822c8cb8
	ctx.lr = 0x827FA294;
	sub_822C8CB8(ctx, base);
	// 827FA294: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FA298: 485F9191  bl 0x82df3428
	ctx.lr = 0x827FA29C;
	sub_82DF3428(ctx, base);
	// 827FA29C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA2A0: 485F9189  bl 0x82df3428
	ctx.lr = 0x827FA2A4;
	sub_82DF3428(ctx, base);
	// 827FA2A4: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 827FA2A8: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 827FA2AC: 489AE819  bl 0x831a8ac4
	ctx.lr = 0x827FA2B0;
	sub_831A8A8C(ctx, base);
	// 827FA2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FA2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FA2B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FA2BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FA2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FA2C8 size=220
    let mut pc: u32 = 0x827FA2C8;
    'dispatch: loop {
        match pc {
            0x827FA2C8 => {
    //   block [0x827FA2C8..0x827FA3A4)
	// 827FA2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FA2D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FA2D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FA2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA2DC: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 827FA2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FA2E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827FA2E8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827FA2EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FA2F0: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 827FA2F4: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 827FA2F8: 388A85E4  addi r4, r10, -0x7a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -31260;
	// 827FA2FC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 827FA300: 38DF00F4  addi r6, r31, 0xf4
	ctx.r[6].s64 = ctx.r[31].s64 + 244;
	// 827FA304: 38BF00F0  addi r5, r31, 0xf0
	ctx.r[5].s64 = ctx.r[31].s64 + 240;
	// 827FA308: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FA30C: 480A8585  bl 0x828a2890
	ctx.lr = 0x827FA310;
	sub_828A2890(ctx, base);
	// 827FA310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FA314: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827FA318: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827FA31C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FA320: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827FA324: 419A0024  beq cr6, 0x827fa348
	if ctx.cr[6].eq {
	pc = 0x827FA348; continue 'dispatch;
	}
	// 827FA328: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827FA32C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827FA330: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FA334: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827FA338: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827FA33C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827FA340: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FA344: 4082FFE8  bne 0x827fa32c
	if !ctx.cr[0].eq {
	pc = 0x827FA32C; continue 'dispatch;
	}
	// 827FA348: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FA34C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FA350: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 827FA354: 388A6AE8  addi r4, r10, 0x6ae8
	ctx.r[4].s64 = ctx.r[10].s64 + 27368;
	// 827FA358: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827FA35C: 38A0011C  li r5, 0x11c
	ctx.r[5].s64 = 284;
	// 827FA360: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FA364: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827FA368: 4865E6D9  bl 0x82e58a40
	ctx.lr = 0x827FA36C;
	sub_82E58A40(ctx, base);
	// 827FA36C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FA370: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FA374: 419A0008  beq cr6, 0x827fa37c
	if ctx.cr[6].eq {
	pc = 0x827FA37C; continue 'dispatch;
	}
	// 827FA378: 4BAC6519  bl 0x822c0890
	ctx.lr = 0x827FA37C;
	sub_822C0890(ctx, base);
	// 827FA37C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827FA380: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FA384: 419A0008  beq cr6, 0x827fa38c
	if ctx.cr[6].eq {
	pc = 0x827FA38C; continue 'dispatch;
	}
	// 827FA388: 4BAC6509  bl 0x822c0890
	ctx.lr = 0x827FA38C;
	sub_822C0890(ctx, base);
	// 827FA38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FA390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FA394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FA398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FA39C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FA3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FA3A8 size=128
    let mut pc: u32 = 0x827FA3A8;
    'dispatch: loop {
        match pc {
            0x827FA3A8 => {
    //   block [0x827FA3A8..0x827FA428)
	// 827FA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA3AC: 489ADDC1  bl 0x831a816c
	ctx.lr = 0x827FA3B0;
	sub_831A8130(ctx, base);
	// 827FA3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA3B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827FA3B8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827FA3BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FA3C0: 3BEBAD58  addi r31, r11, -0x52a8
	ctx.r[31].s64 = ctx.r[11].s64 + -21160;
	// 827FA3C4: 816AAD60  lwz r11, -0x52a0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21152 as u32) ) } as u64;
	// 827FA3C8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827FA3CC: 40820024  bne 0x827fa3f0
	if !ctx.cr[0].eq {
	pc = 0x827FA3F0; continue 'dispatch;
	}
	// 827FA3D0: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 827FA3D4: 3D008280  lis r8, -0x7d80
	ctx.r[8].s64 = -2105540608;
	// 827FA3D8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827FA3DC: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 827FA3E0: 39089EB0  addi r8, r8, -0x6150
	ctx.r[8].s64 = ctx.r[8].s64 + -24912;
	// 827FA3E4: 916AAD60  stw r11, -0x52a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21152 as u32), ctx.r[11].u32 ) };
	// 827FA3E8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827FA3EC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827FA3F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827FA3F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827FA3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA3FC: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 827FA400: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827FA404: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827FA408: 4BE5A1B9  bl 0x826545c0
	ctx.lr = 0x827FA40C;
	sub_826545C0(ctx, base);
	// 827FA40C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FA410: 4182000C  beq 0x827fa41c
	if ctx.cr[0].eq {
	pc = 0x827FA41C; continue 'dispatch;
	}
	// 827FA414: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FA418: 48000008  b 0x827fa420
	pc = 0x827FA420; continue 'dispatch;
	// 827FA41C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827FA420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FA424: 489ADD98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FA428 size=192
    let mut pc: u32 = 0x827FA428;
    'dispatch: loop {
        match pc {
            0x827FA428 => {
    //   block [0x827FA428..0x827FA4E8)
	// 827FA428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA42C: 489ADD35  bl 0x831a8160
	ctx.lr = 0x827FA430;
	sub_831A8130(ctx, base);
	// 827FA430: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 827FA434: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FA43C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827FA440: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827FA444: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 827FA448: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 827FA44C: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 827FA450: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 827FA454: 4BD17E0D  bl 0x82512260
	ctx.lr = 0x827FA458;
	sub_82512260(ctx, base);
	// 827FA458: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA45C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FA460: 396B6B44  addi r11, r11, 0x6b44
	ctx.r[11].s64 = ctx.r[11].s64 + 27460;
	// 827FA464: 394A6B30  addi r10, r10, 0x6b30
	ctx.r[10].s64 = ctx.r[10].s64 + 27440;
	// 827FA468: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FA46C: 387F00E4  addi r3, r31, 0xe4
	ctx.r[3].s64 = ctx.r[31].s64 + 228;
	// 827FA470: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FA474: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 827FA478: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827FA47C: 489AE095  bl 0x831a8510
	ctx.lr = 0x827FA480;
	sub_831A8510(ctx, base);
	// 827FA480: 93DF0100  stw r30, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[30].u32 ) };
	// 827FA484: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 827FA488: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FA48C: 485F9775  bl 0x82df3c00
	ctx.lr = 0x827FA490;
	sub_82DF3C00(ctx, base);
	// 827FA490: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 827FA494: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827FA498: 485F9769  bl 0x82df3c00
	ctx.lr = 0x827FA49C;
	sub_82DF3C00(ctx, base);
	// 827FA49C: 814100E4  lwz r10, 0xe4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 827FA4A0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827FA4A4: D3FF0114  stfs f31, 0x114(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 827FA4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FA4AC: 935F010C  stw r26, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[26].u32 ) };
	// 827FA4B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA4B4: 915F0110  stw r10, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 827FA4B8: 917F0118  stw r11, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[11].u32 ) };
	// 827FA4BC: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FA4C0: 917F011C  stw r11, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 827FA4C4: 917F0120  stw r11, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 827FA4C8: D01F0134  stfs f0, 0x134(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 827FA4CC: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 827FA4D0: 917F013C  stw r11, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 827FA4D4: 917F0140  stw r11, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 827FA4D8: 917F0144  stw r11, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 827FA4DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FA4E0: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 827FA4E4: 489ADCCC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FA4E8 size=8
    let mut pc: u32 = 0x827FA4E8;
    'dispatch: loop {
        match pc {
            0x827FA4E8 => {
    //   block [0x827FA4E8..0x827FA4F0)
	// 827FA4E8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827FA4EC: 4800007C  b 0x827fa568
	sub_827FA568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FA4F0 size=120
    let mut pc: u32 = 0x827FA4F0;
    'dispatch: loop {
        match pc {
            0x827FA4F0 => {
    //   block [0x827FA4F0..0x827FA568)
	// 827FA4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FA4F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FA4FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FA504: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 827FA508: 4BC6FAF9  bl 0x8246a000
	ctx.lr = 0x827FA50C;
	sub_8246A000(ctx, base);
	// 827FA50C: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FA510: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FA514: 419A0018  beq cr6, 0x827fa52c
	if ctx.cr[6].eq {
	pc = 0x827FA52C; continue 'dispatch;
	}
	// 827FA518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FA51C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FA520: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FA524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FA528: 4E800421  bctrl
	ctx.lr = 0x827FA52C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FA52C: 807F011C  lwz r3, 0x11c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 827FA530: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FA534: 419A0008  beq cr6, 0x827fa53c
	if ctx.cr[6].eq {
	pc = 0x827FA53C; continue 'dispatch;
	}
	// 827FA538: 4BAC6359  bl 0x822c0890
	ctx.lr = 0x827FA53C;
	sub_822C0890(ctx, base);
	// 827FA53C: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 827FA540: 485F8EE9  bl 0x82df3428
	ctx.lr = 0x827FA544;
	sub_82DF3428(ctx, base);
	// 827FA544: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 827FA548: 485F8EE1  bl 0x82df3428
	ctx.lr = 0x827FA54C;
	sub_82DF3428(ctx, base);
	// 827FA54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA550: 4BB54E01  bl 0x8234f350
	ctx.lr = 0x827FA554;
	sub_8234F350(ctx, base);
	// 827FA554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FA558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FA55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FA560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FA564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FA568 size=76
    let mut pc: u32 = 0x827FA568;
    'dispatch: loop {
        match pc {
            0x827FA568 => {
    //   block [0x827FA568..0x827FA5B4)
	// 827FA568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FA570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FA574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FA578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA57C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FA580: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FA584: 4BFFFF6D  bl 0x827fa4f0
	ctx.lr = 0x827FA588;
	sub_827FA4F0(ctx, base);
	// 827FA588: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FA58C: 4182000C  beq 0x827fa598
	if ctx.cr[0].eq {
	pc = 0x827FA598; continue 'dispatch;
	}
	// 827FA590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA594: 485F7E45  bl 0x82df23d8
	ctx.lr = 0x827FA598;
	sub_82DF23D8(ctx, base);
	// 827FA598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA59C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FA5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FA5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FA5A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FA5AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FA5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FA5B8 size=896
    let mut pc: u32 = 0x827FA5B8;
    'dispatch: loop {
        match pc {
            0x827FA5B8 => {
    //   block [0x827FA5B8..0x827FA938)
	// 827FA5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA5BC: 489ADB91  bl 0x831a814c
	ctx.lr = 0x827FA5C0;
	sub_831A8130(ctx, base);
	// 827FA5C0: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 827FA5C4: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA5C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FA5CC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 827FA5D0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FA5D4: 4BD17405  bl 0x825119d8
	ctx.lr = 0x827FA5D8;
	sub_825119D8(ctx, base);
	// 827FA5D8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 827FA5DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA5E0: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 827FA5E4: 485F9425  bl 0x82df3a08
	ctx.lr = 0x827FA5E8;
	sub_82DF3A08(ctx, base);
	// 827FA5E8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FA5EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA5F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FA5F4: 4BD0E18D  bl 0x82508780
	ctx.lr = 0x827FA5F8;
	sub_82508780(ctx, base);
	// 827FA5F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA5FC: 485F8E2D  bl 0x82df3428
	ctx.lr = 0x827FA600;
	sub_82DF3428(ctx, base);
	// 827FA600: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 827FA604: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FA608: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 827FA60C: 409A0008  bne cr6, 0x827fa614
	if !ctx.cr[6].eq {
	pc = 0x827FA614; continue 'dispatch;
	}
	// 827FA610: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 827FA614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FA618: 4BD0E189  bl 0x825087a0
	ctx.lr = 0x827FA61C;
	sub_825087A0(ctx, base);
	// 827FA61C: C01F00E4  lfs f0, 0xe4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FA620: C1BF00EC  lfs f13, 0xec(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FA624: 92FF0124  stw r23, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[23].u32 ) };
	// 827FA628: D01F0128  stfs f0, 0x128(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 827FA62C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 827FA630: D1BF012C  stfs f13, 0x12c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 827FA634: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 827FA638: 4BE82F31  bl 0x8267d568
	ctx.lr = 0x827FA63C;
	sub_8267D568(ctx, base);
	// 827FA63C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FA640: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FA644: 3B0B6AE8  addi r24, r11, 0x6ae8
	ctx.r[24].s64 = ctx.r[11].s64 + 27368;
	// 827FA648: 38A00087  li r5, 0x87
	ctx.r[5].s64 = 135;
	// 827FA64C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 827FA650: 386000A8  li r3, 0xa8
	ctx.r[3].s64 = 168;
	// 827FA654: 485F7D95  bl 0x82df23e8
	ctx.lr = 0x827FA658;
	sub_82DF23E8(ctx, base);
	// 827FA658: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FA65C: 41820010  beq 0x827fa66c
	if ctx.cr[0].eq {
	pc = 0x827FA66C; continue 'dispatch;
	}
	// 827FA660: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FA664: 4BFD5EED  bl 0x827d0550
	ctx.lr = 0x827FA668;
	sub_827D0550(ctx, base);
	// 827FA668: 48000008  b 0x827fa670
	pc = 0x827FA670; continue 'dispatch;
	// 827FA66C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 827FA670: 817F0120  lwz r11, 0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FA674: 3BDF0120  addi r30, r31, 0x120
	ctx.r[30].s64 = ctx.r[31].s64 + 288;
	// 827FA678: 907F0120  stw r3, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[3].u32 ) };
	// 827FA67C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FA680: 419A001C  beq cr6, 0x827fa69c
	if ctx.cr[6].eq {
	pc = 0x827FA69C; continue 'dispatch;
	}
	// 827FA684: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FA688: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827FA68C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FA690: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FA694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FA698: 4E800421  bctrl
	ctx.lr = 0x827FA69C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FA69C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FA6A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA6A4: 3B4B9BC9  addi r26, r11, -0x6437
	ctx.r[26].s64 = ctx.r[11].s64 + -25655;
	// 827FA6A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827FA6AC: 485F935D  bl 0x82df3a08
	ctx.lr = 0x827FA6B0;
	sub_82DF3A08(ctx, base);
	// 827FA6B0: 3BBF0104  addi r29, r31, 0x104
	ctx.r[29].s64 = ctx.r[31].s64 + 260;
	// 827FA6B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FA6B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FA6BC: 485F8BE5  bl 0x82df32a0
	ctx.lr = 0x827FA6C0;
	sub_82DF32A0(ctx, base);
	// 827FA6C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827FA6C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FA6C8: 485F8D61  bl 0x82df3428
	ctx.lr = 0x827FA6CC;
	sub_82DF3428(ctx, base);
	// 827FA6CC: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FA6D0: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 827FA6D4: 41820158  beq 0x827fa82c
	if ctx.cr[0].eq {
	pc = 0x827FA82C; continue 'dispatch;
	}
	// 827FA6D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827FA6DC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 827FA6E0: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 827FA6E4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 827FA6E8: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 827FA6EC: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 827FA6F0: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 827FA6F4: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 827FA6F8: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FA6FC: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FA700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA704: 13B95C07  vcmpneb. (lvlx128) v29, v25, v11
	tmp.u32 = ctx.r[25].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FA708: 13895C07  vcmpneb. (lvlx128) v28, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FA938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FA938 size=760
    let mut pc: u32 = 0x827FA938;
    'dispatch: loop {
        match pc {
            0x827FA938 => {
    //   block [0x827FA938..0x827FAC30)
	// 827FA938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FA93C: 489AD829  bl 0x831a8164
	ctx.lr = 0x827FA940;
	sub_831A8130(ctx, base);
	// 827FA940: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 827FA944: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 827FA948: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FA94C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FA950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FA954: 4BD15DB5  bl 0x82510708
	ctx.lr = 0x827FA958;
	sub_82510708(ctx, base);
	// 827FA958: C01F012C  lfs f0, 0x12c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FA95C: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 827FA960: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FA964: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 827FA968: D01F012C  stfs f0, 0x12c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 827FA96C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827FA970: 41980080  blt cr6, 0x827fa9f0
	if ctx.cr[6].lt {
	pc = 0x827FA9F0; continue 'dispatch;
	}
	// 827FA974: 409A006C  bne cr6, 0x827fa9e0
	if !ctx.cr[6].eq {
	pc = 0x827FA9E0; continue 'dispatch;
	}
	// 827FA978: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FA97C: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FA980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FA984: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827FA988: 419A0044  beq cr6, 0x827fa9cc
	if ctx.cr[6].eq {
	pc = 0x827FA9CC; continue 'dispatch;
	}
	// 827FA98C: 809F0130  lwz r4, 0x130(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 827FA990: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827FA994: 419A0038  beq cr6, 0x827fa9cc
	if ctx.cr[6].eq {
	pc = 0x827FA9CC; continue 'dispatch;
	}
	// 827FA998: C1BF00FC  lfs f13, 0xfc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FA99C: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 827FA9A0: D01F0134  stfs f0, 0x134(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 827FA9A4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 827FA9A8: 41990008  bgt cr6, 0x827fa9b0
	if ctx.cr[6].gt {
	pc = 0x827FA9B0; continue 'dispatch;
	}
	// 827FA9AC: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 827FA9B0: D01F0134  stfs f0, 0x134(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 827FA9B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FA9B8: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 827FA9BC: C08B08A8  lfs f4, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 827FA9C0: FC602090  fmr f3, f4
	ctx.f[3].f64 = ctx.f[4].f64;
	// 827FA9C4: FC402090  fmr f2, f4
	ctx.f[2].f64 = ctx.f[4].f64;
	// 827FA9C8: 4BFD4D79  bl 0x827cf740
	ctx.lr = 0x827FA9CC;
	sub_827CF740(ctx, base);
	// 827FA9CC: C01F012C  lfs f0, 0x12c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FA9D0: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 827FA9D4: 4098000C  bge cr6, 0x827fa9e0
	if !ctx.cr[6].lt {
	pc = 0x827FA9E0; continue 'dispatch;
	}
	// 827FA9D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FA9DC: 4BD15BF5  bl 0x825105d0
	ctx.lr = 0x827FA9E0;
	sub_825105D0(ctx, base);
	// 827FA9E0: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 827FA9E4: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 827FA9E8: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 827FA9EC: 489AD7C8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 827FA9F0: C1BF00EC  lfs f13, 0xec(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FA9F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FA9F8: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 827FA9FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FAA00: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FAA04: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827FAA08: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 827FAA0C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 827FAA10: 41990018  bgt cr6, 0x827faa28
	if ctx.cr[6].gt {
	pc = 0x827FAA28; continue 'dispatch;
	}
	// 827FAA14: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 827FAA18: FF0DF800  fcmpu cr6, f13, f31
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[31].f64);
	// 827FAA1C: 4098000C  bge cr6, 0x827faa28
	if !ctx.cr[6].lt {
	pc = 0x827FAA28; continue 'dispatch;
	}
	// 827FAA20: FDA0F890  fmr f13, f31
	ctx.f[13].f64 = ctx.f[31].f64;
	// 827FAA24: 48000008  b 0x827faa2c
	pc = 0x827FAA2C; continue 'dispatch;
	// 827FAA28: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 827FAA2C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 827FAA30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827FAA34: C00BDFAC  lfs f0, -0x2054(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FAA38: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 827FAA3C: C00A9450  lfs f0, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FAA40: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827FAA44: 489AE385  bl 0x831a8dc8
	ctx.lr = 0x827FAA48;
	sub_831A8DC8(ctx, base);
	// 827FAA48: C01F00E4  lfs f0, 0xe4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FAA4C: FDA00818  frsp f13, f1
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827FAA50: C19F00E8  lfs f12, 0xe8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827FAA54: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FAA58: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 827FAA5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FAA60: EC0C037A  fmadds f0, f12, f13, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 827FAA64: D01F0128  stfs f0, 0x128(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 827FAA68: 419A001C  beq cr6, 0x827faa84
	if ctx.cr[6].eq {
	pc = 0x827FAA84; continue 'dispatch;
	}
	// 827FAA6C: 809F0130  lwz r4, 0x130(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 827FAA70: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827FAA74: 419A0010  beq cr6, 0x827faa84
	if ctx.cr[6].eq {
	pc = 0x827FAA84; continue 'dispatch;
	}
	// 827FAA78: C1BF0114  lfs f13, 0x114(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FAA7C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827FAA80: 4BFD4C59  bl 0x827cf6d8
	ctx.lr = 0x827FAA84;
	sub_827CF6D8(ctx, base);
	// 827FAA84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FAA88: 4BD17061  bl 0x82511ae8
	ctx.lr = 0x827FAA8C;
	sub_82511AE8(ctx, base);
	// 827FAA8C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827FAA90: 807F0118  lwz r3, 0x118(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 827FAA94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827FAA98: 4BC974E1  bl 0x82491f78
	ctx.lr = 0x827FAA9C;
	sub_82491F78(ctx, base);
	// 827FAA9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FAAA0: 4BC6BA39  bl 0x824664d8
	ctx.lr = 0x827FAAA4;
	sub_824664D8(ctx, base);
	// 827FAAA4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 827FAAA8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 827FAAAC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827FAAB0: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 827FAAB4: 83DF0118  lwz r30, 0x118(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 827FAAB8: 4880E501  bl 0x83008fb8
	ctx.lr = 0x827FAABC;
	sub_83008FB8(ctx, base);
	// 827FAABC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827FAAC0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827FAAC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FAAC8: 4BC97489  bl 0x82491f50
	ctx.lr = 0x827FAACC;
	sub_82491F50(ctx, base);
	// 827FAACC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827FAAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FAAD4: 419A0118  beq cr6, 0x827fabec
	if ctx.cr[6].eq {
	pc = 0x827FABEC; continue 'dispatch;
	}
	// 827FAAD8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FAADC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FAAE0: 48000104  b 0x827fabe4
	pc = 0x827FABE4; continue 'dispatch;
	// 827FAAE4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827FAAE8: 4BFDF4E9  bl 0x827d9fd0
	ctx.lr = 0x827FAAEC;
	sub_827D9FD0(ctx, base);
	// 827FAAEC: 4880E4CD  bl 0x83008fb8
	ctx.lr = 0x827FAAF0;
	sub_83008FB8(ctx, base);
	// 827FAAF0: 817F0100  lwz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 827FAAF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FAAF8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 827FAAFC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827FAB00: 419A00DC  beq cr6, 0x827fabdc
	if ctx.cr[6].eq {
	pc = 0x827FABDC; continue 'dispatch;
	}
	// 827FAB04: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 827FAB08: 815F0140  lwz r10, 0x140(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 827FAB0C: 48000014  b 0x827fab20
	pc = 0x827FAB20; continue 'dispatch;
	// 827FAB10: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FAB14: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 827FAB18: 419A0010  beq cr6, 0x827fab28
	if ctx.cr[6].eq {
	pc = 0x827FAB28; continue 'dispatch;
	}
	// 827FAB1C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827FAB20: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827FAB24: 409AFFEC  bne cr6, 0x827fab10
	if !ctx.cr[6].eq {
	pc = 0x827FAB10; continue 'dispatch;
	}
	// 827FAB28: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827FAB2C: 409A00B0  bne cr6, 0x827fabdc
	if !ctx.cr[6].eq {
	pc = 0x827FABDC; continue 'dispatch;
	}
	// 827FAB30: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827FAB34: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827FAB38: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FAB3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827FAB40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FAB44: 4E800421  bctrl
	ctx.lr = 0x827FAB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FAB48: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 827FAB4C: 13E0E0C7  vcmpequd (lvx128) v31, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FAB50: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 827FAB54: C01F0128  lfs f0, 0x128(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FAB58: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 827FAB5C: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 827FAB60: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FAC30 size=20
    let mut pc: u32 = 0x827FAC30;
    'dispatch: loop {
        match pc {
            0x827FAC30 => {
    //   block [0x827FAC30..0x827FAC44)
	// 827FAC30: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 827FAC34: 91630140  stw r11, 0x140(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 827FAC38: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 827FAC3C: 91630144  stw r11, 0x144(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 827FAC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FAC48 size=12
    let mut pc: u32 = 0x827FAC48;
    'dispatch: loop {
        match pc {
            0x827FAC48 => {
    //   block [0x827FAC48..0x827FAC54)
	// 827FAC48: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 827FAC4C: 99630138  stb r11, 0x138(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[11].u8 ) };
	// 827FAC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FAC58 size=12
    let mut pc: u32 = 0x827FAC58;
    'dispatch: loop {
        match pc {
            0x827FAC58 => {
    //   block [0x827FAC58..0x827FAC64)
	// 827FAC58: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FAC5C: 386B6C4C  addi r3, r11, 0x6c4c
	ctx.r[3].s64 = ctx.r[11].s64 + 27724;
	// 827FAC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FAC68 size=136
    let mut pc: u32 = 0x827FAC68;
    'dispatch: loop {
        match pc {
            0x827FAC68 => {
    //   block [0x827FAC68..0x827FACF0)
	// 827FAC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FAC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FAC70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FAC74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAC78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAC7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FAC80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FAC84: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 827FAC88: 409A0020  bne cr6, 0x827faca8
	if !ctx.cr[6].eq {
	pc = 0x827FACA8; continue 'dispatch;
	}
	// 827FAC8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FAC90: 419A0048  beq cr6, 0x827facd8
	if ctx.cr[6].eq {
	pc = 0x827FACD8; continue 'dispatch;
	}
	// 827FAC94: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827FAC98: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 827FAC9C: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 827FACA0: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 827FACA4: 48000034  b 0x827facd8
	pc = 0x827FACD8; continue 'dispatch;
	// 827FACA8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 827FACAC: 419A002C  beq cr6, 0x827facd8
	if ctx.cr[6].eq {
	pc = 0x827FACD8; continue 'dispatch;
	}
	// 827FACB0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FACB4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FACB8: 388BD398  addi r4, r11, -0x2c68
	ctx.r[4].s64 = ctx.r[11].s64 + -11368;
	// 827FACBC: 489AD43D  bl 0x831a80f8
	ctx.lr = 0x827FACC0;
	sub_831A80F8(ctx, base);
	// 827FACC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FACC4: 4182000C  beq 0x827facd0
	if ctx.cr[0].eq {
	pc = 0x827FACD0; continue 'dispatch;
	}
	// 827FACC8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827FACCC: 4800000C  b 0x827facd8
	pc = 0x827FACD8; continue 'dispatch;
	// 827FACD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FACD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FACD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FACE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FACE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FACEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FACF0 size=68
    let mut pc: u32 = 0x827FACF0;
    'dispatch: loop {
        match pc {
            0x827FACF0 => {
    //   block [0x827FACF0..0x827FAD34)
	// 827FACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FACF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FACFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAD00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAD04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FAD08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FAD0C: 4BFFE6DD  bl 0x827f93e8
	ctx.lr = 0x827FAD10;
	sub_827F93E8(ctx, base);
	// 827FAD10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FAD14: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FAD18: 4BFEF479  bl 0x827ea190
	ctx.lr = 0x827FAD1C;
	sub_827EA190(ctx, base);
	// 827FAD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FAD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FAD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FAD28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FAD2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FAD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FAD38 size=60
    let mut pc: u32 = 0x827FAD38;
    'dispatch: loop {
        match pc {
            0x827FAD38 => {
    //   block [0x827FAD38..0x827FAD74)
	// 827FAD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FAD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FAD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAD48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FAD4C: 38840018  addi r4, r4, 0x18
	ctx.r[4].s64 = ctx.r[4].s64 + 24;
	// 827FAD50: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FAD54: 4BFEF78D  bl 0x827ea4e0
	ctx.lr = 0x827FAD58;
	sub_827EA4E0(ctx, base);
	// 827FAD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FAD5C: 997F0138  stb r11, 0x138(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u8 ) };
	// 827FAD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FAD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FAD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FAD6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FAD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FAD78 size=196
    let mut pc: u32 = 0x827FAD78;
    'dispatch: loop {
        match pc {
            0x827FAD78 => {
    //   block [0x827FAD78..0x827FAE3C)
	// 827FAD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FAD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FAD80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FAD84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAD88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAD8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FAD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FAD94: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FAD98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FAD9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FADA0: 4BAC5B99  bl 0x822c0938
	ctx.lr = 0x827FADA4;
	sub_822C0938(ctx, base);
	// 827FADA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FADA8: 41820028  beq 0x827fadd0
	if ctx.cr[0].eq {
	pc = 0x827FADD0; continue 'dispatch;
	}
	// 827FADAC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FADB0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FADB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FADB8: 392B6B90  addi r9, r11, 0x6b90
	ctx.r[9].s64 = ctx.r[11].s64 + 27536;
	// 827FADBC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FADC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FADC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FADC8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FADCC: 48000008  b 0x827fadd4
	pc = 0x827FADD4; continue 'dispatch;
	// 827FADD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FADD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FADD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FADDC: 409A0044  bne cr6, 0x827fae20
	if !ctx.cr[6].eq {
	pc = 0x827FAE20; continue 'dispatch;
	}
	// 827FADE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FADE4: 419A001C  beq cr6, 0x827fae00
	if ctx.cr[6].eq {
	pc = 0x827FAE00; continue 'dispatch;
	}
	// 827FADE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FADEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FADF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FADF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827FADF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FADFC: 4E800421  bctrl
	ctx.lr = 0x827FAE00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FAE00: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FAE04: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FAE08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FAE0C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FAE10: 816BD348  lwz r11, -0x2cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11448 as u32) ) } as u64;
	// 827FAE14: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FAE18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FAE1C: 4BAC51E5  bl 0x822c0000
	ctx.lr = 0x827FAE20;
	sub_822C0000(ctx, base);
	// 827FAE20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FAE24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FAE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FAE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FAE30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FAE34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FAE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FAE40 size=120
    let mut pc: u32 = 0x827FAE40;
    'dispatch: loop {
        match pc {
            0x827FAE40 => {
    //   block [0x827FAE40..0x827FAEB8)
	// 827FAE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FAE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FAE48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAE4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAE50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FAE54: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FAE58: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FAE5C: 396B6BE4  addi r11, r11, 0x6be4
	ctx.r[11].s64 = ctx.r[11].s64 + 27620;
	// 827FAE60: 394A6BCC  addi r10, r10, 0x6bcc
	ctx.r[10].s64 = ctx.r[10].s64 + 27596;
	// 827FAE64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FAE68: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FAE6C: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 827FAE70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FAE74: 419A0008  beq cr6, 0x827fae7c
	if ctx.cr[6].eq {
	pc = 0x827FAE7C; continue 'dispatch;
	}
	// 827FAE78: 4BAC5A19  bl 0x822c0890
	ctx.lr = 0x827FAE7C;
	sub_822C0890(ctx, base);
	// 827FAE7C: 807F012C  lwz r3, 0x12c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 827FAE80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FAE84: 419A0008  beq cr6, 0x827fae8c
	if ctx.cr[6].eq {
	pc = 0x827FAE8C; continue 'dispatch;
	}
	// 827FAE88: 4BAC5A09  bl 0x822c0890
	ctx.lr = 0x827FAE8C;
	sub_822C0890(ctx, base);
	// 827FAE8C: 807F0124  lwz r3, 0x124(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 827FAE90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FAE94: 419A0008  beq cr6, 0x827fae9c
	if ctx.cr[6].eq {
	pc = 0x827FAE9C; continue 'dispatch;
	}
	// 827FAE98: 4BAC59F9  bl 0x822c0890
	ctx.lr = 0x827FAE9C;
	sub_822C0890(ctx, base);
	// 827FAE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FAEA0: 4BFFECB9  bl 0x827f9b58
	ctx.lr = 0x827FAEA4;
	sub_827F9B58(ctx, base);
	// 827FAEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FAEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FAEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FAEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FAEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FAEB8 size=8
    let mut pc: u32 = 0x827FAEB8;
    'dispatch: loop {
        match pc {
            0x827FAEB8 => {
    //   block [0x827FAEB8..0x827FAEC0)
	// 827FAEB8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827FAEBC: 48000264  b 0x827fb120
	sub_827FB120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FAEC0 size=196
    let mut pc: u32 = 0x827FAEC0;
    'dispatch: loop {
        match pc {
            0x827FAEC0 => {
    //   block [0x827FAEC0..0x827FAF84)
	// 827FAEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FAEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FAEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FAECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAED4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FAED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FAEDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FAEE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FAEE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FAEE8: 4BAC5A51  bl 0x822c0938
	ctx.lr = 0x827FAEEC;
	sub_822C0938(ctx, base);
	// 827FAEEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FAEF0: 41820028  beq 0x827faf18
	if ctx.cr[0].eq {
	pc = 0x827FAF18; continue 'dispatch;
	}
	// 827FAEF4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FAEF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FAEFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FAF00: 392B6BA4  addi r9, r11, 0x6ba4
	ctx.r[9].s64 = ctx.r[11].s64 + 27556;
	// 827FAF04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FAF08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FAF0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FAF10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FAF14: 48000008  b 0x827faf1c
	pc = 0x827FAF1C; continue 'dispatch;
	// 827FAF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FAF1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FAF20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FAF24: 409A0044  bne cr6, 0x827faf68
	if !ctx.cr[6].eq {
	pc = 0x827FAF68; continue 'dispatch;
	}
	// 827FAF28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FAF2C: 419A001C  beq cr6, 0x827faf48
	if ctx.cr[6].eq {
	pc = 0x827FAF48; continue 'dispatch;
	}
	// 827FAF30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FAF34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FAF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FAF3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FAF40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FAF44: 4E800421  bctrl
	ctx.lr = 0x827FAF48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FAF48: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FAF4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FAF50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FAF54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FAF58: 816BD348  lwz r11, -0x2cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11448 as u32) ) } as u64;
	// 827FAF5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FAF60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FAF64: 4BAC509D  bl 0x822c0000
	ctx.lr = 0x827FAF68;
	sub_822C0000(ctx, base);
	// 827FAF68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FAF6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FAF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FAF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FAF78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FAF7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FAF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FAF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FAF88 size=196
    let mut pc: u32 = 0x827FAF88;
    'dispatch: loop {
        match pc {
            0x827FAF88 => {
    //   block [0x827FAF88..0x827FB04C)
	// 827FAF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FAF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FAF90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FAF94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FAF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FAF9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FAFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FAFA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FAFA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FAFAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FAFB0: 4BAC5989  bl 0x822c0938
	ctx.lr = 0x827FAFB4;
	sub_822C0938(ctx, base);
	// 827FAFB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FAFB8: 41820028  beq 0x827fafe0
	if ctx.cr[0].eq {
	pc = 0x827FAFE0; continue 'dispatch;
	}
	// 827FAFBC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FAFC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FAFC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FAFC8: 392B6BB8  addi r9, r11, 0x6bb8
	ctx.r[9].s64 = ctx.r[11].s64 + 27576;
	// 827FAFCC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FAFD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FAFD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FAFD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FAFDC: 48000008  b 0x827fafe4
	pc = 0x827FAFE4; continue 'dispatch;
	// 827FAFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FAFE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FAFE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FAFEC: 409A0044  bne cr6, 0x827fb030
	if !ctx.cr[6].eq {
	pc = 0x827FB030; continue 'dispatch;
	}
	// 827FAFF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FAFF4: 419A001C  beq cr6, 0x827fb010
	if ctx.cr[6].eq {
	pc = 0x827FB010; continue 'dispatch;
	}
	// 827FAFF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FAFFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FB000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FB004: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB008: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FB00C: 4E800421  bctrl
	ctx.lr = 0x827FB010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FB010: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FB014: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FB018: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB01C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FB020: 816BD348  lwz r11, -0x2cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11448 as u32) ) } as u64;
	// 827FB024: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FB028: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FB02C: 4BAC4FD5  bl 0x822c0000
	ctx.lr = 0x827FB030;
	sub_822C0000(ctx, base);
	// 827FB030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FB038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FB03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FB040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FB044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FB048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB050 size=72
    let mut pc: u32 = 0x827FB050;
    'dispatch: loop {
        match pc {
            0x827FB050 => {
    //   block [0x827FB050..0x827FB098)
	// 827FB050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FB058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB05C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 827FB060: 419A001C  beq cr6, 0x827fb07c
	if ctx.cr[6].eq {
	pc = 0x827FB07C; continue 'dispatch;
	}
	// 827FB064: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827FB068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827FB06C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 827FB070: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827FB074: 4BFFFBF5  bl 0x827fac68
	ctx.lr = 0x827FB078;
	sub_827FAC68(ctx, base);
	// 827FB078: 48000010  b 0x827fb088
	pc = 0x827FB088; continue 'dispatch;
	// 827FB07C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FB080: 396BD398  addi r11, r11, -0x2c68
	ctx.r[11].s64 = ctx.r[11].s64 + -11368;
	// 827FB084: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FB088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FB08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FB090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FB094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB098 size=132
    let mut pc: u32 = 0x827FB098;
    'dispatch: loop {
        match pc {
            0x827FB098 => {
    //   block [0x827FB098..0x827FB11C)
	// 827FB098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FB0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FB0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FB0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB0B0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FB0B4: 4BFFEB65  bl 0x827f9c18
	ctx.lr = 0x827FB0B8;
	sub_827F9C18(ctx, base);
	// 827FB0B8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB0BC: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FB0C0: 392B6BE4  addi r9, r11, 0x6be4
	ctx.r[9].s64 = ctx.r[11].s64 + 27620;
	// 827FB0C4: 394A6BCC  addi r10, r10, 0x6bcc
	ctx.r[10].s64 = ctx.r[10].s64 + 27596;
	// 827FB0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FB0CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FB0D0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FB0D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FB0D8: 917F0120  stw r11, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 827FB0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FB0E0: 917F0124  stw r11, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u32 ) };
	// 827FB0E4: 917F0128  stw r11, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 827FB0E8: 917F012C  stw r11, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 827FB0EC: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 827FB0F0: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 827FB0F4: 997F0138  stb r11, 0x138(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[11].u8 ) };
	// 827FB0F8: 93DF013C  stw r30, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[30].u32 ) };
	// 827FB0FC: 915F0140  stw r10, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 827FB100: 915F0144  stw r10, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 827FB104: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FB108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FB10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FB110: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FB114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FB118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB120 size=76
    let mut pc: u32 = 0x827FB120;
    'dispatch: loop {
        match pc {
            0x827FB120 => {
    //   block [0x827FB120..0x827FB16C)
	// 827FB120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FB128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FB12C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FB130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB138: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FB13C: 4BFFFD05  bl 0x827fae40
	ctx.lr = 0x827FB140;
	sub_827FAE40(ctx, base);
	// 827FB140: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FB144: 4182000C  beq 0x827fb150
	if ctx.cr[0].eq {
	pc = 0x827FB150; continue 'dispatch;
	}
	// 827FB148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FB14C: 485F728D  bl 0x82df23d8
	ctx.lr = 0x827FB150;
	sub_82DF23D8(ctx, base);
	// 827FB150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FB154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FB158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FB15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FB160: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FB164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FB168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB170 size=216
    let mut pc: u32 = 0x827FB170;
    'dispatch: loop {
        match pc {
            0x827FB170 => {
    //   block [0x827FB170..0x827FB248)
	// 827FB170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB174: 489ACFF9  bl 0x831a816c
	ctx.lr = 0x827FB178;
	sub_831A8130(ctx, base);
	// 827FB178: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB17C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FB180: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FB184: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827FB188: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB18C: 4BAEC07D  bl 0x822e7208
	ctx.lr = 0x827FB190;
	sub_822E7208(ctx, base);
	// 827FB190: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB194: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB198: 388B6C38  addi r4, r11, 0x6c38
	ctx.r[4].s64 = ctx.r[11].s64 + 27704;
	// 827FB19C: 485F886D  bl 0x82df3a08
	ctx.lr = 0x827FB1A0;
	sub_82DF3A08(ctx, base);
	// 827FB1A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FB1A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FB1A8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827FB1AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FB1B0: 4BAEC1E1  bl 0x822e7390
	ctx.lr = 0x827FB1B4;
	sub_822E7390(ctx, base);
	// 827FB1B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB1B8: 485F8271  bl 0x82df3428
	ctx.lr = 0x827FB1BC;
	sub_82DF3428(ctx, base);
	// 827FB1BC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FB1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FB1C4: 419A0054  beq cr6, 0x827fb218
	if ctx.cr[6].eq {
	pc = 0x827FB218; continue 'dispatch;
	}
	// 827FB1C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FB1CC: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 827FB1D0: 409A0008  bne cr6, 0x827fb1d8
	if !ctx.cr[6].eq {
	pc = 0x827FB1D8; continue 'dispatch;
	}
	// 827FB1D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827FB1D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB1DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FB1E0: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827FB1E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FB1E8: 4E800421  bctrl
	ctx.lr = 0x827FB1EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FB1EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FB1F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB1F4: 485F8815  bl 0x82df3a08
	ctx.lr = 0x827FB1F8;
	sub_82DF3A08(ctx, base);
	// 827FB1F8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827FB1FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FB200: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FB204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB208: 4BAF34D1  bl 0x822ee6d8
	ctx.lr = 0x827FB20C;
	sub_822EE6D8(ctx, base);
	// 827FB20C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB210: 485F8219  bl 0x82df3428
	ctx.lr = 0x827FB214;
	sub_82DF3428(ctx, base);
	// 827FB214: 48000010  b 0x827fb224
	pc = 0x827FB224; continue 'dispatch;
	// 827FB218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FB21C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FB220: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827FB224: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FB228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB22C: 419A0008  beq cr6, 0x827fb234
	if ctx.cr[6].eq {
	pc = 0x827FB234; continue 'dispatch;
	}
	// 827FB230: 4BAC5661  bl 0x822c0890
	ctx.lr = 0x827FB234;
	sub_822C0890(ctx, base);
	// 827FB234: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB238: 4BAEBFE9  bl 0x822e7220
	ctx.lr = 0x827FB23C;
	sub_822E7220(ctx, base);
	// 827FB23C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB240: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FB244: 489ACF78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB248 size=180
    let mut pc: u32 = 0x827FB248;
    'dispatch: loop {
        match pc {
            0x827FB248 => {
    //   block [0x827FB248..0x827FB2FC)
	// 827FB248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB24C: 489ACF1D  bl 0x831a8168
	ctx.lr = 0x827FB250;
	sub_831A8130(ctx, base);
	// 827FB250: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB254: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB258: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FB25C: 3BAB6C4C  addi r29, r11, 0x6c4c
	ctx.r[29].s64 = ctx.r[11].s64 + 27724;
	// 827FB260: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FB264: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB268: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FB26C: 485F879D  bl 0x82df3a08
	ctx.lr = 0x827FB270;
	sub_82DF3A08(ctx, base);
	// 827FB270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB274: 4BFFE0DD  bl 0x827f9350
	ctx.lr = 0x827FB278;
	sub_827F9350(ctx, base);
	// 827FB278: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FB27C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB280: 48619D49  bl 0x82e14fc8
	ctx.lr = 0x827FB284;
	sub_82E14FC8(ctx, base);
	// 827FB284: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827FB288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB28C: 485F819D  bl 0x82df3428
	ctx.lr = 0x827FB290;
	sub_82DF3428(ctx, base);
	// 827FB290: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FB294: 41820060  beq 0x827fb2f4
	if ctx.cr[0].eq {
	pc = 0x827FB2F4; continue 'dispatch;
	}
	// 827FB298: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FB29C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB2A0: 485F8769  bl 0x82df3a08
	ctx.lr = 0x827FB2A4;
	sub_82DF3A08(ctx, base);
	// 827FB2A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB2A8: 4BFFE0A9  bl 0x827f9350
	ctx.lr = 0x827FB2AC;
	sub_827F9350(ctx, base);
	// 827FB2AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FB2B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FB2B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FB2B8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB2BC: 48619D75  bl 0x82e15030
	ctx.lr = 0x827FB2C0;
	sub_82E15030(ctx, base);
	// 827FB2C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FB2C4: 395F0018  addi r10, r31, 0x18
	ctx.r[10].s64 = ctx.r[31].s64 + 24;
	// 827FB2C8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827FB2CC: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827FB2D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB2D4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 827FB2D8: 4BAC9189  bl 0x822c4460
	ctx.lr = 0x827FB2DC;
	sub_822C4460(ctx, base);
	// 827FB2DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FB2E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB2E4: 419A0008  beq cr6, 0x827fb2ec
	if ctx.cr[6].eq {
	pc = 0x827FB2EC; continue 'dispatch;
	}
	// 827FB2E8: 4BAC55A9  bl 0x822c0890
	ctx.lr = 0x827FB2EC;
	sub_822C0890(ctx, base);
	// 827FB2EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB2F0: 485F8139  bl 0x82df3428
	ctx.lr = 0x827FB2F4;
	sub_82DF3428(ctx, base);
	// 827FB2F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FB2F8: 489ACEC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB300 size=76
    let mut pc: u32 = 0x827FB300;
    'dispatch: loop {
        match pc {
            0x827FB300 => {
    //   block [0x827FB300..0x827FB34C)
	// 827FB300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FB308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB30C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB310: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FB314: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FB318: 38AA6C54  addi r5, r10, 0x6c54
	ctx.r[5].s64 = ctx.r[10].s64 + 27732;
	// 827FB31C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB320: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 827FB324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FB328: 4E800421  bctrl
	ctx.lr = 0x827FB32C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FB32C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FB330: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB334: 419A0008  beq cr6, 0x827fb33c
	if ctx.cr[6].eq {
	pc = 0x827FB33C; continue 'dispatch;
	}
	// 827FB338: 4BAC5559  bl 0x822c0890
	ctx.lr = 0x827FB33C;
	sub_822C0890(ctx, base);
	// 827FB33C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FB340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FB344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FB348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB350 size=112
    let mut pc: u32 = 0x827FB350;
    'dispatch: loop {
        match pc {
            0x827FB350 => {
    //   block [0x827FB350..0x827FB3C0)
	// 827FB350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FB358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FB35C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FB360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB364: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FB368: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB36C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827FB370: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FB374: 4BFFFA05  bl 0x827fad78
	ctx.lr = 0x827FB378;
	sub_827FAD78(ctx, base);
	// 827FB378: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827FB37C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FB380: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FB384: 4BAC4C7D  bl 0x822c0000
	ctx.lr = 0x827FB388;
	sub_822C0000(ctx, base);
	// 827FB388: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827FB38C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FB390: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FB394: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827FB398: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB39C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FB3A0: 419A0008  beq cr6, 0x827fb3a8
	if ctx.cr[6].eq {
	pc = 0x827FB3A8; continue 'dispatch;
	}
	// 827FB3A4: 4BAC54ED  bl 0x822c0890
	ctx.lr = 0x827FB3A8;
	sub_822C0890(ctx, base);
	// 827FB3A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FB3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FB3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FB3B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FB3B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FB3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB3C0 size=128
    let mut pc: u32 = 0x827FB3C0;
    'dispatch: loop {
        match pc {
            0x827FB3C0 => {
    //   block [0x827FB3C0..0x827FB440)
	// 827FB3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB3C4: 489ACDA9  bl 0x831a816c
	ctx.lr = 0x827FB3C8;
	sub_831A8130(ctx, base);
	// 827FB3C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB3CC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB3D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FB3D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FB3D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FB3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FB3E0: 388B6C70  addi r4, r11, 0x6c70
	ctx.r[4].s64 = ctx.r[11].s64 + 27760;
	// 827FB3E4: 38A0004F  li r5, 0x4f
	ctx.r[5].s64 = 79;
	// 827FB3E8: 38600148  li r3, 0x148
	ctx.r[3].s64 = 328;
	// 827FB3EC: 485F6FFD  bl 0x82df23e8
	ctx.lr = 0x827FB3F0;
	sub_82DF23E8(ctx, base);
	// 827FB3F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FB3F4: 41820018  beq 0x827fb40c
	if ctx.cr[0].eq {
	pc = 0x827FB40C; continue 'dispatch;
	}
	// 827FB3F8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827FB3FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FB400: 4BFFFC99  bl 0x827fb098
	ctx.lr = 0x827FB404;
	sub_827FB098(ctx, base);
	// 827FB404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB408: 48000008  b 0x827fb410
	pc = 0x827FB410; continue 'dispatch;
	// 827FB40C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FB410: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FB414: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FB418: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FB41C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB420: 4BFFFAA1  bl 0x827faec0
	ctx.lr = 0x827FB424;
	sub_827FAEC0(ctx, base);
	// 827FB424: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FB428: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FB42C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB430: 4BAC4BD1  bl 0x822c0000
	ctx.lr = 0x827FB434;
	sub_822C0000(ctx, base);
	// 827FB434: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FB438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FB43C: 489ACD80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FB440 size=288
    let mut pc: u32 = 0x827FB440;
    'dispatch: loop {
        match pc {
            0x827FB440 => {
    //   block [0x827FB440..0x827FB560)
	// 827FB440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB444: 489ACD29  bl 0x831a816c
	ctx.lr = 0x827FB448;
	sub_831A8130(ctx, base);
	// 827FB448: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827FB44C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB454: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FB458: 897F0138  lbz r11, 0x138(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 827FB45C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FB460: 418200E8  beq 0x827fb548
	if ctx.cr[0].eq {
	pc = 0x827FB548; continue 'dispatch;
	}
	// 827FB464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB468: 4880CCC1  bl 0x83008128
	ctx.lr = 0x827FB46C;
	sub_83008128(ctx, base);
	// 827FB46C: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 827FB470: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827FB474: 419A00D4  beq cr6, 0x827fb548
	if ctx.cr[6].eq {
	pc = 0x827FB548; continue 'dispatch;
	}
	// 827FB478: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FB47C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 827FB480: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 827FB484: 3D008336  lis r8, -0x7cca
	ctx.r[8].s64 = -2093613056;
	// 827FB488: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 827FB48C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 827FB490: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827FB494: 388885F0  addi r4, r8, -0x7a10
	ctx.r[4].s64 = ctx.r[8].s64 + -31248;
	// 827FB498: C00AD7BC  lfs f0, -0x2844(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FB49C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 827FB4A0: D3E10070  stfs f31, 0x70(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 827FB4A4: 38FE0020  addi r7, r30, 0x20
	ctx.r[7].s64 = ctx.r[30].s64 + 32;
	// 827FB4A8: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 827FB4AC: 38DF0144  addi r6, r31, 0x144
	ctx.r[6].s64 = ctx.r[31].s64 + 324;
	// 827FB4B0: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 827FB4B4: 38BF0140  addi r5, r31, 0x140
	ctx.r[5].s64 = ctx.r[31].s64 + 320;
	// 827FB4B8: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 827FB4BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB4C0: 480A73D1  bl 0x828a2890
	ctx.lr = 0x827FB4C4;
	sub_828A2890(ctx, base);
	// 827FB4C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FB4C8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827FB4CC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827FB4D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FB4D4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827FB4D8: 419A0024  beq cr6, 0x827fb4fc
	if ctx.cr[6].eq {
	pc = 0x827FB4FC; continue 'dispatch;
	}
	// 827FB4DC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827FB4E0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827FB4E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FB4E8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827FB4EC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827FB4F0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827FB4F4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FB4F8: 4082FFE8  bne 0x827fb4e0
	if !ctx.cr[0].eq {
	pc = 0x827FB4E0; continue 'dispatch;
	}
	// 827FB4FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB500: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 827FB504: 4880CC25  bl 0x83008128
	ctx.lr = 0x827FB508;
	sub_83008128(ctx, base);
	// 827FB508: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB50C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827FB510: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827FB514: 388B6C70  addi r4, r11, 0x6c70
	ctx.r[4].s64 = ctx.r[11].s64 + 27760;
	// 827FB518: 38A000EF  li r5, 0xef
	ctx.r[5].s64 = 239;
	// 827FB51C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827FB520: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 827FB524: 4865D51D  bl 0x82e58a40
	ctx.lr = 0x827FB528;
	sub_82E58A40(ctx, base);
	// 827FB528: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FB52C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB530: 419A0008  beq cr6, 0x827fb538
	if ctx.cr[6].eq {
	pc = 0x827FB538; continue 'dispatch;
	}
	// 827FB534: 4BAC535D  bl 0x822c0890
	ctx.lr = 0x827FB538;
	sub_822C0890(ctx, base);
	// 827FB538: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827FB53C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB540: 419A0008  beq cr6, 0x827fb548
	if ctx.cr[6].eq {
	pc = 0x827FB548; continue 'dispatch;
	}
	// 827FB544: 4BAC534D  bl 0x822c0890
	ctx.lr = 0x827FB548;
	sub_822C0890(ctx, base);
	// 827FB548: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FB54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FB550: 4BFFE4D9  bl 0x827f9a28
	ctx.lr = 0x827FB554;
	sub_827F9A28(ctx, base);
	// 827FB554: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827FB558: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827FB55C: 489ACC60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FB560 size=224
    let mut pc: u32 = 0x827FB560;
    'dispatch: loop {
        match pc {
            0x827FB560 => {
    //   block [0x827FB560..0x827FB640)
	// 827FB560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB564: 489ACC09  bl 0x831a816c
	ctx.lr = 0x827FB568;
	sub_831A8130(ctx, base);
	// 827FB568: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB56C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FB570: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827FB574: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB578: 4862F571  bl 0x82e2aae8
	ctx.lr = 0x827FB57C;
	sub_82E2AAE8(ctx, base);
	// 827FB57C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB584: 388B6C38  addi r4, r11, 0x6c38
	ctx.r[4].s64 = ctx.r[11].s64 + 27704;
	// 827FB588: 485F8481  bl 0x82df3a08
	ctx.lr = 0x827FB58C;
	sub_82DF3A08(ctx, base);
	// 827FB58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FB590: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FB594: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827FB598: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FB59C: 486338D5  bl 0x82e2ee70
	ctx.lr = 0x827FB5A0;
	sub_82E2EE70(ctx, base);
	// 827FB5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FB5A4: 485F7E85  bl 0x82df3428
	ctx.lr = 0x827FB5A8;
	sub_82DF3428(ctx, base);
	// 827FB5A8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FB5AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FB5B0: 419A0060  beq cr6, 0x827fb610
	if ctx.cr[6].eq {
	pc = 0x827FB610; continue 'dispatch;
	}
	// 827FB5B4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB5B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FB5BC: 388B6C70  addi r4, r11, 0x6c70
	ctx.r[4].s64 = ctx.r[11].s64 + 27760;
	// 827FB5C0: 38A0013D  li r5, 0x13d
	ctx.r[5].s64 = 317;
	// 827FB5C4: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 827FB5C8: 485F6E21  bl 0x82df23e8
	ctx.lr = 0x827FB5CC;
	sub_82DF23E8(ctx, base);
	// 827FB5CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FB5D0: 41820014  beq 0x827fb5e4
	if ctx.cr[0].eq {
	pc = 0x827FB5E4; continue 'dispatch;
	}
	// 827FB5D4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827FB5D8: 4861B2B9  bl 0x82e16890
	ctx.lr = 0x827FB5DC;
	sub_82E16890(ctx, base);
	// 827FB5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB5E0: 48000008  b 0x827fb5e8
	pc = 0x827FB5E8; continue 'dispatch;
	// 827FB5E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FB5E8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FB5EC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FB5F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FB5F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB5F8: 4BB5DAC1  bl 0x823590b8
	ctx.lr = 0x827FB5FC;
	sub_823590B8(ctx, base);
	// 827FB5FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FB600: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FB604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FB608: 4BAC49F9  bl 0x822c0000
	ctx.lr = 0x827FB60C;
	sub_822C0000(ctx, base);
	// 827FB60C: 48000010  b 0x827fb61c
	pc = 0x827FB61C; continue 'dispatch;
	// 827FB610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FB614: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FB618: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827FB61C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FB620: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FB624: 419A0008  beq cr6, 0x827fb62c
	if ctx.cr[6].eq {
	pc = 0x827FB62C; continue 'dispatch;
	}
	// 827FB628: 4BAC5269  bl 0x822c0890
	ctx.lr = 0x827FB62C;
	sub_822C0890(ctx, base);
	// 827FB62C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB630: 4862F4D1  bl 0x82e2ab00
	ctx.lr = 0x827FB634;
	sub_82E2AB00(ctx, base);
	// 827FB634: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FB638: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FB63C: 489ACB80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FB640 size=712
    let mut pc: u32 = 0x827FB640;
    'dispatch: loop {
        match pc {
            0x827FB640 => {
    //   block [0x827FB640..0x827FB908)
	// 827FB640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB644: 489ACB25  bl 0x831a8168
	ctx.lr = 0x827FB648;
	sub_831A8130(ctx, base);
	// 827FB648: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 827FB64C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827FB650: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB654: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FB658: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FB65C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FB660: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FB664: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827FB668: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827FB66C: C3CA08A8  lfs f30, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827FB670: D3E10080  stfs f31, 0x80(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 827FB674: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 827FB678: D3C10088  stfs f30, 0x88(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 827FB67C: D3E1008C  stfs f31, 0x8c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 827FB680: 4BD164D9  bl 0x82511b58
	ctx.lr = 0x827FB684;
	sub_82511B58(ctx, base);
	// 827FB684: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FB688: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827FB68C: 4868080D  bl 0x82e7be98
	ctx.lr = 0x827FB690;
	sub_82E7BE98(ctx, base);
	// 827FB690: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FB694: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 827FB698: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB69C: 4868062D  bl 0x82e7bcc8
	ctx.lr = 0x827FB6A0;
	sub_82E7BCC8(ctx, base);
	// 827FB6A0: D3E10064  stfs f31, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827FB6A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FB6A8: 48681DE9  bl 0x82e7d490
	ctx.lr = 0x827FB6AC;
	sub_82E7D490(ctx, base);
	// 827FB6AC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FB6B0: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827FB6B4: C02B6CB8  lfs f1, 0x6cb8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27832 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FB6B8: 48681331  bl 0x82e7c9e8
	ctx.lr = 0x827FB6BC;
	sub_82E7C9E8(ctx, base);
	// 827FB6BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FB6C0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827FB6C4: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 827FB6C8: 48680601  bl 0x82e7bcc8
	ctx.lr = 0x827FB6CC;
	sub_82E7BCC8(ctx, base);
	// 827FB6CC: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 827FB6D0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 827FB6D4: 807F0128  lwz r3, 0x128(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 827FB6D8: D3E100A0  stfs f31, 0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827FB6DC: 3B8100A0  addi r28, r1, 0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + 160;
	// 827FB6E0: D3E100A4  stfs f31, 0xa4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827FB6E4: D3C100A8  stfs f30, 0xa8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827FB6E8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FB6EC: D3C100AC  stfs f30, 0xac(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FB908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FB908 size=372
    let mut pc: u32 = 0x827FB908;
    'dispatch: loop {
        match pc {
            0x827FB908 => {
    //   block [0x827FB908..0x827FBA7C)
	// 827FB908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FB90C: 489AC861  bl 0x831a816c
	ctx.lr = 0x827FB910;
	sub_831A8130(ctx, base);
	// 827FB910: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FB914: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 827FB918: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 827FB91C: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 827FB920: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 827FB924: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 827FB928: C00BBA80  lfs f0, -0x4580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FB92C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FB930: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 827FB934: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 827FB938: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FB93C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 827FB940: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FB944: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FB948: C1A90008  lfs f13, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FB94C: C189000C  lfs f12, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827FB950: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 827FB954: D1A10088  stfs f13, 0x88(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 827FB958: D181008C  stfs f12, 0x8c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FBA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FBA80 size=240
    let mut pc: u32 = 0x827FBA80;
    'dispatch: loop {
        match pc {
            0x827FBA80 => {
    //   block [0x827FBA80..0x827FBB70)
	// 827FBA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FBA84: 489AC6E9  bl 0x831a816c
	ctx.lr = 0x827FBA88;
	sub_831A8130(ctx, base);
	// 827FBA88: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FBA8C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 827FBA90: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 827FBA94: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 827FBA98: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 827FBA9C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 827FBAA0: C00BBA80  lfs f0, -0x4580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FBAA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FBAA8: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827FBAAC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827FBAB0: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FBAB4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827FBAB8: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FBABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FBAC0: C1A90008  lfs f13, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FBAC4: C189000C  lfs f12, 0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827FBAC8: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827FBACC: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827FBAD0: D181006C  stfs f12, 0x6c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FBB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FBB70 size=444
    let mut pc: u32 = 0x827FBB70;
    'dispatch: loop {
        match pc {
            0x827FBB70 => {
    //   block [0x827FBB70..0x827FBD2C)
	// 827FBB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FBB74: 489AC5F5  bl 0x831a8168
	ctx.lr = 0x827FBB78;
	sub_831A8130(ctx, base);
	// 827FBB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FBB7C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827FBB80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FBB84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FBB88: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827FBB8C: 41820038  beq 0x827fbbc4
	if ctx.cr[0].eq {
	pc = 0x827FBBC4; continue 'dispatch;
	}
	// 827FBB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBB94: 489ADDF5  bl 0x831a9988
	ctx.lr = 0x827FBB98;
	sub_831A9988(ctx, base);
	// 827FBB98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FBB9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBBA0: 386BD5B0  addi r3, r11, -0x2a50
	ctx.r[3].s64 = ctx.r[11].s64 + -10832;
	// 827FBBA4: 489AC555  bl 0x831a80f8
	ctx.lr = 0x827FBBA8;
	sub_831A80F8(ctx, base);
	// 827FBBA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBBAC: 41820018  beq 0x827fbbc4
	if ctx.cr[0].eq {
	pc = 0x827FBBC4; continue 'dispatch;
	}
	// 827FBBB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBBB4: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBBB8: 4BE65B41  bl 0x826616f8
	ctx.lr = 0x827FBBBC;
	sub_826616F8(ctx, base);
	// 827FBBBC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827FBBC0: 48000164  b 0x827fbd24
	pc = 0x827FBD24; continue 'dispatch;
	// 827FBBC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827FBBC8: 419A014C  beq cr6, 0x827fbd14
	if ctx.cr[6].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBBD0: 489ADDB9  bl 0x831a9988
	ctx.lr = 0x827FBBD4;
	sub_831A9988(ctx, base);
	// 827FBBD4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 827FBBD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBBDC: 386B3644  addi r3, r11, 0x3644
	ctx.r[3].s64 = ctx.r[11].s64 + 13892;
	// 827FBBE0: 489AC519  bl 0x831a80f8
	ctx.lr = 0x827FBBE4;
	sub_831A80F8(ctx, base);
	// 827FBBE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBBE8: 41820014  beq 0x827fbbfc
	if ctx.cr[0].eq {
	pc = 0x827FBBFC; continue 'dispatch;
	}
	// 827FBBEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBBF0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBBF4: 4BFEF3CD  bl 0x827eafc0
	ctx.lr = 0x827FBBF8;
	sub_827EAFC0(ctx, base);
	// 827FBBF8: 4BFFFFC4  b 0x827fbbbc
	pc = 0x827FBBBC; continue 'dispatch;
	// 827FBBFC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827FBC00: 419A0114  beq cr6, 0x827fbd14
	if ctx.cr[6].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBC08: 489ADD81  bl 0x831a9988
	ctx.lr = 0x827FBC0C;
	sub_831A9988(ctx, base);
	// 827FBC0C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FBC10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBC14: 386BD578  addi r3, r11, -0x2a88
	ctx.r[3].s64 = ctx.r[11].s64 + -10888;
	// 827FBC18: 489AC4E1  bl 0x831a80f8
	ctx.lr = 0x827FBC1C;
	sub_831A80F8(ctx, base);
	// 827FBC1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBC20: 41820014  beq 0x827fbc34
	if ctx.cr[0].eq {
	pc = 0x827FBC34; continue 'dispatch;
	}
	// 827FBC24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBC28: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBC2C: 4BFFFCDD  bl 0x827fb908
	ctx.lr = 0x827FBC30;
	sub_827FB908(ctx, base);
	// 827FBC30: 4BFFFF8C  b 0x827fbbbc
	pc = 0x827FBBBC; continue 'dispatch;
	// 827FBC34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827FBC38: 419A00DC  beq cr6, 0x827fbd14
	if ctx.cr[6].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBC3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBC40: 489ADD49  bl 0x831a9988
	ctx.lr = 0x827FBC44;
	sub_831A9988(ctx, base);
	// 827FBC44: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FBC48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBC4C: 386BD544  addi r3, r11, -0x2abc
	ctx.r[3].s64 = ctx.r[11].s64 + -10940;
	// 827FBC50: 489AC4A9  bl 0x831a80f8
	ctx.lr = 0x827FBC54;
	sub_831A80F8(ctx, base);
	// 827FBC54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBC58: 41820014  beq 0x827fbc6c
	if ctx.cr[0].eq {
	pc = 0x827FBC6C; continue 'dispatch;
	}
	// 827FBC5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBC60: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBC64: 4BFFFE1D  bl 0x827fba80
	ctx.lr = 0x827FBC68;
	sub_827FBA80(ctx, base);
	// 827FBC68: 4BFFFF54  b 0x827fbbbc
	pc = 0x827FBBBC; continue 'dispatch;
	// 827FBC6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827FBC70: 419A00A4  beq cr6, 0x827fbd14
	if ctx.cr[6].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBC78: 489ADD11  bl 0x831a9988
	ctx.lr = 0x827FBC7C;
	sub_831A9988(ctx, base);
	// 827FBC7C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FBC80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBC84: 386BD510  addi r3, r11, -0x2af0
	ctx.r[3].s64 = ctx.r[11].s64 + -10992;
	// 827FBC88: 489AC471  bl 0x831a80f8
	ctx.lr = 0x827FBC8C;
	sub_831A80F8(ctx, base);
	// 827FBC8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBC90: 41820014  beq 0x827fbca4
	if ctx.cr[0].eq {
	pc = 0x827FBCA4; continue 'dispatch;
	}
	// 827FBC94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBC98: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBC9C: 4BFFEF95  bl 0x827fac30
	ctx.lr = 0x827FBCA0;
	sub_827FAC30(ctx, base);
	// 827FBCA0: 4BFFFF1C  b 0x827fbbbc
	pc = 0x827FBBBC; continue 'dispatch;
	// 827FBCA4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827FBCA8: 419A006C  beq cr6, 0x827fbd14
	if ctx.cr[6].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBCB0: 489ADCD9  bl 0x831a9988
	ctx.lr = 0x827FBCB4;
	sub_831A9988(ctx, base);
	// 827FBCB4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FBCB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBCBC: 386BD4DC  addi r3, r11, -0x2b24
	ctx.r[3].s64 = ctx.r[11].s64 + -11044;
	// 827FBCC0: 489AC439  bl 0x831a80f8
	ctx.lr = 0x827FBCC4;
	sub_831A80F8(ctx, base);
	// 827FBCC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBCC8: 41820014  beq 0x827fbcdc
	if ctx.cr[0].eq {
	pc = 0x827FBCDC; continue 'dispatch;
	}
	// 827FBCCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBCD0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBCD4: 4BFFEF75  bl 0x827fac48
	ctx.lr = 0x827FBCD8;
	sub_827FAC48(ctx, base);
	// 827FBCD8: 4BFFFEE4  b 0x827fbbbc
	pc = 0x827FBBBC; continue 'dispatch;
	// 827FBCDC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827FBCE0: 419A0034  beq cr6, 0x827fbd14
	if ctx.cr[6].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBCE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBCE8: 489ADCA1  bl 0x831a9988
	ctx.lr = 0x827FBCEC;
	sub_831A9988(ctx, base);
	// 827FBCEC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 827FBCF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBCF4: 386B6B84  addi r3, r11, 0x6b84
	ctx.r[3].s64 = ctx.r[11].s64 + 27524;
	// 827FBCF8: 489AC401  bl 0x831a80f8
	ctx.lr = 0x827FBCFC;
	sub_831A80F8(ctx, base);
	// 827FBCFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBD00: 41820014  beq 0x827fbd14
	if ctx.cr[0].eq {
	pc = 0x827FBD14; continue 'dispatch;
	}
	// 827FBD04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBD08: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FBD0C: 4BFFF53D  bl 0x827fb248
	ctx.lr = 0x827FBD10;
	sub_827FB248(ctx, base);
	// 827FBD10: 4BFFFEAC  b 0x827fbbbc
	pc = 0x827FBBBC; continue 'dispatch;
	// 827FBD14: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827FBD18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FBD1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FBD20: 4BFFDD21  bl 0x827f9a40
	ctx.lr = 0x827FBD24;
	sub_827F9A40(ctx, base);
	// 827FBD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FBD28: 489AC490  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FBD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FBD30 size=128
    let mut pc: u32 = 0x827FBD30;
    'dispatch: loop {
        match pc {
            0x827FBD30 => {
    //   block [0x827FBD30..0x827FBDB0)
	// 827FBD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FBD34: 489AC439  bl 0x831a816c
	ctx.lr = 0x827FBD38;
	sub_831A8130(ctx, base);
	// 827FBD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FBD3C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827FBD40: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827FBD44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FBD48: 3BEBAD64  addi r31, r11, -0x529c
	ctx.r[31].s64 = ctx.r[11].s64 + -21148;
	// 827FBD4C: 816AAD6C  lwz r11, -0x5294(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21140 as u32) ) } as u64;
	// 827FBD50: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827FBD54: 40820024  bne 0x827fbd78
	if !ctx.cr[0].eq {
	pc = 0x827FBD78; continue 'dispatch;
	}
	// 827FBD58: 3D208271  lis r9, -0x7d8f
	ctx.r[9].s64 = -2106523648;
	// 827FBD5C: 3D008280  lis r8, -0x7d80
	ctx.r[8].s64 = -2105540608;
	// 827FBD60: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827FBD64: 39298E48  addi r9, r9, -0x71b8
	ctx.r[9].s64 = ctx.r[9].s64 + -29112;
	// 827FBD68: 3908B050  addi r8, r8, -0x4fb0
	ctx.r[8].s64 = ctx.r[8].s64 + -20400;
	// 827FBD6C: 916AAD6C  stw r11, -0x5294(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21140 as u32), ctx.r[11].u32 ) };
	// 827FBD70: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 827FBD74: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 827FBD78: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827FBD7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827FBD80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBD84: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 827FBD88: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 827FBD8C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827FBD90: 4BE58831  bl 0x826545c0
	ctx.lr = 0x827FBD94;
	sub_826545C0(ctx, base);
	// 827FBD94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FBD98: 4182000C  beq 0x827fbda4
	if ctx.cr[0].eq {
	pc = 0x827FBDA4; continue 'dispatch;
	}
	// 827FBD9C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FBDA0: 48000008  b 0x827fbda8
	pc = 0x827FBDA8; continue 'dispatch;
	// 827FBDA4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 827FBDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FBDAC: 489AC410  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FBDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FBDB0 size=4384
    let mut pc: u32 = 0x827FBDB0;
    'dispatch: loop {
        match pc {
            0x827FBDB0 => {
    //   block [0x827FBDB0..0x827FCED0)
	// 827FBDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FBDB4: 489AC37D  bl 0x831a8130
	ctx.lr = 0x827FBDB8;
	sub_831A8130(ctx, base);
	// 827FBDB8: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 827FBDBC: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 827FBDC0: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 827FBDC4: 9421F6C0  stwu r1, -0x940(r1)
	ea = ctx.r[1].u32.wrapping_add(-2368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FBDC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FBDCC: 90A10964  stw r5, 0x964(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2404 as u32), ctx.r[5].u32 ) };
	// 827FBDD0: 93C10954  stw r30, 0x954(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2388 as u32), ctx.r[30].u32 ) };
	// 827FBDD4: 4BFFD75D  bl 0x827f9530
	ctx.lr = 0x827FBDD8;
	sub_827F9530(ctx, base);
	// 827FBDD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FBDDC: 4BFFD575  bl 0x827f9350
	ctx.lr = 0x827FBDE0;
	sub_827F9350(ctx, base);
	// 827FBDE0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FBDE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FBDE8: 419A000C  beq cr6, 0x827fbdf4
	if ctx.cr[6].eq {
	pc = 0x827FBDF4; continue 'dispatch;
	}
	// 827FBDEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FBDF0: 48619231  bl 0x82e15020
	ctx.lr = 0x827FBDF4;
	sub_82E15020(ctx, base);
	// 827FBDF4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FBDF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FBDFC: 388B6C70  addi r4, r11, 0x6c70
	ctx.r[4].s64 = ctx.r[11].s64 + 27760;
	// 827FBE00: 38A00077  li r5, 0x77
	ctx.r[5].s64 = 119;
	// 827FBE04: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 827FBE08: 90810080  stw r4, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u32 ) };
	// 827FBE0C: 4BAC45CD  bl 0x822c03d8
	ctx.lr = 0x827FBE10;
	sub_822C03D8(ctx, base);
	// 827FBE10: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FBE14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FBE18: 4182000C  beq 0x827fbe24
	if ctx.cr[0].eq {
	pc = 0x827FBE24; continue 'dispatch;
	}
	// 827FBE1C: 4BFEEE95  bl 0x827eacb0
	ctx.lr = 0x827FBE20;
	sub_827EACB0(ctx, base);
	// 827FBE20: 48000008  b 0x827fbe28
	pc = 0x827FBE28; continue 'dispatch;
	// 827FBE24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FBE28: 397E0120  addi r11, r30, 0x120
	ctx.r[11].s64 = ctx.r[30].s64 + 288;
	// 827FBE2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBE30: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827FBE34: 4BFFF51D  bl 0x827fb350
	ctx.lr = 0x827FBE38;
	sub_827FB350(ctx, base);
	// 827FBE38: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FBE3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FBE40: 388B6C38  addi r4, r11, 0x6c38
	ctx.r[4].s64 = ctx.r[11].s64 + 27704;
	// 827FBE44: 485F7BC5  bl 0x82df3a08
	ctx.lr = 0x827FBE48;
	sub_82DF3A08(ctx, base);
	// 827FBE48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FBE4C: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827FBE50: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 827FBE54: 83DE0120  lwz r30, 0x120(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FBE58: 4BD13671  bl 0x8250f4c8
	ctx.lr = 0x827FBE5C;
	sub_8250F4C8(ctx, base);
	// 827FBE5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FBE60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FBE64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FBE68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827FBE6C: C3AB08A8  lfs f29, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827FBE70: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FBE74: 4BFEEAF5  bl 0x827ea968
	ctx.lr = 0x827FBE78;
	sub_827EA968(ctx, base);
	// 827FBE78: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827FBE7C: 485F5E15  bl 0x82df1c90
	ctx.lr = 0x827FBE80;
	sub_82DF1C90(ctx, base);
	// 827FBE80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FBE84: 485F75A5  bl 0x82df3428
	ctx.lr = 0x827FBE88;
	sub_82DF3428(ctx, base);
	// 827FBE88: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FBE8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FBE90: D3A100C8  stfs f29, 0xc8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 827FBE94: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827FBE98: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 827FBE9C: 3D008207  lis r8, -0x7df9
	ctx.r[8].s64 = -2113470464;
	// 827FBEA0: 9BE100DC  stb r31, 0xdc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[31].u8 ) };
	// 827FBEA4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 827FBEA8: 3BA86ED0  addi r29, r8, 0x6ed0
	ctx.r[29].s64 = ctx.r[8].s64 + 28368;
	// 827FBEAC: C3CA08A4  lfs f30, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827FBEB0: 93C100CC  stw r30, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 827FBEB4: C3E99534  lfs f31, -0x6acc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827FBEB8: 93A100C4  stw r29, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[29].u32 ) };
	// 827FBEBC: D3C100D0  stfs f30, 0xd0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 827FBEC0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827FBEC4: D3E100D4  stfs f31, 0xd4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 827FBEC8: D3E100D8  stfs f31, 0xd8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 827FBECC: 816B147C  lwz r11, 0x147c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5244 as u32) ) } as u64;
	// 827FBED0: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 827FBED4: 486029F5  bl 0x82dfe8c8
	ctx.lr = 0x827FBED8;
	sub_82DFE8C8(ctx, base);
	// 827FBED8: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 827FBEDC: 486029ED  bl 0x82dfe8c8
	ctx.lr = 0x827FBEE0;
	sub_82DFE8C8(ctx, base);
	// 827FBEE0: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FBEE4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FBEE8: D3A100F8  stfs f29, 0xf8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 827FBEEC: D3C10100  stfs f30, 0x100(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 827FBEF0: 93C100FC  stw r30, 0xfc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 827FBEF4: 3B8B6EC4  addi r28, r11, 0x6ec4
	ctx.r[28].s64 = ctx.r[11].s64 + 28356;
	// 827FBEF8: D3E10104  stfs f31, 0x104(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 827FBEFC: D3E10108  stfs f31, 0x108(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 827FBF00: 9BE1010C  stb r31, 0x10c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), ctx.r[31].u8 ) };
	// 827FBF04: 938100F4  stw r28, 0xf4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), ctx.r[28].u32 ) };
	// 827FBF08: 38610110  addi r3, r1, 0x110
	ctx.r[3].s64 = ctx.r[1].s64 + 272;
	// 827FBF0C: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 827FBF10: 816A1478  lwz r11, 0x1478(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5240 as u32) ) } as u64;
	// 827FBF14: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 827FBF18: 486029B1  bl 0x82dfe8c8
	ctx.lr = 0x827FBF1C;
	sub_82DFE8C8(ctx, base);
	// 827FBF1C: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 827FBF20: 486029A9  bl 0x82dfe8c8
	ctx.lr = 0x827FBF24;
	sub_82DFE8C8(ctx, base);
	// 827FBF24: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FBF28: D3A10128  stfs f29, 0x128(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 827FBF2C: 93C1012C  stw r30, 0x12c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), ctx.r[30].u32 ) };
	// 827FBF30: D3C10130  stfs f30, 0x130(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 827FBF34: 93810124  stw r28, 0x124(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), ctx.r[28].u32 ) };
	// 827FBF38: D3E10134  stfs f31, 0x134(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 827FBF3C: 9BE1013C  stb r31, 0x13c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), ctx.r[31].u8 ) };
	// 827FBF40: D3E10138  stfs f31, 0x138(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 827FBF44: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 827FBF48: 816A1484  lwz r11, 0x1484(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5252 as u32) ) } as u64;
	// 827FBF4C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 827FBF50: 91610120  stw r11, 0x120(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 827FBF54: 48602975  bl 0x82dfe8c8
	ctx.lr = 0x827FBF58;
	sub_82DFE8C8(ctx, base);
	// 827FBF58: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 827FBF5C: 4860296D  bl 0x82dfe8c8
	ctx.lr = 0x827FBF60;
	sub_82DFE8C8(ctx, base);
	// 827FBF60: D3A10158  stfs f29, 0x158(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 827FBF64: D3C10160  stfs f30, 0x160(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 827FBF68: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FBF6C: 816A148C  lwz r11, 0x148c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5260 as u32) ) } as u64;
	// 827FBF70: D3E10164  stfs f31, 0x164(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 827FBF74: D3E10168  stfs f31, 0x168(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 827FBF78: 93C1015C  stw r30, 0x15c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 827FBF7C: 93810154  stw r28, 0x154(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), ctx.r[28].u32 ) };
	// 827FBF80: 38610170  addi r3, r1, 0x170
	ctx.r[3].s64 = ctx.r[1].s64 + 368;
	// 827FBF84: 9BE1016C  stb r31, 0x16c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), ctx.r[31].u8 ) };
	// 827FBF88: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 827FBF8C: 91610150  stw r11, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 827FBF90: 48602939  bl 0x82dfe8c8
	ctx.lr = 0x827FBF94;
	sub_82DFE8C8(ctx, base);
	// 827FBF94: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 827FBF98: 48602931  bl 0x82dfe8c8
	ctx.lr = 0x827FBF9C;
	sub_82DFE8C8(ctx, base);
	// 827FBF9C: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FBFA0: D3A10188  stfs f29, 0x188(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), tmp.u32 ) };
	// 827FBFA4: 93C1018C  stw r30, 0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), ctx.r[30].u32 ) };
	// 827FBFA8: D3C10190  stfs f30, 0x190(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(400 as u32), tmp.u32 ) };
	// 827FBFAC: 93810184  stw r28, 0x184(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), ctx.r[28].u32 ) };
	// 827FBFB0: D3E10194  stfs f31, 0x194(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(404 as u32), tmp.u32 ) };
	// 827FBFB4: 9BE1019C  stb r31, 0x19c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), ctx.r[31].u8 ) };
	// 827FBFB8: D3E10198  stfs f31, 0x198(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(408 as u32), tmp.u32 ) };
	// 827FBFBC: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 827FBFC0: 816A1480  lwz r11, 0x1480(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5248 as u32) ) } as u64;
	// 827FBFC4: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 827FBFC8: 91610180  stw r11, 0x180(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 827FBFCC: 486028FD  bl 0x82dfe8c8
	ctx.lr = 0x827FBFD0;
	sub_82DFE8C8(ctx, base);
	// 827FBFD0: 386101A8  addi r3, r1, 0x1a8
	ctx.r[3].s64 = ctx.r[1].s64 + 424;
	// 827FBFD4: 486028F5  bl 0x82dfe8c8
	ctx.lr = 0x827FBFD8;
	sub_82DFE8C8(ctx, base);
	// 827FBFD8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FBFDC: D3A101B8  stfs f29, 0x1b8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 827FBFE0: 93C101BC  stw r30, 0x1bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), ctx.r[30].u32 ) };
	// 827FBFE4: D3C101C0  stfs f30, 0x1c0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 827FBFE8: 938101B4  stw r28, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[28].u32 ) };
	// 827FBFEC: D3E101C4  stfs f31, 0x1c4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), tmp.u32 ) };
	// 827FBFF0: 9BE101CC  stb r31, 0x1cc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), ctx.r[31].u8 ) };
	// 827FBFF4: D3E101C8  stfs f31, 0x1c8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), tmp.u32 ) };
	// 827FBFF8: 386101D0  addi r3, r1, 0x1d0
	ctx.r[3].s64 = ctx.r[1].s64 + 464;
	// 827FBFFC: 816A1488  lwz r11, 0x1488(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5256 as u32) ) } as u64;
	// 827FC000: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 827FC004: 916101B0  stw r11, 0x1b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 827FC008: 486028C1  bl 0x82dfe8c8
	ctx.lr = 0x827FC00C;
	sub_82DFE8C8(ctx, base);
	// 827FC00C: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 827FC010: 486028B9  bl 0x82dfe8c8
	ctx.lr = 0x827FC014;
	sub_82DFE8C8(ctx, base);
	// 827FC014: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FC018: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC01C: D3A101E8  stfs f29, 0x1e8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(488 as u32), tmp.u32 ) };
	// 827FC020: D3C101F0  stfs f30, 0x1f0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(496 as u32), tmp.u32 ) };
	// 827FC024: 93C101EC  stw r30, 0x1ec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(492 as u32), ctx.r[30].u32 ) };
	// 827FC028: 392B6EB4  addi r9, r11, 0x6eb4
	ctx.r[9].s64 = ctx.r[11].s64 + 28340;
	// 827FC02C: D3E101F4  stfs f31, 0x1f4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(500 as u32), tmp.u32 ) };
	// 827FC030: D3E101F8  stfs f31, 0x1f8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(504 as u32), tmp.u32 ) };
	// 827FC034: 9BE101FC  stb r31, 0x1fc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(508 as u32), ctx.r[31].u8 ) };
	// 827FC038: 912101E4  stw r9, 0x1e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(484 as u32), ctx.r[9].u32 ) };
	// 827FC03C: 38610200  addi r3, r1, 0x200
	ctx.r[3].s64 = ctx.r[1].s64 + 512;
	// 827FC040: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 827FC044: 816A1490  lwz r11, 0x1490(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5264 as u32) ) } as u64;
	// 827FC048: 916101E0  stw r11, 0x1e0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(480 as u32), ctx.r[11].u32 ) };
	// 827FC04C: 4860287D  bl 0x82dfe8c8
	ctx.lr = 0x827FC050;
	sub_82DFE8C8(ctx, base);
	// 827FC050: 38610208  addi r3, r1, 0x208
	ctx.r[3].s64 = ctx.r[1].s64 + 520;
	// 827FC054: 48602875  bl 0x82dfe8c8
	ctx.lr = 0x827FC058;
	sub_82DFE8C8(ctx, base);
	// 827FC058: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FC05C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC060: D3A10218  stfs f29, 0x218(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(536 as u32), tmp.u32 ) };
	// 827FC064: D3C10220  stfs f30, 0x220(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(544 as u32), tmp.u32 ) };
	// 827FC068: 93C1021C  stw r30, 0x21c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(540 as u32), ctx.r[30].u32 ) };
	// 827FC06C: 392B6EA0  addi r9, r11, 0x6ea0
	ctx.r[9].s64 = ctx.r[11].s64 + 28320;
	// 827FC070: D3E10224  stfs f31, 0x224(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(548 as u32), tmp.u32 ) };
	// 827FC074: D3E10228  stfs f31, 0x228(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(552 as u32), tmp.u32 ) };
	// 827FC078: 9BE1022C  stb r31, 0x22c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(556 as u32), ctx.r[31].u8 ) };
	// 827FC07C: 91210214  stw r9, 0x214(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(532 as u32), ctx.r[9].u32 ) };
	// 827FC080: 38610230  addi r3, r1, 0x230
	ctx.r[3].s64 = ctx.r[1].s64 + 560;
	// 827FC084: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 827FC088: 816A1494  lwz r11, 0x1494(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5268 as u32) ) } as u64;
	// 827FC08C: 91610210  stw r11, 0x210(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(528 as u32), ctx.r[11].u32 ) };
	// 827FC090: 48602839  bl 0x82dfe8c8
	ctx.lr = 0x827FC094;
	sub_82DFE8C8(ctx, base);
	// 827FC094: 38610238  addi r3, r1, 0x238
	ctx.r[3].s64 = ctx.r[1].s64 + 568;
	// 827FC098: 48602831  bl 0x82dfe8c8
	ctx.lr = 0x827FC09C;
	sub_82DFE8C8(ctx, base);
	// 827FC09C: D3A10248  stfs f29, 0x248(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(584 as u32), tmp.u32 ) };
	// 827FC0A0: D3C10250  stfs f30, 0x250(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(592 as u32), tmp.u32 ) };
	// 827FC0A4: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FC0A8: D3E10254  stfs f31, 0x254(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(596 as u32), tmp.u32 ) };
	// 827FC0AC: 93C1024C  stw r30, 0x24c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(588 as u32), ctx.r[30].u32 ) };
	// 827FC0B0: 816A14C4  lwz r11, 0x14c4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5316 as u32) ) } as u64;
	// 827FC0B4: D3E10258  stfs f31, 0x258(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(600 as u32), tmp.u32 ) };
	// 827FC0B8: 93A10244  stw r29, 0x244(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(580 as u32), ctx.r[29].u32 ) };
	// 827FC0BC: 38610260  addi r3, r1, 0x260
	ctx.r[3].s64 = ctx.r[1].s64 + 608;
	// 827FC0C0: 9BE1025C  stb r31, 0x25c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(604 as u32), ctx.r[31].u8 ) };
	// 827FC0C4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 827FC0C8: 91610240  stw r11, 0x240(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(576 as u32), ctx.r[11].u32 ) };
	// 827FC0CC: 486027FD  bl 0x82dfe8c8
	ctx.lr = 0x827FC0D0;
	sub_82DFE8C8(ctx, base);
	// 827FC0D0: 38610268  addi r3, r1, 0x268
	ctx.r[3].s64 = ctx.r[1].s64 + 616;
	// 827FC0D4: 486027F5  bl 0x82dfe8c8
	ctx.lr = 0x827FC0D8;
	sub_82DFE8C8(ctx, base);
	// 827FC0D8: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FC0DC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC0E0: D3A10278  stfs f29, 0x278(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(632 as u32), tmp.u32 ) };
	// 827FC0E4: D3C10280  stfs f30, 0x280(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(640 as u32), tmp.u32 ) };
	// 827FC0E8: 93C1027C  stw r30, 0x27c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(636 as u32), ctx.r[30].u32 ) };
	// 827FC0EC: 3B8B6E8C  addi r28, r11, 0x6e8c
	ctx.r[28].s64 = ctx.r[11].s64 + 28300;
	// 827FC0F0: D3E10284  stfs f31, 0x284(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(644 as u32), tmp.u32 ) };
	// 827FC0F4: D3E10288  stfs f31, 0x288(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 827FC0F8: 9BE1028C  stb r31, 0x28c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(652 as u32), ctx.r[31].u8 ) };
	// 827FC0FC: 93810274  stw r28, 0x274(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(628 as u32), ctx.r[28].u32 ) };
	// 827FC100: 38610290  addi r3, r1, 0x290
	ctx.r[3].s64 = ctx.r[1].s64 + 656;
	// 827FC104: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 827FC108: 816A14C0  lwz r11, 0x14c0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5312 as u32) ) } as u64;
	// 827FC10C: 91610270  stw r11, 0x270(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(624 as u32), ctx.r[11].u32 ) };
	// 827FC110: 486027B9  bl 0x82dfe8c8
	ctx.lr = 0x827FC114;
	sub_82DFE8C8(ctx, base);
	// 827FC114: 38610298  addi r3, r1, 0x298
	ctx.r[3].s64 = ctx.r[1].s64 + 664;
	// 827FC118: 486027B1  bl 0x82dfe8c8
	ctx.lr = 0x827FC11C;
	sub_82DFE8C8(ctx, base);
	// 827FC11C: 3F60832C  lis r27, -0x7cd4
	ctx.r[27].s64 = -2094268416;
	// 827FC120: D3A102A8  stfs f29, 0x2a8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(680 as u32), tmp.u32 ) };
	// 827FC124: 93C102AC  stw r30, 0x2ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(684 as u32), ctx.r[30].u32 ) };
	// 827FC128: D3C102B0  stfs f30, 0x2b0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(688 as u32), tmp.u32 ) };
	// 827FC12C: 938102A4  stw r28, 0x2a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(676 as u32), ctx.r[28].u32 ) };
	// 827FC130: D3E102B4  stfs f31, 0x2b4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(692 as u32), tmp.u32 ) };
	// 827FC134: 9BE102BC  stb r31, 0x2bc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(700 as u32), ctx.r[31].u8 ) };
	// 827FC138: D3E102B8  stfs f31, 0x2b8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(696 as u32), tmp.u32 ) };
	// 827FC13C: 386102C0  addi r3, r1, 0x2c0
	ctx.r[3].s64 = ctx.r[1].s64 + 704;
	// 827FC140: 817B14CC  lwz r11, 0x14cc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(5324 as u32) ) } as u64;
	// 827FC144: 916102A0  stw r11, 0x2a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(672 as u32), ctx.r[11].u32 ) };
	// 827FC148: 48602781  bl 0x82dfe8c8
	ctx.lr = 0x827FC14C;
	sub_82DFE8C8(ctx, base);
	// 827FC14C: 386102C8  addi r3, r1, 0x2c8
	ctx.r[3].s64 = ctx.r[1].s64 + 712;
	// 827FC150: 48602779  bl 0x82dfe8c8
	ctx.lr = 0x827FC154;
	sub_82DFE8C8(ctx, base);
	// 827FC154: 3F40832C  lis r26, -0x7cd4
	ctx.r[26].s64 = -2094268416;
	// 827FC158: D3A102D8  stfs f29, 0x2d8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(728 as u32), tmp.u32 ) };
	// 827FC15C: 93C102DC  stw r30, 0x2dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(732 as u32), ctx.r[30].u32 ) };
	// 827FC160: D3C102E0  stfs f30, 0x2e0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(736 as u32), tmp.u32 ) };
	// 827FC164: 938102D4  stw r28, 0x2d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(724 as u32), ctx.r[28].u32 ) };
	// 827FC168: D3E102E4  stfs f31, 0x2e4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(740 as u32), tmp.u32 ) };
	// 827FC16C: 9BE102EC  stb r31, 0x2ec(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(748 as u32), ctx.r[31].u8 ) };
	// 827FC170: D3E102E8  stfs f31, 0x2e8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(744 as u32), tmp.u32 ) };
	// 827FC174: 386102F0  addi r3, r1, 0x2f0
	ctx.r[3].s64 = ctx.r[1].s64 + 752;
	// 827FC178: 817A14D4  lwz r11, 0x14d4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(5332 as u32) ) } as u64;
	// 827FC17C: 916102D0  stw r11, 0x2d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(720 as u32), ctx.r[11].u32 ) };
	// 827FC180: 48602749  bl 0x82dfe8c8
	ctx.lr = 0x827FC184;
	sub_82DFE8C8(ctx, base);
	// 827FC184: 386102F8  addi r3, r1, 0x2f8
	ctx.r[3].s64 = ctx.r[1].s64 + 760;
	// 827FC188: 48602741  bl 0x82dfe8c8
	ctx.lr = 0x827FC18C;
	sub_82DFE8C8(ctx, base);
	// 827FC18C: 3F20832C  lis r25, -0x7cd4
	ctx.r[25].s64 = -2094268416;
	// 827FC190: D3A10308  stfs f29, 0x308(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(776 as u32), tmp.u32 ) };
	// 827FC194: 93C1030C  stw r30, 0x30c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(780 as u32), ctx.r[30].u32 ) };
	// 827FC198: D3C10310  stfs f30, 0x310(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(784 as u32), tmp.u32 ) };
	// 827FC19C: 93810304  stw r28, 0x304(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(772 as u32), ctx.r[28].u32 ) };
	// 827FC1A0: D3E10314  stfs f31, 0x314(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(788 as u32), tmp.u32 ) };
	// 827FC1A4: 9BE1031C  stb r31, 0x31c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(796 as u32), ctx.r[31].u8 ) };
	// 827FC1A8: D3E10318  stfs f31, 0x318(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(792 as u32), tmp.u32 ) };
	// 827FC1AC: 38610320  addi r3, r1, 0x320
	ctx.r[3].s64 = ctx.r[1].s64 + 800;
	// 827FC1B0: 817914C8  lwz r11, 0x14c8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(5320 as u32) ) } as u64;
	// 827FC1B4: 91610300  stw r11, 0x300(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(768 as u32), ctx.r[11].u32 ) };
	// 827FC1B8: 48602711  bl 0x82dfe8c8
	ctx.lr = 0x827FC1BC;
	sub_82DFE8C8(ctx, base);
	// 827FC1BC: 38610328  addi r3, r1, 0x328
	ctx.r[3].s64 = ctx.r[1].s64 + 808;
	// 827FC1C0: 48602709  bl 0x82dfe8c8
	ctx.lr = 0x827FC1C4;
	sub_82DFE8C8(ctx, base);
	// 827FC1C4: 3F00832C  lis r24, -0x7cd4
	ctx.r[24].s64 = -2094268416;
	// 827FC1C8: D3A10338  stfs f29, 0x338(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(824 as u32), tmp.u32 ) };
	// 827FC1CC: 93C1033C  stw r30, 0x33c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(828 as u32), ctx.r[30].u32 ) };
	// 827FC1D0: D3C10340  stfs f30, 0x340(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(832 as u32), tmp.u32 ) };
	// 827FC1D4: 93810334  stw r28, 0x334(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(820 as u32), ctx.r[28].u32 ) };
	// 827FC1D8: D3E10344  stfs f31, 0x344(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(836 as u32), tmp.u32 ) };
	// 827FC1DC: 9BE1034C  stb r31, 0x34c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(844 as u32), ctx.r[31].u8 ) };
	// 827FC1E0: D3E10348  stfs f31, 0x348(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(840 as u32), tmp.u32 ) };
	// 827FC1E4: 38610350  addi r3, r1, 0x350
	ctx.r[3].s64 = ctx.r[1].s64 + 848;
	// 827FC1E8: 817814D0  lwz r11, 0x14d0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(5328 as u32) ) } as u64;
	// 827FC1EC: 91610330  stw r11, 0x330(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(816 as u32), ctx.r[11].u32 ) };
	// 827FC1F0: 486026D9  bl 0x82dfe8c8
	ctx.lr = 0x827FC1F4;
	sub_82DFE8C8(ctx, base);
	// 827FC1F4: 38610358  addi r3, r1, 0x358
	ctx.r[3].s64 = ctx.r[1].s64 + 856;
	// 827FC1F8: 486026D1  bl 0x82dfe8c8
	ctx.lr = 0x827FC1FC;
	sub_82DFE8C8(ctx, base);
	// 827FC1FC: 3F80832C  lis r28, -0x7cd4
	ctx.r[28].s64 = -2094268416;
	// 827FC200: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC204: D3A10368  stfs f29, 0x368(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(872 as u32), tmp.u32 ) };
	// 827FC208: D3C10370  stfs f30, 0x370(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(880 as u32), tmp.u32 ) };
	// 827FC20C: 93C1036C  stw r30, 0x36c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(876 as u32), ctx.r[30].u32 ) };
	// 827FC210: 394B6E74  addi r10, r11, 0x6e74
	ctx.r[10].s64 = ctx.r[11].s64 + 28276;
	// 827FC214: D3E10374  stfs f31, 0x374(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(884 as u32), tmp.u32 ) };
	// 827FC218: D3E10378  stfs f31, 0x378(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(888 as u32), tmp.u32 ) };
	// 827FC21C: 9BE1037C  stb r31, 0x37c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(892 as u32), ctx.r[31].u8 ) };
	// 827FC220: 91410364  stw r10, 0x364(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(868 as u32), ctx.r[10].u32 ) };
	// 827FC224: 38610380  addi r3, r1, 0x380
	ctx.r[3].s64 = ctx.r[1].s64 + 896;
	// 827FC228: 817C14D8  lwz r11, 0x14d8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5336 as u32) ) } as u64;
	// 827FC22C: 91610360  stw r11, 0x360(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(864 as u32), ctx.r[11].u32 ) };
	// 827FC230: 48602699  bl 0x82dfe8c8
	ctx.lr = 0x827FC234;
	sub_82DFE8C8(ctx, base);
	// 827FC234: 38610388  addi r3, r1, 0x388
	ctx.r[3].s64 = ctx.r[1].s64 + 904;
	// 827FC238: 48602691  bl 0x82dfe8c8
	ctx.lr = 0x827FC23C;
	sub_82DFE8C8(ctx, base);
	// 827FC23C: 3EE0832C  lis r23, -0x7cd4
	ctx.r[23].s64 = -2094268416;
	// 827FC240: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC244: D3A10398  stfs f29, 0x398(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(920 as u32), tmp.u32 ) };
	// 827FC248: D3C103A0  stfs f30, 0x3a0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(928 as u32), tmp.u32 ) };
	// 827FC24C: 93C1039C  stw r30, 0x39c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(924 as u32), ctx.r[30].u32 ) };
	// 827FC250: 394B6E5C  addi r10, r11, 0x6e5c
	ctx.r[10].s64 = ctx.r[11].s64 + 28252;
	// 827FC254: D3E103A4  stfs f31, 0x3a4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(932 as u32), tmp.u32 ) };
	// 827FC258: D3E103A8  stfs f31, 0x3a8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(936 as u32), tmp.u32 ) };
	// 827FC25C: 9BE103AC  stb r31, 0x3ac(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(940 as u32), ctx.r[31].u8 ) };
	// 827FC260: 91410394  stw r10, 0x394(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(916 as u32), ctx.r[10].u32 ) };
	// 827FC264: 386103B0  addi r3, r1, 0x3b0
	ctx.r[3].s64 = ctx.r[1].s64 + 944;
	// 827FC268: 817714DC  lwz r11, 0x14dc(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(5340 as u32) ) } as u64;
	// 827FC26C: 91610390  stw r11, 0x390(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(912 as u32), ctx.r[11].u32 ) };
	// 827FC270: 48602659  bl 0x82dfe8c8
	ctx.lr = 0x827FC274;
	sub_82DFE8C8(ctx, base);
	// 827FC274: 386103B8  addi r3, r1, 0x3b8
	ctx.r[3].s64 = ctx.r[1].s64 + 952;
	// 827FC278: 48602651  bl 0x82dfe8c8
	ctx.lr = 0x827FC27C;
	sub_82DFE8C8(ctx, base);
	// 827FC27C: 3EC0832C  lis r22, -0x7cd4
	ctx.r[22].s64 = -2094268416;
	// 827FC280: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC284: D3A103C8  stfs f29, 0x3c8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(968 as u32), tmp.u32 ) };
	// 827FC288: D3C103D0  stfs f30, 0x3d0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(976 as u32), tmp.u32 ) };
	// 827FC28C: 93C103CC  stw r30, 0x3cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(972 as u32), ctx.r[30].u32 ) };
	// 827FC290: 394B6E44  addi r10, r11, 0x6e44
	ctx.r[10].s64 = ctx.r[11].s64 + 28228;
	// 827FC294: D3E103D4  stfs f31, 0x3d4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(980 as u32), tmp.u32 ) };
	// 827FC298: D3E103D8  stfs f31, 0x3d8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(984 as u32), tmp.u32 ) };
	// 827FC29C: 9BE103DC  stb r31, 0x3dc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(988 as u32), ctx.r[31].u8 ) };
	// 827FC2A0: 914103C4  stw r10, 0x3c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(964 as u32), ctx.r[10].u32 ) };
	// 827FC2A4: 386103E0  addi r3, r1, 0x3e0
	ctx.r[3].s64 = ctx.r[1].s64 + 992;
	// 827FC2A8: 817614E0  lwz r11, 0x14e0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5344 as u32) ) } as u64;
	// 827FC2AC: 916103C0  stw r11, 0x3c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(960 as u32), ctx.r[11].u32 ) };
	// 827FC2B0: 48602619  bl 0x82dfe8c8
	ctx.lr = 0x827FC2B4;
	sub_82DFE8C8(ctx, base);
	// 827FC2B4: 386103E8  addi r3, r1, 0x3e8
	ctx.r[3].s64 = ctx.r[1].s64 + 1000;
	// 827FC2B8: 48602611  bl 0x82dfe8c8
	ctx.lr = 0x827FC2BC;
	sub_82DFE8C8(ctx, base);
	// 827FC2BC: 3EA0832C  lis r21, -0x7cd4
	ctx.r[21].s64 = -2094268416;
	// 827FC2C0: D3A103F8  stfs f29, 0x3f8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1016 as u32), tmp.u32 ) };
	// 827FC2C4: 93A103F4  stw r29, 0x3f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1012 as u32), ctx.r[29].u32 ) };
	// 827FC2C8: D3C10400  stfs f30, 0x400(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1024 as u32), tmp.u32 ) };
	// 827FC2CC: 93C103FC  stw r30, 0x3fc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1020 as u32), ctx.r[30].u32 ) };
	// 827FC2D0: D3E10404  stfs f31, 0x404(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1028 as u32), tmp.u32 ) };
	// 827FC2D4: 9BE1040C  stb r31, 0x40c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1036 as u32), ctx.r[31].u8 ) };
	// 827FC2D8: D3E10408  stfs f31, 0x408(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1032 as u32), tmp.u32 ) };
	// 827FC2DC: 38610410  addi r3, r1, 0x410
	ctx.r[3].s64 = ctx.r[1].s64 + 1040;
	// 827FC2E0: 817514E4  lwz r11, 0x14e4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(5348 as u32) ) } as u64;
	// 827FC2E4: 916103F0  stw r11, 0x3f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1008 as u32), ctx.r[11].u32 ) };
	// 827FC2E8: 486025E1  bl 0x82dfe8c8
	ctx.lr = 0x827FC2EC;
	sub_82DFE8C8(ctx, base);
	// 827FC2EC: 38610418  addi r3, r1, 0x418
	ctx.r[3].s64 = ctx.r[1].s64 + 1048;
	// 827FC2F0: 486025D9  bl 0x82dfe8c8
	ctx.lr = 0x827FC2F4;
	sub_82DFE8C8(ctx, base);
	// 827FC2F4: 3E80832C  lis r20, -0x7cd4
	ctx.r[20].s64 = -2094268416;
	// 827FC2F8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC2FC: D3A10428  stfs f29, 0x428(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1064 as u32), tmp.u32 ) };
	// 827FC300: D3C10430  stfs f30, 0x430(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1072 as u32), tmp.u32 ) };
	// 827FC304: 93C1042C  stw r30, 0x42c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1068 as u32), ctx.r[30].u32 ) };
	// 827FC308: 394B6E34  addi r10, r11, 0x6e34
	ctx.r[10].s64 = ctx.r[11].s64 + 28212;
	// 827FC30C: D3E10434  stfs f31, 0x434(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1076 as u32), tmp.u32 ) };
	// 827FC310: D3E10438  stfs f31, 0x438(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1080 as u32), tmp.u32 ) };
	// 827FC314: 9BE1043C  stb r31, 0x43c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1084 as u32), ctx.r[31].u8 ) };
	// 827FC318: 91410424  stw r10, 0x424(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1060 as u32), ctx.r[10].u32 ) };
	// 827FC31C: 38610440  addi r3, r1, 0x440
	ctx.r[3].s64 = ctx.r[1].s64 + 1088;
	// 827FC320: 817414B4  lwz r11, 0x14b4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5300 as u32) ) } as u64;
	// 827FC324: 91610420  stw r11, 0x420(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1056 as u32), ctx.r[11].u32 ) };
	// 827FC328: 486025A1  bl 0x82dfe8c8
	ctx.lr = 0x827FC32C;
	sub_82DFE8C8(ctx, base);
	// 827FC32C: 38610448  addi r3, r1, 0x448
	ctx.r[3].s64 = ctx.r[1].s64 + 1096;
	// 827FC330: 48602599  bl 0x82dfe8c8
	ctx.lr = 0x827FC334;
	sub_82DFE8C8(ctx, base);
	// 827FC334: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC338: 816B14FC  lwz r11, 0x14fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5372 as u32) ) } as u64;
	// 827FC33C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC340: D3A10458  stfs f29, 0x458(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1112 as u32), tmp.u32 ) };
	// 827FC344: 93E1045C  stw r31, 0x45c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1116 as u32), ctx.r[31].u32 ) };
	// 827FC348: 3A6A6E24  addi r19, r10, 0x6e24
	ctx.r[19].s64 = ctx.r[10].s64 + 28196;
	// 827FC34C: D3C10460  stfs f30, 0x460(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1120 as u32), tmp.u32 ) };
	// 827FC350: D3E10464  stfs f31, 0x464(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1124 as u32), tmp.u32 ) };
	// 827FC354: 9BE1046C  stb r31, 0x46c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1132 as u32), ctx.r[31].u8 ) };
	// 827FC358: D3E10468  stfs f31, 0x468(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1128 as u32), tmp.u32 ) };
	// 827FC35C: 92610454  stw r19, 0x454(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1108 as u32), ctx.r[19].u32 ) };
	// 827FC360: 91610450  stw r11, 0x450(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1104 as u32), ctx.r[11].u32 ) };
	// 827FC364: 38610470  addi r3, r1, 0x470
	ctx.r[3].s64 = ctx.r[1].s64 + 1136;
	// 827FC368: 48602561  bl 0x82dfe8c8
	ctx.lr = 0x827FC36C;
	sub_82DFE8C8(ctx, base);
	// 827FC36C: 38610478  addi r3, r1, 0x478
	ctx.r[3].s64 = ctx.r[1].s64 + 1144;
	// 827FC370: 48602559  bl 0x82dfe8c8
	ctx.lr = 0x827FC374;
	sub_82DFE8C8(ctx, base);
	// 827FC374: 3E40832C  lis r18, -0x7cd4
	ctx.r[18].s64 = -2094268416;
	// 827FC378: D3A10488  stfs f29, 0x488(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1160 as u32), tmp.u32 ) };
	// 827FC37C: 93E1048C  stw r31, 0x48c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1164 as u32), ctx.r[31].u32 ) };
	// 827FC380: D3C10490  stfs f30, 0x490(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1168 as u32), tmp.u32 ) };
	// 827FC384: 92610484  stw r19, 0x484(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1156 as u32), ctx.r[19].u32 ) };
	// 827FC388: D3E10494  stfs f31, 0x494(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1172 as u32), tmp.u32 ) };
	// 827FC38C: 9BE1049C  stb r31, 0x49c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1180 as u32), ctx.r[31].u8 ) };
	// 827FC390: D3E10498  stfs f31, 0x498(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1176 as u32), tmp.u32 ) };
	// 827FC394: 386104A0  addi r3, r1, 0x4a0
	ctx.r[3].s64 = ctx.r[1].s64 + 1184;
	// 827FC398: 81721508  lwz r11, 0x1508(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(5384 as u32) ) } as u64;
	// 827FC39C: 91610480  stw r11, 0x480(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1152 as u32), ctx.r[11].u32 ) };
	// 827FC3A0: 48602529  bl 0x82dfe8c8
	ctx.lr = 0x827FC3A4;
	sub_82DFE8C8(ctx, base);
	// 827FC3A4: 386104A8  addi r3, r1, 0x4a8
	ctx.r[3].s64 = ctx.r[1].s64 + 1192;
	// 827FC3A8: 48602521  bl 0x82dfe8c8
	ctx.lr = 0x827FC3AC;
	sub_82DFE8C8(ctx, base);
	// 827FC3AC: 3E60832C  lis r19, -0x7cd4
	ctx.r[19].s64 = -2094268416;
	// 827FC3B0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC3B4: D3A104B8  stfs f29, 0x4b8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1208 as u32), tmp.u32 ) };
	// 827FC3B8: D3C104C0  stfs f30, 0x4c0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1216 as u32), tmp.u32 ) };
	// 827FC3BC: 93C104BC  stw r30, 0x4bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1212 as u32), ctx.r[30].u32 ) };
	// 827FC3C0: 394B6E10  addi r10, r11, 0x6e10
	ctx.r[10].s64 = ctx.r[11].s64 + 28176;
	// 827FC3C4: D3E104C4  stfs f31, 0x4c4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1220 as u32), tmp.u32 ) };
	// 827FC3C8: D3E104C8  stfs f31, 0x4c8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1224 as u32), tmp.u32 ) };
	// 827FC3CC: 9BE104CC  stb r31, 0x4cc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1228 as u32), ctx.r[31].u8 ) };
	// 827FC3D0: 914104B4  stw r10, 0x4b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1204 as u32), ctx.r[10].u32 ) };
	// 827FC3D4: 386104D0  addi r3, r1, 0x4d0
	ctx.r[3].s64 = ctx.r[1].s64 + 1232;
	// 827FC3D8: 8173149C  lwz r11, 0x149c(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(5276 as u32) ) } as u64;
	// 827FC3DC: 916104B0  stw r11, 0x4b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1200 as u32), ctx.r[11].u32 ) };
	// 827FC3E0: 486024E9  bl 0x82dfe8c8
	ctx.lr = 0x827FC3E4;
	sub_82DFE8C8(ctx, base);
	// 827FC3E4: 386104D8  addi r3, r1, 0x4d8
	ctx.r[3].s64 = ctx.r[1].s64 + 1240;
	// 827FC3E8: 486024E1  bl 0x82dfe8c8
	ctx.lr = 0x827FC3EC;
	sub_82DFE8C8(ctx, base);
	// 827FC3EC: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC3F0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC3F4: D3A104E8  stfs f29, 0x4e8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1256 as u32), tmp.u32 ) };
	// 827FC3F8: D3C104F0  stfs f30, 0x4f0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1264 as u32), tmp.u32 ) };
	// 827FC3FC: 93C104EC  stw r30, 0x4ec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1260 as u32), ctx.r[30].u32 ) };
	// 827FC400: 394A6DFC  addi r10, r10, 0x6dfc
	ctx.r[10].s64 = ctx.r[10].s64 + 28156;
	// 827FC404: D3E104F4  stfs f31, 0x4f4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1268 as u32), tmp.u32 ) };
	// 827FC408: D3E104F8  stfs f31, 0x4f8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1272 as u32), tmp.u32 ) };
	// 827FC40C: 9BE104FC  stb r31, 0x4fc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1276 as u32), ctx.r[31].u8 ) };
	// 827FC410: 914104E4  stw r10, 0x4e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1252 as u32), ctx.r[10].u32 ) };
	// 827FC414: 38610500  addi r3, r1, 0x500
	ctx.r[3].s64 = ctx.r[1].s64 + 1280;
	// 827FC418: 816B1498  lwz r11, 0x1498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5272 as u32) ) } as u64;
	// 827FC41C: 916104E0  stw r11, 0x4e0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1248 as u32), ctx.r[11].u32 ) };
	// 827FC420: 486024A9  bl 0x82dfe8c8
	ctx.lr = 0x827FC424;
	sub_82DFE8C8(ctx, base);
	// 827FC424: 38610508  addi r3, r1, 0x508
	ctx.r[3].s64 = ctx.r[1].s64 + 1288;
	// 827FC428: 486024A1  bl 0x82dfe8c8
	ctx.lr = 0x827FC42C;
	sub_82DFE8C8(ctx, base);
	// 827FC42C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC430: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC434: D3A10518  stfs f29, 0x518(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1304 as u32), tmp.u32 ) };
	// 827FC438: D3C10520  stfs f30, 0x520(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1312 as u32), tmp.u32 ) };
	// 827FC43C: 93C1051C  stw r30, 0x51c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1308 as u32), ctx.r[30].u32 ) };
	// 827FC440: 394A6DE8  addi r10, r10, 0x6de8
	ctx.r[10].s64 = ctx.r[10].s64 + 28136;
	// 827FC444: D3E10524  stfs f31, 0x524(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1316 as u32), tmp.u32 ) };
	// 827FC448: D3E10528  stfs f31, 0x528(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1320 as u32), tmp.u32 ) };
	// 827FC44C: 9BE1052C  stb r31, 0x52c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1324 as u32), ctx.r[31].u8 ) };
	// 827FC450: 91410514  stw r10, 0x514(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1300 as u32), ctx.r[10].u32 ) };
	// 827FC454: 38610530  addi r3, r1, 0x530
	ctx.r[3].s64 = ctx.r[1].s64 + 1328;
	// 827FC458: 816B14A4  lwz r11, 0x14a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5284 as u32) ) } as u64;
	// 827FC45C: 91610510  stw r11, 0x510(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1296 as u32), ctx.r[11].u32 ) };
	// 827FC460: 48602469  bl 0x82dfe8c8
	ctx.lr = 0x827FC464;
	sub_82DFE8C8(ctx, base);
	// 827FC464: 38610538  addi r3, r1, 0x538
	ctx.r[3].s64 = ctx.r[1].s64 + 1336;
	// 827FC468: 48602461  bl 0x82dfe8c8
	ctx.lr = 0x827FC46C;
	sub_82DFE8C8(ctx, base);
	// 827FC46C: D3A10548  stfs f29, 0x548(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1352 as u32), tmp.u32 ) };
	// 827FC470: D3C10550  stfs f30, 0x550(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1360 as u32), tmp.u32 ) };
	// 827FC474: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC478: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC47C: 816B14A0  lwz r11, 0x14a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5280 as u32) ) } as u64;
	// 827FC480: 394A6DD4  addi r10, r10, 0x6dd4
	ctx.r[10].s64 = ctx.r[10].s64 + 28116;
	// 827FC484: D3E10554  stfs f31, 0x554(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1364 as u32), tmp.u32 ) };
	// 827FC488: 93C1054C  stw r30, 0x54c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1356 as u32), ctx.r[30].u32 ) };
	// 827FC48C: D3E10558  stfs f31, 0x558(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1368 as u32), tmp.u32 ) };
	// 827FC490: 91410544  stw r10, 0x544(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1348 as u32), ctx.r[10].u32 ) };
	// 827FC494: 9BE1055C  stb r31, 0x55c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1372 as u32), ctx.r[31].u8 ) };
	// 827FC498: 38610560  addi r3, r1, 0x560
	ctx.r[3].s64 = ctx.r[1].s64 + 1376;
	// 827FC49C: 91610540  stw r11, 0x540(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1344 as u32), ctx.r[11].u32 ) };
	// 827FC4A0: 48602429  bl 0x82dfe8c8
	ctx.lr = 0x827FC4A4;
	sub_82DFE8C8(ctx, base);
	// 827FC4A4: 38610568  addi r3, r1, 0x568
	ctx.r[3].s64 = ctx.r[1].s64 + 1384;
	// 827FC4A8: 48602421  bl 0x82dfe8c8
	ctx.lr = 0x827FC4AC;
	sub_82DFE8C8(ctx, base);
	// 827FC4AC: 3E20832C  lis r17, -0x7cd4
	ctx.r[17].s64 = -2094268416;
	// 827FC4B0: D3A10578  stfs f29, 0x578(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1400 as u32), tmp.u32 ) };
	// 827FC4B4: 93E1057C  stw r31, 0x57c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1404 as u32), ctx.r[31].u32 ) };
	// 827FC4B8: D3C10580  stfs f30, 0x580(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1408 as u32), tmp.u32 ) };
	// 827FC4BC: 93A10574  stw r29, 0x574(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1396 as u32), ctx.r[29].u32 ) };
	// 827FC4C0: D3E10584  stfs f31, 0x584(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1412 as u32), tmp.u32 ) };
	// 827FC4C4: 9BE1058C  stb r31, 0x58c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1420 as u32), ctx.r[31].u8 ) };
	// 827FC4C8: D3E10588  stfs f31, 0x588(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1416 as u32), tmp.u32 ) };
	// 827FC4CC: 38610590  addi r3, r1, 0x590
	ctx.r[3].s64 = ctx.r[1].s64 + 1424;
	// 827FC4D0: 817114A8  lwz r11, 0x14a8(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(5288 as u32) ) } as u64;
	// 827FC4D4: 91610570  stw r11, 0x570(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1392 as u32), ctx.r[11].u32 ) };
	// 827FC4D8: 486023F1  bl 0x82dfe8c8
	ctx.lr = 0x827FC4DC;
	sub_82DFE8C8(ctx, base);
	// 827FC4DC: 38610598  addi r3, r1, 0x598
	ctx.r[3].s64 = ctx.r[1].s64 + 1432;
	// 827FC4E0: 486023E9  bl 0x82dfe8c8
	ctx.lr = 0x827FC4E4;
	sub_82DFE8C8(ctx, base);
	// 827FC4E4: 3E00832C  lis r16, -0x7cd4
	ctx.r[16].s64 = -2094268416;
	// 827FC4E8: D3A105A8  stfs f29, 0x5a8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1448 as u32), tmp.u32 ) };
	// 827FC4EC: 93E105AC  stw r31, 0x5ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1452 as u32), ctx.r[31].u32 ) };
	// 827FC4F0: D3C105B0  stfs f30, 0x5b0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1456 as u32), tmp.u32 ) };
	// 827FC4F4: 93A105A4  stw r29, 0x5a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1444 as u32), ctx.r[29].u32 ) };
	// 827FC4F8: D3E105B4  stfs f31, 0x5b4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1460 as u32), tmp.u32 ) };
	// 827FC4FC: 9BE105BC  stb r31, 0x5bc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1468 as u32), ctx.r[31].u8 ) };
	// 827FC500: D3E105B8  stfs f31, 0x5b8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1464 as u32), tmp.u32 ) };
	// 827FC504: 386105C0  addi r3, r1, 0x5c0
	ctx.r[3].s64 = ctx.r[1].s64 + 1472;
	// 827FC508: 817014AC  lwz r11, 0x14ac(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(5292 as u32) ) } as u64;
	// 827FC50C: 916105A0  stw r11, 0x5a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1440 as u32), ctx.r[11].u32 ) };
	// 827FC510: 486023B9  bl 0x82dfe8c8
	ctx.lr = 0x827FC514;
	sub_82DFE8C8(ctx, base);
	// 827FC514: 386105C8  addi r3, r1, 0x5c8
	ctx.r[3].s64 = ctx.r[1].s64 + 1480;
	// 827FC518: 486023B1  bl 0x82dfe8c8
	ctx.lr = 0x827FC51C;
	sub_82DFE8C8(ctx, base);
	// 827FC51C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC520: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC524: D3A105D8  stfs f29, 0x5d8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1496 as u32), tmp.u32 ) };
	// 827FC528: D3C105E0  stfs f30, 0x5e0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1504 as u32), tmp.u32 ) };
	// 827FC52C: 93E105DC  stw r31, 0x5dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1500 as u32), ctx.r[31].u32 ) };
	// 827FC530: 394A6DC4  addi r10, r10, 0x6dc4
	ctx.r[10].s64 = ctx.r[10].s64 + 28100;
	// 827FC534: D3E105E4  stfs f31, 0x5e4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1508 as u32), tmp.u32 ) };
	// 827FC538: D3E105E8  stfs f31, 0x5e8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1512 as u32), tmp.u32 ) };
	// 827FC53C: 9BE105EC  stb r31, 0x5ec(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1516 as u32), ctx.r[31].u8 ) };
	// 827FC540: 914105D4  stw r10, 0x5d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1492 as u32), ctx.r[10].u32 ) };
	// 827FC544: 386105F0  addi r3, r1, 0x5f0
	ctx.r[3].s64 = ctx.r[1].s64 + 1520;
	// 827FC548: 816B14F8  lwz r11, 0x14f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5368 as u32) ) } as u64;
	// 827FC54C: 916105D0  stw r11, 0x5d0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1488 as u32), ctx.r[11].u32 ) };
	// 827FC550: 48602379  bl 0x82dfe8c8
	ctx.lr = 0x827FC554;
	sub_82DFE8C8(ctx, base);
	// 827FC554: 386105F8  addi r3, r1, 0x5f8
	ctx.r[3].s64 = ctx.r[1].s64 + 1528;
	// 827FC558: 48602371  bl 0x82dfe8c8
	ctx.lr = 0x827FC55C;
	sub_82DFE8C8(ctx, base);
	// 827FC55C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC560: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC564: D3A10608  stfs f29, 0x608(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1544 as u32), tmp.u32 ) };
	// 827FC568: D3C10610  stfs f30, 0x610(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1552 as u32), tmp.u32 ) };
	// 827FC56C: 93E1060C  stw r31, 0x60c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1548 as u32), ctx.r[31].u32 ) };
	// 827FC570: 394A6DB4  addi r10, r10, 0x6db4
	ctx.r[10].s64 = ctx.r[10].s64 + 28084;
	// 827FC574: D3E10614  stfs f31, 0x614(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1556 as u32), tmp.u32 ) };
	// 827FC578: D3E10618  stfs f31, 0x618(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1560 as u32), tmp.u32 ) };
	// 827FC57C: 9BE1061C  stb r31, 0x61c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1564 as u32), ctx.r[31].u8 ) };
	// 827FC580: 91410604  stw r10, 0x604(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1540 as u32), ctx.r[10].u32 ) };
	// 827FC584: 38610620  addi r3, r1, 0x620
	ctx.r[3].s64 = ctx.r[1].s64 + 1568;
	// 827FC588: 816B1500  lwz r11, 0x1500(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5376 as u32) ) } as u64;
	// 827FC58C: 91610600  stw r11, 0x600(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1536 as u32), ctx.r[11].u32 ) };
	// 827FC590: 48602339  bl 0x82dfe8c8
	ctx.lr = 0x827FC594;
	sub_82DFE8C8(ctx, base);
	// 827FC594: 38610628  addi r3, r1, 0x628
	ctx.r[3].s64 = ctx.r[1].s64 + 1576;
	// 827FC598: 48602331  bl 0x82dfe8c8
	ctx.lr = 0x827FC59C;
	sub_82DFE8C8(ctx, base);
	// 827FC59C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC5A0: D3A10638  stfs f29, 0x638(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1592 as u32), tmp.u32 ) };
	// 827FC5A4: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 827FC5A8: D3C10640  stfs f30, 0x640(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1600 as u32), tmp.u32 ) };
	// 827FC5AC: 392B6DA4  addi r9, r11, 0x6da4
	ctx.r[9].s64 = ctx.r[11].s64 + 28068;
	// 827FC5B0: D3E10644  stfs f31, 0x644(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1604 as u32), tmp.u32 ) };
	// 827FC5B4: 93E1063C  stw r31, 0x63c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1596 as u32), ctx.r[31].u32 ) };
	// 827FC5B8: D3E10648  stfs f31, 0x648(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1608 as u32), tmp.u32 ) };
	// 827FC5BC: 38610650  addi r3, r1, 0x650
	ctx.r[3].s64 = ctx.r[1].s64 + 1616;
	// 827FC5C0: 816A1504  lwz r11, 0x1504(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5380 as u32) ) } as u64;
	// 827FC5C4: 91210634  stw r9, 0x634(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1588 as u32), ctx.r[9].u32 ) };
	// 827FC5C8: 9BE1064C  stb r31, 0x64c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1612 as u32), ctx.r[31].u8 ) };
	// 827FC5CC: 91610630  stw r11, 0x630(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1584 as u32), ctx.r[11].u32 ) };
	// 827FC5D0: 486022F9  bl 0x82dfe8c8
	ctx.lr = 0x827FC5D4;
	sub_82DFE8C8(ctx, base);
	// 827FC5D4: 38610658  addi r3, r1, 0x658
	ctx.r[3].s64 = ctx.r[1].s64 + 1624;
	// 827FC5D8: 486022F1  bl 0x82dfe8c8
	ctx.lr = 0x827FC5DC;
	sub_82DFE8C8(ctx, base);
	// 827FC5DC: 3DE0832C  lis r15, -0x7cd4
	ctx.r[15].s64 = -2094268416;
	// 827FC5E0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC5E4: D3A10668  stfs f29, 0x668(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1640 as u32), tmp.u32 ) };
	// 827FC5E8: D3C10670  stfs f30, 0x670(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1648 as u32), tmp.u32 ) };
	// 827FC5EC: 93C1066C  stw r30, 0x66c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1644 as u32), ctx.r[30].u32 ) };
	// 827FC5F0: 394B6D90  addi r10, r11, 0x6d90
	ctx.r[10].s64 = ctx.r[11].s64 + 28048;
	// 827FC5F4: D3E10674  stfs f31, 0x674(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1652 as u32), tmp.u32 ) };
	// 827FC5F8: D3E10678  stfs f31, 0x678(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1656 as u32), tmp.u32 ) };
	// 827FC5FC: 9BE1067C  stb r31, 0x67c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1660 as u32), ctx.r[31].u8 ) };
	// 827FC600: 91410664  stw r10, 0x664(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1636 as u32), ctx.r[10].u32 ) };
	// 827FC604: 38610680  addi r3, r1, 0x680
	ctx.r[3].s64 = ctx.r[1].s64 + 1664;
	// 827FC608: 816F14EC  lwz r11, 0x14ec(r15)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(5356 as u32) ) } as u64;
	// 827FC60C: 91610660  stw r11, 0x660(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1632 as u32), ctx.r[11].u32 ) };
	// 827FC610: 486022B9  bl 0x82dfe8c8
	ctx.lr = 0x827FC614;
	sub_82DFE8C8(ctx, base);
	// 827FC614: 38610688  addi r3, r1, 0x688
	ctx.r[3].s64 = ctx.r[1].s64 + 1672;
	// 827FC618: 486022B1  bl 0x82dfe8c8
	ctx.lr = 0x827FC61C;
	sub_82DFE8C8(ctx, base);
	// 827FC61C: 3DC0832C  lis r14, -0x7cd4
	ctx.r[14].s64 = -2094268416;
	// 827FC620: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC624: D3A10698  stfs f29, 0x698(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1688 as u32), tmp.u32 ) };
	// 827FC628: D3C106A0  stfs f30, 0x6a0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1696 as u32), tmp.u32 ) };
	// 827FC62C: 93C1069C  stw r30, 0x69c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1692 as u32), ctx.r[30].u32 ) };
	// 827FC630: 394B6D7C  addi r10, r11, 0x6d7c
	ctx.r[10].s64 = ctx.r[11].s64 + 28028;
	// 827FC634: D3E106A4  stfs f31, 0x6a4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1700 as u32), tmp.u32 ) };
	// 827FC638: D3E106A8  stfs f31, 0x6a8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1704 as u32), tmp.u32 ) };
	// 827FC63C: 9BE106AC  stb r31, 0x6ac(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1708 as u32), ctx.r[31].u8 ) };
	// 827FC640: 91410694  stw r10, 0x694(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1684 as u32), ctx.r[10].u32 ) };
	// 827FC644: 386106B0  addi r3, r1, 0x6b0
	ctx.r[3].s64 = ctx.r[1].s64 + 1712;
	// 827FC648: 816E14F0  lwz r11, 0x14f0(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(5360 as u32) ) } as u64;
	// 827FC64C: 91610690  stw r11, 0x690(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1680 as u32), ctx.r[11].u32 ) };
	// 827FC650: 48602279  bl 0x82dfe8c8
	ctx.lr = 0x827FC654;
	sub_82DFE8C8(ctx, base);
	// 827FC654: 386106B8  addi r3, r1, 0x6b8
	ctx.r[3].s64 = ctx.r[1].s64 + 1720;
	// 827FC658: 48602271  bl 0x82dfe8c8
	ctx.lr = 0x827FC65C;
	sub_82DFE8C8(ctx, base);
	// 827FC65C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 827FC660: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC664: D3A106C8  stfs f29, 0x6c8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1736 as u32), tmp.u32 ) };
	// 827FC668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FC66C: D3C106D0  stfs f30, 0x6d0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1744 as u32), tmp.u32 ) };
	// 827FC670: 394A6D70  addi r10, r10, 0x6d70
	ctx.r[10].s64 = ctx.r[10].s64 + 28016;
	// 827FC674: D3E106D4  stfs f31, 0x6d4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1748 as u32), tmp.u32 ) };
	// 827FC678: D3E106D8  stfs f31, 0x6d8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1752 as u32), tmp.u32 ) };
	// 827FC67C: 93C106CC  stw r30, 0x6cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1740 as u32), ctx.r[30].u32 ) };
	// 827FC680: 9BE106DC  stb r31, 0x6dc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1756 as u32), ctx.r[31].u8 ) };
	// 827FC684: 386106E0  addi r3, r1, 0x6e0
	ctx.r[3].s64 = ctx.r[1].s64 + 1760;
	// 827FC688: 914106C4  stw r10, 0x6c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1732 as u32), ctx.r[10].u32 ) };
	// 827FC68C: 816B14F4  lwz r11, 0x14f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5364 as u32) ) } as u64;
	// 827FC690: 916106C0  stw r11, 0x6c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1728 as u32), ctx.r[11].u32 ) };
	// 827FC694: 48602235  bl 0x82dfe8c8
	ctx.lr = 0x827FC698;
	sub_82DFE8C8(ctx, base);
	// 827FC698: 386106E8  addi r3, r1, 0x6e8
	ctx.r[3].s64 = ctx.r[1].s64 + 1768;
	// 827FC69C: 4860222D  bl 0x82dfe8c8
	ctx.lr = 0x827FC6A0;
	sub_82DFE8C8(ctx, base);
	// 827FC6A0: 3FC0832C  lis r30, -0x7cd4
	ctx.r[30].s64 = -2094268416;
	// 827FC6A4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC6A8: D3A106F8  stfs f29, 0x6f8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1784 as u32), tmp.u32 ) };
	// 827FC6AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FC6B0: D3C10700  stfs f30, 0x700(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1792 as u32), tmp.u32 ) };
	// 827FC6B4: 392B6D60  addi r9, r11, 0x6d60
	ctx.r[9].s64 = ctx.r[11].s64 + 28000;
	// 827FC6B8: D3E10704  stfs f31, 0x704(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1796 as u32), tmp.u32 ) };
	// 827FC6BC: D3E10708  stfs f31, 0x708(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1800 as u32), tmp.u32 ) };
	// 827FC6C0: 914106FC  stw r10, 0x6fc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1788 as u32), ctx.r[10].u32 ) };
	// 827FC6C4: 912106F4  stw r9, 0x6f4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1780 as u32), ctx.r[9].u32 ) };
	// 827FC6C8: 38610710  addi r3, r1, 0x710
	ctx.r[3].s64 = ctx.r[1].s64 + 1808;
	// 827FC6CC: 9BE1070C  stb r31, 0x70c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1804 as u32), ctx.r[31].u8 ) };
	// 827FC6D0: 817E14B0  lwz r11, 0x14b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5296 as u32) ) } as u64;
	// 827FC6D4: 916106F0  stw r11, 0x6f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1776 as u32), ctx.r[11].u32 ) };
	// 827FC6D8: 486021F1  bl 0x82dfe8c8
	ctx.lr = 0x827FC6DC;
	sub_82DFE8C8(ctx, base);
	// 827FC6DC: 38610718  addi r3, r1, 0x718
	ctx.r[3].s64 = ctx.r[1].s64 + 1816;
	// 827FC6E0: 486021E9  bl 0x82dfe8c8
	ctx.lr = 0x827FC6E4;
	sub_82DFE8C8(ctx, base);
	// 827FC6E4: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC6E8: D3A10728  stfs f29, 0x728(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1832 as u32), tmp.u32 ) };
	// 827FC6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FC6F0: D3C10730  stfs f30, 0x730(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1840 as u32), tmp.u32 ) };
	// 827FC6F4: 3D20832D  lis r9, -0x7cd3
	ctx.r[9].s64 = -2094202880;
	// 827FC6F8: D3E10734  stfs f31, 0x734(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1844 as u32), tmp.u32 ) };
	// 827FC6FC: 394A6D54  addi r10, r10, 0x6d54
	ctx.r[10].s64 = ctx.r[10].s64 + 27988;
	// 827FC700: D3E10738  stfs f31, 0x738(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1848 as u32), tmp.u32 ) };
	// 827FC704: 9161072C  stw r11, 0x72c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1836 as u32), ctx.r[11].u32 ) };
	// 827FC708: 38610740  addi r3, r1, 0x740
	ctx.r[3].s64 = ctx.r[1].s64 + 1856;
	// 827FC70C: 9961073C  stb r11, 0x73c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1852 as u32), ctx.r[11].u8 ) };
	// 827FC710: 8169F470  lwz r11, -0xb90(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2960 as u32) ) } as u64;
	// 827FC714: 91410724  stw r10, 0x724(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1828 as u32), ctx.r[10].u32 ) };
	// 827FC718: 91610720  stw r11, 0x720(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1824 as u32), ctx.r[11].u32 ) };
	// 827FC71C: 486021AD  bl 0x82dfe8c8
	ctx.lr = 0x827FC720;
	sub_82DFE8C8(ctx, base);
	// 827FC720: 38610748  addi r3, r1, 0x748
	ctx.r[3].s64 = ctx.r[1].s64 + 1864;
	// 827FC724: 486021A5  bl 0x82dfe8c8
	ctx.lr = 0x827FC728;
	sub_82DFE8C8(ctx, base);
	// 827FC728: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 827FC72C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FC730: D3A10758  stfs f29, 0x758(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1880 as u32), tmp.u32 ) };
	// 827FC734: D3C10760  stfs f30, 0x760(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1888 as u32), tmp.u32 ) };
	// 827FC738: 93A10754  stw r29, 0x754(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1876 as u32), ctx.r[29].u32 ) };
	// 827FC73C: 9161075C  stw r11, 0x75c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1884 as u32), ctx.r[11].u32 ) };
	// 827FC740: D3E10764  stfs f31, 0x764(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1892 as u32), tmp.u32 ) };
	// 827FC744: 9961076C  stb r11, 0x76c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1900 as u32), ctx.r[11].u8 ) };
	// 827FC748: D3E10768  stfs f31, 0x768(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1896 as u32), tmp.u32 ) };
	// 827FC74C: 38610770  addi r3, r1, 0x770
	ctx.r[3].s64 = ctx.r[1].s64 + 1904;
	// 827FC750: 816AF46C  lwz r11, -0xb94(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2964 as u32) ) } as u64;
	// 827FC754: 91610750  stw r11, 0x750(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1872 as u32), ctx.r[11].u32 ) };
	// 827FC758: 48602171  bl 0x82dfe8c8
	ctx.lr = 0x827FC75C;
	sub_82DFE8C8(ctx, base);
	// 827FC75C: 38610778  addi r3, r1, 0x778
	ctx.r[3].s64 = ctx.r[1].s64 + 1912;
	// 827FC760: 48602169  bl 0x82dfe8c8
	ctx.lr = 0x827FC764;
	sub_82DFE8C8(ctx, base);
	// 827FC764: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 827FC768: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FC76C: D3A10788  stfs f29, 0x788(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1928 as u32), tmp.u32 ) };
	// 827FC770: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827FC774: D3C10790  stfs f30, 0x790(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1936 as u32), tmp.u32 ) };
	// 827FC778: D3E10794  stfs f31, 0x794(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1940 as u32), tmp.u32 ) };
	// 827FC77C: 9141078C  stw r10, 0x78c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1932 as u32), ctx.r[10].u32 ) };
	// 827FC780: D3E10798  stfs f31, 0x798(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1944 as u32), tmp.u32 ) };
	// 827FC784: 93A10784  stw r29, 0x784(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1924 as u32), ctx.r[29].u32 ) };
	// 827FC788: 9921079C  stb r9, 0x79c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1948 as u32), ctx.r[9].u8 ) };
	// 827FC78C: 386107A0  addi r3, r1, 0x7a0
	ctx.r[3].s64 = ctx.r[1].s64 + 1952;
	// 827FC790: 816BF474  lwz r11, -0xb8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2956 as u32) ) } as u64;
	// 827FC794: 91610780  stw r11, 0x780(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1920 as u32), ctx.r[11].u32 ) };
	// 827FC798: 48602131  bl 0x82dfe8c8
	ctx.lr = 0x827FC79C;
	sub_82DFE8C8(ctx, base);
	// 827FC79C: 386107A8  addi r3, r1, 0x7a8
	ctx.r[3].s64 = ctx.r[1].s64 + 1960;
	// 827FC7A0: 48602129  bl 0x82dfe8c8
	ctx.lr = 0x827FC7A4;
	sub_82DFE8C8(ctx, base);
	// 827FC7A4: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 827FC7A8: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FC7AC: D3A107B8  stfs f29, 0x7b8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1976 as u32), tmp.u32 ) };
	// 827FC7B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827FC7B4: D3C107C0  stfs f30, 0x7c0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1984 as u32), tmp.u32 ) };
	// 827FC7B8: 394A6D44  addi r10, r10, 0x6d44
	ctx.r[10].s64 = ctx.r[10].s64 + 27972;
	// 827FC7BC: D3E107C4  stfs f31, 0x7c4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1988 as u32), tmp.u32 ) };
	// 827FC7C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827FC7C4: D3E107C8  stfs f31, 0x7c8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1992 as u32), tmp.u32 ) };
	// 827FC7C8: 912107BC  stw r9, 0x7bc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1980 as u32), ctx.r[9].u32 ) };
	// 827FC7CC: 386107D0  addi r3, r1, 0x7d0
	ctx.r[3].s64 = ctx.r[1].s64 + 2000;
	// 827FC7D0: 914107B4  stw r10, 0x7b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1972 as u32), ctx.r[10].u32 ) };
	// 827FC7D4: 990107CC  stb r8, 0x7cc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(1996 as u32), ctx.r[8].u8 ) };
	// 827FC7D8: 816BF468  lwz r11, -0xb98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2968 as u32) ) } as u64;
	// 827FC7DC: 916107B0  stw r11, 0x7b0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1968 as u32), ctx.r[11].u32 ) };
	// 827FC7E0: 486020E9  bl 0x82dfe8c8
	ctx.lr = 0x827FC7E4;
	sub_82DFE8C8(ctx, base);
	// 827FC7E4: 386107D8  addi r3, r1, 0x7d8
	ctx.r[3].s64 = ctx.r[1].s64 + 2008;
	// 827FC7E8: 486020E1  bl 0x82dfe8c8
	ctx.lr = 0x827FC7EC;
	sub_82DFE8C8(ctx, base);
	// 827FC7EC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FC7F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FC7F4: D3A107E8  stfs f29, 0x7e8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2024 as u32), tmp.u32 ) };
	// 827FC7F8: 396B6D30  addi r11, r11, 0x6d30
	ctx.r[11].s64 = ctx.r[11].s64 + 27952;
	// 827FC7FC: D3C107F0  stfs f30, 0x7f0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2032 as u32), tmp.u32 ) };
	// 827FC800: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827FC804: D3E107F4  stfs f31, 0x7f4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2036 as u32), tmp.u32 ) };
	// 827FC808: D3E107F8  stfs f31, 0x7f8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2040 as u32), tmp.u32 ) };
	// 827FC80C: 916107E0  stw r11, 0x7e0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2016 as u32), ctx.r[11].u32 ) };
	// 827FC810: 916107E4  stw r11, 0x7e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2020 as u32), ctx.r[11].u32 ) };
	// 827FC814: 38610800  addi r3, r1, 0x800
	ctx.r[3].s64 = ctx.r[1].s64 + 2048;
	// 827FC818: 914107EC  stw r10, 0x7ec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(2028 as u32), ctx.r[10].u32 ) };
	// 827FC81C: 992107FC  stb r9, 0x7fc(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(2044 as u32), ctx.r[9].u8 ) };
	// 827FC820: 486020A9  bl 0x82dfe8c8
	ctx.lr = 0x827FC824;
	sub_82DFE8C8(ctx, base);
	// 827FC824: 38610808  addi r3, r1, 0x808
	ctx.r[3].s64 = ctx.r[1].s64 + 2056;
	// 827FC828: 486020A1  bl 0x82dfe8c8
	ctx.lr = 0x827FC82C;
	sub_82DFE8C8(ctx, base);
	// 827FC82C: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC830: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 827FC834: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 827FC838: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC83C: 4BFEDE85  bl 0x827ea6c0
	ctx.lr = 0x827FC840;
	sub_827EA6C0(ctx, base);
	// 827FC840: 80610954  lwz r3, 0x954(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC844: 4BFFCB0D  bl 0x827f9350
	ctx.lr = 0x827FC848;
	sub_827F9350(ctx, base);
	// 827FC848: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC84C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FC850: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC854: 4BFEDE75  bl 0x827ea6c8
	ctx.lr = 0x827FC858;
	sub_827EA6C8(ctx, base);
	// 827FC858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC85C: 80921508  lwz r4, 0x1508(r18)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(5384 as u32) ) } as u64;
	// 827FC860: 485F71A9  bl 0x82df3a08
	ctx.lr = 0x827FC864;
	sub_82DF3A08(ctx, base);
	// 827FC864: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FC868: 81410954  lwz r10, 0x954(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC86C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC870: C3AB964C  lfs f29, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827FC874: 806A0120  lwz r3, 0x120(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC878: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC87C: 4BFEDABD  bl 0x827ea338
	ctx.lr = 0x827FC880;
	sub_827EA338(ctx, base);
	// 827FC880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC884: 485F6BA5  bl 0x82df3428
	ctx.lr = 0x827FC888;
	sub_82DF3428(ctx, base);
	// 827FC888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC88C: 808F14EC  lwz r4, 0x14ec(r15)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(5356 as u32) ) } as u64;
	// 827FC890: 485F7179  bl 0x82df3a08
	ctx.lr = 0x827FC894;
	sub_82DF3A08(ctx, base);
	// 827FC894: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC898: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC89C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC8A0: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC8A4: 4BFEDA95  bl 0x827ea338
	ctx.lr = 0x827FC8A8;
	sub_827EA338(ctx, base);
	// 827FC8A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC8AC: 485F6B7D  bl 0x82df3428
	ctx.lr = 0x827FC8B0;
	sub_82DF3428(ctx, base);
	// 827FC8B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC8B4: 808E14F0  lwz r4, 0x14f0(r14)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(5360 as u32) ) } as u64;
	// 827FC8B8: 485F7151  bl 0x82df3a08
	ctx.lr = 0x827FC8BC;
	sub_82DF3A08(ctx, base);
	// 827FC8BC: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC8C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC8C4: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC8C8: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC8CC: 4BFEDA6D  bl 0x827ea338
	ctx.lr = 0x827FC8D0;
	sub_827EA338(ctx, base);
	// 827FC8D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC8D4: 485F6B55  bl 0x82df3428
	ctx.lr = 0x827FC8D8;
	sub_82DF3428(ctx, base);
	// 827FC8D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC8DC: 809E14B0  lwz r4, 0x14b0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5296 as u32) ) } as u64;
	// 827FC8E0: 485F7129  bl 0x82df3a08
	ctx.lr = 0x827FC8E4;
	sub_82DF3A08(ctx, base);
	// 827FC8E4: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC8E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC8EC: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC8F0: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC8F4: 4BFEDA45  bl 0x827ea338
	ctx.lr = 0x827FC8F8;
	sub_827EA338(ctx, base);
	// 827FC8F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC8FC: 485F6B2D  bl 0x82df3428
	ctx.lr = 0x827FC900;
	sub_82DF3428(ctx, base);
	// 827FC900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC904: 809414B4  lwz r4, 0x14b4(r20)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5300 as u32) ) } as u64;
	// 827FC908: 485F7101  bl 0x82df3a08
	ctx.lr = 0x827FC90C;
	sub_82DF3A08(ctx, base);
	// 827FC90C: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC910: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC914: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC918: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC91C: 4BFEDA1D  bl 0x827ea338
	ctx.lr = 0x827FC920;
	sub_827EA338(ctx, base);
	// 827FC920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC924: 485F6B05  bl 0x82df3428
	ctx.lr = 0x827FC928;
	sub_82DF3428(ctx, base);
	// 827FC928: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 827FC92C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC930: 808B1478  lwz r4, 0x1478(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5240 as u32) ) } as u64;
	// 827FC934: 485F70D5  bl 0x82df3a08
	ctx.lr = 0x827FC938;
	sub_82DF3A08(ctx, base);
	// 827FC938: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC93C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC940: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC944: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC948: 4BFED9F1  bl 0x827ea338
	ctx.lr = 0x827FC94C;
	sub_827EA338(ctx, base);
	// 827FC94C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC950: 485F6AD9  bl 0x82df3428
	ctx.lr = 0x827FC954;
	sub_82DF3428(ctx, base);
	// 827FC954: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827FC958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC95C: 808B1494  lwz r4, 0x1494(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5268 as u32) ) } as u64;
	// 827FC960: 485F70A9  bl 0x82df3a08
	ctx.lr = 0x827FC964;
	sub_82DF3A08(ctx, base);
	// 827FC964: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC968: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC96C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC970: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC974: 4BFED9C5  bl 0x827ea338
	ctx.lr = 0x827FC978;
	sub_827EA338(ctx, base);
	// 827FC978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC97C: 485F6AAD  bl 0x82df3428
	ctx.lr = 0x827FC980;
	sub_82DF3428(ctx, base);
	// 827FC980: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 827FC984: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC988: 808B1490  lwz r4, 0x1490(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5264 as u32) ) } as u64;
	// 827FC98C: 485F707D  bl 0x82df3a08
	ctx.lr = 0x827FC990;
	sub_82DF3A08(ctx, base);
	// 827FC990: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC994: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC998: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC99C: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC9A0: 4BFED999  bl 0x827ea338
	ctx.lr = 0x827FC9A4;
	sub_827EA338(ctx, base);
	// 827FC9A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC9A8: 485F6A81  bl 0x82df3428
	ctx.lr = 0x827FC9AC;
	sub_82DF3428(ctx, base);
	// 827FC9AC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 827FC9B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC9B4: 808B147C  lwz r4, 0x147c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5244 as u32) ) } as u64;
	// 827FC9B8: 485F7051  bl 0x82df3a08
	ctx.lr = 0x827FC9BC;
	sub_82DF3A08(ctx, base);
	// 827FC9BC: 81610954  lwz r11, 0x954(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC9C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC9C4: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC9C8: 806B0120  lwz r3, 0x120(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC9CC: 4BFED96D  bl 0x827ea338
	ctx.lr = 0x827FC9D0;
	sub_827EA338(ctx, base);
	// 827FC9D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC9D4: 485F6A55  bl 0x82df3428
	ctx.lr = 0x827FC9D8;
	sub_82DF3428(ctx, base);
	// 827FC9D8: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 827FC9DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FC9E0: 808B1480  lwz r4, 0x1480(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5248 as u32) ) } as u64;
	// 827FC9E4: 485F7025  bl 0x82df3a08
	ctx.lr = 0x827FC9E8;
	sub_82DF3A08(ctx, base);
	// 827FC9E8: 83E10954  lwz r31, 0x954(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2388 as u32) ) } as u64;
	// 827FC9EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FC9F0: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FC9F4: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FC9F8: 4BFED941  bl 0x827ea338
	ctx.lr = 0x827FC9FC;
	sub_827EA338(ctx, base);
	// 827FC9FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA00: 485F6A29  bl 0x82df3428
	ctx.lr = 0x827FCA04;
	sub_82DF3428(ctx, base);
	// 827FCA04: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 827FCA08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA0C: 808B1484  lwz r4, 0x1484(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5252 as u32) ) } as u64;
	// 827FCA10: 485F6FF9  bl 0x82df3a08
	ctx.lr = 0x827FCA14;
	sub_82DF3A08(ctx, base);
	// 827FCA14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCA18: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCA1C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCA20: 4BFED919  bl 0x827ea338
	ctx.lr = 0x827FCA24;
	sub_827EA338(ctx, base);
	// 827FCA24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA28: 485F6A01  bl 0x82df3428
	ctx.lr = 0x827FCA2C;
	sub_82DF3428(ctx, base);
	// 827FCA2C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827FCA30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA34: 808B1488  lwz r4, 0x1488(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5256 as u32) ) } as u64;
	// 827FCA38: 485F6FD1  bl 0x82df3a08
	ctx.lr = 0x827FCA3C;
	sub_82DF3A08(ctx, base);
	// 827FCA3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCA40: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCA44: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCA48: 4BFED8F1  bl 0x827ea338
	ctx.lr = 0x827FCA4C;
	sub_827EA338(ctx, base);
	// 827FCA4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA50: 485F69D9  bl 0x82df3428
	ctx.lr = 0x827FCA54;
	sub_82DF3428(ctx, base);
	// 827FCA54: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827FCA58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA5C: 808B148C  lwz r4, 0x148c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5260 as u32) ) } as u64;
	// 827FCA60: 485F6FA9  bl 0x82df3a08
	ctx.lr = 0x827FCA64;
	sub_82DF3A08(ctx, base);
	// 827FCA64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCA68: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCA6C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCA70: 4BFED8C9  bl 0x827ea338
	ctx.lr = 0x827FCA74;
	sub_827EA338(ctx, base);
	// 827FCA74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA78: 485F69B1  bl 0x82df3428
	ctx.lr = 0x827FCA7C;
	sub_82DF3428(ctx, base);
	// 827FCA7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA80: 809514E4  lwz r4, 0x14e4(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(5348 as u32) ) } as u64;
	// 827FCA84: 485F6F85  bl 0x82df3a08
	ctx.lr = 0x827FCA88;
	sub_82DF3A08(ctx, base);
	// 827FCA88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCA8C: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCA90: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCA94: 4BFED8A5  bl 0x827ea338
	ctx.lr = 0x827FCA98;
	sub_827EA338(ctx, base);
	// 827FCA98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCA9C: 485F698D  bl 0x82df3428
	ctx.lr = 0x827FCAA0;
	sub_82DF3428(ctx, base);
	// 827FCAA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCAA4: 809C14D8  lwz r4, 0x14d8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(5336 as u32) ) } as u64;
	// 827FCAA8: 485F6F61  bl 0x82df3a08
	ctx.lr = 0x827FCAAC;
	sub_82DF3A08(ctx, base);
	// 827FCAAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCAB0: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCAB4: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCAB8: 4BFED881  bl 0x827ea338
	ctx.lr = 0x827FCABC;
	sub_827EA338(ctx, base);
	// 827FCABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCAC0: 485F6969  bl 0x82df3428
	ctx.lr = 0x827FCAC4;
	sub_82DF3428(ctx, base);
	// 827FCAC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCAC8: 809714DC  lwz r4, 0x14dc(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(5340 as u32) ) } as u64;
	// 827FCACC: 485F6F3D  bl 0x82df3a08
	ctx.lr = 0x827FCAD0;
	sub_82DF3A08(ctx, base);
	// 827FCAD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCAD4: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCAD8: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCADC: 4BFED85D  bl 0x827ea338
	ctx.lr = 0x827FCAE0;
	sub_827EA338(ctx, base);
	// 827FCAE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCAE4: 485F6945  bl 0x82df3428
	ctx.lr = 0x827FCAE8;
	sub_82DF3428(ctx, base);
	// 827FCAE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCAEC: 809614E0  lwz r4, 0x14e0(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5344 as u32) ) } as u64;
	// 827FCAF0: 485F6F19  bl 0x82df3a08
	ctx.lr = 0x827FCAF4;
	sub_82DF3A08(ctx, base);
	// 827FCAF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCAF8: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCAFC: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCB00: 4BFED839  bl 0x827ea338
	ctx.lr = 0x827FCB04;
	sub_827EA338(ctx, base);
	// 827FCB04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB08: 485F6921  bl 0x82df3428
	ctx.lr = 0x827FCB0C;
	sub_82DF3428(ctx, base);
	// 827FCB0C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FCB10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB14: 808B14C4  lwz r4, 0x14c4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5316 as u32) ) } as u64;
	// 827FCB18: 485F6EF1  bl 0x82df3a08
	ctx.lr = 0x827FCB1C;
	sub_82DF3A08(ctx, base);
	// 827FCB1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCB20: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCB24: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCB28: 4BFED811  bl 0x827ea338
	ctx.lr = 0x827FCB2C;
	sub_827EA338(ctx, base);
	// 827FCB2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB30: 485F68F9  bl 0x82df3428
	ctx.lr = 0x827FCB34;
	sub_82DF3428(ctx, base);
	// 827FCB34: 81610090  lwz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 827FCB38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB3C: 808B14C0  lwz r4, 0x14c0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5312 as u32) ) } as u64;
	// 827FCB40: 485F6EC9  bl 0x82df3a08
	ctx.lr = 0x827FCB44;
	sub_82DF3A08(ctx, base);
	// 827FCB44: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCB48: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCB4C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCB50: 4BFED7E9  bl 0x827ea338
	ctx.lr = 0x827FCB54;
	sub_827EA338(ctx, base);
	// 827FCB54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB58: 485F68D1  bl 0x82df3428
	ctx.lr = 0x827FCB5C;
	sub_82DF3428(ctx, base);
	// 827FCB5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB60: 809B14CC  lwz r4, 0x14cc(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(5324 as u32) ) } as u64;
	// 827FCB64: 485F6EA5  bl 0x82df3a08
	ctx.lr = 0x827FCB68;
	sub_82DF3A08(ctx, base);
	// 827FCB68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCB6C: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCB70: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCB74: 4BFED7C5  bl 0x827ea338
	ctx.lr = 0x827FCB78;
	sub_827EA338(ctx, base);
	// 827FCB78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB7C: 485F68AD  bl 0x82df3428
	ctx.lr = 0x827FCB80;
	sub_82DF3428(ctx, base);
	// 827FCB80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCB84: 809A14D4  lwz r4, 0x14d4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(5332 as u32) ) } as u64;
	// 827FCB88: 485F6E81  bl 0x82df3a08
	ctx.lr = 0x827FCB8C;
	sub_82DF3A08(ctx, base);
	// 827FCB8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCB90: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCB94: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCB98: 4BFED7A1  bl 0x827ea338
	ctx.lr = 0x827FCB9C;
	sub_827EA338(ctx, base);
	// 827FCB9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCBA0: 485F6889  bl 0x82df3428
	ctx.lr = 0x827FCBA4;
	sub_82DF3428(ctx, base);
	// 827FCBA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCBA8: 809914C8  lwz r4, 0x14c8(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(5320 as u32) ) } as u64;
	// 827FCBAC: 485F6E5D  bl 0x82df3a08
	ctx.lr = 0x827FCBB0;
	sub_82DF3A08(ctx, base);
	// 827FCBB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCBB4: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCBB8: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCBBC: 4BFED77D  bl 0x827ea338
	ctx.lr = 0x827FCBC0;
	sub_827EA338(ctx, base);
	// 827FCBC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCBC4: 485F6865  bl 0x82df3428
	ctx.lr = 0x827FCBC8;
	sub_82DF3428(ctx, base);
	// 827FCBC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCBCC: 809814D0  lwz r4, 0x14d0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(5328 as u32) ) } as u64;
	// 827FCBD0: 485F6E39  bl 0x82df3a08
	ctx.lr = 0x827FCBD4;
	sub_82DF3A08(ctx, base);
	// 827FCBD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCBD8: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCBDC: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCBE0: 4BFED759  bl 0x827ea338
	ctx.lr = 0x827FCBE4;
	sub_827EA338(ctx, base);
	// 827FCBE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCBE8: 485F6841  bl 0x82df3428
	ctx.lr = 0x827FCBEC;
	sub_82DF3428(ctx, base);
	// 827FCBEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCBF0: 809114A8  lwz r4, 0x14a8(r17)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(5288 as u32) ) } as u64;
	// 827FCBF4: 485F6E15  bl 0x82df3a08
	ctx.lr = 0x827FCBF8;
	sub_82DF3A08(ctx, base);
	// 827FCBF8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 827FCBFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCC00: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCC04: C3AB6150  lfs f29, 0x6150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827FCC08: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCC0C: 4BFED72D  bl 0x827ea338
	ctx.lr = 0x827FCC10;
	sub_827EA338(ctx, base);
	// 827FCC10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC14: 485F6815  bl 0x82df3428
	ctx.lr = 0x827FCC18;
	sub_82DF3428(ctx, base);
	// 827FCC18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC1C: 809014AC  lwz r4, 0x14ac(r16)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(5292 as u32) ) } as u64;
	// 827FCC20: 485F6DE9  bl 0x82df3a08
	ctx.lr = 0x827FCC24;
	sub_82DF3A08(ctx, base);
	// 827FCC24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCC28: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCC2C: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCC30: 4BFED709  bl 0x827ea338
	ctx.lr = 0x827FCC34;
	sub_827EA338(ctx, base);
	// 827FCC34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC38: 485F67F1  bl 0x82df3428
	ctx.lr = 0x827FCC3C;
	sub_82DF3428(ctx, base);
	// 827FCC3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC40: 8093149C  lwz r4, 0x149c(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(5276 as u32) ) } as u64;
	// 827FCC44: 485F6DC5  bl 0x82df3a08
	ctx.lr = 0x827FCC48;
	sub_82DF3A08(ctx, base);
	// 827FCC48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCC4C: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCC50: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827FCC54: 4BFED6E5  bl 0x827ea338
	ctx.lr = 0x827FCC58;
	sub_827EA338(ctx, base);
	// 827FCC58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC5C: 485F67CD  bl 0x82df3428
	ctx.lr = 0x827FCC60;
	sub_82DF3428(ctx, base);
	// 827FCC60: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FCC64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC68: 809D14F4  lwz r4, 0x14f4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(5364 as u32) ) } as u64;
	// 827FCC6C: 485F6D9D  bl 0x82df3a08
	ctx.lr = 0x827FCC70;
	sub_82DF3A08(ctx, base);
	// 827FCC70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827FCC74: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FCC78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827FCC7C: C02B7BC8  lfs f1, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FCC80: 4BFED6B9  bl 0x827ea338
	ctx.lr = 0x827FCC84;
	sub_827EA338(ctx, base);
	// 827FCC84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCC88: 485F67A1  bl 0x82df3428
	ctx.lr = 0x827FCC8C;
	sub_82DF3428(ctx, base);
	// 827FCC8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FCC90: 4BD15E09  bl 0x82512a98
	ctx.lr = 0x827FCC94;
	sub_82512A98(ctx, base);
	// 827FCC94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FCC98: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 827FCC9C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 827FCCA0: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 827FCCA4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 827FCCA8: 486A3A89  bl 0x82ea0730
	ctx.lr = 0x827FCCAC;
	sub_82EA0730(ctx, base);
	// 827FCCAC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FCCB0: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827FCCB4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827FCCB8: 392B6D20  addi r9, r11, 0x6d20
	ctx.r[9].s64 = ctx.r[11].s64 + 27936;
	// 827FCCBC: 394ABC40  addi r10, r10, -0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + -17344;
	// 827FCCC0: 39010810  addi r8, r1, 0x810
	ctx.r[8].s64 = ctx.r[1].s64 + 2064;
	// 827FCCC4: 38E10820  addi r7, r1, 0x820
	ctx.r[7].s64 = ctx.r[1].s64 + 2080;
	// 827FCCC8: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 827FCCCC: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FCCD0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FCCD4: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FCCD8: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FCED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FCED0 size=136
    let mut pc: u32 = 0x827FCED0;
    'dispatch: loop {
        match pc {
            0x827FCED0 => {
    //   block [0x827FCED0..0x827FCF58)
	// 827FCED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FCED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FCED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FCEDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FCEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FCEE4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827FCEE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FCEEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FCEF0: 808B6750  lwz r4, 0x6750(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26448 as u32) ) } as u64;
	// 827FCEF4: 4BAE803D  bl 0x822e4f30
	ctx.lr = 0x827FCEF8;
	sub_822E4F30(ctx, base);
	// 827FCEF8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827FCEFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FCF00: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FCF04: 808B6754  lwz r4, 0x6754(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26452 as u32) ) } as u64;
	// 827FCF08: 4BAE8029  bl 0x822e4f30
	ctx.lr = 0x827FCF0C;
	sub_822E4F30(ctx, base);
	// 827FCF0C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827FCF10: E89E0000  ld r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827FCF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FCF18: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827FCF1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827FCF20: 4BC8F369  bl 0x8248c288
	ctx.lr = 0x827FCF24;
	sub_8248C288(ctx, base);
	// 827FCF24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FCF28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FCF2C: 4BC92475  bl 0x8248f3a0
	ctx.lr = 0x827FCF30;
	sub_8248F3A0(ctx, base);
	// 827FCF30: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827FCF34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FCF38: C02B66D4  lfs f1, 0x66d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26324 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FCF3C: 4BAF04D5  bl 0x822ed410
	ctx.lr = 0x827FCF40;
	sub_822ED410(ctx, base);
	// 827FCF40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FCF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FCF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FCF4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FCF50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FCF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FCF58 size=16
    let mut pc: u32 = 0x827FCF58;
    'dispatch: loop {
        match pc {
            0x827FCF58 => {
    //   block [0x827FCF58..0x827FCF68)
	// 827FCF58: 89640018  lbz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 827FCF5C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827FCF60: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827FCF64: 4BD12BDC  b 0x8250fb40
	sub_8250FB40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FCF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FCF68 size=144
    let mut pc: u32 = 0x827FCF68;
    'dispatch: loop {
        match pc {
            0x827FCF68 => {
    //   block [0x827FCF68..0x827FCFF8)
	// 827FCF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FCF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FCF70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FCF74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FCF78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FCF7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FCF80: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827FCF84: 4BD14A55  bl 0x825119d8
	ctx.lr = 0x827FCF88;
	sub_825119D8(ctx, base);
	// 827FCF88: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FCF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FCF90: 388B6EE0  addi r4, r11, 0x6ee0
	ctx.r[4].s64 = ctx.r[11].s64 + 28384;
	// 827FCF94: 38A00045  li r5, 0x45
	ctx.r[5].s64 = 69;
	// 827FCF98: 386000A8  li r3, 0xa8
	ctx.r[3].s64 = 168;
	// 827FCF9C: 485F544D  bl 0x82df23e8
	ctx.lr = 0x827FCFA0;
	sub_82DF23E8(ctx, base);
	// 827FCFA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FCFA4: 41820010  beq 0x827fcfb4
	if ctx.cr[0].eq {
	pc = 0x827FCFB4; continue 'dispatch;
	}
	// 827FCFA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FCFAC: 4BFD35A5  bl 0x827d0550
	ctx.lr = 0x827FCFB0;
	sub_827D0550(ctx, base);
	// 827FCFB0: 48000008  b 0x827fcfb8
	pc = 0x827FCFB8; continue 'dispatch;
	// 827FCFB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827FCFB8: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 827FCFBC: 907E00E4  stw r3, 0xe4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[3].u32 ) };
	// 827FCFC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FCFC4: 419A001C  beq cr6, 0x827fcfe0
	if ctx.cr[6].eq {
	pc = 0x827FCFE0; continue 'dispatch;
	}
	// 827FCFC8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FCFCC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827FCFD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FCFD4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FCFD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FCFDC: 4E800421  bctrl
	ctx.lr = 0x827FCFE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FCFE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FCFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FCFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FCFEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FCFF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FCFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FCFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FCFF8 size=100
    let mut pc: u32 = 0x827FCFF8;
    'dispatch: loop {
        match pc {
            0x827FCFF8 => {
    //   block [0x827FCFF8..0x827FD05C)
	// 827FCFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FCFFC: 489AB171  bl 0x831a816c
	ctx.lr = 0x827FD000;
	sub_831A8130(ctx, base);
	// 827FD000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD004: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FD008: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FD00C: 3BFD00E4  addi r31, r29, 0xe4
	ctx.r[31].s64 = ctx.r[29].s64 + 228;
	// 827FD010: 807D00E4  lwz r3, 0xe4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(228 as u32) ) } as u64;
	// 827FD014: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD018: 419A0030  beq cr6, 0x827fd048
	if ctx.cr[6].eq {
	pc = 0x827FD048; continue 'dispatch;
	}
	// 827FD01C: 4BFD2E25  bl 0x827cfe40
	ctx.lr = 0x827FD020;
	sub_827CFE40(ctx, base);
	// 827FD020: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD028: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD02C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD030: 419A0018  beq cr6, 0x827fd048
	if ctx.cr[6].eq {
	pc = 0x827FD048; continue 'dispatch;
	}
	// 827FD034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD038: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FD03C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FD044: 4E800421  bctrl
	ctx.lr = 0x827FD048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FD048: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FD04C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FD050: 4BD15019  bl 0x82512068
	ctx.lr = 0x827FD054;
	sub_82512068(ctx, base);
	// 827FD054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD058: 489AB164  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD060 size=80
    let mut pc: u32 = 0x827FD060;
    'dispatch: loop {
        match pc {
            0x827FD060 => {
    //   block [0x827FD060..0x827FD0B0)
	// 827FD060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD06C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD074: 4BD151ED  bl 0x82512260
	ctx.lr = 0x827FD078;
	sub_82512260(ctx, base);
	// 827FD078: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD07C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD080: 396B6F3C  addi r11, r11, 0x6f3c
	ctx.r[11].s64 = ctx.r[11].s64 + 28476;
	// 827FD084: 394A6F28  addi r10, r10, 0x6f28
	ctx.r[10].s64 = ctx.r[10].s64 + 28456;
	// 827FD088: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827FD08C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD090: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FD094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD098: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 827FD09C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FD0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD0A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FD0B0 size=8
    let mut pc: u32 = 0x827FD0B0;
    'dispatch: loop {
        match pc {
            0x827FD0B0 => {
    //   block [0x827FD0B0..0x827FD0B8)
	// 827FD0B0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827FD0B4: 480000BC  b 0x827fd170
	sub_827FD170(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD0B8 size=80
    let mut pc: u32 = 0x827FD0B8;
    'dispatch: loop {
        match pc {
            0x827FD0B8 => {
    //   block [0x827FD0B8..0x827FD108)
	// 827FD0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD0C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD0C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD0C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD0CC: 4BD15265  bl 0x82512330
	ctx.lr = 0x827FD0D0;
	sub_82512330(ctx, base);
	// 827FD0D0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD0D4: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD0D8: 396B6F3C  addi r11, r11, 0x6f3c
	ctx.r[11].s64 = ctx.r[11].s64 + 28476;
	// 827FD0DC: 394A6F28  addi r10, r10, 0x6f28
	ctx.r[10].s64 = ctx.r[10].s64 + 28456;
	// 827FD0E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827FD0E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD0E8: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FD0EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD0F0: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 827FD0F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FD0F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD0FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD100: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD108 size=104
    let mut pc: u32 = 0x827FD108;
    'dispatch: loop {
        match pc {
            0x827FD108 => {
    //   block [0x827FD108..0x827FD170)
	// 827FD108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD11C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD120: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD124: 396B6F3C  addi r11, r11, 0x6f3c
	ctx.r[11].s64 = ctx.r[11].s64 + 28476;
	// 827FD128: 394A6F28  addi r10, r10, 0x6f28
	ctx.r[10].s64 = ctx.r[10].s64 + 28456;
	// 827FD12C: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 827FD130: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD138: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FD13C: 419A0018  beq cr6, 0x827fd154
	if ctx.cr[6].eq {
	pc = 0x827FD154; continue 'dispatch;
	}
	// 827FD140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD144: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FD148: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD14C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FD150: 4E800421  bctrl
	ctx.lr = 0x827FD154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FD154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD158: 4BB521F9  bl 0x8234f350
	ctx.lr = 0x827FD15C;
	sub_8234F350(ctx, base);
	// 827FD15C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FD160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD170 size=76
    let mut pc: u32 = 0x827FD170;
    'dispatch: loop {
        match pc {
            0x827FD170 => {
    //   block [0x827FD170..0x827FD1BC)
	// 827FD170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD17C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FD18C: 4BFFFF7D  bl 0x827fd108
	ctx.lr = 0x827FD190;
	sub_827FD108(ctx, base);
	// 827FD190: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD194: 4182000C  beq 0x827fd1a0
	if ctx.cr[0].eq {
	pc = 0x827FD1A0; continue 'dispatch;
	}
	// 827FD198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD19C: 485F523D  bl 0x82df23d8
	ctx.lr = 0x827FD1A0;
	sub_82DF23D8(ctx, base);
	// 827FD1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD1A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD1B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD1B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FD1C0 size=196
    let mut pc: u32 = 0x827FD1C0;
    'dispatch: loop {
        match pc {
            0x827FD1C0 => {
    //   block [0x827FD1C0..0x827FD284)
	// 827FD1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD1C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD1CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD1D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD1D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FD1DC: 4BDDAE75  bl 0x825d8050
	ctx.lr = 0x827FD1E0;
	sub_825D8050(ctx, base);
	// 827FD1E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD1E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827FD1E8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827FD1EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FD1F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FD1F4: 419A0024  beq cr6, 0x827fd218
	if ctx.cr[6].eq {
	pc = 0x827FD218; continue 'dispatch;
	}
	// 827FD1F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827FD1FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827FD200: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FD204: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827FD208: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827FD20C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827FD210: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FD214: 4082FFE8  bne 0x827fd1fc
	if !ctx.cr[0].eq {
	pc = 0x827FD1FC; continue 'dispatch;
	}
	// 827FD218: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 827FD21C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 827FD220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD224: 4880BD95  bl 0x83008fb8
	ctx.lr = 0x827FD228;
	sub_83008FB8(ctx, base);
	// 827FD228: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FD22C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD230: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827FD234: 388A6EE0  addi r4, r10, 0x6ee0
	ctx.r[4].s64 = ctx.r[10].s64 + 28384;
	// 827FD238: 38A00061  li r5, 0x61
	ctx.r[5].s64 = 97;
	// 827FD23C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD240: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FD244: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 827FD248: 4865B7F9  bl 0x82e58a40
	ctx.lr = 0x827FD24C;
	sub_82E58A40(ctx, base);
	// 827FD24C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FD250: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD254: 419A0008  beq cr6, 0x827fd25c
	if ctx.cr[6].eq {
	pc = 0x827FD25C; continue 'dispatch;
	}
	// 827FD258: 4BAC3639  bl 0x822c0890
	ctx.lr = 0x827FD25C;
	sub_822C0890(ctx, base);
	// 827FD25C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FD260: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD264: 419A0008  beq cr6, 0x827fd26c
	if ctx.cr[6].eq {
	pc = 0x827FD26C; continue 'dispatch;
	}
	// 827FD268: 4BAC3629  bl 0x822c0890
	ctx.lr = 0x827FD26C;
	sub_822C0890(ctx, base);
	// 827FD26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FD270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD278: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD27C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD288 size=164
    let mut pc: u32 = 0x827FD288;
    'dispatch: loop {
        match pc {
            0x827FD288 => {
    //   block [0x827FD288..0x827FD32C)
	// 827FD288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD28C: 489AAEDD  bl 0x831a8168
	ctx.lr = 0x827FD290;
	sub_831A8130(ctx, base);
	// 827FD290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD294: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827FD298: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FD29C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD2A0: 57BC063F  clrlwi. r28, r29, 0x18
	ctx.r[28].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 827FD2A4: 41820038  beq 0x827fd2dc
	if ctx.cr[0].eq {
	pc = 0x827FD2DC; continue 'dispatch;
	}
	// 827FD2A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD2AC: 489AC6DD  bl 0x831a9988
	ctx.lr = 0x827FD2B0;
	sub_831A9988(ctx, base);
	// 827FD2B0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 827FD2B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FD2B8: 386B5A88  addi r3, r11, 0x5a88
	ctx.r[3].s64 = ctx.r[11].s64 + 23176;
	// 827FD2BC: 489AAE3D  bl 0x831a80f8
	ctx.lr = 0x827FD2C0;
	sub_831A80F8(ctx, base);
	// 827FD2C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD2C4: 41820018  beq 0x827fd2dc
	if ctx.cr[0].eq {
	pc = 0x827FD2DC; continue 'dispatch;
	}
	// 827FD2C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FD2CC: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FD2D0: 4BFFFEF1  bl 0x827fd1c0
	ctx.lr = 0x827FD2D4;
	sub_827FD1C0(ctx, base);
	// 827FD2D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827FD2D8: 4800004C  b 0x827fd324
	pc = 0x827FD324; continue 'dispatch;
	// 827FD2DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827FD2E0: 419A0034  beq cr6, 0x827fd314
	if ctx.cr[6].eq {
	pc = 0x827FD314; continue 'dispatch;
	}
	// 827FD2E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD2E8: 489AC6A1  bl 0x831a9988
	ctx.lr = 0x827FD2EC;
	sub_831A9988(ctx, base);
	// 827FD2EC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 827FD2F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FD2F4: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 827FD2F8: 489AAE01  bl 0x831a80f8
	ctx.lr = 0x827FD2FC;
	sub_831A80F8(ctx, base);
	// 827FD2FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD300: 41820014  beq 0x827fd314
	if ctx.cr[0].eq {
	pc = 0x827FD314; continue 'dispatch;
	}
	// 827FD304: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FD308: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 827FD30C: 4BFFFC4D  bl 0x827fcf58
	ctx.lr = 0x827FD310;
	sub_827FCF58(ctx, base);
	// 827FD310: 4BFFFFC4  b 0x827fd2d4
	pc = 0x827FD2D4; continue 'dispatch;
	// 827FD314: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827FD318: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FD31C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD320: 4BD152F9  bl 0x82512618
	ctx.lr = 0x827FD324;
	sub_82512618(ctx, base);
	// 827FD324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FD328: 489AAE90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FD330 size=12
    let mut pc: u32 = 0x827FD330;
    'dispatch: loop {
        match pc {
            0x827FD330 => {
    //   block [0x827FD330..0x827FD33C)
	// 827FD330: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD334: 386B6F88  addi r3, r11, 0x6f88
	ctx.r[3].s64 = ctx.r[11].s64 + 28552;
	// 827FD338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD340 size=104
    let mut pc: u32 = 0x827FD340;
    'dispatch: loop {
        match pc {
            0x827FD340 => {
    //   block [0x827FD340..0x827FD3A8)
	// 827FD340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD344: 489AAE25  bl 0x831a8168
	ctx.lr = 0x827FD348;
	sub_831A8130(ctx, base);
	// 827FD348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD34C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD350: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FD354: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827FD358: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 827FD35C: 4BFFC8BD  bl 0x827f9c18
	ctx.lr = 0x827FD360;
	sub_827F9C18(ctx, base);
	// 827FD360: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD364: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD368: 396B6FAC  addi r11, r11, 0x6fac
	ctx.r[11].s64 = ctx.r[11].s64 + 28588;
	// 827FD36C: 394A6F94  addi r10, r10, 0x6f94
	ctx.r[10].s64 = ctx.r[10].s64 + 28564;
	// 827FD370: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD374: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 827FD378: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FD37C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FD380: 485F6689  bl 0x82df3a08
	ctx.lr = 0x827FD384;
	sub_82DF3A08(ctx, base);
	// 827FD384: 387F0124  addi r3, r31, 0x124
	ctx.r[3].s64 = ctx.r[31].s64 + 292;
	// 827FD388: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FD38C: 485F667D  bl 0x82df3a08
	ctx.lr = 0x827FD390;
	sub_82DF3A08(ctx, base);
	// 827FD390: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 827FD394: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827FD398: 485F6671  bl 0x82df3a08
	ctx.lr = 0x827FD39C;
	sub_82DF3A08(ctx, base);
	// 827FD39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FD3A4: 489AAE14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD3A8 size=96
    let mut pc: u32 = 0x827FD3A8;
    'dispatch: loop {
        match pc {
            0x827FD3A8 => {
    //   block [0x827FD3A8..0x827FD408)
	// 827FD3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD3B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD3B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD3B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD3BC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD3C0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD3C4: 396B6FAC  addi r11, r11, 0x6fac
	ctx.r[11].s64 = ctx.r[11].s64 + 28588;
	// 827FD3C8: 394A6F94  addi r10, r10, 0x6f94
	ctx.r[10].s64 = ctx.r[10].s64 + 28564;
	// 827FD3CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD3D0: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 827FD3D4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FD3D8: 485F6051  bl 0x82df3428
	ctx.lr = 0x827FD3DC;
	sub_82DF3428(ctx, base);
	// 827FD3DC: 387F0124  addi r3, r31, 0x124
	ctx.r[3].s64 = ctx.r[31].s64 + 292;
	// 827FD3E0: 485F6049  bl 0x82df3428
	ctx.lr = 0x827FD3E4;
	sub_82DF3428(ctx, base);
	// 827FD3E4: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 827FD3E8: 485F6041  bl 0x82df3428
	ctx.lr = 0x827FD3EC;
	sub_82DF3428(ctx, base);
	// 827FD3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD3F0: 4BFFC769  bl 0x827f9b58
	ctx.lr = 0x827FD3F4;
	sub_827F9B58(ctx, base);
	// 827FD3F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FD3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FD408 size=4
    let mut pc: u32 = 0x827FD408;
    'dispatch: loop {
        match pc {
            0x827FD408 => {
    //   block [0x827FD408..0x827FD40C)
	// 827FD408: 4BFFC128  b 0x827f9530
	sub_827F9530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD410 size=56
    let mut pc: u32 = 0x827FD410;
    'dispatch: loop {
        match pc {
            0x827FD410 => {
    //   block [0x827FD410..0x827FD448)
	// 827FD410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD41C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD424: 4BFFBF1D  bl 0x827f9340
	ctx.lr = 0x827FD428;
	sub_827F9340(ctx, base);
	// 827FD428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827FD42C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD430: 4BC91F19  bl 0x8248f348
	ctx.lr = 0x827FD434;
	sub_8248F348(ctx, base);
	// 827FD434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FD438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FD448 size=8
    let mut pc: u32 = 0x827FD448;
    'dispatch: loop {
        match pc {
            0x827FD448 => {
    //   block [0x827FD448..0x827FD450)
	// 827FD448: 38630124  addi r3, r3, 0x124
	ctx.r[3].s64 = ctx.r[3].s64 + 292;
	// 827FD44C: 485F5D64  b 0x82df31b0
	sub_82DF31B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FD450 size=4
    let mut pc: u32 = 0x827FD450;
    'dispatch: loop {
        match pc {
            0x827FD450 => {
    //   block [0x827FD450..0x827FD454)
	// 827FD450: 4BFFBF98  b 0x827f93e8
	sub_827F93E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD458 size=76
    let mut pc: u32 = 0x827FD458;
    'dispatch: loop {
        match pc {
            0x827FD458 => {
    //   block [0x827FD458..0x827FD4A4)
	// 827FD458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD46C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FD474: 4BFFC04D  bl 0x827f94c0
	ctx.lr = 0x827FD478;
	sub_827F94C0(ctx, base);
	// 827FD478: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FD47C: 4BC91ECD  bl 0x8248f348
	ctx.lr = 0x827FD480;
	sub_8248F348(ctx, base);
	// 827FD480: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FD484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD488: 4BFFC321  bl 0x827f97a8
	ctx.lr = 0x827FD48C;
	sub_827F97A8(ctx, base);
	// 827FD48C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD498: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD49C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FD4A8 size=8
    let mut pc: u32 = 0x827FD4A8;
    'dispatch: loop {
        match pc {
            0x827FD4A8 => {
    //   block [0x827FD4A8..0x827FD4B0)
	// 827FD4A8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827FD4AC: 480000CC  b 0x827fd578
	sub_827FD578(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD4B0 size=88
    let mut pc: u32 = 0x827FD4B0;
    'dispatch: loop {
        match pc {
            0x827FD4B0 => {
    //   block [0x827FD4B0..0x827FD508)
	// 827FD4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD4B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD4BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD4C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD4C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD4C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FD4CC: 807F0188  lwz r3, 0x188(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(392 as u32) ) } as u64;
	// 827FD4D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD4D4: 419A0008  beq cr6, 0x827fd4dc
	if ctx.cr[6].eq {
	pc = 0x827FD4DC; continue 'dispatch;
	}
	// 827FD4D8: 483491F1  bl 0x82b466c8
	ctx.lr = 0x827FD4DC;
	sub_82B466C8(ctx, base);
	// 827FD4DC: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 827FD4E0: 4865F1B9  bl 0x82e5c698
	ctx.lr = 0x827FD4E4;
	sub_82E5C698(ctx, base);
	// 827FD4E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FD4E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD4EC: 48006E3D  bl 0x82804328
	ctx.lr = 0x827FD4F0;
	sub_82804328(ctx, base);
	// 827FD4F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD4FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD508 size=108
    let mut pc: u32 = 0x827FD508;
    'dispatch: loop {
        match pc {
            0x827FD508 => {
    //   block [0x827FD508..0x827FD574)
	// 827FD508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD50C: 489AAC61  bl 0x831a816c
	ctx.lr = 0x827FD510;
	sub_831A8130(ctx, base);
	// 827FD510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD514: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FD518: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FD51C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD520: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD524: 41820038  beq 0x827fd55c
	if ctx.cr[0].eq {
	pc = 0x827FD55C; continue 'dispatch;
	}
	// 827FD528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD52C: 489AC45D  bl 0x831a9988
	ctx.lr = 0x827FD530;
	sub_831A9988(ctx, base);
	// 827FD530: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FD534: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FD538: 386BD1B0  addi r3, r11, -0x2e50
	ctx.r[3].s64 = ctx.r[11].s64 + -11856;
	// 827FD53C: 489AABBD  bl 0x831a80f8
	ctx.lr = 0x827FD540;
	sub_831A80F8(ctx, base);
	// 827FD540: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD544: 41820018  beq 0x827fd55c
	if ctx.cr[0].eq {
	pc = 0x827FD55C; continue 'dispatch;
	}
	// 827FD548: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FD54C: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 827FD550: 4BFFFF09  bl 0x827fd458
	ctx.lr = 0x827FD554;
	sub_827FD458(ctx, base);
	// 827FD554: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827FD558: 48000014  b 0x827fd56c
	pc = 0x827FD56C; continue 'dispatch;
	// 827FD55C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827FD560: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FD564: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FD568: 4BFFC4D9  bl 0x827f9a40
	ctx.lr = 0x827FD56C;
	sub_827F9A40(ctx, base);
	// 827FD56C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD570: 489AAC4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD578 size=76
    let mut pc: u32 = 0x827FD578;
    'dispatch: loop {
        match pc {
            0x827FD578 => {
    //   block [0x827FD578..0x827FD5C4)
	// 827FD578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD58C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD590: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FD594: 4BFFFE15  bl 0x827fd3a8
	ctx.lr = 0x827FD598;
	sub_827FD3A8(ctx, base);
	// 827FD598: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD59C: 4182000C  beq 0x827fd5a8
	if ctx.cr[0].eq {
	pc = 0x827FD5A8; continue 'dispatch;
	}
	// 827FD5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD5A4: 485F4E35  bl 0x82df23d8
	ctx.lr = 0x827FD5A8;
	sub_82DF23D8(ctx, base);
	// 827FD5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD5AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD5B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD5BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FD5C8 size=204
    let mut pc: u32 = 0x827FD5C8;
    'dispatch: loop {
        match pc {
            0x827FD5C8 => {
    //   block [0x827FD5C8..0x827FD694)
	// 827FD5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD5D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD5D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD5D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD5E0: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 827FD5E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FD5E8: 419A0018  beq cr6, 0x827fd600
	if ctx.cr[6].eq {
	pc = 0x827FD600; continue 'dispatch;
	}
	// 827FD5EC: 486011F5  bl 0x82dfe7e0
	ctx.lr = 0x827FD5F0;
	sub_82DFE7E0(ctx, base);
	// 827FD5F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FD5F4: 4182000C  beq 0x827fd600
	if ctx.cr[0].eq {
	pc = 0x827FD600; continue 'dispatch;
	}
	// 827FD5F8: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 827FD5FC: 48608F25  bl 0x82e06520
	ctx.lr = 0x827FD600;
	sub_82E06520(ctx, base);
	// 827FD600: 3BDF00C8  addi r30, r31, 0xc8
	ctx.r[30].s64 = ctx.r[31].s64 + 200;
	// 827FD604: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD608: 488B4979  bl 0x830b1f80
	ctx.lr = 0x827FD60C;
	sub_830B1F80(ctx, base);
	// 827FD60C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD610: 4182006C  beq 0x827fd67c
	if ctx.cr[0].eq {
	pc = 0x827FD67C; continue 'dispatch;
	}
	// 827FD614: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FD618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD61C: 4BD11EAD  bl 0x8250f4c8
	ctx.lr = 0x827FD620;
	sub_8250F4C8(ctx, base);
	// 827FD620: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FD628: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 827FD62C: 409A0008  bne cr6, 0x827fd634
	if !ctx.cr[6].eq {
	pc = 0x827FD634; continue 'dispatch;
	}
	// 827FD630: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827FD634: 38BF0100  addi r5, r31, 0x100
	ctx.r[5].s64 = ctx.r[31].s64 + 256;
	// 827FD638: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FD63C: 4BD0EA4D  bl 0x8250c088
	ctx.lr = 0x827FD640;
	sub_8250C088(ctx, base);
	// 827FD640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD644: 485F464D  bl 0x82df1c90
	ctx.lr = 0x827FD648;
	sub_82DF1C90(ctx, base);
	// 827FD648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD64C: 488B48DD  bl 0x830b1f28
	ctx.lr = 0x827FD650;
	sub_830B1F28(ctx, base);
	// 827FD650: C0410064  lfs f2, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 827FD654: C0210060  lfs f1, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FD658: 488B59A9  bl 0x830b3000
	ctx.lr = 0x827FD65C;
	sub_830B3000(ctx, base);
	// 827FD65C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD660: 488B48C9  bl 0x830b1f28
	ctx.lr = 0x827FD664;
	sub_830B1F28(ctx, base);
	// 827FD664: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FD668: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD66C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FD670: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 827FD674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FD678: 4E800421  bctrl
	ctx.lr = 0x827FD67C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FD67C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FD680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD688: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD68C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD698 size=164
    let mut pc: u32 = 0x827FD698;
    'dispatch: loop {
        match pc {
            0x827FD698 => {
    //   block [0x827FD698..0x827FD73C)
	// 827FD698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD6A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD6A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD6A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD6AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FD6B0: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 827FD6B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD6B8: 488B48C9  bl 0x830b1f80
	ctx.lr = 0x827FD6BC;
	sub_830B1F80(ctx, base);
	// 827FD6BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD6C0: 41820040  beq 0x827fd700
	if ctx.cr[0].eq {
	pc = 0x827FD700; continue 'dispatch;
	}
	// 827FD6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD6C8: 488B4861  bl 0x830b1f28
	ctx.lr = 0x827FD6CC;
	sub_830B1F28(ctx, base);
	// 827FD6CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827FD6D0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FD6D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FD6D8: 38CBBF64  addi r6, r11, -0x409c
	ctx.r[6].s64 = ctx.r[11].s64 + -16540;
	// 827FD6DC: 38AA7098  addi r5, r10, 0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + 28824;
	// 827FD6E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827FD6E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD6E8: 488B93E1  bl 0x830b6ac8
	ctx.lr = 0x827FD6EC;
	sub_830B6AC8(ctx, base);
	// 827FD6EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FD6F0: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 827FD6F4: 488B4935  bl 0x830b2028
	ctx.lr = 0x827FD6F8;
	sub_830B2028(ctx, base);
	// 827FD6F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD6FC: 488B47DD  bl 0x830b1ed8
	ctx.lr = 0x827FD700;
	sub_830B1ED8(ctx, base);
	// 827FD700: 3BFF00C8  addi r31, r31, 0xc8
	ctx.r[31].s64 = ctx.r[31].s64 + 200;
	// 827FD704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD708: 488B4879  bl 0x830b1f80
	ctx.lr = 0x827FD70C;
	sub_830B1F80(ctx, base);
	// 827FD70C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD710: 41820014  beq 0x827fd724
	if ctx.cr[0].eq {
	pc = 0x827FD724; continue 'dispatch;
	}
	// 827FD714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD718: 488B4811  bl 0x830b1f28
	ctx.lr = 0x827FD71C;
	sub_830B1F28(ctx, base);
	// 827FD71C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827FD720: 482E5C21  bl 0x82ae3340
	ctx.lr = 0x827FD724;
	sub_82AE3340(ctx, base);
	// 827FD724: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD740 size=196
    let mut pc: u32 = 0x827FD740;
    'dispatch: loop {
        match pc {
            0x827FD740 => {
    //   block [0x827FD740..0x827FD804)
	// 827FD740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD74C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD754: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FD758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD75C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FD760: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD764: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD768: 4BAC31D1  bl 0x822c0938
	ctx.lr = 0x827FD76C;
	sub_822C0938(ctx, base);
	// 827FD76C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD770: 41820028  beq 0x827fd798
	if ctx.cr[0].eq {
	pc = 0x827FD798; continue 'dispatch;
	}
	// 827FD774: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD778: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FD77C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FD780: 392B6FFC  addi r9, r11, 0x6ffc
	ctx.r[9].s64 = ctx.r[11].s64 + 28668;
	// 827FD784: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FD788: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FD78C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FD790: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FD794: 48000008  b 0x827fd79c
	pc = 0x827FD79C; continue 'dispatch;
	// 827FD798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD79C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD7A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FD7A4: 409A0044  bne cr6, 0x827fd7e8
	if !ctx.cr[6].eq {
	pc = 0x827FD7E8; continue 'dispatch;
	}
	// 827FD7A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FD7AC: 419A001C  beq cr6, 0x827fd7c8
	if ctx.cr[6].eq {
	pc = 0x827FD7C8; continue 'dispatch;
	}
	// 827FD7B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD7B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FD7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD7BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD7C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FD7C4: 4E800421  bctrl
	ctx.lr = 0x827FD7C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FD7C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FD7CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FD7D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD7D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FD7D8: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FD7DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FD7E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FD7E4: 4BAC281D  bl 0x822c0000
	ctx.lr = 0x827FD7E8;
	sub_822C0000(ctx, base);
	// 827FD7E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD7EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD7F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD7FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD808 size=196
    let mut pc: u32 = 0x827FD808;
    'dispatch: loop {
        match pc {
            0x827FD808 => {
    //   block [0x827FD808..0x827FD8CC)
	// 827FD808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD81C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FD820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD824: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FD828: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD82C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD830: 4BAC3109  bl 0x822c0938
	ctx.lr = 0x827FD834;
	sub_822C0938(ctx, base);
	// 827FD834: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD838: 41820028  beq 0x827fd860
	if ctx.cr[0].eq {
	pc = 0x827FD860; continue 'dispatch;
	}
	// 827FD83C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD840: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FD844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FD848: 392B7010  addi r9, r11, 0x7010
	ctx.r[9].s64 = ctx.r[11].s64 + 28688;
	// 827FD84C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FD850: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FD854: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FD858: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FD85C: 48000008  b 0x827fd864
	pc = 0x827FD864; continue 'dispatch;
	// 827FD860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD864: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FD86C: 409A0044  bne cr6, 0x827fd8b0
	if !ctx.cr[6].eq {
	pc = 0x827FD8B0; continue 'dispatch;
	}
	// 827FD870: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FD874: 419A001C  beq cr6, 0x827fd890
	if ctx.cr[6].eq {
	pc = 0x827FD890; continue 'dispatch;
	}
	// 827FD878: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD87C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FD880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD884: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827FD888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FD88C: 4E800421  bctrl
	ctx.lr = 0x827FD890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FD890: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FD894: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FD898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD89C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FD8A0: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FD8A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FD8A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FD8AC: 4BAC2755  bl 0x822c0000
	ctx.lr = 0x827FD8B0;
	sub_822C0000(ctx, base);
	// 827FD8B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD8B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD8C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD8C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD8D0 size=196
    let mut pc: u32 = 0x827FD8D0;
    'dispatch: loop {
        match pc {
            0x827FD8D0 => {
    //   block [0x827FD8D0..0x827FD994)
	// 827FD8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD8D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD8DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD8E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD8E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FD8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD8EC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FD8F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD8F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD8F8: 4BAC3041  bl 0x822c0938
	ctx.lr = 0x827FD8FC;
	sub_822C0938(ctx, base);
	// 827FD8FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD900: 41820028  beq 0x827fd928
	if ctx.cr[0].eq {
	pc = 0x827FD928; continue 'dispatch;
	}
	// 827FD904: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD908: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FD90C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FD910: 392B7024  addi r9, r11, 0x7024
	ctx.r[9].s64 = ctx.r[11].s64 + 28708;
	// 827FD914: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FD918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FD91C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FD920: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FD924: 48000008  b 0x827fd92c
	pc = 0x827FD92C; continue 'dispatch;
	// 827FD928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD92C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FD934: 409A0044  bne cr6, 0x827fd978
	if !ctx.cr[6].eq {
	pc = 0x827FD978; continue 'dispatch;
	}
	// 827FD938: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FD93C: 419A001C  beq cr6, 0x827fd958
	if ctx.cr[6].eq {
	pc = 0x827FD958; continue 'dispatch;
	}
	// 827FD940: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD944: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FD948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FD94C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FD950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FD954: 4E800421  bctrl
	ctx.lr = 0x827FD958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FD958: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FD95C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FD960: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FD964: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FD968: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FD96C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FD970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FD974: 4BAC268D  bl 0x822c0000
	ctx.lr = 0x827FD978;
	sub_822C0000(ctx, base);
	// 827FD978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FD97C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FD980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FD984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FD988: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FD98C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FD990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FD998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FD998 size=196
    let mut pc: u32 = 0x827FD998;
    'dispatch: loop {
        match pc {
            0x827FD998 => {
    //   block [0x827FD998..0x827FDA5C)
	// 827FD998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FD99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FD9A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FD9A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FD9A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FD9AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FD9B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD9B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FD9B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FD9BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD9C0: 4BAC2F79  bl 0x822c0938
	ctx.lr = 0x827FD9C4;
	sub_822C0938(ctx, base);
	// 827FD9C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FD9C8: 41820028  beq 0x827fd9f0
	if ctx.cr[0].eq {
	pc = 0x827FD9F0; continue 'dispatch;
	}
	// 827FD9CC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FD9D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FD9D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FD9D8: 392B7038  addi r9, r11, 0x7038
	ctx.r[9].s64 = ctx.r[11].s64 + 28728;
	// 827FD9DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FD9E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FD9E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FD9E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FD9EC: 48000008  b 0x827fd9f4
	pc = 0x827FD9F4; continue 'dispatch;
	// 827FD9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FD9F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FD9F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FD9FC: 409A0044  bne cr6, 0x827fda40
	if !ctx.cr[6].eq {
	pc = 0x827FDA40; continue 'dispatch;
	}
	// 827FDA00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDA04: 419A001C  beq cr6, 0x827fda20
	if ctx.cr[6].eq {
	pc = 0x827FDA20; continue 'dispatch;
	}
	// 827FDA08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDA0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FDA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDA14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDA18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FDA1C: 4E800421  bctrl
	ctx.lr = 0x827FDA20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FDA20: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FDA24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FDA28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FDA2C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FDA30: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FDA34: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FDA38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FDA3C: 4BAC25C5  bl 0x822c0000
	ctx.lr = 0x827FDA40;
	sub_822C0000(ctx, base);
	// 827FDA40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDA44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDA48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDA4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDA50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FDA54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDA60 size=196
    let mut pc: u32 = 0x827FDA60;
    'dispatch: loop {
        match pc {
            0x827FDA60 => {
    //   block [0x827FDA60..0x827FDB24)
	// 827FDA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FDA68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FDA6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FDA70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDA74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FDA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDA7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FDA80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FDA84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDA88: 4BAC2EB1  bl 0x822c0938
	ctx.lr = 0x827FDA8C;
	sub_822C0938(ctx, base);
	// 827FDA8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FDA90: 41820028  beq 0x827fdab8
	if ctx.cr[0].eq {
	pc = 0x827FDAB8; continue 'dispatch;
	}
	// 827FDA94: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FDA98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FDA9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FDAA0: 392B704C  addi r9, r11, 0x704c
	ctx.r[9].s64 = ctx.r[11].s64 + 28748;
	// 827FDAA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FDAA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FDAAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FDAB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FDAB4: 48000008  b 0x827fdabc
	pc = 0x827FDABC; continue 'dispatch;
	// 827FDAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDABC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDAC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FDAC4: 409A0044  bne cr6, 0x827fdb08
	if !ctx.cr[6].eq {
	pc = 0x827FDB08; continue 'dispatch;
	}
	// 827FDAC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDACC: 419A001C  beq cr6, 0x827fdae8
	if ctx.cr[6].eq {
	pc = 0x827FDAE8; continue 'dispatch;
	}
	// 827FDAD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDAD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FDAD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDADC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDAE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FDAE4: 4E800421  bctrl
	ctx.lr = 0x827FDAE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FDAE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FDAEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FDAF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FDAF4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FDAF8: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FDAFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FDB00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FDB04: 4BAC24FD  bl 0x822c0000
	ctx.lr = 0x827FDB08;
	sub_822C0000(ctx, base);
	// 827FDB08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDB0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDB18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FDB1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDB28 size=196
    let mut pc: u32 = 0x827FDB28;
    'dispatch: loop {
        match pc {
            0x827FDB28 => {
    //   block [0x827FDB28..0x827FDBEC)
	// 827FDB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FDB30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FDB34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FDB38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDB3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FDB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDB44: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FDB48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FDB4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDB50: 4BAC2DE9  bl 0x822c0938
	ctx.lr = 0x827FDB54;
	sub_822C0938(ctx, base);
	// 827FDB54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FDB58: 41820028  beq 0x827fdb80
	if ctx.cr[0].eq {
	pc = 0x827FDB80; continue 'dispatch;
	}
	// 827FDB5C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FDB60: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FDB64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FDB68: 392B7060  addi r9, r11, 0x7060
	ctx.r[9].s64 = ctx.r[11].s64 + 28768;
	// 827FDB6C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FDB70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FDB74: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FDB78: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FDB7C: 48000008  b 0x827fdb84
	pc = 0x827FDB84; continue 'dispatch;
	// 827FDB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDB84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDB88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FDB8C: 409A0044  bne cr6, 0x827fdbd0
	if !ctx.cr[6].eq {
	pc = 0x827FDBD0; continue 'dispatch;
	}
	// 827FDB90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDB94: 419A001C  beq cr6, 0x827fdbb0
	if ctx.cr[6].eq {
	pc = 0x827FDBB0; continue 'dispatch;
	}
	// 827FDB98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDB9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FDBA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDBA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDBA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FDBAC: 4E800421  bctrl
	ctx.lr = 0x827FDBB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FDBB0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FDBB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FDBB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FDBBC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FDBC0: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FDBC4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FDBC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FDBCC: 4BAC2435  bl 0x822c0000
	ctx.lr = 0x827FDBD0;
	sub_822C0000(ctx, base);
	// 827FDBD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDBD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDBE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FDBE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDBF0 size=196
    let mut pc: u32 = 0x827FDBF0;
    'dispatch: loop {
        match pc {
            0x827FDBF0 => {
    //   block [0x827FDBF0..0x827FDCB4)
	// 827FDBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FDBF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FDBFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FDC00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDC04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FDC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDC0C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FDC10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FDC14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDC18: 4BAC2D21  bl 0x822c0938
	ctx.lr = 0x827FDC1C;
	sub_822C0938(ctx, base);
	// 827FDC1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FDC20: 41820028  beq 0x827fdc48
	if ctx.cr[0].eq {
	pc = 0x827FDC48; continue 'dispatch;
	}
	// 827FDC24: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FDC28: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FDC2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FDC30: 392B7074  addi r9, r11, 0x7074
	ctx.r[9].s64 = ctx.r[11].s64 + 28788;
	// 827FDC34: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FDC38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FDC3C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FDC40: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FDC44: 48000008  b 0x827fdc4c
	pc = 0x827FDC4C; continue 'dispatch;
	// 827FDC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDC4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDC50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FDC54: 409A0044  bne cr6, 0x827fdc98
	if !ctx.cr[6].eq {
	pc = 0x827FDC98; continue 'dispatch;
	}
	// 827FDC58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDC5C: 419A001C  beq cr6, 0x827fdc78
	if ctx.cr[6].eq {
	pc = 0x827FDC78; continue 'dispatch;
	}
	// 827FDC60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDC64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FDC68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDC6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDC70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FDC74: 4E800421  bctrl
	ctx.lr = 0x827FDC78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FDC78: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FDC7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FDC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FDC84: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FDC88: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FDC8C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FDC90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FDC94: 4BAC236D  bl 0x822c0000
	ctx.lr = 0x827FDC98;
	sub_822C0000(ctx, base);
	// 827FDC98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDC9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDCA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FDCAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDCB8 size=196
    let mut pc: u32 = 0x827FDCB8;
    'dispatch: loop {
        match pc {
            0x827FDCB8 => {
    //   block [0x827FDCB8..0x827FDD7C)
	// 827FDCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FDCC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FDCC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FDCC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDCCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FDCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDCD4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827FDCD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FDCDC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDCE0: 4BAC2C59  bl 0x822c0938
	ctx.lr = 0x827FDCE4;
	sub_822C0938(ctx, base);
	// 827FDCE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FDCE8: 41820028  beq 0x827fdd10
	if ctx.cr[0].eq {
	pc = 0x827FDD10; continue 'dispatch;
	}
	// 827FDCEC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FDCF0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827FDCF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827FDCF8: 392B7088  addi r9, r11, 0x7088
	ctx.r[9].s64 = ctx.r[11].s64 + 28808;
	// 827FDCFC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827FDD00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FDD04: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827FDD08: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827FDD0C: 48000008  b 0x827fdd14
	pc = 0x827FDD14; continue 'dispatch;
	// 827FDD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDD14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDD18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FDD1C: 409A0044  bne cr6, 0x827fdd60
	if !ctx.cr[6].eq {
	pc = 0x827FDD60; continue 'dispatch;
	}
	// 827FDD20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDD24: 419A001C  beq cr6, 0x827fdd40
	if ctx.cr[6].eq {
	pc = 0x827FDD40; continue 'dispatch;
	}
	// 827FDD28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDD2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827FDD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDD34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDD38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FDD3C: 4E800421  bctrl
	ctx.lr = 0x827FDD40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FDD40: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827FDD44: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FDD48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FDD4C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827FDD50: 816BD5E4  lwz r11, -0x2a1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10780 as u32) ) } as u64;
	// 827FDD54: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827FDD58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FDD5C: 4BAC22A5  bl 0x822c0000
	ctx.lr = 0x827FDD60;
	sub_822C0000(ctx, base);
	// 827FDD60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDD64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDD70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FDD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDD80 size=112
    let mut pc: u32 = 0x827FDD80;
    'dispatch: loop {
        match pc {
            0x827FDD80 => {
    //   block [0x827FDD80..0x827FDDF0)
	// 827FDD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FDD88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FDD8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDD90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FDD94: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FDD98: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FDD9C: 396B70D4  addi r11, r11, 0x70d4
	ctx.r[11].s64 = ctx.r[11].s64 + 28884;
	// 827FDDA0: 394A70C0  addi r10, r10, 0x70c0
	ctx.r[10].s64 = ctx.r[10].s64 + 28864;
	// 827FDDA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDDA8: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FDDAC: 807F018C  lwz r3, 0x18c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(396 as u32) ) } as u64;
	// 827FDDB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FDDB4: 419A0008  beq cr6, 0x827fddbc
	if ctx.cr[6].eq {
	pc = 0x827FDDBC; continue 'dispatch;
	}
	// 827FDDB8: 4BAC2AD9  bl 0x822c0890
	ctx.lr = 0x827FDDBC;
	sub_822C0890(ctx, base);
	// 827FDDBC: 807F0184  lwz r3, 0x184(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(388 as u32) ) } as u64;
	// 827FDDC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FDDC4: 419A0008  beq cr6, 0x827fddcc
	if ctx.cr[6].eq {
	pc = 0x827FDDCC; continue 'dispatch;
	}
	// 827FDDC8: 4BAC2AC9  bl 0x822c0890
	ctx.lr = 0x827FDDCC;
	sub_822C0890(ctx, base);
	// 827FDDCC: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 827FDDD0: 4865FA79  bl 0x82e5d848
	ctx.lr = 0x827FDDD4;
	sub_82E5D848(ctx, base);
	// 827FDDD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDDD8: 4BFFBD81  bl 0x827f9b58
	ctx.lr = 0x827FDDDC;
	sub_827F9B58(ctx, base);
	// 827FDDDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827FDDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDDE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827FDDF0 size=8
    let mut pc: u32 = 0x827FDDF0;
    'dispatch: loop {
        match pc {
            0x827FDDF0 => {
    //   block [0x827FDDF0..0x827FDDF8)
	// 827FDDF0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827FDDF4: 4800027C  b 0x827fe070
	sub_827FE070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDDF8 size=204
    let mut pc: u32 = 0x827FDDF8;
    'dispatch: loop {
        match pc {
            0x827FDDF8 => {
    //   block [0x827FDDF8..0x827FDEC4)
	// 827FDDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDDFC: 489AA36D  bl 0x831a8168
	ctx.lr = 0x827FDE00;
	sub_831A8130(ctx, base);
	// 827FDE00: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDE04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FDE08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FDE0C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FDE10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827FDE14: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827FDE18: 4BD11701  bl 0x8250f518
	ctx.lr = 0x827FDE1C;
	sub_8250F518(ctx, base);
	// 827FDE1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDE20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FDE24: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827FDE28: 409A0008  bne cr6, 0x827fde30
	if !ctx.cr[6].eq {
	pc = 0x827FDE30; continue 'dispatch;
	}
	// 827FDE2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827FDE30: 4BD2A211  bl 0x82528040
	ctx.lr = 0x827FDE34;
	sub_82528040(ctx, base);
	// 827FDE34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FDE38: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FDE3C: 485F3E55  bl 0x82df1c90
	ctx.lr = 0x827FDE40;
	sub_82DF1C90(ctx, base);
	// 827FDE40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDE44: 419A0074  beq cr6, 0x827fdeb8
	if ctx.cr[6].eq {
	pc = 0x827FDEB8; continue 'dispatch;
	}
	// 827FDE48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FDE4C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827FDE50: 4867F601  bl 0x82e7d450
	ctx.lr = 0x827FDE54;
	sub_82E7D450(ctx, base);
	// 827FDE54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FDE58: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDEC8 size=96
    let mut pc: u32 = 0x827FDEC8;
    'dispatch: loop {
        match pc {
            0x827FDEC8 => {
    //   block [0x827FDEC8..0x827FDF28)
	// 827FDEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FDED0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FDED4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FDEDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FDEE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FDEE4: 482ADE05  bl 0x82aabce8
	ctx.lr = 0x827FDEE8;
	sub_82AABCE8(ctx, base);
	// 827FDEE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FDEEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDEF0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDEF4: 4BFFB465  bl 0x827f9358
	ctx.lr = 0x827FDEF8;
	sub_827F9358(ctx, base);
	// 827FDEF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FDEFC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FDF00: 486150D1  bl 0x82e12fd0
	ctx.lr = 0x827FDF04;
	sub_82E12FD0(ctx, base);
	// 827FDF04: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FDF08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FDF0C: 419A0008  beq cr6, 0x827fdf14
	if ctx.cr[6].eq {
	pc = 0x827FDF14; continue 'dispatch;
	}
	// 827FDF10: 4BAC2981  bl 0x822c0890
	ctx.lr = 0x827FDF14;
	sub_822C0890(ctx, base);
	// 827FDF14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FDF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FDF20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FDF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FDF28 size=128
    let mut pc: u32 = 0x827FDF28;
    'dispatch: loop {
        match pc {
            0x827FDF28 => {
    //   block [0x827FDF28..0x827FDFA8)
	// 827FDF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDF2C: 489AA241  bl 0x831a816c
	ctx.lr = 0x827FDF30;
	sub_831A8130(ctx, base);
	// 827FDF30: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDF34: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FDF38: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827FDF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FDF40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FDF44: C1AB08A8  lfs f13, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827FDF48: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827FDF4C: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827FDF50: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827FDF54: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827FDF58: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827FDF5C: 4BD13BFD  bl 0x82511b58
	ctx.lr = 0x827FDF60;
	sub_82511B58(ctx, base);
	// 827FDF60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FDF64: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827FDF68: 4867DF31  bl 0x82e7be98
	ctx.lr = 0x827FDF6C;
	sub_82E7BE98(ctx, base);
	// 827FDF6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FDF70: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FDF74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FDF78: 4867DD51  bl 0x82e7bcc8
	ctx.lr = 0x827FDF7C;
	sub_82E7BCC8(ctx, base);
	// 827FDF7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDF80: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 827FDF84: 83DE0018  lwz r30, 0x18(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 827FDF88: 4BD13B61  bl 0x82511ae8
	ctx.lr = 0x827FDF8C;
	sub_82511AE8(ctx, base);
	// 827FDF8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FDF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FDF94: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827FDF98: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827FDF9C: 4BFFFE5D  bl 0x827fddf8
	ctx.lr = 0x827FDFA0;
	sub_827FDDF8(ctx, base);
	// 827FDFA0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827FDFA4: 489AA218  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FDFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FDFA8 size=84
    let mut pc: u32 = 0x827FDFA8;
    'dispatch: loop {
        match pc {
            0x827FDFA8 => {
    //   block [0x827FDFA8..0x827FDFFC)
	// 827FDFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FDFAC: 489AA1C1  bl 0x831a816c
	ctx.lr = 0x827FDFB0;
	sub_831A8130(ctx, base);
	// 827FDFB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FDFB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FDFB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FDFBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FDFC0: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 827FDFC4: 409A0008  bne cr6, 0x827fdfcc
	if !ctx.cr[6].eq {
	pc = 0x827FDFCC; continue 'dispatch;
	}
	// 827FDFC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FDFCC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FDFD0: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FDFD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FDFD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDFDC: 4BFFF8F5  bl 0x827fd8d0
	ctx.lr = 0x827FDFE0;
	sub_827FD8D0(ctx, base);
	// 827FDFE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FDFE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FDFE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FDFEC: 4BAC2015  bl 0x822c0000
	ctx.lr = 0x827FDFF0;
	sub_822C0000(ctx, base);
	// 827FDFF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FDFF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FDFF8: 489AA1C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE000 size=112
    let mut pc: u32 = 0x827FE000;
    'dispatch: loop {
        match pc {
            0x827FE000 => {
    //   block [0x827FE000..0x827FE070)
	// 827FE000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE004: 489AA169  bl 0x831a816c
	ctx.lr = 0x827FE008;
	sub_831A8130(ctx, base);
	// 827FE008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE00C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE010: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FE014: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827FE018: 4BFFBC01  bl 0x827f9c18
	ctx.lr = 0x827FE01C;
	sub_827F9C18(ctx, base);
	// 827FE01C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE020: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FE024: 396B70D4  addi r11, r11, 0x70d4
	ctx.r[11].s64 = ctx.r[11].s64 + 28884;
	// 827FE028: 394A70C0  addi r10, r10, 0x70c0
	ctx.r[10].s64 = ctx.r[10].s64 + 28864;
	// 827FE02C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE030: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 827FE034: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827FE038: 4865F899  bl 0x82e5d8d0
	ctx.lr = 0x827FE03C;
	sub_82E5D8D0(ctx, base);
	// 827FE03C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FE040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FE044: 394A6F7C  addi r10, r10, 0x6f7c
	ctx.r[10].s64 = ctx.r[10].s64 + 28540;
	// 827FE048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE04C: 915F0120  stw r10, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[10].u32 ) };
	// 827FE050: 917F0180  stw r11, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 827FE054: 917F0184  stw r11, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 827FE058: 917F0188  stw r11, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 827FE05C: 917F018C  stw r11, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[11].u32 ) };
	// 827FE060: 93DF0190  stw r30, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[30].u32 ) };
	// 827FE064: 9BBF0194  stb r29, 0x194(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[29].u8 ) };
	// 827FE068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE06C: 489AA150  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE070 size=76
    let mut pc: u32 = 0x827FE070;
    'dispatch: loop {
        match pc {
            0x827FE070 => {
    //   block [0x827FE070..0x827FE0BC)
	// 827FE070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FE078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FE07C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FE080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE084: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE088: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827FE08C: 4BFFFCF5  bl 0x827fdd80
	ctx.lr = 0x827FE090;
	sub_827FDD80(ctx, base);
	// 827FE090: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827FE094: 4182000C  beq 0x827fe0a0
	if ctx.cr[0].eq {
	pc = 0x827FE0A0; continue 'dispatch;
	}
	// 827FE098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE09C: 485F433D  bl 0x82df23d8
	ctx.lr = 0x827FE0A0;
	sub_82DF23D8(ctx, base);
	// 827FE0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FE0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FE0B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FE0B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FE0B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE0C0 size=224
    let mut pc: u32 = 0x827FE0C0;
    'dispatch: loop {
        match pc {
            0x827FE0C0 => {
    //   block [0x827FE0C0..0x827FE1A0)
	// 827FE0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE0C4: 489AA0A9  bl 0x831a816c
	ctx.lr = 0x827FE0C8;
	sub_831A8130(ctx, base);
	// 827FE0C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE0CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE0D0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827FE0D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE0D8: 4862CA11  bl 0x82e2aae8
	ctx.lr = 0x827FE0DC;
	sub_82E2AAE8(ctx, base);
	// 827FE0DC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE0E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE0E4: 388B71AC  addi r4, r11, 0x71ac
	ctx.r[4].s64 = ctx.r[11].s64 + 29100;
	// 827FE0E8: 485F5921  bl 0x82df3a08
	ctx.lr = 0x827FE0EC;
	sub_82DF3A08(ctx, base);
	// 827FE0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE0F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FE0F4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827FE0F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FE0FC: 48630D75  bl 0x82e2ee70
	ctx.lr = 0x827FE100;
	sub_82E2EE70(ctx, base);
	// 827FE100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE104: 485F5325  bl 0x82df3428
	ctx.lr = 0x827FE108;
	sub_82DF3428(ctx, base);
	// 827FE108: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FE10C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE110: 419A0060  beq cr6, 0x827fe170
	if ctx.cr[6].eq {
	pc = 0x827FE170; continue 'dispatch;
	}
	// 827FE114: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE118: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE11C: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FE120: 38A000A9  li r5, 0xa9
	ctx.r[5].s64 = 169;
	// 827FE124: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 827FE128: 485F42C1  bl 0x82df23e8
	ctx.lr = 0x827FE12C;
	sub_82DF23E8(ctx, base);
	// 827FE12C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FE130: 41820014  beq 0x827fe144
	if ctx.cr[0].eq {
	pc = 0x827FE144; continue 'dispatch;
	}
	// 827FE134: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827FE138: 48618759  bl 0x82e16890
	ctx.lr = 0x827FE13C;
	sub_82E16890(ctx, base);
	// 827FE13C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE140: 48000008  b 0x827fe148
	pc = 0x827FE148; continue 'dispatch;
	// 827FE144: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE148: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE14C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FE150: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE154: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE158: 4BB5AF61  bl 0x823590b8
	ctx.lr = 0x827FE15C;
	sub_823590B8(ctx, base);
	// 827FE15C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE160: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE164: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE168: 4BAC1E99  bl 0x822c0000
	ctx.lr = 0x827FE16C;
	sub_822C0000(ctx, base);
	// 827FE16C: 48000010  b 0x827fe17c
	pc = 0x827FE17C; continue 'dispatch;
	// 827FE170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FE174: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE178: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827FE17C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FE180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE184: 419A0008  beq cr6, 0x827fe18c
	if ctx.cr[6].eq {
	pc = 0x827FE18C; continue 'dispatch;
	}
	// 827FE188: 4BAC2709  bl 0x822c0890
	ctx.lr = 0x827FE18C;
	sub_822C0890(ctx, base);
	// 827FE18C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE190: 4862C971  bl 0x82e2ab00
	ctx.lr = 0x827FE194;
	sub_82E2AB00(ctx, base);
	// 827FE194: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE198: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FE19C: 489AA020  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE1A0 size=216
    let mut pc: u32 = 0x827FE1A0;
    'dispatch: loop {
        match pc {
            0x827FE1A0 => {
    //   block [0x827FE1A0..0x827FE278)
	// 827FE1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE1A4: 489A9FC9  bl 0x831a816c
	ctx.lr = 0x827FE1A8;
	sub_831A8130(ctx, base);
	// 827FE1A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE1AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FE1B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE1B4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827FE1B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE1BC: 4BAE904D  bl 0x822e7208
	ctx.lr = 0x827FE1C0;
	sub_822E7208(ctx, base);
	// 827FE1C0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE1C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE1C8: 388B71AC  addi r4, r11, 0x71ac
	ctx.r[4].s64 = ctx.r[11].s64 + 29100;
	// 827FE1CC: 485F583D  bl 0x82df3a08
	ctx.lr = 0x827FE1D0;
	sub_82DF3A08(ctx, base);
	// 827FE1D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE1D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FE1D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827FE1DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FE1E0: 4BAE91B1  bl 0x822e7390
	ctx.lr = 0x827FE1E4;
	sub_822E7390(ctx, base);
	// 827FE1E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE1E8: 485F5241  bl 0x82df3428
	ctx.lr = 0x827FE1EC;
	sub_82DF3428(ctx, base);
	// 827FE1EC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FE1F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE1F4: 419A0054  beq cr6, 0x827fe248
	if ctx.cr[6].eq {
	pc = 0x827FE248; continue 'dispatch;
	}
	// 827FE1F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FE1FC: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 827FE200: 409A0008  bne cr6, 0x827fe208
	if !ctx.cr[6].eq {
	pc = 0x827FE208; continue 'dispatch;
	}
	// 827FE204: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827FE208: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FE20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE210: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827FE214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FE218: 4E800421  bctrl
	ctx.lr = 0x827FE21C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FE21C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE224: 485F57E5  bl 0x82df3a08
	ctx.lr = 0x827FE228;
	sub_82DF3A08(ctx, base);
	// 827FE228: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827FE22C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FE230: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FE234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE238: 4BAF04A1  bl 0x822ee6d8
	ctx.lr = 0x827FE23C;
	sub_822EE6D8(ctx, base);
	// 827FE23C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE240: 485F51E9  bl 0x82df3428
	ctx.lr = 0x827FE244;
	sub_82DF3428(ctx, base);
	// 827FE244: 48000010  b 0x827fe254
	pc = 0x827FE254; continue 'dispatch;
	// 827FE248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FE24C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE250: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827FE254: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FE258: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE25C: 419A0008  beq cr6, 0x827fe264
	if ctx.cr[6].eq {
	pc = 0x827FE264; continue 'dispatch;
	}
	// 827FE260: 4BAC2631  bl 0x822c0890
	ctx.lr = 0x827FE264;
	sub_822C0890(ctx, base);
	// 827FE264: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE268: 4BAE8FB9  bl 0x822e7220
	ctx.lr = 0x827FE26C;
	sub_822E7220(ctx, base);
	// 827FE26C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE270: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FE274: 489A9F48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE278 size=124
    let mut pc: u32 = 0x827FE278;
    'dispatch: loop {
        match pc {
            0x827FE278 => {
    //   block [0x827FE278..0x827FE2F4)
	// 827FE278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FE280: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FE284: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE288: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE28C: 3BE40188  addi r31, r4, 0x188
	ctx.r[31].s64 = ctx.r[4].s64 + 392;
	// 827FE290: 81640188  lwz r11, 0x188(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(392 as u32) ) } as u64;
	// 827FE294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE298: 409A0048  bne cr6, 0x827fe2e0
	if !ctx.cr[6].eq {
	pc = 0x827FE2E0; continue 'dispatch;
	}
	// 827FE29C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FE2A0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FE2A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE2A8: 38AA71C4  addi r5, r10, 0x71c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29124;
	// 827FE2AC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 827FE2B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FE2B4: 4E800421  bctrl
	ctx.lr = 0x827FE2B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FE2B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827FE2BC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827FE2C0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827FE2C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FE2C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE2CC: 4BAC6195  bl 0x822c4460
	ctx.lr = 0x827FE2D0;
	sub_822C4460(ctx, base);
	// 827FE2D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FE2D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE2D8: 419A0008  beq cr6, 0x827fe2e0
	if ctx.cr[6].eq {
	pc = 0x827FE2E0; continue 'dispatch;
	}
	// 827FE2DC: 4BAC25B5  bl 0x822c0890
	ctx.lr = 0x827FE2E0;
	sub_822C0890(ctx, base);
	// 827FE2E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FE2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FE2EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FE2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE2F8 size=140
    let mut pc: u32 = 0x827FE2F8;
    'dispatch: loop {
        match pc {
            0x827FE2F8 => {
    //   block [0x827FE2F8..0x827FE384)
	// 827FE2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE2FC: 489A9E71  bl 0x831a816c
	ctx.lr = 0x827FE300;
	sub_831A8130(ctx, base);
	// 827FE300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE304: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE308: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE30C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE310: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE314: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FE318: 38A00175  li r5, 0x175
	ctx.r[5].s64 = 373;
	// 827FE31C: 3860012C  li r3, 0x12c
	ctx.r[3].s64 = 300;
	// 827FE320: 485F40C9  bl 0x82df23e8
	ctx.lr = 0x827FE324;
	sub_82DF23E8(ctx, base);
	// 827FE324: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FE328: 41820028  beq 0x827fe350
	if ctx.cr[0].eq {
	pc = 0x827FE350; continue 'dispatch;
	}
	// 827FE32C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE330: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FE334: 38AB71E8  addi r5, r11, 0x71e8
	ctx.r[5].s64 = ctx.r[11].s64 + 29160;
	// 827FE338: 38CA71D8  addi r6, r10, 0x71d8
	ctx.r[6].s64 = ctx.r[10].s64 + 29144;
	// 827FE33C: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 827FE340: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE344: 4BFFEFFD  bl 0x827fd340
	ctx.lr = 0x827FE348;
	sub_827FD340(ctx, base);
	// 827FE348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE34C: 48000008  b 0x827fe354
	pc = 0x827FE354; continue 'dispatch;
	// 827FE350: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE354: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE358: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FE35C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE360: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE364: 4BFFF6FD  bl 0x827fda60
	ctx.lr = 0x827FE368;
	sub_827FDA60(ctx, base);
	// 827FE368: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE36C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE374: 4BAC1C8D  bl 0x822c0000
	ctx.lr = 0x827FE378;
	sub_822C0000(ctx, base);
	// 827FE378: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE37C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE380: 489A9E3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE388 size=140
    let mut pc: u32 = 0x827FE388;
    'dispatch: loop {
        match pc {
            0x827FE388 => {
    //   block [0x827FE388..0x827FE414)
	// 827FE388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE38C: 489A9DE1  bl 0x831a816c
	ctx.lr = 0x827FE390;
	sub_831A8130(ctx, base);
	// 827FE390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE394: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE398: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE39C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE3A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE3A4: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FE3A8: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 827FE3AC: 3860012C  li r3, 0x12c
	ctx.r[3].s64 = 300;
	// 827FE3B0: 485F4039  bl 0x82df23e8
	ctx.lr = 0x827FE3B4;
	sub_82DF23E8(ctx, base);
	// 827FE3B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FE3B8: 41820028  beq 0x827fe3e0
	if ctx.cr[0].eq {
	pc = 0x827FE3E0; continue 'dispatch;
	}
	// 827FE3BC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE3C0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827FE3C4: 38AB7210  addi r5, r11, 0x7210
	ctx.r[5].s64 = ctx.r[11].s64 + 29200;
	// 827FE3C8: 38CA7200  addi r6, r10, 0x7200
	ctx.r[6].s64 = ctx.r[10].s64 + 29184;
	// 827FE3CC: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 827FE3D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE3D4: 4BFFEF6D  bl 0x827fd340
	ctx.lr = 0x827FE3D8;
	sub_827FD340(ctx, base);
	// 827FE3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE3DC: 48000008  b 0x827fe3e4
	pc = 0x827FE3E4; continue 'dispatch;
	// 827FE3E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE3E4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE3E8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FE3EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE3F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE3F4: 4BFFF66D  bl 0x827fda60
	ctx.lr = 0x827FE3F8;
	sub_827FDA60(ctx, base);
	// 827FE3F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE3FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE404: 4BAC1BFD  bl 0x822c0000
	ctx.lr = 0x827FE408;
	sub_822C0000(ctx, base);
	// 827FE408: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE40C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE410: 489A9DAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE418 size=232
    let mut pc: u32 = 0x827FE418;
    'dispatch: loop {
        match pc {
            0x827FE418 => {
    //   block [0x827FE418..0x827FE500)
	// 827FE418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE41C: 489A9D51  bl 0x831a816c
	ctx.lr = 0x827FE420;
	sub_831A8130(ctx, base);
	// 827FE420: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE424: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE428: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE42C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827FE430: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE434: 4862C6B5  bl 0x82e2aae8
	ctx.lr = 0x827FE438;
	sub_82E2AAE8(ctx, base);
	// 827FE438: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 827FE43C: 485F4D75  bl 0x82df31b0
	ctx.lr = 0x827FE440;
	sub_82DF31B0(ctx, base);
	// 827FE440: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE448: 485F55C1  bl 0x82df3a08
	ctx.lr = 0x827FE44C;
	sub_82DF3A08(ctx, base);
	// 827FE44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE450: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FE454: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827FE458: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FE45C: 48630A15  bl 0x82e2ee70
	ctx.lr = 0x827FE460;
	sub_82E2EE70(ctx, base);
	// 827FE460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE464: 485F4FC5  bl 0x82df3428
	ctx.lr = 0x827FE468;
	sub_82DF3428(ctx, base);
	// 827FE468: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FE46C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE470: 419A0060  beq cr6, 0x827fe4d0
	if ctx.cr[6].eq {
	pc = 0x827FE4D0; continue 'dispatch;
	}
	// 827FE474: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE478: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE47C: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FE480: 38A001A5  li r5, 0x1a5
	ctx.r[5].s64 = 421;
	// 827FE484: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 827FE488: 485F3F61  bl 0x82df23e8
	ctx.lr = 0x827FE48C;
	sub_82DF23E8(ctx, base);
	// 827FE48C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FE490: 41820014  beq 0x827fe4a4
	if ctx.cr[0].eq {
	pc = 0x827FE4A4; continue 'dispatch;
	}
	// 827FE494: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827FE498: 486183F9  bl 0x82e16890
	ctx.lr = 0x827FE49C;
	sub_82E16890(ctx, base);
	// 827FE49C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE4A0: 48000008  b 0x827fe4a8
	pc = 0x827FE4A8; continue 'dispatch;
	// 827FE4A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE4A8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE4AC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FE4B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE4B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE4B8: 4BB5AC01  bl 0x823590b8
	ctx.lr = 0x827FE4BC;
	sub_823590B8(ctx, base);
	// 827FE4BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE4C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE4C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE4C8: 4BAC1B39  bl 0x822c0000
	ctx.lr = 0x827FE4CC;
	sub_822C0000(ctx, base);
	// 827FE4CC: 48000010  b 0x827fe4dc
	pc = 0x827FE4DC; continue 'dispatch;
	// 827FE4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FE4D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE4D8: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827FE4DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FE4E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE4E4: 419A0008  beq cr6, 0x827fe4ec
	if ctx.cr[6].eq {
	pc = 0x827FE4EC; continue 'dispatch;
	}
	// 827FE4E8: 4BAC23A9  bl 0x822c0890
	ctx.lr = 0x827FE4EC;
	sub_822C0890(ctx, base);
	// 827FE4EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE4F0: 4862C611  bl 0x82e2ab00
	ctx.lr = 0x827FE4F4;
	sub_82E2AB00(ctx, base);
	// 827FE4F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE4F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FE4FC: 489A9CC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE500 size=220
    let mut pc: u32 = 0x827FE500;
    'dispatch: loop {
        match pc {
            0x827FE500 => {
    //   block [0x827FE500..0x827FE5DC)
	// 827FE500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE504: 489A9C69  bl 0x831a816c
	ctx.lr = 0x827FE508;
	sub_831A8130(ctx, base);
	// 827FE508: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE50C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FE510: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE514: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827FE518: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE51C: 4BAE8CED  bl 0x822e7208
	ctx.lr = 0x827FE520;
	sub_822E7208(ctx, base);
	// 827FE520: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 827FE524: 485F4C8D  bl 0x82df31b0
	ctx.lr = 0x827FE528;
	sub_82DF31B0(ctx, base);
	// 827FE528: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE52C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE530: 485F54D9  bl 0x82df3a08
	ctx.lr = 0x827FE534;
	sub_82DF3A08(ctx, base);
	// 827FE534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE538: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FE53C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827FE540: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FE544: 4BAE8E4D  bl 0x822e7390
	ctx.lr = 0x827FE548;
	sub_822E7390(ctx, base);
	// 827FE548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE54C: 485F4EDD  bl 0x82df3428
	ctx.lr = 0x827FE550;
	sub_82DF3428(ctx, base);
	// 827FE550: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FE554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE558: 419A0054  beq cr6, 0x827fe5ac
	if ctx.cr[6].eq {
	pc = 0x827FE5AC; continue 'dispatch;
	}
	// 827FE55C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FE560: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 827FE564: 409A0008  bne cr6, 0x827fe56c
	if !ctx.cr[6].eq {
	pc = 0x827FE56C; continue 'dispatch;
	}
	// 827FE568: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827FE56C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FE570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE574: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827FE578: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FE57C: 4E800421  bctrl
	ctx.lr = 0x827FE580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FE580: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE584: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE588: 485F5481  bl 0x82df3a08
	ctx.lr = 0x827FE58C;
	sub_82DF3A08(ctx, base);
	// 827FE58C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827FE590: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827FE594: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827FE598: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE59C: 4BAF013D  bl 0x822ee6d8
	ctx.lr = 0x827FE5A0;
	sub_822EE6D8(ctx, base);
	// 827FE5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827FE5A4: 485F4E85  bl 0x82df3428
	ctx.lr = 0x827FE5A8;
	sub_82DF3428(ctx, base);
	// 827FE5A8: 48000010  b 0x827fe5b8
	pc = 0x827FE5B8; continue 'dispatch;
	// 827FE5AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827FE5B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE5B4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827FE5B8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FE5BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE5C0: 419A0008  beq cr6, 0x827fe5c8
	if ctx.cr[6].eq {
	pc = 0x827FE5C8; continue 'dispatch;
	}
	// 827FE5C4: 4BAC22CD  bl 0x822c0890
	ctx.lr = 0x827FE5C8;
	sub_822C0890(ctx, base);
	// 827FE5C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE5CC: 4BAE8C55  bl 0x822e7220
	ctx.lr = 0x827FE5D0;
	sub_822E7220(ctx, base);
	// 827FE5D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE5D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827FE5D8: 489A9BE4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FE5E0 size=352
    let mut pc: u32 = 0x827FE5E0;
    'dispatch: loop {
        match pc {
            0x827FE5E0 => {
    //   block [0x827FE5E0..0x827FE740)
	// 827FE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE5E4: 489A9B85  bl 0x831a8168
	ctx.lr = 0x827FE5E8;
	sub_831A8130(ctx, base);
	// 827FE5E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE5EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE5F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FE5F4: 809D00C0  lwz r4, 0xc0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 827FE5F8: 4BDE0479  bl 0x825dea70
	ctx.lr = 0x827FE5FC;
	sub_825DEA70(ctx, base);
	// 827FE5FC: 3BFD00D0  addi r31, r29, 0xd0
	ctx.r[31].s64 = ctx.r[29].s64 + 208;
	// 827FE600: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE608: 488B3A21  bl 0x830b2028
	ctx.lr = 0x827FE60C;
	sub_830B2028(ctx, base);
	// 827FE60C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE610: 488B3971  bl 0x830b1f80
	ctx.lr = 0x827FE614;
	sub_830B1F80(ctx, base);
	// 827FE614: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FE618: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FE61C: 488B38BD  bl 0x830b1ed8
	ctx.lr = 0x827FE620;
	sub_830B1ED8(ctx, base);
	// 827FE620: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827FE624: 419A0114  beq cr6, 0x827fe738
	if ctx.cr[6].eq {
	pc = 0x827FE738; continue 'dispatch;
	}
	// 827FE628: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE630: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FE634: 38A00224  li r5, 0x224
	ctx.r[5].s64 = 548;
	// 827FE638: 386000D8  li r3, 0xd8
	ctx.r[3].s64 = 216;
	// 827FE63C: 485F3DAD  bl 0x82df23e8
	ctx.lr = 0x827FE640;
	sub_82DF23E8(ctx, base);
	// 827FE640: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827FE644: 4182002C  beq 0x827fe670
	if ctx.cr[0].eq {
	pc = 0x827FE670; continue 'dispatch;
	}
	// 827FE648: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE64C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827FE650: 4BC9B761  bl 0x82499db0
	ctx.lr = 0x827FE654;
	sub_82499DB0(ctx, base);
	// 827FE654: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FE658: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE65C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE660: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FE664: 4BDE1A6D  bl 0x825e00d0
	ctx.lr = 0x827FE668;
	sub_825E00D0(ctx, base);
	// 827FE668: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE66C: 48000008  b 0x827fe674
	pc = 0x827FE674; continue 'dispatch;
	// 827FE670: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE674: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827FE678: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE67C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FE680: 4BB0EEA1  bl 0x8230d520
	ctx.lr = 0x827FE684;
	sub_8230D520(ctx, base);
	// 827FE684: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE688: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE68C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FE690: 4BAC1971  bl 0x822c0000
	ctx.lr = 0x827FE694;
	sub_822C0000(ctx, base);
	// 827FE694: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FE698: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827FE69C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827FE6A0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 827FE6A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827FE6A8: 419A0024  beq cr6, 0x827fe6cc
	if ctx.cr[6].eq {
	pc = 0x827FE6CC; continue 'dispatch;
	}
	// 827FE6AC: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 827FE6B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827FE6B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FE6B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827FE6BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827FE6C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827FE6C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FE6C8: 4082FFE8  bne 0x827fe6b0
	if !ctx.cr[0].eq {
	pc = 0x827FE6B0; continue 'dispatch;
	}
	// 827FE6CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FE6D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE6D4: 4BD10DF5  bl 0x8250f4c8
	ctx.lr = 0x827FE6D8;
	sub_8250F4C8(ctx, base);
	// 827FE6D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FE6DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE6E0: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 827FE6E4: 409A0008  bne cr6, 0x827fe6ec
	if !ctx.cr[6].eq {
	pc = 0x827FE6EC; continue 'dispatch;
	}
	// 827FE6E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE6EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FE6F0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827FE6F4: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 827FE6F8: 4BD10E21  bl 0x8250f518
	ctx.lr = 0x827FE6FC;
	sub_8250F518(ctx, base);
	// 827FE6FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FE700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE704: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827FE708: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827FE70C: 4BD0EDFD  bl 0x8250d508
	ctx.lr = 0x827FE710;
	sub_8250D508(ctx, base);
	// 827FE710: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FE714: 485F357D  bl 0x82df1c90
	ctx.lr = 0x827FE718;
	sub_82DF1C90(ctx, base);
	// 827FE718: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827FE71C: 485F3575  bl 0x82df1c90
	ctx.lr = 0x827FE720;
	sub_82DF1C90(ctx, base);
	// 827FE720: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE724: 4BFFEF75  bl 0x827fd698
	ctx.lr = 0x827FE728;
	sub_827FD698(ctx, base);
	// 827FE728: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827FE72C: 419A000C  beq cr6, 0x827fe738
	if ctx.cr[6].eq {
	pc = 0x827FE738; continue 'dispatch;
	}
	// 827FE730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE734: 4BAC215D  bl 0x822c0890
	ctx.lr = 0x827FE738;
	sub_822C0890(ctx, base);
	// 827FE738: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827FE73C: 489A9A7C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE740 size=128
    let mut pc: u32 = 0x827FE740;
    'dispatch: loop {
        match pc {
            0x827FE740 => {
    //   block [0x827FE740..0x827FE7C0)
	// 827FE740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE744: 489A9A29  bl 0x831a816c
	ctx.lr = 0x827FE748;
	sub_831A8130(ctx, base);
	// 827FE748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE74C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FE750: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE754: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE758: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FE75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE760: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 827FE764: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 827FE768: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 827FE76C: 485F3C7D  bl 0x82df23e8
	ctx.lr = 0x827FE770;
	sub_82DF23E8(ctx, base);
	// 827FE770: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FE774: 41820018  beq 0x827fe78c
	if ctx.cr[0].eq {
	pc = 0x827FE78C; continue 'dispatch;
	}
	// 827FE778: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827FE77C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE780: 48354759  bl 0x82b52ed8
	ctx.lr = 0x827FE784;
	sub_82B52ED8(ctx, base);
	// 827FE784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE788: 48000008  b 0x827fe790
	pc = 0x827FE790; continue 'dispatch;
	// 827FE78C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE790: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE794: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FE798: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE79C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE7A0: 4BFFF389  bl 0x827fdb28
	ctx.lr = 0x827FE7A4;
	sub_827FDB28(ctx, base);
	// 827FE7A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE7A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE7AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE7B0: 4BAC1851  bl 0x822c0000
	ctx.lr = 0x827FE7B4;
	sub_822C0000(ctx, base);
	// 827FE7B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE7B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE7BC: 489A9A00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE7C0 size=124
    let mut pc: u32 = 0x827FE7C0;
    'dispatch: loop {
        match pc {
            0x827FE7C0 => {
    //   block [0x827FE7C0..0x827FE83C)
	// 827FE7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE7C4: 489A99A9  bl 0x831a816c
	ctx.lr = 0x827FE7C8;
	sub_831A8130(ctx, base);
	// 827FE7C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE7CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FE7D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE7D8: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 827FE7DC: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 827FE7E0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 827FE7E4: 485F3C05  bl 0x82df23e8
	ctx.lr = 0x827FE7E8;
	sub_82DF23E8(ctx, base);
	// 827FE7E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827FE7EC: 4182001C  beq 0x827fe808
	if ctx.cr[0].eq {
	pc = 0x827FE808; continue 'dispatch;
	}
	// 827FE7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE7F4: 4865AE8D  bl 0x82e59680
	ctx.lr = 0x827FE7F8;
	sub_82E59680(ctx, base);
	// 827FE7F8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE7FC: 396B70AC  addi r11, r11, 0x70ac
	ctx.r[11].s64 = ctx.r[11].s64 + 28844;
	// 827FE800: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE804: 48000008  b 0x827fe80c
	pc = 0x827FE80C; continue 'dispatch;
	// 827FE808: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE80C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE810: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827FE814: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE81C: 4BFFF3D5  bl 0x827fdbf0
	ctx.lr = 0x827FE820;
	sub_827FDBF0(ctx, base);
	// 827FE820: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE824: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE82C: 4BAC17D5  bl 0x822c0000
	ctx.lr = 0x827FE830;
	sub_822C0000(ctx, base);
	// 827FE830: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827FE834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827FE838: 489A9984  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FE840 size=136
    let mut pc: u32 = 0x827FE840;
    'dispatch: loop {
        match pc {
            0x827FE840 => {
    //   block [0x827FE840..0x827FE8C8)
	// 827FE840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE844: 489A9925  bl 0x831a8168
	ctx.lr = 0x827FE848;
	sub_831A8130(ctx, base);
	// 827FE848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE84C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE850: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827FE854: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827FE858: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827FE85C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 827FE860: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE864: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FE868: 38A00063  li r5, 0x63
	ctx.r[5].s64 = 99;
	// 827FE86C: 38600198  li r3, 0x198
	ctx.r[3].s64 = 408;
	// 827FE870: 485F3B79  bl 0x82df23e8
	ctx.lr = 0x827FE874;
	sub_82DF23E8(ctx, base);
	// 827FE874: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FE878: 4182001C  beq 0x827fe894
	if ctx.cr[0].eq {
	pc = 0x827FE894; continue 'dispatch;
	}
	// 827FE87C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827FE880: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827FE884: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE888: 4BFFF779  bl 0x827fe000
	ctx.lr = 0x827FE88C;
	sub_827FE000(ctx, base);
	// 827FE88C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FE890: 48000008  b 0x827fe898
	pc = 0x827FE898; continue 'dispatch;
	// 827FE894: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE898: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827FE89C: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 827FE8A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE8A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE8A8: 4BFFEE99  bl 0x827fd740
	ctx.lr = 0x827FE8AC;
	sub_827FD740(ctx, base);
	// 827FE8AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE8B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE8B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE8B8: 4BAC1749  bl 0x822c0000
	ctx.lr = 0x827FE8BC;
	sub_822C0000(ctx, base);
	// 827FE8BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827FE8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FE8C4: 489A98F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FE8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FE8C8 size=484
    let mut pc: u32 = 0x827FE8C8;
    'dispatch: loop {
        match pc {
            0x827FE8C8 => {
    //   block [0x827FE8C8..0x827FEAAC)
	// 827FE8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FE8CC: 489A989D  bl 0x831a8168
	ctx.lr = 0x827FE8D0;
	sub_831A8130(ctx, base);
	// 827FE8D0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827FE8D4: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FE8D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827FE8DC: 4BFFAC55  bl 0x827f9530
	ctx.lr = 0x827FE8E0;
	sub_827F9530(ctx, base);
	// 827FE8E0: 817D0120  lwz r11, 0x120(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(288 as u32) ) } as u64;
	// 827FE8E4: 3BDD0120  addi r30, r29, 0x120
	ctx.r[30].s64 = ctx.r[29].s64 + 288;
	// 827FE8E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FE8EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827FE8F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827FE8F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827FE8F8: 4E800421  bctrl
	ctx.lr = 0x827FE8FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827FE8FC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE904: 3B8B7160  addi r28, r11, 0x7160
	ctx.r[28].s64 = ctx.r[11].s64 + 29024;
	// 827FE908: 38A00081  li r5, 0x81
	ctx.r[5].s64 = 129;
	// 827FE90C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827FE910: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 827FE914: 485F3AD5  bl 0x82df23e8
	ctx.lr = 0x827FE918;
	sub_82DF23E8(ctx, base);
	// 827FE918: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827FE91C: 4182001C  beq 0x827fe938
	if ctx.cr[0].eq {
	pc = 0x827FE938; continue 'dispatch;
	}
	// 827FE920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE924: 4801605D  bl 0x82814980
	ctx.lr = 0x827FE928;
	sub_82814980(ctx, base);
	// 827FE928: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FE92C: 396B7124  addi r11, r11, 0x7124
	ctx.r[11].s64 = ctx.r[11].s64 + 28964;
	// 827FE930: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827FE934: 48000008  b 0x827fe93c
	pc = 0x827FE93C; continue 'dispatch;
	// 827FE938: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827FE93C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827FE940: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE944: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FE948: 4BFFEEC1  bl 0x827fd808
	ctx.lr = 0x827FE94C;
	sub_827FD808(ctx, base);
	// 827FE94C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827FE950: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FE954: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827FE958: 4BAC16A9  bl 0x822c0000
	ctx.lr = 0x827FE95C;
	sub_822C0000(ctx, base);
	// 827FE95C: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FE960: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827FE964: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FE968: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 827FE96C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827FE970: 419A0024  beq cr6, 0x827fe994
	if ctx.cr[6].eq {
	pc = 0x827FE994; continue 'dispatch;
	}
	// 827FE974: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827FE978: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827FE97C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FE980: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827FE984: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827FE988: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827FE98C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FE990: 4082FFE8  bne 0x827fe978
	if !ctx.cr[0].eq {
	pc = 0x827FE978; continue 'dispatch;
	}
	// 827FE994: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827FE998: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827FE99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FE9A0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 827FE9A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827FE9A8: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827FE9AC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827FE9B0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827FE9B4: 4865FCF5  bl 0x82e5e6a8
	ctx.lr = 0x827FE9B8;
	sub_82E5E6A8(ctx, base);
	// 827FE9B8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827FE9BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE9C0: 419A0008  beq cr6, 0x827fe9c8
	if ctx.cr[6].eq {
	pc = 0x827FE9C8; continue 'dispatch;
	}
	// 827FE9C4: 4BAC1ECD  bl 0x822c0890
	ctx.lr = 0x827FE9C8;
	sub_822C0890(ctx, base);
	// 827FE9C8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FE9CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FE9D0: 419A0008  beq cr6, 0x827fe9d8
	if ctx.cr[6].eq {
	pc = 0x827FE9D8; continue 'dispatch;
	}
	// 827FE9D4: 4BAC1EBD  bl 0x822c0890
	ctx.lr = 0x827FE9D8;
	sub_822C0890(ctx, base);
	// 827FE9D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FE9DC: 419A000C  beq cr6, 0x827fe9e8
	if ctx.cr[6].eq {
	pc = 0x827FE9E8; continue 'dispatch;
	}
	// 827FE9E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FE9E4: 4BAC1EAD  bl 0x822c0890
	ctx.lr = 0x827FE9E8;
	sub_822C0890(ctx, base);
	// 827FE9E8: 817D0180  lwz r11, 0x180(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(384 as u32) ) } as u64;
	// 827FE9EC: 3BFD0180  addi r31, r29, 0x180
	ctx.r[31].s64 = ctx.r[29].s64 + 384;
	// 827FE9F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FE9F4: 409A0078  bne cr6, 0x827fea6c
	if !ctx.cr[6].eq {
	pc = 0x827FEA6C; continue 'dispatch;
	}
	// 827FE9F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FE9FC: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 827FEA00: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 827FEA04: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827FEA08: C06B9534  lfs f3, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 827FEA0C: 4867D60D  bl 0x82e7c018
	ctx.lr = 0x827FEA10;
	sub_82E7C018(ctx, base);
	// 827FEA10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827FEA14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827FEA18: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 827FEA1C: C02BD96C  lfs f1, -0x2694(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9876 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827FEA20: 4867DFC9  bl 0x82e7c9e8
	ctx.lr = 0x827FEA24;
	sub_82E7C9E8(ctx, base);
	// 827FEA24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FEA28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827FEA2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827FEA30: 4BAC5ED1  bl 0x822c4900
	ctx.lr = 0x827FEA34;
	sub_822C4900(ctx, base);
	// 827FEA34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827FEA38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827FEA3C: 38A00087  li r5, 0x87
	ctx.r[5].s64 = 135;
	// 827FEA40: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 827FEA44: 485F39A5  bl 0x82df23e8
	ctx.lr = 0x827FEA48;
	sub_82DF23E8(ctx, base);
	// 827FEA48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827FEA4C: 41820014  beq 0x827fea60
	if ctx.cr[0].eq {
	pc = 0x827FEA60; continue 'dispatch;
	}
	// 827FEA50: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 827FEA54: 4861469D  bl 0x82e130f0
	ctx.lr = 0x827FEA58;
	sub_82E130F0(ctx, base);
	// 827FEA58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827FEA5C: 48000008  b 0x827fea64
	pc = 0x827FEA64; continue 'dispatch;
	// 827FEA60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827FEA64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FEA68: 4BAE32E9  bl 0x822e1d50
	ctx.lr = 0x827FEA6C;
	sub_822E1D50(ctx, base);
	// 827FEA6C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FEA70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827FEA74: 419A002C  beq cr6, 0x827feaa0
	if ctx.cr[6].eq {
	pc = 0x827FEAA0; continue 'dispatch;
	}
	// 827FEA78: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827FEA7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827FEA80: 482AD269  bl 0x82aabce8
	ctx.lr = 0x827FEA84;
	sub_82AABCE8(ctx, base);
	// 827FEA84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827FEA88: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FEA8C: 486143A5  bl 0x82e12e30
	ctx.lr = 0x827FEA90;
	sub_82E12E30(ctx, base);
	// 827FEA90: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827FEA94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FEA98: 419A0008  beq cr6, 0x827feaa0
	if ctx.cr[6].eq {
	pc = 0x827FEAA0; continue 'dispatch;
	}
	// 827FEA9C: 4BAC1DF5  bl 0x822c0890
	ctx.lr = 0x827FEAA0;
	sub_822C0890(ctx, base);
	// 827FEAA0: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 827FEAA4: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827FEAA8: 489A9710  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FEAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827FEAB0 size=200
    let mut pc: u32 = 0x827FEAB0;
    'dispatch: loop {
        match pc {
            0x827FEAB0 => {
    //   block [0x827FEAB0..0x827FEB78)
	// 827FEAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FEAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FEAB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FEABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FEAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FEAC4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827FEAC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827FEACC: 388B6910  addi r4, r11, 0x6910
	ctx.r[4].s64 = ctx.r[11].s64 + 26896;
	// 827FEAD0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827FEAD4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 827FEAD8: 4BFFFC69  bl 0x827fe740
	ctx.lr = 0x827FEADC;
	sub_827FE740(ctx, base);
	// 827FEADC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827FEAE0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827FEAE4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827FEAE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827FEAEC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827FEAF0: 419A0024  beq cr6, 0x827feb14
	if ctx.cr[6].eq {
	pc = 0x827FEB14; continue 'dispatch;
	}
	// 827FEAF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827FEAF8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827FEAFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FEB00: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827FEB04: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827FEB08: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827FEB0C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827FEB10: 4082FFE8  bne 0x827feaf8
	if !ctx.cr[0].eq {
	pc = 0x827FEAF8; continue 'dispatch;
	}
	// 827FEB14: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 827FEB18: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 827FEB1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FEB20: 4880A499  bl 0x83008fb8
	ctx.lr = 0x827FEB24;
	sub_83008FB8(ctx, base);
	// 827FEB24: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827FEB28: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827FEB2C: 388B7160  addi r4, r11, 0x7160
	ctx.r[4].s64 = ctx.r[11].s64 + 29024;
	// 827FEB30: 38A00110  li r5, 0x110
	ctx.r[5].s64 = 272;
	// 827FEB34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827FEB38: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 827FEB3C: 486584AD  bl 0x82e56fe8
	ctx.lr = 0x827FEB40;
	sub_82E56FE8(ctx, base);
	// 827FEB40: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827FEB44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FEB48: 419A0008  beq cr6, 0x827feb50
	if ctx.cr[6].eq {
	pc = 0x827FEB50; continue 'dispatch;
	}
	// 827FEB4C: 4BAC1D45  bl 0x822c0890
	ctx.lr = 0x827FEB50;
	sub_822C0890(ctx, base);
	// 827FEB50: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827FEB54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827FEB58: 419A0008  beq cr6, 0x827feb60
	if ctx.cr[6].eq {
	pc = 0x827FEB60; continue 'dispatch;
	}
	// 827FEB5C: 4BAC1D35  bl 0x822c0890
	ctx.lr = 0x827FEB60;
	sub_822C0890(ctx, base);
	// 827FEB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827FEB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827FEB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827FEB6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827FEB70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827FEB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827FEB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827FEB78 size=424
    let mut pc: u32 = 0x827FEB78;
    'dispatch: loop {
        match pc {
            0x827FEB78 => {
    //   block [0x827FEB78..0x827FED20)
	// 827FEB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827FEB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827FEB80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827FEB84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827FEB88: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827FEB8C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827FEB90: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 827FEB94: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 827FEB98: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 827FEB9C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 827FEBA0: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 827FEBA4: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 827FEBA8: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 827FEBAC: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FEBB0: 3BC100B0  addi r30, r1, 0xb0
	ctx.r[30].s64 = ctx.r[1].s64 + 176;
	// 827FEBB4: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FEBB8: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FEBBC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 827FEBC0: 13885C07  vcmpneb. (lvlx128) v28, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827FEBC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


