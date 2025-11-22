pub fn sub_82E7CDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7CDC0 size=72
    let mut pc: u32 = 0x82E7CDC0;
    'dispatch: loop {
        match pc {
            0x82E7CDC0 => {
    //   block [0x82E7CDC0..0x82E7CE08)
	// 82E7CDC0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7CE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7CE08 size=88
    let mut pc: u32 = 0x82E7CE08;
    'dispatch: loop {
        match pc {
            0x82E7CE08 => {
    //   block [0x82E7CE08..0x82E7CE60)
	// 82E7CE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7CE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7CE10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7CE14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7CE18: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E7CE1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7CE20: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E7CE24: E88B0008  ld r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82E7CE28: E8AB0010  ld r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82E7CE2C: E8CB0018  ld r6, 0x18(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82E7CE30: E8EB0020  ld r7, 0x20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82E7CE34: E90B0028  ld r8, 0x28(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82E7CE38: E92B0030  ld r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82E7CE3C: E94B0038  ld r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82E7CE40: 4BD92B79  bl 0x82c0f9b8
	ctx.lr = 0x82E7CE44;
	sub_82C0F9B8(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7CE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7CE60 size=192
    let mut pc: u32 = 0x82E7CE60;
    'dispatch: loop {
        match pc {
            0x82E7CE60 => {
    //   block [0x82E7CE60..0x82E7CF20)
	// 82E7CE60: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7CF20 size=76
    let mut pc: u32 = 0x82E7CF20;
    'dispatch: loop {
        match pc {
            0x82E7CF20 => {
    //   block [0x82E7CF20..0x82E7CF6C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7CF70 size=240
    let mut pc: u32 = 0x82E7CF70;
    'dispatch: loop {
        match pc {
            0x82E7CF70 => {
    //   block [0x82E7CF70..0x82E7D060)
	// 82E7CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7CF78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7CF7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7CF80: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7CF84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7CF88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7CF8C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E7CF90: 480004C1  bl 0x82e7d450
	ctx.lr = 0x82E7CF94;
	sub_82E7D450(ctx, base);
	// 82E7CF94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E7CF98: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E7CF9C: 480004B5  bl 0x82e7d450
	ctx.lr = 0x82E7CFA0;
	sub_82E7D450(ctx, base);
	// 82E7CFA0: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E7CFA4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E7CFA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7CFAC: 48000375  bl 0x82e7d320
	ctx.lr = 0x82E7CFB0;
	sub_82E7D320(ctx, base);
	// 82E7CFB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7CFB4: 480004DD  bl 0x82e7d490
	ctx.lr = 0x82E7CFB8;
	sub_82E7D490(ctx, base);
	// 82E7CFB8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E7CFBC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E7CFC0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82E7CFC4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82E7CFC8: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7CFCC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7D060 size=280
    let mut pc: u32 = 0x82E7D060;
    'dispatch: loop {
        match pc {
            0x82E7D060 => {
    //   block [0x82E7D060..0x82E7D178)
	// 82E7D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7D064: 4832B109  bl 0x831a816c
	ctx.lr = 0x82E7D068;
	sub_831A8130(ctx, base);
	// 82E7D068: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E7D06C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E7D070: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7D074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7D078: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7D07C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E7D080: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E7D084: 480003CD  bl 0x82e7d450
	ctx.lr = 0x82E7D088;
	sub_82E7D450(ctx, base);
	// 82E7D088: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E7D08C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E7D090: 480003C1  bl 0x82e7d450
	ctx.lr = 0x82E7D094;
	sub_82E7D450(ctx, base);
	// 82E7D094: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E7D098: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E7D09C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7D0A0: 48000281  bl 0x82e7d320
	ctx.lr = 0x82E7D0A4;
	sub_82E7D320(ctx, base);
	// 82E7D0A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7D0A8: 480003E9  bl 0x82e7d490
	ctx.lr = 0x82E7D0AC;
	sub_82E7D490(ctx, base);
	// 82E7D0AC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E7D0B0: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E7D0B4: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82E7D0B8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82E7D0BC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D0C0: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7D178 size=332
    let mut pc: u32 = 0x82E7D178;
    'dispatch: loop {
        match pc {
            0x82E7D178 => {
    //   block [0x82E7D178..0x82E7D2C4)
	// 82E7D178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7D17C: 4832AFF1  bl 0x831a816c
	ctx.lr = 0x82E7D180;
	sub_831A8130(ctx, base);
	// 82E7D180: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E7D184: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E7D188: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7D18C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E7D190: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E7D194: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7D198: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E7D19C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7D1A0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82E7D1A4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E7D1A8: 48000179  bl 0x82e7d320
	ctx.lr = 0x82E7D1AC;
	sub_82E7D320(ctx, base);
	// 82E7D1AC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E7D1B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7D1B4: 4800029D  bl 0x82e7d450
	ctx.lr = 0x82E7D1B8;
	sub_82E7D450(ctx, base);
	// 82E7D1B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E7D1BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7D1C0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E7D1C4: 4800015D  bl 0x82e7d320
	ctx.lr = 0x82E7D1C8;
	sub_82E7D320(ctx, base);
	// 82E7D1C8: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82E7D1CC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E7D1D0: 48000281  bl 0x82e7d450
	ctx.lr = 0x82E7D1D4;
	sub_82E7D450(ctx, base);
	// 82E7D1D4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E7D1D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E7D1DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7D1E0: 48000141  bl 0x82e7d320
	ctx.lr = 0x82E7D1E4;
	sub_82E7D320(ctx, base);
	// 82E7D1E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7D1E8: 480002A9  bl 0x82e7d490
	ctx.lr = 0x82E7D1EC;
	sub_82E7D490(ctx, base);
	// 82E7D1EC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82E7D1F0: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E7D1F4: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82E7D1F8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82E7D1FC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D200: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7D2C8 size=84
    let mut pc: u32 = 0x82E7D2C8;
    'dispatch: loop {
        match pc {
            0x82E7D2C8 => {
    //   block [0x82E7D2C8..0x82E7D31C)
	// 82E7D2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7D2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7D2D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7D2D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7D2D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E7D2DC: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E7D2E0: D0210054  stfs f1, 0x54(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E7D2E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7D2E8: D0210058  stfs f1, 0x58(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E7D2EC: D021005C  stfs f1, 0x5c(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D320 size=76
    let mut pc: u32 = 0x82E7D320;
    'dispatch: loop {
        match pc {
            0x82E7D320 => {
    //   block [0x82E7D320..0x82E7D36C)
	// 82E7D320: C1650004  lfs f11, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E7D324: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7D328: C1440008  lfs f10, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E7D32C: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D330: ED0A02F2  fmuls f8, f10, f11
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E7D334: C1250000  lfs f9, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E7D338: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D33C: ECC90332  fmuls f6, f9, f12
	ctx.f[6].f64 = (((ctx.f[9].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E7D340: C0E40000  lfs f7, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E7D344: ECA70372  fmuls f5, f7, f13
	ctx.f[5].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E7D348: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D34C: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D350: EC0C4378  fmsubs f0, f12, f13, f8
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[8].f64) as f32) as f64);
	// 82E7D354: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D358: EC0732F8  fmsubs f0, f7, f11, f6
	ctx.f[0].f64 = (((ctx.f[7].f64 * ctx.f[11].f64 - ctx.f[6].f64) as f32) as f64);
	// 82E7D35C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D360: EC092AB8  fmsubs f0, f9, f10, f5
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[10].f64 - ctx.f[5].f64) as f32) as f64);
	// 82E7D364: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D370 size=28
    let mut pc: u32 = 0x82E7D370;
    'dispatch: loop {
        match pc {
            0x82E7D370 => {
    //   block [0x82E7D370..0x82E7D38C)
	// 82E7D370: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7D374: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D378: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D37C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D380: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D384: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7D390 size=12
    let mut pc: u32 = 0x82E7D390;
    'dispatch: loop {
        match pc {
            0x82E7D390 => {
    //   block [0x82E7D390..0x82E7D39C)
	// 82E7D390: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7D394: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E7D398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7D3A0 size=172
    let mut pc: u32 = 0x82E7D3A0;
    'dispatch: loop {
        match pc {
            0x82E7D3A0 => {
    //   block [0x82E7D3A0..0x82E7D44C)
	// 82E7D3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7D3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7D3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7D3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7D3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7D3B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7D3B8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7D3BC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E7D3C0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E7D3C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E7D3C8: C19E0008  lfs f12, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D3CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7D3D0: C00BDFAC  lfs f0, -0x2054(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D3D4: C1AA9450  lfs f13, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D3D8: EC20637A  fmadds f1, f0, f13, f12
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E7D3DC: 4B5D394D  bl 0x82450d28
	ctx.lr = 0x82E7D3E0;
	sub_82450D28(ctx, base);
	// 82E7D3E0: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82E7D3E4: C03E0004  lfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7D3E8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E7D3EC: 4B5D393D  bl 0x82450d28
	ctx.lr = 0x82E7D3F0;
	sub_82450D28(ctx, base);
	// 82E7D3F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7D3F4: C1810054  lfs f12, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7D3FC: C1610058  lfs f11, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E7D400: C141005C  lfs f10, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E7D404: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D408: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D40C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D410: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D414: ED800332  fmuls f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E7D418: D19F0004  stfs f12, 4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D41C: ED6002F2  fmuls f11, f0, f11
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E7D420: EC0002B2  fmuls f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 82E7D424: ED8B0372  fmuls f12, f11, f13
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E7D428: D19F0000  stfs f12, 0(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D42C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E7D430: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7D438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7D43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7D440: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7D444: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7D448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D450 size=64
    let mut pc: u32 = 0x82E7D450;
    'dispatch: loop {
        match pc {
            0x82E7D450 => {
    //   block [0x82E7D450..0x82E7D490)
	// 82E7D450: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D490 size=116
    let mut pc: u32 = 0x82E7D490;
    'dispatch: loop {
        match pc {
            0x82E7D490 => {
    //   block [0x82E7D490..0x82E7D504)
	// 82E7D490: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D508 size=32
    let mut pc: u32 = 0x82E7D508;
    'dispatch: loop {
        match pc {
            0x82E7D508 => {
    //   block [0x82E7D508..0x82E7D528)
	// 82E7D508: 3961001C  addi r11, r1, 0x1c
	ctx.r[11].s64 = ctx.r[1].s64 + 28;
	// 82E7D50C: D021001C  stfs f1, 0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E7D510: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D514: 13C05C07  vcmpneb. (lvlx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D528 size=20
    let mut pc: u32 = 0x82E7D528;
    'dispatch: loop {
        match pc {
            0x82E7D528 => {
    //   block [0x82E7D528..0x82E7D53C)
	// 82E7D528: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D52C: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7D540 size=252
    let mut pc: u32 = 0x82E7D540;
    'dispatch: loop {
        match pc {
            0x82E7D540 => {
    //   block [0x82E7D540..0x82E7D63C)
	// 82E7D540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7D544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7D548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7D54C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7D550: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82E7D554: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E7D558: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7D55C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D640 size=68
    let mut pc: u32 = 0x82E7D640;
    'dispatch: loop {
        match pc {
            0x82E7D640 => {
    //   block [0x82E7D640..0x82E7D684)
	// 82E7D640: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E7D644: D021002C  stfs f1, 0x2c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E7D648: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82E7D64C: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D650: 3961002C  addi r11, r1, 0x2c
	ctx.r[11].s64 = ctx.r[1].s64 + 44;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D688 size=76
    let mut pc: u32 = 0x82E7D688;
    'dispatch: loop {
        match pc {
            0x82E7D688 => {
    //   block [0x82E7D688..0x82E7D6D4)
	// 82E7D688: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D68C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82E7D690: 13C020C7  vcmpequd (lvx128) v30, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7D694: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D6D8 size=132
    let mut pc: u32 = 0x82E7D6D8;
    'dispatch: loop {
        match pc {
            0x82E7D6D8 => {
    //   block [0x82E7D6D8..0x82E7D75C)
	// 82E7D6D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7D6DC: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7D6E0: 55490001  rlwinm. r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E7D6E4: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7D6E8: 41820008  beq 0x82e7d6f0
	if ctx.cr[0].eq {
	pc = 0x82E7D6F0; continue 'dispatch;
	}
	// 82E7D6EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E7D6F0: 55490043  rlwinm. r9, r10, 0, 1, 1
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E7D6F4: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E7D6F8: 41820008  beq 0x82e7d700
	if ctx.cr[0].eq {
	pc = 0x82E7D700; continue 'dispatch;
	}
	// 82E7D6FC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E7D700: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E7D704: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7D708: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E7D70C: 55660001  rlwinm. r6, r11, 0, 0, 0
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82E7D710: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E7D714: 4182000C  beq 0x82e7d720
	if ctx.cr[0].eq {
	pc = 0x82E7D720; continue 'dispatch;
	}
	// 82E7D718: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E7D71C: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E7D720: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E7D724: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82E7D728: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E7D72C: 7D0B4A78  xor r11, r8, r9
	ctx.r[11].u64 = ctx.r[8].u64 ^ ctx.r[9].u64;
	// 82E7D730: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E7D734: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E7D738: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E7D73C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7D740: C006DFB4  lfs f0, -0x204c(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D744: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82E7D748: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7D74C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E7D750: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E7D754: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E7D758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7D760 size=44
    let mut pc: u32 = 0x82E7D760;
    'dispatch: loop {
        match pc {
            0x82E7D760 => {
    //   block [0x82E7D760..0x82E7D78C)
	// 82E7D760: 6C8B3CC1  xoris r11, r4, 0x3cc1
	ctx.r[11].u64 = ctx.r[4].u64 ^ 1019281408;
	// 82E7D764: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E7D768: 6C8A01C9  xoris r10, r4, 0x1c9
	ctx.r[10].u64 = ctx.r[4].u64 ^ 29949952;
	// 82E7D76C: 6C892E43  xoris r9, r4, 0x2e43
	ctx.r[9].u64 = ctx.r[4].u64 ^ 776142848;
	// 82E7D770: 696B9185  xori r11, r11, 0x9185
	ctx.r[11].u64 = ctx.r[11].u64 ^ 37253;
	// 82E7D774: 694AC2D0  xori r10, r10, 0xc2d0
	ctx.r[10].u64 = ctx.r[10].u64 ^ 49872;
	// 82E7D778: 6929C13E  xori r9, r9, 0xc13e
	ctx.r[9].u64 = ctx.r[9].u64 ^ 49470;
	// 82E7D77C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E7D780: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E7D784: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E7D788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D790 size=60
    let mut pc: u32 = 0x82E7D790;
    'dispatch: loop {
        match pc {
            0x82E7D790 => {
    //   block [0x82E7D790..0x82E7D7CC)
	// 82E7D790: 7C8B1E70  srawi r11, r4, 3
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 3) as i64;
	// 82E7D794: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7D798: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82E7D79C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82E7D7A0: 7C881E70  srawi r8, r4, 3
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[4].s32 >> 3) as i64;
	// 82E7D7A4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82E7D7A8: 55081838  slwi r8, r8, 3
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E7D7AC: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E7D7B0: 7D482050  subf r10, r8, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 82E7D7B4: 7D2A5630  sraw r10, r9, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> tmp.u32) as i64;
	// 82E7D7B8: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82E7D7BC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E7D7C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E7D7C4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82E7D7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7D7D0 size=12
    let mut pc: u32 = 0x82E7D7D0;
    'dispatch: loop {
        match pc {
            0x82E7D7D0 => {
    //   block [0x82E7D7D0..0x82E7D7DC)
	// 82E7D7D0: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E7D7D4: B0A30000  sth r5, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 82E7D7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D7E0 size=36
    let mut pc: u32 = 0x82E7D7E0;
    'dispatch: loop {
        match pc {
            0x82E7D7E0 => {
    //   block [0x82E7D7E0..0x82E7D804)
	// 82E7D7E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7D7E4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D7E8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D7EC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D7F0: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D7F4: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D7F8: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E7D7FC: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E7D800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D808 size=52
    let mut pc: u32 = 0x82E7D808;
    'dispatch: loop {
        match pc {
            0x82E7D808 => {
    //   block [0x82E7D808..0x82E7D83C)
	// 82E7D808: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D80C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D810: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D814: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D818: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D81C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D820: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D824: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D828: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D82C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E7D830: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D834: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E7D838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D840 size=100
    let mut pc: u32 = 0x82E7D840;
    'dispatch: loop {
        match pc {
            0x82E7D840 => {
    //   block [0x82E7D840..0x82E7D8A4)
	// 82E7D840: C1840000  lfs f12, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D844: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D848: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82E7D84C: 4199004C  bgt cr6, 0x82e7d898
	if ctx.cr[6].gt {
	pc = 0x82E7D898; continue 'dispatch;
	}
	// 82E7D850: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D854: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D858: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E7D85C: 4199003C  bgt cr6, 0x82e7d898
	if ctx.cr[6].gt {
	pc = 0x82E7D898; continue 'dispatch;
	}
	// 82E7D860: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D864: C1630010  lfs f11, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E7D868: FF0B0000  fcmpu cr6, f11, f0
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[0].f64);
	// 82E7D86C: 4199002C  bgt cr6, 0x82e7d898
	if ctx.cr[6].gt {
	pc = 0x82E7D898; continue 'dispatch;
	}
	// 82E7D870: C1630004  lfs f11, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E7D874: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82E7D878: 41990020  bgt cr6, 0x82e7d898
	if ctx.cr[6].gt {
	pc = 0x82E7D898; continue 'dispatch;
	}
	// 82E7D87C: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D880: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82E7D884: 41990014  bgt cr6, 0x82e7d898
	if ctx.cr[6].gt {
	pc = 0x82E7D898; continue 'dispatch;
	}
	// 82E7D888: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D88C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E7D890: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E7D894: 40990008  ble cr6, 0x82e7d89c
	if !ctx.cr[6].gt {
	pc = 0x82E7D89C; continue 'dispatch;
	}
	// 82E7D898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7D89C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E7D8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D8A8 size=16
    let mut pc: u32 = 0x82E7D8A8;
    'dispatch: loop {
        match pc {
            0x82E7D8A8 => {
    //   block [0x82E7D8A8..0x82E7D8B8)
	// 82E7D8A8: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D8AC: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D8B0: EC206828  fsubs f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E7D8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D8B8 size=16
    let mut pc: u32 = 0x82E7D8B8;
    'dispatch: loop {
        match pc {
            0x82E7D8B8 => {
    //   block [0x82E7D8B8..0x82E7D8C8)
	// 82E7D8B8: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D8BC: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D8C0: EC206828  fsubs f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E7D8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D8C8 size=16
    let mut pc: u32 = 0x82E7D8C8;
    'dispatch: loop {
        match pc {
            0x82E7D8C8 => {
    //   block [0x82E7D8C8..0x82E7D8D8)
	// 82E7D8C8: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D8CC: C1A30010  lfs f13, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D8D0: EC206828  fsubs f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E7D8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D8D8 size=84
    let mut pc: u32 = 0x82E7D8D8;
    'dispatch: loop {
        match pc {
            0x82E7D8D8 => {
    //   block [0x82E7D8D8..0x82E7D92C)
	// 82E7D8D8: C0040004  lfs f0, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D8DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E7D8E0: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D8E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E7D8E8: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E7D8EC: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D8F0: C184000C  lfs f12, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D8F4: ED8C002A  fadds f12, f12, f0
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 82E7D8F8: C1A40014  lfs f13, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D8FC: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D900: ED4D002A  fadds f10, f13, f0
	ctx.f[10].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82E7D904: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D908: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D90C: D1A3000C  stfs f13, 0xc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D910: EDAB0032  fmuls f13, f11, f0
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E7D914: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D918: EDAC0032  fmuls f13, f12, f0
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E7D91C: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D920: EC0A0032  fmuls f0, f10, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E7D924: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D930 size=40
    let mut pc: u32 = 0x82E7D930;
    'dispatch: loop {
        match pc {
            0x82E7D930 => {
    //   block [0x82E7D930..0x82E7D958)
	// 82E7D930: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7D934: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D938: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D93C: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D940: C1840010  lfs f12, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D944: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D948: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D94C: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D950: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D958 size=28
    let mut pc: u32 = 0x82E7D958;
    'dispatch: loop {
        match pc {
            0x82E7D958 => {
    //   block [0x82E7D958..0x82E7D974)
	// 82E7D958: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D95C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D960: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D964: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D968: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D96C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E7D970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D978 size=40
    let mut pc: u32 = 0x82E7D978;
    'dispatch: loop {
        match pc {
            0x82E7D978 => {
    //   block [0x82E7D978..0x82E7D9A0)
	// 82E7D978: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7D97C: C0040004  lfs f0, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D980: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7D984: C1A4000C  lfs f13, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7D988: C1840014  lfs f12, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7D98C: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D990: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D994: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7D998: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7D9A0 size=28
    let mut pc: u32 = 0x82E7D9A0;
    'dispatch: loop {
        match pc {
            0x82E7D9A0 => {
    //   block [0x82E7D9A0..0x82E7D9BC)
	// 82E7D9A0: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D9A4: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7D9A8: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D9AC: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7D9B0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7D9B4: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E7D9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7D9C0 size=100
    let mut pc: u32 = 0x82E7D9C0;
    'dispatch: loop {
        match pc {
            0x82E7D9C0 => {
    //   block [0x82E7D9C0..0x82E7DA24)
	// 82E7D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7D9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7D9C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7D9CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7D9D0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E7D9D4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7D9D8: 3BEB6920  addi r31, r11, 0x6920
	ctx.r[31].s64 = ctx.r[11].s64 + 26912;
	// 82E7D9DC: 816A6924  lwz r11, 0x6924(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26916 as u32) ) } as u64;
	// 82E7D9E0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E7D9E4: 40820020  bne 0x82e7da04
	if !ctx.cr[0].eq {
	pc = 0x82E7DA04; continue 'dispatch;
	}
	// 82E7D9E8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E7D9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7D9F0: 916A6924  stw r11, 0x6924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26916 as u32), ctx.r[11].u32 ) };
	// 82E7D9F4: 4BF74AE5  bl 0x82df24d8
	ctx.lr = 0x82E7D9F8;
	sub_82DF24D8(ctx, base);
	// 82E7D9F8: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82E7D9FC: 386B1E88  addi r3, r11, 0x1e88
	ctx.r[3].s64 = ctx.r[11].s64 + 7816;
	// 82E7DA00: 4832AAD9  bl 0x831a84d8
	ctx.lr = 0x82E7DA04;
	sub_831A84D8(ctx, base);
	// 82E7DA04: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7DA08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7DA0C: 93EBEA00  stw r31, -0x1600(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-5632 as u32), ctx.r[31].u32 ) };
	// 82E7DA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7DA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7DA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7DA1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7DA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DA28 size=8
    let mut pc: u32 = 0x82E7DA28;
    'dispatch: loop {
        match pc {
            0x82E7DA28 => {
    //   block [0x82E7DA28..0x82E7DA30)
	// 82E7DA28: 2B060005  cmplwi cr6, r6, 5
	ctx.cr[6].compare_u32(ctx.r[6].u32, 5 as u32, &mut ctx.xer);
	// 82E7DA2C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DA30 size=24
    let mut pc: u32 = 0x82E7DA30;
    'dispatch: loop {
        match pc {
            0x82E7DA30 => {
    //   block [0x82E7DA30..0x82E7DA48)
	// 82E7DA30: 3D8082E8  lis r12, -0x7d18
	ctx.r[12].s64 = -2098724864;
	// 82E7DA34: 398CDA48  addi r12, r12, -0x25b8
	ctx.r[12].s64 = ctx.r[12].s64 + -9656;
	// 82E7DA38: 54C0103A  slwi r0, r6, 2
	ctx.r[0].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E7DA3C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E7DA40: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E7DA44: 4E800420  bctr
	match ctx.r[6].u64 {
		0 => {
			// ERROR: 0x82E7DA60
			return;
		},
		1 => {
			// ERROR: 0x82E7DA70
			return;
		},
		2 => {
			// ERROR: 0x82E7DA80
			return;
		},
		3 => {
			// ERROR: 0x82E7DA90
			return;
		},
		4 => {
			// ERROR: 0x82E7DAA0
			return;
		},
		5 => {
			// ERROR: 0x82E7DAB0
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DA48 size=40
    let mut pc: u32 = 0x82E7DA48;
    'dispatch: loop {
        match pc {
            0x82E7DA48 => {
    //   block [0x82E7DA48..0x82E7DA70)
	// 82E7DA48: 82E7DA60  lwz r23, -0x25a0(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-9632 as u32) ) } as u64;
	// 82E7DA4C: 82E7DA70  lwz r23, -0x2590(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-9616 as u32) ) } as u64;
	// 82E7DA50: 82E7DA80  lwz r23, -0x2580(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-9600 as u32) ) } as u64;
	// 82E7DA54: 82E7DA90  lwz r23, -0x2570(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-9584 as u32) ) } as u64;
	// 82E7DA58: 82E7DAA0  lwz r23, -0x2560(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-9568 as u32) ) } as u64;
	// 82E7DA5C: 82E7DAB0  lwz r23, -0x2550(r7)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-9552 as u32) ) } as u64;
	// 82E7DA60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E7DA64: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA68: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DA70 size=16
    let mut pc: u32 = 0x82E7DA70;
    'dispatch: loop {
        match pc {
            0x82E7DA70 => {
    //   block [0x82E7DA70..0x82E7DA80)
	// 82E7DA70: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82E7DA74: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA78: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DA80 size=16
    let mut pc: u32 = 0x82E7DA80;
    'dispatch: loop {
        match pc {
            0x82E7DA80 => {
    //   block [0x82E7DA80..0x82E7DA90)
	// 82E7DA80: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82E7DA84: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA88: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DA90 size=16
    let mut pc: u32 = 0x82E7DA90;
    'dispatch: loop {
        match pc {
            0x82E7DA90 => {
    //   block [0x82E7DA90..0x82E7DAA0)
	// 82E7DA90: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E7DA94: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA98: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DAA0 size=16
    let mut pc: u32 = 0x82E7DAA0;
    'dispatch: loop {
        match pc {
            0x82E7DAA0 => {
    //   block [0x82E7DAA0..0x82E7DAB0)
	// 82E7DAA0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82E7DAA4: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DAA8: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DAB0 size=16
    let mut pc: u32 = 0x82E7DAB0;
    'dispatch: loop {
        match pc {
            0x82E7DAB0 => {
    //   block [0x82E7DAB0..0x82E7DAC0)
	// 82E7DAB0: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 82E7DAB4: B1640000  sth r11, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DAB8: B1650000  sth r11, 0(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82E7DABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DAC0 size=8
    let mut pc: u32 = 0x82E7DAC0;
    'dispatch: loop {
        match pc {
            0x82E7DAC0 => {
    //   block [0x82E7DAC0..0x82E7DAC8)
	// 82E7DAC0: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82E7DAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DAC8 size=8
    let mut pc: u32 = 0x82E7DAC8;
    'dispatch: loop {
        match pc {
            0x82E7DAC8 => {
    //   block [0x82E7DAC8..0x82E7DAD0)
	// 82E7DAC8: 806300EC  lwz r3, 0xec(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(236 as u32) ) } as u64;
	// 82E7DACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7DAD0 size=8
    let mut pc: u32 = 0x82E7DAD0;
    'dispatch: loop {
        match pc {
            0x82E7DAD0 => {
    //   block [0x82E7DAD0..0x82E7DAD8)
	// 82E7DAD0: C02300E8  lfs f1, 0xe8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(232 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7DAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DAD8 size=8
    let mut pc: u32 = 0x82E7DAD8;
    'dispatch: loop {
        match pc {
            0x82E7DAD8 => {
    //   block [0x82E7DAD8..0x82E7DAE0)
	// 82E7DAD8: 38630164  addi r3, r3, 0x164
	ctx.r[3].s64 = ctx.r[3].s64 + 356;
	// 82E7DADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DAE0 size=88
    let mut pc: u32 = 0x82E7DAE0;
    'dispatch: loop {
        match pc {
            0x82E7DAE0 => {
    //   block [0x82E7DAE0..0x82E7DB38)
	// 82E7DAE0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DAE4: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E7DAE8: 810B0068  lwz r8, 0x68(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E7DAEC: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7DAF0: 419A0040  beq cr6, 0x82e7db30
	if ctx.cr[6].eq {
	pc = 0x82E7DB30; continue 'dispatch;
	}
	// 82E7DAF4: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DAF8: 816A00DC  lwz r11, 0xdc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(220 as u32) ) } as u64;
	// 82E7DAFC: 814A00E0  lwz r10, 0xe0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E7DB00: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7DB04: 419A0020  beq cr6, 0x82e7db24
	if ctx.cr[6].eq {
	pc = 0x82E7DB24; continue 'dispatch;
	}
	// 82E7DB08: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DB0C: 80C700CC  lwz r6, 0xcc(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(204 as u32) ) } as u64;
	// 82E7DB10: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82E7DB14: 419A0024  beq cr6, 0x82e7db38
	if ctx.cr[6].eq {
		sub_82E7DB38(ctx, base);
		return;
	}
	// 82E7DB18: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E7DB1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7DB20: 409AFFE8  bne cr6, 0x82e7db08
	if !ctx.cr[6].eq {
	pc = 0x82E7DB08; continue 'dispatch;
	}
	// 82E7DB24: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82E7DB28: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7DB2C: 409AFFC8  bne cr6, 0x82e7daf4
	if !ctx.cr[6].eq {
	pc = 0x82E7DAF4; continue 'dispatch;
	}
	// 82E7DB30: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7DB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DB38 size=8
    let mut pc: u32 = 0x82E7DB38;
    'dispatch: loop {
        match pc {
            0x82E7DB38 => {
    //   block [0x82E7DB38..0x82E7DB40)
	// 82E7DB38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7DB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DB40 size=88
    let mut pc: u32 = 0x82E7DB40;
    'dispatch: loop {
        match pc {
            0x82E7DB40 => {
    //   block [0x82E7DB40..0x82E7DB98)
	// 82E7DB40: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DB44: 810B0064  lwz r8, 0x64(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E7DB48: 80EB0068  lwz r7, 0x68(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E7DB4C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E7DB50: 419A0040  beq cr6, 0x82e7db90
	if ctx.cr[6].eq {
	pc = 0x82E7DB90; continue 'dispatch;
	}
	// 82E7DB54: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DB58: 816A00DC  lwz r11, 0xdc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(220 as u32) ) } as u64;
	// 82E7DB5C: 812A00E0  lwz r9, 0xe0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E7DB60: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7DB64: 419A0020  beq cr6, 0x82e7db84
	if ctx.cr[6].eq {
	pc = 0x82E7DB84; continue 'dispatch;
	}
	// 82E7DB68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DB6C: 80CA00CC  lwz r6, 0xcc(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(204 as u32) ) } as u64;
	// 82E7DB70: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 82E7DB74: 419A0024  beq cr6, 0x82e7db98
	if ctx.cr[6].eq {
		sub_82E7DB98(ctx, base);
		return;
	}
	// 82E7DB78: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E7DB7C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7DB80: 409AFFE8  bne cr6, 0x82e7db68
	if !ctx.cr[6].eq {
	pc = 0x82E7DB68; continue 'dispatch;
	}
	// 82E7DB84: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82E7DB88: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E7DB8C: 409AFFC8  bne cr6, 0x82e7db54
	if !ctx.cr[6].eq {
	pc = 0x82E7DB54; continue 'dispatch;
	}
	// 82E7DB90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7DB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7DB98 size=16
    let mut pc: u32 = 0x82E7DB98;
    'dispatch: loop {
        match pc {
            0x82E7DB98 => {
    //   block [0x82E7DB98..0x82E7DBA8)
	// 82E7DB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7DB9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7DBA0: 916A00CC  stw r11, 0xcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82E7DBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7DBA8 size=156
    let mut pc: u32 = 0x82E7DBA8;
    'dispatch: loop {
        match pc {
            0x82E7DBA8 => {
    //   block [0x82E7DBA8..0x82E7DC44)
	// 82E7DBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7DBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7DBB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7DBB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7DBB8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E7DBBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7DBC0: 83C3015C  lwz r30, 0x15c(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(348 as u32) ) } as u64;
	// 82E7DBC4: 54AB063E  clrlwi r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E7DBC8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E7DBCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7DBD0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DBD4: 419A0028  beq cr6, 0x82e7dbfc
	if ctx.cr[6].eq {
	pc = 0x82E7DBFC; continue 'dispatch;
	}
	// 82E7DBD8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7DBDC: 419A004C  beq cr6, 0x82e7dc28
	if ctx.cr[6].eq {
	pc = 0x82E7DC28; continue 'dispatch;
	}
	// 82E7DBE0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DBE4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E7DBE8: 480042E9  bl 0x82e81ed0
	ctx.lr = 0x82E7DBEC;
	sub_82E81ED0(ctx, base);
	// 82E7DBEC: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DBF0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7DBF4: 409AFFEC  bne cr6, 0x82e7dbe0
	if !ctx.cr[6].eq {
	pc = 0x82E7DBE0; continue 'dispatch;
	}
	// 82E7DBF8: 48000030  b 0x82e7dc28
	pc = 0x82E7DC28; continue 'dispatch;
	// 82E7DBFC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7DC00: 419A0028  beq cr6, 0x82e7dc28
	if ctx.cr[6].eq {
	pc = 0x82E7DC28; continue 'dispatch;
	}
	// 82E7DC04: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DC08: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7DC0C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82E7DC10: 4098000C  bge cr6, 0x82e7dc1c
	if !ctx.cr[6].lt {
	pc = 0x82E7DC1C; continue 'dispatch;
	}
	// 82E7DC14: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E7DC18: 480042B9  bl 0x82e81ed0
	ctx.lr = 0x82E7DC1C;
	sub_82E81ED0(ctx, base);
	// 82E7DC1C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DC20: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7DC24: 409AFFE0  bne cr6, 0x82e7dc04
	if !ctx.cr[6].eq {
	pc = 0x82E7DC04; continue 'dispatch;
	}
	// 82E7DC28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7DC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7DC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7DC34: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E7DC38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7DC3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7DC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7DC48 size=212
    let mut pc: u32 = 0x82E7DC48;
    'dispatch: loop {
        match pc {
            0x82E7DC48 => {
    //   block [0x82E7DC48..0x82E7DD1C)
	// 82E7DC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7DC4C: 4832A521  bl 0x831a816c
	ctx.lr = 0x82E7DC50;
	sub_831A8130(ctx, base);
	// 82E7DC50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7DC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7DC58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E7DC5C: 3BDF00DC  addi r30, r31, 0xdc
	ctx.r[30].s64 = ctx.r[31].s64 + 220;
	// 82E7DC60: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E7DC64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7DC68: 4BF75F69  bl 0x82df3bd0
	ctx.lr = 0x82E7DC6C;
	sub_82DF3BD0(ctx, base);
	// 82E7DC6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7DC70: 4BF75F39  bl 0x82df3ba8
	ctx.lr = 0x82E7DC74;
	sub_82DF3BA8(ctx, base);
	// 82E7DC74: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7DC78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7DC7C: 409A0078  bne cr6, 0x82e7dcf4
	if !ctx.cr[6].eq {
	pc = 0x82E7DCF4; continue 'dispatch;
	}
	// 82E7DC80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E7DC84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E7DC88: 4BFC2281  bl 0x82e3ff08
	ctx.lr = 0x82E7DC8C;
	sub_82E3FF08(ctx, base);
	// 82E7DC8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E7DC90: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E7DC94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7DC98: 48004761  bl 0x82e823f8
	ctx.lr = 0x82E7DC9C;
	sub_82E823F8(ctx, base);
	// 82E7DC9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E7DCA0: 3BFF00E0  addi r31, r31, 0xe0
	ctx.r[31].s64 = ctx.r[31].s64 + 224;
	// 82E7DCA4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E7DCA8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E7DCAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DCB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7DCB4: 4B4467AD  bl 0x822c4460
	ctx.lr = 0x82E7DCB8;
	sub_822C4460(ctx, base);
	// 82E7DCB8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E7DCBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7DCC0: 419A0008  beq cr6, 0x82e7dcc8
	if ctx.cr[6].eq {
	pc = 0x82E7DCC8; continue 'dispatch;
	}
	// 82E7DCC4: 4B442BCD  bl 0x822c0890
	ctx.lr = 0x82E7DCC8;
	sub_822C0890(ctx, base);
	// 82E7DCC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7DCCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7DCD0: 4BFFFE11  bl 0x82e7dae0
	ctx.lr = 0x82E7DCD4;
	sub_82E7DAE0(ctx, base);
	// 82E7DCD4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7DCD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7DCDC: 409A0038  bne cr6, 0x82e7dd14
	if !ctx.cr[6].eq {
	pc = 0x82E7DD14; continue 'dispatch;
	}
	// 82E7DCE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7DCE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7DCE8: 4BFFFE59  bl 0x82e7db40
	ctx.lr = 0x82E7DCEC;
	sub_82E7DB40(ctx, base);
	// 82E7DCEC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E7DCF0: 4832A4CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82E7DCF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7DCF8: 395F00E0  addi r10, r31, 0xe0
	ctx.r[10].s64 = ctx.r[31].s64 + 224;
	// 82E7DCFC: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82E7DD00: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E7DD04: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E7DD08: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E7DD0C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E7DD10: 4B446751  bl 0x822c4460
	ctx.lr = 0x82E7DD14;
	sub_822C4460(ctx, base);
	// 82E7DD14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E7DD18: 4832A4A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7DD20 size=96
    let mut pc: u32 = 0x82E7DD20;
    'dispatch: loop {
        match pc {
            0x82E7DD20 => {
    //   block [0x82E7DD20..0x82E7DD80)
	// 82E7DD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7DD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7DD28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7DD2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7DD30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7DD34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7DD38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7DD3C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DD40: 48000AE1  bl 0x82e7e820
	ctx.lr = 0x82E7DD44;
	sub_82E7E820(ctx, base);
	// 82E7DD44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E7DD48: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82E7DD4C: 4BF75E85  bl 0x82df3bd0
	ctx.lr = 0x82E7DD50;
	sub_82DF3BD0(ctx, base);
	// 82E7DD50: 397F0164  addi r11, r31, 0x164
	ctx.r[11].s64 = ctx.r[31].s64 + 356;
	// 82E7DD54: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82E7DD58: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E7DD5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DD60: 917F0164  stw r11, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[11].u32 ) };
	// 82E7DD64: 4B4466FD  bl 0x822c4460
	ctx.lr = 0x82E7DD68;
	sub_822C4460(ctx, base);
	// 82E7DD68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7DD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7DD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7DD74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7DD78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7DD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7DD80 size=116
    let mut pc: u32 = 0x82E7DD80;
    'dispatch: loop {
        match pc {
            0x82E7DD80 => {
    //   block [0x82E7DD80..0x82E7DDF4)
	// 82E7DD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7DD84: 4832A3E5  bl 0x831a8168
	ctx.lr = 0x82E7DD88;
	sub_831A8130(ctx, base);
	// 82E7DD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7DD8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E7DD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E7DD94: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DD98: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DD9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7DDA0: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DDA4: 91290004  stw r9, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E7DDA8: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DDAC: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7DDB0: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E7DDB4: 419A0038  beq cr6, 0x82e7ddec
	if ctx.cr[6].eq {
	pc = 0x82E7DDEC; continue 'dispatch;
	}
	// 82E7DDB8: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82E7DDBC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E7DDC0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DDC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7DDC8: 419A0008  beq cr6, 0x82e7ddd0
	if ctx.cr[6].eq {
	pc = 0x82E7DDD0; continue 'dispatch;
	}
	// 82E7DDCC: 4B442AC5  bl 0x822c0890
	ctx.lr = 0x82E7DDD0;
	sub_822C0890(ctx, base);
	// 82E7DDD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7DDD4: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E7DDD8: 4BF743B1  bl 0x82df2188
	ctx.lr = 0x82E7DDDC;
	sub_82DF2188(ctx, base);
	// 82E7DDDC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DDE0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82E7DDE4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E7DDE8: 409AFFD4  bne cr6, 0x82e7ddbc
	if !ctx.cr[6].eq {
	pc = 0x82E7DDBC; continue 'dispatch;
	}
	// 82E7DDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7DDF0: 4832A3C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7DDF8 size=108
    let mut pc: u32 = 0x82E7DDF8;
    'dispatch: loop {
        match pc {
            0x82E7DDF8 => {
    //   block [0x82E7DDF8..0x82E7DE64)
	// 82E7DDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7DDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7DE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7DE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7DE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7DE0C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E7DE10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7DE14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DE18: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DE1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7DE20: 419A002C  beq cr6, 0x82e7de4c
	if ctx.cr[6].eq {
	pc = 0x82E7DE4C; continue 'dispatch;
	}
	// 82E7DE24: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E7DE28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7DE2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DE30: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E7DE34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E7DE38: 4E800421  bctrl
	ctx.lr = 0x82E7DE3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E7DE3C: 389F0158  addi r4, r31, 0x158
	ctx.r[4].s64 = ctx.r[31].s64 + 344;
	// 82E7DE40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7DE44: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DE48: 48008251  bl 0x82e86098
	ctx.lr = 0x82E7DE4C;
	sub_82E86098(ctx, base);
	// 82E7DE4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7DE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7DE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7DE58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7DE5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7DE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7DE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7DE68 size=608
    let mut pc: u32 = 0x82E7DE68;
    'dispatch: loop {
        match pc {
            0x82E7DE68 => {
    //   block [0x82E7DE68..0x82E7E0C8)
	// 82E7DE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7DE6C: 4832A301  bl 0x831a816c
	ctx.lr = 0x82E7DE70;
	sub_831A8130(ctx, base);
	// 82E7DE70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7DE74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E7DE78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E7DE7C: 7F1DF840  cmplw cr6, r29, r31
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82E7DE80: 419A023C  beq cr6, 0x82e7e0bc
	if ctx.cr[6].eq {
	pc = 0x82E7E0BC; continue 'dispatch;
	}
	// 82E7DE84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DE88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7DE8C: 419A001C  beq cr6, 0x82e7dea8
	if ctx.cr[6].eq {
	pc = 0x82E7DEA8; continue 'dispatch;
	}
	// 82E7DE90: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DE94: 3BC0000C  li r30, 0xc
	ctx.r[30].s64 = 12;
	// 82E7DE98: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E7DE9C: 7CEAF3D6  divw r7, r10, r30
	ctx.r[7].s32 = ctx.r[10].s32 / ctx.r[30].s32;
	// 82E7DEA0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82E7DEA4: 409A0018  bne cr6, 0x82e7debc
	if !ctx.cr[6].eq {
	pc = 0x82E7DEBC; continue 'dispatch;
	}
	// 82E7DEA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7DEAC: 4B887ED5  bl 0x82705d80
	ctx.lr = 0x82E7DEB0;
	sub_82705D80(ctx, base);
	// 82E7DEB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7DEB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7DEB8: 4832A304  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82E7DEBC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DEC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7DEC4: 409A000C  bne cr6, 0x82e7ded0
	if !ctx.cr[6].eq {
	pc = 0x82E7DED0; continue 'dispatch;
	}
	// 82E7DEC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E7DECC: 48000010  b 0x82e7dedc
	pc = 0x82E7DEDC; continue 'dispatch;
	// 82E7DED0: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DED4: 7CCA4050  subf r6, r10, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82E7DED8: 7D06F3D6  divw r8, r6, r30
	ctx.r[8].s32 = ctx.r[6].s32 / ctx.r[30].s32;
	// 82E7DEDC: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7DEE0: 41990070  bgt cr6, 0x82e7df50
	if ctx.cr[6].gt {
	pc = 0x82E7DF50; continue 'dispatch;
	}
	// 82E7DEE4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7DEE8: 419A002C  beq cr6, 0x82e7df14
	if ctx.cr[6].eq {
	pc = 0x82E7DF14; continue 'dispatch;
	}
	// 82E7DEEC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DEF0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E7DEF4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DEF8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E7DEFC: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DF00: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E7DF04: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E7DF08: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82E7DF0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7DF10: 409AFFDC  bne cr6, 0x82e7deec
	if !ctx.cr[6].eq {
	pc = 0x82E7DEEC; continue 'dispatch;
	}
	// 82E7DF14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DF18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7DF1C: 419A0010  beq cr6, 0x82e7df2c
	if ctx.cr[6].eq {
	pc = 0x82E7DF2C; continue 'dispatch;
	}
	// 82E7DF20: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DF24: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E7DF28: 7D69F3D6  divw r11, r9, r30
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[30].s32;
	// 82E7DF2C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E7DF30: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DF34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7DF38: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E7DF3C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7DF40: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E7DF44: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E7DF48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7DF4C: 4832A270  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82E7DF50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7DF54: 409A000C  bne cr6, 0x82e7df60
	if !ctx.cr[6].eq {
	pc = 0x82E7DF60; continue 'dispatch;
	}
	// 82E7DF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7DF5C: 48000010  b 0x82e7df6c
	pc = 0x82E7DF6C; continue 'dispatch;
	// 82E7DF60: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E7DF64: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E7DF68: 7D69F3D6  divw r11, r9, r30
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[30].s32;
	// 82E7DF6C: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E7DF70: 419900B8  bgt cr6, 0x82e7e028
	if ctx.cr[6].gt {
	pc = 0x82E7E028; continue 'dispatch;
	}
	// 82E7DF74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7DF78: 409A000C  bne cr6, 0x82e7df84
	if !ctx.cr[6].eq {
	pc = 0x82E7DF84; continue 'dispatch;
	}
	// 82E7DF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E7DF80: 48000010  b 0x82e7df90
	pc = 0x82E7DF90; continue 'dispatch;
	// 82E7DF84: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DF88: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E7DF8C: 7D29F3D6  divw r9, r9, r30
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[30].s32;
	// 82E7DF90: 5528083C  slwi r8, r9, 1
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E7DF94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DF98: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E7DF9C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E7DFA0: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E7DFA4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7DFA8: 419A002C  beq cr6, 0x82e7dfd4
	if ctx.cr[6].eq {
	pc = 0x82E7DFD4; continue 'dispatch;
	}
	// 82E7DFAC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DFB0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E7DFB4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DFB8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E7DFBC: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DFC0: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E7DFC4: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E7DFC8: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82E7DFCC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7DFD0: 409AFFDC  bne cr6, 0x82e7dfac
	if !ctx.cr[6].eq {
	pc = 0x82E7DFAC; continue 'dispatch;
	}
	// 82E7DFD4: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DFD8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82E7DFDC: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7DFE0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7DFE4: 419A00D4  beq cr6, 0x82e7e0b8
	if ctx.cr[6].eq {
	pc = 0x82E7E0B8; continue 'dispatch;
	}
	// 82E7DFE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7DFEC: 419A001C  beq cr6, 0x82e7e008
	if ctx.cr[6].eq {
	pc = 0x82E7E008; continue 'dispatch;
	}
	// 82E7DFF0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7DFF4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E7DFF8: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7DFFC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E7E000: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7E004: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E7E008: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E7E00C: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82E7E010: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7E014: 409AFFD4  bne cr6, 0x82e7dfe8
	if !ctx.cr[6].eq {
	pc = 0x82E7DFE8; continue 'dispatch;
	}
	// 82E7E018: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E7E01C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7E020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7E024: 4832A198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82E7E028: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7E02C: 419A0014  beq cr6, 0x82e7e040
	if ctx.cr[6].eq {
	pc = 0x82E7E040; continue 'dispatch;
	}
	// 82E7E030: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E7E034: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82E7E038: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E7E03C: 4BF7414D  bl 0x82df2188
	ctx.lr = 0x82E7E040;
	sub_82DF2188(ctx, base);
	// 82E7E040: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E048: 409A000C  bne cr6, 0x82e7e054
	if !ctx.cr[6].eq {
	pc = 0x82E7E054; continue 'dispatch;
	}
	// 82E7E04C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E7E050: 48000010  b 0x82e7e060
	pc = 0x82E7E060; continue 'dispatch;
	// 82E7E054: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7E058: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E7E05C: 7C89F3D6  divw r4, r9, r30
	ctx.r[4].s32 = ctx.r[9].s32 / ctx.r[30].s32;
	// 82E7E060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7E064: 4B729C75  bl 0x825a7cd8
	ctx.lr = 0x82E7E068;
	sub_825A7CD8(ctx, base);
	// 82E7E068: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7E06C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E070: 419A004C  beq cr6, 0x82e7e0bc
	if ctx.cr[6].eq {
	pc = 0x82E7E0BC; continue 'dispatch;
	}
	// 82E7E074: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7E078: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E07C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E080: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7E084: 419A0034  beq cr6, 0x82e7e0b8
	if ctx.cr[6].eq {
	pc = 0x82E7E0B8; continue 'dispatch;
	}
	// 82E7E088: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7E08C: 419A001C  beq cr6, 0x82e7e0a8
	if ctx.cr[6].eq {
	pc = 0x82E7E0A8; continue 'dispatch;
	}
	// 82E7E090: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E094: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E7E098: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E09C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E7E0A0: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7E0A4: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E7E0A8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E7E0AC: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 82E7E0B0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E7E0B4: 409AFFD4  bne cr6, 0x82e7e088
	if !ctx.cr[6].eq {
	pc = 0x82E7E088; continue 'dispatch;
	}
	// 82E7E0B8: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E7E0BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7E0C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7E0C4: 4832A0F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7E0C8 size=160
    let mut pc: u32 = 0x82E7E0C8;
    'dispatch: loop {
        match pc {
            0x82E7E0C8 => {
    //   block [0x82E7E0C8..0x82E7E168)
	// 82E7E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7E0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7E0D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E0D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E0DC: 807F0170  lwz r3, 0x170(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 82E7E0E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E0E4: 419A0008  beq cr6, 0x82e7e0ec
	if ctx.cr[6].eq {
	pc = 0x82E7E0EC; continue 'dispatch;
	}
	// 82E7E0E8: 4B4427A9  bl 0x822c0890
	ctx.lr = 0x82E7E0EC;
	sub_822C0890(ctx, base);
	// 82E7E0EC: 807F0168  lwz r3, 0x168(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 82E7E0F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E0F4: 419A0008  beq cr6, 0x82e7e0fc
	if ctx.cr[6].eq {
	pc = 0x82E7E0FC; continue 'dispatch;
	}
	// 82E7E0F8: 4B442799  bl 0x822c0890
	ctx.lr = 0x82E7E0FC;
	sub_822C0890(ctx, base);
	// 82E7E0FC: 387F0158  addi r3, r31, 0x158
	ctx.r[3].s64 = ctx.r[31].s64 + 344;
	// 82E7E100: 4BFFFC81  bl 0x82e7dd80
	ctx.lr = 0x82E7E104;
	sub_82E7DD80(ctx, base);
	// 82E7E104: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E7E108: 809F015C  lwz r4, 0x15c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 82E7E10C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E7E110: 4BF74079  bl 0x82df2188
	ctx.lr = 0x82E7E114;
	sub_82DF2188(ctx, base);
	// 82E7E114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E7E118: 915F015C  stw r10, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[10].u32 ) };
	// 82E7E11C: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E7E120: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E124: 419A0008  beq cr6, 0x82e7e12c
	if ctx.cr[6].eq {
	pc = 0x82E7E12C; continue 'dispatch;
	}
	// 82E7E128: 4B442769  bl 0x822c0890
	ctx.lr = 0x82E7E12C;
	sub_822C0890(ctx, base);
	// 82E7E12C: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82E7E130: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E134: 419A0008  beq cr6, 0x82e7e13c
	if ctx.cr[6].eq {
	pc = 0x82E7E13C; continue 'dispatch;
	}
	// 82E7E138: 4B442759  bl 0x822c0890
	ctx.lr = 0x82E7E13C;
	sub_822C0890(ctx, base);
	// 82E7E13C: 807F0144  lwz r3, 0x144(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 82E7E140: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E144: 419A0008  beq cr6, 0x82e7e14c
	if ctx.cr[6].eq {
	pc = 0x82E7E14C; continue 'dispatch;
	}
	// 82E7E148: 4B442749  bl 0x822c0890
	ctx.lr = 0x82E7E14C;
	sub_822C0890(ctx, base);
	// 82E7E14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7E150: 4B954251  bl 0x827d23a0
	ctx.lr = 0x82E7E154;
	sub_827D23A0(ctx, base);
	// 82E7E154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7E158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7E15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7E160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7E164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7E168 size=508
    let mut pc: u32 = 0x82E7E168;
    'dispatch: loop {
        match pc {
            0x82E7E168 => {
    //   block [0x82E7E168..0x82E7E364)
	// 82E7E168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E16C: 48329FF9  bl 0x831a8164
	ctx.lr = 0x82E7E170;
	sub_831A8130(ctx, base);
	// 82E7E170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E174: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7E178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E17C: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 82E7E180: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82E7E184: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82E7E188: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E18C: 393E0030  addi r9, r30, 0x30
	ctx.r[9].s64 = ctx.r[30].s64 + 48;
	// 82E7E190: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 82E7E194: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82E7E198: 389E0070  addi r4, r30, 0x70
	ctx.r[4].s64 = ctx.r[30].s64 + 112;
	// 82E7E19C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82E7E1A0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E7E1A4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7E368 size=116
    let mut pc: u32 = 0x82E7E368;
    'dispatch: loop {
        match pc {
            0x82E7E368 => {
    //   block [0x82E7E368..0x82E7E3DC)
	// 82E7E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E36C: 48329DF5  bl 0x831a8160
	ctx.lr = 0x82E7E370;
	sub_831A8130(ctx, base);
	// 82E7E370: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E378: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E7E37C: 3BDF0158  addi r30, r31, 0x158
	ctx.r[30].s64 = ctx.r[31].s64 + 344;
	// 82E7E380: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E7E384: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E7E388: 837F015C  lwz r27, 0x15c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 82E7E38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7E390: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E7E394: 80BB0004  lwz r5, 4(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E398: 4800B6A9  bl 0x82e89a40
	ctx.lr = 0x82E7E39C;
	sub_82E89A40(ctx, base);
	// 82E7E39C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E7E3A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E7E3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7E3A8: 4800BC51  bl 0x82e89ff8
	ctx.lr = 0x82E7E3AC;
	sub_82E89FF8(ctx, base);
	// 82E7E3AC: 935B0004  stw r26, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82E7E3B0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E3B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7E3B8: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E7E3BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E7E3C0: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E3C4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E3C8: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E7E3CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E7E3D0: 4E800421  bctrl
	ctx.lr = 0x82E7E3D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E7E3D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E7E3D8: 48329DD8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7E3E0 size=576
    let mut pc: u32 = 0x82E7E3E0;
    'dispatch: loop {
        match pc {
            0x82E7E3E0 => {
    //   block [0x82E7E3E0..0x82E7E620)
	// 82E7E3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E3E4: 48329D81  bl 0x831a8164
	ctx.lr = 0x82E7E3E8;
	sub_831A8130(ctx, base);
	// 82E7E3E8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E7E3EC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E3F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E3F4: 4B95429D  bl 0x827d2690
	ctx.lr = 0x82E7E3F8;
	sub_827D2690(ctx, base);
	// 82E7E3F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E7E3FC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E7E400: 93DF0140  stw r30, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[30].u32 ) };
	// 82E7E404: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E7E408: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 82E7E40C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E7E410: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 82E7E414: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E7E418: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 82E7E41C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E7E420: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 82E7E424: 3BBF0140  addi r29, r31, 0x140
	ctx.r[29].s64 = ctx.r[31].s64 + 320;
	// 82E7E428: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 82E7E42C: 3B7F0158  addi r27, r31, 0x158
	ctx.r[27].s64 = ctx.r[31].s64 + 344;
	// 82E7E430: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E7E434: 4BF73C95  bl 0x82df20c8
	ctx.lr = 0x82E7E438;
	sub_82DF20C8(ctx, base);
	// 82E7E438: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E43C: 419A0008  beq cr6, 0x82e7e444
	if ctx.cr[6].eq {
	pc = 0x82E7E444; continue 'dispatch;
	}
	// 82E7E440: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E7E444: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7E448: 41820008  beq 0x82e7e450
	if ctx.cr[0].eq {
	pc = 0x82E7E450; continue 'dispatch;
	}
	// 82E7E44C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E7E450: 907B0004  stw r3, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E7E454: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E7E458: 93DB0008  stw r30, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E7E45C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E7E460: 93DF0164  stw r30, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[30].u32 ) };
	// 82E7E464: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82E7E468: 93DF0168  stw r30, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[30].u32 ) };
	// 82E7E46C: 3B9F0180  addi r28, r31, 0x180
	ctx.r[28].s64 = ctx.r[31].s64 + 384;
	// 82E7E470: 93DF016C  stw r30, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[30].u32 ) };
	// 82E7E474: 93DF0170  stw r30, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[30].u32 ) };
	// 82E7E478: C00A08A8  lfs f0, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7E47C: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 82E7E480: D01F0180  stfs f0, 0x180(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), tmp.u32 ) };
	// 82E7E484: D01F0184  stfs f0, 0x184(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 82E7E488: D01F0188  stfs f0, 0x188(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), tmp.u32 ) };
	// 82E7E48C: D01F018C  stfs f0, 0x18c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), tmp.u32 ) };
	// 82E7E490: B17F0194  sth r11, 0x194(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[11].u16 ) };
	// 82E7E494: D01F0190  stfs f0, 0x190(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), tmp.u32 ) };
	// 82E7E498: B17F0196  sth r11, 0x196(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(406 as u32), ctx.r[11].u16 ) };
	// 82E7E49C: 93DF0198  stw r30, 0x198(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(408 as u32), ctx.r[30].u32 ) };
	// 82E7E4A0: 93DF019C  stw r30, 0x19c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(412 as u32), ctx.r[30].u32 ) };
	// 82E7E4A4: 993F01A0  stb r9, 0x1a0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[9].u8 ) };
	// 82E7E4A8: 9BDF01A1  stb r30, 0x1a1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(417 as u32), ctx.r[30].u8 ) };
	// 82E7E4AC: 9BDF01A2  stb r30, 0x1a2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(418 as u32), ctx.r[30].u8 ) };
	// 82E7E4B0: 9BDF01A3  stb r30, 0x1a3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(419 as u32), ctx.r[30].u8 ) };
	// 82E7E4B4: 997F01A4  stb r11, 0x1a4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[11].u8 ) };
	// 82E7E4B8: 9BDF01A5  stb r30, 0x1a5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(421 as u32), ctx.r[30].u8 ) };
	// 82E7E4BC: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E7E4C0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E4C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E4C8: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E7E4CC: 419A0008  beq cr6, 0x82e7e4d4
	if ctx.cr[6].eq {
	pc = 0x82E7E4D4; continue 'dispatch;
	}
	// 82E7E4D0: 4B4423C1  bl 0x822c0890
	ctx.lr = 0x82E7E4D4;
	sub_822C0890(ctx, base);
	// 82E7E4D4: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 82E7E4D8: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82E7E4DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E4E0: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 82E7E4E4: 419A0008  beq cr6, 0x82e7e4ec
	if ctx.cr[6].eq {
	pc = 0x82E7E4EC; continue 'dispatch;
	}
	// 82E7E4E8: 4B4423A9  bl 0x822c0890
	ctx.lr = 0x82E7E4EC;
	sub_822C0890(ctx, base);
	// 82E7E4EC: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 82E7E4F0: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E7E4F4: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 82E7E4F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E4FC: 419A0008  beq cr6, 0x82e7e504
	if ctx.cr[6].eq {
	pc = 0x82E7E504; continue 'dispatch;
	}
	// 82E7E500: 4B442391  bl 0x822c0890
	ctx.lr = 0x82E7E504;
	sub_822C0890(ctx, base);
	// 82E7E504: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E7E508: 4BFFF879  bl 0x82e7dd80
	ctx.lr = 0x82E7E50C;
	sub_82E7DD80(ctx, base);
	// 82E7E50C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7E510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82E7E514: 3B6B6910  addi r27, r11, 0x6910
	ctx.r[27].s64 = ctx.r[11].s64 + 26896;
	// 82E7E518: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82E7E51C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82E7E520: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82E7E524: 38CA4620  addi r6, r10, 0x4620
	ctx.r[6].s64 = ctx.r[10].s64 + 17952;
	// 82E7E528: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82E7E52C: 13E0D8C7  vcmpequd (lvx128) v31, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7E620 size=340
    let mut pc: u32 = 0x82E7E620;
    'dispatch: loop {
        match pc {
            0x82E7E620 => {
    //   block [0x82E7E620..0x82E7E774)
	// 82E7E620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E624: 48329B45  bl 0x831a8168
	ctx.lr = 0x82E7E628;
	sub_831A8130(ctx, base);
	// 82E7E628: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E62C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E7E630: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E7E634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E638: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E7E63C: 4BFFFB2D  bl 0x82e7e168
	ctx.lr = 0x82E7E640;
	sub_82E7E168(ctx, base);
	// 82E7E640: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E7E644: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7E648: 4BFC18C1  bl 0x82e3ff08
	ctx.lr = 0x82E7E64C;
	sub_82E3FF08(ctx, base);
	// 82E7E64C: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 82E7E650: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E7E654: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E7E658: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E7E65C: 48003F7D  bl 0x82e825d8
	ctx.lr = 0x82E7E660;
	sub_82E825D8(ctx, base);
	// 82E7E660: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E7E664: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E668: 409A0020  bne cr6, 0x82e7e688
	if !ctx.cr[6].eq {
	pc = 0x82E7E688; continue 'dispatch;
	}
	// 82E7E66C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E7E670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E674: 419A0008  beq cr6, 0x82e7e67c
	if ctx.cr[6].eq {
	pc = 0x82E7E67C; continue 'dispatch;
	}
	// 82E7E678: 4B442219  bl 0x822c0890
	ctx.lr = 0x82E7E67C;
	sub_822C0890(ctx, base);
	// 82E7E67C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7E680: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E7E684: 48329B34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82E7E688: 48000199  bl 0x82e7e820
	ctx.lr = 0x82E7E68C;
	sub_82E7E820(ctx, base);
	// 82E7E68C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E7E690: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7E694: 4BF7553D  bl 0x82df3bd0
	ctx.lr = 0x82E7E698;
	sub_82DF3BD0(ctx, base);
	// 82E7E698: 397F0164  addi r11, r31, 0x164
	ctx.r[11].s64 = ctx.r[31].s64 + 356;
	// 82E7E69C: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E7E6A0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E7E6A4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E7E6A8: 917F0164  stw r11, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[11].u32 ) };
	// 82E7E6AC: 4B445DB5  bl 0x822c4460
	ctx.lr = 0x82E7E6B0;
	sub_822C4460(ctx, base);
	// 82E7E6B0: C1BF0134  lfs f13, 0x134(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7E6B4: FD806A10  fabs f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E7E6B8: 3FC08212  lis r30, -0x7dee
	ctx.r[30].s64 = -2112749568;
	// 82E7E6BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E7E6C0: C01EDFB0  lfs f0, -0x2050(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7E6C4: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82E7E6C8: 41980008  blt cr6, 0x82e7e6d0
	if ctx.cr[6].lt {
	pc = 0x82E7E6D0; continue 'dispatch;
	}
	// 82E7E6CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7E6D0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E7E6D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E6D8: 419A0010  beq cr6, 0x82e7e6e8
	if ctx.cr[6].eq {
	pc = 0x82E7E6E8; continue 'dispatch;
	}
	// 82E7E6DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82E7E6E0: C00BE830  lfs f0, -0x17d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7E6E4: D01F0134  stfs f0, 0x134(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 82E7E6E8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E7E6EC: C03F0134  lfs f1, 0x134(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7E6F0: 48000141  bl 0x82e7e830
	ctx.lr = 0x82E7E6F4;
	sub_82E7E830(ctx, base);
	// 82E7E6F4: C1BF0130  lfs f13, 0x130(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7E6F8: FD806A10  fabs f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E7E6FC: C01EDFB0  lfs f0, -0x2050(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7E700: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E7E704: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82E7E708: 41980008  blt cr6, 0x82e7e710
	if ctx.cr[6].lt {
	pc = 0x82E7E710; continue 'dispatch;
	}
	// 82E7E70C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7E710: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E7E714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E718: 419A0010  beq cr6, 0x82e7e728
	if ctx.cr[6].eq {
	pc = 0x82E7E728; continue 'dispatch;
	}
	// 82E7E71C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E7E720: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7E724: D01F0130  stfs f0, 0x130(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 82E7E728: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E7E72C: C03F0130  lfs f1, 0x130(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7E730: 4824AB69  bl 0x830c9298
	ctx.lr = 0x82E7E734;
	sub_830C9298(ctx, base);
	// 82E7E734: 38BF0196  addi r5, r31, 0x196
	ctx.r[5].s64 = ctx.r[31].s64 + 406;
	// 82E7E738: 389F0194  addi r4, r31, 0x194
	ctx.r[4].s64 = ctx.r[31].s64 + 404;
	// 82E7E73C: 80DF00B0  lwz r6, 0xb0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E7E740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7E744: 4BFFF2E5  bl 0x82e7da28
	ctx.lr = 0x82E7E748;
	sub_82E7DA28(ctx, base);
	// 82E7E748: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E7E74C: 38BF00DC  addi r5, r31, 0xdc
	ctx.r[5].s64 = ctx.r[31].s64 + 220;
	// 82E7E750: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E7E754: 4BFFF4F5  bl 0x82e7dc48
	ctx.lr = 0x82E7E758;
	sub_82E7DC48(ctx, base);
	// 82E7E758: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E7E75C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E760: 419A0008  beq cr6, 0x82e7e768
	if ctx.cr[6].eq {
	pc = 0x82E7E768; continue 'dispatch;
	}
	// 82E7E764: 4B44212D  bl 0x822c0890
	ctx.lr = 0x82E7E768;
	sub_822C0890(ctx, base);
	// 82E7E768: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7E76C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E7E770: 48329A48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7E778 size=20
    let mut pc: u32 = 0x82E7E778;
    'dispatch: loop {
        match pc {
            0x82E7E778 => {
    //   block [0x82E7E778..0x82E7E78C)
	// 82E7E778: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E7E77C: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E7E780: 392BBA98  addi r9, r11, -0x4568
	ctx.r[9].s64 = ctx.r[11].s64 + -17768;
	// 82E7E784: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E7E788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7E790 size=24
    let mut pc: u32 = 0x82E7E790;
    'dispatch: loop {
        match pc {
            0x82E7E790 => {
    //   block [0x82E7E790..0x82E7E7A8)
	// 82E7E790: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E7E794: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E7E798: 396BBA98  addi r11, r11, -0x4568
	ctx.r[11].s64 = ctx.r[11].s64 + -17768;
	// 82E7E79C: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82E7E7A0: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E7E7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7E7A8 size=120
    let mut pc: u32 = 0x82E7E7A8;
    'dispatch: loop {
        match pc {
            0x82E7E7A8 => {
    //   block [0x82E7E7A8..0x82E7E820)
	// 82E7E7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7E7B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7E7B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7E7B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E7BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7E7C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E7C4: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82E7E7C8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E7E7CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E7D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7E7D4: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7E7D8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E7E7DC: 4BF753F5  bl 0x82df3bd0
	ctx.lr = 0x82E7E7E0;
	sub_82DF3BD0(ctx, base);
	// 82E7E7E0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E7E7E4: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 82E7E7E8: 4BF753E9  bl 0x82df3bd0
	ctx.lr = 0x82E7E7EC;
	sub_82DF3BD0(ctx, base);
	// 82E7E7EC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E7E7F0: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E7E7F4: 4BF753DD  bl 0x82df3bd0
	ctx.lr = 0x82E7E7F8;
	sub_82DF3BD0(ctx, base);
	// 82E7E7F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7E7FC: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 82E7E800: 4BF753D1  bl 0x82df3bd0
	ctx.lr = 0x82E7E804;
	sub_82DF3BD0(ctx, base);
	// 82E7E804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7E808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7E80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7E810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7E814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7E818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7E81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7E820 size=8
    let mut pc: u32 = 0x82E7E820;
    'dispatch: loop {
        match pc {
            0x82E7E820 => {
    //   block [0x82E7E820..0x82E7E828)
	// 82E7E820: 38630038  addi r3, r3, 0x38
	ctx.r[3].s64 = ctx.r[3].s64 + 56;
	// 82E7E824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7E828 size=8
    let mut pc: u32 = 0x82E7E828;
    'dispatch: loop {
        match pc {
            0x82E7E828 => {
    //   block [0x82E7E828..0x82E7E830)
	// 82E7E828: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 82E7E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7E830 size=8
    let mut pc: u32 = 0x82E7E830;
    'dispatch: loop {
        match pc {
            0x82E7E830 => {
    //   block [0x82E7E830..0x82E7E838)
	// 82E7E830: D023004C  stfs f1, 0x4c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82E7E834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7E838 size=132
    let mut pc: u32 = 0x82E7E838;
    'dispatch: loop {
        match pc {
            0x82E7E838 => {
    //   block [0x82E7E838..0x82E7E8BC)
	// 82E7E838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E83C: 4832992D  bl 0x831a8168
	ctx.lr = 0x82E7E840;
	sub_831A8130(ctx, base);
	// 82E7E840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E844: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E7E848: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7E84C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E850: 409A000C  bne cr6, 0x82e7e85c
	if !ctx.cr[6].eq {
	pc = 0x82E7E85C; continue 'dispatch;
	}
	// 82E7E854: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E7E858: 48000010  b 0x82e7e868
	pc = 0x82E7E868; continue 'dispatch;
	// 82E7E85C: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E7E860: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E7E864: 7D3F1E70  srawi r31, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 82E7E868: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E7E86C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E7E870: 409A000C  bne cr6, 0x82e7e87c
	if !ctx.cr[6].eq {
	pc = 0x82E7E87C; continue 'dispatch;
	}
	// 82E7E874: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E7E878: 48000028  b 0x82e7e8a0
	pc = 0x82E7E8A0; continue 'dispatch;
	// 82E7E87C: 40990024  ble cr6, 0x82e7e8a0
	if !ctx.cr[6].gt {
	pc = 0x82E7E8A0; continue 'dispatch;
	}
	// 82E7E880: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E7E884: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7E888: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E7E88C: 4BF7FF55  bl 0x82dfe7e0
	ctx.lr = 0x82E7E890;
	sub_82DFE7E0(ctx, base);
	// 82E7E890: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E7E894: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E7E898: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E7E89C: 4082FFE8  bne 0x82e7e884
	if !ctx.cr[0].eq {
	pc = 0x82E7E884; continue 'dispatch;
	}
	// 82E7E8A0: 807C0024  lwz r3, 0x24(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E7E8A4: 4BF7FF3D  bl 0x82dfe7e0
	ctx.lr = 0x82E7E8A8;
	sub_82DFE7E0(ctx, base);
	// 82E7E8A8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7E8AC: 57AA063E  clrlwi r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82E7E8B0: 7D635038  and r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82E7E8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7E8B8: 48329900  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7E8C0 size=464
    let mut pc: u32 = 0x82E7E8C0;
    'dispatch: loop {
        match pc {
            0x82E7E8C0 => {
    //   block [0x82E7E8C0..0x82E7EA90)
	// 82E7E8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7E8C4: 4832989D  bl 0x831a8160
	ctx.lr = 0x82E7E8C8;
	sub_831A8130(ctx, base);
	// 82E7E8C8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E7E8CC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7E8D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E7E8D4: 54AB063E  clrlwi r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E7E8D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E7E8DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E8E0: 3BFC001C  addi r31, r28, 0x1c
	ctx.r[31].s64 = ctx.r[28].s64 + 28;
	// 82E7E8E4: 409A0008  bne cr6, 0x82e7e8ec
	if !ctx.cr[6].eq {
	pc = 0x82E7E8EC; continue 'dispatch;
	}
	// 82E7E8E8: 3BFC0024  addi r31, r28, 0x24
	ctx.r[31].s64 = ctx.r[28].s64 + 36;
	// 82E7E8EC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E8F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7E8F4: 419A018C  beq cr6, 0x82e7ea80
	if ctx.cr[6].eq {
	pc = 0x82E7EA80; continue 'dispatch;
	}
	// 82E7E8F8: 4BF7FEE9  bl 0x82dfe7e0
	ctx.lr = 0x82E7E8FC;
	sub_82DFE7E0(ctx, base);
	// 82E7E8FC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7E900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7E904: 419A017C  beq cr6, 0x82e7ea80
	if ctx.cr[6].eq {
	pc = 0x82E7EA80; continue 'dispatch;
	}
	// 82E7E908: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E90C: 809B0010  lwz r4, 0x10(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7E910: 4BF99681  bl 0x82e17f90
	ctx.lr = 0x82E7E914;
	sub_82E17F90(ctx, base);
	// 82E7E914: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7E918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7E91C: 808B693C  lwz r4, 0x693c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26940 as u32) ) } as u64;
	// 82E7E920: 4BF99639  bl 0x82e17f58
	ctx.lr = 0x82E7E924;
	sub_82E17F58(ctx, base);
	// 82E7E924: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E7E928: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E92C: 4BF9924D  bl 0x82e17b78
	ctx.lr = 0x82E7E930;
	sub_82E17B78(ctx, base);
	// 82E7E930: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E7E934: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E7E938: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E7E93C: 4BF99385  bl 0x82e17cc0
	ctx.lr = 0x82E7E940;
	sub_82E17CC0(ctx, base);
	// 82E7E940: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82E7E944: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7E948: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7E94C: 3BAB6934  addi r29, r11, 0x6934
	ctx.r[29].s64 = ctx.r[11].s64 + 26932;
	// 82E7E950: 817F6938  lwz r11, 0x6938(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26936 as u32) ) } as u64;
	// 82E7E954: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82E7E958: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7E95C: 409A001C  bne cr6, 0x82e7e978
	if !ctx.cr[6].eq {
	pc = 0x82E7E978; continue 'dispatch;
	}
	// 82E7E960: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E7E964: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E7E968: 917F6938  stw r11, 0x6938(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26936 as u32), ctx.r[11].u32 ) };
	// 82E7E96C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7E970: 388ADFDC  addi r4, r10, -0x2024
	ctx.r[4].s64 = ctx.r[10].s64 + -8228;
	// 82E7E974: 4BF7ABBD  bl 0x82df9530
	ctx.lr = 0x82E7E978;
	sub_82DF9530(ctx, base);
	// 82E7E978: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E7E97C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E980: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E984: 4BF932CD  bl 0x82e11c50
	ctx.lr = 0x82E7E988;
	sub_82E11C50(ctx, base);
	// 82E7E988: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7E98C: 3BAB6930  addi r29, r11, 0x6930
	ctx.r[29].s64 = ctx.r[11].s64 + 26928;
	// 82E7E990: 817F6938  lwz r11, 0x6938(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26936 as u32) ) } as u64;
	// 82E7E994: 556A07BC  rlwinm r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E7E998: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7E99C: 409A001C  bne cr6, 0x82e7e9b8
	if !ctx.cr[6].eq {
	pc = 0x82E7E9B8; continue 'dispatch;
	}
	// 82E7E9A0: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82E7E9A4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E7E9A8: 917F6938  stw r11, 0x6938(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26936 as u32), ctx.r[11].u32 ) };
	// 82E7E9AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7E9B0: 388ADFCC  addi r4, r10, -0x2034
	ctx.r[4].s64 = ctx.r[10].s64 + -8244;
	// 82E7E9B4: 4BF7AB7D  bl 0x82df9530
	ctx.lr = 0x82E7E9B8;
	sub_82DF9530(ctx, base);
	// 82E7E9B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7E9BC: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E9C0: C07C004C  lfs f3, 0x4c(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(76 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E7E9C4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7E9C8: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E7E9CC: FC80F890  fmr f4, f31
	ctx.f[4].f64 = ctx.f[31].f64;
	// 82E7E9D0: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82E7E9D4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E7E9D8: 4BF93241  bl 0x82e11c18
	ctx.lr = 0x82E7E9DC;
	sub_82E11C18(ctx, base);
	// 82E7E9DC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7E9E0: 3BAB692C  addi r29, r11, 0x692c
	ctx.r[29].s64 = ctx.r[11].s64 + 26924;
	// 82E7E9E4: 817F6938  lwz r11, 0x6938(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26936 as u32) ) } as u64;
	// 82E7E9E8: 556A077A  rlwinm r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E7E9EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7E9F0: 409A001C  bne cr6, 0x82e7ea0c
	if !ctx.cr[6].eq {
	pc = 0x82E7EA0C; continue 'dispatch;
	}
	// 82E7E9F4: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82E7E9F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E7E9FC: 917F6938  stw r11, 0x6938(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26936 as u32), ctx.r[11].u32 ) };
	// 82E7EA00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E7EA04: 388ADFB8  addi r4, r10, -0x2048
	ctx.r[4].s64 = ctx.r[10].s64 + -8264;
	// 82E7EA08: 4BF7AB29  bl 0x82df9530
	ctx.lr = 0x82E7EA0C;
	sub_82DF9530(ctx, base);
	// 82E7EA0C: C03C0048  lfs f1, 0x48(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7EA10: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EA14: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EA18: FC400890  fmr f2, f1
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82E7EA1C: FC80F890  fmr f4, f31
	ctx.f[4].f64 = ctx.f[31].f64;
	// 82E7EA20: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82E7EA24: 4BF931F5  bl 0x82e11c18
	ctx.lr = 0x82E7EA28;
	sub_82E11C18(ctx, base);
	// 82E7EA28: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EA2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E7EA30: 419A0040  beq cr6, 0x82e7ea70
	if ctx.cr[6].eq {
	pc = 0x82E7EA70; continue 'dispatch;
	}
	// 82E7EA34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EA38: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E7EA3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7EA40: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E7EA44: D3EB1330  stfs f31, 0x1330(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4912 as u32), tmp.u32 ) };
	// 82E7EA48: D3EB1334  stfs f31, 0x1334(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4916 as u32), tmp.u32 ) };
	// 82E7EA4C: D3EB1338  stfs f31, 0x1338(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4920 as u32), tmp.u32 ) };
	// 82E7EA50: D3EB133C  stfs f31, 0x133c(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4924 as u32), tmp.u32 ) };
	// 82E7EA54: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82E7EA58: 65490002  oris r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u64 | 131072;
	// 82E7EA5C: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82E7EA60: 4BF900C9  bl 0x82e0eb28
	ctx.lr = 0x82E7EA64;
	sub_82E0EB28(ctx, base);
	// 82E7EA64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E7EA68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7EA6C: 4BF901B5  bl 0x82e0ec20
	ctx.lr = 0x82E7EA70;
	sub_82E0EC20(ctx, base);
	// 82E7EA70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7EA74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E7EA78: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E7EA7C: 48329734  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E7EA80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7EA84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E7EA88: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E7EA8C: 48329724  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7EA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7EA90 size=800
    let mut pc: u32 = 0x82E7EA90;
    'dispatch: loop {
        match pc {
            0x82E7EA90 => {
    //   block [0x82E7EA90..0x82E7EDB0)
	// 82E7EA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7EA94: 483296A5  bl 0x831a8138
	ctx.lr = 0x82E7EA98;
	sub_831A8130(ctx, base);
	// 82E7EA98: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7EA9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E7EAA0: 7C902378  mr r16, r4
	ctx.r[16].u64 = ctx.r[4].u64;
	// 82E7EAA4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E7EAA8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E7EAAC: 4BF7FD35  bl 0x82dfe7e0
	ctx.lr = 0x82E7EAB0;
	sub_82DFE7E0(ctx, base);
	// 82E7EAB0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7EAB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7EAB8: 419A02EC  beq cr6, 0x82e7eda4
	if ctx.cr[6].eq {
	pc = 0x82E7EDA4; continue 'dispatch;
	}
	// 82E7EABC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E7EAC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7EAC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E7EAC8: 4BFFFDF9  bl 0x82e7e8c0
	ctx.lr = 0x82E7EACC;
	sub_82E7E8C0(ctx, base);
	// 82E7EACC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7EAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7EAD4: 419A02D0  beq cr6, 0x82e7eda4
	if ctx.cr[6].eq {
	pc = 0x82E7EDA4; continue 'dispatch;
	}
	// 82E7EAD8: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7EADC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E7EAE0: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EAE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7EAE8: 409A000C  bne cr6, 0x82e7eaf4
	if !ctx.cr[6].eq {
	pc = 0x82E7EAF4; continue 'dispatch;
	}
	// 82E7EAEC: 7F31CB78  mr r17, r25
	ctx.r[17].u64 = ctx.r[25].u64;
	// 82E7EAF0: 48000010  b 0x82e7eb00
	pc = 0x82E7EB00; continue 'dispatch;
	// 82E7EAF4: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E7EAF8: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E7EAFC: 7D311E70  srawi r17, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[17].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 82E7EB00: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E7EB04: 2B110000  cmplwi cr6, r17, 0
	ctx.cr[6].compare_u32(ctx.r[17].u32, 0 as u32, &mut ctx.xer);
	// 82E7EB08: 419A0144  beq cr6, 0x82e7ec4c
	if ctx.cr[6].eq {
	pc = 0x82E7EC4C; continue 'dispatch;
	}
	// 82E7EB0C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E7EB10: 7F32CB78  mr r18, r25
	ctx.r[18].u64 = ctx.r[25].u64;
	// 82E7EB14: 3AE00004  li r23, 4
	ctx.r[23].s64 = 4;
	// 82E7EB18: 3A60000C  li r19, 0xc
	ctx.r[19].s64 = 12;
	// 82E7EB1C: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 82E7EB20: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82E7EB24: 3AA00014  li r21, 0x14
	ctx.r[21].s64 = 20;
	// 82E7EB28: 3AC00018  li r22, 0x18
	ctx.r[22].s64 = 24;
	// 82E7EB2C: 3B0BBAC8  addi r24, r11, -0x4538
	ctx.r[24].s64 = ctx.r[11].s64 + -17720;
	// 82E7EB30: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7EB34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7EB38: 419A0024  beq cr6, 0x82e7eb5c
	if ctx.cr[6].eq {
	pc = 0x82E7EB5C; continue 'dispatch;
	}
	// 82E7EB3C: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E7EB40: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E7EB44: 7D2B1E70  srawi r11, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 82E7EB48: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E7EB4C: 40980010  bge cr6, 0x82e7eb5c
	if !ctx.cr[6].lt {
	pc = 0x82E7EB5C; continue 'dispatch;
	}
	// 82E7EB50: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7EB54: 7D4B9214  add r10, r11, r18
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82E7EB58: 4800000C  b 0x82e7eb64
	pc = 0x82E7EB64; continue 'dispatch;
	// 82E7EB5C: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7EB60: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E7EB64: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EB68: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E7EB6C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E7EB70: 419A00CC  beq cr6, 0x82e7ec3c
	if ctx.cr[6].eq {
	pc = 0x82E7EC3C; continue 'dispatch;
	}
	// 82E7EB74: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7EB78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7EB7C: 419A001C  beq cr6, 0x82e7eb98
	if ctx.cr[6].eq {
	pc = 0x82E7EB98; continue 'dispatch;
	}
	// 82E7EB80: 813C0014  lwz r9, 0x14(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E7EB84: 7D0A4850  subf r8, r10, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82E7EB88: 7D0A1E70  srawi r10, r8, 3
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[8].s32 >> 3) as i64;
	// 82E7EB8C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7EB90: 40980008  bge cr6, 0x82e7eb98
	if !ctx.cr[6].lt {
	pc = 0x82E7EB98; continue 'dispatch;
	}
	// 82E7EB94: 7D6B9214  add r11, r11, r18
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[18].u64;
	// 82E7EB98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7EB9C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EBA0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7EBA4: 4BF8FA6D  bl 0x82e0e610
	ctx.lr = 0x82E7EBA8;
	sub_82E0E610(ctx, base);
	// 82E7EBA8: 817C0034  lwz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E7EBAC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82E7EBB0: 40990008  ble cr6, 0x82e7ebb8
	if !ctx.cr[6].gt {
	pc = 0x82E7EBB8; continue 'dispatch;
	}
	// 82E7EBB4: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82E7EBB8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7EBBC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E7EBC0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E7EBC4: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 82E7EBC8: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E7EBCC: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 82E7EBD0: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82E7EBD4: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 82E7EBD8: 7D4BC02E  lwzx r10, r11, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E7EBDC: 9261006C  stw r19, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[19].u32 ) };
	// 82E7EBE0: 93210070  stw r25, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82E7EBE4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82E7EBE8: 92810078  stw r20, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[20].u32 ) };
	// 82E7EBEC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82E7EBF0: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82E7EBF4: 9361007C  stw r27, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[27].u32 ) };
	// 82E7EBF8: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82E7EBFC: 92A10084  stw r21, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 82E7EC00: 93610088  stw r27, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 82E7EC04: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 82E7EC08: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 82E7EC0C: 93610094  stw r27, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 82E7EC10: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EC14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EC18: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7EC1C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E7EC20: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E7EC24: 814B01D4  lwz r10, 0x1d4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(468 as u32) ) } as u64;
	// 82E7EC28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E7EC2C: 4E800421  bctrl
	ctx.lr = 0x82E7EC30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E7EC30: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E7EC34: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82E7EC38: 4082FFD8  bne 0x82e7ec10
	if !ctx.cr[0].eq {
	pc = 0x82E7EC10; continue 'dispatch;
	}
	// 82E7EC3C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E7EC40: 3A520008  addi r18, r18, 8
	ctx.r[18].s64 = ctx.r[18].s64 + 8;
	// 82E7EC44: 7F1F8840  cmplw cr6, r31, r17
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[17].u32, &mut ctx.xer);
	// 82E7EC48: 4198FEE8  blt cr6, 0x82e7eb30
	if ctx.cr[6].lt {
	pc = 0x82E7EB30; continue 'dispatch;
	}
	// 82E7EC4C: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7EC50: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7EC54: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E7EC58: 419A0088  beq cr6, 0x82e7ece0
	if ctx.cr[6].eq {
	pc = 0x82E7ECE0; continue 'dispatch;
	}
	// 82E7EC5C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E7EC60: 815C0030  lwz r10, 0x30(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E7EC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E7EC68: 396BBA98  addi r11, r11, -0x4568
	ctx.r[11].s64 = ctx.r[11].s64 + -17768;
	// 82E7EC6C: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E7EC70: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82E7EC74: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82E7EC78: 7FE9582E  lwzx r31, r9, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E7EC7C: 7FC9402E  lwzx r30, r9, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E7EC80: 4B466669  bl 0x822e52e8
	ctx.lr = 0x82E7EC84;
	sub_822E52E8(ctx, base);
	// 82E7EC84: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E7EC88: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82E7EC8C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7EC90: 4B466659  bl 0x822e52e8
	ctx.lr = 0x82E7EC94;
	sub_822E52E8(ctx, base);
	// 82E7EC94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E7EC98: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E7EC9C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ECA0: 4B466649  bl 0x822e52e8
	ctx.lr = 0x82E7ECA4;
	sub_822E52E8(ctx, base);
	// 82E7ECA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E7ECA8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82E7ECAC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ECB0: 4B466639  bl 0x822e52e8
	ctx.lr = 0x82E7ECB4;
	sub_822E52E8(ctx, base);
	// 82E7ECB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7ECB8: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 82E7ECBC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ECC0: 4B466629  bl 0x822e52e8
	ctx.lr = 0x82E7ECC4;
	sub_822E52E8(ctx, base);
	// 82E7ECC4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E7ECC8: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82E7ECCC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ECD0: 4B466619  bl 0x822e52e8
	ctx.lr = 0x82E7ECD4;
	sub_822E52E8(ctx, base);
	// 82E7ECD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7ECD8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E7ECDC: 483294AC  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
	// 82E7ECE0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82E7ECE4: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82E7ECE8: 4B466601  bl 0x822e52e8
	ctx.lr = 0x82E7ECEC;
	sub_822E52E8(ctx, base);
	// 82E7ECEC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E7ECF0: 38800148  li r4, 0x148
	ctx.r[4].s64 = 328;
	// 82E7ECF4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ECF8: 4B4665F1  bl 0x822e52e8
	ctx.lr = 0x82E7ECFC;
	sub_822E52E8(ctx, base);
	// 82E7ECFC: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 82E7ED00: 3880014C  li r4, 0x14c
	ctx.r[4].s64 = 332;
	// 82E7ED04: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 82E7ED08: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED0C: 4B4665DD  bl 0x822e52e8
	ctx.lr = 0x82E7ED10;
	sub_822E52E8(ctx, base);
	// 82E7ED10: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E7ED14: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82E7ED18: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED1C: 4B4665CD  bl 0x822e52e8
	ctx.lr = 0x82E7ED20;
	sub_822E52E8(ctx, base);
	// 82E7ED20: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E7ED24: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 82E7ED28: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED2C: 4B4665BD  bl 0x822e52e8
	ctx.lr = 0x82E7ED30;
	sub_822E52E8(ctx, base);
	// 82E7ED30: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 82E7ED34: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED38: 88B00008  lbz r5, 8(r16)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7ED3C: 4B4665AD  bl 0x822e52e8
	ctx.lr = 0x82E7ED40;
	sub_822E52E8(ctx, base);
	// 82E7ED40: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82E7ED44: 38800068  li r4, 0x68
	ctx.r[4].s64 = 104;
	// 82E7ED48: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED4C: 4B46659D  bl 0x822e52e8
	ctx.lr = 0x82E7ED50;
	sub_822E52E8(ctx, base);
	// 82E7ED50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E7ED54: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82E7ED58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED5C: 4B46658D  bl 0x822e52e8
	ctx.lr = 0x82E7ED60;
	sub_822E52E8(ctx, base);
	// 82E7ED60: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E7ED64: 813C0030  lwz r9, 0x30(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E7ED68: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 82E7ED6C: 396BBA98  addi r11, r11, -0x4568
	ctx.r[11].s64 = ctx.r[11].s64 + -17768;
	// 82E7ED70: 55281838  slwi r8, r9, 3
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E7ED74: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E7ED78: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED7C: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E7ED80: 7FE8502E  lwzx r31, r8, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E7ED84: 4B466565  bl 0x822e52e8
	ctx.lr = 0x82E7ED88;
	sub_822E52E8(ctx, base);
	// 82E7ED88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7ED8C: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 82E7ED90: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E7ED94: 4B466555  bl 0x822e52e8
	ctx.lr = 0x82E7ED98;
	sub_822E52E8(ctx, base);
	// 82E7ED98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7ED9C: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E7EDA0: 483293E8  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
	// 82E7EDA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7EDA8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E7EDAC: 483293DC  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7EDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7EDB0 size=232
    let mut pc: u32 = 0x82E7EDB0;
    'dispatch: loop {
        match pc {
            0x82E7EDB0 => {
    //   block [0x82E7EDB0..0x82E7EE98)
	// 82E7EDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7EDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7EDB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7EDBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7EDC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7EDC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7EDC8: 4BFABD21  bl 0x82e2aae8
	ctx.lr = 0x82E7EDCC;
	sub_82E2AAE8(ctx, base);
	// 82E7EDCC: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 82E7EDD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7EDD4: 4BF74E2D  bl 0x82df3c00
	ctx.lr = 0x82E7EDD8;
	sub_82DF3C00(ctx, base);
	// 82E7EDD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E7EDDC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E7EDE0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E7EDE4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E7EDE8: 4BFACE69  bl 0x82e2bc50
	ctx.lr = 0x82E7EDEC;
	sub_82E2BC50(ctx, base);
	// 82E7EDEC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E7EDF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7EDF4: 419A0070  beq cr6, 0x82e7ee64
	if ctx.cr[6].eq {
	pc = 0x82E7EE64; continue 'dispatch;
	}
	// 82E7EDF8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E7EDFC: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82E7EE00: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E7EE04: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E7EE08: 4B445659  bl 0x822c4460
	ctx.lr = 0x82E7EE0C;
	sub_822C4460(ctx, base);
	// 82E7EE0C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7EE10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7EE14: 388BDFF0  addi r4, r11, -0x2010
	ctx.r[4].s64 = ctx.r[11].s64 + -8208;
	// 82E7EE18: 4BF74BF1  bl 0x82df3a08
	ctx.lr = 0x82E7EE1C;
	sub_82DF3A08(ctx, base);
	// 82E7EE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E7EE20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E7EE24: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E7EE28: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E7EE2C: 4BFACE25  bl 0x82e2bc50
	ctx.lr = 0x82E7EE30;
	sub_82E2BC50(ctx, base);
	// 82E7EE30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E7EE34: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 82E7EE38: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E7EE3C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E7EE40: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7EE44: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E7EE48: 4B445619  bl 0x822c4460
	ctx.lr = 0x82E7EE4C;
	sub_822C4460(ctx, base);
	// 82E7EE4C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E7EE50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7EE54: 419A0008  beq cr6, 0x82e7ee5c
	if ctx.cr[6].eq {
	pc = 0x82E7EE5C; continue 'dispatch;
	}
	// 82E7EE58: 4B441A39  bl 0x822c0890
	ctx.lr = 0x82E7EE5C;
	sub_822C0890(ctx, base);
	// 82E7EE5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7EE60: 4BF745C9  bl 0x82df3428
	ctx.lr = 0x82E7EE64;
	sub_82DF3428(ctx, base);
	// 82E7EE64: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E7EE68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7EE6C: 419A0008  beq cr6, 0x82e7ee74
	if ctx.cr[6].eq {
	pc = 0x82E7EE74; continue 'dispatch;
	}
	// 82E7EE70: 4B441A21  bl 0x822c0890
	ctx.lr = 0x82E7EE74;
	sub_822C0890(ctx, base);
	// 82E7EE74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7EE78: 4BF745B1  bl 0x82df3428
	ctx.lr = 0x82E7EE7C;
	sub_82DF3428(ctx, base);
	// 82E7EE7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7EE80: 4BFABC81  bl 0x82e2ab00
	ctx.lr = 0x82E7EE84;
	sub_82E2AB00(ctx, base);
	// 82E7EE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7EE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7EE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7EE90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7EE98 size=212
    let mut pc: u32 = 0x82E7EE98;
    'dispatch: loop {
        match pc {
            0x82E7EE98 => {
    //   block [0x82E7EE98..0x82E7EF6C)
	// 82E7EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7EEA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7EEA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7EEA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7EEAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7EEB0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7EEB4: 3BFE0030  addi r31, r30, 0x30
	ctx.r[31].s64 = ctx.r[30].s64 + 48;
	// 82E7EEB8: 394BE008  addi r10, r11, -0x1ff8
	ctx.r[10].s64 = ctx.r[11].s64 + -8184;
	// 82E7EEBC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7EEC0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E7EEC4: 4BF74565  bl 0x82df3428
	ctx.lr = 0x82E7EEC8;
	sub_82DF3428(ctx, base);
	// 82E7EEC8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E7EECC: 4BF7455D  bl 0x82df3428
	ctx.lr = 0x82E7EED0;
	sub_82DF3428(ctx, base);
	// 82E7EED0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E7EED4: 4BF74555  bl 0x82df3428
	ctx.lr = 0x82E7EED8;
	sub_82DF3428(ctx, base);
	// 82E7EED8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E7EEDC: 4BF7454D  bl 0x82df3428
	ctx.lr = 0x82E7EEE0;
	sub_82DF3428(ctx, base);
	// 82E7EEE0: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E7EEE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7EEE8: 419A0008  beq cr6, 0x82e7eef0
	if ctx.cr[6].eq {
	pc = 0x82E7EEF0; continue 'dispatch;
	}
	// 82E7EEEC: 4B4419A5  bl 0x822c0890
	ctx.lr = 0x82E7EEF0;
	sub_822C0890(ctx, base);
	// 82E7EEF0: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E7EEF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7EEF8: 419A0008  beq cr6, 0x82e7ef00
	if ctx.cr[6].eq {
	pc = 0x82E7EF00; continue 'dispatch;
	}
	// 82E7EEFC: 4B441995  bl 0x822c0890
	ctx.lr = 0x82E7EF00;
	sub_822C0890(ctx, base);
	// 82E7EF00: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7EF04: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82E7EF08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7EF0C: 419A0024  beq cr6, 0x82e7ef30
	if ctx.cr[6].eq {
	pc = 0x82E7EF30; continue 'dispatch;
	}
	// 82E7EF10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7EF14: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E7EF18: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7EF1C: 4B95336D  bl 0x827d2288
	ctx.lr = 0x82E7EF20;
	sub_827D2288(ctx, base);
	// 82E7EF20: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E7EF24: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7EF28: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E7EF2C: 4BF7325D  bl 0x82df2188
	ctx.lr = 0x82E7EF30;
	sub_82DF2188(ctx, base);
	// 82E7EF30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E7EF34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7EF38: 392A9B84  addi r9, r10, -0x647c
	ctx.r[9].s64 = ctx.r[10].s64 + -25724;
	// 82E7EF3C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E7EF40: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E7EF44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E7EF48: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E7EF4C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E7EF50: 4BF744D9  bl 0x82df3428
	ctx.lr = 0x82E7EF54;
	sub_82DF3428(ctx, base);
	// 82E7EF54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7EF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7EF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7EF60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7EF64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7EF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7EF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7EF70 size=196
    let mut pc: u32 = 0x82E7EF70;
    'dispatch: loop {
        match pc {
            0x82E7EF70 => {
    //   block [0x82E7EF70..0x82E7F034)
	// 82E7EF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7EF74: 483291F5  bl 0x831a8168
	ctx.lr = 0x82E7EF78;
	sub_831A8130(ctx, base);
	// 82E7EF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7EF7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7EF80: 4BF7F819  bl 0x82dfe798
	ctx.lr = 0x82E7EF84;
	sub_82DFE798(ctx, base);
	// 82E7EF84: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7EF88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E7EF8C: 394BE008  addi r10, r11, -0x1ff8
	ctx.r[10].s64 = ctx.r[11].s64 + -8184;
	// 82E7EF90: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 82E7EF94: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E7EF98: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 82E7EF9C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E7EFA0: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 82E7EFA4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82E7EFA8: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82E7EFAC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E7EFB0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82E7EFB4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82E7EFB8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82E7EFBC: 9BDF002C  stb r30, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u8 ) };
	// 82E7EFC0: 4BF74131  bl 0x82df30f0
	ctx.lr = 0x82E7EFC4;
	sub_82DF30F0(ctx, base);
	// 82E7EFC4: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82E7EFC8: 4BF74129  bl 0x82df30f0
	ctx.lr = 0x82E7EFCC;
	sub_82DF30F0(ctx, base);
	// 82E7EFCC: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 82E7EFD0: 4BF74121  bl 0x82df30f0
	ctx.lr = 0x82E7EFD4;
	sub_82DF30F0(ctx, base);
	// 82E7EFD4: 387D0014  addi r3, r29, 0x14
	ctx.r[3].s64 = ctx.r[29].s64 + 20;
	// 82E7EFD8: 4BF74119  bl 0x82df30f0
	ctx.lr = 0x82E7EFDC;
	sub_82DF30F0(ctx, base);
	// 82E7EFDC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82E7EFE0: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 82E7EFE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E7EFE8: C0099450  lfs f0, -0x6bb0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7EFEC: C1A8E830  lfs f13, -0x17d0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7EFF0: D01F0048  stfs f0, 0x48(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82E7EFF4: D1BF004C  stfs f13, 0x4c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82E7EFF8: 4B447BB1  bl 0x822c6ba8
	ctx.lr = 0x82E7EFFC;
	sub_822C6BA8(ctx, base);
	// 82E7EFFC: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82E7F000: 4BF74429  bl 0x82df3428
	ctx.lr = 0x82E7F004;
	sub_82DF3428(ctx, base);
	// 82E7F004: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E7F008: 4BF74421  bl 0x82df3428
	ctx.lr = 0x82E7F00C;
	sub_82DF3428(ctx, base);
	// 82E7F00C: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 82E7F010: 4BF74419  bl 0x82df3428
	ctx.lr = 0x82E7F014;
	sub_82DF3428(ctx, base);
	// 82E7F014: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82E7F018: 4BF74411  bl 0x82df3428
	ctx.lr = 0x82E7F01C;
	sub_82DF3428(ctx, base);
	// 82E7F01C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E7F020: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82E7F024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F028: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82E7F02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7F030: 48329188  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F038 size=80
    let mut pc: u32 = 0x82E7F038;
    'dispatch: loop {
        match pc {
            0x82E7F038 => {
    //   block [0x82E7F038..0x82E7F088)
	// 82E7F038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7F044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F04C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F050: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7F054: 4BFFFE45  bl 0x82e7ee98
	ctx.lr = 0x82E7F058;
	sub_82E7EE98(ctx, base);
	// 82E7F058: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E7F05C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F060: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F064: 419A000C  beq cr6, 0x82e7f070
	if ctx.cr[6].eq {
	pc = 0x82E7F070; continue 'dispatch;
	}
	// 82E7F068: 4BF73371  bl 0x82df23d8
	ctx.lr = 0x82E7F06C;
	sub_82DF23D8(ctx, base);
	// 82E7F06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7F074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F07C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7F080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F088 size=252
    let mut pc: u32 = 0x82E7F088;
    'dispatch: loop {
        match pc {
            0x82E7F088 => {
    //   block [0x82E7F088..0x82E7F184)
	// 82E7F088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7F094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F09C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7F0A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E7F0A4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E7F0A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7F0AC: 4BFABA3D  bl 0x82e2aae8
	ctx.lr = 0x82E7F0B0;
	sub_82E2AAE8(ctx, base);
	// 82E7F0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F0B4: 4BF74AF5  bl 0x82df3ba8
	ctx.lr = 0x82E7F0B8;
	sub_82DF3BA8(ctx, base);
	// 82E7F0B8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F0BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F0C0: 409A0040  bne cr6, 0x82e7f100
	if !ctx.cr[6].eq {
	pc = 0x82E7F100; continue 'dispatch;
	}
	// 82E7F0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E7F0C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7F0CC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E7F0D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E7F0D4: 4BFACB0D  bl 0x82e2bbe0
	ctx.lr = 0x82E7F0D8;
	sub_82E2BBE0(ctx, base);
	// 82E7F0D8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E7F0DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7F0E0: 419A0018  beq cr6, 0x82e7f0f8
	if ctx.cr[6].eq {
	pc = 0x82E7F0F8; continue 'dispatch;
	}
	// 82E7F0E4: 38800035  li r4, 0x35
	ctx.r[4].s64 = 53;
	// 82E7F0E8: 482489F9  bl 0x830c7ae0
	ctx.lr = 0x82E7F0EC;
	sub_830C7AE0(ctx, base);
	// 82E7F0EC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E7F0F0: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82E7F0F4: 4BD34015  bl 0x82bb3108
	ctx.lr = 0x82E7F0F8;
	sub_82BB3108(ctx, base);
	// 82E7F0F8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E7F0FC: 4800005C  b 0x82e7f158
	pc = 0x82E7F158; continue 'dispatch;
	// 82E7F100: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82E7F104: 4BF732BD  bl 0x82df23c0
	ctx.lr = 0x82E7F108;
	sub_82DF23C0(ctx, base);
	// 82E7F108: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7F10C: 419A0010  beq cr6, 0x82e7f11c
	if ctx.cr[6].eq {
	pc = 0x82E7F11C; continue 'dispatch;
	}
	// 82E7F110: 4B44BD51  bl 0x822cae60
	ctx.lr = 0x82E7F114;
	sub_822CAE60(ctx, base);
	// 82E7F114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F118: 48000008  b 0x82e7f120
	pc = 0x82E7F120; continue 'dispatch;
	// 82E7F11C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E7F120: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E7F124: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7F128: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7F12C: 4B75FDDD  bl 0x825def08
	ctx.lr = 0x82E7F130;
	sub_825DEF08(ctx, base);
	// 82E7F130: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E7F134: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7F138: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7F13C: 4B440EC5  bl 0x822c0000
	ctx.lr = 0x82E7F140;
	sub_822C0000(ctx, base);
	// 82E7F140: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E7F144: 4BF7F715  bl 0x82dfe858
	ctx.lr = 0x82E7F148;
	sub_82DFE858(ctx, base);
	// 82E7F148: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E7F14C: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82E7F150: 4BD33FB9  bl 0x82bb3108
	ctx.lr = 0x82E7F154;
	sub_82BB3108(ctx, base);
	// 82E7F154: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E7F158: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7F15C: 419A0008  beq cr6, 0x82e7f164
	if ctx.cr[6].eq {
	pc = 0x82E7F164; continue 'dispatch;
	}
	// 82E7F160: 4B441731  bl 0x822c0890
	ctx.lr = 0x82E7F164;
	sub_822C0890(ctx, base);
	// 82E7F164: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E7F168: 4BFAB999  bl 0x82e2ab00
	ctx.lr = 0x82E7F16C;
	sub_82E2AB00(ctx, base);
	// 82E7F16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7F170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F178: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7F17C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F188 size=180
    let mut pc: u32 = 0x82E7F188;
    'dispatch: loop {
        match pc {
            0x82E7F188 => {
    //   block [0x82E7F188..0x82E7F23C)
	// 82E7F188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7F194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F19C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F1A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7F1A4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E7F1A8: 4B447A01  bl 0x822c6ba8
	ctx.lr = 0x82E7F1AC;
	sub_822C6BA8(ctx, base);
	// 82E7F1AC: 389F003C  addi r4, r31, 0x3c
	ctx.r[4].s64 = ctx.r[31].s64 + 60;
	// 82E7F1B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7F1B4: 4BF74A4D  bl 0x82df3c00
	ctx.lr = 0x82E7F1B8;
	sub_82DF3C00(ctx, base);
	// 82E7F1B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7F1BC: 4BF749ED  bl 0x82df3ba8
	ctx.lr = 0x82E7F1C0;
	sub_82DF3BA8(ctx, base);
	// 82E7F1C0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F1C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F1C8: 409A0054  bne cr6, 0x82e7f21c
	if !ctx.cr[6].eq {
	pc = 0x82E7F21C; continue 'dispatch;
	}
	// 82E7F1CC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E7F1D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E7F1D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E7F1D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F1DC: 4BFFFEAD  bl 0x82e7f088
	ctx.lr = 0x82E7F1E0;
	sub_82E7F088(ctx, base);
	// 82E7F1E0: 389F0040  addi r4, r31, 0x40
	ctx.r[4].s64 = ctx.r[31].s64 + 64;
	// 82E7F1E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7F1E8: 4BF74A19  bl 0x82df3c00
	ctx.lr = 0x82E7F1EC;
	sub_82DF3C00(ctx, base);
	// 82E7F1EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7F1F0: 4BF749B9  bl 0x82df3ba8
	ctx.lr = 0x82E7F1F4;
	sub_82DF3BA8(ctx, base);
	// 82E7F1F4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F1F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F1FC: 409A0018  bne cr6, 0x82e7f214
	if !ctx.cr[6].eq {
	pc = 0x82E7F214; continue 'dispatch;
	}
	// 82E7F200: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E7F204: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E7F208: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E7F20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F210: 4BFFFE79  bl 0x82e7f088
	ctx.lr = 0x82E7F214;
	sub_82E7F088(ctx, base);
	// 82E7F214: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E7F218: 4BF74211  bl 0x82df3428
	ctx.lr = 0x82E7F21C;
	sub_82DF3428(ctx, base);
	// 82E7F21C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7F220: 4BF74209  bl 0x82df3428
	ctx.lr = 0x82E7F224;
	sub_82DF3428(ctx, base);
	// 82E7F224: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7F228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F230: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7F234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F240 size=72
    let mut pc: u32 = 0x82E7F240;
    'dispatch: loop {
        match pc {
            0x82E7F240 => {
    //   block [0x82E7F240..0x82E7F288)
	// 82E7F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F244: 48328F29  bl 0x831a816c
	ctx.lr = 0x82E7F248;
	sub_831A8130(ctx, base);
	// 82E7F248: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F24C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F250: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E7F254: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E7F258: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E7F25C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E7F260: 4BFFF549  bl 0x82e7e7a8
	ctx.lr = 0x82E7F264;
	sub_82E7E7A8(ctx, base);
	// 82E7F264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F268: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E7F26C: 4BFFFB45  bl 0x82e7edb0
	ctx.lr = 0x82E7F270;
	sub_82E7EDB0(ctx, base);
	// 82E7F270: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E7F274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F278: 4BFFFF11  bl 0x82e7f188
	ctx.lr = 0x82E7F27C;
	sub_82E7F188(ctx, base);
	// 82E7F27C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7F280: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7F284: 48328F38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7F288 size=20
    let mut pc: u32 = 0x82E7F288;
    'dispatch: loop {
        match pc {
            0x82E7F288 => {
    //   block [0x82E7F288..0x82E7F29C)
	// 82E7F288: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E7F28C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E7F290: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7F294: 994B0005  stb r10, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E7F298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F2A0 size=156
    let mut pc: u32 = 0x82E7F2A0;
    'dispatch: loop {
        match pc {
            0x82E7F2A0 => {
    //   block [0x82E7F2A0..0x82E7F33C)
	// 82E7F2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F2A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F2AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F2B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F2B4: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7F2B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F2BC: 419A001C  beq cr6, 0x82e7f2d8
	if ctx.cr[6].eq {
	pc = 0x82E7F2D8; continue 'dispatch;
	}
	// 82E7F2C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7F2C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F2D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F2D4: 4E800020  blr
	return;
	// 82E7F2D8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7F2DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F2E0: 419A0044  beq cr6, 0x82e7f324
	if ctx.cr[6].eq {
	pc = 0x82E7F324; continue 'dispatch;
	}
	// 82E7F2E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F2EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7F2F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E7F2F4: 4E800421  bctrl
	ctx.lr = 0x82E7F2F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E7F2F8: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F2FC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E7F300: 419A0024  beq cr6, 0x82e7f324
	if ctx.cr[6].eq {
	pc = 0x82E7F324; continue 'dispatch;
	}
	// 82E7F304: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E7F308: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E7F30C: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E7F310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F31C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F320: 4E800020  blr
	return;
	// 82E7F324: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7F328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7F340 size=92
    let mut pc: u32 = 0x82E7F340;
    'dispatch: loop {
        match pc {
            0x82E7F340 => {
    //   block [0x82E7F340..0x82E7F39C)
	// 82E7F340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F34C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F350: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F354: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7F358: 4BFFFF49  bl 0x82e7f2a0
	ctx.lr = 0x82E7F35C;
	sub_82E7F2A0(ctx, base);
	// 82E7F35C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F360: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F364: 409A0020  bne cr6, 0x82e7f384
	if !ctx.cr[6].eq {
	pc = 0x82E7F384; continue 'dispatch;
	}
	// 82E7F368: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E7F36C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7F370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F37C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F380: 4E800020  blr
	return;
	// 82E7F384: C03F0024  lfs f1, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E7F388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F3A0 size=68
    let mut pc: u32 = 0x82E7F3A0;
    'dispatch: loop {
        match pc {
            0x82E7F3A0 => {
    //   block [0x82E7F3A0..0x82E7F3E4)
	// 82E7F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F3A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F3AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F3B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F3B4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7F3B8: 4BFFFEE9  bl 0x82e7f2a0
	ctx.lr = 0x82E7F3BC;
	sub_82E7F2A0(ctx, base);
	// 82E7F3BC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F3C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7F3C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F3C8: 419A0008  beq cr6, 0x82e7f3d0
	if ctx.cr[6].eq {
	pc = 0x82E7F3D0; continue 'dispatch;
	}
	// 82E7F3CC: 887F0035  lbz r3, 0x35(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E7F3D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F3DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7F3E8 size=8
    let mut pc: u32 = 0x82E7F3E8;
    'dispatch: loop {
        match pc {
            0x82E7F3E8 => {
    //   block [0x82E7F3E8..0x82E7F3F0)
	// 82E7F3E8: 886300E9  lbz r3, 0xe9(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(233 as u32) ) } as u64;
	// 82E7F3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F3F0 size=60
    let mut pc: u32 = 0x82E7F3F0;
    'dispatch: loop {
        match pc {
            0x82E7F3F0 => {
    //   block [0x82E7F3F0..0x82E7F42C)
	// 82E7F3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F3F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F3FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F404: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7F408: 4BFFFE99  bl 0x82e7f2a0
	ctx.lr = 0x82E7F40C;
	sub_82E7F2A0(ctx, base);
	// 82E7F40C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F410: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 82E7F414: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F430 size=60
    let mut pc: u32 = 0x82E7F430;
    'dispatch: loop {
        match pc {
            0x82E7F430 => {
    //   block [0x82E7F430..0x82E7F46C)
	// 82E7F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F43C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F444: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7F448: 4BFFFE59  bl 0x82e7f2a0
	ctx.lr = 0x82E7F44C;
	sub_82E7F2A0(ctx, base);
	// 82E7F44C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F450: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 82E7F454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7F470 size=16
    let mut pc: u32 = 0x82E7F470;
    'dispatch: loop {
        match pc {
            0x82E7F470 => {
    //   block [0x82E7F470..0x82E7F480)
	// 82E7F470: 81830000  lwz r12, 0(r3)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F474: 816C0010  lwz r11, 0x10(r12)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7F478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E7F47C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7F480 size=240
    let mut pc: u32 = 0x82E7F480;
    'dispatch: loop {
        match pc {
            0x82E7F480 => {
    //   block [0x82E7F480..0x82E7F570)
	// 82E7F480: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7F484: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7F570 size=152
    let mut pc: u32 = 0x82E7F570;
    'dispatch: loop {
        match pc {
            0x82E7F570 => {
    //   block [0x82E7F570..0x82E7F608)
	// 82E7F570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F57C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F580: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E7F584: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E7F588: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E7F58C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F590: 390B6910  addi r8, r11, 0x6910
	ctx.r[8].s64 = ctx.r[11].s64 + 26896;
	// 82E7F594: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82E7F598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7F59C: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7F5A0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E7F5A4: C1A908A8  lfs f13, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7F5A8: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82E7F5AC: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7F5B0: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7F5B4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E7F5B8: D1BF0008  stfs f13, 8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7F5BC: 98FF0014  stb r7, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u8 ) };
	// 82E7F5C0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7F5C4: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82E7F5C8: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E7F5CC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E7F5D0: 13E040C7  vcmpequd (lvx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7F5D4: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F608 size=72
    let mut pc: u32 = 0x82E7F608;
    'dispatch: loop {
        match pc {
            0x82E7F608 => {
    //   block [0x82E7F608..0x82E7F650)
	// 82E7F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F610: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F614: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F61C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7F620: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E7F624: 392BE010  addi r9, r11, -0x1ff0
	ctx.r[9].s64 = ctx.r[11].s64 + -8176;
	// 82E7F628: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7F62C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E7F630: 419A000C  beq cr6, 0x82e7f63c
	if ctx.cr[6].eq {
	pc = 0x82E7F63C; continue 'dispatch;
	}
	// 82E7F634: 4B440C35  bl 0x822c0268
	ctx.lr = 0x82E7F638;
	sub_822C0268(ctx, base);
	// 82E7F638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F63C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F650 size=72
    let mut pc: u32 = 0x82E7F650;
    'dispatch: loop {
        match pc {
            0x82E7F650 => {
    //   block [0x82E7F650..0x82E7F698)
	// 82E7F650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F65C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F664: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7F668: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E7F66C: 392BE01C  addi r9, r11, -0x1fe4
	ctx.r[9].s64 = ctx.r[11].s64 + -8164;
	// 82E7F670: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7F674: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E7F678: 419A000C  beq cr6, 0x82e7f684
	if ctx.cr[6].eq {
	pc = 0x82E7F684; continue 'dispatch;
	}
	// 82E7F67C: 4B440BED  bl 0x822c0268
	ctx.lr = 0x82E7F680;
	sub_822C0268(ctx, base);
	// 82E7F680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7F698 size=156
    let mut pc: u32 = 0x82E7F698;
    'dispatch: loop {
        match pc {
            0x82E7F698 => {
    //   block [0x82E7F698..0x82E7F734)
	// 82E7F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F6A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F6A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F6A8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E7F6AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F6B0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E7F6B4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E7F6B8: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E7F6BC: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7F6C0: 388B0040  addi r4, r11, 0x40
	ctx.r[4].s64 = ctx.r[11].s64 + 64;
	// 82E7F6C4: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E7F6C8: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E7F6CC: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E7F6D0: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E7F6D4: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E7F6D8: C16B000C  lfs f11, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E7F6DC: D17F000C  stfs f11, 0xc(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E7F6E0: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7F6E4: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E7F6E8: 88EB0014  lbz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E7F6EC: 98FF0014  stb r7, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u8 ) };
	// 82E7F6F0: 88CB0015  lbz r6, 0x15(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E7F6F4: 98DF0015  stb r6, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[6].u8 ) };
	// 82E7F6F8: C14B0018  lfs f10, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E7F6FC: D15F0018  stfs f10, 0x18(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E7F700: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E7F704: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82E7F708: 13EB48C7  vcmpequd (lvx128) v31, v11, v9
	tmp.u32 = ctx.r[11].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F738 size=92
    let mut pc: u32 = 0x82E7F738;
    'dispatch: loop {
        match pc {
            0x82E7F738 => {
    //   block [0x82E7F738..0x82E7F794)
	// 82E7F738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F74C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7F750: 4BFFFB51  bl 0x82e7f2a0
	ctx.lr = 0x82E7F754;
	sub_82E7F2A0(ctx, base);
	// 82E7F754: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F758: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F75C: 409A001C  bne cr6, 0x82e7f778
	if !ctx.cr[6].eq {
	pc = 0x82E7F778; continue 'dispatch;
	}
	// 82E7F760: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E7F764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F774: 4E800020  blr
	return;
	// 82E7F778: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E7F77C: 5563B7FE  rlwinm r3, r11, 0x16, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82E7F780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7F784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F78C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7F798 size=20
    let mut pc: u32 = 0x82E7F798;
    'dispatch: loop {
        match pc {
            0x82E7F798 => {
    //   block [0x82E7F798..0x82E7F7AC)
	// 82E7F798: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F79C: 39400090  li r10, 0x90
	ctx.r[10].s64 = 144;
	// 82E7F7A0: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7F7B0 size=28
    let mut pc: u32 = 0x82E7F7B0;
    'dispatch: loop {
        match pc {
            0x82E7F7B0 => {
    //   block [0x82E7F7B0..0x82E7F7CC)
	// 82E7F7B0: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82E7F7B4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7F7B8: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F7D0 size=196
    let mut pc: u32 = 0x82E7F7D0;
    'dispatch: loop {
        match pc {
            0x82E7F7D0 => {
    //   block [0x82E7F7D0..0x82E7F894)
	// 82E7F7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F7D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7F7DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F7E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F7E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7F7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7F7EC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E7F7F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E7F7F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7F7F8: 4B441141  bl 0x822c0938
	ctx.lr = 0x82E7F7FC;
	sub_822C0938(ctx, base);
	// 82E7F7FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7F800: 419A0028  beq cr6, 0x82e7f828
	if ctx.cr[6].eq {
	pc = 0x82E7F828; continue 'dispatch;
	}
	// 82E7F804: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7F808: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E7F80C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E7F810: 392BE180  addi r9, r11, -0x1e80
	ctx.r[9].s64 = ctx.r[11].s64 + -7808;
	// 82E7F814: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E7F818: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E7F81C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E7F820: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E7F824: 48000008  b 0x82e7f82c
	pc = 0x82E7F82C; continue 'dispatch;
	// 82E7F828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E7F82C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7F830: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F834: 409A0044  bne cr6, 0x82e7f878
	if !ctx.cr[6].eq {
	pc = 0x82E7F878; continue 'dispatch;
	}
	// 82E7F838: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E7F83C: 419A001C  beq cr6, 0x82e7f858
	if ctx.cr[6].eq {
	pc = 0x82E7F858; continue 'dispatch;
	}
	// 82E7F840: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F844: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E7F848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7F84C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F850: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E7F854: 4E800421  bctrl
	ctx.lr = 0x82E7F858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E7F858: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E7F85C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E7F860: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E7F864: 392A0828  addi r9, r10, 0x828
	ctx.r[9].s64 = ctx.r[10].s64 + 2088;
	// 82E7F868: 816BBAE0  lwz r11, -0x4520(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17696 as u32) ) } as u64;
	// 82E7F86C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E7F870: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E7F874: 4B44078D  bl 0x822c0000
	ctx.lr = 0x82E7F878;
	sub_822C0000(ctx, base);
	// 82E7F878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7F87C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7F880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7F884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7F888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7F88C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7F890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F898 size=136
    let mut pc: u32 = 0x82E7F898;
    'dispatch: loop {
        match pc {
            0x82E7F898 => {
    //   block [0x82E7F898..0x82E7F920)
	// 82E7F898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F89C: 483288CD  bl 0x831a8168
	ctx.lr = 0x82E7F8A0;
	sub_831A8130(ctx, base);
	// 82E7F8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F8A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7F8A8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E7F8AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E7F8B0: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E7F8B4: 419A0064  beq cr6, 0x82e7f918
	if ctx.cr[6].eq {
	pc = 0x82E7F918; continue 'dispatch;
	}
	// 82E7F8B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F8BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7F8C0: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7F8C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7F8C8: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7F8CC: 419A0040  beq cr6, 0x82e7f90c
	if ctx.cr[6].eq {
	pc = 0x82E7F90C; continue 'dispatch;
	}
	// 82E7F8D0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E7F8D4: 419A0024  beq cr6, 0x82e7f8f8
	if ctx.cr[6].eq {
	pc = 0x82E7F8F8; continue 'dispatch;
	}
	// 82E7F8D8: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82E7F8DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E7F8E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E7F8E4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E7F8E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E7F8EC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E7F8F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E7F8F4: 4082FFE8  bne 0x82e7f8dc
	if !ctx.cr[0].eq {
	pc = 0x82E7F8DC; continue 'dispatch;
	}
	// 82E7F8F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7F8FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7F900: 419A0008  beq cr6, 0x82e7f908
	if ctx.cr[6].eq {
	pc = 0x82E7F908; continue 'dispatch;
	}
	// 82E7F904: 4B440F8D  bl 0x822c0890
	ctx.lr = 0x82E7F908;
	sub_822C0890(ctx, base);
	// 82E7F908: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E7F90C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E7F910: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E7F914: 409AFFA4  bne cr6, 0x82e7f8b8
	if !ctx.cr[6].eq {
	pc = 0x82E7F8B8; continue 'dispatch;
	}
	// 82E7F918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7F91C: 4832889C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7F920 size=12
    let mut pc: u32 = 0x82E7F920;
    'dispatch: loop {
        match pc {
            0x82E7F920 => {
    //   block [0x82E7F920..0x82E7F92C)
	// 82E7F920: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E7F924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7F928: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F92C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7F92C size=20
    let mut pc: u32 = 0x82E7F92C;
    'dispatch: loop {
        match pc {
            0x82E7F92C => {
    //   block [0x82E7F92C..0x82E7F940)
	// 82E7F92C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F930: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E7F934: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7F938: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E7F93C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7F940 size=4
    let mut pc: u32 = 0x82E7F940;
    'dispatch: loop {
        match pc {
            0x82E7F940 => {
    //   block [0x82E7F940..0x82E7F944)
	// 82E7F940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7F948 size=708
    let mut pc: u32 = 0x82E7F948;
    'dispatch: loop {
        match pc {
            0x82E7F948 => {
    //   block [0x82E7F948..0x82E7FC0C)
	// 82E7F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7F950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7F954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7F958: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7F95C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7F960: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E7F964: 387EFFF8  addi r3, r30, -8
	ctx.r[3].s64 = ctx.r[30].s64 + -8;
	// 82E7F968: 4BFFF939  bl 0x82e7f2a0
	ctx.lr = 0x82E7F96C;
	sub_82E7F2A0(ctx, base);
	// 82E7F96C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7F970: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7F974: 409A003C  bne cr6, 0x82e7f9b0
	if !ctx.cr[6].eq {
	pc = 0x82E7F9B0; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E7FC10 size=56
    let mut pc: u32 = 0x82E7FC10;
    'dispatch: loop {
        match pc {
            0x82E7FC10 => {
    //   block [0x82E7FC10..0x82E7FC48)
	// 82E7FC10: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FC14: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E7FC18: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E7FC1C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E7FC20: 396300A0  addi r11, r3, 0xa0
	ctx.r[11].s64 = ctx.r[3].s64 + 160;
	// 82E7FC24: 13E03C07  vcmpneb. (lvlx128) v31, v0, v7
	tmp.u32 = ctx.r[7].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7FC28: 13C83C07  vcmpneb. (lvlx128) v30, v8, v7
	tmp.u32 = ctx.r[8].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7FC2C: 13A93C07  vcmpneb. (lvlx128) v29, v9, v7
	tmp.u32 = ctx.r[9].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E7FC30: 138A3C07  vcmpneb. (lvlx128) v28, v10, v7
	tmp.u32 = ctx.r[10].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FC48 size=72
    let mut pc: u32 = 0x82E7FC48;
    'dispatch: loop {
        match pc {
            0x82E7FC48 => {
    //   block [0x82E7FC48..0x82E7FC90)
	// 82E7FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FC50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FC54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FC5C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E7FC60: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E7FC64: 392BE024  addi r9, r11, -0x1fdc
	ctx.r[9].s64 = ctx.r[11].s64 + -8156;
	// 82E7FC68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E7FC6C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E7FC70: 419A000C  beq cr6, 0x82e7fc7c
	if ctx.cr[6].eq {
	pc = 0x82E7FC7C; continue 'dispatch;
	}
	// 82E7FC74: 4B4405F5  bl 0x822c0268
	ctx.lr = 0x82E7FC78;
	sub_822C0268(ctx, base);
	// 82E7FC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7FC7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7FC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FC88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FC90 size=124
    let mut pc: u32 = 0x82E7FC90;
    'dispatch: loop {
        match pc {
            0x82E7FC90 => {
    //   block [0x82E7FC90..0x82E7FD0C)
	// 82E7FC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FC98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FC9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FCA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FCA4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82E7FCA8: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E7FCAC: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FCB0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E7FCB4: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7FCB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7FCBC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E7FCC0: 419A0024  beq cr6, 0x82e7fce4
	if ctx.cr[6].eq {
	pc = 0x82E7FCE4; continue 'dispatch;
	}
	// 82E7FCC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E7FCC8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E7FCCC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E7FCD0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E7FCD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E7FCD8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E7FCDC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E7FCE0: 4082FFE8  bne 0x82e7fcc8
	if !ctx.cr[0].eq {
	pc = 0x82E7FCC8; continue 'dispatch;
	}
	// 82E7FCE4: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7FCE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7FCEC: 419A0008  beq cr6, 0x82e7fcf4
	if ctx.cr[6].eq {
	pc = 0x82E7FCF4; continue 'dispatch;
	}
	// 82E7FCF0: 4B440BA1  bl 0x822c0890
	ctx.lr = 0x82E7FCF4;
	sub_822C0890(ctx, base);
	// 82E7FCF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7FCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7FCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FD04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FD10 size=92
    let mut pc: u32 = 0x82E7FD10;
    'dispatch: loop {
        match pc {
            0x82E7FD10 => {
    //   block [0x82E7FD10..0x82E7FD6C)
	// 82E7FD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FD14: 48328459  bl 0x831a816c
	ctx.lr = 0x82E7FD18;
	sub_831A8130(ctx, base);
	// 82E7FD18: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E7FD1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FD20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FD24: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E7FD28: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E7FD2C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7FD30: 4BFFF571  bl 0x82e7f2a0
	ctx.lr = 0x82E7FD34;
	sub_82E7F2A0(ctx, base);
	// 82E7FD34: 83DF00FC  lwz r30, 0xfc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E7FD38: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FD3C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7FD40: 419A0020  beq cr6, 0x82e7fd60
	if ctx.cr[6].eq {
	pc = 0x82E7FD60; continue 'dispatch;
	}
	// 82E7FD44: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E7FD48: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FD4C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E7FD50: 48009641  bl 0x82e89390
	ctx.lr = 0x82E7FD54;
	sub_82E89390(ctx, base);
	// 82E7FD54: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FD58: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7FD5C: 409AFFE8  bne cr6, 0x82e7fd44
	if !ctx.cr[6].eq {
	pc = 0x82E7FD44; continue 'dispatch;
	}
	// 82E7FD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E7FD64: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E7FD68: 48328454  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7FD70 size=92
    let mut pc: u32 = 0x82E7FD70;
    'dispatch: loop {
        match pc {
            0x82E7FD70 => {
    //   block [0x82E7FD70..0x82E7FDCC)
	// 82E7FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FD78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FD7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FD80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FD84: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7FD88: D03F002C  stfs f1, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E7FD8C: 4BFFF515  bl 0x82e7f2a0
	ctx.lr = 0x82E7FD90;
	sub_82E7F2A0(ctx, base);
	// 82E7FD90: 815F00FC  lwz r10, 0xfc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E7FD94: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FD98: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7FD9C: 419A001C  beq cr6, 0x82e7fdb8
	if ctx.cr[6].eq {
	pc = 0x82E7FDB8; continue 'dispatch;
	}
	// 82E7FDA0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FDA4: C01F002C  lfs f0, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E7FDA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FDAC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7FDB0: D0090124  stfs f0, 0x124(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 82E7FDB4: 409AFFEC  bne cr6, 0x82e7fda0
	if !ctx.cr[6].eq {
	pc = 0x82E7FDA0; continue 'dispatch;
	}
	// 82E7FDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7FDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FDC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E7FDD0 size=100
    let mut pc: u32 = 0x82E7FDD0;
    'dispatch: loop {
        match pc {
            0x82E7FDD0 => {
    //   block [0x82E7FDD0..0x82E7FE34)
	// 82E7FDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FDD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FDDC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E7FDE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FDE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FDE8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E7FDEC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7FDF0: D3FF0028  stfs f31, 0x28(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E7FDF4: 4BFFF4AD  bl 0x82e7f2a0
	ctx.lr = 0x82E7FDF8;
	sub_82E7F2A0(ctx, base);
	// 82E7FDF8: 815F00FC  lwz r10, 0xfc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E7FDFC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FE00: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7FE04: 419A0018  beq cr6, 0x82e7fe1c
	if ctx.cr[6].eq {
	pc = 0x82E7FE1C; continue 'dispatch;
	}
	// 82E7FE08: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FE0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FE10: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7FE14: D3E9011C  stfs f31, 0x11c(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 82E7FE18: 409AFFF0  bne cr6, 0x82e7fe08
	if !ctx.cr[6].eq {
	pc = 0x82E7FE08; continue 'dispatch;
	}
	// 82E7FE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7FE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FE28: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7FE2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FE38 size=144
    let mut pc: u32 = 0x82E7FE38;
    'dispatch: loop {
        match pc {
            0x82E7FE38 => {
    //   block [0x82E7FE38..0x82E7FEC8)
	// 82E7FE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FE40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FE44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FE48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FE4C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7FE50: 4BFFF451  bl 0x82e7f2a0
	ctx.lr = 0x82E7FE54;
	sub_82E7F2A0(ctx, base);
	// 82E7FE54: 811F00FC  lwz r8, 0xfc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E7FE58: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FE5C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7FE60: 419A0040  beq cr6, 0x82e7fea0
	if ctx.cr[6].eq {
	pc = 0x82E7FEA0; continue 'dispatch;
	}
	// 82E7FE64: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FE68: 814B010C  lwz r10, 0x10c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) } as u64;
	// 82E7FE6C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FE70: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7FE74: 419A0020  beq cr6, 0x82e7fe94
	if ctx.cr[6].eq {
	pc = 0x82E7FE94; continue 'dispatch;
	}
	// 82E7FE78: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FE7C: 80C70160  lwz r6, 0x160(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(352 as u32) ) } as u64;
	// 82E7FE80: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82E7FE84: 41990030  bgt cr6, 0x82e7feb4
	if ctx.cr[6].gt {
	pc = 0x82E7FEB4; continue 'dispatch;
	}
	// 82E7FE88: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FE8C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E7FE90: 409AFFE8  bne cr6, 0x82e7fe78
	if !ctx.cr[6].eq {
	pc = 0x82E7FE78; continue 'dispatch;
	}
	// 82E7FE94: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FE98: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E7FE9C: 409AFFC8  bne cr6, 0x82e7fe64
	if !ctx.cr[6].eq {
	pc = 0x82E7FE64; continue 'dispatch;
	}
	// 82E7FEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7FEA4: 48002135  bl 0x82e81fd8
	ctx.lr = 0x82E7FEA8;
	sub_82E81FD8(ctx, base);
	// 82E7FEA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E7FEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7FEB0: 48002219  bl 0x82e820c8
	ctx.lr = 0x82E7FEB4;
	sub_82E820C8(ctx, base);
	// 82E7FEB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E7FEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FEC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FEC8 size=124
    let mut pc: u32 = 0x82E7FEC8;
    'dispatch: loop {
        match pc {
            0x82E7FEC8 => {
    //   block [0x82E7FEC8..0x82E7FF44)
	// 82E7FEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7FED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FEE0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7FEE4: 4BFFF3BD  bl 0x82e7f2a0
	ctx.lr = 0x82E7FEE8;
	sub_82E7F2A0(ctx, base);
	// 82E7FEE8: 83DF00FC  lwz r30, 0xfc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E7FEEC: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FEF0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7FEF4: 419A0038  beq cr6, 0x82e7ff2c
	if ctx.cr[6].eq {
	pc = 0x82E7FF2C; continue 'dispatch;
	}
	// 82E7FEF8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FEFC: 48002075  bl 0x82e81f70
	ctx.lr = 0x82E7FF00;
	sub_82E81F70(ctx, base);
	// 82E7FF00: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E7FF04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7FF08: 409A0018  bne cr6, 0x82e7ff20
	if !ctx.cr[6].eq {
	pc = 0x82E7FF20; continue 'dispatch;
	}
	// 82E7FF0C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7FF10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FF14: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7FF18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E7FF1C: 4E800421  bctrl
	ctx.lr = 0x82E7FF20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E7FF20: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FF24: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E7FF28: 409AFFD0  bne cr6, 0x82e7fef8
	if !ctx.cr[6].eq {
	pc = 0x82E7FEF8; continue 'dispatch;
	}
	// 82E7FF2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7FF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FF38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7FF3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FF48 size=140
    let mut pc: u32 = 0x82E7FF48;
    'dispatch: loop {
        match pc {
            0x82E7FF48 => {
    //   block [0x82E7FF48..0x82E7FFD4)
	// 82E7FF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7FF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FF5C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7FF60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E7FF64: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FF68: 3BE50004  addi r31, r5, 4
	ctx.r[31].s64 = ctx.r[5].s64 + 4;
	// 82E7FF6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E7FF70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E7FF74: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E7FF78: 419A0024  beq cr6, 0x82e7ff9c
	if ctx.cr[6].eq {
	pc = 0x82E7FF9C; continue 'dispatch;
	}
	// 82E7FF7C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E7FF80: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E7FF84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E7FF88: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E7FF8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E7FF90: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E7FF94: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E7FF98: 4082FFE8  bne 0x82e7ff80
	if !ctx.cr[0].eq {
	pc = 0x82E7FF80; continue 'dispatch;
	}
	// 82E7FF9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E7FFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7FFA4: 4BFFFCED  bl 0x82e7fc90
	ctx.lr = 0x82E7FFA8;
	sub_82E7FC90(ctx, base);
	// 82E7FFA8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7FFAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E7FFB0: 419A0008  beq cr6, 0x82e7ffb8
	if ctx.cr[6].eq {
	pc = 0x82E7FFB8; continue 'dispatch;
	}
	// 82E7FFB4: 4B4408DD  bl 0x822c0890
	ctx.lr = 0x82E7FFB8;
	sub_822C0890(ctx, base);
	// 82E7FFB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7FFBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E7FFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7FFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7FFC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7FFCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E7FFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7FFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E7FFD8 size=124
    let mut pc: u32 = 0x82E7FFD8;
    'dispatch: loop {
        match pc {
            0x82E7FFD8 => {
    //   block [0x82E7FFD8..0x82E80054)
	// 82E7FFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7FFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E7FFE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E7FFE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7FFE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E7FFEC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E7FFF0: 909F0030  stw r4, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82E7FFF4: 4BFFF2AD  bl 0x82e7f2a0
	ctx.lr = 0x82E7FFF8;
	sub_82E7F2A0(ctx, base);
	// 82E7FFF8: 80FF00FC  lwz r7, 0xfc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E7FFFC: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80000: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E80004: 419A003C  beq cr6, 0x82e80040
	if ctx.cr[6].eq {
	pc = 0x82E80040; continue 'dispatch;
	}
	// 82E80008: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8000C: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E80010: 814B010C  lwz r10, 0x10c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) } as u64;
	// 82E80014: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80018: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E8001C: 419A0018  beq cr6, 0x82e80034
	if ctx.cr[6].eq {
	pc = 0x82E80034; continue 'dispatch;
	}
	// 82E80020: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80024: 91260198  stw r9, 0x198(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(408 as u32), ctx.r[9].u32 ) };
	// 82E80028: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8002C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E80030: 409AFFF0  bne cr6, 0x82e80020
	if !ctx.cr[6].eq {
	pc = 0x82E80020; continue 'dispatch;
	}
	// 82E80034: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80038: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E8003C: 409AFFCC  bne cr6, 0x82e80008
	if !ctx.cr[6].eq {
	pc = 0x82E80008; continue 'dispatch;
	}
	// 82E80040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E80044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E8004C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80058 size=128
    let mut pc: u32 = 0x82E80058;
    'dispatch: loop {
        match pc {
            0x82E80058 => {
    //   block [0x82E80058..0x82E800D8)
	// 82E80058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8005C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E80064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8006C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E80074: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E80078: 4BFFF229  bl 0x82e7f2a0
	ctx.lr = 0x82E8007C;
	sub_82E7F2A0(ctx, base);
	// 82E8007C: 811F00FC  lwz r8, 0xfc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E80080: 81280000  lwz r9, 0(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80084: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E80088: 419A0038  beq cr6, 0x82e800c0
	if ctx.cr[6].eq {
	pc = 0x82E800C0; continue 'dispatch;
	}
	// 82E8008C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80090: 814B010C  lwz r10, 0x10c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) } as u64;
	// 82E80094: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80098: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E8009C: 419A0018  beq cr6, 0x82e800b4
	if ctx.cr[6].eq {
	pc = 0x82E800B4; continue 'dispatch;
	}
	// 82E800A0: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E800A4: 9BC701A4  stb r30, 0x1a4(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(420 as u32), ctx.r[30].u8 ) };
	// 82E800A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E800AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E800B0: 409AFFF0  bne cr6, 0x82e800a0
	if !ctx.cr[6].eq {
	pc = 0x82E800A0; continue 'dispatch;
	}
	// 82E800B4: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E800B8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E800BC: 409AFFD0  bne cr6, 0x82e8008c
	if !ctx.cr[6].eq {
	pc = 0x82E8008C; continue 'dispatch;
	}
	// 82E800C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E800C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E800C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E800CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E800D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E800D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E800D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E800D8 size=132
    let mut pc: u32 = 0x82E800D8;
    'dispatch: loop {
        match pc {
            0x82E800D8 => {
    //   block [0x82E800D8..0x82E8015C)
	// 82E800D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E800DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E800E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E800E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E800E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E800EC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E800F0: D03F0024  stfs f1, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E800F4: 4BFFF1AD  bl 0x82e7f2a0
	ctx.lr = 0x82E800F8;
	sub_82E7F2A0(ctx, base);
	// 82E800F8: 80FF00FC  lwz r7, 0xfc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E800FC: 81070000  lwz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80100: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E80104: 419A0044  beq cr6, 0x82e80148
	if ctx.cr[6].eq {
	pc = 0x82E80148; continue 'dispatch;
	}
	// 82E80108: 81280008  lwz r9, 8(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8010C: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E80110: 8149010C  lwz r10, 0x10c(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(268 as u32) ) } as u64;
	// 82E80114: D0090118  stfs f0, 0x118(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 82E80118: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8011C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E80120: 419A001C  beq cr6, 0x82e8013c
	if ctx.cr[6].eq {
	pc = 0x82E8013C; continue 'dispatch;
	}
	// 82E80124: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80128: C0090118  lfs f0, 0x118(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(280 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E8012C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80130: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E80134: D0060190  stfs f0, 0x190(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(400 as u32), tmp.u32 ) };
	// 82E80138: 409AFFEC  bne cr6, 0x82e80124
	if !ctx.cr[6].eq {
	pc = 0x82E80124; continue 'dispatch;
	}
	// 82E8013C: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80140: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E80144: 409AFFC4  bne cr6, 0x82e80108
	if !ctx.cr[6].eq {
	pc = 0x82E80108; continue 'dispatch;
	}
	// 82E80148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E8014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80160 size=32
    let mut pc: u32 = 0x82E80160;
    'dispatch: loop {
        match pc {
            0x82E80160 => {
    //   block [0x82E80160..0x82E80180)
	// 82E80160: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82E80164: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80168: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E8016C: 88830014  lbz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E80170: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80174: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E80178: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E8017C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E80180 size=36
    let mut pc: u32 = 0x82E80180;
    'dispatch: loop {
        match pc {
            0x82E80180 => {
    //   block [0x82E80180..0x82E801A4)
	// 82E80180: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82E80184: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80188: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E8018C: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E80190: 88A30018  lbz r5, 0x18(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E80194: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80198: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E8019C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E801A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E801A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E801A8 size=32
    let mut pc: u32 = 0x82E801A8;
    'dispatch: loop {
        match pc {
            0x82E801A8 => {
    //   block [0x82E801A8..0x82E801C8)
	// 82E801A8: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82E801AC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E801B0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E801B4: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E801B8: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E801BC: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E801C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E801C4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E801C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E801C8 size=32
    let mut pc: u32 = 0x82E801C8;
    'dispatch: loop {
        match pc {
            0x82E801C8 => {
    //   block [0x82E801C8..0x82E801E8)
	// 82E801C8: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82E801CC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E801D0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E801D4: 80830014  lwz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E801D8: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E801DC: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E801E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E801E4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E801E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E801E8 size=24
    let mut pc: u32 = 0x82E801E8;
    'dispatch: loop {
        match pc {
            0x82E801E8 => {
    //   block [0x82E801E8..0x82E80200)
	// 82E801E8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E801EC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E801F0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E801F4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E801F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E801FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80200 size=16
    let mut pc: u32 = 0x82E80200;
    'dispatch: loop {
        match pc {
            0x82E80200 => {
    //   block [0x82E80200..0x82E80210)
	// 82E80200: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E80204: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E80208: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E8020C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80210 size=84
    let mut pc: u32 = 0x82E80210;
    'dispatch: loop {
        match pc {
            0x82E80210 => {
    //   block [0x82E80210..0x82E80264)
	// 82E80210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80214: 419A003C  beq cr6, 0x82e80250
	if ctx.cr[6].eq {
	pc = 0x82E80250; continue 'dispatch;
	}
	// 82E80218: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8021C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E80220: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80224: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E80228: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E8022C: 419A0024  beq cr6, 0x82e80250
	if ctx.cr[6].eq {
	pc = 0x82E80250; continue 'dispatch;
	}
	// 82E80230: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E80234: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82E80238: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8023C: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82E80240: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E80244: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E80248: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8024C: 4082FFE8  bne 0x82e80234
	if !ctx.cr[0].eq {
	pc = 0x82E80234; continue 'dispatch;
	}
	// 82E80250: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E80254: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82E80258: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E8025C: 409AFFB4  bne cr6, 0x82e80210
	if !ctx.cr[6].eq {
	pc = 0x82E80210; continue 'dispatch;
	}
	// 82E80260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80268 size=228
    let mut pc: u32 = 0x82E80268;
    'dispatch: loop {
        match pc {
            0x82E80268 => {
    //   block [0x82E80268..0x82E8034C)
	// 82E80268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8026C: 48327F01  bl 0x831a816c
	ctx.lr = 0x82E80270;
	sub_831A8130(ctx, base);
	// 82E80270: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80274: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E80278: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8027C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E80280: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E80284: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E80288: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8028C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E80294: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E80298: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82E8029C: 419A0024  beq cr6, 0x82e802c0
	if ctx.cr[6].eq {
	pc = 0x82E802C0; continue 'dispatch;
	}
	// 82E802A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E802A4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 82E802A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E802AC: 7D005828  lwarx r8, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 82E802B0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E802B4: 7D00592D  stwcx. r8, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E802B8: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E802BC: 4082FFE8  bne 0x82e802a4
	if !ctx.cr[0].eq {
	pc = 0x82E802A4; continue 'dispatch;
	}
	// 82E802C0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E802C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E802C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E802CC: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E802D0: 4BFFFC79  bl 0x82e7ff48
	ctx.lr = 0x82E802D4;
	sub_82E7FF48(ctx, base);
	// 82E802D4: FBDF0000  std r30, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82E802D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E802DC: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E802E0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E802E4: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E802E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E802EC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E802F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E802F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E802F8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E802FC: 419A0024  beq cr6, 0x82e80320
	if ctx.cr[6].eq {
	pc = 0x82E80320; continue 'dispatch;
	}
	// 82E80300: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E80304: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E80308: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8030C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E80310: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E80314: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E80318: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8031C: 4082FFE8  bne 0x82e80304
	if !ctx.cr[0].eq {
	pc = 0x82E80304; continue 'dispatch;
	}
	// 82E80320: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E80324: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80328: 419A0008  beq cr6, 0x82e80330
	if ctx.cr[6].eq {
	pc = 0x82E80330; continue 'dispatch;
	}
	// 82E8032C: 4B440565  bl 0x822c0890
	ctx.lr = 0x82E80330;
	sub_822C0890(ctx, base);
	// 82E80330: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80334: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80338: 419A0008  beq cr6, 0x82e80340
	if ctx.cr[6].eq {
	pc = 0x82E80340; continue 'dispatch;
	}
	// 82E8033C: 4B440555  bl 0x822c0890
	ctx.lr = 0x82E80340;
	sub_822C0890(ctx, base);
	// 82E80340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80344: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E80348: 48327E74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80350 size=168
    let mut pc: u32 = 0x82E80350;
    'dispatch: loop {
        match pc {
            0x82E80350 => {
    //   block [0x82E80350..0x82E803F8)
	// 82E80350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E8035C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80364: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80368: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E8036C: 392AE054  addi r9, r10, -0x1fac
	ctx.r[9].s64 = ctx.r[10].s64 + -8108;
	// 82E80370: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E80374: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E80378: 39440008  addi r10, r4, 8
	ctx.r[10].s64 = ctx.r[4].s64 + 8;
	// 82E8037C: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80380: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E80384: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E80388: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8038C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82E80390: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80394: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82E80398: 80A4000C  lwz r5, 0xc(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E8039C: 90BF0014  stw r5, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82E803A0: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E803A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E803A8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E803AC: 419A0024  beq cr6, 0x82e803d0
	if ctx.cr[6].eq {
	pc = 0x82E803D0; continue 'dispatch;
	}
	// 82E803B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E803B4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E803B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E803BC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E803C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E803C4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E803C8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E803CC: 4082FFE8  bne 0x82e803b4
	if !ctx.cr[0].eq {
	pc = 0x82E803B4; continue 'dispatch;
	}
	// 82E803D0: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E803D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E803D8: 419A0008  beq cr6, 0x82e803e0
	if ctx.cr[6].eq {
	pc = 0x82E803E0; continue 'dispatch;
	}
	// 82E803DC: 4B4404B5  bl 0x822c0890
	ctx.lr = 0x82E803E0;
	sub_822C0890(ctx, base);
	// 82E803E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E803E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E803E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E803EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E803F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E803F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E803F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E803F8 size=36
    let mut pc: u32 = 0x82E803F8;
    'dispatch: loop {
        match pc {
            0x82E803F8 => {
    //   block [0x82E803F8..0x82E8041C)
	// 82E803F8: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82E803FC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80400: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E80404: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E80408: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8040C: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E80410: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E80414: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E80418: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80420 size=16
    let mut pc: u32 = 0x82E80420;
    'dispatch: loop {
        match pc {
            0x82E80420 => {
    //   block [0x82E80420..0x82E80430)
	// 82E80420: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E80424: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82E80428: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E8042C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80430 size=76
    let mut pc: u32 = 0x82E80430;
    'dispatch: loop {
        match pc {
            0x82E80430 => {
    //   block [0x82E80430..0x82E8047C)
	// 82E80430: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E80434: 419A003C  beq cr6, 0x82e80470
	if ctx.cr[6].eq {
	pc = 0x82E80470; continue 'dispatch;
	}
	// 82E80438: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8043C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E80440: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E80448: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E8044C: 419A0024  beq cr6, 0x82e80470
	if ctx.cr[6].eq {
	pc = 0x82E80470; continue 'dispatch;
	}
	// 82E80450: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E80454: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82E80458: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8045C: 7D205828  lwarx r9, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82E80460: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E80464: 7D20592D  stwcx. r9, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E80468: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8046C: 4082FFE8  bne 0x82e80454
	if !ctx.cr[0].eq {
	pc = 0x82E80454; continue 'dispatch;
	}
	// 82E80470: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82E80474: 4200FFBC  bdnz 0x82e80430
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E80430; continue 'dispatch;
	}
	// 82E80478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80480 size=104
    let mut pc: u32 = 0x82E80480;
    'dispatch: loop {
        match pc {
            0x82E80480 => {
    //   block [0x82E80480..0x82E804E8)
	// 82E80480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E8048C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80498: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E8049C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E804A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E804A4: 419A0008  beq cr6, 0x82e804ac
	if ctx.cr[6].eq {
	pc = 0x82E804AC; continue 'dispatch;
	}
	// 82E804A8: 4B4403E9  bl 0x822c0890
	ctx.lr = 0x82E804AC;
	sub_822C0890(ctx, base);
	// 82E804AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E804B0: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E804B4: 392BE024  addi r9, r11, -0x1fdc
	ctx.r[9].s64 = ctx.r[11].s64 + -8156;
	// 82E804B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E804BC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E804C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E804C4: 419A000C  beq cr6, 0x82e804d0
	if ctx.cr[6].eq {
	pc = 0x82E804D0; continue 'dispatch;
	}
	// 82E804C8: 4B43FDA1  bl 0x822c0268
	ctx.lr = 0x82E804CC;
	sub_822C0268(ctx, base);
	// 82E804CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E804D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E804D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E804D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E804DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E804E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E804E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E804E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E804E8 size=140
    let mut pc: u32 = 0x82E804E8;
    'dispatch: loop {
        match pc {
            0x82E804E8 => {
    //   block [0x82E804E8..0x82E80574)
	// 82E804E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E804EC: 48327C7D  bl 0x831a8168
	ctx.lr = 0x82E804F0;
	sub_831A8130(ctx, base);
	// 82E804F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E804F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E804F8: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82E804FC: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 82E80500: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E80504: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80508: 419A0020  beq cr6, 0x82e80528
	if ctx.cr[6].eq {
	pc = 0x82E80528; continue 'dispatch;
	}
	// 82E8050C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80510: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E80514: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80518: 48005921  bl 0x82e85e38
	ctx.lr = 0x82E8051C;
	sub_82E85E38(ctx, base);
	// 82E8051C: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E80520: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80524: 4BF71C65  bl 0x82df2188
	ctx.lr = 0x82E80528;
	sub_82DF2188(ctx, base);
	// 82E80528: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E8052C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E80530: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E80534: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E80538: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8053C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80540: 419A0020  beq cr6, 0x82e80560
	if ctx.cr[6].eq {
	pc = 0x82E80560; continue 'dispatch;
	}
	// 82E80544: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E80548: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E8054C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80550: 480058E9  bl 0x82e85e38
	ctx.lr = 0x82E80554;
	sub_82E85E38(ctx, base);
	// 82E80554: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E80558: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8055C: 4BF71C2D  bl 0x82df2188
	ctx.lr = 0x82E80560;
	sub_82DF2188(ctx, base);
	// 82E80560: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E80564: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E80568: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E8056C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E80570: 48327C48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80578 size=240
    let mut pc: u32 = 0x82E80578;
    'dispatch: loop {
        match pc {
            0x82E80578 => {
    //   block [0x82E80578..0x82E80668)
	// 82E80578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8058C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E80590: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80594: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E80598: 390BE070  addi r8, r11, -0x1f90
	ctx.r[8].s64 = ctx.r[11].s64 + -8080;
	// 82E8059C: 38EAE068  addi r7, r10, -0x1f98
	ctx.r[7].s64 = ctx.r[10].s64 + -8088;
	// 82E805A0: 38C9E05C  addi r6, r9, -0x1fa4
	ctx.r[6].s64 = ctx.r[9].s64 + -8100;
	// 82E805A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E805A8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82E805AC: 90DF001C  stw r6, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 82E805B0: 807F0128  lwz r3, 0x128(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 82E805B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E805B8: 419A003C  beq cr6, 0x82e805f4
	if ctx.cr[6].eq {
	pc = 0x82E805F4; continue 'dispatch;
	}
	// 82E805BC: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82E805C0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 82E805C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E805C8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82E805CC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E805D0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E805D4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E805D8: 4082FFE8  bne 0x82e805c0
	if !ctx.cr[0].eq {
	pc = 0x82E805C0; continue 'dispatch;
	}
	// 82E805DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E805E0: 409A0014  bne cr6, 0x82e805f4
	if !ctx.cr[6].eq {
	pc = 0x82E805F4; continue 'dispatch;
	}
	// 82E805E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E805E8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E805EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E805F0: 4E800421  bctrl
	ctx.lr = 0x82E805F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E805F4: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E805F8: 4BFFFEF1  bl 0x82e804e8
	ctx.lr = 0x82E805FC;
	sub_82E804E8(ctx, base);
	// 82E805FC: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 82E80600: 4BFFD781  bl 0x82e7dd80
	ctx.lr = 0x82E80604;
	sub_82E7DD80(ctx, base);
	// 82E80604: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E80608: 809F00FC  lwz r4, 0xfc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E8060C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E80610: 4BF71B79  bl 0x82df2188
	ctx.lr = 0x82E80614;
	sub_82DF2188(ctx, base);
	// 82E80614: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E80618: 915F00FC  stw r10, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[10].u32 ) };
	// 82E8061C: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82E80620: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80624: 419A0008  beq cr6, 0x82e8062c
	if ctx.cr[6].eq {
	pc = 0x82E8062C; continue 'dispatch;
	}
	// 82E80628: 4B440269  bl 0x822c0890
	ctx.lr = 0x82E8062C;
	sub_822C0890(ctx, base);
	// 82E8062C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82E80630: 4BF72DF9  bl 0x82df3428
	ctx.lr = 0x82E80634;
	sub_82DF3428(ctx, base);
	// 82E80634: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E80638: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E8063C: 392BE010  addi r9, r11, -0x1ff0
	ctx.r[9].s64 = ctx.r[11].s64 + -8176;
	// 82E80640: 390AE01C  addi r8, r10, -0x1fe4
	ctx.r[8].s64 = ctx.r[10].s64 + -8164;
	// 82E80644: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 82E80648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8064C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82E80650: 48001871  bl 0x82e81ec0
	ctx.lr = 0x82E80654;
	sub_82E81EC0(ctx, base);
	// 82E80654: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E80658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E8065C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80660: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80668 size=8
    let mut pc: u32 = 0x82E80668;
    'dispatch: loop {
        match pc {
            0x82E80668 => {
    //   block [0x82E80668..0x82E80670)
	// 82E80668: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82E8066C: 48000194  b 0x82e80800
	sub_82E80800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E80670 size=8
    let mut pc: u32 = 0x82E80670;
    'dispatch: loop {
        match pc {
            0x82E80670 => {
    //   block [0x82E80670..0x82E80678)
	// 82E80670: 3863FFE4  addi r3, r3, -0x1c
	ctx.r[3].s64 = ctx.r[3].s64 + -28;
	// 82E80674: 4800018C  b 0x82e80800
	sub_82E80800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80678 size=108
    let mut pc: u32 = 0x82E80678;
    'dispatch: loop {
        match pc {
            0x82E80678 => {
    //   block [0x82E80678..0x82E806E4)
	// 82E80678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8067C: 48327AE9  bl 0x831a8164
	ctx.lr = 0x82E80680;
	sub_831A8130(ctx, base);
	// 82E80680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80688: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E8068C: 3BBF00F8  addi r29, r31, 0xf8
	ctx.r[29].s64 = ctx.r[31].s64 + 248;
	// 82E80690: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E80694: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E80698: 839F00FC  lwz r28, 0xfc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E8069C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E806A0: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E806A4: 4800939D  bl 0x82e89a40
	ctx.lr = 0x82E806A8;
	sub_82E89A40(ctx, base);
	// 82E806A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E806AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E806B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E806B4: 48009945  bl 0x82e89ff8
	ctx.lr = 0x82E806B8;
	sub_82E89FF8(ctx, base);
	// 82E806B8: 937C0004  stw r27, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82E806BC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E806C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E806C4: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E806C8: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82E806CC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E806D0: 409A0008  bne cr6, 0x82e806d8
	if !ctx.cr[6].eq {
	pc = 0x82E806D8; continue 'dispatch;
	}
	// 82E806D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E806D8: 916A00F8  stw r11, 0xf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 82E806DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E806E0: 48327AD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E806E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E806E8 size=276
    let mut pc: u32 = 0x82E806E8;
    'dispatch: loop {
        match pc {
            0x82E806E8 => {
    //   block [0x82E806E8..0x82E807FC)
	// 82E806E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E806EC: 48327A75  bl 0x831a8160
	ctx.lr = 0x82E806F0;
	sub_831A8130(ctx, base);
	// 82E806F0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E806F4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E806F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E806FC: 419A0014  beq cr6, 0x82e80710
	if ctx.cr[6].eq {
	pc = 0x82E80710; continue 'dispatch;
	}
	// 82E80700: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80704: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E80708: 7D2B1E71  srawi. r11, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E8070C: 40820014  bne 0x82e80720
	if !ctx.cr[0].eq {
	pc = 0x82E80720; continue 'dispatch;
	}
	// 82E80710: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 82E80714: 48005F25  bl 0x82e86638
	ctx.lr = 0x82E80718;
	sub_82E86638(ctx, base);
	// 82E80718: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E8071C: 48327A94  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E80720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E80724: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80728: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82E8072C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E80730: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E80734: 83830008  lwz r28, 8(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80738: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82E8073C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E80740: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80744: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E80748: 83630014  lwz r27, 0x14(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E8074C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E80750: 83430018  lwz r26, 0x18(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E80754: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E80758: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E8075C: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82E80760: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E80764: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82E80768: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E8076C: 419A0034  beq cr6, 0x82e807a0
	if ctx.cr[6].eq {
	pc = 0x82E807A0; continue 'dispatch;
	}
	// 82E80770: 7D7FE050  subf r11, r31, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82E80774: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82E80778: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E8077C: 40980024  bge cr6, 0x82e807a0
	if !ctx.cr[6].lt {
	pc = 0x82E807A0; continue 'dispatch;
	}
	// 82E80780: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80788: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8078C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E80790: 4E800421  bctrl
	ctx.lr = 0x82E80794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E80794: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E80798: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82E8079C: 4BFFFFCC  b 0x82e80768
	pc = 0x82E80768; continue 'dispatch;
	// 82E807A0: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82E807A4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E807A8: 419A0024  beq cr6, 0x82e807cc
	if ctx.cr[6].eq {
	pc = 0x82E807CC; continue 'dispatch;
	}
	// 82E807AC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E807B0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E807B4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E807B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E807BC: 4800567D  bl 0x82e85e38
	ctx.lr = 0x82E807C0;
	sub_82E85E38(ctx, base);
	// 82E807C0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E807C4: 807E110C  lwz r3, 0x110c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E807C8: 4BF719C1  bl 0x82df2188
	ctx.lr = 0x82E807CC;
	sub_82DF2188(ctx, base);
	// 82E807CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E807D0: 419A0024  beq cr6, 0x82e807f4
	if ctx.cr[6].eq {
	pc = 0x82E807F4; continue 'dispatch;
	}
	// 82E807D4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E807D8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E807DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E807E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E807E4: 48005655  bl 0x82e85e38
	ctx.lr = 0x82E807E8;
	sub_82E85E38(ctx, base);
	// 82E807E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E807EC: 807E110C  lwz r3, 0x110c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E807F0: 4BF71999  bl 0x82df2188
	ctx.lr = 0x82E807F4;
	sub_82DF2188(ctx, base);
	// 82E807F4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E807F8: 483279B8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80800 size=80
    let mut pc: u32 = 0x82E80800;
    'dispatch: loop {
        match pc {
            0x82E80800 => {
    //   block [0x82E80800..0x82E80850)
	// 82E80800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E8080C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80818: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E8081C: 4BFFFD5D  bl 0x82e80578
	ctx.lr = 0x82E80820;
	sub_82E80578(ctx, base);
	// 82E80820: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E80824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80828: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8082C: 419A000C  beq cr6, 0x82e80838
	if ctx.cr[6].eq {
	pc = 0x82E80838; continue 'dispatch;
	}
	// 82E80830: 4BF71BA9  bl 0x82df23d8
	ctx.lr = 0x82E80834;
	sub_82DF23D8(ctx, base);
	// 82E80834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80838: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E8083C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80844: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E80848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E8084C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E80850 size=444
    let mut pc: u32 = 0x82E80850;
    'dispatch: loop {
        match pc {
            0x82E80850 => {
    //   block [0x82E80850..0x82E80A0C)
	// 82E80850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80854: 48327915  bl 0x831a8168
	ctx.lr = 0x82E80858;
	sub_831A8130(ctx, base);
	// 82E80858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8085C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80860: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E80864: 48001625  bl 0x82e81e88
	ctx.lr = 0x82E80868;
	sub_82E81E88(ctx, base);
	// 82E80868: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E8086C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80870: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E80874: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82E80878: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82E8087C: 38CBE01C  addi r6, r11, -0x1fe4
	ctx.r[6].s64 = ctx.r[11].s64 + -8164;
	// 82E80880: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E80884: 38AAE010  addi r5, r10, -0x1ff0
	ctx.r[5].s64 = ctx.r[10].s64 + -8176;
	// 82E80888: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82E8088C: 3968E068  addi r11, r8, -0x1f98
	ctx.r[11].s64 = ctx.r[8].s64 + -8088;
	// 82E80890: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E80894: 3889E070  addi r4, r9, -0x1f90
	ctx.r[4].s64 = ctx.r[9].s64 + -8080;
	// 82E80898: 9BDF0019  stb r30, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[30].u8 ) };
	// 82E8089C: 3947E05C  addi r10, r7, -0x1fa4
	ctx.r[10].s64 = ctx.r[7].s64 + -8100;
	// 82E808A0: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82E808A4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E808A8: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E808AC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E808B0: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82E808B4: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E808B8: 4BFFECB9  bl 0x82e7f570
	ctx.lr = 0x82E808BC;
	sub_82E7F570(ctx, base);
	// 82E808BC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82E808C0: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82E808C4: 38696880  addi r3, r9, 0x6880
	ctx.r[3].s64 = ctx.r[9].s64 + 26752;
	// 82E808C8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E808CC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E808D0: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E808D4: 397F00A0  addi r11, r31, 0xa0
	ctx.r[11].s64 = ctx.r[31].s64 + 160;
	// 82E808D8: C00708A8  lfs f0, 0x8a8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E808DC: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82E808E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E808E4: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82E808E8: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82E808EC: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82E808F0: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82E808F4: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82E808F8: 388508B0  addi r4, r5, 0x8b0
	ctx.r[4].s64 = ctx.r[5].s64 + 2224;
	// 82E808FC: 13E81C07  vcmpneb. (lvlx128) v31, v8, v3
	tmp.u32 = ctx.r[8].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E80900: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E80904: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E80908: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E8090C: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E80910: 13801C07  vcmpneb. (lvlx128) v28, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E80A10 size=332
    let mut pc: u32 = 0x82E80A10;
    'dispatch: loop {
        match pc {
            0x82E80A10 => {
    //   block [0x82E80A10..0x82E80B5C)
	// 82E80A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80A14: 48327755  bl 0x831a8168
	ctx.lr = 0x82E80A18;
	sub_831A8130(ctx, base);
	// 82E80A18: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E80A1C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80A20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80A24: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E80A28: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E80A2C: 4BFFEC6D  bl 0x82e7f698
	ctx.lr = 0x82E80A30;
	sub_82E7F698(ctx, base);
	// 82E80A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80A34: C03F0020  lfs f1, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E80A38: 480014C1  bl 0x82e81ef8
	ctx.lr = 0x82E80A3C;
	sub_82E81EF8(ctx, base);
	// 82E80A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80A40: 889F0034  lbz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E80A44: 4800153D  bl 0x82e81f80
	ctx.lr = 0x82E80A48;
	sub_82E81F80(ctx, base);
	// 82E80A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80A4C: C03F0024  lfs f1, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E80A50: 4BFFF689  bl 0x82e800d8
	ctx.lr = 0x82E80A54;
	sub_82E800D8(ctx, base);
	// 82E80A54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80A58: C03F0028  lfs f1, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E80A5C: 4BFFF375  bl 0x82e7fdd0
	ctx.lr = 0x82E80A60;
	sub_82E7FDD0(ctx, base);
	// 82E80A60: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E80A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80A68: 616A0400  ori r10, r11, 0x400
	ctx.r[10].u64 = ctx.r[11].u64 | 1024;
	// 82E80A6C: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E80A70: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E80A74: 4BFFF565  bl 0x82e7ffd8
	ctx.lr = 0x82E80A78;
	sub_82E7FFD8(ctx, base);
	// 82E80A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E80A7C: C03F002C  lfs f1, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E80A80: 4BFFF2F1  bl 0x82e7fd70
	ctx.lr = 0x82E80A84;
	sub_82E7FD70(ctx, base);
	// 82E80A84: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82E80A88: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82E80A8C: 38E00070  li r7, 0x70
	ctx.r[7].s64 = 112;
	// 82E80A90: 38C96880  addi r6, r9, 0x6880
	ctx.r[6].s64 = ctx.r[9].s64 + 26752;
	// 82E80A94: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82E80A98: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82E80A9C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82E80AA0: 13FF40C7  vcmpequd (lvx128) v31, v31, v8
	tmp.u32 = ctx.r[31].u32 + ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80B60 size=196
    let mut pc: u32 = 0x82E80B60;
    'dispatch: loop {
        match pc {
            0x82E80B60 => {
    //   block [0x82E80B60..0x82E80C24)
	// 82E80B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E80B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80B74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E80B78: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 82E80B7C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E80B80: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82E80B84: 4B43FDB5  bl 0x822c0938
	ctx.lr = 0x82E80B88;
	sub_822C0938(ctx, base);
	// 82E80B88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80B8C: 419A0040  beq cr6, 0x82e80bcc
	if ctx.cr[6].eq {
	pc = 0x82E80BCC; continue 'dispatch;
	}
	// 82E80B90: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82E80B94: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80B98: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82E80B9C: 390AE02C  addi r8, r10, -0x1fd4
	ctx.r[8].s64 = ctx.r[10].s64 + -8148;
	// 82E80BA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80BA4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80BA8: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80BAC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80BB0: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80BB4: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E80BB8: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E80BBC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E80BC0: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82E80BC4: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E80BC8: 48000008  b 0x82e80bd0
	pc = 0x82E80BD0; continue 'dispatch;
	// 82E80BCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E80BD0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E80BD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80BD8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80BDC: 4BFFEBF5  bl 0x82e7f7d0
	ctx.lr = 0x82E80BE0;
	sub_82E7F7D0(ctx, base);
	// 82E80BE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80BE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80BE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80BEC: 4B43F415  bl 0x822c0000
	ctx.lr = 0x82E80BF0;
	sub_822C0000(ctx, base);
	// 82E80BF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E80BF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E80BF8: 480065A9  bl 0x82e871a0
	ctx.lr = 0x82E80BFC;
	sub_82E871A0(ctx, base);
	// 82E80BFC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E80C00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80C04: 419A0008  beq cr6, 0x82e80c0c
	if ctx.cr[6].eq {
	pc = 0x82E80C0C; continue 'dispatch;
	}
	// 82E80C08: 4B43FC89  bl 0x822c0890
	ctx.lr = 0x82E80C0C;
	sub_822C0890(ctx, base);
	// 82E80C0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E80C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80C18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E80C1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80C28 size=196
    let mut pc: u32 = 0x82E80C28;
    'dispatch: loop {
        match pc {
            0x82E80C28 => {
    //   block [0x82E80C28..0x82E80CEC)
	// 82E80C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E80C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80C3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E80C40: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 82E80C44: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E80C48: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82E80C4C: F8C100A0  std r6, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[6].u64 ) };
	// 82E80C50: 4B43FCE9  bl 0x822c0938
	ctx.lr = 0x82E80C54;
	sub_822C0938(ctx, base);
	// 82E80C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80C58: 419A003C  beq cr6, 0x82e80c94
	if ctx.cr[6].eq {
	pc = 0x82E80C94; continue 'dispatch;
	}
	// 82E80C5C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E80C60: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 82E80C64: 392BE034  addi r9, r11, -0x1fcc
	ctx.r[9].s64 = ctx.r[11].s64 + -8140;
	// 82E80C68: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82E80C6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E80C70: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82E80C74: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E80C78: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80C7C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E80C80: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E80C84: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E80C88: 4200FFF0  bdnz 0x82e80c78
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E80C78; continue 'dispatch;
	}
	// 82E80C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80C90: 48000008  b 0x82e80c98
	pc = 0x82E80C98; continue 'dispatch;
	// 82E80C94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E80C98: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E80C9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80CA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80CA4: 4BFFEB2D  bl 0x82e7f7d0
	ctx.lr = 0x82E80CA8;
	sub_82E7F7D0(ctx, base);
	// 82E80CA8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80CAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80CB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80CB4: 4B43F34D  bl 0x822c0000
	ctx.lr = 0x82E80CB8;
	sub_822C0000(ctx, base);
	// 82E80CB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E80CBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E80CC0: 480064E1  bl 0x82e871a0
	ctx.lr = 0x82E80CC4;
	sub_82E871A0(ctx, base);
	// 82E80CC4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E80CC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80CCC: 419A0008  beq cr6, 0x82e80cd4
	if ctx.cr[6].eq {
	pc = 0x82E80CD4; continue 'dispatch;
	}
	// 82E80CD0: 4B43FBC1  bl 0x822c0890
	ctx.lr = 0x82E80CD4;
	sub_822C0890(ctx, base);
	// 82E80CD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E80CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E80CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80CF0 size=196
    let mut pc: u32 = 0x82E80CF0;
    'dispatch: loop {
        match pc {
            0x82E80CF0 => {
    //   block [0x82E80CF0..0x82E80DB4)
	// 82E80CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E80CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80D04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E80D08: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 82E80D0C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E80D10: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82E80D14: 4B43FC25  bl 0x822c0938
	ctx.lr = 0x82E80D18;
	sub_822C0938(ctx, base);
	// 82E80D18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80D1C: 419A0040  beq cr6, 0x82e80d5c
	if ctx.cr[6].eq {
	pc = 0x82E80D5C; continue 'dispatch;
	}
	// 82E80D20: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82E80D24: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80D28: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82E80D2C: 390AE03C  addi r8, r10, -0x1fc4
	ctx.r[8].s64 = ctx.r[10].s64 + -8132;
	// 82E80D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80D34: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80D38: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80D3C: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80D40: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80D44: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E80D48: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E80D4C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E80D50: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82E80D54: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E80D58: 48000008  b 0x82e80d60
	pc = 0x82E80D60; continue 'dispatch;
	// 82E80D5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E80D60: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E80D64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80D68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80D6C: 4BFFEA65  bl 0x82e7f7d0
	ctx.lr = 0x82E80D70;
	sub_82E7F7D0(ctx, base);
	// 82E80D70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80D74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80D78: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80D7C: 4B43F285  bl 0x822c0000
	ctx.lr = 0x82E80D80;
	sub_822C0000(ctx, base);
	// 82E80D80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E80D84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E80D88: 48006419  bl 0x82e871a0
	ctx.lr = 0x82E80D8C;
	sub_82E871A0(ctx, base);
	// 82E80D8C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E80D90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80D94: 419A0008  beq cr6, 0x82e80d9c
	if ctx.cr[6].eq {
	pc = 0x82E80D9C; continue 'dispatch;
	}
	// 82E80D98: 4B43FAF9  bl 0x822c0890
	ctx.lr = 0x82E80D9C;
	sub_822C0890(ctx, base);
	// 82E80D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E80DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E80DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80DB8 size=196
    let mut pc: u32 = 0x82E80DB8;
    'dispatch: loop {
        match pc {
            0x82E80DB8 => {
    //   block [0x82E80DB8..0x82E80E7C)
	// 82E80DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80DC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E80DC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80DCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E80DD0: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 82E80DD4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E80DD8: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82E80DDC: 4B43FB5D  bl 0x822c0938
	ctx.lr = 0x82E80DE0;
	sub_822C0938(ctx, base);
	// 82E80DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80DE4: 419A0040  beq cr6, 0x82e80e24
	if ctx.cr[6].eq {
	pc = 0x82E80E24; continue 'dispatch;
	}
	// 82E80DE8: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82E80DEC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80DF0: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82E80DF4: 390AE044  addi r8, r10, -0x1fbc
	ctx.r[8].s64 = ctx.r[10].s64 + -8124;
	// 82E80DF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80DFC: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80E00: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80E04: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80E08: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80E0C: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E80E10: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E80E14: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E80E18: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82E80E1C: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E80E20: 48000008  b 0x82e80e28
	pc = 0x82E80E28; continue 'dispatch;
	// 82E80E24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E80E28: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E80E2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80E30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80E34: 4BFFE99D  bl 0x82e7f7d0
	ctx.lr = 0x82E80E38;
	sub_82E7F7D0(ctx, base);
	// 82E80E38: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80E3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80E40: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80E44: 4B43F1BD  bl 0x822c0000
	ctx.lr = 0x82E80E48;
	sub_822C0000(ctx, base);
	// 82E80E48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E80E4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E80E50: 48006351  bl 0x82e871a0
	ctx.lr = 0x82E80E54;
	sub_82E871A0(ctx, base);
	// 82E80E54: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E80E58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80E5C: 419A0008  beq cr6, 0x82e80e64
	if ctx.cr[6].eq {
	pc = 0x82E80E64; continue 'dispatch;
	}
	// 82E80E60: 4B43FA31  bl 0x822c0890
	ctx.lr = 0x82E80E64;
	sub_822C0890(ctx, base);
	// 82E80E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E80E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80E70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E80E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80E80 size=196
    let mut pc: u32 = 0x82E80E80;
    'dispatch: loop {
        match pc {
            0x82E80E80 => {
    //   block [0x82E80E80..0x82E80F44)
	// 82E80E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E80E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E80E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E80E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80E94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E80E98: F8810090  std r4, 0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 82E80E9C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E80EA0: F8A10098  std r5, 0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[5].u64 ) };
	// 82E80EA4: 4B43FA95  bl 0x822c0938
	ctx.lr = 0x82E80EA8;
	sub_822C0938(ctx, base);
	// 82E80EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80EAC: 419A0040  beq cr6, 0x82e80eec
	if ctx.cr[6].eq {
	pc = 0x82E80EEC; continue 'dispatch;
	}
	// 82E80EB0: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82E80EB4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E80EB8: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82E80EBC: 390AE04C  addi r8, r10, -0x1fb4
	ctx.r[8].s64 = ctx.r[10].s64 + -8116;
	// 82E80EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80EC4: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E80EC8: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E80ECC: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80ED0: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80ED4: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E80ED8: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82E80EDC: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82E80EE0: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82E80EE4: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82E80EE8: 48000008  b 0x82e80ef0
	pc = 0x82E80EF0; continue 'dispatch;
	// 82E80EEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E80EF0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E80EF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80EF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80EFC: 4BFFE8D5  bl 0x82e7f7d0
	ctx.lr = 0x82E80F00;
	sub_82E7F7D0(ctx, base);
	// 82E80F00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80F04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80F08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80F0C: 4B43F0F5  bl 0x822c0000
	ctx.lr = 0x82E80F10;
	sub_822C0000(ctx, base);
	// 82E80F10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E80F14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E80F18: 48006289  bl 0x82e871a0
	ctx.lr = 0x82E80F1C;
	sub_82E871A0(ctx, base);
	// 82E80F1C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E80F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80F24: 419A0008  beq cr6, 0x82e80f2c
	if ctx.cr[6].eq {
	pc = 0x82E80F2C; continue 'dispatch;
	}
	// 82E80F28: 4B43F969  bl 0x822c0890
	ctx.lr = 0x82E80F2C;
	sub_822C0890(ctx, base);
	// 82E80F2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E80F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E80F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E80F38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E80F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E80F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E80F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E80F48 size=216
    let mut pc: u32 = 0x82E80F48;
    'dispatch: loop {
        match pc {
            0x82E80F48 => {
    //   block [0x82E80F48..0x82E81020)
	// 82E80F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E80F4C: 48327221  bl 0x831a816c
	ctx.lr = 0x82E80F50;
	sub_831A8130(ctx, base);
	// 82E80F50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E80F54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E80F58: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E80F5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E80F60: 4B43F9D9  bl 0x822c0938
	ctx.lr = 0x82E80F64;
	sub_822C0938(ctx, base);
	// 82E80F64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E80F68: 419A0060  beq cr6, 0x82e80fc8
	if ctx.cr[6].eq {
	pc = 0x82E80FC8; continue 'dispatch;
	}
	// 82E80F6C: 395E0008  addi r10, r30, 8
	ctx.r[10].s64 = ctx.r[30].s64 + 8;
	// 82E80F70: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E80F74: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82E80F78: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E80F7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E80F80: 811E000C  lwz r8, 0xc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E80F84: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E80F88: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 82E80F8C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82E80F90: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82E80F94: 419A0024  beq cr6, 0x82e80fb8
	if ctx.cr[6].eq {
	pc = 0x82E80FB8; continue 'dispatch;
	}
	// 82E80F98: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E80F9C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E80FA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E80FA4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E80FA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E80FAC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E80FB0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E80FB4: 4082FFE8  bne 0x82e80f9c
	if !ctx.cr[0].eq {
	pc = 0x82E80F9C; continue 'dispatch;
	}
	// 82E80FB8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E80FBC: 4BFFF395  bl 0x82e80350
	ctx.lr = 0x82E80FC0;
	sub_82E80350(ctx, base);
	// 82E80FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E80FC4: 48000008  b 0x82e80fcc
	pc = 0x82E80FCC; continue 'dispatch;
	// 82E80FC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E80FCC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E80FD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80FD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80FD8: 4BFFE7F9  bl 0x82e7f7d0
	ctx.lr = 0x82E80FDC;
	sub_82E7F7D0(ctx, base);
	// 82E80FDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E80FE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E80FE4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E80FE8: 4B43F019  bl 0x822c0000
	ctx.lr = 0x82E80FEC;
	sub_822C0000(ctx, base);
	// 82E80FEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E80FF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E80FF4: 480061AD  bl 0x82e871a0
	ctx.lr = 0x82E80FF8;
	sub_82E871A0(ctx, base);
	// 82E80FF8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E80FFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E81000: 419A0008  beq cr6, 0x82e81008
	if ctx.cr[6].eq {
	pc = 0x82E81008; continue 'dispatch;
	}
	// 82E81004: 4B43F88D  bl 0x822c0890
	ctx.lr = 0x82E81008;
	sub_822C0890(ctx, base);
	// 82E81008: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E8100C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E81010: 419A0008  beq cr6, 0x82e81018
	if ctx.cr[6].eq {
	pc = 0x82E81018; continue 'dispatch;
	}
	// 82E81014: 4B43F87D  bl 0x822c0890
	ctx.lr = 0x82E81018;
	sub_822C0890(ctx, base);
	// 82E81018: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E8101C: 483271A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E81020 size=388
    let mut pc: u32 = 0x82E81020;
    'dispatch: loop {
        match pc {
            0x82E81020 => {
    //   block [0x82E81020..0x82E811A4)
	// 82E81020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81024: 48327149  bl 0x831a816c
	ctx.lr = 0x82E81028;
	sub_831A8130(ctx, base);
	// 82E81028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8102C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81030: 48000E59  bl 0x82e81e88
	ctx.lr = 0x82E81034;
	sub_82E81E88(ctx, base);
	// 82E81034: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E81038: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E8103C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E81040: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82E81044: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82E81048: 38CBE01C  addi r6, r11, -0x1fe4
	ctx.r[6].s64 = ctx.r[11].s64 + -8164;
	// 82E8104C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E81050: 38AAE010  addi r5, r10, -0x1ff0
	ctx.r[5].s64 = ctx.r[10].s64 + -8176;
	// 82E81054: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82E81058: 3968E068  addi r11, r8, -0x1f98
	ctx.r[11].s64 = ctx.r[8].s64 + -8088;
	// 82E8105C: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E81060: 3889E070  addi r4, r9, -0x1f90
	ctx.r[4].s64 = ctx.r[9].s64 + -8080;
	// 82E81064: 9BDF0019  stb r30, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[30].u8 ) };
	// 82E81068: 3947E05C  addi r10, r7, -0x1fa4
	ctx.r[10].s64 = ctx.r[7].s64 + -8100;
	// 82E8106C: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82E81070: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E81074: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E81078: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E8107C: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82E81080: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E81084: 4BFFE4ED  bl 0x82e7f570
	ctx.lr = 0x82E81088;
	sub_82E7F570(ctx, base);
	// 82E81088: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82E8108C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82E81090: 38696880  addi r3, r9, 0x6880
	ctx.r[3].s64 = ctx.r[9].s64 + 26752;
	// 82E81094: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E81098: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E8109C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E810A0: 397F00A0  addi r11, r31, 0xa0
	ctx.r[11].s64 = ctx.r[31].s64 + 160;
	// 82E810A4: C00708A8  lfs f0, 0x8a8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E810A8: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82E810AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E810B0: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82E810B4: 3CA08200  lis r5, -0x7e00
	ctx.r[5].s64 = -2113929216;
	// 82E810B8: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82E810BC: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82E810C0: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82E810C4: 388508B0  addi r4, r5, 0x8b0
	ctx.r[4].s64 = ctx.r[5].s64 + 2224;
	// 82E810C8: 13E81C07  vcmpneb. (lvlx128) v31, v8, v3
	tmp.u32 = ctx.r[8].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E810CC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82E810D0: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E810D4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E810D8: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E810DC: 13801C07  vcmpneb. (lvlx128) v28, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E811A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E811A8 size=124
    let mut pc: u32 = 0x82E811A8;
    'dispatch: loop {
        match pc {
            0x82E811A8 => {
    //   block [0x82E811A8..0x82E81224)
	// 82E811A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E811AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E811B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E811B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E811B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E811BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E811C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E811C4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E811C8: 4BFFE0D9  bl 0x82e7f2a0
	ctx.lr = 0x82E811CC;
	sub_82E7F2A0(ctx, base);
	// 82E811CC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E811D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E811D4: 409A0034  bne cr6, 0x82e81208
	if !ctx.cr[6].eq {
	pc = 0x82E81208; continue 'dispatch;
	}
	// 82E811D8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E811DC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E811E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E811E4: 9BC10054  stb r30, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u8 ) };
	// 82E811E8: 392B11A8  addi r9, r11, 0x11a8
	ctx.r[9].s64 = ctx.r[11].s64 + 4520;
	// 82E811EC: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E811F0: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E811F4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E811F8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E811FC: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81200: 4BFFF961  bl 0x82e80b60
	ctx.lr = 0x82E81204;
	sub_82E80B60(ctx, base);
	// 82E81204: 48000008  b 0x82e8120c
	pc = 0x82E8120C; continue 'dispatch;
	// 82E81208: 9BDF00E8  stb r30, 0xe8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u8 ) };
	// 82E8120C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E81210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E81214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E81218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E8121C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E81228 size=172
    let mut pc: u32 = 0x82E81228;
    'dispatch: loop {
        match pc {
            0x82E81228 => {
    //   block [0x82E81228..0x82E812D4)
	// 82E81228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E81230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E81234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E81238: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E8123C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E81240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81244: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E81248: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E8124C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E81250: 4BFFE051  bl 0x82e7f2a0
	ctx.lr = 0x82E81254;
	sub_82E7F2A0(ctx, base);
	// 82E81254: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8125C: 409A004C  bne cr6, 0x82e812a8
	if !ctx.cr[6].eq {
	pc = 0x82E812A8; continue 'dispatch;
	}
	// 82E81260: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E81264: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E81268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E8126C: 3D4082E8  lis r10, -0x7d18
	ctx.r[10].s64 = -2098724864;
	// 82E81270: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E81274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E81278: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82E8127C: 390A1228  addi r8, r10, 0x1228
	ctx.r[8].s64 = ctx.r[10].s64 + 4648;
	// 82E81280: 9BC10058  stb r30, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u8 ) };
	// 82E81284: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E81288: 80E10058  lwz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E8128C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E81290: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82E81294: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 82E81298: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E8129C: E8C10070  ld r6, 0x70(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E812A0: 4BFFF989  bl 0x82e80c28
	ctx.lr = 0x82E812A4;
	sub_82E80C28(ctx, base);
	// 82E812A4: 48000014  b 0x82e812b8
	pc = 0x82E812B8; continue 'dispatch;
	// 82E812A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E812AC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E812B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E812B4: 4BFFEA5D  bl 0x82e7fd10
	ctx.lr = 0x82E812B8;
	sub_82E7FD10(ctx, base);
	// 82E812B8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E812BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E812C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E812C4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E812C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E812CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E812D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E812D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E812D8 size=128
    let mut pc: u32 = 0x82E812D8;
    'dispatch: loop {
        match pc {
            0x82E812D8 => {
    //   block [0x82E812D8..0x82E81358)
	// 82E812D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E812DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E812E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E812E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E812E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E812EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E812F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E812F4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E812F8: 4BFFDFA9  bl 0x82e7f2a0
	ctx.lr = 0x82E812FC;
	sub_82E7F2A0(ctx, base);
	// 82E812FC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81300: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81304: 409A0030  bne cr6, 0x82e81334
	if !ctx.cr[6].eq {
	pc = 0x82E81334; continue 'dispatch;
	}
	// 82E81308: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E8130C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E81310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E81314: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E81318: 392B12D8  addi r9, r11, 0x12d8
	ctx.r[9].s64 = ctx.r[11].s64 + 4824;
	// 82E8131C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81320: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E81324: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E81328: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E8132C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81330: 4BFFFA89  bl 0x82e80db8
	ctx.lr = 0x82E81334;
	sub_82E80DB8(ctx, base);
	// 82E81334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E81338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8133C: 4BFFEC9D  bl 0x82e7ffd8
	ctx.lr = 0x82E81340;
	sub_82E7FFD8(ctx, base);
	// 82E81340: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E81344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E81348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E8134C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E81350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E81358 size=128
    let mut pc: u32 = 0x82E81358;
    'dispatch: loop {
        match pc {
            0x82E81358 => {
    //   block [0x82E81358..0x82E813D8)
	// 82E81358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8135C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E81360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E81364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E81368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8136C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81370: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E81374: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E81378: 4BFFDF29  bl 0x82e7f2a0
	ctx.lr = 0x82E8137C;
	sub_82E7F2A0(ctx, base);
	// 82E8137C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81384: 409A0030  bne cr6, 0x82e813b4
	if !ctx.cr[6].eq {
	pc = 0x82E813B4; continue 'dispatch;
	}
	// 82E81388: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E8138C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E81390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E81394: 9BC10054  stb r30, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u8 ) };
	// 82E81398: 392B1358  addi r9, r11, 0x1358
	ctx.r[9].s64 = ctx.r[11].s64 + 4952;
	// 82E8139C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E813A0: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E813A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E813A8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E813AC: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E813B0: 4BFFF7B1  bl 0x82e80b60
	ctx.lr = 0x82E813B4;
	sub_82E80B60(ctx, base);
	// 82E813B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E813B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E813BC: 4BFFEC9D  bl 0x82e80058
	ctx.lr = 0x82E813C0;
	sub_82E80058(ctx, base);
	// 82E813C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E813C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E813C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E813CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E813D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E813D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E813D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E813D8 size=128
    let mut pc: u32 = 0x82E813D8;
    'dispatch: loop {
        match pc {
            0x82E813D8 => {
    //   block [0x82E813D8..0x82E81458)
	// 82E813D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E813DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E813E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E813E4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E813E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E813EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E813F0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E813F4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E813F8: 4BFFDEA9  bl 0x82e7f2a0
	ctx.lr = 0x82E813FC;
	sub_82E7F2A0(ctx, base);
	// 82E813FC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81404: 409A0038  bne cr6, 0x82e8143c
	if !ctx.cr[6].eq {
	pc = 0x82E8143C; continue 'dispatch;
	}
	// 82E81408: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E8140C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E81410: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E81414: 3D4082E8  lis r10, -0x7d18
	ctx.r[10].s64 = -2098724864;
	// 82E81418: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E8141C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E81420: 390A13D8  addi r8, r10, 0x13d8
	ctx.r[8].s64 = ctx.r[10].s64 + 5080;
	// 82E81424: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81428: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E8142C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E81430: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82E81434: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81438: 4BFFF8B9  bl 0x82e80cf0
	ctx.lr = 0x82E8143C;
	sub_82E80CF0(ctx, base);
	// 82E8143C: D3FF00E4  stfs f31, 0xe4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 82E81440: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E81444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E81448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E8144C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E81450: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E81458 size=148
    let mut pc: u32 = 0x82E81458;
    'dispatch: loop {
        match pc {
            0x82E81458 => {
    //   block [0x82E81458..0x82E814EC)
	// 82E81458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E81460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E81464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E81468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8146C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E81474: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E81478: 4BFFDE29  bl 0x82e7f2a0
	ctx.lr = 0x82E8147C;
	sub_82E7F2A0(ctx, base);
	// 82E8147C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81484: 409A0034  bne cr6, 0x82e814b8
	if !ctx.cr[6].eq {
	pc = 0x82E814B8; continue 'dispatch;
	}
	// 82E81488: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E8148C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E81490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E81494: 9BC10054  stb r30, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u8 ) };
	// 82E81498: 392B1458  addi r9, r11, 0x1458
	ctx.r[9].s64 = ctx.r[11].s64 + 5208;
	// 82E8149C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E814A0: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E814A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E814A8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E814AC: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E814B0: 4BFFF6B1  bl 0x82e80b60
	ctx.lr = 0x82E814B4;
	sub_82E80B60(ctx, base);
	// 82E814B4: 48000020  b 0x82e814d4
	pc = 0x82E814D4; continue 'dispatch;
	// 82E814B8: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E814BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E814C0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E814C4: 616A0400  ori r10, r11, 0x400
	ctx.r[10].u64 = ctx.r[11].u64 | 1024;
	// 82E814C8: 409A0008  bne cr6, 0x82e814d0
	if !ctx.cr[6].eq {
	pc = 0x82E814D0; continue 'dispatch;
	}
	// 82E814CC: 556A05A8  rlwinm r10, r11, 0, 0x16, 0x14
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E814D0: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E814D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E814D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E814DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E814E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E814E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E814E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E814F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E814F0 size=140
    let mut pc: u32 = 0x82E814F0;
    'dispatch: loop {
        match pc {
            0x82E814F0 => {
    //   block [0x82E814F0..0x82E8157C)
	// 82E814F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E814F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E814F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E814FC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E81500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E81504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81508: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E8150C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E81510: 4BFFDD91  bl 0x82e7f2a0
	ctx.lr = 0x82E81514;
	sub_82E7F2A0(ctx, base);
	// 82E81514: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81518: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8151C: 409A003C  bne cr6, 0x82e81558
	if !ctx.cr[6].eq {
	pc = 0x82E81558; continue 'dispatch;
	}
	// 82E81520: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E81524: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E81528: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E8152C: 3D4082E8  lis r10, -0x7d18
	ctx.r[10].s64 = -2098724864;
	// 82E81530: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E81534: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E81538: 390A14F0  addi r8, r10, 0x14f0
	ctx.r[8].s64 = ctx.r[10].s64 + 5360;
	// 82E8153C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81540: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E81544: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E81548: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82E8154C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81550: 4BFFF7A1  bl 0x82e80cf0
	ctx.lr = 0x82E81554;
	sub_82E80CF0(ctx, base);
	// 82E81554: 48000010  b 0x82e81564
	pc = 0x82E81564; continue 'dispatch;
	// 82E81558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8155C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E81560: 4BFFEB79  bl 0x82e800d8
	ctx.lr = 0x82E81564;
	sub_82E800D8(ctx, base);
	// 82E81564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E81568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E8156C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E81570: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E81574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E81580 size=140
    let mut pc: u32 = 0x82E81580;
    'dispatch: loop {
        match pc {
            0x82E81580 => {
    //   block [0x82E81580..0x82E8160C)
	// 82E81580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E81588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E8158C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E81590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E81594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81598: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E8159C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E815A0: 4BFFDD01  bl 0x82e7f2a0
	ctx.lr = 0x82E815A4;
	sub_82E7F2A0(ctx, base);
	// 82E815A4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E815A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E815AC: 409A003C  bne cr6, 0x82e815e8
	if !ctx.cr[6].eq {
	pc = 0x82E815E8; continue 'dispatch;
	}
	// 82E815B0: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E815B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E815B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E815BC: 3D4082E8  lis r10, -0x7d18
	ctx.r[10].s64 = -2098724864;
	// 82E815C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E815C4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E815C8: 390A1580  addi r8, r10, 0x1580
	ctx.r[8].s64 = ctx.r[10].s64 + 5504;
	// 82E815CC: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E815D0: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E815D4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E815D8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82E815DC: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E815E0: 4BFFF711  bl 0x82e80cf0
	ctx.lr = 0x82E815E4;
	sub_82E80CF0(ctx, base);
	// 82E815E4: 48000010  b 0x82e815f4
	pc = 0x82E815F4; continue 'dispatch;
	// 82E815E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E815EC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E815F0: 4BFFE7E1  bl 0x82e7fdd0
	ctx.lr = 0x82E815F4;
	sub_82E7FDD0(ctx, base);
	// 82E815F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E815F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E815FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E81600: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E81604: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E81610 size=124
    let mut pc: u32 = 0x82E81610;
    'dispatch: loop {
        match pc {
            0x82E81610 => {
    //   block [0x82E81610..0x82E8168C)
	// 82E81610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E81618: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E8161C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E81620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E81624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81628: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E8162C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E81630: 4BFFDC71  bl 0x82e7f2a0
	ctx.lr = 0x82E81634;
	sub_82E7F2A0(ctx, base);
	// 82E81634: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8163C: 409A0034  bne cr6, 0x82e81670
	if !ctx.cr[6].eq {
	pc = 0x82E81670; continue 'dispatch;
	}
	// 82E81640: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E81644: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E81648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E8164C: 9BC10054  stb r30, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u8 ) };
	// 82E81650: 392B1610  addi r9, r11, 0x1610
	ctx.r[9].s64 = ctx.r[11].s64 + 5648;
	// 82E81654: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81658: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 82E8165C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E81660: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E81664: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81668: 4BFFF4F9  bl 0x82e80b60
	ctx.lr = 0x82E8166C;
	sub_82E80B60(ctx, base);
	// 82E8166C: 48000008  b 0x82e81674
	pc = 0x82E81674; continue 'dispatch;
	// 82E81670: 9BDF0035  stb r30, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[30].u8 ) };
	// 82E81674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E81678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E8167C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E81680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E81684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E81690 size=176
    let mut pc: u32 = 0x82E81690;
    'dispatch: loop {
        match pc {
            0x82E81690 => {
    //   block [0x82E81690..0x82E81740)
	// 82E81690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81694: 48326AD9  bl 0x831a816c
	ctx.lr = 0x82E81698;
	sub_831A8130(ctx, base);
	// 82E81698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8169C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E816A0: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 82E816A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E816A8: 4BFFDBF9  bl 0x82e7f2a0
	ctx.lr = 0x82E816AC;
	sub_82E7F2A0(ctx, base);
	// 82E816AC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E816B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E816B4: 409A0034  bne cr6, 0x82e816e8
	if !ctx.cr[6].eq {
	pc = 0x82E816E8; continue 'dispatch;
	}
	// 82E816B8: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E816BC: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82E816C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E816C4: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E816C8: 392BF470  addi r9, r11, -0xb90
	ctx.r[9].s64 = ctx.r[11].s64 + -2960;
	// 82E816CC: 387D0104  addi r3, r29, 0x104
	ctx.r[3].s64 = ctx.r[29].s64 + 260;
	// 82E816D0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E816D4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E816D8: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E816DC: 4BFFF7A5  bl 0x82e80e80
	ctx.lr = 0x82E816E0;
	sub_82E80E80(ctx, base);
	// 82E816E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E816E4: 48326AD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82E816E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E816EC: 4BFFDBB5  bl 0x82e7f2a0
	ctx.lr = 0x82E816F0;
	sub_82E7F2A0(ctx, base);
	// 82E816F0: 3BDD00F8  addi r30, r29, 0xf8
	ctx.r[30].s64 = ctx.r[29].s64 + 248;
	// 82E816F4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E816F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E816FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E81700: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81704: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E81708: 419A0028  beq cr6, 0x82e81730
	if ctx.cr[6].eq {
	pc = 0x82E81730; continue 'dispatch;
	}
	// 82E8170C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E81710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81714: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81718: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E8171C: 4E800421  bctrl
	ctx.lr = 0x82E81720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E81720: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E81724: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81728: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E8172C: 409AFFE0  bne cr6, 0x82e8170c
	if !ctx.cr[6].eq {
	pc = 0x82E8170C; continue 'dispatch;
	}
	// 82E81730: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E81734: 480009E5  bl 0x82e82118
	ctx.lr = 0x82E81738;
	sub_82E82118(ctx, base);
	// 82E81738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E8173C: 48326A80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E81740 size=272
    let mut pc: u32 = 0x82E81740;
    'dispatch: loop {
        match pc {
            0x82E81740 => {
    //   block [0x82E81740..0x82E81850)
	// 82E81740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81744: 48326A19  bl 0x831a815c
	ctx.lr = 0x82E81748;
	sub_831A8130(ctx, base);
	// 82E81748: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8174C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E81750: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82E81754: 3BFB0014  addi r31, r27, 0x14
	ctx.r[31].s64 = ctx.r[27].s64 + 20;
	// 82E81758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8175C: 4BFFDB45  bl 0x82e7f2a0
	ctx.lr = 0x82E81760;
	sub_82E7F2A0(ctx, base);
	// 82E81760: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81764: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81768: 409A0038  bne cr6, 0x82e817a0
	if !ctx.cr[6].eq {
	pc = 0x82E817A0; continue 'dispatch;
	}
	// 82E8176C: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E81770: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82E81774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E81778: 9B210054  stb r25, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u8 ) };
	// 82E8177C: 392B1740  addi r9, r11, 0x1740
	ctx.r[9].s64 = ctx.r[11].s64 + 5952;
	// 82E81780: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81784: 387B0104  addi r3, r27, 0x104
	ctx.r[3].s64 = ctx.r[27].s64 + 260;
	// 82E81788: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E8178C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E81790: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E81794: 4BFFF3CD  bl 0x82e80b60
	ctx.lr = 0x82E81798;
	sub_82E80B60(ctx, base);
	// 82E81798: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E8179C: 48326A10  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82E817A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E817A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E817A8: 997B00EA  stb r11, 0xea(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(234 as u32), ctx.r[11].u8 ) };
	// 82E817AC: 4BFFDAF5  bl 0x82e7f2a0
	ctx.lr = 0x82E817B0;
	sub_82E7F2A0(ctx, base);
	// 82E817B0: 3B5B00F8  addi r26, r27, 0xf8
	ctx.r[26].s64 = ctx.r[27].s64 + 248;
	// 82E817B4: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E817B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E817BC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E817C0: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E817C4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E817C8: 419A0060  beq cr6, 0x82e81828
	if ctx.cr[6].eq {
	pc = 0x82E81828; continue 'dispatch;
	}
	// 82E817CC: 573C063E  clrlwi r28, r25, 0x18
	ctx.r[28].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82E817D0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E817D4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E817D8: 48008CB9  bl 0x82e8a490
	ctx.lr = 0x82E817DC;
	sub_82E8A490(ctx, base);
	// 82E817DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E817E0: 419A0038  beq cr6, 0x82e81818
	if ctx.cr[6].eq {
	pc = 0x82E81818; continue 'dispatch;
	}
	// 82E817E4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E817E8: 3BCB0108  addi r30, r11, 0x108
	ctx.r[30].s64 = ctx.r[11].s64 + 264;
	// 82E817EC: 816B010C  lwz r11, 0x10c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) } as u64;
	// 82E817F0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E817F4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E817F8: 419A0020  beq cr6, 0x82e81818
	if ctx.cr[6].eq {
	pc = 0x82E81818; continue 'dispatch;
	}
	// 82E817FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E81800: 386B0158  addi r3, r11, 0x158
	ctx.r[3].s64 = ctx.r[11].s64 + 344;
	// 82E81804: 4BFFC57D  bl 0x82e7dd80
	ctx.lr = 0x82E81808;
	sub_82E7DD80(ctx, base);
	// 82E81808: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8180C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81810: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E81814: 409AFFE8  bne cr6, 0x82e817fc
	if !ctx.cr[6].eq {
	pc = 0x82E817FC; continue 'dispatch;
	}
	// 82E81818: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8181C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E81820: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E81824: 409AFFAC  bne cr6, 0x82e817d0
	if !ctx.cr[6].eq {
	pc = 0x82E817D0; continue 'dispatch;
	}
	// 82E81828: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82E8182C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E81830: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81834: 419A0010  beq cr6, 0x82e81844
	if ctx.cr[6].eq {
	pc = 0x82E81844; continue 'dispatch;
	}
	// 82E81838: 480007A1  bl 0x82e81fd8
	ctx.lr = 0x82E8183C;
	sub_82E81FD8(ctx, base);
	// 82E8183C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E81840: 4832696C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82E81844: 480007A5  bl 0x82e81fe8
	ctx.lr = 0x82E81848;
	sub_82E81FE8(ctx, base);
	// 82E81848: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E8184C: 48326960  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81850 size=12
    let mut pc: u32 = 0x82E81850;
    'dispatch: loop {
        match pc {
            0x82E81850 => {
    //   block [0x82E81850..0x82E8185C)
	// 82E81850: 988300E9  stb r4, 0xe9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(233 as u32), ctx.r[4].u8 ) };
	// 82E81854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E81858: 4BFFFEE8  b 0x82e81740
	sub_82E81740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E81860 size=376
    let mut pc: u32 = 0x82E81860;
    'dispatch: loop {
        match pc {
            0x82E81860 => {
    //   block [0x82E81860..0x82E819D8)
	// 82E81860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81864: 48326905  bl 0x831a8168
	ctx.lr = 0x82E81868;
	sub_831A8130(ctx, base);
	// 82E81868: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8186C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E81870: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E81874: 387CFFF8  addi r3, r28, -8
	ctx.r[3].s64 = ctx.r[28].s64 + -8;
	// 82E81878: 4BFFDA29  bl 0x82e7f2a0
	ctx.lr = 0x82E8187C;
	sub_82E7F2A0(ctx, base);
	// 82E8187C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81884: 409A00F0  bne cr6, 0x82e81974
	if !ctx.cr[6].eq {
	pc = 0x82E81974; continue 'dispatch;
	}
	// 82E81888: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82E8188C: 4BF70B35  bl 0x82df23c0
	ctx.lr = 0x82E81890;
	sub_82DF23C0(ctx, base);
	// 82E81890: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E81894: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E81898: 419A0038  beq cr6, 0x82e818d0
	if ctx.cr[6].eq {
	pc = 0x82E818D0; continue 'dispatch;
	}
	// 82E8189C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82E818A0: 13E0FC07  vcmpneb. (lvlx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E818A4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82E818A8: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E818AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E818B0: 13C8FC07  vcmpneb. (lvlx128) v30, v8, v31
	tmp.u32 = ctx.r[8].u32 + ctx.r[31].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E818B4: 13A9FC07  vcmpneb. (lvlx128) v29, v9, v31
	tmp.u32 = ctx.r[9].u32 + ctx.r[31].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E818B8: 138AFC07  vcmpneb. (lvlx128) v28, v10, v31
	tmp.u32 = ctx.r[10].u32 + ctx.r[31].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E819D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E819D8 size=208
    let mut pc: u32 = 0x82E819D8;
    'dispatch: loop {
        match pc {
            0x82E819D8 => {
    //   block [0x82E819D8..0x82E81AA8)
	// 82E819D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E819DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E819E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E819E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E819E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E819EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E819F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E819F4: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82E819F8: 4BFFD8A9  bl 0x82e7f2a0
	ctx.lr = 0x82E819FC;
	sub_82E7F2A0(ctx, base);
	// 82E819FC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81A00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81A04: 409A0074  bne cr6, 0x82e81a78
	if !ctx.cr[6].eq {
	pc = 0x82E81A78; continue 'dispatch;
	}
	// 82E81A08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E81A0C: 3D4082E8  lis r10, -0x7d18
	ctx.r[10].s64 = -2098724864;
	// 82E81A10: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E81A18: 38EA19D8  addi r7, r10, 0x19d8
	ctx.r[7].s64 = ctx.r[10].s64 + 6616;
	// 82E81A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82E81A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81A24: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82E81A28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E81A2C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E81A30: 419A0024  beq cr6, 0x82e81a54
	if ctx.cr[6].eq {
	pc = 0x82E81A54; continue 'dispatch;
	}
	// 82E81A34: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E81A38: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E81A3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E81A40: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E81A44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E81A48: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E81A4C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E81A50: 4082FFE8  bne 0x82e81a38
	if !ctx.cr[0].eq {
	pc = 0x82E81A38; continue 'dispatch;
	}
	// 82E81A54: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E81A58: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E81A5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E81A60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E81A64: 4BFFE805  bl 0x82e80268
	ctx.lr = 0x82E81A68;
	sub_82E80268(ctx, base);
	// 82E81A68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E81A6C: 387E0104  addi r3, r30, 0x104
	ctx.r[3].s64 = ctx.r[30].s64 + 260;
	// 82E81A70: 4BFFF4D9  bl 0x82e80f48
	ctx.lr = 0x82E81A74;
	sub_82E80F48(ctx, base);
	// 82E81A74: 4800001C  b 0x82e81a90
	pc = 0x82E81A90; continue 'dispatch;
	// 82E81A78: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81A7C: 397E00F0  addi r11, r30, 0xf0
	ctx.r[11].s64 = ctx.r[30].s64 + 240;
	// 82E81A80: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82E81A84: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E81A88: 915E00F0  stw r10, 0xf0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(240 as u32), ctx.r[10].u32 ) };
	// 82E81A8C: 4B4429D5  bl 0x822c4460
	ctx.lr = 0x82E81A90;
	sub_822C4460(ctx, base);
	// 82E81A90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E81A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E81A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E81A9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E81AA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E81AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E81AA8 size=376
    let mut pc: u32 = 0x82E81AA8;
    'dispatch: loop {
        match pc {
            0x82E81AA8 => {
    //   block [0x82E81AA8..0x82E81C20)
	// 82E81AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81AAC: 483266BD  bl 0x831a8168
	ctx.lr = 0x82E81AB0;
	sub_831A8130(ctx, base);
	// 82E81AB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E81AB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E81AB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E81ABC: 3BBE0014  addi r29, r30, 0x14
	ctx.r[29].s64 = ctx.r[30].s64 + 20;
	// 82E81AC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E81AC4: 4BFFD7DD  bl 0x82e7f2a0
	ctx.lr = 0x82E81AC8;
	sub_82E7F2A0(ctx, base);
	// 82E81AC8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81ACC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81AD0: 409A00CC  bne cr6, 0x82e81b9c
	if !ctx.cr[6].eq {
	pc = 0x82E81B9C; continue 'dispatch;
	}
	// 82E81AD4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E81AD8: 4BF708E9  bl 0x82df23c0
	ctx.lr = 0x82E81ADC;
	sub_82DF23C0(ctx, base);
	// 82E81ADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E81AE0: 419A0014  beq cr6, 0x82e81af4
	if ctx.cr[6].eq {
	pc = 0x82E81AF4; continue 'dispatch;
	}
	// 82E81AE4: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E81AE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E81C20 size=616
    let mut pc: u32 = 0x82E81C20;
    'dispatch: loop {
        match pc {
            0x82E81C20 => {
    //   block [0x82E81C20..0x82E81E88)
	// 82E81C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E81C24: 48326545  bl 0x831a8168
	ctx.lr = 0x82E81C28;
	sub_831A8130(ctx, base);
	// 82E81C28: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82E81C2C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E81C30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E81C34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E81C38: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E81C3C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E81C40: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82E81C44: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E81C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E81C4C: 4BFFD655  bl 0x82e7f2a0
	ctx.lr = 0x82E81C50;
	sub_82E7F2A0(ctx, base);
	// 82E81C50: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81C54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81C58: 419A0220  beq cr6, 0x82e81e78
	if ctx.cr[6].eq {
	pc = 0x82E81E78; continue 'dispatch;
	}
	// 82E81C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81C60: 48000349  bl 0x82e81fa8
	ctx.lr = 0x82E81C64;
	sub_82E81FA8(ctx, base);
	// 82E81C64: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81C6C: 409A020C  bne cr6, 0x82e81e78
	if !ctx.cr[6].eq {
	pc = 0x82E81E78; continue 'dispatch;
	}
	// 82E81C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81C74: 4800034D  bl 0x82e81fc0
	ctx.lr = 0x82E81C78;
	sub_82E81FC0(ctx, base);
	// 82E81C78: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81C7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81C80: 409A01F8  bne cr6, 0x82e81e78
	if !ctx.cr[6].eq {
	pc = 0x82E81E78; continue 'dispatch;
	}
	// 82E81C84: C01F00E4  lfs f0, 0xe4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81C88: 389F0124  addi r4, r31, 0x124
	ctx.r[4].s64 = ctx.r[31].s64 + 292;
	// 82E81C8C: EFC007F2  fmuls f30, f0, f31
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E81C90: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E81C94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E81C98: ED8DF028  fsubs f12, f13, f30
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[30].f64) as f32) as f64);
	// 82E81C9C: D19F0004  stfs f12, 4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81CA0: 4BB46AB9  bl 0x829c8758
	ctx.lr = 0x82E81CA4;
	sub_829C8758(ctx, base);
	// 82E81CA4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E81CA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81CAC: 419A0008  beq cr6, 0x82e81cb4
	if ctx.cr[6].eq {
	pc = 0x82E81CB4; continue 'dispatch;
	}
	// 82E81CB0: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82E81CB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82E81CB8: C1BF0004  lfs f13, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E81CBC: C00B17A0  lfs f0, 0x17a0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81CC0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E81CC4: 41990054  bgt cr6, 0x82e81d18
	if ctx.cr[6].gt {
	pc = 0x82E81D18; continue 'dispatch;
	}
	// 82E81CC8: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E81CCC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E81CD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E81CD4: 419A00A4  beq cr6, 0x82e81d78
	if ctx.cr[6].eq {
	pc = 0x82E81D78; continue 'dispatch;
	}
	// 82E81CD8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82E81CDC: 419A009C  beq cr6, 0x82e81d78
	if ctx.cr[6].eq {
	pc = 0x82E81D78; continue 'dispatch;
	}
	// 82E81CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81CE4: 4800028D  bl 0x82e81f70
	ctx.lr = 0x82E81CE8;
	sub_82E81F70(ctx, base);
	// 82E81CE8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E81CEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81CF0: 419A0088  beq cr6, 0x82e81d78
	if ctx.cr[6].eq {
	pc = 0x82E81D78; continue 'dispatch;
	}
	// 82E81CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81CF8: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E81CFC: 480001FD  bl 0x82e81ef8
	ctx.lr = 0x82E81D00;
	sub_82E81EF8(ctx, base);
	// 82E81D00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81D04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E81D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81D0C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E81D10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E81D14: 4E800421  bctrl
	ctx.lr = 0x82E81D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E81D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E81D1C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E81D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81D24: 419A005C  beq cr6, 0x82e81d80
	if ctx.cr[6].eq {
	pc = 0x82E81D80; continue 'dispatch;
	}
	// 82E81D28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E81D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81D30: 4BFFFA11  bl 0x82e81740
	ctx.lr = 0x82E81D34;
	sub_82E81740(ctx, base);
	// 82E81D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E81D38: 4BFFE101  bl 0x82e7fe38
	ctx.lr = 0x82E81D3C;
	sub_82E7FE38(ctx, base);
	// 82E81D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E81D40: 4BFFD561  bl 0x82e7f2a0
	ctx.lr = 0x82E81D44;
	sub_82E7F2A0(ctx, base);
	// 82E81D44: 83DF00FC  lwz r30, 0xfc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E81D48: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81D4C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E81D50: 419A0118  beq cr6, 0x82e81e68
	if ctx.cr[6].eq {
	pc = 0x82E81E68; continue 'dispatch;
	}
	// 82E81D54: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E81D58: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E81D5C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E81D60: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E81D64: 48007E75  bl 0x82e89bd8
	ctx.lr = 0x82E81D68;
	sub_82E89BD8(ctx, base);
	// 82E81D68: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81D6C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E81D70: 409AFFE4  bne cr6, 0x82e81d54
	if !ctx.cr[6].eq {
	pc = 0x82E81D54; continue 'dispatch;
	}
	// 82E81D74: 480000F4  b 0x82e81e68
	pc = 0x82E81E68; continue 'dispatch;
	// 82E81D78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E81D7C: 4BFFFFA0  b 0x82e81d1c
	pc = 0x82E81D1C; continue 'dispatch;
	// 82E81D80: 897F00EA  lbz r11, 0xea(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(234 as u32) ) } as u64;
	// 82E81D84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81D88: 419A0040  beq cr6, 0x82e81dc8
	if ctx.cr[6].eq {
	pc = 0x82E81DC8; continue 'dispatch;
	}
	// 82E81D8C: C01F002C  lfs f0, 0x2c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81D90: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E81D94: FD800210  fabs f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E81D98: C1ABDFB0  lfs f13, -0x2050(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E81D9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E81DA0: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82E81DA4: 41980008  blt cr6, 0x82e81dac
	if ctx.cr[6].lt {
	pc = 0x82E81DAC; continue 'dispatch;
	}
	// 82E81DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E81DAC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E81DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E81DB4: 409A0014  bne cr6, 0x82e81dc8
	if !ctx.cr[6].eq {
	pc = 0x82E81DC8; continue 'dispatch;
	}
	// 82E81DB8: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E81DBC: 997F00EA  stb r11, 0xea(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(234 as u32), ctx.r[11].u8 ) };
	// 82E81DC0: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E81DC4: D19F0004  stfs f12, 4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81DC8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E81DCC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81DD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E81DD4: 409A0034  bne cr6, 0x82e81e08
	if !ctx.cr[6].eq {
	pc = 0x82E81E08; continue 'dispatch;
	}
	// 82E81DD8: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81DDC: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E81DE0: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81DE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E81DE8: 419A0014  beq cr6, 0x82e81dfc
	if ctx.cr[6].eq {
	pc = 0x82E81DFC; continue 'dispatch;
	}
	// 82E81DEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82E81DF0: C00BFB3C  lfs f0, -0x4c4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81DF4: EFE06828  fsubs f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E81DF8: 48000018  b 0x82e81e10
	pc = 0x82E81E10; continue 'dispatch;
	// 82E81DFC: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81E00: EFE06828  fsubs f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E81E04: 4800000C  b 0x82e81e10
	pc = 0x82E81E10; continue 'dispatch;
	// 82E81E08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E81E0C: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E81E10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E81E14: 4BFFD48D  bl 0x82e7f2a0
	ctx.lr = 0x82E81E18;
	sub_82E7F2A0(ctx, base);
	// 82E81E18: 83DF00FC  lwz r30, 0xfc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 82E81E1C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81E20: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E81E24: 419A0044  beq cr6, 0x82e81e68
	if ctx.cr[6].eq {
	pc = 0x82E81E68; continue 'dispatch;
	}
	// 82E81E28: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E81E2C: 4800701D  bl 0x82e88e48
	ctx.lr = 0x82E81E30;
	sub_82E88E48(ctx, base);
	// 82E81E30: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E81E34: 41990028  bgt cr6, 0x82e81e5c
	if ctx.cr[6].gt {
	pc = 0x82E81E5C; continue 'dispatch;
	}
	// 82E81E38: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E81E3C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E81E40: 556AB7FE  rlwinm r10, r11, 0x16, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82E81E44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E81E48: 419A0014  beq cr6, 0x82e81e5c
	if ctx.cr[6].eq {
	pc = 0x82E81E5C; continue 'dispatch;
	}
	// 82E81E4C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E81E50: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E81E54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E81E58: 48007D81  bl 0x82e89bd8
	ctx.lr = 0x82E81E5C;
	sub_82E89BD8(ctx, base);
	// 82E81E5C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E81E60: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E81E64: 409AFFC4  bne cr6, 0x82e81e28
	if !ctx.cr[6].eq {
	pc = 0x82E81E28; continue 'dispatch;
	}
	// 82E81E68: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E81E6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E81E70: 419A0008  beq cr6, 0x82e81e78
	if ctx.cr[6].eq {
	pc = 0x82E81E78; continue 'dispatch;
	}
	// 82E81E74: 4B43EA1D  bl 0x822c0890
	ctx.lr = 0x82E81E78;
	sub_822C0890(ctx, base);
	// 82E81E78: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E81E7C: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E81E80: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E81E84: 48326334  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E81E88 size=52
    let mut pc: u32 = 0x82E81E88;
    'dispatch: loop {
        match pc {
            0x82E81E88 => {
    //   block [0x82E81E88..0x82E81EBC)
	// 82E81E88: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E81E8C: 89430010  lbz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81E90: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E81E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E81E98: 38E9E088  addi r7, r9, -0x1f78
	ctx.r[7].s64 = ctx.r[9].s64 + -8056;
	// 82E81E9C: 554606BE  clrlwi r6, r10, 0x1a
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82E81EA0: 9903000C  stb r8, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u8 ) };
	// 82E81EA4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81EA8: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E81EAC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81EB0: 98C30010  stb r6, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[6].u8 ) };
	// 82E81EB4: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E81EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81EC0 size=16
    let mut pc: u32 = 0x82E81EC0;
    'dispatch: loop {
        match pc {
            0x82E81EC0 => {
    //   block [0x82E81EC0..0x82E81ED0)
	// 82E81EC0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E81EC4: 394BE088  addi r10, r11, -0x1f78
	ctx.r[10].s64 = ctx.r[11].s64 + -8056;
	// 82E81EC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E81ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E81ED0 size=32
    let mut pc: u32 = 0x82E81ED0;
    'dispatch: loop {
        match pc {
            0x82E81ED0 => {
    //   block [0x82E81ED0..0x82E81EF0)
	// 82E81ED0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E81ED4: C00B9534  lfs f0, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81ED8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E81EDC: 409A0014  bne cr6, 0x82e81ef0
	if !ctx.cr[6].eq {
		sub_82E81EF0(ctx, base);
		return;
	}
	// 82E81EE0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82E81EE4: C00BFB3C  lfs f0, -0x4c4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81EE8: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E81EF0 size=8
    let mut pc: u32 = 0x82E81EF0;
    'dispatch: loop {
        match pc {
            0x82E81EF0 => {
    //   block [0x82E81EF0..0x82E81EF8)
	// 82E81EF0: D0230004  stfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E81EF8 size=32
    let mut pc: u32 = 0x82E81EF8;
    'dispatch: loop {
        match pc {
            0x82E81EF8 => {
    //   block [0x82E81EF8..0x82E81F18)
	// 82E81EF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E81EFC: C00B9534  lfs f0, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E81F00: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E81F04: 409A0014  bne cr6, 0x82e81f18
	if !ctx.cr[6].eq {
		sub_82E81F18(ctx, base);
		return;
	}
	// 82E81F08: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82E81F0C: C1ABFB3C  lfs f13, -0x4c4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E81F10: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81F14: 48000008  b 0x82e81f1c
	sub_82E81F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E81F18 size=32
    let mut pc: u32 = 0x82E81F18;
    'dispatch: loop {
        match pc {
            0x82E81F18 => {
    //   block [0x82E81F18..0x82E81F38)
	// 82E81F18: D0230004  stfs f1, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E81F1C: D0230008  stfs f1, 8(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E81F20: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82E81F24: 409A0014  bne cr6, 0x82e81f38
	if !ctx.cr[6].eq {
		sub_82E81F38(ctx, base);
		return;
	}
	// 82E81F28: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81F2C: 616A0040  ori r10, r11, 0x40
	ctx.r[10].u64 = ctx.r[11].u64 | 64;
	// 82E81F30: 99430010  stb r10, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82E81F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81F38 size=20
    let mut pc: u32 = 0x82E81F38;
    'dispatch: loop {
        match pc {
            0x82E81F38 => {
    //   block [0x82E81F38..0x82E81F4C)
	// 82E81F38: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81F3C: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E81F40: 554A06B0  rlwinm r10, r10, 0, 0x1a, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81F44: 99430010  stb r10, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82E81F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81F50 size=28
    let mut pc: u32 = 0x82E81F50;
    'dispatch: loop {
        match pc {
            0x82E81F50 => {
    //   block [0x82E81F50..0x82E81F6C)
	// 82E81F50: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E81F54: 3D400400  lis r10, 0x400
	ctx.r[10].s64 = 67108864;
	// 82E81F58: 5569000E  rlwinm r9, r11, 0, 0, 7
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81F5C: 7D095050  subf r8, r9, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E81F60: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82E81F64: 54E3DFFE  rlwinm r3, r7, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82E81F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81F70 size=12
    let mut pc: u32 = 0x82E81F70;
    'dispatch: loop {
        match pc {
            0x82E81F70 => {
    //   block [0x82E81F70..0x82E81F7C)
	// 82E81F70: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81F74: 5563C9FE  srwi r3, r11, 7
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E81F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81F80 size=16
    let mut pc: u32 = 0x82E81F80;
    'dispatch: loop {
        match pc {
            0x82E81F80 => {
    //   block [0x82E81F80..0x82E81F90)
	// 82E81F80: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E81F84: 508B3C70  rlwimi r11, r4, 7, 0x11, 0x18
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(7) as u64) & 0x0000000000007F80) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF807F);
	// 82E81F88: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82E81F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81F90 size=24
    let mut pc: u32 = 0x82E81F90;
    'dispatch: loop {
        match pc {
            0x82E81F90 => {
    //   block [0x82E81F90..0x82E81FA8)
	// 82E81F90: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E81F94: 5569000E  rlwinm r9, r11, 0, 0, 7
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81F98: 3D09FE00  addis r8, r9, -0x200
	ctx.r[8].s64 = ctx.r[9].s64 + -33554432;
	// 82E81F9C: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82E81FA0: 54E3DFFE  rlwinm r3, r7, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82E81FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81FA8 size=24
    let mut pc: u32 = 0x82E81FA8;
    'dispatch: loop {
        match pc {
            0x82E81FA8 => {
    //   block [0x82E81FA8..0x82E81FC0)
	// 82E81FA8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E81FAC: 5569000E  rlwinm r9, r11, 0, 0, 7
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81FB0: 3D09FF00  addis r8, r9, -0x100
	ctx.r[8].s64 = ctx.r[9].s64 + -16777216;
	// 82E81FB4: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82E81FB8: 54E3DFFE  rlwinm r3, r7, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82E81FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81FC0 size=24
    let mut pc: u32 = 0x82E81FC0;
    'dispatch: loop {
        match pc {
            0x82E81FC0 => {
    //   block [0x82E81FC0..0x82E81FD8)
	// 82E81FC0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E81FC4: 5569000E  rlwinm r9, r11, 0, 0, 7
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E81FC8: 39090000  addi r8, r9, 0
	ctx.r[8].s64 = ctx.r[9].s64 + 0;
	// 82E81FCC: 7D070034  cntlzw r7, r8
	ctx.r[7].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82E81FD0: 54E3DFFE  rlwinm r3, r7, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82E81FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81FD8 size=12
    let mut pc: u32 = 0x82E81FD8;
    'dispatch: loop {
        match pc {
            0x82E81FD8 => {
    //   block [0x82E81FD8..0x82E81FE4)
	// 82E81FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E81FDC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E81FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81FE8 size=12
    let mut pc: u32 = 0x82E81FE8;
    'dispatch: loop {
        match pc {
            0x82E81FE8 => {
    //   block [0x82E81FE8..0x82E81FF4)
	// 82E81FE8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82E81FEC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E81FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E81FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E81FF8 size=28
    let mut pc: u32 = 0x82E81FF8;
    'dispatch: loop {
        match pc {
            0x82E81FF8 => {
    //   block [0x82E81FF8..0x82E82014)
	// 82E81FF8: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E81FFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E82000: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E82004: 409A0008  bne cr6, 0x82e8200c
	if !ctx.cr[6].eq {
	pc = 0x82E8200C; continue 'dispatch;
	}
	// 82E82008: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82E8200C: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E82010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E82018 size=12
    let mut pc: u32 = 0x82E82018;
    'dispatch: loop {
        match pc {
            0x82E82018 => {
    //   block [0x82E82018..0x82E82024)
	// 82E82018: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E8201C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82E82020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82028 size=72
    let mut pc: u32 = 0x82E82028;
    'dispatch: loop {
        match pc {
            0x82E82028 => {
    //   block [0x82E82028..0x82E82070)
	// 82E82028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E82030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E82034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E82038: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8203C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E82040: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E82044: 392BE088  addi r9, r11, -0x1f78
	ctx.r[9].s64 = ctx.r[11].s64 + -8056;
	// 82E82048: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E8204C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E82050: 419A000C  beq cr6, 0x82e8205c
	if ctx.cr[6].eq {
	pc = 0x82E8205C; continue 'dispatch;
	}
	// 82E82054: 4BF70385  bl 0x82df23d8
	ctx.lr = 0x82E82058;
	sub_82DF23D8(ctx, base);
	// 82E82058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8205C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E82060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E82064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E82068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E8206C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E82070 size=48
    let mut pc: u32 = 0x82E82070;
    'dispatch: loop {
        match pc {
            0x82E82070 => {
    //   block [0x82E82070..0x82E820A0)
	// 82E82070: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E82074: 556A0672  rlwinm r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82078: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E8207C: 419A0024  beq cr6, 0x82e820a0
	if ctx.cr[6].eq {
		sub_82E820A0(ctx, base);
		return;
	}
	// 82E82080: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82E82084: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E82088: C00BFB3C  lfs f0, -0x4c4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E8208C: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82E82090: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E82094: FD8D036E  fsel f12, f13, f13, f0
	ctx.f[12].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82E82098: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E8209C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E820A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E820A0 size=36
    let mut pc: u32 = 0x82E820A0;
    'dispatch: loop {
        match pc {
            0x82E820A0 => {
    //   block [0x82E820A0..0x82E820C4)
	// 82E820A0: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E820A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E820A8: ED800828  fsubs f12, f0, f1
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82E820AC: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E820B0: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82E820B4: 41980008  blt cr6, 0x82e820bc
	if ctx.cr[6].lt {
	pc = 0x82E820BC; continue 'dispatch;
	}
	// 82E820B8: EDA00828  fsubs f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82E820BC: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E820C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E820C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E820C8 size=44
    let mut pc: u32 = 0x82E820C8;
    'dispatch: loop {
        match pc {
            0x82E820C8 => {
    //   block [0x82E820C8..0x82E820F4)
	// 82E820C8: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E820CC: 3D400400  lis r10, 0x400
	ctx.r[10].s64 = 67108864;
	// 82E820D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E820D4: 419A0028  beq cr6, 0x82e820fc
	if ctx.cr[6].eq {
		sub_82E820FC(ctx, base);
		return;
	}
	// 82E820D8: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E820DC: 5528000E  rlwinm r8, r9, 0, 0, 7
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E820E0: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E820E4: 419A0010  beq cr6, 0x82e820f4
	if ctx.cr[6].eq {
		sub_82E820F4(ctx, base);
		return;
	}
	// 82E820E8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E820EC: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E820F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E820F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E820F4 size=8
    let mut pc: u32 = 0x82E820F4;
    'dispatch: loop {
        match pc {
            0x82E820F4 => {
    //   block [0x82E820F4..0x82E820FC)
	// 82E820F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E820F8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E820FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E820FC size=16
    let mut pc: u32 = 0x82E820FC;
    'dispatch: loop {
        match pc {
            0x82E820FC => {
    //   block [0x82E820FC..0x82E8210C)
	// 82E820FC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E82100: 5569000E  rlwinm r9, r11, 0, 0, 7
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82104: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E82108: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E8210C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E8210C size=12
    let mut pc: u32 = 0x82E8210C;
    'dispatch: loop {
        match pc {
            0x82E8210C => {
    //   block [0x82E8210C..0x82E82118)
	// 82E8210C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E82110: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E82114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E82118 size=36
    let mut pc: u32 = 0x82E82118;
    'dispatch: loop {
        match pc {
            0x82E82118 => {
    //   block [0x82E82118..0x82E8213C)
	// 82E82118: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E8211C: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E82120: C1AB9534  lfs f13, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E82124: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E82128: 409A0014  bne cr6, 0x82e8213c
	if !ctx.cr[6].eq {
		sub_82E8213C(ctx, base);
		return;
	}
	// 82E8212C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82E82130: C18BFB3C  lfs f12, -0x4c4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1220 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E82134: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E82138: 48000008  b 0x82e82140
	sub_82E8213C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E8213C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E8213C size=36
    let mut pc: u32 = 0x82E8213C;
    'dispatch: loop {
        match pc {
            0x82E8213C => {
    //   block [0x82E8213C..0x82E82160)
	// 82E8213C: D0030004  stfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E82140: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82E82144: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E82148: 9943000C  stb r10, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82E8214C: 409A0014  bne cr6, 0x82e82160
	if !ctx.cr[6].eq {
		sub_82E82160(ctx, base);
		return;
	}
	// 82E82150: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E82154: 61690040  ori r9, r11, 0x40
	ctx.r[9].u64 = ctx.r[11].u64 | 64;
	// 82E82158: 99230010  stb r9, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 82E8215C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E82160 size=20
    let mut pc: u32 = 0x82E82160;
    'dispatch: loop {
        match pc {
            0x82E82160 => {
    //   block [0x82E82160..0x82E82174)
	// 82E82160: 89630010  lbz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E82164: 5569063E  clrlwi r9, r11, 0x18
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E82168: 552906B0  rlwinm r9, r9, 0, 0x1a, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8216C: 99230010  stb r9, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 82E82170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82178 size=96
    let mut pc: u32 = 0x82E82178;
    'dispatch: loop {
        match pc {
            0x82E82178 => {
    //   block [0x82E82178..0x82E821D8)
	// 82E82178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E82180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E82184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E82188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8218C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E82190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82194: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82198: 388BBB0C  addi r4, r11, -0x44f4
	ctx.r[4].s64 = ctx.r[11].s64 + -17652;
	// 82E8219C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E821A0: 4BF71869  bl 0x82df3a08
	ctx.lr = 0x82E821A4;
	sub_82DF3A08(ctx, base);
	// 82E821A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E821A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E821AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E821B0: 48008569  bl 0x82e8a718
	ctx.lr = 0x82E821B4;
	sub_82E8A718(ctx, base);
	// 82E821B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E821B8: 4BF71271  bl 0x82df3428
	ctx.lr = 0x82E821BC;
	sub_82DF3428(ctx, base);
	// 82E821BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E821C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E821C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E821C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E821CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E821D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E821D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E821D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E821D8 size=140
    let mut pc: u32 = 0x82E821D8;
    'dispatch: loop {
        match pc {
            0x82E821D8 => {
    //   block [0x82E821D8..0x82E82264)
	// 82E821D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E821DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E821E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E821E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E821E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E821EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E821F0: 3BFE0060  addi r31, r30, 0x60
	ctx.r[31].s64 = ctx.r[30].s64 + 96;
	// 82E821F4: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E821F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E821FC: 419A0024  beq cr6, 0x82e82220
	if ctx.cr[6].eq {
	pc = 0x82E82220; continue 'dispatch;
	}
	// 82E82200: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E82204: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E82208: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8220C: 4B95007D  bl 0x827d2288
	ctx.lr = 0x82E82210;
	sub_827D2288(ctx, base);
	// 82E82210: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E82214: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82218: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E8221C: 4BF6FF6D  bl 0x82df2188
	ctx.lr = 0x82E82220;
	sub_82DF2188(ctx, base);
	// 82E82220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E82224: 387E0050  addi r3, r30, 0x50
	ctx.r[3].s64 = ctx.r[30].s64 + 80;
	// 82E82228: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E8222C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E82230: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E82234: 4BF711F5  bl 0x82df3428
	ctx.lr = 0x82E82238;
	sub_82DF3428(ctx, base);
	// 82E82238: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E8223C: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E82240: 394B9B84  addi r10, r11, -0x647c
	ctx.r[10].s64 = ctx.r[11].s64 + -25724;
	// 82E82244: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E82248: 4BF711E1  bl 0x82df3428
	ctx.lr = 0x82E8224C;
	sub_82DF3428(ctx, base);
	// 82E8224C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E82250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E82254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E82258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E8225C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E82260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82268 size=80
    let mut pc: u32 = 0x82E82268;
    'dispatch: loop {
        match pc {
            0x82E82268 => {
    //   block [0x82E82268..0x82E822B8)
	// 82E82268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E82270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E82274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E82278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8227C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82280: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E82284: 4BFFFF55  bl 0x82e821d8
	ctx.lr = 0x82E82288;
	sub_82E821D8(ctx, base);
	// 82E82288: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E8228C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E82294: 419A000C  beq cr6, 0x82e822a0
	if ctx.cr[6].eq {
	pc = 0x82E822A0; continue 'dispatch;
	}
	// 82E82298: 4BF70141  bl 0x82df23d8
	ctx.lr = 0x82E8229C;
	sub_82DF23D8(ctx, base);
	// 82E8229C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E822A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E822A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E822A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E822AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E822B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E822B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E822B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E822B8 size=148
    let mut pc: u32 = 0x82E822B8;
    'dispatch: loop {
        match pc {
            0x82E822B8 => {
    //   block [0x82E822B8..0x82E8234C)
	// 82E822B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E822BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E822C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E822C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E822C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E822CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E822D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E822D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E822D8: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E822DC: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E822E0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82E822E4: 4BF70105  bl 0x82df23e8
	ctx.lr = 0x82E822E8;
	sub_82DF23E8(ctx, base);
	// 82E822E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E822EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E822F0: 419A0030  beq cr6, 0x82e82320
	if ctx.cr[6].eq {
	pc = 0x82E82320; continue 'dispatch;
	}
	// 82E822F4: 4BF7C4A5  bl 0x82dfe798
	ctx.lr = 0x82E822F8;
	sub_82DFE798(ctx, base);
	// 82E822F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E822FC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E82300: 394BE0A0  addi r10, r11, -0x1f60
	ctx.r[10].s64 = ctx.r[11].s64 + -8032;
	// 82E82304: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E82308: 4BFFD269  bl 0x82e7f570
	ctx.lr = 0x82E8230C;
	sub_82E7F570(ctx, base);
	// 82E8230C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E82310: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E82314: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E82318: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E8231C: 48000008  b 0x82e82324
	pc = 0x82E82324; continue 'dispatch;
	// 82E82320: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E82324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82328: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E8232C: 4BF7C54D  bl 0x82dfe878
	ctx.lr = 0x82E82330;
	sub_82DFE878(ctx, base);
	// 82E82330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E82338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E8233C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E82340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E82344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E82348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82350 size=164
    let mut pc: u32 = 0x82E82350;
    'dispatch: loop {
        match pc {
            0x82E82350 => {
    //   block [0x82E82350..0x82E823F4)
	// 82E82350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82354: 48325E19  bl 0x831a816c
	ctx.lr = 0x82E82358;
	sub_831A8130(ctx, base);
	// 82E82358: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8235C: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E82360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82364: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E82368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E8236C: 392B22B8  addi r9, r11, 0x22b8
	ctx.r[9].s64 = ctx.r[11].s64 + 8888;
	// 82E82370: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E82374: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E82378: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E8237C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E82380: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E82384: 4BF7C575  bl 0x82dfe8f8
	ctx.lr = 0x82E82388;
	sub_82DFE8F8(ctx, base);
	// 82E82388: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E8238C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E82390: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E82394: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E82398: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E8239C: 4BF7F4FD  bl 0x82e01898
	ctx.lr = 0x82E823A0;
	sub_82E01898(ctx, base);
	// 82E823A0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E823A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E823A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E823AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E823B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E823B4: 419A0024  beq cr6, 0x82e823d8
	if ctx.cr[6].eq {
	pc = 0x82E823D8; continue 'dispatch;
	}
	// 82E823B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E823BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E823C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E823C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E823C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E823CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E823D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E823D4: 4082FFE8  bne 0x82e823bc
	if !ctx.cr[0].eq {
	pc = 0x82E823BC; continue 'dispatch;
	}
	// 82E823D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E823DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E823E0: 419A0008  beq cr6, 0x82e823e8
	if ctx.cr[6].eq {
	pc = 0x82E823E8; continue 'dispatch;
	}
	// 82E823E4: 4B43E4AD  bl 0x822c0890
	ctx.lr = 0x82E823E8;
	sub_822C0890(ctx, base);
	// 82E823E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E823EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E823F0: 48325DCC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E823F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E823F8 size=104
    let mut pc: u32 = 0x82E823F8;
    'dispatch: loop {
        match pc {
            0x82E823F8 => {
    //   block [0x82E823F8..0x82E82460)
	// 82E823F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E823FC: 48325D71  bl 0x831a816c
	ctx.lr = 0x82E82400;
	sub_831A8130(ctx, base);
	// 82E82400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E82404: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E82408: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8240C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E82410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82414: 388BBB0C  addi r4, r11, -0x44f4
	ctx.r[4].s64 = ctx.r[11].s64 + -17652;
	// 82E82418: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E8241C: 4BF715ED  bl 0x82df3a08
	ctx.lr = 0x82E82420;
	sub_82DF3A08(ctx, base);
	// 82E82420: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E82424: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E82428: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E8242C: 480082ED  bl 0x82e8a718
	ctx.lr = 0x82E82430;
	sub_82E8A718(ctx, base);
	// 82E82430: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E82434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82438: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8243C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E82440: 4BFFFF11  bl 0x82e82350
	ctx.lr = 0x82E82444;
	sub_82E82350(ctx, base);
	// 82E82444: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E82448: 4BF70FE1  bl 0x82df3428
	ctx.lr = 0x82E8244C;
	sub_82DF3428(ctx, base);
	// 82E8244C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82450: 4BF70FD9  bl 0x82df3428
	ctx.lr = 0x82E82454;
	sub_82DF3428(ctx, base);
	// 82E82454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E8245C: 48325D60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82460 size=96
    let mut pc: u32 = 0x82E82460;
    'dispatch: loop {
        match pc {
            0x82E82460 => {
    //   block [0x82E82460..0x82E824C0)
	// 82E82460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E82468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E8246C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E82470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E82474: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E82478: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8247C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82480: 388BBB00  addi r4, r11, -0x4500
	ctx.r[4].s64 = ctx.r[11].s64 + -17664;
	// 82E82484: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E82488: 4BF71581  bl 0x82df3a08
	ctx.lr = 0x82E8248C;
	sub_82DF3A08(ctx, base);
	// 82E8248C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82490: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E82494: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E82498: 48008281  bl 0x82e8a718
	ctx.lr = 0x82E8249C;
	sub_82E8A718(ctx, base);
	// 82E8249C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E824A0: 4BF70F89  bl 0x82df3428
	ctx.lr = 0x82E824A4;
	sub_82DF3428(ctx, base);
	// 82E824A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E824A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E824AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E824B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E824B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E824B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E824BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E824C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E824C0 size=112
    let mut pc: u32 = 0x82E824C0;
    'dispatch: loop {
        match pc {
            0x82E824C0 => {
    //   block [0x82E824C0..0x82E82530)
	// 82E824C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E824C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E824C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E824CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E824D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E824D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E824D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E824DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E824E0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E824E4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E824E8: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82E824EC: 4BF6FEFD  bl 0x82df23e8
	ctx.lr = 0x82E824F0;
	sub_82DF23E8(ctx, base);
	// 82E824F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E824F4: 419A0010  beq cr6, 0x82e82504
	if ctx.cr[6].eq {
	pc = 0x82E82504; continue 'dispatch;
	}
	// 82E824F8: 4BFFCA79  bl 0x82e7ef70
	ctx.lr = 0x82E824FC;
	sub_82E7EF70(ctx, base);
	// 82E824FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82500: 48000008  b 0x82e82508
	pc = 0x82E82508; continue 'dispatch;
	// 82E82504: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E82508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8250C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E82510: 4BF7C369  bl 0x82dfe878
	ctx.lr = 0x82E82514;
	sub_82DFE878(ctx, base);
	// 82E82514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E8251C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E82520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E82524: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E82528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E8252C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82530 size=164
    let mut pc: u32 = 0x82E82530;
    'dispatch: loop {
        match pc {
            0x82E82530 => {
    //   block [0x82E82530..0x82E825D4)
	// 82E82530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82534: 48325C39  bl 0x831a816c
	ctx.lr = 0x82E82538;
	sub_831A8130(ctx, base);
	// 82E82538: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8253C: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E82540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82544: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E82548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E8254C: 392B24C0  addi r9, r11, 0x24c0
	ctx.r[9].s64 = ctx.r[11].s64 + 9408;
	// 82E82550: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E82554: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E82558: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E8255C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E82560: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E82564: 4BF7C395  bl 0x82dfe8f8
	ctx.lr = 0x82E82568;
	sub_82DFE8F8(ctx, base);
	// 82E82568: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E8256C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E82570: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E82574: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E82578: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E8257C: 4BF7F31D  bl 0x82e01898
	ctx.lr = 0x82E82580;
	sub_82E01898(ctx, base);
	// 82E82580: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82584: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E82588: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8258C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E82590: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E82594: 419A0024  beq cr6, 0x82e825b8
	if ctx.cr[6].eq {
	pc = 0x82E825B8; continue 'dispatch;
	}
	// 82E82598: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E8259C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E825A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E825A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E825A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E825AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E825B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E825B4: 4082FFE8  bne 0x82e8259c
	if !ctx.cr[0].eq {
	pc = 0x82E8259C; continue 'dispatch;
	}
	// 82E825B8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E825BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E825C0: 419A0008  beq cr6, 0x82e825c8
	if ctx.cr[6].eq {
	pc = 0x82E825C8; continue 'dispatch;
	}
	// 82E825C4: 4B43E2CD  bl 0x822c0890
	ctx.lr = 0x82E825C8;
	sub_822C0890(ctx, base);
	// 82E825C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E825CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E825D0: 48325BEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E825D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E825D8 size=104
    let mut pc: u32 = 0x82E825D8;
    'dispatch: loop {
        match pc {
            0x82E825D8 => {
    //   block [0x82E825D8..0x82E82640)
	// 82E825D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E825DC: 48325B91  bl 0x831a816c
	ctx.lr = 0x82E825E0;
	sub_831A8130(ctx, base);
	// 82E825E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E825E4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E825E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E825EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E825F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E825F4: 388BBB00  addi r4, r11, -0x4500
	ctx.r[4].s64 = ctx.r[11].s64 + -17664;
	// 82E825F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E825FC: 4BF7140D  bl 0x82df3a08
	ctx.lr = 0x82E82600;
	sub_82DF3A08(ctx, base);
	// 82E82600: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E82604: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E82608: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E8260C: 4800810D  bl 0x82e8a718
	ctx.lr = 0x82E82610;
	sub_82E8A718(ctx, base);
	// 82E82610: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E82614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82618: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8261C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E82620: 4BFFFF11  bl 0x82e82530
	ctx.lr = 0x82E82624;
	sub_82E82530(ctx, base);
	// 82E82624: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E82628: 4BF70E01  bl 0x82df3428
	ctx.lr = 0x82E8262C;
	sub_82DF3428(ctx, base);
	// 82E8262C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82630: 4BF70DF9  bl 0x82df3428
	ctx.lr = 0x82E82634;
	sub_82DF3428(ctx, base);
	// 82E82634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E8263C: 48325B80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E82640 size=216
    let mut pc: u32 = 0x82E82640;
    'dispatch: loop {
        match pc {
            0x82E82640 => {
    //   block [0x82E82640..0x82E82718)
	// 82E82640: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E82644: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82648: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E8264C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82650: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82E82654: 38EA0007  addi r7, r10, 7
	ctx.r[7].s64 = ctx.r[10].s64 + 7;
	// 82E82658: 38CB6910  addi r6, r11, 0x6910
	ctx.r[6].s64 = ctx.r[11].s64 + 26896;
	// 82E8265C: 54EB003A  rlwinm r11, r7, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82660: 7CA9502E  lwzx r5, r9, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E82664: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E82668: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E8266C: 13E030C7  vcmpequd (lvx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E82670: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E82674: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E82678: 554B003A  rlwinm r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8267C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E82718 size=260
    let mut pc: u32 = 0x82E82718;
    'dispatch: loop {
        match pc {
            0x82E82718 => {
    //   block [0x82E82718..0x82E8281C)
	// 82E82718: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82E8271C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82E82720: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82724: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E82728: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8272C: 390A0007  addi r8, r10, 7
	ctx.r[8].s64 = ctx.r[10].s64 + 7;
	// 82E82730: 38EB6910  addi r7, r11, 0x6910
	ctx.r[7].s64 = ctx.r[11].s64 + 26896;
	// 82E82734: 550B003A  rlwinm r11, r8, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82738: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E8273C: 7CA9502E  lwzx r5, r9, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E82740: 54CA003A  rlwinm r10, r6, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82744: 13E038C7  vcmpequd (lvx128) v31, v0, v7
	tmp.u32 = ctx.r[7].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E82748: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E8274C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82E82750: 38CA0007  addi r6, r10, 7
	ctx.r[6].s64 = ctx.r[10].s64 + 7;
	// 82E82754: 7CEB482E  lwzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E82758: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E8275C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82E82760: 54CB003A  rlwinm r11, r6, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82764: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E82768: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E82820 size=252
    let mut pc: u32 = 0x82E82820;
    'dispatch: loop {
        match pc {
            0x82E82820 => {
    //   block [0x82E82820..0x82E8291C)
	// 82E82820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82824: 4832593D  bl 0x831a8160
	ctx.lr = 0x82E82828;
	sub_831A8130(ctx, base);
	// 82E82828: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8282C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82E82830: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82834: 38EA0007  addi r7, r10, 7
	ctx.r[7].s64 = ctx.r[10].s64 + 7;
	// 82E82838: 38C9BA80  addi r6, r9, -0x4580
	ctx.r[6].s64 = ctx.r[9].s64 + -17792;
	// 82E8283C: 54EB003A  rlwinm r11, r7, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82840: C009BA80  lfs f0, -0x4580(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E82844: 38AB0007  addi r5, r11, 7
	ctx.r[5].s64 = ctx.r[11].s64 + 7;
	// 82E82848: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E8284C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E82850: 54AA003A  rlwinm r10, r5, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82854: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E82858: C1A60008  lfs f13, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E8285C: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E82860: 392A0007  addi r9, r10, 7
	ctx.r[9].s64 = ctx.r[10].s64 + 7;
	// 82E82864: C186000C  lfs f12, 0xc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E82868: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E8286C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E82870: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82874: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E82878: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E8287C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E82880: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E82884: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E82888: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E8288C: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82E82890: 7D09402E  lwzx r8, r9, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E82894: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82E82898: 50E5843E  rlwimi r5, r7, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E8289C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E828A0: 50E6801E  rlwimi r6, r7, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 82E828A4: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E828A8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E828AC: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E828B0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E828B4: 517F843E  rlwimi r31, r11, 0x10, 0x10, 0x1f
	ctx.r[31].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[31].u64 & 0xFFFFFFFFFFFF0000);
	// 82E828B8: 517E801E  rlwimi r30, r11, 0x10, 0, 0xf
	ctx.r[30].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[30].u64 & 0xFFFFFFFF0000FFFF);
	// 82E828BC: 515C801E  rlwimi r28, r10, 0x10, 0, 0xf
	ctx.r[28].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[28].u64 & 0xFFFFFFFF0000FFFF);
	// 82E828C0: 515D843E  rlwimi r29, r10, 0x10, 0x10, 0x1f
	ctx.r[29].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[29].u64 & 0xFFFFFFFFFFFF0000);
	// 82E828C4: 54A7C43E  rlwinm r7, r5, 0x18, 0x10, 0x1f
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E828C8: 511B843E  rlwimi r27, r8, 0x10, 0x10, 0x1f
	ctx.r[27].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[27].u64 & 0xFFFFFFFFFFFF0000);
	// 82E828CC: 511A801E  rlwimi r26, r8, 0x10, 0, 0xf
	ctx.r[26].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[26].u64 & 0xFFFFFFFF0000FFFF);
	// 82E828D0: 54C6401E  rlwinm r6, r6, 8, 0, 0xf
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82E828D4: 57E5C43E  rlwinm r5, r31, 0x18, 0x10, 0x1f
	ctx.r[5].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82E828D8: 57CB401E  rlwinm r11, r30, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00FFFFFFu64;
	// 82E828DC: 57AAC43E  rlwinm r10, r29, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82E828E0: 5788401E  rlwinm r8, r28, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[28].u32 as u64 & 0x00FFFFFFu64;
	// 82E828E4: 7CE73378  or r7, r7, r6
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82E828E8: 577FC43E  rlwinm r31, r27, 0x18, 0x10, 0x1f
	ctx.r[31].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 82E828EC: 575E401E  rlwinm r30, r26, 8, 0, 0xf
	ctx.r[30].u64 = ctx.r[26].u32 as u64 & 0x00FFFFFFu64;
	// 82E828F0: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E828F4: 39290007  addi r9, r9, 7
	ctx.r[9].s64 = ctx.r[9].s64 + 7;
	// 82E828F8: 7CA65B78  or r6, r5, r11
	ctx.r[6].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82E828FC: 7D454378  or r5, r10, r8
	ctx.r[5].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 82E82900: 7FEBF378  or r11, r31, r30
	ctx.r[11].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 82E82904: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E82908: 552A003A  rlwinm r10, r9, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8290C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E82910: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E82914: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E82918: 48325898  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E82920 size=500
    let mut pc: u32 = 0x82E82920;
    'dispatch: loop {
        match pc {
            0x82E82920 => {
    //   block [0x82E82920..0x82E82B14)
	// 82E82920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82924: 48325845  bl 0x831a8168
	ctx.lr = 0x82E82928;
	sub_831A8130(ctx, base);
	// 82E82928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8292C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82930: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E82934: 4BF707BD  bl 0x82df30f0
	ctx.lr = 0x82E82938;
	sub_82DF30F0(ctx, base);
	// 82E82938: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82E8293C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E82940: 4BF707B1  bl 0x82df30f0
	ctx.lr = 0x82E82944;
	sub_82DF30F0(ctx, base);
	// 82E82944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82948: 4BF707A9  bl 0x82df30f0
	ctx.lr = 0x82E8294C;
	sub_82DF30F0(ctx, base);
	// 82E8294C: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82950: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E82958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E8295C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E82960: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E82964: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82968: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E8296C: 4BF71D45  bl 0x82df46b0
	ctx.lr = 0x82E82970;
	sub_82DF46B0(ctx, base);
	// 82E82970: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82974: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E82978: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E8297C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82980: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E82984: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82988: 913C0004  stw r9, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E8298C: 4BF71245  bl 0x82df3bd0
	ctx.lr = 0x82E82990;
	sub_82DF3BD0(ctx, base);
	// 82E82990: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82994: 4BF70A95  bl 0x82df3428
	ctx.lr = 0x82E82998;
	sub_82DF3428(ctx, base);
	// 82E82998: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E8299C: 4BF70755  bl 0x82df30f0
	ctx.lr = 0x82E829A0;
	sub_82DF30F0(ctx, base);
	// 82E829A0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E829A4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E829A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E829AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E829B0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E829B4: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E829B8: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E829BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E829C0: 4BF71CF1  bl 0x82df46b0
	ctx.lr = 0x82E829C4;
	sub_82DF46B0(ctx, base);
	// 82E829C4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E829C8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E829CC: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E829D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E829D4: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 82E829D8: 54E6003A  rlwinm r6, r7, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E829DC: 90DC0004  stw r6, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E829E0: 4BF711F1  bl 0x82df3bd0
	ctx.lr = 0x82E829E4;
	sub_82DF3BD0(ctx, base);
	// 82E829E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E829E8: 4BF70A41  bl 0x82df3428
	ctx.lr = 0x82E829EC;
	sub_82DF3428(ctx, base);
	// 82E829EC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E829F0: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E829F4: 38AB0007  addi r5, r11, 7
	ctx.r[5].s64 = ctx.r[11].s64 + 7;
	// 82E829F8: 54A4003A  rlwinm r4, r5, 0, 0, 0x1d
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E829FC: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82E82A00: 909C0004  stw r4, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E82A04: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E82A08: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E82A0C: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82A10: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82A14: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E82A18: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E82A1C: 7D063B78  or r6, r8, r7
	ctx.r[6].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E82A20: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82E82A24: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82A28: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82A2C: 388B0007  addi r4, r11, 7
	ctx.r[4].s64 = ctx.r[11].s64 + 7;
	// 82E82A30: 7C6B282E  lwzx r3, r11, r5
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E82A34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E82A38: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E82A3C: 506B843E  rlwimi r11, r3, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82A40: 506A801E  rlwimi r10, r3, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82A44: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E82A48: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E82A4C: 5487003A  rlwinm r7, r4, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82A50: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E82A54: 90FC0004  stw r7, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E82A58: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82E82A5C: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82A60: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82A64: 386B000B  addi r3, r11, 0xb
	ctx.r[3].s64 = ctx.r[11].s64 + 11;
	// 82E82A68: 7C8B282A  ldx r4, r11, r5
	ctx.r[4].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) };
	// 82E82A6C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E82A70: 508B801E  rlwimi r11, r4, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E82A78: 5568401E  rlwinm r8, r11, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82E82A7C: 78890022  rldicl r9, r4, 0x20, 0x20
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82E82A80: 7906460C  rldimi r6, r8, 8, 0x18
	ctx.r[6].u64 = ((ctx.r[8].u64).rotate_left(8) & 0x000000FFFFFFFF00) | (ctx.r[6].u64 & 0xFFFFFF00000000FF);
	// 82E82A84: 548A021E  rlwinm r10, r4, 0, 8, 0xf
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82A88: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82E82A8C: 7CCB5378  or r11, r6, r10
	ctx.r[11].u64 = ctx.r[6].u64 | ctx.r[10].u64;
	// 82E82A90: 5125801E  rlwimi r5, r9, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82A94: 796A83E4  sldi r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u64.wrapping_shl(16);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 82E82A98: 5487000E  rlwinm r7, r4, 0, 0, 7
	ctx.r[7].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82A9C: 78890222  rldicl r9, r4, 0x20, 0x28
	ctx.r[9].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82E82AA0: 78888420  rldicl r8, r4, 0x10, 0x30
	ctx.r[8].u64 = ctx.r[4].u64 & 0x0000FFFFFFFFFFFFu64;
	// 82E82AA4: 7D473B78  or r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82E82AA8: 792903E4  rldicr r9, r9, 0, 0x2f
	ctx.r[9].u64 = (ctx.r[9].u64).rotate_left(0) & 0xFFFFFFFFFFFF0000;
	// 82E82AAC: 54A6401E  rlwinm r6, r5, 8, 0, 0xf
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E82AB0: 7D244378  or r4, r9, r8
	ctx.r[4].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E82AB4: 78E545E4  sldi r5, r7, 8
	ctx.r[5].u64 = ctx.r[7].u64.wrapping_shl(8);
	ctx.r[5].u32 = ctx.r[5].u64 as u32;
	// 82E82AB8: 788AC202  rldicl r10, r4, 0x38, 8
	ctx.r[10].u64 = ctx.r[4].u64 & 0x00000000000000FFu64;
	// 82E82ABC: 7CAB3378  or r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 | ctx.r[6].u64;
	// 82E82AC0: 5469003A  rlwinm r9, r3, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82AC4: 7D685378  or r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E82AC8: 913C0004  stw r9, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E82ACC: F91F0008  std r8, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 82E82AD0: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82AD4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82AD8: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E82ADC: 7CAB382E  lwzx r5, r11, r7
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E82AE0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E82AE4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E82AE8: 50A4843E  rlwimi r4, r5, 0x10, 0x10, 0x1f
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82AEC: 50A3801E  rlwimi r3, r5, 0x10, 0, 0xf
	ctx.r[3].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[3].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82AF0: 548BC43E  rlwinm r11, r4, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E82AF4: 546A401E  rlwinm r10, r3, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00FFFFFFu64;
	// 82E82AF8: 54C9003A  rlwinm r9, r6, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82AFC: 7D685378  or r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E82B00: 913C0004  stw r9, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E82B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E82B08: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82E82B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E82B10: 483256A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E82B18 size=816
    let mut pc: u32 = 0x82E82B18;
    'dispatch: loop {
        match pc {
            0x82E82B18 => {
    //   block [0x82E82B18..0x82E82E48)
	// 82E82B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82B1C: 4832564D  bl 0x831a8168
	ctx.lr = 0x82E82B20;
	sub_831A8130(ctx, base);
	// 82E82B20: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E82B24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E82B28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82B2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E82B30: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E82B34: 4BF705BD  bl 0x82df30f0
	ctx.lr = 0x82E82B38;
	sub_82DF30F0(ctx, base);
	// 82E82B38: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82B3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E82B40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82B44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82B48: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E82B4C: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E82B50: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82B54: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E82B58: 4BF71B59  bl 0x82df46b0
	ctx.lr = 0x82E82B5C;
	sub_82DF46B0(ctx, base);
	// 82E82B5C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82B60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E82B64: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E82B68: 387E0040  addi r3, r30, 0x40
	ctx.r[3].s64 = ctx.r[30].s64 + 64;
	// 82E82B6C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E82B70: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82B74: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E82B78: 4BF71059  bl 0x82df3bd0
	ctx.lr = 0x82E82B7C;
	sub_82DF3BD0(ctx, base);
	// 82E82B7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82B80: 4BF708A9  bl 0x82df3428
	ctx.lr = 0x82E82B84;
	sub_82DF3428(ctx, base);
	// 82E82B84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82B88: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82B8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E82B90: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E82B94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E82B98: 5507003A  rlwinm r7, r8, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82B9C: 7CAB302E  lwzx r5, r11, r6
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E82BA0: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E82BA4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E82BA8: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E82BAC: 50AB843E  rlwimi r11, r5, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82BB0: 50AA801E  rlwimi r10, r5, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82BB4: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E82BB8: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E82BBC: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E82BC0: 90FE0000  stw r7, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E82BC4: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82BC8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82BCC: 38AB0007  addi r5, r11, 7
	ctx.r[5].s64 = ctx.r[11].s64 + 7;
	// 82E82BD0: 7D6B302E  lwzx r11, r11, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E82BD4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E82BD8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E82BDC: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82BE0: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82BE4: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E82BE8: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E82BEC: 54A6003A  rlwinm r6, r5, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82BF0: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E82BF4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E82BF8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82E82BFC: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E82C00: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E82C04: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82C08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82C0C: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E82C10: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E82C14: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82E82C18: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82E82C1C: 5107843E  rlwimi r7, r8, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82C20: 5106801E  rlwimi r6, r8, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82C24: 54E5C43E  rlwinm r5, r7, 0x18, 0x10, 0x1f
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82E82C28: 54CB401E  rlwinm r11, r6, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82E82C2C: 552A003A  rlwinm r10, r9, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82C30: 7CA95B78  or r9, r5, r11
	ctx.r[9].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82E82C34: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E82C38: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E82C3C: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E82C40: D1BE0008  stfs f13, 8(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E82C44: 4BFFF9FD  bl 0x82e82640
	ctx.lr = 0x82E82C48;
	sub_82E82640(ctx, base);
	// 82E82C48: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82E82C4C: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82E82C50: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E82C54: 13E040C7  vcmpequd (lvx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E82E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E82E48 size=792
    let mut pc: u32 = 0x82E82E48;
    'dispatch: loop {
        match pc {
            0x82E82E48 => {
    //   block [0x82E82E48..0x82E83160)
	// 82E82E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E82E4C: 48325321  bl 0x831a816c
	ctx.lr = 0x82E82E50;
	sub_831A8130(ctx, base);
	// 82E82E50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E82E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E82E58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82E5C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E82E60: 4BF70291  bl 0x82df30f0
	ctx.lr = 0x82E82E64;
	sub_82DF30F0(ctx, base);
	// 82E82E64: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82E68: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82E6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E82E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82E74: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E82E78: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E82E7C: 8BCB0000  lbz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82E80: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E82E84: 4BF7182D  bl 0x82df46b0
	ctx.lr = 0x82E82E88;
	sub_82DF46B0(ctx, base);
	// 82E82E88: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82E8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E82E90: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E82E94: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 82E82E98: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E82E9C: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82EA0: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E82EA4: 4BF70D2D  bl 0x82df3bd0
	ctx.lr = 0x82E82EA8;
	sub_82DF3BD0(ctx, base);
	// 82E82EA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E82EAC: 4BF7057D  bl 0x82df3428
	ctx.lr = 0x82E82EB0;
	sub_82DF3428(ctx, base);
	// 82E82EB0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82EB4: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82EB8: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E82EBC: 5507003A  rlwinm r7, r8, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82EC0: 7CAB302E  lwzx r5, r11, r6
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E82EC4: 90FD0004  stw r7, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E82EC8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E82ECC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E82ED0: 50A4843E  rlwimi r4, r5, 0x10, 0x10, 0x1f
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82ED4: 50A3801E  rlwimi r3, r5, 0x10, 0, 0xf
	ctx.r[3].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[3].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82ED8: 548BC43E  rlwinm r11, r4, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E82EDC: 546A401E  rlwinm r10, r3, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00FFFFFFu64;
	// 82E82EE0: 7D695378  or r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E82EE4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E82EE8: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82EEC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82EF0: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E82EF4: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E82EF8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E82EFC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E82F00: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82F04: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82F08: 54A3C43E  rlwinm r3, r5, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E82F0C: 548B401E  rlwinm r11, r4, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E82F10: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82F14: 7C695B78  or r9, r3, r11
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E82F18: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E82F1C: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E82F20: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82F24: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82F28: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E82F2C: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E82F30: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E82F34: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E82F38: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82F3C: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82F40: 54A3C43E  rlwinm r3, r5, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E82F44: 548B401E  rlwinm r11, r4, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E82F48: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82F4C: 7C695B78  or r9, r3, r11
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E82F50: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E82F54: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E82F58: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82F5C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82F60: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E82F64: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E82F68: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E82F6C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E82F70: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82F74: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82F78: 54A3C43E  rlwinm r3, r5, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E82F7C: 548B401E  rlwinm r11, r4, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E82F80: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82F84: 7C695B78  or r9, r3, r11
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E82F88: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E82F8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E82F90: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82E82F94: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E82F98: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 82E82F9C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E82FA0: 98DF000C  stb r6, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u8 ) };
	// 82E82FA4: 80BD0000  lwz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E82FA8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E82FAC: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E82FB0: 7D2B282E  lwzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E82FB4: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82E82FB8: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82E82FBC: 5128843E  rlwimi r8, r9, 0x10, 0x10, 0x1f
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF0000);
	// 82E82FC0: 5127801E  rlwimi r7, r9, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82E82FC4: 5506C43E  rlwinm r6, r8, 0x18, 0x10, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82E82FC8: 54E5401E  rlwinm r5, r7, 8, 0, 0xf
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82E82FCC: 554B003A  rlwinm r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E82FD0: 7CCA2B78  or r10, r6, r5
	ctx.r[10].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 82E82FD4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E82FD8: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E82FDC: C0010054  lfs f0, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E82FE0: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E82FE4: 4BFFF65D  bl 0x82e82640
	ctx.lr = 0x82E82FE8;
	sub_82E82640(ctx, base);
	// 82E82FE8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E82FEC: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	// 82E82FF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E82FF4: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E83160 size=528
    let mut pc: u32 = 0x82E83160;
    'dispatch: loop {
        match pc {
            0x82E83160 => {
    //   block [0x82E83160..0x82E83370)
	// 82E83160: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83164: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83168: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E8316C: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83170: 7CEB502E  lwzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E83174: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E83178: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E8317C: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E83180: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83184: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83188: 54CBC43E  rlwinm r11, r6, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E8318C: 54AA401E  rlwinm r10, r5, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E83190: 7D695378  or r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E83194: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82E83198: 5507DFFE  rlwinm r7, r8, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E8319C: 68E60001  xori r6, r7, 1
	ctx.r[6].u64 = ctx.r[7].u64 ^ 1;
	// 82E831A0: 98C30000  stb r6, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 82E831A4: 80A40000  lwz r5, 0(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E831A8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E831AC: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E831B0: 7D2B282E  lwzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E831B4: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82E831B8: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82E831BC: 5128843E  rlwimi r8, r9, 0x10, 0x10, 0x1f
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF0000);
	// 82E831C0: 5127801E  rlwimi r7, r9, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82E831C4: 5506C43E  rlwinm r6, r8, 0x18, 0x10, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82E831C8: 54E5401E  rlwinm r5, r7, 8, 0, 0xf
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82E831CC: 554B003A  rlwinm r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E831D0: 7CCA2B78  or r10, r6, r5
	ctx.r[10].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 82E831D4: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E831D8: 7D490034  cntlzw r9, r10
	ctx.r[9].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E831DC: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82E831E0: 69070001  xori r7, r8, 1
	ctx.r[7].u64 = ctx.r[8].u64 ^ 1;
	// 82E831E4: 98E30001  stb r7, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 82E831E8: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E831EC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E831F0: 38AB0007  addi r5, r11, 7
	ctx.r[5].s64 = ctx.r[11].s64 + 7;
	// 82E831F4: 54AB003A  rlwinm r11, r5, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E831F8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E831FC: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E83200: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83204: 7D0B302E  lwzx r8, r11, r6
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E83208: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82E8320C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82E83210: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E83214: 5107843E  rlwimi r7, r8, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83218: 5106801E  rlwimi r6, r8, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 82E8321C: 54E5C43E  rlwinm r5, r7, 0x18, 0x10, 0x1f
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82E83220: 54CB401E  rlwinm r11, r6, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82E83224: 7CAA5B78  or r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82E83228: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 82E8322C: C001FFF0  lfs f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E83230: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E83234: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83238: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8323C: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E83240: 7CEB482E  lwzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83244: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E83248: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E8324C: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83250: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83254: 54CBC43E  rlwinm r11, r6, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E83258: 54AA401E  rlwinm r10, r5, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E8325C: 5509003A  rlwinm r9, r8, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83260: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E83264: 7D685378  or r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E83268: 9101FFF0  stw r8, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u32 ) };
	// 82E8326C: C1A1FFF0  lfs f13, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E83270: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E83274: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83278: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8327C: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E83280: 7CAB382E  lwzx r5, r11, r7
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E83284: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E83288: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E8328C: 54C9003A  rlwinm r9, r6, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83290: 50AB843E  rlwimi r11, r5, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83294: 50AA801E  rlwimi r10, r5, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83298: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E8329C: 5568C43E  rlwinm r8, r11, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E832A0: 5547401E  rlwinm r7, r10, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E832A4: 7D063B78  or r6, r8, r7
	ctx.r[6].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E832A8: 90C1FFF0  stw r6, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[6].u32 ) };
	// 82E832AC: C181FFF0  lfs f12, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E832B0: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E832B4: 80A40000  lwz r5, 0(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E832B8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E832BC: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E832C0: 7D2B282E  lwzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E832C4: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82E832C8: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82E832CC: 5128843E  rlwimi r8, r9, 0x10, 0x10, 0x1f
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF0000);
	// 82E832D0: 5127801E  rlwimi r7, r9, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82E832D4: 5506C43E  rlwinm r6, r8, 0x18, 0x10, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82E832D8: 54E5401E  rlwinm r5, r7, 8, 0, 0xf
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82E832DC: 554B003A  rlwinm r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E832E0: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E832E4: 7CCA2B78  or r10, r6, r5
	ctx.r[10].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 82E832E8: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 82E832EC: C161FFF0  lfs f11, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E832F0: D1630010  stfs f11, 0x10(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E832F4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E832F8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E832FC: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E83300: 7CEB482E  lwzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83304: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E83308: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E8330C: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83310: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83314: 54CBC43E  rlwinm r11, r6, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E83318: 54AA401E  rlwinm r10, r5, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E8331C: 5509003A  rlwinm r9, r8, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83320: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E83324: 7D685378  or r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E83328: 9101FFF0  stw r8, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u32 ) };
	// 82E8332C: C141FFF0  lfs f10, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E83330: D1430014  stfs f10, 0x14(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E83334: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83338: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8333C: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E83340: 7CAB382E  lwzx r5, r11, r7
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E83344: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E83348: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E8334C: 50AB843E  rlwimi r11, r5, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83350: 50AA801E  rlwimi r10, r5, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83354: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E83358: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E8335C: 54C7003A  rlwinm r7, r6, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83360: 7D264378  or r6, r9, r8
	ctx.r[6].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E83364: 90E40004  stw r7, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E83368: 90C30018  stw r6, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82E8336C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E83370 size=172
    let mut pc: u32 = 0x82E83370;
    'dispatch: loop {
        match pc {
            0x82E83370 => {
    //   block [0x82E83370..0x82E8341C)
	// 82E83370: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83374: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83378: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E8337C: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83380: 7CEB502E  lwzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E83384: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E83388: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E8338C: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E83390: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83394: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83398: 54CBC43E  rlwinm r11, r6, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E8339C: 54AA401E  rlwinm r10, r5, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E833A0: 7D695378  or r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E833A4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E833A8: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E833AC: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E833B0: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E833B4: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E833B8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E833BC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82E833C0: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E833C4: 50CB801E  rlwimi r11, r6, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82E833C8: 54AAC43E  rlwinm r10, r5, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E833CC: 5569401E  rlwinm r9, r11, 8, 0, 0xf
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82E833D0: 54E8003A  rlwinm r8, r7, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E833D4: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82E833D8: 91040004  stw r8, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E833DC: 90E30004  stw r7, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E833E0: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E833E4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E833E8: 38AB0007  addi r5, r11, 7
	ctx.r[5].s64 = ctx.r[11].s64 + 7;
	// 82E833EC: 7D6B302E  lwzx r11, r11, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E833F0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E833F4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E833F8: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E833FC: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83400: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E83404: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E83408: 54A6003A  rlwinm r6, r5, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8340C: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E83410: 90C40004  stw r6, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E83414: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82E83418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E83420 size=232
    let mut pc: u32 = 0x82E83420;
    'dispatch: loop {
        match pc {
            0x82E83420 => {
    //   block [0x82E83420..0x82E83508)
	// 82E83420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E83424: 48324D49  bl 0x831a816c
	ctx.lr = 0x82E83428;
	sub_831A8130(ctx, base);
	// 82E83428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8342C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E83430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83434: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E83438: 4BF6FCB9  bl 0x82df30f0
	ctx.lr = 0x82E8343C;
	sub_82DF30F0(ctx, base);
	// 82E8343C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83440: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E83448: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E8344C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E83450: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83454: 8BEB0000  lbz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83458: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E8345C: 4BF71255  bl 0x82df46b0
	ctx.lr = 0x82E83460;
	sub_82DF46B0(ctx, base);
	// 82E83460: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83464: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E83468: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E8346C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E83470: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E83474: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83478: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E8347C: 4BF70755  bl 0x82df3bd0
	ctx.lr = 0x82E83480;
	sub_82DF3BD0(ctx, base);
	// 82E83480: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83484: 4BF6FFA5  bl 0x82df3428
	ctx.lr = 0x82E83488;
	sub_82DF3428(ctx, base);
	// 82E83488: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8348C: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83490: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E83494: 5507003A  rlwinm r7, r8, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83498: 7CAB302E  lwzx r5, r11, r6
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E8349C: 90FD0004  stw r7, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E834A0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E834A4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E834A8: 50A4843E  rlwimi r4, r5, 0x10, 0x10, 0x1f
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF0000);
	// 82E834AC: 50A3801E  rlwimi r3, r5, 0x10, 0, 0xf
	ctx.r[3].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[3].u64 & 0xFFFFFFFF0000FFFF);
	// 82E834B0: 548BC43E  rlwinm r11, r4, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E834B4: 546A401E  rlwinm r10, r3, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00FFFFFFu64;
	// 82E834B8: 7D695378  or r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E834BC: 913E0010  stw r9, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82E834C0: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E834C4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E834C8: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E834CC: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E834D0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E834D4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E834D8: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E834DC: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E834E0: 54A3C43E  rlwinm r3, r5, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E834E4: 548B401E  rlwinm r11, r4, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E834E8: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E834EC: 7C695B78  or r9, r3, r11
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E834F0: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E834F4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E834F8: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E834FC: D01E000C  stfs f0, 0xc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E83500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E83504: 48324CB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E83508 size=500
    let mut pc: u32 = 0x82E83508;
    'dispatch: loop {
        match pc {
            0x82E83508 => {
    //   block [0x82E83508..0x82E836FC)
	// 82E83508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8350C: 48324C5D  bl 0x831a8168
	ctx.lr = 0x82E83510;
	sub_831A8130(ctx, base);
	// 82E83510: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E83514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E83518: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E8351C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E83520: 4BF6FBD1  bl 0x82df30f0
	ctx.lr = 0x82E83524;
	sub_82DF30F0(ctx, base);
	// 82E83524: 38BF0001  addi r5, r31, 1
	ctx.r[5].s64 = ctx.r[31].s64 + 1;
	// 82E83528: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E8352C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E83530: 8BBF0000  lbz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83534: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E83538: 4BF71179  bl 0x82df46b0
	ctx.lr = 0x82E8353C;
	sub_82DF46B0(ctx, base);
	// 82E8353C: 395D0004  addi r10, r29, 4
	ctx.r[10].s64 = ctx.r[29].s64 + 4;
	// 82E83540: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E83544: 555D003A  rlwinm r29, r10, 0, 0, 0x1d
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83548: 4BF6FBA9  bl 0x82df30f0
	ctx.lr = 0x82E8354C;
	sub_82DF30F0(ctx, base);
	// 82E8354C: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82E83550: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E83554: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83558: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E8355C: 7F9DF8AE  lbzx r28, r29, r31
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E83560: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E83564: 4BF7114D  bl 0x82df46b0
	ctx.lr = 0x82E83568;
	sub_82DF46B0(ctx, base);
	// 82E83568: 7D7CEA14  add r11, r28, r29
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82E8356C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83570: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82E83574: 550B003A  rlwinm r11, r8, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83578: 38EB000B  addi r7, r11, 0xb
	ctx.r[7].s64 = ctx.r[11].s64 + 11;
	// 82E8357C: 54EB003A  rlwinm r11, r7, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83580: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 82E83584: 54DD003A  rlwinm r29, r6, 0, 0, 0x1d
	ctx.r[29].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83588: 4BF6FB69  bl 0x82df30f0
	ctx.lr = 0x82E8358C;
	sub_82DF30F0(ctx, base);
	// 82E8358C: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82E83590: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E83594: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83598: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E8359C: 7F9DF8AE  lbzx r28, r29, r31
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E835A0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E835A4: 4BF7110D  bl 0x82df46b0
	ctx.lr = 0x82E835A8;
	sub_82DF46B0(ctx, base);
	// 82E835A8: 7D7CEA14  add r11, r28, r29
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82E835AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E835B0: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E835B4: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E835B8: 555D003A  rlwinm r29, r10, 0, 0, 0x1d
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E835BC: 4BF70615  bl 0x82df3bd0
	ctx.lr = 0x82E835C0;
	sub_82DF3BD0(ctx, base);
	// 82E835C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E835C4: 4BF6FE65  bl 0x82df3428
	ctx.lr = 0x82E835C8;
	sub_82DF3428(ctx, base);
	// 82E835C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E835CC: 4BF6FB25  bl 0x82df30f0
	ctx.lr = 0x82E835D0;
	sub_82DF30F0(ctx, base);
	// 82E835D0: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82E835D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E835D8: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E835DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E835E0: 7F9DF8AE  lbzx r28, r29, r31
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E835E4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E835E8: 4BF710C9  bl 0x82df46b0
	ctx.lr = 0x82E835EC;
	sub_82DF46B0(ctx, base);
	// 82E835EC: 7D7CEA14  add r11, r28, r29
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82E835F0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E835F4: 390B0004  addi r8, r11, 4
	ctx.r[8].s64 = ctx.r[11].s64 + 4;
	// 82E835F8: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82E835FC: 551D003A  rlwinm r29, r8, 0, 0, 0x1d
	ctx.r[29].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83600: 4BF705D1  bl 0x82df3bd0
	ctx.lr = 0x82E83604;
	sub_82DF3BD0(ctx, base);
	// 82E83604: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E83608: 4BF6FE21  bl 0x82df3428
	ctx.lr = 0x82E8360C;
	sub_82DF3428(ctx, base);
	// 82E8360C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E83610: 4BF6FAE1  bl 0x82df30f0
	ctx.lr = 0x82E83614;
	sub_82DF30F0(ctx, base);
	// 82E83614: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82E83618: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E8361C: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83620: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E83624: 7F9DF8AE  lbzx r28, r29, r31
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E83628: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E8362C: 4BF71085  bl 0x82df46b0
	ctx.lr = 0x82E83630;
	sub_82DF46B0(ctx, base);
	// 82E83630: 7D7CEA14  add r11, r28, r29
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82E83634: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E83638: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 82E8363C: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82E83640: 54DD003A  rlwinm r29, r6, 0, 0, 0x1d
	ctx.r[29].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83644: 4BF7058D  bl 0x82df3bd0
	ctx.lr = 0x82E83648;
	sub_82DF3BD0(ctx, base);
	// 82E83648: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E8364C: 4BF6FDDD  bl 0x82df3428
	ctx.lr = 0x82E83650;
	sub_82DF3428(ctx, base);
	// 82E83650: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E83654: 4BF6FA9D  bl 0x82df30f0
	ctx.lr = 0x82E83658;
	sub_82DF30F0(ctx, base);
	// 82E83658: 7F9DF8AE  lbzx r28, r29, r31
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E8365C: 7D7DFA14  add r11, r29, r31
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 82E83660: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E83664: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83668: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E8366C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E83670: 4BF71041  bl 0x82df46b0
	ctx.lr = 0x82E83674;
	sub_82DF46B0(ctx, base);
	// 82E83674: 7D7CEA14  add r11, r28, r29
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 82E83678: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E8367C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E83680: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82E83684: 555D003A  rlwinm r29, r10, 0, 0, 0x1d
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83688: 4BF70549  bl 0x82df3bd0
	ctx.lr = 0x82E8368C;
	sub_82DF3BD0(ctx, base);
	// 82E8368C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E83690: 4BF6FD99  bl 0x82df3428
	ctx.lr = 0x82E83694;
	sub_82DF3428(ctx, base);
	// 82E83694: 7CFDF82E  lwzx r7, r29, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E83698: 393D0007  addi r9, r29, 7
	ctx.r[9].s64 = ctx.r[29].s64 + 7;
	// 82E8369C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E836A0: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E836A4: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E836A8: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E836AC: 54C4C43E  rlwinm r4, r6, 0x18, 0x10, 0x1f
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E836B0: 54AB401E  rlwinm r11, r5, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E836B4: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E836B8: 7C8A5B78  or r10, r4, r11
	ctx.r[10].u64 = ctx.r[4].u64 | ctx.r[11].u64;
	// 82E836BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E836C0: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E836C4: 7D28F82E  lwzx r9, r8, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E836C8: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82E836CC: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82E836D0: 5128843E  rlwimi r8, r9, 0x10, 0x10, 0x1f
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF0000);
	// 82E836D4: 5127801E  rlwimi r7, r9, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82E836D8: 5506C43E  rlwinm r6, r8, 0x18, 0x10, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82E836DC: 54E5401E  rlwinm r5, r7, 8, 0, 0xf
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82E836E0: 7CC42B78  or r4, r6, r5
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 82E836E4: 909E0000  stw r4, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E836E8: 4BF6FD41  bl 0x82df3428
	ctx.lr = 0x82E836EC;
	sub_82DF3428(ctx, base);
	// 82E836EC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E836F0: 4BF6FD39  bl 0x82df3428
	ctx.lr = 0x82E836F4;
	sub_82DF3428(ctx, base);
	// 82E836F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E836F8: 48324AC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E83700 size=120
    let mut pc: u32 = 0x82E83700;
    'dispatch: loop {
        match pc {
            0x82E83700 => {
    //   block [0x82E83700..0x82E83778)
	// 82E83700: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E83704: 419A006C  beq cr6, 0x82e83770
	if ctx.cr[6].eq {
	pc = 0x82E83770; continue 'dispatch;
	}
	// 82E83708: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 82E8370C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 82E83710: 394A384C  addi r10, r10, 0x384c
	ctx.r[10].s64 = ctx.r[10].s64 + 14412;
	// 82E83714: 396B3854  addi r11, r11, 0x3854
	ctx.r[11].s64 = ctx.r[11].s64 + 14420;
	// 82E83718: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E8371C: 419A0044  beq cr6, 0x82e83760
	if ctx.cr[6].eq {
	pc = 0x82E83760; continue 'dispatch;
	}
	// 82E83720: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E83724: C0030004  lfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E83728: D0050004  stfs f0, 4(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E8372C: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83730: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E83734: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E83738: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E8373C: D1A50008  stfs f13, 8(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E83740: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E83744: D185000C  stfs f12, 0xc(r5)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E83748: C1630010  lfs f11, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E8374C: D1650010  stfs f11, 0x10(r5)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E83750: C1430014  lfs f10, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E83754: D1450014  stfs f10, 0x14(r5)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E83758: C1230018  lfs f9, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E8375C: D1250018  stfs f9, 0x18(r5)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E83760: 3863001C  addi r3, r3, 0x1c
	ctx.r[3].s64 = ctx.r[3].s64 + 28;
	// 82E83764: 38A5001C  addi r5, r5, 0x1c
	ctx.r[5].s64 = ctx.r[5].s64 + 28;
	// 82E83768: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E8376C: 409AFFAC  bne cr6, 0x82e83718
	if !ctx.cr[6].eq {
	pc = 0x82E83718; continue 'dispatch;
	}
	// 82E83770: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E83774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E83778 size=252
    let mut pc: u32 = 0x82E83778;
    'dispatch: loop {
        match pc {
            0x82E83778 => {
    //   block [0x82E83778..0x82E83874)
	// 82E83778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8377C: 483249E5  bl 0x831a8160
	ctx.lr = 0x82E83780;
	sub_831A8130(ctx, base);
	// 82E83780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E83784: 3D600924  lis r11, 0x924
	ctx.r[11].s64 = 153354240;
	// 82E83788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8378C: 616A9249  ori r10, r11, 0x9249
	ctx.r[10].u64 = ctx.r[11].u64 | 37449;
	// 82E83790: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E83794: 40990010  ble cr6, 0x82e837a4
	if !ctx.cr[6].gt {
	pc = 0x82E837A4; continue 'dispatch;
	}
	// 82E83798: 4BD34589  bl 0x82bb7d20
	ctx.lr = 0x82E8379C;
	sub_82BB7D20(ctx, base);
	// 82E8379C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E837A0: 48324A10  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E837A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E837A8: 3B40001C  li r26, 0x1c
	ctx.r[26].s64 = 28;
	// 82E837AC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E837B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E837B4: 409A000C  bne cr6, 0x82e837c0
	if !ctx.cr[6].eq {
	pc = 0x82E837C0; continue 'dispatch;
	}
	// 82E837B8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82E837BC: 48000010  b 0x82e837cc
	pc = 0x82E837CC; continue 'dispatch;
	// 82E837C0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E837C4: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E837C8: 7D69D3D6  divw r11, r9, r26
	ctx.r[11].s32 = ctx.r[9].s32 / ctx.r[26].s32;
	// 82E837CC: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E837D0: 4098009C  bge cr6, 0x82e8386c
	if !ctx.cr[6].lt {
	pc = 0x82E8386C; continue 'dispatch;
	}
	// 82E837D4: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82E837D8: 1F84001C  mulli r28, r4, 0x1c
	ctx.r[28].s64 = ctx.r[4].s64 * 28;
	// 82E837DC: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E837E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E837E4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E837E8: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 82E837EC: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E837F0: 4BF6E8D9  bl 0x82df20c8
	ctx.lr = 0x82E837F4;
	sub_82DF20C8(ctx, base);
	// 82E837F4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E837F8: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E837FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E83800: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83804: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E83808: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E8380C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E83810: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E83814: 9B6A0000  stb r27, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82E83818: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E8381C: 4BFFFEE5  bl 0x82e83700
	ctx.lr = 0x82E83820;
	sub_82E83700(ctx, base);
	// 82E83820: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83824: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E83828: 419A002C  beq cr6, 0x82e83854
	if ctx.cr[6].eq {
	pc = 0x82E83854; continue 'dispatch;
	}
	// 82E8382C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E83830: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E83834: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E83838: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82E8383C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E83840: 7F6AD3D6  divw r27, r10, r26
	ctx.r[27].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 82E83844: 4B947EB5  bl 0x827cb6f8
	ctx.lr = 0x82E83848;
	sub_827CB6F8(ctx, base);
	// 82E83848: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E8384C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83850: 4BF6E939  bl 0x82df2188
	ctx.lr = 0x82E83854;
	sub_82DF2188(ctx, base);
	// 82E83854: 1D7B001C  mulli r11, r27, 0x1c
	ctx.r[11].s64 = ctx.r[27].s64 * 28;
	// 82E83858: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E8385C: 7D5CF214  add r10, r28, r30
	ctx.r[10].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82E83860: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E83864: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E83868: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E8386C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E83870: 48324940  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E83878 size=248
    let mut pc: u32 = 0x82E83878;
    'dispatch: loop {
        match pc {
            0x82E83878 => {
    //   block [0x82E83878..0x82E83970)
	// 82E83878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8387C: 483248E9  bl 0x831a8164
	ctx.lr = 0x82E83880;
	sub_831A8130(ctx, base);
	// 82E83880: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E83884: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82E83888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E8388C: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82E83890: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E83894: 40990010  ble cr6, 0x82e838a4
	if !ctx.cr[6].gt {
	pc = 0x82E838A4; continue 'dispatch;
	}
	// 82E83898: 4BD34489  bl 0x82bb7d20
	ctx.lr = 0x82E8389C;
	sub_82BB7D20(ctx, base);
	// 82E8389C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E838A0: 48324914  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82E838A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E838A8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E838AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E838B0: 409A000C  bne cr6, 0x82e838bc
	if !ctx.cr[6].eq {
	pc = 0x82E838BC; continue 'dispatch;
	}
	// 82E838B4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82E838B8: 48000010  b 0x82e838c8
	pc = 0x82E838C8; continue 'dispatch;
	// 82E838BC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E838C0: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E838C4: 7D2B1E70  srawi r11, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 82E838C8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E838CC: 4098009C  bge cr6, 0x82e83968
	if !ctx.cr[6].lt {
	pc = 0x82E83968; continue 'dispatch;
	}
	// 82E838D0: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82E838D4: 549C1838  slwi r28, r4, 3
	ctx.r[28].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82E838D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E838DC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E838E0: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 82E838E4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E838E8: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E838EC: 4BF6E7DD  bl 0x82df20c8
	ctx.lr = 0x82E838F0;
	sub_82DF20C8(ctx, base);
	// 82E838F0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E838F4: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E838F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E838FC: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83900: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E83904: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E83908: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8390C: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E83910: 9B6A0000  stb r27, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82E83914: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E83918: 4BFFC8E9  bl 0x82e80200
	ctx.lr = 0x82E8391C;
	sub_82E80200(ctx, base);
	// 82E8391C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83920: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E83924: 419A002C  beq cr6, 0x82e83950
	if ctx.cr[6].eq {
	pc = 0x82E83950; continue 'dispatch;
	}
	// 82E83928: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8392C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E83930: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E83934: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82E83938: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E8393C: 7D5B1E70  srawi r27, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 82E83940: 4B94E949  bl 0x827d2288
	ctx.lr = 0x82E83944;
	sub_827D2288(ctx, base);
	// 82E83944: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E83948: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E8394C: 4BF6E83D  bl 0x82df2188
	ctx.lr = 0x82E83950;
	sub_82DF2188(ctx, base);
	// 82E83950: 576B1838  slwi r11, r27, 3
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E83954: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E83958: 7D5CF214  add r10, r28, r30
	ctx.r[10].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82E8395C: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E83960: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E83964: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E83968: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E8396C: 48324848  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E83970 size=568
    let mut pc: u32 = 0x82E83970;
    'dispatch: loop {
        match pc {
            0x82E83970 => {
    //   block [0x82E83970..0x82E83BA8)
	// 82E83970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E83974: 483247D1  bl 0x831a8144
	ctx.lr = 0x82E83978;
	sub_831A8130(ctx, base);
	// 82E83978: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8397C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E83980: 54BE063E  clrlwi r30, r5, 0x18
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E83984: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E83988: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E8398C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83990: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83994: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E83998: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8399C: 7CEB502E  lwzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E839A0: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E839A4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E839A8: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E839AC: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E839B0: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E839B4: 54C4C43E  rlwinm r4, r6, 0x18, 0x10, 0x1f
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E839B8: 54A3401E  rlwinm r3, r5, 8, 0, 0xf
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E839BC: 7C8B1B78  or r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 | ctx.r[3].u64;
	// 82E839C0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E839C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E839C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E839CC: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E839D0: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E839D4: 552A003A  rlwinm r10, r9, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E839D8: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82E839DC: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82E839E0: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 82E839E4: 5107843E  rlwimi r7, r8, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 82E839E8: 5106801E  rlwimi r6, r8, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 82E839EC: 54A4003A  rlwinm r4, r5, 0, 0, 0x1d
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E839F0: 54E3C43E  rlwinm r3, r7, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82E839F4: 54CB401E  rlwinm r11, r6, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82E839F8: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E839FC: 7C7D5B78  or r29, r3, r11
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E83A00: 419A0010  beq cr6, 0x82e83a10
	if ctx.cr[6].eq {
	pc = 0x82E83A10; continue 'dispatch;
	}
	// 82E83A04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83A08: 387B0004  addi r3, r27, 4
	ctx.r[3].s64 = ctx.r[27].s64 + 4;
	// 82E83A0C: 4BFFFD6D  bl 0x82e83778
	ctx.lr = 0x82E83A10;
	sub_82E83778(ctx, base);
	// 82E83A10: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E83A14: 4099018C  ble cr6, 0x82e83ba0
	if !ctx.cr[6].gt {
	pc = 0x82E83BA0; continue 'dispatch;
	}
	// 82E83A18: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 82E83A1C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 82E83A20: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 82E83A24: 3B4B384C  addi r26, r11, 0x384c
	ctx.r[26].s64 = ctx.r[11].s64 + 14412;
	// 82E83A28: 3BAA3854  addi r29, r10, 0x3854
	ctx.r[29].s64 = ctx.r[10].s64 + 14420;
	// 82E83A2C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83A30: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E83A34: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83A38: 390A0007  addi r8, r10, 7
	ctx.r[8].s64 = ctx.r[10].s64 + 7;
	// 82E83A3C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E83A40: 550B003A  rlwinm r11, r8, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83A44: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E83A48: 7CC9502E  lwzx r6, r9, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E83A4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E83A50: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83A54: 7CAB482E  lwzx r5, r11, r9
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83A58: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E83A5C: 386A0007  addi r3, r10, 7
	ctx.r[3].s64 = ctx.r[10].s64 + 7;
	// 82E83A60: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E83A64: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82E83A68: 546B003A  rlwinm r11, r3, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83A6C: 7CEA482E  lwzx r7, r10, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83A70: 50C8801E  rlwimi r8, r6, 0x10, 0, 0xf
	ctx.r[8].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[8].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83A74: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E83A78: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E83A7C: 50C4843E  rlwimi r4, r6, 0x10, 0x10, 0x1f
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83A80: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E83A84: 7F2B482E  lwzx r25, r11, r9
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83A88: 554B003A  rlwinm r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83A8C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82E83A90: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E83A94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E83A98: 50A3843E  rlwimi r3, r5, 0x10, 0x10, 0x1f
	ctx.r[3].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[3].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83A9C: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83AA0: 7EAB482E  lwzx r21, r11, r9
	ctx.r[21].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83AA4: 50B8801E  rlwimi r24, r5, 0x10, 0, 0xf
	ctx.r[24].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[24].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83AA8: 396A0007  addi r11, r10, 7
	ctx.r[11].s64 = ctx.r[10].s64 + 7;
	// 82E83AAC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E83AB0: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82E83AB4: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83AB8: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E83ABC: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82E83AC0: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82E83AC4: 50F7843E  rlwimi r23, r7, 0x10, 0x10, 0x1f
	ctx.r[23].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[23].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83AC8: 552B003A  rlwinm r11, r9, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83ACC: 50F6801E  rlwimi r22, r7, 0x10, 0, 0xf
	ctx.r[22].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[22].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83AD0: 38CB0004  addi r6, r11, 4
	ctx.r[6].s64 = ctx.r[11].s64 + 4;
	// 82E83AD4: 7F34CB78  mr r20, r25
	ctx.r[20].u64 = ctx.r[25].u64;
	// 82E83AD8: 54CB003A  rlwinm r11, r6, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83ADC: 7F33CB78  mr r19, r25
	ctx.r[19].u64 = ctx.r[25].u64;
	// 82E83AE0: 38AB0004  addi r5, r11, 4
	ctx.r[5].s64 = ctx.r[11].s64 + 4;
	// 82E83AE4: 5334843E  rlwimi r20, r25, 0x10, 0x10, 0x1f
	ctx.r[20].u64 = (((ctx.r[25].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[20].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83AE8: 54AB003A  rlwinm r11, r5, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83AEC: 5333801E  rlwimi r19, r25, 0x10, 0, 0xf
	ctx.r[19].u64 = (((ctx.r[25].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[19].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83AF0: 38EB0004  addi r7, r11, 4
	ctx.r[7].s64 = ctx.r[11].s64 + 4;
	// 82E83AF4: 5484C43E  rlwinm r4, r4, 0x18, 0x10, 0x1f
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E83AF8: 54EB003A  rlwinm r11, r7, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83AFC: 5508401E  rlwinm r8, r8, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00FFFFFFu64;
	// 82E83B00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E83B04: 5719401E  rlwinm r25, r24, 8, 0, 0xf
	ctx.r[25].u64 = ctx.r[24].u32 as u64 & 0x00FFFFFFu64;
	// 82E83B08: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 82E83B0C: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82E83B10: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82E83B14: 5463C43E  rlwinm r3, r3, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E83B18: 56F8C43E  rlwinm r24, r23, 0x18, 0x10, 0x1f
	ctx.r[24].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	// 82E83B1C: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 82E83B20: 56D7401E  rlwinm r23, r22, 8, 0, 0xf
	ctx.r[23].u64 = ctx.r[22].u32 as u64 & 0x00FFFFFFu64;
	// 82E83B24: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83B28: 7C884378  or r8, r4, r8
	ctx.r[8].u64 = ctx.r[4].u64 | ctx.r[8].u64;
	// 82E83B2C: 52A9843E  rlwimi r9, r21, 0x10, 0x10, 0x1f
	ctx.r[9].u64 = (((ctx.r[21].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[9].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83B30: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E83B34: 52A6801E  rlwimi r6, r21, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[21].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82E83B3C: 5145843E  rlwimi r5, r10, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83B40: 5147801E  rlwimi r7, r10, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[10].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83B44: 7C64CB78  or r4, r3, r25
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[25].u64;
	// 82E83B48: 568AC43E  rlwinm r10, r20, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	// 82E83B4C: 5676401E  rlwinm r22, r19, 8, 0, 0xf
	ctx.r[22].u64 = ctx.r[19].u32 as u64 & 0x00FFFFFFu64;
	// 82E83B50: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82E83B54: 7F03BB78  or r3, r24, r23
	ctx.r[3].u64 = ctx.r[24].u64 | ctx.r[23].u64;
	// 82E83B58: 552BC43E  rlwinm r11, r9, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82E83B5C: 54C9401E  rlwinm r9, r6, 8, 0, 0xf
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82E83B60: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82E83B64: 54A8C43E  rlwinm r8, r5, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E83B68: 54E7401E  rlwinm r7, r7, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82E83B6C: 7D46B378  or r6, r10, r22
	ctx.r[6].u64 = ctx.r[10].u64 | ctx.r[22].u64;
	// 82E83B70: 7D654B78  or r5, r11, r9
	ctx.r[5].u64 = ctx.r[11].u64 | ctx.r[9].u64;
	// 82E83B74: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82E83B78: 7D043B78  or r4, r8, r7
	ctx.r[4].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E83B7C: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82E83B80: 90810068  stw r4, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82E83B84: 419A0010  beq cr6, 0x82e83b94
	if ctx.cr[6].eq {
	pc = 0x82E83B94; continue 'dispatch;
	}
	// 82E83B88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E83B8C: 387B0004  addi r3, r27, 4
	ctx.r[3].s64 = ctx.r[27].s64 + 4;
	// 82E83B90: 4B948051  bl 0x827cbbe0
	ctx.lr = 0x82E83B94;
	sub_827CBBE0(ctx, base);
	// 82E83B94: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E83B98: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82E83B9C: 4082FE90  bne 0x82e83a2c
	if !ctx.cr[0].eq {
	pc = 0x82E83A2C; continue 'dispatch;
	}
	// 82E83BA0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E83BA4: 483245F0  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E83BA8 size=264
    let mut pc: u32 = 0x82E83BA8;
    'dispatch: loop {
        match pc {
            0x82E83BA8 => {
    //   block [0x82E83BA8..0x82E83CB0)
	// 82E83BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E83BAC: 483245BD  bl 0x831a8168
	ctx.lr = 0x82E83BB0;
	sub_831A8130(ctx, base);
	// 82E83BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E83BB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E83BB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83BBC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E83BC0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E83BC4: 4BF6F52D  bl 0x82df30f0
	ctx.lr = 0x82E83BC8;
	sub_82DF30F0(ctx, base);
	// 82E83BC8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83BCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E83BD0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83BD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83BD8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E83BDC: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83BE0: 8BEB0000  lbz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83BE4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E83BE8: 4BF70AC9  bl 0x82df46b0
	ctx.lr = 0x82E83BEC;
	sub_82DF46B0(ctx, base);
	// 82E83BEC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83BF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83BF4: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E83BF8: 578557FE  rlwinm r5, r28, 0xa, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x003FFFFFu64;
	// 82E83BFC: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E83C00: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83C04: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E83C08: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E83C0C: 4BFFFD65  bl 0x82e83970
	ctx.lr = 0x82E83C10;
	sub_82E83970(ctx, base);
	// 82E83C10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C14: 57855FFE  rlwinm r5, r28, 0xb, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x001FFFFFu64;
	// 82E83C18: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E83C1C: 4BFFFD55  bl 0x82e83970
	ctx.lr = 0x82E83C20;
	sub_82E83970(ctx, base);
	// 82E83C20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C24: 578567FE  rlwinm r5, r28, 0xc, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x000FFFFFu64;
	// 82E83C28: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E83C2C: 4BFFFD45  bl 0x82e83970
	ctx.lr = 0x82E83C30;
	sub_82E83970(ctx, base);
	// 82E83C30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C34: 57856FFE  rlwinm r5, r28, 0xd, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x0007FFFFu64;
	// 82E83C38: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E83C3C: 4BFFFD35  bl 0x82e83970
	ctx.lr = 0x82E83C40;
	sub_82E83970(ctx, base);
	// 82E83C40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C44: 57859FFE  rlwinm r5, r28, 0x13, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x00001FFFu64;
	// 82E83C48: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83C4C: 4BFFFD25  bl 0x82e83970
	ctx.lr = 0x82E83C50;
	sub_82E83970(ctx, base);
	// 82E83C50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C54: 578597FE  rlwinm r5, r28, 0x12, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x00003FFFu64;
	// 82E83C58: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E83C5C: 4BFFFD15  bl 0x82e83970
	ctx.lr = 0x82E83C60;
	sub_82E83970(ctx, base);
	// 82E83C60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C64: 57858FFE  rlwinm r5, r28, 0x11, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x00007FFFu64;
	// 82E83C68: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E83C6C: 4BFFFD05  bl 0x82e83970
	ctx.lr = 0x82E83C70;
	sub_82E83970(ctx, base);
	// 82E83C70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C74: 578587FE  rlwinm r5, r28, 0x10, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82E83C78: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E83C7C: 4BFFFCF5  bl 0x82e83970
	ctx.lr = 0x82E83C80;
	sub_82E83970(ctx, base);
	// 82E83C80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C84: 57857FFE  rlwinm r5, r28, 0xf, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x0001FFFFu64;
	// 82E83C88: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E83C8C: 4BFFFCE5  bl 0x82e83970
	ctx.lr = 0x82E83C90;
	sub_82E83970(ctx, base);
	// 82E83C90: 578577FE  rlwinm r5, r28, 0xe, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[28].u32 as u64 & 0x0003FFFFu64;
	// 82E83C94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E83C98: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E83C9C: 4BFFFCD5  bl 0x82e83970
	ctx.lr = 0x82E83CA0;
	sub_82E83970(ctx, base);
	// 82E83CA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83CA4: 4BF6F785  bl 0x82df3428
	ctx.lr = 0x82E83CA8;
	sub_82DF3428(ctx, base);
	// 82E83CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E83CAC: 4832450C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E83CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E83CB0 size=2356
    let mut pc: u32 = 0x82E83CB0;
    'dispatch: loop {
        match pc {
            0x82E83CB0 => {
    //   block [0x82E83CB0..0x82E845E4)
	// 82E83CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E83CB4: 483244B9  bl 0x831a816c
	ctx.lr = 0x82E83CB8;
	sub_831A8130(ctx, base);
	// 82E83CB8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E83CBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E83CC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83CC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E83CC8: 4BF6F429  bl 0x82df30f0
	ctx.lr = 0x82E83CCC;
	sub_82DF30F0(ctx, base);
	// 82E83CCC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83CD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83CD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E83CD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83CDC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E83CE0: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E83CE4: 8BAB0000  lbz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83CE8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E83CEC: 4BF709C5  bl 0x82df46b0
	ctx.lr = 0x82E83CF0;
	sub_82DF46B0(ctx, base);
	// 82E83CF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83CF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E83CF8: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E83CFC: 387E00D8  addi r3, r30, 0xd8
	ctx.r[3].s64 = ctx.r[30].s64 + 216;
	// 82E83D00: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E83D04: 5549003A  rlwinm r9, r10, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83D08: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E83D0C: 4BF6FEC5  bl 0x82df3bd0
	ctx.lr = 0x82E83D10;
	sub_82DF3BD0(ctx, base);
	// 82E83D10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E83D14: 4BF6F715  bl 0x82df3428
	ctx.lr = 0x82E83D18;
	sub_82DF3428(ctx, base);
	// 82E83D18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83D1C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83D20: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E83D24: 5507003A  rlwinm r7, r8, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83D28: 7CAB302E  lwzx r5, r11, r6
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E83D2C: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82E83D30: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E83D34: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E83D38: 50A4843E  rlwimi r4, r5, 0x10, 0x10, 0x1f
	ctx.r[4].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[4].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83D3C: 50A3801E  rlwimi r3, r5, 0x10, 0, 0xf
	ctx.r[3].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[3].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83D40: 548BC43E  rlwinm r11, r4, 0x18, 0x10, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E83D44: 546A401E  rlwinm r10, r3, 8, 0, 0xf
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00FFFFFFu64;
	// 82E83D48: 7D695378  or r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E83D4C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82E83D50: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E83D54: D01E00BC  stfs f0, 0xbc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82E83D58: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83D5C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83D60: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E83D64: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E83D68: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E83D6C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E83D70: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83D74: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83D78: 54A3C43E  rlwinm r3, r5, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E83D7C: 548B401E  rlwinm r11, r4, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E83D80: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83D84: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E83D88: 7C695B78  or r9, r3, r11
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E83D8C: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82E83D90: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E83D94: D1BE00C0  stfs f13, 0xc0(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82E83D98: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83D9C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83DA0: 38EB0007  addi r7, r11, 7
	ctx.r[7].s64 = ctx.r[11].s64 + 7;
	// 82E83DA4: 7CCB402E  lwzx r6, r11, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E83DA8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E83DAC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E83DB0: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83DB4: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83DB8: 54A3C43E  rlwinm r3, r5, 0x18, 0x10, 0x1f
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E83DBC: 548B401E  rlwinm r11, r4, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E83DC0: 54EA003A  rlwinm r10, r7, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83DC4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E83DC8: 7C695B78  or r9, r3, r11
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[11].u64;
	// 82E83DCC: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82E83DD0: C1810060  lfs f12, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E83DD4: FD60665E  fctidz f11, f12
	ctx.f[11].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 82E83DD8: D9610060  stfd f11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[11].u64 ) };
	// 82E83DDC: A1010066  lhz r8, 0x66(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82E83DE0: B11E0090  sth r8, 0x90(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[8].u16 ) };
	// 82E83DE4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83DE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83DEC: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E83DF0: 54C5003A  rlwinm r5, r6, 0, 0, 0x1d
	ctx.r[5].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83DF4: 7C8B382E  lwzx r4, r11, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E83DF8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E83DFC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82E83E00: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E83E04: 5083843E  rlwimi r3, r4, 0x10, 0x10, 0x1f
	ctx.r[3].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[3].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83E08: 508B801E  rlwimi r11, r4, 0x10, 0, 0xf
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[11].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83E0C: 546AC43E  rlwinm r10, r3, 0x18, 0x10, 0x1f
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E83E10: 5569401E  rlwinm r9, r11, 8, 0, 0xf
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82E83E14: 7D484B78  or r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82E83E18: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82E83E1C: C1410060  lfs f10, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E83E20: FD20565E  fctidz f9, f10
	ctx.f[9].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 82E83E24: D9210060  stfd f9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[9].u64 ) };
	// 82E83E28: A0E10066  lhz r7, 0x66(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82E83E2C: B0FE0092  sth r7, 0x92(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(146 as u32), ctx.r[7].u16 ) };
	// 82E83E30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83E34: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83E38: 38AB0007  addi r5, r11, 7
	ctx.r[5].s64 = ctx.r[11].s64 + 7;
	// 82E83E3C: 54A4003A  rlwinm r4, r5, 0, 0, 0x1d
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83E40: 7C6B302E  lwzx r3, r11, r6
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E83E44: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E83E48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E83E4C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E83E50: 506B843E  rlwimi r11, r3, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83E54: 506A801E  rlwimi r10, r3, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83E58: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E83E5C: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E83E60: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E83E64: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82E83E68: C1010060  lfs f8, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82E83E6C: FCE0465E  fctidz f7, f8
	ctx.f[7].s64 = if ctx.f[8].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[8].f64.trunc() as i64 };
	// 82E83E70: D8E10060  stfd f7, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[7].u64 ) };
	// 82E83E74: A0C10066  lhz r6, 0x66(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82E83E78: B0DE0094  sth r6, 0x94(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[6].u16 ) };
	// 82E83E7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83E80: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83E84: 388B0007  addi r4, r11, 7
	ctx.r[4].s64 = ctx.r[11].s64 + 7;
	// 82E83E88: 5483003A  rlwinm r3, r4, 0, 0, 0x1d
	ctx.r[3].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83E8C: 7D6B282E  lwzx r11, r11, r5
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E83E90: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E83E94: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E83E98: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E83E9C: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83EA0: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83EA4: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E83EA8: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E83EAC: 7D063B78  or r6, r8, r7
	ctx.r[6].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E83EB0: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82E83EB4: C0C10060  lfs f6, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E83EB8: FCA0365E  fctidz f5, f6
	ctx.f[5].s64 = if ctx.f[6].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[6].f64.trunc() as i64 };
	// 82E83EBC: D8A10060  stfd f5, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[5].u64 ) };
	// 82E83EC0: A0A10066  lhz r5, 0x66(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(102 as u32) ) } as u64;
	// 82E83EC4: B0BE0096  sth r5, 0x96(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(150 as u32), ctx.r[5].u16 ) };
	// 82E83EC8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83ECC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83ED0: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E83ED4: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83ED8: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E83EDC: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E83EE0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E83EE4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E83EE8: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83EEC: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83EF0: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E83EF4: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E83EF8: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E83EFC: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82E83F00: C0810060  lfs f4, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82E83F04: D09E0080  stfs f4, 0x80(r30)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E83F08: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83F0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83F10: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E83F14: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E83F18: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E83F1C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E83F20: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83F24: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83F28: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E83F2C: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E83F30: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83F34: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E83F38: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E83F3C: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82E83F40: C0610060  lfs f3, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E83F44: D07E0084  stfs f3, 0x84(r30)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E83F48: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83F4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83F50: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E83F54: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E83F58: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E83F5C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E83F60: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83F64: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83F68: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E83F6C: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E83F70: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83F74: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E83F78: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E83F7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E83F80: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82E83F84: C0410060  lfs f2, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E83F88: D05E0098  stfs f2, 0x98(r30)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82E83F8C: 387E00A0  addi r3, r30, 0xa0
	ctx.r[3].s64 = ctx.r[30].s64 + 160;
	// 82E83F90: 4BFFF3E1  bl 0x82e83370
	ctx.lr = 0x82E83F94;
	sub_82E83370(ctx, base);
	// 82E83F94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E83F98: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E83F9C: 388B0007  addi r4, r11, 7
	ctx.r[4].s64 = ctx.r[11].s64 + 7;
	// 82E83FA0: 548B003A  rlwinm r11, r4, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83FA4: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E83FA8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E83FAC: 546A003A  rlwinm r10, r3, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83FB0: 7CCB382E  lwzx r6, r11, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E83FB4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E83FB8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E83FBC: 392A0007  addi r9, r10, 7
	ctx.r[9].s64 = ctx.r[10].s64 + 7;
	// 82E83FC0: 7C6A382E  lwzx r3, r10, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E83FC4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E83FC8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E83FCC: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E83FD0: 506B843E  rlwimi r11, r3, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83FD4: 506A801E  rlwimi r10, r3, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83FD8: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E83FDC: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E83FE0: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E83FE4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E83FE8: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E83FEC: 50C5843E  rlwimi r5, r6, 0x10, 0x10, 0x1f
	ctx.r[5].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[5].u64 & 0xFFFFFFFFFFFF0000);
	// 82E83FF0: 50C4801E  rlwimi r4, r6, 0x10, 0, 0xf
	ctx.r[4].u64 = (((ctx.r[6].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[4].u64 & 0xFFFFFFFF0000FFFF);
	// 82E83FF4: 90FE00CC  stw r7, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[7].u32 ) };
	// 82E83FF8: 54A6C43E  rlwinm r6, r5, 0x18, 0x10, 0x1f
	ctx.r[6].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82E83FFC: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84000: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84004: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E84008: 7D6B282E  lwzx r11, r11, r5
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E8400C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E84010: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E84014: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E84018: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E8401C: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E84020: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E84024: 5465003A  rlwinm r5, r3, 0, 0, 0x1d
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84028: 7D033B78  or r3, r8, r7
	ctx.r[3].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E8402C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82E84030: 5484401E  rlwinm r4, r4, 8, 0, 0xf
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x00FFFFFFu64;
	// 82E84034: 907E00AC  stw r3, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[3].u32 ) };
	// 82E84038: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8403C: 7CCA2378  or r10, r6, r4
	ctx.r[10].u64 = ctx.r[6].u64 | ctx.r[4].u64;
	// 82E84040: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84044: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E84048: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E8404C: 7D470034  cntlzw r7, r10
	ctx.r[7].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82E84050: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E84054: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E84058: 506B843E  rlwimi r11, r3, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E8405C: 506A801E  rlwimi r10, r3, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[3].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E84060: 5526003A  rlwinm r6, r9, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84064: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E84068: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E8406C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E84070: 54E5DFFE  rlwinm r5, r7, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82E84074: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E84078: 68A40001  xori r4, r5, 1
	ctx.r[4].u64 = ctx.r[5].u64 ^ 1;
	// 82E8407C: 90FE00B0  stw r7, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[7].u32 ) };
	// 82E84080: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84084: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84088: 7CAB302E  lwzx r5, r11, r6
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 82E8408C: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E84090: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82E84094: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E84098: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82E8409C: 50AA843E  rlwimi r10, r5, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E840A0: 50A9801E  rlwimi r9, r5, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E840A4: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E840A8: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E840AC: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E840B0: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E840B4: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E840B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E840BC: 90BE00B4  stw r5, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82E840C0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E840C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E840C8: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E840CC: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E840D0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E840D4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E840D8: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E840DC: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E840E0: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E840E4: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E840E8: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E840EC: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E840F0: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E840F4: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82E840F8: C0210060  lfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E840FC: D03E0088  stfs f1, 0x88(r30)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82E84100: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84104: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84108: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E8410C: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E84110: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E84114: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E84118: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E8411C: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E84120: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E84124: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E84128: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8412C: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E84130: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E84134: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82E84138: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E8413C: D01E008C  stfs f0, 0x8c(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82E84140: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84144: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84148: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E8414C: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E84150: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E84154: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E84158: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E8415C: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E84160: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E84164: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E84168: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8416C: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E84170: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E84174: 90BE00B8  stw r5, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[5].u32 ) };
	// 82E84178: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8417C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84180: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E84184: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E84188: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E8418C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82E84190: 516A843E  rlwimi r10, r11, 0x10, 0x10, 0x1f
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[10].u64 & 0xFFFFFFFFFFFF0000);
	// 82E84194: 5169801E  rlwimi r9, r11, 0x10, 0, 0xf
	ctx.r[9].u64 = (((ctx.r[11].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[9].u64 & 0xFFFFFFFF0000FFFF);
	// 82E84198: 5548C43E  rlwinm r8, r10, 0x18, 0x10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82E8419C: 5527401E  rlwinm r7, r9, 8, 0, 0xf
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00FFFFFFu64;
	// 82E841A0: 5466003A  rlwinm r6, r3, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E841A4: 7D053B78  or r5, r8, r7
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82E841A8: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82E841AC: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82E841B0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E841B4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E841B8: 386B0007  addi r3, r11, 7
	ctx.r[3].s64 = ctx.r[11].s64 + 7;
	// 82E841BC: 546A003A  rlwinm r10, r3, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 82E841C0: 7D2B202E  lwzx r9, r11, r4
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E841C4: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82E841C8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E841CC: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82E841D0: 5128843E  rlwimi r8, r9, 0x10, 0x10, 0x1f
	ctx.r[8].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[8].u64 & 0xFFFFFFFFFFFF0000);
	// 82E841D4: 5127801E  rlwimi r7, r9, 0x10, 0, 0xf
	ctx.r[7].u64 = (((ctx.r[9].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[7].u64 & 0xFFFFFFFF0000FFFF);
	// 82E841D8: 5506C43E  rlwinm r6, r8, 0x18, 0x10, 0x1f
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82E841DC: 54E5401E  rlwinm r5, r7, 8, 0, 0xf
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00FFFFFFu64;
	// 82E841E0: 7CCB2B78  or r11, r6, r5
	ctx.r[11].u64 = ctx.r[6].u64 | ctx.r[5].u64;
	// 82E841E4: 917E00C8  stw r11, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82E841E8: 419A000C  beq cr6, 0x82e841f4
	if ctx.cr[6].eq {
	pc = 0x82E841F4; continue 'dispatch;
	}
	// 82E841EC: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 82E841F0: 917E00C8  stw r11, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82E841F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E841F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E841FC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84200: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E84204: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E84208: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8420C: 7CEB502E  lwzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E84210: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E84214: 54E6463E  srwi r6, r7, 0x18
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shr(24);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E84218: 98DE00C4  stb r6, 0xc4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[6].u8 ) };
	// 82E8421C: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84220: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84224: 394B0007  addi r10, r11, 7
	ctx.r[10].s64 = ctx.r[11].s64 + 7;
	// 82E84228: 7D2B282E  lwzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E8422C: 5548003A  rlwinm r8, r10, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84230: 5527463E  srwi r7, r9, 0x18
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E84234: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82E84238: 98FE00C5  stb r7, 0xc5(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(197 as u32), ctx.r[7].u8 ) };
	// 82E8423C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84240: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E84244: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84248: 54CA003A  rlwinm r10, r6, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8424C: 7D2B282E  lwzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82E84250: 5528463E  srwi r8, r9, 0x18
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shr(24);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E84254: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E84258: 991E00C6  stb r8, 0xc6(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(198 as u32), ctx.r[8].u8 ) };
	// 82E8425C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84260: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84264: 38CB0007  addi r6, r11, 7
	ctx.r[6].s64 = ctx.r[11].s64 + 7;
	// 82E84268: 7CAB382E  lwzx r5, r11, r7
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82E8426C: 54CB003A  rlwinm r11, r6, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84270: 54AA463E  srwi r10, r5, 0x18
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shr(24);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E84274: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E84278: 995E00C7  stb r10, 0xc7(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(199 as u32), ctx.r[10].u8 ) };
	// 82E8427C: 4BFFE3C5  bl 0x82e82640
	ctx.lr = 0x82E84280;
	sub_82E82640(ctx, base);
	// 82E84280: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E84284: 397E0050  addi r11, r30, 0x50
	ctx.r[11].s64 = ctx.r[30].s64 + 80;
	// 82E84288: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E8428C: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E845E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E845E8 size=1220
    let mut pc: u32 = 0x82E845E8;
    'dispatch: loop {
        match pc {
            0x82E845E8 => {
    //   block [0x82E845E8..0x82E84AAC)
	// 82E845E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E845EC: 48323B69  bl 0x831a8154
	ctx.lr = 0x82E845F0;
	sub_831A8130(ctx, base);
	// 82E845F0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E845F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E845F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E845FC: 4BF7A26D  bl 0x82dfe868
	ctx.lr = 0x82E84600;
	sub_82DFE868(ctx, base);
	// 82E84600: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E84604: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E84608: 409A049C  bne cr6, 0x82e84aa4
	if !ctx.cr[6].eq {
	pc = 0x82E84AA4; continue 'dispatch;
	}
	// 82E8460C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82E84610: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E84614: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E84618: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82E8461C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E84620: 4BFFE301  bl 0x82e82920
	ctx.lr = 0x82E84624;
	sub_82E82920(ctx, base);
	// 82E84624: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E84628: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E8462C: 80A100A0  lwz r5, 0xa0(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E84630: 4BFFE4E9  bl 0x82e82b18
	ctx.lr = 0x82E84634;
	sub_82E82B18(ctx, base);
	// 82E84634: 3B1F0060  addi r24, r31, 0x60
	ctx.r[24].s64 = ctx.r[31].s64 + 96;
	// 82E84638: 808100A4  lwz r4, 0xa4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E8463C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E84640: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E84644: 4BFFF235  bl 0x82e83878
	ctx.lr = 0x82E84648;
	sub_82E83878(ctx, base);
	// 82E84648: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E8464C: 40990448  ble cr6, 0x82e84a94
	if !ctx.cr[6].gt {
	pc = 0x82E84A94; continue 'dispatch;
	}
	// 82E84650: 83A10064  lwz r29, 0x64(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E84654: 7FF7FB78  mr r23, r31
	ctx.r[23].u64 = ctx.r[31].u64;
	// 82E84658: 83C10060  lwz r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E8465C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E84660: 4BF6EA91  bl 0x82df30f0
	ctx.lr = 0x82E84664;
	sub_82DF30F0(ctx, base);
	// 82E84664: 7D7DF214  add r11, r29, r30
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82E84668: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E8466C: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82E84670: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E84674: 7FFDF0AE  lbzx r31, r29, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E84678: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E8467C: 4BF70035  bl 0x82df46b0
	ctx.lr = 0x82E84680;
	sub_82DF46B0(ctx, base);
	// 82E84680: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82E84684: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 82E84688: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E8468C: 554B003A  rlwinm r11, r10, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84690: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E84694: 5528003A  rlwinm r8, r9, 0, 0, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84698: 7CEBF02E  lwzx r7, r11, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E8469C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82E846A0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E846A4: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82E846A8: 50E6843E  rlwimi r6, r7, 0x10, 0x10, 0x1f
	ctx.r[6].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[6].u64 & 0xFFFFFFFFFFFF0000);
	// 82E846AC: 50E5801E  rlwimi r5, r7, 0x10, 0, 0xf
	ctx.r[5].u64 = (((ctx.r[7].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[5].u64 & 0xFFFFFFFF0000FFFF);
	// 82E846B0: 54C4C43E  rlwinm r4, r6, 0x18, 0x10, 0x1f
	ctx.r[4].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82E846B4: 54AB401E  rlwinm r11, r5, 8, 0, 0xf
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x00FFFFFFu64;
	// 82E846B8: 7C9A5B78  or r26, r4, r11
	ctx.r[26].u64 = ctx.r[4].u64 | ctx.r[11].u64;
	// 82E846BC: 4B43C27D  bl 0x822c0938
	ctx.lr = 0x82E846C0;
	sub_822C0938(ctx, base);
	// 82E846C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E846C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E846C8: 419A0030  beq cr6, 0x82e846f8
	if ctx.cr[6].eq {
	pc = 0x82E846F8; continue 'dispatch;
	}
	// 82E846CC: 4B94CEDD  bl 0x827d15a8
	ctx.lr = 0x82E846D0;
	sub_827D15A8(ctx, base);
	// 82E846D0: 397F0080  addi r11, r31, 0x80
	ctx.r[11].s64 = ctx.r[31].s64 + 128;
	// 82E846D4: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82E846D8: 4BF6EA19  bl 0x82df30f0
	ctx.lr = 0x82E846DC;
	sub_82DF30F0(ctx, base);
	// 82E846DC: 933F00D0  stw r25, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[25].u32 ) };
	// 82E846E0: 933F00D4  stw r25, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[25].u32 ) };
	// 82E846E4: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 82E846E8: 933F00DC  stw r25, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[25].u32 ) };
	// 82E846EC: 933F00E0  stw r25, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[25].u32 ) };
	// 82E846F0: 933F00E4  stw r25, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[25].u32 ) };
	// 82E846F4: 48000008  b 0x82e846fc
	pc = 0x82E846FC; continue 'dispatch;
	// 82E846F8: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E846FC: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 82E84700: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E84704: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82E84708: 4B94EE59  bl 0x827d3560
	ctx.lr = 0x82E8470C;
	sub_827D3560(ctx, base);
	// 82E8470C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E84710: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E84714: 3861007C  addi r3, r1, 0x7c
	ctx.r[3].s64 = ctx.r[1].s64 + 124;
	// 82E84718: 4B43B8E9  bl 0x822c0000
	ctx.lr = 0x82E8471C;
	sub_822C0000(ctx, base);
	// 82E8471C: 83E10078  lwz r31, 0x78(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E84720: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E84724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E84728: 4BFFE721  bl 0x82e82e48
	ctx.lr = 0x82E8472C;
	sub_82E82E48(ctx, base);
	// 82E8472C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82E84730: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E84734: 4BFFEA2D  bl 0x82e83160
	ctx.lr = 0x82E84738;
	sub_82E83160(ctx, base);
	// 82E84738: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E8473C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E84740: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E84744: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 82E84748: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E8474C: 3BDF0080  addi r30, r31, 0x80
	ctx.r[30].s64 = ctx.r[31].s64 + 128;
	// 82E84750: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E84754: 552B003A  rlwinm r11, r9, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E84758: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82E8475C: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82E84760: 5107843E  rlwimi r7, r8, 0x10, 0x10, 0x1f
	ctx.r[7].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[7].u64 & 0xFFFFFFFFFFFF0000);
	// 82E84764: 5106801E  rlwimi r6, r8, 0x10, 0, 0xf
	ctx.r[6].u64 = (((ctx.r[8].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[6].u64 & 0xFFFFFFFF0000FFFF);
	// 82E84768: 54E5C43E  rlwinm r5, r7, 0x18, 0x10, 0x1f
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82E8476C: 54C9401E  rlwinm r9, r6, 8, 0, 0xf
	ctx.r[9].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82E84770: 390B0007  addi r8, r11, 7
	ctx.r[8].s64 = ctx.r[11].s64 + 7;
	// 82E84774: 7CA74B78  or r7, r5, r9
	ctx.r[7].u64 = ctx.r[5].u64 | ctx.r[9].u64;
	// 82E84778: 5506003A  rlwinm r6, r8, 0, 0, 0x1d
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E8477C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82E84780: C001005C  lfs f0, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E84784: D01F00BC  stfs f0, 0xbc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82E84788: 90C10064  stw r6, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[6].u32 ) };
	// 82E8478C: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E84790: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E84794: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E84798: 50AB843E  rlwimi r11, r5, 0x10, 0x10, 0x1f
	ctx.r[11].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x000000000000FFFF) | (ctx.r[11].u64 & 0xFFFFFFFFFFFF0000);
	// 82E8479C: 50AA801E  rlwimi r10, r5, 0x10, 0, 0xf
	ctx.r[10].u64 = (((ctx.r[5].u32).rotate_left(16) as u64) & 0x00000000FFFF0000) | (ctx.r[10].u64 & 0xFFFFFFFF0000FFFF);
	// 82E847A0: 5569C43E  rlwinm r9, r11, 0x18, 0x10, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E847A4: 5548401E  rlwinm r8, r10, 8, 0, 0xf
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00FFFFFFu64;
	// 82E847A8: 7D274378  or r7, r9, r8
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82E847AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82E847B0: C1A1005C  lfs f13, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E847B4: D1BF00B8  stfs f13, 0xb8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 82E847B8: 88DF0091  lbz r6, 0x91(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(145 as u32) ) } as u64;
	// 82E847BC: 98DF00B0  stb r6, 0xb0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[6].u8 ) };
	// 82E847C0: C19F0098  lfs f12, 0x98(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E847C4: D19F00B4  stfs f12, 0xb4(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 82E847C8: 80BF00A8  lwz r5, 0xa8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E847CC: 90BF00C0  stw r5, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[5].u32 ) };
	// 82E847D0: 4BFFDE71  bl 0x82e82640
	ctx.lr = 0x82E847D4;
	sub_82E82640(ctx, base);
	// 82E847D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E847D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E847DC: 3B9F00AC  addi r28, r31, 0xac
	ctx.r[28].s64 = ctx.r[31].s64 + 172;
	// 82E847E0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84AB0 size=8
    let mut pc: u32 = 0x82E84AB0;
    'dispatch: loop {
        match pc {
            0x82E84AB0 => {
    //   block [0x82E84AB0..0x82E84AB8)
	// 82E84AB0: 9083007C  stw r4, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 82E84AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84AB8 size=8
    let mut pc: u32 = 0x82E84AB8;
    'dispatch: loop {
        match pc {
            0x82E84AB8 => {
    //   block [0x82E84AB8..0x82E84AC0)
	// 82E84AB8: 8063007C  lwz r3, 0x7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E84ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E84AC0 size=8
    let mut pc: u32 = 0x82E84AC0;
    'dispatch: loop {
        match pc {
            0x82E84AC0 => {
    //   block [0x82E84AC0..0x82E84AC8)
	// 82E84AC0: C0230078  lfs f1, 0x78(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E84AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E84AC8 size=8
    let mut pc: u32 = 0x82E84AC8;
    'dispatch: loop {
        match pc {
            0x82E84AC8 => {
    //   block [0x82E84AC8..0x82E84AD0)
	// 82E84AC8: D0230078  stfs f1, 0x78(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E84ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84AD0 size=8
    let mut pc: u32 = 0x82E84AD0;
    'dispatch: loop {
        match pc {
            0x82E84AD0 => {
    //   block [0x82E84AD0..0x82E84AD8)
	// 82E84AD0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E84AD4: 4BFF9004  b 0x82e7dad8
	sub_82E7DAD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84AD8 size=12
    let mut pc: u32 = 0x82E84AD8;
    'dispatch: loop {
        match pc {
            0x82E84AD8 => {
    //   block [0x82E84AD8..0x82E84AE4)
	// 82E84AD8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E84ADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84AE0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84AE4 size=8
    let mut pc: u32 = 0x82E84AE4;
    'dispatch: loop {
        match pc {
            0x82E84AE4 => {
    //   block [0x82E84AE4..0x82E84AEC)
	// 82E84AE4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E84AE8: 4BFF9238  b 0x82e7dd20
	sub_82E7DD20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84AEC size=4
    let mut pc: u32 = 0x82E84AEC;
    'dispatch: loop {
        match pc {
            0x82E84AEC => {
    //   block [0x82E84AEC..0x82E84AF0)
	// 82E84AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84AF0 size=136
    let mut pc: u32 = 0x82E84AF0;
    'dispatch: loop {
        match pc {
            0x82E84AF0 => {
    //   block [0x82E84AF0..0x82E84B78)
	// 82E84AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E84AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E84AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E84B00: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E84B04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84B08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E84B0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E84B10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E84B14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84B18: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E84B1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E84B20: 4E800421  bctrl
	ctx.lr = 0x82E84B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84B24: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84B28: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E84B2C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E84B30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E84B34: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E84B38: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E84B3C: 4E800421  bctrl
	ctx.lr = 0x82E84B40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84B40: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84B44: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E84B48: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E84B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E84B50: 80C7001C  lwz r6, 0x1c(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E84B54: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82E84B58: 4E800421  bctrl
	ctx.lr = 0x82E84B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84B5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E84B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84B68: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E84B6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E84B70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84B78 size=8
    let mut pc: u32 = 0x82E84B78;
    'dispatch: loop {
        match pc {
            0x82E84B78 => {
    //   block [0x82E84B78..0x82E84B80)
	// 82E84B78: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82E84B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84B80 size=140
    let mut pc: u32 = 0x82E84B80;
    'dispatch: loop {
        match pc {
            0x82E84B80 => {
    //   block [0x82E84B80..0x82E84C0C)
	// 82E84B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E84B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E84B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84B90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E84B94: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84B98: 4BFF8F29  bl 0x82e7dac0
	ctx.lr = 0x82E84B9C;
	sub_82E7DAC0(ctx, base);
	// 82E84B9C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82E84BA0: 409A001C  bne cr6, 0x82e84bbc
	if !ctx.cr[6].eq {
	pc = 0x82E84BBC; continue 'dispatch;
	}
	// 82E84BA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E84BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84BB8: 4E800020  blr
	return;
	// 82E84BBC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84BC0: 4BFF8F01  bl 0x82e7dac0
	ctx.lr = 0x82E84BC4;
	sub_82E7DAC0(ctx, base);
	// 82E84BC4: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82E84BC8: 419A002C  beq cr6, 0x82e84bf4
	if ctx.cr[6].eq {
	pc = 0x82E84BF4; continue 'dispatch;
	}
	// 82E84BCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84BD0: 814B019C  lwz r10, 0x19c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 82E84BD4: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 82E84BD8: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82E84BDC: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E84BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84BF0: 4E800020  blr
	return;
	// 82E84BF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E84BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84C10 size=140
    let mut pc: u32 = 0x82E84C10;
    'dispatch: loop {
        match pc {
            0x82E84C10 => {
    //   block [0x82E84C10..0x82E84C9C)
	// 82E84C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E84C18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E84C1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84C20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E84C24: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84C28: 4BFF8E99  bl 0x82e7dac0
	ctx.lr = 0x82E84C2C;
	sub_82E7DAC0(ctx, base);
	// 82E84C2C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82E84C30: 409A001C  bne cr6, 0x82e84c4c
	if !ctx.cr[6].eq {
	pc = 0x82E84C4C; continue 'dispatch;
	}
	// 82E84C34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E84C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84C48: 4E800020  blr
	return;
	// 82E84C4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84C50: 4BFF8E71  bl 0x82e7dac0
	ctx.lr = 0x82E84C54;
	sub_82E7DAC0(ctx, base);
	// 82E84C54: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82E84C58: 419A002C  beq cr6, 0x82e84c84
	if ctx.cr[6].eq {
	pc = 0x82E84C84; continue 'dispatch;
	}
	// 82E84C5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84C60: 814B019C  lwz r10, 0x19c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 82E84C64: 392A0000  addi r9, r10, 0
	ctx.r[9].s64 = ctx.r[10].s64 + 0;
	// 82E84C68: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82E84C6C: 5503DFFE  rlwinm r3, r8, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E84C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84C7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84C80: 4E800020  blr
	return;
	// 82E84C84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E84C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84CA0 size=308
    let mut pc: u32 = 0x82E84CA0;
    'dispatch: loop {
        match pc {
            0x82E84CA0 => {
    //   block [0x82E84CA0..0x82E84D04)
	// 82E84CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84CA4: 483234C1  bl 0x831a8164
	ctx.lr = 0x82E84CA8;
	sub_831A8130(ctx, base);
	// 82E84CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84CAC: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82E84CB0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E84CB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E84CB8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E84CBC: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82E84CC0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84CC4: 4BFF8DFD  bl 0x82e7dac0
	ctx.lr = 0x82E84CC8;
	sub_82E7DAC0(ctx, base);
	// 82E84CC8: 2B030006  cmplwi cr6, r3, 6
	ctx.cr[6].compare_u32(ctx.r[3].u32, 6 as u32, &mut ctx.xer);
	// 82E84CCC: 419900FC  bgt cr6, 0x82e84dc8
	if ctx.cr[6].gt {
	pc = 0x82E84DC8; continue 'dispatch;
	}
	// 82E84CD0: 3D8082E8  lis r12, -0x7d18
	ctx.r[12].s64 = -2098724864;
	// 82E84CD4: 398C4CE8  addi r12, r12, 0x4ce8
	ctx.r[12].s64 = ctx.r[12].s64 + 19688;
	// 82E84CD8: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E84CDC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82E84CE0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82E84CE4: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x82E84D94; continue 'dispatch;
		},
		1 => {
	pc = 0x82E84DC8; continue 'dispatch;
		},
		2 => {
	pc = 0x82E84D04; continue 'dispatch;
		},
		3 => {
	pc = 0x82E84DC8; continue 'dispatch;
		},
		4 => {
	pc = 0x82E84D2C; continue 'dispatch;
		},
		5 => {
	pc = 0x82E84D60; continue 'dispatch;
		},
		6 => {
	pc = 0x82E84D94; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82E84CE8: 82E84D94  lwz r23, 0x4d94(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19860 as u32) ) } as u64;
	// 82E84CEC: 82E84DC8  lwz r23, 0x4dc8(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19912 as u32) ) } as u64;
	// 82E84CF0: 82E84D04  lwz r23, 0x4d04(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19716 as u32) ) } as u64;
	// 82E84CF4: 82E84DC8  lwz r23, 0x4dc8(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19912 as u32) ) } as u64;
	// 82E84CF8: 82E84D2C  lwz r23, 0x4d2c(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19756 as u32) ) } as u64;
	// 82E84CFC: 82E84D60  lwz r23, 0x4d60(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19808 as u32) ) } as u64;
	// 82E84D00: 82E84D94  lwz r23, 0x4d94(r8)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(19860 as u32) ) } as u64;
            }
            0x82E84D04 => {
    //   block [0x82E84D04..0x82E84D2C)
	// 82E84D04: 807B001C  lwz r3, 0x1c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E84D08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84D0C: 419A00BC  beq cr6, 0x82e84dc8
	if ctx.cr[6].eq {
	pc = 0x82E84DC8; continue 'dispatch;
	}
	// 82E84D10: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E84D14: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E84D18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E84D1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E84D20: 48005A81  bl 0x82e8a7a0
	ctx.lr = 0x82E84D24;
	sub_82E8A7A0(ctx, base);
	// 82E84D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E84D28: 4832348C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            0x82E84D2C => {
    //   block [0x82E84D2C..0x82E84D60)
	// 82E84D2C: 807B0014  lwz r3, 0x14(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E84D30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84D34: 419A0094  beq cr6, 0x82e84dc8
	if ctx.cr[6].eq {
	pc = 0x82E84DC8; continue 'dispatch;
	}
	// 82E84D38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84D3C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E84D40: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E84D44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E84D48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E84D4C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E84D50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E84D54: 4E800421  bctrl
	ctx.lr = 0x82E84D58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E84D5C: 48323458  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            0x82E84D60 => {
    //   block [0x82E84D60..0x82E84D94)
	// 82E84D60: 807B0018  lwz r3, 0x18(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E84D64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84D68: 419A0060  beq cr6, 0x82e84dc8
	if ctx.cr[6].eq {
	pc = 0x82E84DC8; continue 'dispatch;
	}
	// 82E84D6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84D70: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E84D74: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E84D78: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E84D7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E84D80: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E84D84: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E84D88: 4E800421  bctrl
	ctx.lr = 0x82E84D8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E84D90: 48323424  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            0x82E84D94 => {
    //   block [0x82E84D94..0x82E84DC8)
	// 82E84D94: 807B0014  lwz r3, 0x14(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E84D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84D9C: 419A002C  beq cr6, 0x82e84dc8
	if ctx.cr[6].eq {
	pc = 0x82E84DC8; continue 'dispatch;
	}
	// 82E84DA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84DA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E84DA8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E84DAC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E84DB0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E84DB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E84DB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E84DBC: 4E800421  bctrl
	ctx.lr = 0x82E84DC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E84DC4: 483233F0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            0x82E84DC8 => {
    //   block [0x82E84DC8..0x82E84DD4)
	// 82E84DC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E84DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E84DD0: 483233E4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84DD8 size=196
    let mut pc: u32 = 0x82E84DD8;
    'dispatch: loop {
        match pc {
            0x82E84DD8 => {
    //   block [0x82E84DD8..0x82E84E9C)
	// 82E84DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E84DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E84DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E84DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84DEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E84DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E84DF4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E84DF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E84DFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E84E00: 4B43BB39  bl 0x822c0938
	ctx.lr = 0x82E84E04;
	sub_822C0938(ctx, base);
	// 82E84E04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84E08: 419A0028  beq cr6, 0x82e84e30
	if ctx.cr[6].eq {
	pc = 0x82E84E30; continue 'dispatch;
	}
	// 82E84E0C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E84E10: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E84E14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E84E18: 392BE180  addi r9, r11, -0x1e80
	ctx.r[9].s64 = ctx.r[11].s64 + -7808;
	// 82E84E1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E84E20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E84E24: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E84E28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E84E2C: 48000008  b 0x82e84e34
	pc = 0x82E84E34; continue 'dispatch;
	// 82E84E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E84E34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E84E38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E84E3C: 409A0044  bne cr6, 0x82e84e80
	if !ctx.cr[6].eq {
	pc = 0x82E84E80; continue 'dispatch;
	}
	// 82E84E40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E84E44: 419A001C  beq cr6, 0x82e84e60
	if ctx.cr[6].eq {
	pc = 0x82E84E60; continue 'dispatch;
	}
	// 82E84E48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84E4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E84E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E84E54: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84E58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E84E5C: 4E800421  bctrl
	ctx.lr = 0x82E84E60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E84E60: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E84E64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E84E68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E84E6C: 392A0828  addi r9, r10, 0x828
	ctx.r[9].s64 = ctx.r[10].s64 + 2088;
	// 82E84E70: 816BBAF4  lwz r11, -0x450c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17676 as u32) ) } as u64;
	// 82E84E74: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E84E78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E84E7C: 4B43B185  bl 0x822c0000
	ctx.lr = 0x82E84E80;
	sub_822C0000(ctx, base);
	// 82E84E80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E84E84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E84E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84E90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E84E94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84EA0 size=144
    let mut pc: u32 = 0x82E84EA0;
    'dispatch: loop {
        match pc {
            0x82E84EA0 => {
    //   block [0x82E84EA0..0x82E84F30)
	// 82E84EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84EA4: 483232C5  bl 0x831a8168
	ctx.lr = 0x82E84EA8;
	sub_831A8130(ctx, base);
	// 82E84EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84EAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E84EB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E84EB4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E84EB8: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E84EBC: 419A0068  beq cr6, 0x82e84f24
	if ctx.cr[6].eq {
	pc = 0x82E84F24; continue 'dispatch;
	}
	// 82E84EC0: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82E84EC4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82E84EC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84ECC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E84ED0: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84ED4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84ED8: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E84EDC: 419A0040  beq cr6, 0x82e84f1c
	if ctx.cr[6].eq {
	pc = 0x82E84F1C; continue 'dispatch;
	}
	// 82E84EE0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E84EE4: 419A0024  beq cr6, 0x82e84f08
	if ctx.cr[6].eq {
	pc = 0x82E84F08; continue 'dispatch;
	}
	// 82E84EE8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82E84EEC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E84EF0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E84EF4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E84EF8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E84EFC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E84F00: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E84F04: 4082FFE8  bne 0x82e84eec
	if !ctx.cr[0].eq {
	pc = 0x82E84EEC; continue 'dispatch;
	}
	// 82E84F08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E84F0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E84F10: 419A0008  beq cr6, 0x82e84f18
	if ctx.cr[6].eq {
	pc = 0x82E84F18; continue 'dispatch;
	}
	// 82E84F14: 4B43B97D  bl 0x822c0890
	ctx.lr = 0x82E84F18;
	sub_822C0890(ctx, base);
	// 82E84F18: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E84F1C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E84F20: 409AFFA0  bne cr6, 0x82e84ec0
	if !ctx.cr[6].eq {
	pc = 0x82E84EC0; continue 'dispatch;
	}
	// 82E84F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E84F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E84F2C: 4832328C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84F30 size=72
    let mut pc: u32 = 0x82E84F30;
    'dispatch: loop {
        match pc {
            0x82E84F30 => {
    //   block [0x82E84F30..0x82E84F78)
	// 82E84F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E84F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E84F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E84F44: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E84F48: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82E84F4C: 392B087C  addi r9, r11, 0x87c
	ctx.r[9].s64 = ctx.r[11].s64 + 2172;
	// 82E84F50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E84F54: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E84F58: 419A000C  beq cr6, 0x82e84f64
	if ctx.cr[6].eq {
	pc = 0x82E84F64; continue 'dispatch;
	}
	// 82E84F5C: 4B43B30D  bl 0x822c0268
	ctx.lr = 0x82E84F60;
	sub_822C0268(ctx, base);
	// 82E84F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E84F64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E84F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E84F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E84F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E84F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84F78 size=16
    let mut pc: u32 = 0x82E84F78;
    'dispatch: loop {
        match pc {
            0x82E84F78 => {
    //   block [0x82E84F78..0x82E84F88)
	// 82E84F78: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E84F7C: 816B6950  lwz r11, 0x6950(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26960 as u32) ) } as u64;
	// 82E84F80: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E84F84: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84F88 size=16
    let mut pc: u32 = 0x82E84F88;
    'dispatch: loop {
        match pc {
            0x82E84F88 => {
    //   block [0x82E84F88..0x82E84F98)
	// 82E84F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84F8C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E84F90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E84F94: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E84F98 size=4
    let mut pc: u32 = 0x82E84F98;
    'dispatch: loop {
        match pc {
            0x82E84F98 => {
    //   block [0x82E84F98..0x82E84F9C)
	// 82E84F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E84FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E84FA0 size=220
    let mut pc: u32 = 0x82E84FA0;
    'dispatch: loop {
        match pc {
            0x82E84FA0 => {
    //   block [0x82E84FA0..0x82E8507C)
	// 82E84FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E84FA4: 483231B9  bl 0x831a815c
	ctx.lr = 0x82E84FA8;
	sub_831A8130(ctx, base);
	// 82E84FA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E84FAC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E84FB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E84FB4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E84FB8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E84FBC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E84FC0: 419A00B0  beq cr6, 0x82e85070
	if ctx.cr[6].eq {
	pc = 0x82E85070; continue 'dispatch;
	}
	// 82E84FC4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E84FC8: 4BF79819  bl 0x82dfe7e0
	ctx.lr = 0x82E84FCC;
	sub_82DFE7E0(ctx, base);
	// 82E84FCC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E84FD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E84FD4: 419A009C  beq cr6, 0x82e85070
	if ctx.cr[6].eq {
	pc = 0x82E85070; continue 'dispatch;
	}
	// 82E84FD8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E84FDC: 419A0094  beq cr6, 0x82e85070
	if ctx.cr[6].eq {
	pc = 0x82E85070; continue 'dispatch;
	}
	// 82E84FE0: 83FA0064  lwz r31, 0x64(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E84FE4: 3B3A0060  addi r25, r26, 0x60
	ctx.r[25].s64 = ctx.r[26].s64 + 96;
	// 82E84FE8: 817A0068  lwz r11, 0x68(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E84FEC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E84FF0: 419A0064  beq cr6, 0x82e85054
	if ctx.cr[6].eq {
	pc = 0x82E85054; continue 'dispatch;
	}
	// 82E84FF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E84FF8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E84FFC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85000: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E85004: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E85008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E8500C: 814B006C  lwz r10, 0x6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E85010: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E85014: 4E800421  bctrl
	ctx.lr = 0x82E85018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E85018: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8501C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E85020: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82E85024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85028: 551DDFFE  rlwinm r29, r8, 0x1b, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E8502C: 419A000C  beq cr6, 0x82e85038
	if ctx.cr[6].eq {
	pc = 0x82E85038; continue 'dispatch;
	}
	// 82E85030: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E85034: 4B43B85D  bl 0x822c0890
	ctx.lr = 0x82E85038;
	sub_822C0890(ctx, base);
	// 82E85038: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82E8503C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85040: 409A0030  bne cr6, 0x82e85070
	if !ctx.cr[6].eq {
	pc = 0x82E85070; continue 'dispatch;
	}
	// 82E85044: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E85048: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E8504C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E85050: 409AFFA4  bne cr6, 0x82e84ff4
	if !ctx.cr[6].eq {
	pc = 0x82E84FF4; continue 'dispatch;
	}
	// 82E85054: 38BA0010  addi r5, r26, 0x10
	ctx.r[5].s64 = ctx.r[26].s64 + 16;
	// 82E85058: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E8505C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E85060: 4BFFB9B1  bl 0x82e80a10
	ctx.lr = 0x82E85064;
	sub_82E80A10(ctx, base);
	// 82E85064: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E85068: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E8506C: 48323140  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 82E85070: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E85074: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E85078: 48323134  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85080 size=144
    let mut pc: u32 = 0x82E85080;
    'dispatch: loop {
        match pc {
            0x82E85080 => {
    //   block [0x82E85080..0x82E85110)
	// 82E85080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E85084: 483230E5  bl 0x831a8168
	ctx.lr = 0x82E85088;
	sub_831A8130(ctx, base);
	// 82E85088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8508C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E85090: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E85094: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E85098: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E8509C: 419A0068  beq cr6, 0x82e85104
	if ctx.cr[6].eq {
	pc = 0x82E85104; continue 'dispatch;
	}
	// 82E850A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E850A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E850A8: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E850AC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E850B0: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E850B4: 419A0040  beq cr6, 0x82e850f4
	if ctx.cr[6].eq {
	pc = 0x82E850F4; continue 'dispatch;
	}
	// 82E850B8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E850BC: 419A0024  beq cr6, 0x82e850e0
	if ctx.cr[6].eq {
	pc = 0x82E850E0; continue 'dispatch;
	}
	// 82E850C0: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82E850C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E850C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E850CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E850D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E850D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E850D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E850DC: 4082FFE8  bne 0x82e850c4
	if !ctx.cr[0].eq {
	pc = 0x82E850C4; continue 'dispatch;
	}
	// 82E850E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E850E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E850E8: 419A0008  beq cr6, 0x82e850f0
	if ctx.cr[6].eq {
	pc = 0x82E850F0; continue 'dispatch;
	}
	// 82E850EC: 4B43B7A5  bl 0x822c0890
	ctx.lr = 0x82E850F0;
	sub_822C0890(ctx, base);
	// 82E850F0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E850F4: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E850F8: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E850FC: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E85100: 409AFFA0  bne cr6, 0x82e850a0
	if !ctx.cr[6].eq {
	pc = 0x82E850A0; continue 'dispatch;
	}
	// 82E85104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E85108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E8510C: 483230AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85110 size=188
    let mut pc: u32 = 0x82E85110;
    'dispatch: loop {
        match pc {
            0x82E85110 => {
    //   block [0x82E85110..0x82E851CC)
	// 82E85110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E85114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E85118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E8511C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E85120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E85124: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E85128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E8512C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E85130: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E85134: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E85138: 4B43B801  bl 0x822c0938
	ctx.lr = 0x82E8513C;
	sub_822C0938(ctx, base);
	// 82E8513C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E85140: 419A0028  beq cr6, 0x82e85168
	if ctx.cr[6].eq {
	pc = 0x82E85168; continue 'dispatch;
	}
	// 82E85144: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E85148: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E8514C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E85150: 392BE0B8  addi r9, r11, -0x1f48
	ctx.r[9].s64 = ctx.r[11].s64 + -8008;
	// 82E85154: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E85158: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E8515C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E85160: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E85164: 48000008  b 0x82e8516c
	pc = 0x82E8516C; continue 'dispatch;
	// 82E85168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E8516C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E85170: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85174: 409A003C  bne cr6, 0x82e851b0
	if !ctx.cr[6].eq {
	pc = 0x82E851B0; continue 'dispatch;
	}
	// 82E85178: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E8517C: 419A0014  beq cr6, 0x82e85190
	if ctx.cr[6].eq {
	pc = 0x82E85190; continue 'dispatch;
	}
	// 82E85180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E85184: 4BFF8F45  bl 0x82e7e0c8
	ctx.lr = 0x82E85188;
	sub_82E7E0C8(ctx, base);
	// 82E85188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8518C: 4BF6D24D  bl 0x82df23d8
	ctx.lr = 0x82E85190;
	sub_82DF23D8(ctx, base);
	// 82E85190: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E85194: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E85198: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E8519C: 392A0828  addi r9, r10, 0x828
	ctx.r[9].s64 = ctx.r[10].s64 + 2088;
	// 82E851A0: 816BBAF4  lwz r11, -0x450c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17676 as u32) ) } as u64;
	// 82E851A4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E851A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E851AC: 4B43AE55  bl 0x822c0000
	ctx.lr = 0x82E851B0;
	sub_822C0000(ctx, base);
	// 82E851B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E851B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E851B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E851BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E851C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E851C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E851C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E851D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E851D0 size=64
    let mut pc: u32 = 0x82E851D0;
    'dispatch: loop {
        match pc {
            0x82E851D0 => {
    //   block [0x82E851D0..0x82E85210)
	// 82E851D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E851D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E851D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E851DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E851E0: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E851E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E851E8: 419A0014  beq cr6, 0x82e851fc
	if ctx.cr[6].eq {
	pc = 0x82E851FC; continue 'dispatch;
	}
	// 82E851EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E851F0: 4BFF8ED9  bl 0x82e7e0c8
	ctx.lr = 0x82E851F4;
	sub_82E7E0C8(ctx, base);
	// 82E851F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E851F8: 4BF6D1E1  bl 0x82df23d8
	ctx.lr = 0x82E851FC;
	sub_82DF23D8(ctx, base);
	// 82E851FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E85200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E85204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E85208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E8520C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85210 size=100
    let mut pc: u32 = 0x82E85210;
    'dispatch: loop {
        match pc {
            0x82E85210 => {
    //   block [0x82E85210..0x82E85274)
	// 82E85210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E85214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E85218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E8521C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E85220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E85224: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82E85228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E8522C: 38E8E0C8  addi r7, r8, -0x1f38
	ctx.r[7].s64 = ctx.r[8].s64 + -7992;
	// 82E85230: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82E85234: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E85238: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E8523C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E85240: 38850004  addi r4, r5, 4
	ctx.r[4].s64 = ctx.r[5].s64 + 4;
	// 82E85244: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E85248: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E8524C: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E85250: 80C50000  lwz r6, 0(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85254: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E85258: 4B43F209  bl 0x822c4460
	ctx.lr = 0x82E8525C;
	sub_822C4460(ctx, base);
	// 82E8525C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E85260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E85264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E85268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E8526C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E85270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85278 size=104
    let mut pc: u32 = 0x82E85278;
    'dispatch: loop {
        match pc {
            0x82E85278 => {
    //   block [0x82E85278..0x82E852E0)
	// 82E85278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8527C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E85280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E85284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E85288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8528C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E85290: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E85294: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E85298: 394BE0C8  addi r10, r11, -0x1f38
	ctx.r[10].s64 = ctx.r[11].s64 + -7992;
	// 82E8529C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E852A0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E852A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E852A8: 419A0008  beq cr6, 0x82e852b0
	if ctx.cr[6].eq {
	pc = 0x82E852B0; continue 'dispatch;
	}
	// 82E852AC: 4B43B5E5  bl 0x822c0890
	ctx.lr = 0x82E852B0;
	sub_822C0890(ctx, base);
	// 82E852B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82E852B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E852B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E852BC: 419A000C  beq cr6, 0x82e852c8
	if ctx.cr[6].eq {
	pc = 0x82E852C8; continue 'dispatch;
	}
	// 82E852C0: 4BF6D119  bl 0x82df23d8
	ctx.lr = 0x82E852C4;
	sub_82DF23D8(ctx, base);
	// 82E852C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E852C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E852CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E852D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E852D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E852D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E852DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E852E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E852E0 size=24
    let mut pc: u32 = 0x82E852E0;
    'dispatch: loop {
        match pc {
            0x82E852E0 => {
    //   block [0x82E852E0..0x82E852F8)
	// 82E852E0: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E852E4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E852E8: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E852EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E852F0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E852F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E852F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E852F8 size=36
    let mut pc: u32 = 0x82E852F8;
    'dispatch: loop {
        match pc {
            0x82E852F8 => {
    //   block [0x82E852F8..0x82E8531C)
	// 82E852F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E852FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E85300: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E85304: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E85308: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E8530C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E85310: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E85314: 4082FFE8  bne 0x82e852fc
	if !ctx.cr[0].eq {
	pc = 0x82E852FC; continue 'dispatch;
	}
	// 82E85318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E85320 size=28
    let mut pc: u32 = 0x82E85320;
    'dispatch: loop {
        match pc {
            0x82E85320 => {
    //   block [0x82E85320..0x82E8533C)
	// 82E85320: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85324: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E85328: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82E8532C: 394B0028  addi r10, r11, 0x28
	ctx.r[10].s64 = ctx.r[11].s64 + 40;
	// 82E85330: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E85334: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82E85338: 4B43F128  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E85340 size=24
    let mut pc: u32 = 0x82E85340;
    'dispatch: loop {
        match pc {
            0x82E85340 => {
    //   block [0x82E85340..0x82E85358)
	// 82E85340: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E85344: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E85348: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E8534C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85350: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E85354: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E85358 size=36
    let mut pc: u32 = 0x82E85358;
    'dispatch: loop {
        match pc {
            0x82E85358 => {
    //   block [0x82E85358..0x82E8537C)
	// 82E85358: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E8535C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E85360: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E85364: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E85368: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E8536C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E85370: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E85374: 4082FFE8  bne 0x82e8535c
	if !ctx.cr[0].eq {
	pc = 0x82E8535C; continue 'dispatch;
	}
	// 82E85378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E85380 size=28
    let mut pc: u32 = 0x82E85380;
    'dispatch: loop {
        match pc {
            0x82E85380 => {
    //   block [0x82E85380..0x82E8539C)
	// 82E85380: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85384: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E85388: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82E8538C: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 82E85390: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E85394: 912B0030  stw r9, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82E85398: 4B43F0C8  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E853A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E853A0 size=24
    let mut pc: u32 = 0x82E853A0;
    'dispatch: loop {
        match pc {
            0x82E853A0 => {
    //   block [0x82E853A0..0x82E853B8)
	// 82E853A0: 81640020  lwz r11, 0x20(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E853A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E853A8: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E853AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E853B0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E853B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E853B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E853B8 size=36
    let mut pc: u32 = 0x82E853B8;
    'dispatch: loop {
        match pc {
            0x82E853B8 => {
    //   block [0x82E853B8..0x82E853DC)
	// 82E853B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E853BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E853C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E853C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E853C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E853CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E853D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E853D4: 4082FFE8  bne 0x82e853bc
	if !ctx.cr[0].eq {
	pc = 0x82E853BC; continue 'dispatch;
	}
	// 82E853D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E853E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E853E0 size=28
    let mut pc: u32 = 0x82E853E0;
    'dispatch: loop {
        match pc {
            0x82E853E0 => {
    //   block [0x82E853E0..0x82E853FC)
	// 82E853E0: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E853E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E853E8: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82E853EC: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82E853F0: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E853F4: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E853F8: 4B43F068  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85400 size=524
    let mut pc: u32 = 0x82E85400;
    'dispatch: loop {
        match pc {
            0x82E85400 => {
    //   block [0x82E85400..0x82E8560C)
	// 82E85400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E85404: 48322D5D  bl 0x831a8160
	ctx.lr = 0x82E85408;
	sub_831A8130(ctx, base);
	// 82E85408: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E8540C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E85410: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E85414: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E85418: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E8541C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E85420: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85424: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E85428: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E8542C: 419A0148  beq cr6, 0x82e85574
	if ctx.cr[6].eq {
	pc = 0x82E85574; continue 'dispatch;
	}
	// 82E85430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E85434: 4BFBAAD5  bl 0x82e3ff08
	ctx.lr = 0x82E85438;
	sub_82E3FF08(ctx, base);
	// 82E85438: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E8543C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E85440: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E85444: 4BFFCFB5  bl 0x82e823f8
	ctx.lr = 0x82E85448;
	sub_82E823F8(ctx, base);
	// 82E85448: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E8544C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85450: 419A0114  beq cr6, 0x82e85564
	if ctx.cr[6].eq {
	pc = 0x82E85564; continue 'dispatch;
	}
	// 82E85454: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 82E85458: 4BF6CF69  bl 0x82df23c0
	ctx.lr = 0x82E8545C;
	sub_82DF23C0(ctx, base);
	// 82E8545C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E85460: 419A0014  beq cr6, 0x82e85474
	if ctx.cr[6].eq {
	pc = 0x82E85474; continue 'dispatch;
	}
	// 82E85464: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E85468: 4BFFB3E9  bl 0x82e80850
	ctx.lr = 0x82E8546C;
	sub_82E80850(ctx, base);
	// 82E8546C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E85470: 48000008  b 0x82e85478
	pc = 0x82E85478; continue 'dispatch;
	// 82E85474: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82E85478: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82E8547C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E85480: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E85484: 4BFFF955  bl 0x82e84dd8
	ctx.lr = 0x82E85488;
	sub_82E84DD8(ctx, base);
	// 82E85488: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E8548C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E85490: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E85494: 4B43AB6D  bl 0x822c0000
	ctx.lr = 0x82E85498;
	sub_822C0000(ctx, base);
	// 82E85498: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E8549C: 4BF79345  bl 0x82dfe7e0
	ctx.lr = 0x82E854A0;
	sub_82DFE7E0(ctx, base);
	// 82E854A0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E854A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E854A8: 419A00E0  beq cr6, 0x82e85588
	if ctx.cr[6].eq {
	pc = 0x82E85588; continue 'dispatch;
	}
	// 82E854AC: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E854B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E854B4: 419A00A0  beq cr6, 0x82e85554
	if ctx.cr[6].eq {
	pc = 0x82E85554; continue 'dispatch;
	}
	// 82E854B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E854BC: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E854C0: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E854C4: 4BFFFADD  bl 0x82e84fa0
	ctx.lr = 0x82E854C8;
	sub_82E84FA0(ctx, base);
	// 82E854C8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E854CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E854D0: 419A0084  beq cr6, 0x82e85554
	if ctx.cr[6].eq {
	pc = 0x82E85554; continue 'dispatch;
	}
	// 82E854D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E854D8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E854DC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E854E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E854E4: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E854E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E854EC: 4E800421  bctrl
	ctx.lr = 0x82E854F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E854F0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E854F4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E854F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E854FC: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E85500: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E85504: 419A0034  beq cr6, 0x82e85538
	if ctx.cr[6].eq {
	pc = 0x82E85538; continue 'dispatch;
	}
	// 82E85508: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E8550C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E85510: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E85514: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E85518: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E8551C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E85520: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E85524: 4082FFE8  bne 0x82e8550c
	if !ctx.cr[0].eq {
	pc = 0x82E8550C; continue 'dispatch;
	}
	// 82E85528: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E8552C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E85530: 419A0008  beq cr6, 0x82e85538
	if ctx.cr[6].eq {
	pc = 0x82E85538; continue 'dispatch;
	}
	// 82E85534: 4B43B35D  bl 0x822c0890
	ctx.lr = 0x82E85538;
	sub_822C0890(ctx, base);
	// 82E85538: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E8553C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E85540: 419A003C  beq cr6, 0x82e8557c
	if ctx.cr[6].eq {
	pc = 0x82E8557C; continue 'dispatch;
	}
	// 82E85544: 4B43B34D  bl 0x822c0890
	ctx.lr = 0x82E85548;
	sub_822C0890(ctx, base);
	// 82E85548: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E8554C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E85550: 48322C60  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E85554: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E85558: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E8555C: 419A0008  beq cr6, 0x82e85564
	if ctx.cr[6].eq {
	pc = 0x82E85564; continue 'dispatch;
	}
	// 82E85560: 4B43B331  bl 0x822c0890
	ctx.lr = 0x82E85564;
	sub_822C0890(ctx, base);
	// 82E85564: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E85568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E8556C: 419A0008  beq cr6, 0x82e85574
	if ctx.cr[6].eq {
	pc = 0x82E85574; continue 'dispatch;
	}
	// 82E85570: 4B43B321  bl 0x822c0890
	ctx.lr = 0x82E85574;
	sub_822C0890(ctx, base);
	// 82E85574: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E85578: 935D0004  stw r26, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82E8557C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E85580: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E85584: 48322C2C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E85588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8558C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E85590: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E85594: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E85598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E8559C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E855A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E855A4: 4E800421  bctrl
	ctx.lr = 0x82E855A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E855A8: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E855AC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E855B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E855B4: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E855B8: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E855BC: 419A0034  beq cr6, 0x82e855f0
	if ctx.cr[6].eq {
	pc = 0x82E855F0; continue 'dispatch;
	}
	// 82E855C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E855C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E855C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E855CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E855D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E855D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E855D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E855DC: 4082FFE8  bne 0x82e855c4
	if !ctx.cr[0].eq {
	pc = 0x82E855C4; continue 'dispatch;
	}
	// 82E855E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E855E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E855E8: 419A0008  beq cr6, 0x82e855f0
	if ctx.cr[6].eq {
	pc = 0x82E855F0; continue 'dispatch;
	}
	// 82E855EC: 4B43B2A5  bl 0x822c0890
	ctx.lr = 0x82E855F0;
	sub_822C0890(ctx, base);
	// 82E855F0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E855F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E855F8: 419AFF84  beq cr6, 0x82e8557c
	if ctx.cr[6].eq {
	pc = 0x82E8557C; continue 'dispatch;
	}
	// 82E855FC: 4B43B295  bl 0x822c0890
	ctx.lr = 0x82E85600;
	sub_822C0890(ctx, base);
	// 82E85600: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E85604: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E85608: 48322BA8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85610 size=228
    let mut pc: u32 = 0x82E85610;
    'dispatch: loop {
        match pc {
            0x82E85610 => {
    //   block [0x82E85610..0x82E856F4)
	// 82E85610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E85614: 48322B51  bl 0x831a8164
	ctx.lr = 0x82E85618;
	sub_831A8130(ctx, base);
	// 82E85618: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E8561C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E85620: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E85624: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E85628: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E8562C: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E85630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85634: 419A00B4  beq cr6, 0x82e856e8
	if ctx.cr[6].eq {
	pc = 0x82E856E8; continue 'dispatch;
	}
	// 82E85638: 839D003C  lwz r28, 0x3c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E8563C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E85640: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E85644: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85648: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E8564C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E85650: 419A0098  beq cr6, 0x82e856e8
	if ctx.cr[6].eq {
	pc = 0x82E856E8; continue 'dispatch;
	}
	// 82E85654: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85658: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82E8565C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E85660: 4BFFA0D9  bl 0x82e7f738
	ctx.lr = 0x82E85664;
	sub_82E7F738(ctx, base);
	// 82E85664: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E85668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8566C: 419A006C  beq cr6, 0x82e856d8
	if ctx.cr[6].eq {
	pc = 0x82E856D8; continue 'dispatch;
	}
	// 82E85670: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85674: 4BFFC8DD  bl 0x82e81f50
	ctx.lr = 0x82E85678;
	sub_82E81F50(ctx, base);
	// 82E85678: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E8567C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85680: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85684: 419A0044  beq cr6, 0x82e856c8
	if ctx.cr[6].eq {
	pc = 0x82E856C8; continue 'dispatch;
	}
	// 82E85688: 4BFF9D19  bl 0x82e7f3a0
	ctx.lr = 0x82E8568C;
	sub_82E7F3A0(ctx, base);
	// 82E8568C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E85690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85694: 409A0044  bne cr6, 0x82e856d8
	if !ctx.cr[6].eq {
	pc = 0x82E856D8; continue 'dispatch;
	}
	// 82E85698: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8569C: 4BFF9D4D  bl 0x82e7f3e8
	ctx.lr = 0x82E856A0;
	sub_82E7F3E8(ctx, base);
	// 82E856A0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E856A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E856A8: 409A0030  bne cr6, 0x82e856d8
	if !ctx.cr[6].eq {
	pc = 0x82E856D8; continue 'dispatch;
	}
	// 82E856AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E856B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E856B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E856B8: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E856BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E856C0: 4E800421  bctrl
	ctx.lr = 0x82E856C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E856C4: 48000014  b 0x82e856d8
	pc = 0x82E856D8; continue 'dispatch;
	// 82E856C8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E856CC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E856D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E856D4: 4BFFC54D  bl 0x82e81c20
	ctx.lr = 0x82E856D8;
	sub_82E81C20(ctx, base);
	// 82E856D8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E856DC: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E856E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E856E4: 409AFF70  bne cr6, 0x82e85654
	if !ctx.cr[6].eq {
	pc = 0x82E85654; continue 'dispatch;
	}
	// 82E856E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E856EC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E856F0: 48322AC4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E856F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E856F8 size=172
    let mut pc: u32 = 0x82E856F8;
    'dispatch: loop {
        match pc {
            0x82E856F8 => {
    //   block [0x82E856F8..0x82E857A4)
	// 82E856F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E856FC: 48322A69  bl 0x831a8164
	ctx.lr = 0x82E85700;
	sub_831A8130(ctx, base);
	// 82E85700: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E85704: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E85708: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E8570C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E85710: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E85714: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E85718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8571C: 419A007C  beq cr6, 0x82e85798
	if ctx.cr[6].eq {
	pc = 0x82E85798; continue 'dispatch;
	}
	// 82E85720: 839D0048  lwz r28, 0x48(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E85724: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E85728: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E8572C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85730: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E85734: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E85738: 419A0060  beq cr6, 0x82e85798
	if ctx.cr[6].eq {
	pc = 0x82E85798; continue 'dispatch;
	}
	// 82E8573C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85740: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82E85744: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E85748: 4BFFC809  bl 0x82e81f50
	ctx.lr = 0x82E8574C;
	sub_82E81F50(ctx, base);
	// 82E8574C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E85750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85754: 419A0020  beq cr6, 0x82e85774
	if ctx.cr[6].eq {
	pc = 0x82E85774; continue 'dispatch;
	}
	// 82E85758: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8575C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E85760: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E85764: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E85768: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E8576C: 4E800421  bctrl
	ctx.lr = 0x82E85770;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E85770: 48000018  b 0x82e85788
	pc = 0x82E85788; continue 'dispatch;
	// 82E85774: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E85778: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8577C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E85780: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E85784: 4BFFC49D  bl 0x82e81c20
	ctx.lr = 0x82E85788;
	sub_82E81C20(ctx, base);
	// 82E85788: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82E8578C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E85790: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E85794: 409AFFA8  bne cr6, 0x82e8573c
	if !ctx.cr[6].eq {
	pc = 0x82E8573C; continue 'dispatch;
	}
	// 82E85798: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E8579C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E857A0: 48322A14  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E857A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E857A8 size=188
    let mut pc: u32 = 0x82E857A8;
    'dispatch: loop {
        match pc {
            0x82E857A8 => {
    //   block [0x82E857A8..0x82E85864)
	// 82E857A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E857AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E857B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E857B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E857B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E857BC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E857C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E857C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E857C8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E857CC: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82E857D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E857D4: 4E800421  bctrl
	ctx.lr = 0x82E857D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E857D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E857DC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E857E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E857E4: 419A001C  beq cr6, 0x82e85800
	if ctx.cr[6].eq {
	pc = 0x82E85800; continue 'dispatch;
	}
	// 82E857E8: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E857EC: 7F1E4840  cmplw cr6, r30, r9
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E857F0: 419A0038  beq cr6, 0x82e85828
	if ctx.cr[6].eq {
	pc = 0x82E85828; continue 'dispatch;
	}
	// 82E857F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E857F8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E857FC: 409AFFEC  bne cr6, 0x82e857e8
	if !ctx.cr[6].eq {
	pc = 0x82E857E8; continue 'dispatch;
	}
	// 82E85800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E85804: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E85808: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E8580C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E85810: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E85814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E85818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E8581C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E85820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E85824: 4E800020  blr
	return;
	// 82E85828: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E8582C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E85830: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E85834: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85838: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E8583C: 419AFFD0  beq cr6, 0x82e8580c
	if ctx.cr[6].eq {
	pc = 0x82E8580C; continue 'dispatch;
	}
	// 82E85840: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E85844: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E85848: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8584C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E85850: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E85854: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E85858: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E8585C: 4082FFE8  bne 0x82e85844
	if !ctx.cr[0].eq {
	pc = 0x82E85844; continue 'dispatch;
	}
	// 82E85860: 4BFFFFAC  b 0x82e8580c
	pc = 0x82E8580C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E85868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E85868 size=376
    let mut pc: u32 = 0x82E85868;
    'dispatch: loop {
        match pc {
            0x82E85868 => {
    //   block [0x82E85868..0x82E859E0)
	// 82E85868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E8586C: 483228E9  bl 0x831a8154
	ctx.lr = 0x82E85870;
	sub_831A8130(ctx, base);
	// 82E85870: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E85874: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E85878: 386001A0  li r3, 0x1a0
	ctx.r[3].s64 = 416;
	// 82E8587C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E85880: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82E85884: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E85888: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E8588C: 4BF6CB35  bl 0x82df23c0
	ctx.lr = 0x82E85890;
	sub_82DF23C0(ctx, base);
	// 82E85890: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82E85894: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E85898: 419A0010  beq cr6, 0x82e858a8
	if ctx.cr[6].eq {
	pc = 0x82E858A8; continue 'dispatch;
	}
	// 82E8589C: 48004965  bl 0x82e8a200
	ctx.lr = 0x82E858A0;
	sub_82E8A200(ctx, base);
	// 82E858A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E858A4: 48000008  b 0x82e858ac
	pc = 0x82E858AC; continue 'dispatch;
	// 82E858A8: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 82E858AC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E858B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E858B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E858B8: 4BFFF521  bl 0x82e84dd8
	ctx.lr = 0x82E858BC;
	sub_82E84DD8(ctx, base);
	// 82E858BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E858C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E858C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E858C8: 4B43A739  bl 0x822c0000
	ctx.lr = 0x82E858CC;
	sub_822C0000(ctx, base);
	// 82E858CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E858D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E858D4: 419A00AC  beq cr6, 0x82e85980
	if ctx.cr[6].eq {
	pc = 0x82E85980; continue 'dispatch;
	}
	// 82E858D8: 835B00E0  lwz r26, 0xe0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(224 as u32) ) } as u64;
	// 82E858DC: 83FB00DC  lwz r31, 0xdc(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(220 as u32) ) } as u64;
	// 82E858E0: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E858E4: 419A0060  beq cr6, 0x82e85944
	if ctx.cr[6].eq {
	pc = 0x82E85944; continue 'dispatch;
	}
	// 82E858E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E858EC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E858F0: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E858F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E858F8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E858FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E85900: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E85904: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E85908: 4E800421  bctrl
	ctx.lr = 0x82E8590C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E8590C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E85910: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E85914: 7D280034  cntlzw r8, r9
	ctx.r[8].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82E85918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8591C: 551DDFFE  rlwinm r29, r8, 0x1b, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 82E85920: 419A000C  beq cr6, 0x82e8592c
	if ctx.cr[6].eq {
	pc = 0x82E8592C; continue 'dispatch;
	}
	// 82E85924: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E85928: 4B43AF69  bl 0x822c0890
	ctx.lr = 0x82E8592C;
	sub_822C0890(ctx, base);
	// 82E8592C: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82E85930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E85934: 409A00A0  bne cr6, 0x82e859d4
	if !ctx.cr[6].eq {
	pc = 0x82E859D4; continue 'dispatch;
	}
	// 82E85938: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E8593C: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E85940: 409AFFA8  bne cr6, 0x82e858e8
	if !ctx.cr[6].eq {
	pc = 0x82E858E8; continue 'dispatch;
	}
	// 82E85944: 38DB0080  addi r6, r27, 0x80
	ctx.r[6].s64 = ctx.r[27].s64 + 128;
	// 82E85948: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E8594C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E85950: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E85954: 480044F5  bl 0x82e89e48
	ctx.lr = 0x82E85958;
	sub_82E89E48(ctx, base);
	// 82E85958: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E8595C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E85960: 38BB00D0  addi r5, r27, 0xd0
	ctx.r[5].s64 = ctx.r[27].s64 + 208;
	// 82E85964: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E85968: 814B00A0  lwz r10, 0xa0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E8596C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E85970: 4E800421  bctrl
	ctx.lr = 0x82E85974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E85974: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E85978: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E8597C: 4BFFACFD  bl 0x82e80678
	ctx.lr = 0x82E85980;
	sub_82E80678(ctx, base);
	// 82E85980: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E85984: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E85988: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E8598C: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E85990: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E85994: 419A0034  beq cr6, 0x82e859c8
	if ctx.cr[6].eq {
	pc = 0x82E859C8; continue 'dispatch;
	}
	// 82E85998: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E8599C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E859A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E859A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E859A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E859AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E859B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E859B4: 4082FFE8  bne 0x82e8599c
	if !ctx.cr[0].eq {
	pc = 0x82E8599C; continue 'dispatch;
	}
	// 82E859B8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E859BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E859C0: 419A0008  beq cr6, 0x82e859c8
	if ctx.cr[6].eq {
	pc = 0x82E859C8; continue 'dispatch;
	}
	// 82E859C4: 4B43AECD  bl 0x822c0890
	ctx.lr = 0x82E859C8;
	sub_822C0890(ctx, base);
	// 82E859C8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E859CC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E859D0: 483227D4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 82E859D4: 92F90000  stw r23, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82E859D8: 92F90004  stw r23, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 82E859DC: 4BFFFFDC  b 0x82e859b8
	pc = 0x82E859B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


