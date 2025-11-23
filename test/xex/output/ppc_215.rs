pub fn sub_832621D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832621D0 size=96
    let mut pc: u32 = 0x832621D0;
    'dispatch: loop {
        match pc {
            0x832621D0 => {
    //   block [0x832621D0..0x83262230)
	// 832621D0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832621D4: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 832621D8: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 832621DC: 3D008210  lis r8, -0x7df0
	ctx.r[8].s64 = -2112880640;
	// 832621E0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832621E4: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832621E8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832621EC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832621F0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262230 size=96
    let mut pc: u32 = 0x83262230;
    'dispatch: loop {
        match pc {
            0x83262230 => {
    //   block [0x83262230..0x83262290)
	// 83262230: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262234: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262238: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8326223C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262240: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83262244: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262248: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 8326224C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262250: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 83262254: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262258: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8326225C: 3885B2C0  addi r4, r5, -0x4d40
	ctx.r[4].s64 = ctx.r[5].s64 + -19776;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262290 size=96
    let mut pc: u32 = 0x83262290;
    'dispatch: loop {
        match pc {
            0x83262290 => {
    //   block [0x83262290..0x832622F0)
	// 83262290: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262294: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83262298: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8326229C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832622A0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832622A4: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832622A8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832622AC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832622B0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832622F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832622F0 size=112
    let mut pc: u32 = 0x832622F0;
    'dispatch: loop {
        match pc {
            0x832622F0 => {
    //   block [0x832622F0..0x83262360)
	// 832622F0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832622F4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832622F8: 392BD074  addi r9, r11, -0x2f8c
	ctx.r[9].s64 = ctx.r[11].s64 + -12172;
	// 832622FC: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83262300: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83262304: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 83262308: C1ABD074  lfs f13, -0x2f8c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8326230C: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
	// 83262310: C009C410  lfs f0, -0x3bf0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15344 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262314: 3881FFF0  addi r4, r1, -0x10
	ctx.r[4].s64 = ctx.r[1].s64 + -16;
	// 83262318: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8326231C: 3C60834A  lis r3, -0x7cb6
	ctx.r[3].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262360 size=64
    let mut pc: u32 = 0x83262360;
    'dispatch: loop {
        match pc {
            0x83262360 => {
    //   block [0x83262360..0x832623A0)
	// 83262360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326236C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262374: 388BFF08  addi r4, r11, -0xf8
	ctx.r[4].s64 = ctx.r[11].s64 + -248;
	// 83262378: 386AB2AC  addi r3, r10, -0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + -19796;
	// 8326237C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262380: 4AFCAB51  bl 0x8222ced0
	ctx.lr = 0x83262384;
	sub_8222CED0(ctx, base);
	// 83262384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262388: 3869CBB0  addi r3, r9, -0x3450
	ctx.r[3].s64 = ctx.r[9].s64 + -13392;
	// 8326238C: 4BA47B95  bl 0x82ca9f20
	ctx.lr = 0x83262390;
	sub_82CA9F20(ctx, base);
	// 83262390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326239C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832623A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832623A0 size=64
    let mut pc: u32 = 0x832623A0;
    'dispatch: loop {
        match pc {
            0x832623A0 => {
    //   block [0x832623A0..0x832623E0)
	// 832623A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832623A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832623A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832623AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832623B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832623B4: 388BFF18  addi r4, r11, -0xe8
	ctx.r[4].s64 = ctx.r[11].s64 + -232;
	// 832623B8: 386AB2F0  addi r3, r10, -0x4d10
	ctx.r[3].s64 = ctx.r[10].s64 + -19728;
	// 832623BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832623C0: 4AFCAB11  bl 0x8222ced0
	ctx.lr = 0x832623C4;
	sub_8222CED0(ctx, base);
	// 832623C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832623C8: 3869CBC0  addi r3, r9, -0x3440
	ctx.r[3].s64 = ctx.r[9].s64 + -13376;
	// 832623CC: 4BA47B55  bl 0x82ca9f20
	ctx.lr = 0x832623D0;
	sub_82CA9F20(ctx, base);
	// 832623D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832623D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832623D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832623DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832623E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832623E0 size=64
    let mut pc: u32 = 0x832623E0;
    'dispatch: loop {
        match pc {
            0x832623E0 => {
    //   block [0x832623E0..0x83262420)
	// 832623E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832623E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832623E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832623EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832623F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832623F4: 388BFF28  addi r4, r11, -0xd8
	ctx.r[4].s64 = ctx.r[11].s64 + -216;
	// 832623F8: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 832623FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262400: 4AFCAAD1  bl 0x8222ced0
	ctx.lr = 0x83262404;
	sub_8222CED0(ctx, base);
	// 83262404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262408: 3869CBD0  addi r3, r9, -0x3430
	ctx.r[3].s64 = ctx.r[9].s64 + -13360;
	// 8326240C: 4BA47B15  bl 0x82ca9f20
	ctx.lr = 0x83262410;
	sub_82CA9F20(ctx, base);
	// 83262410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326241C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262420 size=112
    let mut pc: u32 = 0x83262420;
    'dispatch: loop {
        match pc {
            0x83262420 => {
    //   block [0x83262420..0x83262490)
	// 83262420: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262424: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83262428: 392BB7A4  addi r9, r11, -0x485c
	ctx.r[9].s64 = ctx.r[11].s64 + -18524;
	// 8326242C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262430: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83262434: C00BB7A4  lfs f0, -0x485c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18524 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262438: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 8326243C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262440: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262490 size=100
    let mut pc: u32 = 0x83262490;
    'dispatch: loop {
        match pc {
            0x83262490 => {
    //   block [0x83262490..0x832624F4)
	// 83262490: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262494: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262498: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 8326249C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 832624A0: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 832624A4: C00B92D4  lfs f0, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832624A8: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 832624AC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832624B0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 832624B4: C1A901B0  lfs f13, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832624B8: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832624BC: 3885B310  addi r4, r5, -0x4cf0
	ctx.r[4].s64 = ctx.r[5].s64 + -19696;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832624F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832624F8 size=96
    let mut pc: u32 = 0x832624F8;
    'dispatch: loop {
        match pc {
            0x832624F8 => {
    //   block [0x832624F8..0x83262558)
	// 832624F8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 832624FC: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83262500: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83262504: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83262508: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326250C: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83262510: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83262514: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262518: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262558 size=108
    let mut pc: u32 = 0x83262558;
    'dispatch: loop {
        match pc {
            0x83262558 => {
    //   block [0x83262558..0x832625C4)
	// 83262558: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326255C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262560: 392BCFCC  addi r9, r11, -0x3034
	ctx.r[9].s64 = ctx.r[11].s64 + -12340;
	// 83262564: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262568: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 8326256C: C00BCFCC  lfs f0, -0x3034(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262570: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83262574: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262578: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 8326257C: C009C4B8  lfs f0, -0x3b48(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262580: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83262584: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832625C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832625C8 size=108
    let mut pc: u32 = 0x832625C8;
    'dispatch: loop {
        match pc {
            0x832625C8 => {
    //   block [0x832625C8..0x83262634)
	// 832625C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832625CC: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832625D0: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 832625D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832625D8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832625DC: 38C1FFF4  addi r6, r1, -0xc
	ctx.r[6].s64 = ctx.r[1].s64 + -12;
	// 832625E0: C1AB92D4  lfs f13, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832625E4: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 832625E8: C00901B0  lfs f0, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832625EC: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 832625F0: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262638 size=108
    let mut pc: u32 = 0x83262638;
    'dispatch: loop {
        match pc {
            0x83262638 => {
    //   block [0x83262638..0x832626A4)
	// 83262638: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326263C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262640: 392BE0DC  addi r9, r11, -0x1f24
	ctx.r[9].s64 = ctx.r[11].s64 + -7972;
	// 83262644: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262648: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 8326264C: C00BE0DC  lfs f0, -0x1f24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7972 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262650: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83262654: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262658: 38A1FFF8  addi r5, r1, -8
	ctx.r[5].s64 = ctx.r[1].s64 + -8;
	// 8326265C: C009B3B4  lfs f0, -0x4c4c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19532 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262660: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83262664: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832626A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832626A8 size=64
    let mut pc: u32 = 0x832626A8;
    'dispatch: loop {
        match pc {
            0x832626A8 => {
    //   block [0x832626A8..0x832626E8)
	// 832626A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832626AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832626B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832626B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832626B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832626BC: 388B35B0  addi r4, r11, 0x35b0
	ctx.r[4].s64 = ctx.r[11].s64 + 13744;
	// 832626C0: 386AB2F8  addi r3, r10, -0x4d08
	ctx.r[3].s64 = ctx.r[10].s64 + -19720;
	// 832626C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832626C8: 4AFCA809  bl 0x8222ced0
	ctx.lr = 0x832626CC;
	sub_8222CED0(ctx, base);
	// 832626CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832626D0: 3869CBE0  addi r3, r9, -0x3420
	ctx.r[3].s64 = ctx.r[9].s64 + -13344;
	// 832626D4: 4BA4784D  bl 0x82ca9f20
	ctx.lr = 0x832626D8;
	sub_82CA9F20(ctx, base);
	// 832626D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832626DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832626E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832626E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832626E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832626E8 size=96
    let mut pc: u32 = 0x832626E8;
    'dispatch: loop {
        match pc {
            0x832626E8 => {
    //   block [0x832626E8..0x83262748)
	// 832626E8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 832626EC: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832626F0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832626F4: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 832626F8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832626FC: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83262700: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83262704: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262708: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262748 size=96
    let mut pc: u32 = 0x83262748;
    'dispatch: loop {
        match pc {
            0x83262748 => {
    //   block [0x83262748..0x832627A8)
	// 83262748: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326274C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262750: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83262754: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262758: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8326275C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262760: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83262764: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262768: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 8326276C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262770: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 83262774: 3885B370  addi r4, r5, -0x4c90
	ctx.r[4].s64 = ctx.r[5].s64 + -19600;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832627A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832627A8 size=376
    let mut pc: u32 = 0x832627A8;
    'dispatch: loop {
        match pc {
            0x832627A8 => {
    //   block [0x832627A8..0x83262920)
	// 832627A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832627AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832627B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832627B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832627B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832627BC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832627C0: 3BEBB380  addi r31, r11, -0x4c80
	ctx.r[31].s64 = ctx.r[11].s64 + -19584;
	// 832627C4: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 832627C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832627CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832627D0: 4AFCA701  bl 0x8222ced0
	ctx.lr = 0x832627D4;
	sub_8222CED0(ctx, base);
	// 832627D4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832627D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832627DC: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 832627E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832627E4: 4AFCA6ED  bl 0x8222ced0
	ctx.lr = 0x832627E8;
	sub_8222CED0(ctx, base);
	// 832627E8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832627EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832627F0: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 832627F4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832627F8: 4AFCA6D9  bl 0x8222ced0
	ctx.lr = 0x832627FC;
	sub_8222CED0(ctx, base);
	// 832627FC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83262800: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262804: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 83262808: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8326280C: 4AFCA6C5  bl 0x8222ced0
	ctx.lr = 0x83262810;
	sub_8222CED0(ctx, base);
	// 83262810: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83262814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262818: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8326281C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83262820: 4AFCA6B1  bl 0x8222ced0
	ctx.lr = 0x83262824;
	sub_8222CED0(ctx, base);
	// 83262824: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83262828: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326282C: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 83262830: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83262834: 4AFCA69D  bl 0x8222ced0
	ctx.lr = 0x83262838;
	sub_8222CED0(ctx, base);
	// 83262838: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8326283C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262840: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 83262844: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83262848: 4AFCA689  bl 0x8222ced0
	ctx.lr = 0x8326284C;
	sub_8222CED0(ctx, base);
	// 8326284C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83262850: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262854: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 83262858: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8326285C: 4AFCA675  bl 0x8222ced0
	ctx.lr = 0x83262860;
	sub_8222CED0(ctx, base);
	// 83262860: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83262864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262868: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8326286C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83262870: 4AFCA661  bl 0x8222ced0
	ctx.lr = 0x83262874;
	sub_8222CED0(ctx, base);
	// 83262874: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83262878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326287C: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 83262880: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83262884: 4AFCA64D  bl 0x8222ced0
	ctx.lr = 0x83262888;
	sub_8222CED0(ctx, base);
	// 83262888: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8326288C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262890: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 83262894: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83262898: 4AFCA639  bl 0x8222ced0
	ctx.lr = 0x8326289C;
	sub_8222CED0(ctx, base);
	// 8326289C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832628A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628A4: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 832628A8: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 832628AC: 4AFCA625  bl 0x8222ced0
	ctx.lr = 0x832628B0;
	sub_8222CED0(ctx, base);
	// 832628B0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832628B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628B8: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 832628BC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832628C0: 4AFCA611  bl 0x8222ced0
	ctx.lr = 0x832628C4;
	sub_8222CED0(ctx, base);
	// 832628C4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832628C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628CC: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 832628D0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832628D4: 4AFCA5FD  bl 0x8222ced0
	ctx.lr = 0x832628D8;
	sub_8222CED0(ctx, base);
	// 832628D8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832628DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628E0: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 832628E4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832628E8: 4AFCA5E9  bl 0x8222ced0
	ctx.lr = 0x832628EC;
	sub_8222CED0(ctx, base);
	// 832628EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832628F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628F4: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 832628F8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832628FC: 4AFCA5D5  bl 0x8222ced0
	ctx.lr = 0x83262900;
	sub_8222CED0(ctx, base);
	// 83262900: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83262904: 386ACBF0  addi r3, r10, -0x3410
	ctx.r[3].s64 = ctx.r[10].s64 + -13328;
	// 83262908: 4BA47619  bl 0x82ca9f20
	ctx.lr = 0x8326290C;
	sub_82CA9F20(ctx, base);
	// 8326290C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326291C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262920 size=376
    let mut pc: u32 = 0x83262920;
    'dispatch: loop {
        match pc {
            0x83262920 => {
    //   block [0x83262920..0x83262A98)
	// 83262920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326292C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262930: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83262934: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83262938: 3BEBB3C0  addi r31, r11, -0x4c40
	ctx.r[31].s64 = ctx.r[11].s64 + -19520;
	// 8326293C: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 83262940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83262944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262948: 4AFCA589  bl 0x8222ced0
	ctx.lr = 0x8326294C;
	sub_8222CED0(ctx, base);
	// 8326294C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83262950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262954: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 83262958: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8326295C: 4AFCA575  bl 0x8222ced0
	ctx.lr = 0x83262960;
	sub_8222CED0(ctx, base);
	// 83262960: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83262964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262968: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8326296C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83262970: 4AFCA561  bl 0x8222ced0
	ctx.lr = 0x83262974;
	sub_8222CED0(ctx, base);
	// 83262974: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83262978: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326297C: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 83262980: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83262984: 4AFCA54D  bl 0x8222ced0
	ctx.lr = 0x83262988;
	sub_8222CED0(ctx, base);
	// 83262988: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8326298C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262990: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 83262994: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83262998: 4AFCA539  bl 0x8222ced0
	ctx.lr = 0x8326299C;
	sub_8222CED0(ctx, base);
	// 8326299C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832629A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629A4: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 832629A8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832629AC: 4AFCA525  bl 0x8222ced0
	ctx.lr = 0x832629B0;
	sub_8222CED0(ctx, base);
	// 832629B0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832629B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629B8: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 832629BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832629C0: 4AFCA511  bl 0x8222ced0
	ctx.lr = 0x832629C4;
	sub_8222CED0(ctx, base);
	// 832629C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832629C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629CC: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 832629D0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832629D4: 4AFCA4FD  bl 0x8222ced0
	ctx.lr = 0x832629D8;
	sub_8222CED0(ctx, base);
	// 832629D8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832629DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629E0: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 832629E4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832629E8: 4AFCA4E9  bl 0x8222ced0
	ctx.lr = 0x832629EC;
	sub_8222CED0(ctx, base);
	// 832629EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832629F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629F4: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 832629F8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832629FC: 4AFCA4D5  bl 0x8222ced0
	ctx.lr = 0x83262A00;
	sub_8222CED0(ctx, base);
	// 83262A00: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83262A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A08: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 83262A0C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83262A10: 4AFCA4C1  bl 0x8222ced0
	ctx.lr = 0x83262A14;
	sub_8222CED0(ctx, base);
	// 83262A14: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83262A18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A1C: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 83262A20: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83262A24: 4AFCA4AD  bl 0x8222ced0
	ctx.lr = 0x83262A28;
	sub_8222CED0(ctx, base);
	// 83262A28: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83262A2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A30: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 83262A34: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83262A38: 4AFCA499  bl 0x8222ced0
	ctx.lr = 0x83262A3C;
	sub_8222CED0(ctx, base);
	// 83262A3C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83262A40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A44: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 83262A48: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83262A4C: 4AFCA485  bl 0x8222ced0
	ctx.lr = 0x83262A50;
	sub_8222CED0(ctx, base);
	// 83262A50: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83262A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A58: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 83262A5C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83262A60: 4AFCA471  bl 0x8222ced0
	ctx.lr = 0x83262A64;
	sub_8222CED0(ctx, base);
	// 83262A64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83262A68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A6C: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 83262A70: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83262A74: 4AFCA45D  bl 0x8222ced0
	ctx.lr = 0x83262A78;
	sub_8222CED0(ctx, base);
	// 83262A78: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83262A7C: 386ACC58  addi r3, r10, -0x33a8
	ctx.r[3].s64 = ctx.r[10].s64 + -13224;
	// 83262A80: 4BA474A1  bl 0x82ca9f20
	ctx.lr = 0x83262A84;
	sub_82CA9F20(ctx, base);
	// 83262A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262A90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83262A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262A98 size=56
    let mut pc: u32 = 0x83262A98;
    'dispatch: loop {
        match pc {
            0x83262A98 => {
    //   block [0x83262A98..0x83262AD0)
	// 83262A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262AA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262AA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262AAC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83262AB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262AB4: 4AF912A5  bl 0x821f3d58
	ctx.lr = 0x83262AB8;
	sub_821F3D58(ctx, base);
	// 83262AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262ABC: 906AB400  stw r3, -0x4c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19456 as u32), ctx.r[3].u32 ) };
	// 83262AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262AD0 size=56
    let mut pc: u32 = 0x83262AD0;
    'dispatch: loop {
        match pc {
            0x83262AD0 => {
    //   block [0x83262AD0..0x83262B08)
	// 83262AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262ADC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262AE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262AE4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83262AE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262AEC: 4AF9126D  bl 0x821f3d58
	ctx.lr = 0x83262AF0;
	sub_821F3D58(ctx, base);
	// 83262AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262AF4: 906AB404  stw r3, -0x4bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19452 as u32), ctx.r[3].u32 ) };
	// 83262AF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262B08 size=56
    let mut pc: u32 = 0x83262B08;
    'dispatch: loop {
        match pc {
            0x83262B08 => {
    //   block [0x83262B08..0x83262B40)
	// 83262B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262B10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262B14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262B18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262B1C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83262B20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262B24: 4AF91235  bl 0x821f3d58
	ctx.lr = 0x83262B28;
	sub_821F3D58(ctx, base);
	// 83262B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262B2C: 906AB408  stw r3, -0x4bf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19448 as u32), ctx.r[3].u32 ) };
	// 83262B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262B40 size=56
    let mut pc: u32 = 0x83262B40;
    'dispatch: loop {
        match pc {
            0x83262B40 => {
    //   block [0x83262B40..0x83262B78)
	// 83262B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262B4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262B50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262B54: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83262B58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262B5C: 4AF911FD  bl 0x821f3d58
	ctx.lr = 0x83262B60;
	sub_821F3D58(ctx, base);
	// 83262B60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262B64: 906AB40C  stw r3, -0x4bf4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19444 as u32), ctx.r[3].u32 ) };
	// 83262B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262B78 size=56
    let mut pc: u32 = 0x83262B78;
    'dispatch: loop {
        match pc {
            0x83262B78 => {
    //   block [0x83262B78..0x83262BB0)
	// 83262B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262B88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262B8C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83262B90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262B94: 4AF911C5  bl 0x821f3d58
	ctx.lr = 0x83262B98;
	sub_821F3D58(ctx, base);
	// 83262B98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262B9C: 906AB410  stw r3, -0x4bf0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19440 as u32), ctx.r[3].u32 ) };
	// 83262BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262BB0 size=56
    let mut pc: u32 = 0x83262BB0;
    'dispatch: loop {
        match pc {
            0x83262BB0 => {
    //   block [0x83262BB0..0x83262BE8)
	// 83262BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262BC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262BC4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83262BC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262BCC: 4AF9118D  bl 0x821f3d58
	ctx.lr = 0x83262BD0;
	sub_821F3D58(ctx, base);
	// 83262BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262BD4: 906AB414  stw r3, -0x4bec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19436 as u32), ctx.r[3].u32 ) };
	// 83262BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262BE8 size=56
    let mut pc: u32 = 0x83262BE8;
    'dispatch: loop {
        match pc {
            0x83262BE8 => {
    //   block [0x83262BE8..0x83262C20)
	// 83262BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262BF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262BFC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83262C00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262C04: 4AF91155  bl 0x821f3d58
	ctx.lr = 0x83262C08;
	sub_821F3D58(ctx, base);
	// 83262C08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262C0C: 906AB418  stw r3, -0x4be8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19432 as u32), ctx.r[3].u32 ) };
	// 83262C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262C20 size=56
    let mut pc: u32 = 0x83262C20;
    'dispatch: loop {
        match pc {
            0x83262C20 => {
    //   block [0x83262C20..0x83262C58)
	// 83262C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262C30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262C34: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83262C38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262C3C: 4AF9111D  bl 0x821f3d58
	ctx.lr = 0x83262C40;
	sub_821F3D58(ctx, base);
	// 83262C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262C44: 906AB41C  stw r3, -0x4be4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19428 as u32), ctx.r[3].u32 ) };
	// 83262C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262C58 size=56
    let mut pc: u32 = 0x83262C58;
    'dispatch: loop {
        match pc {
            0x83262C58 => {
    //   block [0x83262C58..0x83262C90)
	// 83262C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262C64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262C68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262C6C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83262C70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262C74: 4AF910E5  bl 0x821f3d58
	ctx.lr = 0x83262C78;
	sub_821F3D58(ctx, base);
	// 83262C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262C7C: 906AB420  stw r3, -0x4be0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19424 as u32), ctx.r[3].u32 ) };
	// 83262C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262C90 size=56
    let mut pc: u32 = 0x83262C90;
    'dispatch: loop {
        match pc {
            0x83262C90 => {
    //   block [0x83262C90..0x83262CC8)
	// 83262C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262C9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262CA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262CA4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83262CA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262CAC: 4AF910AD  bl 0x821f3d58
	ctx.lr = 0x83262CB0;
	sub_821F3D58(ctx, base);
	// 83262CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262CB4: 906AB424  stw r3, -0x4bdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19420 as u32), ctx.r[3].u32 ) };
	// 83262CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262CC8 size=56
    let mut pc: u32 = 0x83262CC8;
    'dispatch: loop {
        match pc {
            0x83262CC8 => {
    //   block [0x83262CC8..0x83262D00)
	// 83262CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262CD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262CD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262CDC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83262CE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262CE4: 4AF91075  bl 0x821f3d58
	ctx.lr = 0x83262CE8;
	sub_821F3D58(ctx, base);
	// 83262CE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262CEC: 906AB428  stw r3, -0x4bd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19416 as u32), ctx.r[3].u32 ) };
	// 83262CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262D00 size=56
    let mut pc: u32 = 0x83262D00;
    'dispatch: loop {
        match pc {
            0x83262D00 => {
    //   block [0x83262D00..0x83262D38)
	// 83262D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262D0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262D14: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83262D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262D1C: 4AF9103D  bl 0x821f3d58
	ctx.lr = 0x83262D20;
	sub_821F3D58(ctx, base);
	// 83262D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262D24: 906AB42C  stw r3, -0x4bd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19412 as u32), ctx.r[3].u32 ) };
	// 83262D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262D38 size=56
    let mut pc: u32 = 0x83262D38;
    'dispatch: loop {
        match pc {
            0x83262D38 => {
    //   block [0x83262D38..0x83262D70)
	// 83262D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262D44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262D48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262D4C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83262D50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262D54: 4AF91005  bl 0x821f3d58
	ctx.lr = 0x83262D58;
	sub_821F3D58(ctx, base);
	// 83262D58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262D5C: 906AB430  stw r3, -0x4bd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19408 as u32), ctx.r[3].u32 ) };
	// 83262D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262D70 size=56
    let mut pc: u32 = 0x83262D70;
    'dispatch: loop {
        match pc {
            0x83262D70 => {
    //   block [0x83262D70..0x83262DA8)
	// 83262D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262D7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262D80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262D84: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83262D88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262D8C: 4AF90FCD  bl 0x821f3d58
	ctx.lr = 0x83262D90;
	sub_821F3D58(ctx, base);
	// 83262D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262D94: 906AB434  stw r3, -0x4bcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19404 as u32), ctx.r[3].u32 ) };
	// 83262D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262DA8 size=56
    let mut pc: u32 = 0x83262DA8;
    'dispatch: loop {
        match pc {
            0x83262DA8 => {
    //   block [0x83262DA8..0x83262DE0)
	// 83262DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262DB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262DB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262DBC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83262DC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262DC4: 4AF90F95  bl 0x821f3d58
	ctx.lr = 0x83262DC8;
	sub_821F3D58(ctx, base);
	// 83262DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262DCC: 906AB438  stw r3, -0x4bc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19400 as u32), ctx.r[3].u32 ) };
	// 83262DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262DE0 size=56
    let mut pc: u32 = 0x83262DE0;
    'dispatch: loop {
        match pc {
            0x83262DE0 => {
    //   block [0x83262DE0..0x83262E18)
	// 83262DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262DEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262DF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262DF4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83262DF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262DFC: 4AF90F5D  bl 0x821f3d58
	ctx.lr = 0x83262E00;
	sub_821F3D58(ctx, base);
	// 83262E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262E04: 906AB43C  stw r3, -0x4bc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19396 as u32), ctx.r[3].u32 ) };
	// 83262E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262E18 size=56
    let mut pc: u32 = 0x83262E18;
    'dispatch: loop {
        match pc {
            0x83262E18 => {
    //   block [0x83262E18..0x83262E50)
	// 83262E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262E24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262E28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262E2C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83262E30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262E34: 4AF90F25  bl 0x821f3d58
	ctx.lr = 0x83262E38;
	sub_821F3D58(ctx, base);
	// 83262E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262E3C: 906AB440  stw r3, -0x4bc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19392 as u32), ctx.r[3].u32 ) };
	// 83262E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262E50 size=56
    let mut pc: u32 = 0x83262E50;
    'dispatch: loop {
        match pc {
            0x83262E50 => {
    //   block [0x83262E50..0x83262E88)
	// 83262E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262E5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262E60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262E64: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83262E68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262E6C: 4AF90EED  bl 0x821f3d58
	ctx.lr = 0x83262E70;
	sub_821F3D58(ctx, base);
	// 83262E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262E74: 906AB444  stw r3, -0x4bbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19388 as u32), ctx.r[3].u32 ) };
	// 83262E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262E88 size=56
    let mut pc: u32 = 0x83262E88;
    'dispatch: loop {
        match pc {
            0x83262E88 => {
    //   block [0x83262E88..0x83262EC0)
	// 83262E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262E94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262E98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262E9C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83262EA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262EA4: 4AF90EB5  bl 0x821f3d58
	ctx.lr = 0x83262EA8;
	sub_821F3D58(ctx, base);
	// 83262EA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262EAC: 906AB448  stw r3, -0x4bb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19384 as u32), ctx.r[3].u32 ) };
	// 83262EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262EC0 size=56
    let mut pc: u32 = 0x83262EC0;
    'dispatch: loop {
        match pc {
            0x83262EC0 => {
    //   block [0x83262EC0..0x83262EF8)
	// 83262EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262ECC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262ED0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262ED4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83262ED8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262EDC: 4AF90E7D  bl 0x821f3d58
	ctx.lr = 0x83262EE0;
	sub_821F3D58(ctx, base);
	// 83262EE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262EE4: 906AB44C  stw r3, -0x4bb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19380 as u32), ctx.r[3].u32 ) };
	// 83262EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262EF8 size=56
    let mut pc: u32 = 0x83262EF8;
    'dispatch: loop {
        match pc {
            0x83262EF8 => {
    //   block [0x83262EF8..0x83262F30)
	// 83262EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262F04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262F08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262F0C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83262F10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262F14: 4AF90E45  bl 0x821f3d58
	ctx.lr = 0x83262F18;
	sub_821F3D58(ctx, base);
	// 83262F18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262F1C: 906AB450  stw r3, -0x4bb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19376 as u32), ctx.r[3].u32 ) };
	// 83262F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262F30 size=64
    let mut pc: u32 = 0x83262F30;
    'dispatch: loop {
        match pc {
            0x83262F30 => {
    //   block [0x83262F30..0x83262F70)
	// 83262F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262F3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262F44: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 83262F48: 386AB454  addi r3, r10, -0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + -19372;
	// 83262F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262F50: 4AFC9F81  bl 0x8222ced0
	ctx.lr = 0x83262F54;
	sub_8222CED0(ctx, base);
	// 83262F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262F58: 3869CCC0  addi r3, r9, -0x3340
	ctx.r[3].s64 = ctx.r[9].s64 + -13120;
	// 83262F5C: 4BA46FC5  bl 0x82ca9f20
	ctx.lr = 0x83262F60;
	sub_82CA9F20(ctx, base);
	// 83262F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262F70 size=64
    let mut pc: u32 = 0x83262F70;
    'dispatch: loop {
        match pc {
            0x83262F70 => {
    //   block [0x83262F70..0x83262FB0)
	// 83262F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262F7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262F84: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83262F88: 386AB458  addi r3, r10, -0x4ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -19368;
	// 83262F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262F90: 4AFC9F41  bl 0x8222ced0
	ctx.lr = 0x83262F94;
	sub_8222CED0(ctx, base);
	// 83262F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262F98: 3869CCD0  addi r3, r9, -0x3330
	ctx.r[3].s64 = ctx.r[9].s64 + -13104;
	// 83262F9C: 4BA46F85  bl 0x82ca9f20
	ctx.lr = 0x83262FA0;
	sub_82CA9F20(ctx, base);
	// 83262FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262FB0 size=64
    let mut pc: u32 = 0x83262FB0;
    'dispatch: loop {
        match pc {
            0x83262FB0 => {
    //   block [0x83262FB0..0x83262FF0)
	// 83262FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262FBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262FC4: 388BFF7C  addi r4, r11, -0x84
	ctx.r[4].s64 = ctx.r[11].s64 + -132;
	// 83262FC8: 386AB45C  addi r3, r10, -0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -19364;
	// 83262FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262FD0: 4AFC9F01  bl 0x8222ced0
	ctx.lr = 0x83262FD4;
	sub_8222CED0(ctx, base);
	// 83262FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262FD8: 3869CCE0  addi r3, r9, -0x3320
	ctx.r[3].s64 = ctx.r[9].s64 + -13088;
	// 83262FDC: 4BA46F45  bl 0x82ca9f20
	ctx.lr = 0x83262FE0;
	sub_82CA9F20(ctx, base);
	// 83262FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262FF0 size=64
    let mut pc: u32 = 0x83262FF0;
    'dispatch: loop {
        match pc {
            0x83262FF0 => {
    //   block [0x83262FF0..0x83263030)
	// 83262FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262FFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263004: 388BFFA0  addi r4, r11, -0x60
	ctx.r[4].s64 = ctx.r[11].s64 + -96;
	// 83263008: 386AB460  addi r3, r10, -0x4ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -19360;
	// 8326300C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263010: 4AFC9EC1  bl 0x8222ced0
	ctx.lr = 0x83263014;
	sub_8222CED0(ctx, base);
	// 83263014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263018: 3869CCF0  addi r3, r9, -0x3310
	ctx.r[3].s64 = ctx.r[9].s64 + -13072;
	// 8326301C: 4BA46F05  bl 0x82ca9f20
	ctx.lr = 0x83263020;
	sub_82CA9F20(ctx, base);
	// 83263020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326302C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263030 size=64
    let mut pc: u32 = 0x83263030;
    'dispatch: loop {
        match pc {
            0x83263030 => {
    //   block [0x83263030..0x83263070)
	// 83263030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326303C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263044: 388BFFC4  addi r4, r11, -0x3c
	ctx.r[4].s64 = ctx.r[11].s64 + -60;
	// 83263048: 386AB464  addi r3, r10, -0x4b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19356;
	// 8326304C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263050: 4AFC9E81  bl 0x8222ced0
	ctx.lr = 0x83263054;
	sub_8222CED0(ctx, base);
	// 83263054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263058: 3869CD00  addi r3, r9, -0x3300
	ctx.r[3].s64 = ctx.r[9].s64 + -13056;
	// 8326305C: 4BA46EC5  bl 0x82ca9f20
	ctx.lr = 0x83263060;
	sub_82CA9F20(ctx, base);
	// 83263060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263070 size=64
    let mut pc: u32 = 0x83263070;
    'dispatch: loop {
        match pc {
            0x83263070 => {
    //   block [0x83263070..0x832630B0)
	// 83263070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326307C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263084: 388BFFF0  addi r4, r11, -0x10
	ctx.r[4].s64 = ctx.r[11].s64 + -16;
	// 83263088: 386AB468  addi r3, r10, -0x4b98
	ctx.r[3].s64 = ctx.r[10].s64 + -19352;
	// 8326308C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263090: 4AFC9E41  bl 0x8222ced0
	ctx.lr = 0x83263094;
	sub_8222CED0(ctx, base);
	// 83263094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263098: 3869CD10  addi r3, r9, -0x32f0
	ctx.r[3].s64 = ctx.r[9].s64 + -13040;
	// 8326309C: 4BA46E85  bl 0x82ca9f20
	ctx.lr = 0x832630A0;
	sub_82CA9F20(ctx, base);
	// 832630A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832630A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832630A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832630AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832630B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832630B0 size=64
    let mut pc: u32 = 0x832630B0;
    'dispatch: loop {
        match pc {
            0x832630B0 => {
    //   block [0x832630B0..0x832630F0)
	// 832630B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832630B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832630B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832630BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832630C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832630C4: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 832630C8: 386AB46C  addi r3, r10, -0x4b94
	ctx.r[3].s64 = ctx.r[10].s64 + -19348;
	// 832630CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832630D0: 4AFC9E01  bl 0x8222ced0
	ctx.lr = 0x832630D4;
	sub_8222CED0(ctx, base);
	// 832630D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832630D8: 3869CD20  addi r3, r9, -0x32e0
	ctx.r[3].s64 = ctx.r[9].s64 + -13024;
	// 832630DC: 4BA46E45  bl 0x82ca9f20
	ctx.lr = 0x832630E0;
	sub_82CA9F20(ctx, base);
	// 832630E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832630E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832630E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832630EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832630F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832630F0 size=64
    let mut pc: u32 = 0x832630F0;
    'dispatch: loop {
        match pc {
            0x832630F0 => {
    //   block [0x832630F0..0x83263130)
	// 832630F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832630F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832630F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832630FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263104: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 83263108: 386AB470  addi r3, r10, -0x4b90
	ctx.r[3].s64 = ctx.r[10].s64 + -19344;
	// 8326310C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263110: 4AFC9DC1  bl 0x8222ced0
	ctx.lr = 0x83263114;
	sub_8222CED0(ctx, base);
	// 83263114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263118: 3869CD30  addi r3, r9, -0x32d0
	ctx.r[3].s64 = ctx.r[9].s64 + -13008;
	// 8326311C: 4BA46E05  bl 0x82ca9f20
	ctx.lr = 0x83263120;
	sub_82CA9F20(ctx, base);
	// 83263120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326312C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263130 size=64
    let mut pc: u32 = 0x83263130;
    'dispatch: loop {
        match pc {
            0x83263130 => {
    //   block [0x83263130..0x83263170)
	// 83263130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326313C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263144: 388B0058  addi r4, r11, 0x58
	ctx.r[4].s64 = ctx.r[11].s64 + 88;
	// 83263148: 386AB474  addi r3, r10, -0x4b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19340;
	// 8326314C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263150: 4AFC9D81  bl 0x8222ced0
	ctx.lr = 0x83263154;
	sub_8222CED0(ctx, base);
	// 83263154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263158: 3869CD40  addi r3, r9, -0x32c0
	ctx.r[3].s64 = ctx.r[9].s64 + -12992;
	// 8326315C: 4BA46DC5  bl 0x82ca9f20
	ctx.lr = 0x83263160;
	sub_82CA9F20(ctx, base);
	// 83263160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326316C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263170 size=64
    let mut pc: u32 = 0x83263170;
    'dispatch: loop {
        match pc {
            0x83263170 => {
    //   block [0x83263170..0x832631B0)
	// 83263170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326317C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263184: 388B0084  addi r4, r11, 0x84
	ctx.r[4].s64 = ctx.r[11].s64 + 132;
	// 83263188: 386AB478  addi r3, r10, -0x4b88
	ctx.r[3].s64 = ctx.r[10].s64 + -19336;
	// 8326318C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263190: 4AFC9D41  bl 0x8222ced0
	ctx.lr = 0x83263194;
	sub_8222CED0(ctx, base);
	// 83263194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263198: 3869CD50  addi r3, r9, -0x32b0
	ctx.r[3].s64 = ctx.r[9].s64 + -12976;
	// 8326319C: 4BA46D85  bl 0x82ca9f20
	ctx.lr = 0x832631A0;
	sub_82CA9F20(ctx, base);
	// 832631A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832631A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832631A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832631AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832631B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832631B0 size=64
    let mut pc: u32 = 0x832631B0;
    'dispatch: loop {
        match pc {
            0x832631B0 => {
    //   block [0x832631B0..0x832631F0)
	// 832631B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832631B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832631B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832631BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832631C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832631C4: 388B00A4  addi r4, r11, 0xa4
	ctx.r[4].s64 = ctx.r[11].s64 + 164;
	// 832631C8: 386AB47C  addi r3, r10, -0x4b84
	ctx.r[3].s64 = ctx.r[10].s64 + -19332;
	// 832631CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832631D0: 4AFC9D01  bl 0x8222ced0
	ctx.lr = 0x832631D4;
	sub_8222CED0(ctx, base);
	// 832631D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832631D8: 3869CD60  addi r3, r9, -0x32a0
	ctx.r[3].s64 = ctx.r[9].s64 + -12960;
	// 832631DC: 4BA46D45  bl 0x82ca9f20
	ctx.lr = 0x832631E0;
	sub_82CA9F20(ctx, base);
	// 832631E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832631E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832631E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832631EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832631F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832631F0 size=56
    let mut pc: u32 = 0x832631F0;
    'dispatch: loop {
        match pc {
            0x832631F0 => {
    //   block [0x832631F0..0x83263228)
	// 832631F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832631F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832631F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832631FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263200: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263204: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83263208: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326320C: 4AF90B4D  bl 0x821f3d58
	ctx.lr = 0x83263210;
	sub_821F3D58(ctx, base);
	// 83263210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263214: 906AB480  stw r3, -0x4b80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19328 as u32), ctx.r[3].u32 ) };
	// 83263218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326321C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263228 size=56
    let mut pc: u32 = 0x83263228;
    'dispatch: loop {
        match pc {
            0x83263228 => {
    //   block [0x83263228..0x83263260)
	// 83263228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263234: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263238: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326323C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83263240: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263244: 4AF90B15  bl 0x821f3d58
	ctx.lr = 0x83263248;
	sub_821F3D58(ctx, base);
	// 83263248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326324C: 906AB484  stw r3, -0x4b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19324 as u32), ctx.r[3].u32 ) };
	// 83263250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263260 size=56
    let mut pc: u32 = 0x83263260;
    'dispatch: loop {
        match pc {
            0x83263260 => {
    //   block [0x83263260..0x83263298)
	// 83263260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326326C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263274: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83263278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326327C: 4AF90ADD  bl 0x821f3d58
	ctx.lr = 0x83263280;
	sub_821F3D58(ctx, base);
	// 83263280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263284: 906AB488  stw r3, -0x4b78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19320 as u32), ctx.r[3].u32 ) };
	// 83263288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326328C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263298 size=56
    let mut pc: u32 = 0x83263298;
    'dispatch: loop {
        match pc {
            0x83263298 => {
    //   block [0x83263298..0x832632D0)
	// 83263298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326329C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832632A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832632A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832632A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832632AC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832632B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832632B4: 4AF90AA5  bl 0x821f3d58
	ctx.lr = 0x832632B8;
	sub_821F3D58(ctx, base);
	// 832632B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832632BC: 906AB48C  stw r3, -0x4b74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19316 as u32), ctx.r[3].u32 ) };
	// 832632C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832632C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832632C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832632CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832632D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832632D0 size=56
    let mut pc: u32 = 0x832632D0;
    'dispatch: loop {
        match pc {
            0x832632D0 => {
    //   block [0x832632D0..0x83263308)
	// 832632D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832632D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832632D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832632DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832632E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832632E4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832632E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832632EC: 4AF90A6D  bl 0x821f3d58
	ctx.lr = 0x832632F0;
	sub_821F3D58(ctx, base);
	// 832632F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832632F4: 906AB490  stw r3, -0x4b70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19312 as u32), ctx.r[3].u32 ) };
	// 832632F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832632FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263308 size=56
    let mut pc: u32 = 0x83263308;
    'dispatch: loop {
        match pc {
            0x83263308 => {
    //   block [0x83263308..0x83263340)
	// 83263308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326330C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326331C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83263320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263324: 4AF90A35  bl 0x821f3d58
	ctx.lr = 0x83263328;
	sub_821F3D58(ctx, base);
	// 83263328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326332C: 906AB494  stw r3, -0x4b6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19308 as u32), ctx.r[3].u32 ) };
	// 83263330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263340 size=56
    let mut pc: u32 = 0x83263340;
    'dispatch: loop {
        match pc {
            0x83263340 => {
    //   block [0x83263340..0x83263378)
	// 83263340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326334C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263354: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83263358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326335C: 4AF909FD  bl 0x821f3d58
	ctx.lr = 0x83263360;
	sub_821F3D58(ctx, base);
	// 83263360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263364: 906AB498  stw r3, -0x4b68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19304 as u32), ctx.r[3].u32 ) };
	// 83263368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326336C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263378 size=56
    let mut pc: u32 = 0x83263378;
    'dispatch: loop {
        match pc {
            0x83263378 => {
    //   block [0x83263378..0x832633B0)
	// 83263378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263384: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326338C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83263390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263394: 4AF909C5  bl 0x821f3d58
	ctx.lr = 0x83263398;
	sub_821F3D58(ctx, base);
	// 83263398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326339C: 906AB49C  stw r3, -0x4b64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19300 as u32), ctx.r[3].u32 ) };
	// 832633A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832633A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832633A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832633AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832633B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832633B0 size=56
    let mut pc: u32 = 0x832633B0;
    'dispatch: loop {
        match pc {
            0x832633B0 => {
    //   block [0x832633B0..0x832633E8)
	// 832633B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832633B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832633B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832633BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832633C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832633C4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832633C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832633CC: 4AF9098D  bl 0x821f3d58
	ctx.lr = 0x832633D0;
	sub_821F3D58(ctx, base);
	// 832633D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832633D4: 906AB4A0  stw r3, -0x4b60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19296 as u32), ctx.r[3].u32 ) };
	// 832633D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832633DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832633E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832633E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832633E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832633E8 size=56
    let mut pc: u32 = 0x832633E8;
    'dispatch: loop {
        match pc {
            0x832633E8 => {
    //   block [0x832633E8..0x83263420)
	// 832633E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832633EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832633F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832633F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832633F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832633FC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83263400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263404: 4AF90955  bl 0x821f3d58
	ctx.lr = 0x83263408;
	sub_821F3D58(ctx, base);
	// 83263408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326340C: 906AB4A4  stw r3, -0x4b5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19292 as u32), ctx.r[3].u32 ) };
	// 83263410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263420 size=56
    let mut pc: u32 = 0x83263420;
    'dispatch: loop {
        match pc {
            0x83263420 => {
    //   block [0x83263420..0x83263458)
	// 83263420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326342C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263434: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83263438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326343C: 4AF9091D  bl 0x821f3d58
	ctx.lr = 0x83263440;
	sub_821F3D58(ctx, base);
	// 83263440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263444: 906AB4A8  stw r3, -0x4b58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19288 as u32), ctx.r[3].u32 ) };
	// 83263448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326344C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263458 size=56
    let mut pc: u32 = 0x83263458;
    'dispatch: loop {
        match pc {
            0x83263458 => {
    //   block [0x83263458..0x83263490)
	// 83263458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326345C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263464: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263468: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326346C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83263470: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263474: 4AF908E5  bl 0x821f3d58
	ctx.lr = 0x83263478;
	sub_821F3D58(ctx, base);
	// 83263478: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326347C: 906AB4AC  stw r3, -0x4b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19284 as u32), ctx.r[3].u32 ) };
	// 83263480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263490 size=56
    let mut pc: u32 = 0x83263490;
    'dispatch: loop {
        match pc {
            0x83263490 => {
    //   block [0x83263490..0x832634C8)
	// 83263490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326349C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832634A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832634A4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832634A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832634AC: 4AF908AD  bl 0x821f3d58
	ctx.lr = 0x832634B0;
	sub_821F3D58(ctx, base);
	// 832634B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832634B4: 906AB4B0  stw r3, -0x4b50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19280 as u32), ctx.r[3].u32 ) };
	// 832634B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832634BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832634C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832634C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832634C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832634C8 size=56
    let mut pc: u32 = 0x832634C8;
    'dispatch: loop {
        match pc {
            0x832634C8 => {
    //   block [0x832634C8..0x83263500)
	// 832634C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832634CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832634D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832634D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832634D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832634DC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832634E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832634E4: 4AF90875  bl 0x821f3d58
	ctx.lr = 0x832634E8;
	sub_821F3D58(ctx, base);
	// 832634E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832634EC: 906AB4B4  stw r3, -0x4b4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19276 as u32), ctx.r[3].u32 ) };
	// 832634F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832634F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832634F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832634FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263500 size=56
    let mut pc: u32 = 0x83263500;
    'dispatch: loop {
        match pc {
            0x83263500 => {
    //   block [0x83263500..0x83263538)
	// 83263500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326350C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263510: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263514: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83263518: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326351C: 4AF9083D  bl 0x821f3d58
	ctx.lr = 0x83263520;
	sub_821F3D58(ctx, base);
	// 83263520: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263524: 906AB4B8  stw r3, -0x4b48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19272 as u32), ctx.r[3].u32 ) };
	// 83263528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326352C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263538 size=56
    let mut pc: u32 = 0x83263538;
    'dispatch: loop {
        match pc {
            0x83263538 => {
    //   block [0x83263538..0x83263570)
	// 83263538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263548: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326354C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83263550: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263554: 4AF90805  bl 0x821f3d58
	ctx.lr = 0x83263558;
	sub_821F3D58(ctx, base);
	// 83263558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326355C: 906AB4BC  stw r3, -0x4b44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19268 as u32), ctx.r[3].u32 ) };
	// 83263560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326356C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263570 size=56
    let mut pc: u32 = 0x83263570;
    'dispatch: loop {
        match pc {
            0x83263570 => {
    //   block [0x83263570..0x832635A8)
	// 83263570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326357C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263580: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263584: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83263588: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326358C: 4AF907CD  bl 0x821f3d58
	ctx.lr = 0x83263590;
	sub_821F3D58(ctx, base);
	// 83263590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263594: 906AB4C0  stw r3, -0x4b40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19264 as u32), ctx.r[3].u32 ) };
	// 83263598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326359C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832635A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832635A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832635A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832635A8 size=56
    let mut pc: u32 = 0x832635A8;
    'dispatch: loop {
        match pc {
            0x832635A8 => {
    //   block [0x832635A8..0x832635E0)
	// 832635A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832635AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832635B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832635B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832635B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832635BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832635C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832635C4: 4AF90795  bl 0x821f3d58
	ctx.lr = 0x832635C8;
	sub_821F3D58(ctx, base);
	// 832635C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832635CC: 906AB4C4  stw r3, -0x4b3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19260 as u32), ctx.r[3].u32 ) };
	// 832635D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832635D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832635D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832635DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832635E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832635E0 size=56
    let mut pc: u32 = 0x832635E0;
    'dispatch: loop {
        match pc {
            0x832635E0 => {
    //   block [0x832635E0..0x83263618)
	// 832635E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832635E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832635E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832635EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832635F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832635F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832635F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832635FC: 4AF9075D  bl 0x821f3d58
	ctx.lr = 0x83263600;
	sub_821F3D58(ctx, base);
	// 83263600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263604: 906AB4C8  stw r3, -0x4b38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19256 as u32), ctx.r[3].u32 ) };
	// 83263608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326360C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263618 size=56
    let mut pc: u32 = 0x83263618;
    'dispatch: loop {
        match pc {
            0x83263618 => {
    //   block [0x83263618..0x83263650)
	// 83263618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326361C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263624: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263628: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326362C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83263630: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263634: 4AF90725  bl 0x821f3d58
	ctx.lr = 0x83263638;
	sub_821F3D58(ctx, base);
	// 83263638: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326363C: 906AB4CC  stw r3, -0x4b34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19252 as u32), ctx.r[3].u32 ) };
	// 83263640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263650 size=56
    let mut pc: u32 = 0x83263650;
    'dispatch: loop {
        match pc {
            0x83263650 => {
    //   block [0x83263650..0x83263688)
	// 83263650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326365C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263660: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263664: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83263668: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326366C: 4AF906ED  bl 0x821f3d58
	ctx.lr = 0x83263670;
	sub_821F3D58(ctx, base);
	// 83263670: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263674: 906AB4D0  stw r3, -0x4b30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19248 as u32), ctx.r[3].u32 ) };
	// 83263678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326367C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263688 size=64
    let mut pc: u32 = 0x83263688;
    'dispatch: loop {
        match pc {
            0x83263688 => {
    //   block [0x83263688..0x832636C8)
	// 83263688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263694: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326369C: 388BF408  addi r4, r11, -0xbf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3064;
	// 832636A0: 386AB4D4  addi r3, r10, -0x4b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19244;
	// 832636A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832636A8: 4AFC9829  bl 0x8222ced0
	ctx.lr = 0x832636AC;
	sub_8222CED0(ctx, base);
	// 832636AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832636B0: 3869CD70  addi r3, r9, -0x3290
	ctx.r[3].s64 = ctx.r[9].s64 + -12944;
	// 832636B4: 4BA4686D  bl 0x82ca9f20
	ctx.lr = 0x832636B8;
	sub_82CA9F20(ctx, base);
	// 832636B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832636BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832636C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832636C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832636C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832636C8 size=64
    let mut pc: u32 = 0x832636C8;
    'dispatch: loop {
        match pc {
            0x832636C8 => {
    //   block [0x832636C8..0x83263708)
	// 832636C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832636CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832636D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832636D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832636D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832636DC: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 832636E0: 386AB4D8  addi r3, r10, -0x4b28
	ctx.r[3].s64 = ctx.r[10].s64 + -19240;
	// 832636E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832636E8: 4AFC97E9  bl 0x8222ced0
	ctx.lr = 0x832636EC;
	sub_8222CED0(ctx, base);
	// 832636EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832636F0: 3869CD80  addi r3, r9, -0x3280
	ctx.r[3].s64 = ctx.r[9].s64 + -12928;
	// 832636F4: 4BA4682D  bl 0x82ca9f20
	ctx.lr = 0x832636F8;
	sub_82CA9F20(ctx, base);
	// 832636F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832636FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263708 size=64
    let mut pc: u32 = 0x83263708;
    'dispatch: loop {
        match pc {
            0x83263708 => {
    //   block [0x83263708..0x83263748)
	// 83263708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263714: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263718: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326371C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83263720: 386AB4DC  addi r3, r10, -0x4b24
	ctx.r[3].s64 = ctx.r[10].s64 + -19236;
	// 83263724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263728: 4AFC97A9  bl 0x8222ced0
	ctx.lr = 0x8326372C;
	sub_8222CED0(ctx, base);
	// 8326372C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263730: 3869CD90  addi r3, r9, -0x3270
	ctx.r[3].s64 = ctx.r[9].s64 + -12912;
	// 83263734: 4BA467ED  bl 0x82ca9f20
	ctx.lr = 0x83263738;
	sub_82CA9F20(ctx, base);
	// 83263738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326373C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83263748 size=96
    let mut pc: u32 = 0x83263748;
    'dispatch: loop {
        match pc {
            0x83263748 => {
    //   block [0x83263748..0x832637A8)
	// 83263748: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326374C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83263750: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 83263754: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83263758: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326375C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83263760: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83263764: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83263768: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832637A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832637A8 size=96
    let mut pc: u32 = 0x832637A8;
    'dispatch: loop {
        match pc {
            0x832637A8 => {
    //   block [0x832637A8..0x83263808)
	// 832637A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832637AC: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832637B0: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 832637B4: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832637B8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832637BC: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832637C0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832637C4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832637C8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83263808 size=96
    let mut pc: u32 = 0x83263808;
    'dispatch: loop {
        match pc {
            0x83263808 => {
    //   block [0x83263808..0x83263868)
	// 83263808: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326380C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83263810: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83263814: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83263818: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326381C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83263820: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83263824: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83263828: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263868 size=64
    let mut pc: u32 = 0x83263868;
    'dispatch: loop {
        match pc {
            0x83263868 => {
    //   block [0x83263868..0x832638A8)
	// 83263868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263874: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326387C: 388B0298  addi r4, r11, 0x298
	ctx.r[4].s64 = ctx.r[11].s64 + 664;
	// 83263880: 386AB510  addi r3, r10, -0x4af0
	ctx.r[3].s64 = ctx.r[10].s64 + -19184;
	// 83263884: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263888: 4AFC9649  bl 0x8222ced0
	ctx.lr = 0x8326388C;
	sub_8222CED0(ctx, base);
	// 8326388C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263890: 3869CDA0  addi r3, r9, -0x3260
	ctx.r[3].s64 = ctx.r[9].s64 + -12896;
	// 83263894: 4BA4668D  bl 0x82ca9f20
	ctx.lr = 0x83263898;
	sub_82CA9F20(ctx, base);
	// 83263898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326389C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832638A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832638A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832638A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832638A8 size=64
    let mut pc: u32 = 0x832638A8;
    'dispatch: loop {
        match pc {
            0x832638A8 => {
    //   block [0x832638A8..0x832638E8)
	// 832638A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832638AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832638B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832638B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832638B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832638BC: 388B02AC  addi r4, r11, 0x2ac
	ctx.r[4].s64 = ctx.r[11].s64 + 684;
	// 832638C0: 386AB514  addi r3, r10, -0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + -19180;
	// 832638C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832638C8: 4AFC9609  bl 0x8222ced0
	ctx.lr = 0x832638CC;
	sub_8222CED0(ctx, base);
	// 832638CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832638D0: 3869CDB0  addi r3, r9, -0x3250
	ctx.r[3].s64 = ctx.r[9].s64 + -12880;
	// 832638D4: 4BA4664D  bl 0x82ca9f20
	ctx.lr = 0x832638D8;
	sub_82CA9F20(ctx, base);
	// 832638D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832638DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832638E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832638E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832638E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832638E8 size=64
    let mut pc: u32 = 0x832638E8;
    'dispatch: loop {
        match pc {
            0x832638E8 => {
    //   block [0x832638E8..0x83263928)
	// 832638E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832638EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832638F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832638F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832638F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832638FC: 388B02B8  addi r4, r11, 0x2b8
	ctx.r[4].s64 = ctx.r[11].s64 + 696;
	// 83263900: 386AB518  addi r3, r10, -0x4ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -19176;
	// 83263904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263908: 4AFC95C9  bl 0x8222ced0
	ctx.lr = 0x8326390C;
	sub_8222CED0(ctx, base);
	// 8326390C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263910: 3869CDC0  addi r3, r9, -0x3240
	ctx.r[3].s64 = ctx.r[9].s64 + -12864;
	// 83263914: 4BA4660D  bl 0x82ca9f20
	ctx.lr = 0x83263918;
	sub_82CA9F20(ctx, base);
	// 83263918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326391C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263928 size=64
    let mut pc: u32 = 0x83263928;
    'dispatch: loop {
        match pc {
            0x83263928 => {
    //   block [0x83263928..0x83263968)
	// 83263928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326392C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263934: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326393C: 388B02C4  addi r4, r11, 0x2c4
	ctx.r[4].s64 = ctx.r[11].s64 + 708;
	// 83263940: 386AB51C  addi r3, r10, -0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -19172;
	// 83263944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263948: 4AFC9589  bl 0x8222ced0
	ctx.lr = 0x8326394C;
	sub_8222CED0(ctx, base);
	// 8326394C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263950: 3869CDD0  addi r3, r9, -0x3230
	ctx.r[3].s64 = ctx.r[9].s64 + -12848;
	// 83263954: 4BA465CD  bl 0x82ca9f20
	ctx.lr = 0x83263958;
	sub_82CA9F20(ctx, base);
	// 83263958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326395C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263968 size=64
    let mut pc: u32 = 0x83263968;
    'dispatch: loop {
        match pc {
            0x83263968 => {
    //   block [0x83263968..0x832639A8)
	// 83263968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263974: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326397C: 388B02CC  addi r4, r11, 0x2cc
	ctx.r[4].s64 = ctx.r[11].s64 + 716;
	// 83263980: 386AB520  addi r3, r10, -0x4ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -19168;
	// 83263984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263988: 4AFC9549  bl 0x8222ced0
	ctx.lr = 0x8326398C;
	sub_8222CED0(ctx, base);
	// 8326398C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263990: 3869CDE0  addi r3, r9, -0x3220
	ctx.r[3].s64 = ctx.r[9].s64 + -12832;
	// 83263994: 4BA4658D  bl 0x82ca9f20
	ctx.lr = 0x83263998;
	sub_82CA9F20(ctx, base);
	// 83263998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326399C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832639A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832639A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832639A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832639A8 size=64
    let mut pc: u32 = 0x832639A8;
    'dispatch: loop {
        match pc {
            0x832639A8 => {
    //   block [0x832639A8..0x832639E8)
	// 832639A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832639AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832639B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832639B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832639B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832639BC: 388BABCC  addi r4, r11, -0x5434
	ctx.r[4].s64 = ctx.r[11].s64 + -21556;
	// 832639C0: 386AB524  addi r3, r10, -0x4adc
	ctx.r[3].s64 = ctx.r[10].s64 + -19164;
	// 832639C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832639C8: 4AFC9509  bl 0x8222ced0
	ctx.lr = 0x832639CC;
	sub_8222CED0(ctx, base);
	// 832639CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832639D0: 3869CDF0  addi r3, r9, -0x3210
	ctx.r[3].s64 = ctx.r[9].s64 + -12816;
	// 832639D4: 4BA4654D  bl 0x82ca9f20
	ctx.lr = 0x832639D8;
	sub_82CA9F20(ctx, base);
	// 832639D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832639DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832639E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832639E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832639E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832639E8 size=64
    let mut pc: u32 = 0x832639E8;
    'dispatch: loop {
        match pc {
            0x832639E8 => {
    //   block [0x832639E8..0x83263A28)
	// 832639E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832639EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832639F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832639F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832639F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832639FC: 388BABD0  addi r4, r11, -0x5430
	ctx.r[4].s64 = ctx.r[11].s64 + -21552;
	// 83263A00: 386AB528  addi r3, r10, -0x4ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -19160;
	// 83263A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263A08: 4AFC94C9  bl 0x8222ced0
	ctx.lr = 0x83263A0C;
	sub_8222CED0(ctx, base);
	// 83263A0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263A10: 3869CE00  addi r3, r9, -0x3200
	ctx.r[3].s64 = ctx.r[9].s64 + -12800;
	// 83263A14: 4BA4650D  bl 0x82ca9f20
	ctx.lr = 0x83263A18;
	sub_82CA9F20(ctx, base);
	// 83263A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263A28 size=64
    let mut pc: u32 = 0x83263A28;
    'dispatch: loop {
        match pc {
            0x83263A28 => {
    //   block [0x83263A28..0x83263A68)
	// 83263A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263A34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83263A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263A3C: 388BABD8  addi r4, r11, -0x5428
	ctx.r[4].s64 = ctx.r[11].s64 + -21544;
	// 83263A40: 386AB52C  addi r3, r10, -0x4ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -19156;
	// 83263A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263A48: 4AFC9489  bl 0x8222ced0
	ctx.lr = 0x83263A4C;
	sub_8222CED0(ctx, base);
	// 83263A4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263A50: 3869CE10  addi r3, r9, -0x31f0
	ctx.r[3].s64 = ctx.r[9].s64 + -12784;
	// 83263A54: 4BA464CD  bl 0x82ca9f20
	ctx.lr = 0x83263A58;
	sub_82CA9F20(ctx, base);
	// 83263A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263A68 size=64
    let mut pc: u32 = 0x83263A68;
    'dispatch: loop {
        match pc {
            0x83263A68 => {
    //   block [0x83263A68..0x83263AA8)
	// 83263A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263A74: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83263A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263A7C: 388B34A0  addi r4, r11, 0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + 13472;
	// 83263A80: 386AB530  addi r3, r10, -0x4ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -19152;
	// 83263A84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263A88: 4AFC9449  bl 0x8222ced0
	ctx.lr = 0x83263A8C;
	sub_8222CED0(ctx, base);
	// 83263A8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263A90: 3869CE20  addi r3, r9, -0x31e0
	ctx.r[3].s64 = ctx.r[9].s64 + -12768;
	// 83263A94: 4BA4648D  bl 0x82ca9f20
	ctx.lr = 0x83263A98;
	sub_82CA9F20(ctx, base);
	// 83263A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263AA8 size=64
    let mut pc: u32 = 0x83263AA8;
    'dispatch: loop {
        match pc {
            0x83263AA8 => {
    //   block [0x83263AA8..0x83263AE8)
	// 83263AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263AB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263ABC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83263AC0: 386AB534  addi r3, r10, -0x4acc
	ctx.r[3].s64 = ctx.r[10].s64 + -19148;
	// 83263AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263AC8: 4AFC9409  bl 0x8222ced0
	ctx.lr = 0x83263ACC;
	sub_8222CED0(ctx, base);
	// 83263ACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263AD0: 3869CE30  addi r3, r9, -0x31d0
	ctx.r[3].s64 = ctx.r[9].s64 + -12752;
	// 83263AD4: 4BA4644D  bl 0x82ca9f20
	ctx.lr = 0x83263AD8;
	sub_82CA9F20(ctx, base);
	// 83263AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263AE8 size=64
    let mut pc: u32 = 0x83263AE8;
    'dispatch: loop {
        match pc {
            0x83263AE8 => {
    //   block [0x83263AE8..0x83263B28)
	// 83263AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263AF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263AFC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83263B00: 386AB538  addi r3, r10, -0x4ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -19144;
	// 83263B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263B08: 4AFC93C9  bl 0x8222ced0
	ctx.lr = 0x83263B0C;
	sub_8222CED0(ctx, base);
	// 83263B0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263B10: 3869CE40  addi r3, r9, -0x31c0
	ctx.r[3].s64 = ctx.r[9].s64 + -12736;
	// 83263B14: 4BA4640D  bl 0x82ca9f20
	ctx.lr = 0x83263B18;
	sub_82CA9F20(ctx, base);
	// 83263B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263B28 size=64
    let mut pc: u32 = 0x83263B28;
    'dispatch: loop {
        match pc {
            0x83263B28 => {
    //   block [0x83263B28..0x83263B68)
	// 83263B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263B34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263B38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263B3C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83263B40: 386AB53C  addi r3, r10, -0x4ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -19140;
	// 83263B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263B48: 4AFC9389  bl 0x8222ced0
	ctx.lr = 0x83263B4C;
	sub_8222CED0(ctx, base);
	// 83263B4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263B50: 3869CE50  addi r3, r9, -0x31b0
	ctx.r[3].s64 = ctx.r[9].s64 + -12720;
	// 83263B54: 4BA463CD  bl 0x82ca9f20
	ctx.lr = 0x83263B58;
	sub_82CA9F20(ctx, base);
	// 83263B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263B68 size=56
    let mut pc: u32 = 0x83263B68;
    'dispatch: loop {
        match pc {
            0x83263B68 => {
    //   block [0x83263B68..0x83263BA0)
	// 83263B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263B7C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83263B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263B84: 4AF901D5  bl 0x821f3d58
	ctx.lr = 0x83263B88;
	sub_821F3D58(ctx, base);
	// 83263B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263B8C: 906AB540  stw r3, -0x4ac0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19136 as u32), ctx.r[3].u32 ) };
	// 83263B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263BA0 size=56
    let mut pc: u32 = 0x83263BA0;
    'dispatch: loop {
        match pc {
            0x83263BA0 => {
    //   block [0x83263BA0..0x83263BD8)
	// 83263BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263BB4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83263BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263BBC: 4AF9019D  bl 0x821f3d58
	ctx.lr = 0x83263BC0;
	sub_821F3D58(ctx, base);
	// 83263BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263BC4: 906AB544  stw r3, -0x4abc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19132 as u32), ctx.r[3].u32 ) };
	// 83263BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263BD8 size=56
    let mut pc: u32 = 0x83263BD8;
    'dispatch: loop {
        match pc {
            0x83263BD8 => {
    //   block [0x83263BD8..0x83263C10)
	// 83263BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263BEC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83263BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263BF4: 4AF90165  bl 0x821f3d58
	ctx.lr = 0x83263BF8;
	sub_821F3D58(ctx, base);
	// 83263BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263BFC: 906AB548  stw r3, -0x4ab8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19128 as u32), ctx.r[3].u32 ) };
	// 83263C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263C10 size=56
    let mut pc: u32 = 0x83263C10;
    'dispatch: loop {
        match pc {
            0x83263C10 => {
    //   block [0x83263C10..0x83263C48)
	// 83263C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263C24: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83263C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263C2C: 4AF9012D  bl 0x821f3d58
	ctx.lr = 0x83263C30;
	sub_821F3D58(ctx, base);
	// 83263C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263C34: 906AB54C  stw r3, -0x4ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19124 as u32), ctx.r[3].u32 ) };
	// 83263C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263C48 size=56
    let mut pc: u32 = 0x83263C48;
    'dispatch: loop {
        match pc {
            0x83263C48 => {
    //   block [0x83263C48..0x83263C80)
	// 83263C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263C5C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83263C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263C64: 4AF900F5  bl 0x821f3d58
	ctx.lr = 0x83263C68;
	sub_821F3D58(ctx, base);
	// 83263C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263C6C: 906AB550  stw r3, -0x4ab0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19120 as u32), ctx.r[3].u32 ) };
	// 83263C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263C80 size=56
    let mut pc: u32 = 0x83263C80;
    'dispatch: loop {
        match pc {
            0x83263C80 => {
    //   block [0x83263C80..0x83263CB8)
	// 83263C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263C94: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83263C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263C9C: 4AF900BD  bl 0x821f3d58
	ctx.lr = 0x83263CA0;
	sub_821F3D58(ctx, base);
	// 83263CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263CA4: 906AB554  stw r3, -0x4aac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19116 as u32), ctx.r[3].u32 ) };
	// 83263CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263CB8 size=56
    let mut pc: u32 = 0x83263CB8;
    'dispatch: loop {
        match pc {
            0x83263CB8 => {
    //   block [0x83263CB8..0x83263CF0)
	// 83263CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263CCC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83263CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263CD4: 4AF90085  bl 0x821f3d58
	ctx.lr = 0x83263CD8;
	sub_821F3D58(ctx, base);
	// 83263CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263CDC: 906AB558  stw r3, -0x4aa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19112 as u32), ctx.r[3].u32 ) };
	// 83263CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263CF0 size=56
    let mut pc: u32 = 0x83263CF0;
    'dispatch: loop {
        match pc {
            0x83263CF0 => {
    //   block [0x83263CF0..0x83263D28)
	// 83263CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263D04: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83263D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263D0C: 4AF9004D  bl 0x821f3d58
	ctx.lr = 0x83263D10;
	sub_821F3D58(ctx, base);
	// 83263D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263D14: 906AB55C  stw r3, -0x4aa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19108 as u32), ctx.r[3].u32 ) };
	// 83263D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263D28 size=56
    let mut pc: u32 = 0x83263D28;
    'dispatch: loop {
        match pc {
            0x83263D28 => {
    //   block [0x83263D28..0x83263D60)
	// 83263D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263D3C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83263D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263D44: 4AF90015  bl 0x821f3d58
	ctx.lr = 0x83263D48;
	sub_821F3D58(ctx, base);
	// 83263D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263D4C: 906AB560  stw r3, -0x4aa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19104 as u32), ctx.r[3].u32 ) };
	// 83263D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263D60 size=56
    let mut pc: u32 = 0x83263D60;
    'dispatch: loop {
        match pc {
            0x83263D60 => {
    //   block [0x83263D60..0x83263D98)
	// 83263D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263D74: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83263D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263D7C: 4AF8FFDD  bl 0x821f3d58
	ctx.lr = 0x83263D80;
	sub_821F3D58(ctx, base);
	// 83263D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263D84: 906AB564  stw r3, -0x4a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19100 as u32), ctx.r[3].u32 ) };
	// 83263D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263D98 size=56
    let mut pc: u32 = 0x83263D98;
    'dispatch: loop {
        match pc {
            0x83263D98 => {
    //   block [0x83263D98..0x83263DD0)
	// 83263D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263DAC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83263DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263DB4: 4AF8FFA5  bl 0x821f3d58
	ctx.lr = 0x83263DB8;
	sub_821F3D58(ctx, base);
	// 83263DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263DBC: 906AB568  stw r3, -0x4a98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19096 as u32), ctx.r[3].u32 ) };
	// 83263DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263DD0 size=56
    let mut pc: u32 = 0x83263DD0;
    'dispatch: loop {
        match pc {
            0x83263DD0 => {
    //   block [0x83263DD0..0x83263E08)
	// 83263DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263DE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263DE4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83263DE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263DEC: 4AF8FF6D  bl 0x821f3d58
	ctx.lr = 0x83263DF0;
	sub_821F3D58(ctx, base);
	// 83263DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263DF4: 906AB56C  stw r3, -0x4a94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19092 as u32), ctx.r[3].u32 ) };
	// 83263DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263E08 size=56
    let mut pc: u32 = 0x83263E08;
    'dispatch: loop {
        match pc {
            0x83263E08 => {
    //   block [0x83263E08..0x83263E40)
	// 83263E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263E1C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83263E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263E24: 4AF8FF35  bl 0x821f3d58
	ctx.lr = 0x83263E28;
	sub_821F3D58(ctx, base);
	// 83263E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263E2C: 906AB570  stw r3, -0x4a90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19088 as u32), ctx.r[3].u32 ) };
	// 83263E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263E40 size=56
    let mut pc: u32 = 0x83263E40;
    'dispatch: loop {
        match pc {
            0x83263E40 => {
    //   block [0x83263E40..0x83263E78)
	// 83263E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263E54: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83263E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263E5C: 4AF8FEFD  bl 0x821f3d58
	ctx.lr = 0x83263E60;
	sub_821F3D58(ctx, base);
	// 83263E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263E64: 906AB574  stw r3, -0x4a8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19084 as u32), ctx.r[3].u32 ) };
	// 83263E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263E78 size=56
    let mut pc: u32 = 0x83263E78;
    'dispatch: loop {
        match pc {
            0x83263E78 => {
    //   block [0x83263E78..0x83263EB0)
	// 83263E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263E8C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83263E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263E94: 4AF8FEC5  bl 0x821f3d58
	ctx.lr = 0x83263E98;
	sub_821F3D58(ctx, base);
	// 83263E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263E9C: 906AB578  stw r3, -0x4a88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19080 as u32), ctx.r[3].u32 ) };
	// 83263EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263EB0 size=56
    let mut pc: u32 = 0x83263EB0;
    'dispatch: loop {
        match pc {
            0x83263EB0 => {
    //   block [0x83263EB0..0x83263EE8)
	// 83263EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263EC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263EC4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83263EC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263ECC: 4AF8FE8D  bl 0x821f3d58
	ctx.lr = 0x83263ED0;
	sub_821F3D58(ctx, base);
	// 83263ED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263ED4: 906AB57C  stw r3, -0x4a84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19076 as u32), ctx.r[3].u32 ) };
	// 83263ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263EE8 size=56
    let mut pc: u32 = 0x83263EE8;
    'dispatch: loop {
        match pc {
            0x83263EE8 => {
    //   block [0x83263EE8..0x83263F20)
	// 83263EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263EF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263EF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263EFC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83263F00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263F04: 4AF8FE55  bl 0x821f3d58
	ctx.lr = 0x83263F08;
	sub_821F3D58(ctx, base);
	// 83263F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263F0C: 906AB580  stw r3, -0x4a80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19072 as u32), ctx.r[3].u32 ) };
	// 83263F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263F20 size=56
    let mut pc: u32 = 0x83263F20;
    'dispatch: loop {
        match pc {
            0x83263F20 => {
    //   block [0x83263F20..0x83263F58)
	// 83263F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263F2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263F30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263F34: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83263F38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263F3C: 4AF8FE1D  bl 0x821f3d58
	ctx.lr = 0x83263F40;
	sub_821F3D58(ctx, base);
	// 83263F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263F44: 906AB584  stw r3, -0x4a7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19068 as u32), ctx.r[3].u32 ) };
	// 83263F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263F58 size=56
    let mut pc: u32 = 0x83263F58;
    'dispatch: loop {
        match pc {
            0x83263F58 => {
    //   block [0x83263F58..0x83263F90)
	// 83263F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263F64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263F68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263F6C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83263F70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263F74: 4AF8FDE5  bl 0x821f3d58
	ctx.lr = 0x83263F78;
	sub_821F3D58(ctx, base);
	// 83263F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263F7C: 906AB588  stw r3, -0x4a78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19064 as u32), ctx.r[3].u32 ) };
	// 83263F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263F90 size=56
    let mut pc: u32 = 0x83263F90;
    'dispatch: loop {
        match pc {
            0x83263F90 => {
    //   block [0x83263F90..0x83263FC8)
	// 83263F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263F9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263FA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263FA4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83263FA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263FAC: 4AF8FDAD  bl 0x821f3d58
	ctx.lr = 0x83263FB0;
	sub_821F3D58(ctx, base);
	// 83263FB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263FB4: 906AB58C  stw r3, -0x4a74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19060 as u32), ctx.r[3].u32 ) };
	// 83263FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263FC8 size=56
    let mut pc: u32 = 0x83263FC8;
    'dispatch: loop {
        match pc {
            0x83263FC8 => {
    //   block [0x83263FC8..0x83264000)
	// 83263FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263FD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263FD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263FDC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83263FE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263FE4: 4AF8FD75  bl 0x821f3d58
	ctx.lr = 0x83263FE8;
	sub_821F3D58(ctx, base);
	// 83263FE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263FEC: 906AB590  stw r3, -0x4a70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19056 as u32), ctx.r[3].u32 ) };
	// 83263FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264000 size=52
    let mut pc: u32 = 0x83264000;
    'dispatch: loop {
        match pc {
            0x83264000 => {
    //   block [0x83264000..0x83264034)
	// 83264000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326400C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264010: 386B0454  addi r3, r11, 0x454
	ctx.r[3].s64 = ctx.r[11].s64 + 1108;
	// 83264014: 4AF2512D  bl 0x82189140
	ctx.lr = 0x83264018;
	sub_82189140(ctx, base);
	// 83264018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326401C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264020: 916AB594  stw r11, -0x4a6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19052 as u32), ctx.r[11].u32 ) };
	// 83264024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326402C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264038 size=52
    let mut pc: u32 = 0x83264038;
    'dispatch: loop {
        match pc {
            0x83264038 => {
    //   block [0x83264038..0x8326406C)
	// 83264038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326403C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264044: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264048: 386B0464  addi r3, r11, 0x464
	ctx.r[3].s64 = ctx.r[11].s64 + 1124;
	// 8326404C: 4AF250F5  bl 0x82189140
	ctx.lr = 0x83264050;
	sub_82189140(ctx, base);
	// 83264050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264054: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264058: 916AB598  stw r11, -0x4a68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19048 as u32), ctx.r[11].u32 ) };
	// 8326405C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264070 size=52
    let mut pc: u32 = 0x83264070;
    'dispatch: loop {
        match pc {
            0x83264070 => {
    //   block [0x83264070..0x832640A4)
	// 83264070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326407C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264080: 386B0470  addi r3, r11, 0x470
	ctx.r[3].s64 = ctx.r[11].s64 + 1136;
	// 83264084: 4AF250BD  bl 0x82189140
	ctx.lr = 0x83264088;
	sub_82189140(ctx, base);
	// 83264088: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326408C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264090: 916AB59C  stw r11, -0x4a64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19044 as u32), ctx.r[11].u32 ) };
	// 83264094: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326409C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832640A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832640A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832640A8 size=52
    let mut pc: u32 = 0x832640A8;
    'dispatch: loop {
        match pc {
            0x832640A8 => {
    //   block [0x832640A8..0x832640DC)
	// 832640A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832640AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832640B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832640B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832640B8: 386B047C  addi r3, r11, 0x47c
	ctx.r[3].s64 = ctx.r[11].s64 + 1148;
	// 832640BC: 4AF25085  bl 0x82189140
	ctx.lr = 0x832640C0;
	sub_82189140(ctx, base);
	// 832640C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832640C4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832640C8: 916AB5A0  stw r11, -0x4a60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19040 as u32), ctx.r[11].u32 ) };
	// 832640CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832640D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832640D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832640D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832640E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832640E0 size=52
    let mut pc: u32 = 0x832640E0;
    'dispatch: loop {
        match pc {
            0x832640E0 => {
    //   block [0x832640E0..0x83264114)
	// 832640E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832640E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832640E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832640EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832640F0: 386B048C  addi r3, r11, 0x48c
	ctx.r[3].s64 = ctx.r[11].s64 + 1164;
	// 832640F4: 4AF2504D  bl 0x82189140
	ctx.lr = 0x832640F8;
	sub_82189140(ctx, base);
	// 832640F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832640FC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264100: 916AB5A4  stw r11, -0x4a5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19036 as u32), ctx.r[11].u32 ) };
	// 83264104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326410C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264118 size=52
    let mut pc: u32 = 0x83264118;
    'dispatch: loop {
        match pc {
            0x83264118 => {
    //   block [0x83264118..0x8326414C)
	// 83264118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264124: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264128: 386B049C  addi r3, r11, 0x49c
	ctx.r[3].s64 = ctx.r[11].s64 + 1180;
	// 8326412C: 4AF25015  bl 0x82189140
	ctx.lr = 0x83264130;
	sub_82189140(ctx, base);
	// 83264130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264134: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264138: 916AB5A8  stw r11, -0x4a58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19032 as u32), ctx.r[11].u32 ) };
	// 8326413C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264150 size=52
    let mut pc: u32 = 0x83264150;
    'dispatch: loop {
        match pc {
            0x83264150 => {
    //   block [0x83264150..0x83264184)
	// 83264150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326415C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264160: 386B04B0  addi r3, r11, 0x4b0
	ctx.r[3].s64 = ctx.r[11].s64 + 1200;
	// 83264164: 4AF24FDD  bl 0x82189140
	ctx.lr = 0x83264168;
	sub_82189140(ctx, base);
	// 83264168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326416C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264170: 916AB5AC  stw r11, -0x4a54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19028 as u32), ctx.r[11].u32 ) };
	// 83264174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326417C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264188 size=52
    let mut pc: u32 = 0x83264188;
    'dispatch: loop {
        match pc {
            0x83264188 => {
    //   block [0x83264188..0x832641BC)
	// 83264188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326418C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264194: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264198: 386B04C4  addi r3, r11, 0x4c4
	ctx.r[3].s64 = ctx.r[11].s64 + 1220;
	// 8326419C: 4AF24FA5  bl 0x82189140
	ctx.lr = 0x832641A0;
	sub_82189140(ctx, base);
	// 832641A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832641A4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832641A8: 916AB5B0  stw r11, -0x4a50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19024 as u32), ctx.r[11].u32 ) };
	// 832641AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832641B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832641B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832641B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832641C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832641C0 size=52
    let mut pc: u32 = 0x832641C0;
    'dispatch: loop {
        match pc {
            0x832641C0 => {
    //   block [0x832641C0..0x832641F4)
	// 832641C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832641C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832641C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832641CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832641D0: 386B04D0  addi r3, r11, 0x4d0
	ctx.r[3].s64 = ctx.r[11].s64 + 1232;
	// 832641D4: 4AF24F6D  bl 0x82189140
	ctx.lr = 0x832641D8;
	sub_82189140(ctx, base);
	// 832641D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832641DC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832641E0: 916AB5B4  stw r11, -0x4a4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19020 as u32), ctx.r[11].u32 ) };
	// 832641E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832641E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832641EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832641F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832641F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832641F8 size=64
    let mut pc: u32 = 0x832641F8;
    'dispatch: loop {
        match pc {
            0x832641F8 => {
    //   block [0x832641F8..0x83264238)
	// 832641F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832641FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264204: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326420C: 388B04E0  addi r4, r11, 0x4e0
	ctx.r[4].s64 = ctx.r[11].s64 + 1248;
	// 83264210: 386AB5B8  addi r3, r10, -0x4a48
	ctx.r[3].s64 = ctx.r[10].s64 + -19016;
	// 83264214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264218: 4AFC8CB9  bl 0x8222ced0
	ctx.lr = 0x8326421C;
	sub_8222CED0(ctx, base);
	// 8326421C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264220: 3869CE60  addi r3, r9, -0x31a0
	ctx.r[3].s64 = ctx.r[9].s64 + -12704;
	// 83264224: 4BA45CFD  bl 0x82ca9f20
	ctx.lr = 0x83264228;
	sub_82CA9F20(ctx, base);
	// 83264228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326422C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264238 size=64
    let mut pc: u32 = 0x83264238;
    'dispatch: loop {
        match pc {
            0x83264238 => {
    //   block [0x83264238..0x83264278)
	// 83264238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326423C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264244: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326424C: 388B3404  addi r4, r11, 0x3404
	ctx.r[4].s64 = ctx.r[11].s64 + 13316;
	// 83264250: 386AB5BC  addi r3, r10, -0x4a44
	ctx.r[3].s64 = ctx.r[10].s64 + -19012;
	// 83264254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264258: 4AFC8C79  bl 0x8222ced0
	ctx.lr = 0x8326425C;
	sub_8222CED0(ctx, base);
	// 8326425C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264260: 3869CE70  addi r3, r9, -0x3190
	ctx.r[3].s64 = ctx.r[9].s64 + -12688;
	// 83264264: 4BA45CBD  bl 0x82ca9f20
	ctx.lr = 0x83264268;
	sub_82CA9F20(ctx, base);
	// 83264268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326426C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264278 size=64
    let mut pc: u32 = 0x83264278;
    'dispatch: loop {
        match pc {
            0x83264278 => {
    //   block [0x83264278..0x832642B8)
	// 83264278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326427C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264284: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326428C: 388B3410  addi r4, r11, 0x3410
	ctx.r[4].s64 = ctx.r[11].s64 + 13328;
	// 83264290: 386AB5C0  addi r3, r10, -0x4a40
	ctx.r[3].s64 = ctx.r[10].s64 + -19008;
	// 83264294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264298: 4AFC8C39  bl 0x8222ced0
	ctx.lr = 0x8326429C;
	sub_8222CED0(ctx, base);
	// 8326429C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832642A0: 3869CE80  addi r3, r9, -0x3180
	ctx.r[3].s64 = ctx.r[9].s64 + -12672;
	// 832642A4: 4BA45C7D  bl 0x82ca9f20
	ctx.lr = 0x832642A8;
	sub_82CA9F20(ctx, base);
	// 832642A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832642AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832642B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832642B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832642B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832642B8 size=64
    let mut pc: u32 = 0x832642B8;
    'dispatch: loop {
        match pc {
            0x832642B8 => {
    //   block [0x832642B8..0x832642F8)
	// 832642B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832642BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832642C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832642C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832642C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832642CC: 388B04EC  addi r4, r11, 0x4ec
	ctx.r[4].s64 = ctx.r[11].s64 + 1260;
	// 832642D0: 386AB5C4  addi r3, r10, -0x4a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19004;
	// 832642D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832642D8: 4AFC8BF9  bl 0x8222ced0
	ctx.lr = 0x832642DC;
	sub_8222CED0(ctx, base);
	// 832642DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832642E0: 3869CE90  addi r3, r9, -0x3170
	ctx.r[3].s64 = ctx.r[9].s64 + -12656;
	// 832642E4: 4BA45C3D  bl 0x82ca9f20
	ctx.lr = 0x832642E8;
	sub_82CA9F20(ctx, base);
	// 832642E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832642EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832642F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832642F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832642F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832642F8 size=196
    let mut pc: u32 = 0x832642F8;
    'dispatch: loop {
        match pc {
            0x832642F8 => {
    //   block [0x832642F8..0x832643BC)
	// 832642F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832642FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83264304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264308: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326430C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83264310: 3BEBB5C8  addi r31, r11, -0x4a38
	ctx.r[31].s64 = ctx.r[11].s64 + -19000;
	// 83264314: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83264318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326431C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264320: 4AFC8BB1  bl 0x8222ced0
	ctx.lr = 0x83264324;
	sub_8222CED0(ctx, base);
	// 83264324: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83264328: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326432C: 388934F8  addi r4, r9, 0x34f8
	ctx.r[4].s64 = ctx.r[9].s64 + 13560;
	// 83264330: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83264334: 4AFC8B9D  bl 0x8222ced0
	ctx.lr = 0x83264338;
	sub_8222CED0(ctx, base);
	// 83264338: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8326433C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264340: 388834D0  addi r4, r8, 0x34d0
	ctx.r[4].s64 = ctx.r[8].s64 + 13520;
	// 83264344: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83264348: 4AFC8B89  bl 0x8222ced0
	ctx.lr = 0x8326434C;
	sub_8222CED0(ctx, base);
	// 8326434C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83264350: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264354: 388734A8  addi r4, r7, 0x34a8
	ctx.r[4].s64 = ctx.r[7].s64 + 13480;
	// 83264358: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8326435C: 4AFC8B75  bl 0x8222ced0
	ctx.lr = 0x83264360;
	sub_8222CED0(ctx, base);
	// 83264360: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83264364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264368: 38863480  addi r4, r6, 0x3480
	ctx.r[4].s64 = ctx.r[6].s64 + 13440;
	// 8326436C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83264370: 4AFC8B61  bl 0x8222ced0
	ctx.lr = 0x83264374;
	sub_8222CED0(ctx, base);
	// 83264374: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83264378: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326437C: 38843458  addi r4, r4, 0x3458
	ctx.r[4].s64 = ctx.r[4].s64 + 13400;
	// 83264380: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83264384: 4AFC8B4D  bl 0x8222ced0
	ctx.lr = 0x83264388;
	sub_8222CED0(ctx, base);
	// 83264388: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8326438C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264390: 38833430  addi r4, r3, 0x3430
	ctx.r[4].s64 = ctx.r[3].s64 + 13360;
	// 83264394: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83264398: 4AFC8B39  bl 0x8222ced0
	ctx.lr = 0x8326439C;
	sub_8222CED0(ctx, base);
	// 8326439C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832643A0: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 832643A4: 4BA45B7D  bl 0x82ca9f20
	ctx.lr = 0x832643A8;
	sub_82CA9F20(ctx, base);
	// 832643A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832643AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832643B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832643B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832643B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832643C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832643C0 size=196
    let mut pc: u32 = 0x832643C0;
    'dispatch: loop {
        match pc {
            0x832643C0 => {
    //   block [0x832643C0..0x83264484)
	// 832643C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832643C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832643C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832643CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832643D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832643D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832643D8: 3BEBB5E4  addi r31, r11, -0x4a1c
	ctx.r[31].s64 = ctx.r[11].s64 + -18972;
	// 832643DC: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 832643E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832643E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832643E8: 4AFC8AE9  bl 0x8222ced0
	ctx.lr = 0x832643EC;
	sub_8222CED0(ctx, base);
	// 832643EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832643F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832643F4: 38893598  addi r4, r9, 0x3598
	ctx.r[4].s64 = ctx.r[9].s64 + 13720;
	// 832643F8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832643FC: 4AFC8AD5  bl 0x8222ced0
	ctx.lr = 0x83264400;
	sub_8222CED0(ctx, base);
	// 83264400: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83264404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264408: 38883580  addi r4, r8, 0x3580
	ctx.r[4].s64 = ctx.r[8].s64 + 13696;
	// 8326440C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83264410: 4AFC8AC1  bl 0x8222ced0
	ctx.lr = 0x83264414;
	sub_8222CED0(ctx, base);
	// 83264414: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83264418: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326441C: 38873568  addi r4, r7, 0x3568
	ctx.r[4].s64 = ctx.r[7].s64 + 13672;
	// 83264420: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83264424: 4AFC8AAD  bl 0x8222ced0
	ctx.lr = 0x83264428;
	sub_8222CED0(ctx, base);
	// 83264428: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8326442C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264430: 38863550  addi r4, r6, 0x3550
	ctx.r[4].s64 = ctx.r[6].s64 + 13648;
	// 83264434: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83264438: 4AFC8A99  bl 0x8222ced0
	ctx.lr = 0x8326443C;
	sub_8222CED0(ctx, base);
	// 8326443C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83264440: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264444: 38843538  addi r4, r4, 0x3538
	ctx.r[4].s64 = ctx.r[4].s64 + 13624;
	// 83264448: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8326444C: 4AFC8A85  bl 0x8222ced0
	ctx.lr = 0x83264450;
	sub_8222CED0(ctx, base);
	// 83264450: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83264454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264458: 38833520  addi r4, r3, 0x3520
	ctx.r[4].s64 = ctx.r[3].s64 + 13600;
	// 8326445C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83264460: 4AFC8A71  bl 0x8222ced0
	ctx.lr = 0x83264464;
	sub_8222CED0(ctx, base);
	// 83264464: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83264468: 386BCF08  addi r3, r11, -0x30f8
	ctx.r[3].s64 = ctx.r[11].s64 + -12536;
	// 8326446C: 4BA45AB5  bl 0x82ca9f20
	ctx.lr = 0x83264470;
	sub_82CA9F20(ctx, base);
	// 83264470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326447C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83264480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264488 size=192
    let mut pc: u32 = 0x83264488;
    'dispatch: loop {
        match pc {
            0x83264488 => {
    //   block [0x83264488..0x832644E0)
	// 83264488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83264494: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264498: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326449C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832644A0: 388B3390  addi r4, r11, 0x3390
	ctx.r[4].s64 = ctx.r[11].s64 + 13200;
	// 832644A4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832644A8: 4AFC8A29  bl 0x8222ced0
	ctx.lr = 0x832644AC;
	sub_8222CED0(ctx, base);
	// 832644AC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832644B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832644B4: 4AF8A685  bl 0x821eeb38
	ctx.lr = 0x832644B8;
	sub_821EEB38(ctx, base);
	// 832644B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832644BC: 4B99F335  bl 0x82c037f0
	ctx.lr = 0x832644C0;
	sub_82C037F0(ctx, base);
	// 832644C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832644C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832644C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832644CC: 916AB600  stw r11, -0x4a00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18944 as u32), ctx.r[11].u32 ) };
	// 832644D0: 4AF62299  bl 0x821c6768
	ctx.lr = 0x832644D4;
	sub_821C6768(ctx, base);
	// 832644D4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832644D8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832644DC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832644E0; continue 'dispatch;
            }
            0x832644E0 => {
    //   block [0x832644E0..0x8326450C)
	// 832644E0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832644E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832644E8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832644EC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832644F0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832644F4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832644F8: 4082FFE8  bne 0x832644e0
	if !ctx.cr[0].eq {
	pc = 0x832644E0; continue 'dispatch;
	}
	// 832644FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83264500: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264504: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83264508: 4AF62261  bl 0x821c6768
	ctx.lr = 0x8326450C;
	sub_821C6768(ctx, base);
	pc = 0x8326450C; continue 'dispatch;
            }
            0x8326450C => {
    //   block [0x8326450C..0x83264548)
	// 8326450C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83264510: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264514: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83264518: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8326451C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83264520: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264524: 4082FFE8  bne 0x8326450c
	if !ctx.cr[0].eq {
	pc = 0x8326450C; continue 'dispatch;
	}
	// 83264528: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326452C: 386BCF70  addi r3, r11, -0x3090
	ctx.r[3].s64 = ctx.r[11].s64 + -12432;
	// 83264530: 4BA459F1  bl 0x82ca9f20
	ctx.lr = 0x83264534;
	sub_82CA9F20(ctx, base);
	// 83264534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83264538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326453C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264540: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83264544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264548 size=192
    let mut pc: u32 = 0x83264548;
    'dispatch: loop {
        match pc {
            0x83264548 => {
    //   block [0x83264548..0x832645A0)
	// 83264548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326454C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83264554: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264558: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326455C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264560: 388B04F4  addi r4, r11, 0x4f4
	ctx.r[4].s64 = ctx.r[11].s64 + 1268;
	// 83264564: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264568: 4AFC8969  bl 0x8222ced0
	ctx.lr = 0x8326456C;
	sub_8222CED0(ctx, base);
	// 8326456C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83264570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264574: 4AF8A5C5  bl 0x821eeb38
	ctx.lr = 0x83264578;
	sub_821EEB38(ctx, base);
	// 83264578: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326457C: 4B99F275  bl 0x82c037f0
	ctx.lr = 0x83264580;
	sub_82C037F0(ctx, base);
	// 83264580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264584: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83264588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8326458C: 916AB604  stw r11, -0x49fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18940 as u32), ctx.r[11].u32 ) };
	// 83264590: 4AF621D9  bl 0x821c6768
	ctx.lr = 0x83264594;
	sub_821C6768(ctx, base);
	// 83264594: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83264598: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8326459C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832645A0; continue 'dispatch;
            }
            0x832645A0 => {
    //   block [0x832645A0..0x832645CC)
	// 832645A0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832645A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832645A8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832645AC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832645B0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832645B4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832645B8: 4082FFE8  bne 0x832645a0
	if !ctx.cr[0].eq {
	pc = 0x832645A0; continue 'dispatch;
	}
	// 832645BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832645C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832645C4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832645C8: 4AF621A1  bl 0x821c6768
	ctx.lr = 0x832645CC;
	sub_821C6768(ctx, base);
	pc = 0x832645CC; continue 'dispatch;
            }
            0x832645CC => {
    //   block [0x832645CC..0x83264608)
	// 832645CC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 832645D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832645D4: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832645D8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832645DC: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832645E0: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832645E4: 4082FFE8  bne 0x832645cc
	if !ctx.cr[0].eq {
	pc = 0x832645CC; continue 'dispatch;
	}
	// 832645E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832645EC: 386BCF78  addi r3, r11, -0x3088
	ctx.r[3].s64 = ctx.r[11].s64 + -12424;
	// 832645F0: 4BA45931  bl 0x82ca9f20
	ctx.lr = 0x832645F4;
	sub_82CA9F20(ctx, base);
	// 832645F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832645F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832645FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83264604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264608 size=64
    let mut pc: u32 = 0x83264608;
    'dispatch: loop {
        match pc {
            0x83264608 => {
    //   block [0x83264608..0x83264648)
	// 83264608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264614: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326461C: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 83264620: 386AB608  addi r3, r10, -0x49f8
	ctx.r[3].s64 = ctx.r[10].s64 + -18936;
	// 83264624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264628: 4AFC88A9  bl 0x8222ced0
	ctx.lr = 0x8326462C;
	sub_8222CED0(ctx, base);
	// 8326462C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264630: 3869CF80  addi r3, r9, -0x3080
	ctx.r[3].s64 = ctx.r[9].s64 + -12416;
	// 83264634: 4BA458ED  bl 0x82ca9f20
	ctx.lr = 0x83264638;
	sub_82CA9F20(ctx, base);
	// 83264638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326463C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264648 size=64
    let mut pc: u32 = 0x83264648;
    'dispatch: loop {
        match pc {
            0x83264648 => {
    //   block [0x83264648..0x83264688)
	// 83264648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264654: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326465C: 388B0510  addi r4, r11, 0x510
	ctx.r[4].s64 = ctx.r[11].s64 + 1296;
	// 83264660: 386AB60C  addi r3, r10, -0x49f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18932;
	// 83264664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264668: 4AFC8869  bl 0x8222ced0
	ctx.lr = 0x8326466C;
	sub_8222CED0(ctx, base);
	// 8326466C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264670: 3869CF90  addi r3, r9, -0x3070
	ctx.r[3].s64 = ctx.r[9].s64 + -12400;
	// 83264674: 4BA458AD  bl 0x82ca9f20
	ctx.lr = 0x83264678;
	sub_82CA9F20(ctx, base);
	// 83264678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326467C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264688 size=64
    let mut pc: u32 = 0x83264688;
    'dispatch: loop {
        match pc {
            0x83264688 => {
    //   block [0x83264688..0x832646C8)
	// 83264688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326468C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264694: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326469C: 388B0530  addi r4, r11, 0x530
	ctx.r[4].s64 = ctx.r[11].s64 + 1328;
	// 832646A0: 386AB610  addi r3, r10, -0x49f0
	ctx.r[3].s64 = ctx.r[10].s64 + -18928;
	// 832646A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832646A8: 4AFC8829  bl 0x8222ced0
	ctx.lr = 0x832646AC;
	sub_8222CED0(ctx, base);
	// 832646AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832646B0: 3869CFA0  addi r3, r9, -0x3060
	ctx.r[3].s64 = ctx.r[9].s64 + -12384;
	// 832646B4: 4BA4586D  bl 0x82ca9f20
	ctx.lr = 0x832646B8;
	sub_82CA9F20(ctx, base);
	// 832646B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832646BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832646C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832646C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832646C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832646C8 size=64
    let mut pc: u32 = 0x832646C8;
    'dispatch: loop {
        match pc {
            0x832646C8 => {
    //   block [0x832646C8..0x83264708)
	// 832646C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832646CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832646D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832646D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832646D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832646DC: 388B0554  addi r4, r11, 0x554
	ctx.r[4].s64 = ctx.r[11].s64 + 1364;
	// 832646E0: 386AB614  addi r3, r10, -0x49ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18924;
	// 832646E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832646E8: 4AFC87E9  bl 0x8222ced0
	ctx.lr = 0x832646EC;
	sub_8222CED0(ctx, base);
	// 832646EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832646F0: 3869CFB0  addi r3, r9, -0x3050
	ctx.r[3].s64 = ctx.r[9].s64 + -12368;
	// 832646F4: 4BA4582D  bl 0x82ca9f20
	ctx.lr = 0x832646F8;
	sub_82CA9F20(ctx, base);
	// 832646F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832646FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264708 size=64
    let mut pc: u32 = 0x83264708;
    'dispatch: loop {
        match pc {
            0x83264708 => {
    //   block [0x83264708..0x83264748)
	// 83264708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326470C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264714: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264718: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326471C: 388B92E4  addi r4, r11, -0x6d1c
	ctx.r[4].s64 = ctx.r[11].s64 + -27932;
	// 83264720: 386AB618  addi r3, r10, -0x49e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18920;
	// 83264724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264728: 4AFC87A9  bl 0x8222ced0
	ctx.lr = 0x8326472C;
	sub_8222CED0(ctx, base);
	// 8326472C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264730: 3869CFC0  addi r3, r9, -0x3040
	ctx.r[3].s64 = ctx.r[9].s64 + -12352;
	// 83264734: 4BA457ED  bl 0x82ca9f20
	ctx.lr = 0x83264738;
	sub_82CA9F20(ctx, base);
	// 83264738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326473C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264748 size=64
    let mut pc: u32 = 0x83264748;
    'dispatch: loop {
        match pc {
            0x83264748 => {
    //   block [0x83264748..0x83264788)
	// 83264748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264754: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264758: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326475C: 388B0578  addi r4, r11, 0x578
	ctx.r[4].s64 = ctx.r[11].s64 + 1400;
	// 83264760: 386AB61C  addi r3, r10, -0x49e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18916;
	// 83264764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264768: 4AFC8769  bl 0x8222ced0
	ctx.lr = 0x8326476C;
	sub_8222CED0(ctx, base);
	// 8326476C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264770: 3869CFD0  addi r3, r9, -0x3030
	ctx.r[3].s64 = ctx.r[9].s64 + -12336;
	// 83264774: 4BA457AD  bl 0x82ca9f20
	ctx.lr = 0x83264778;
	sub_82CA9F20(ctx, base);
	// 83264778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326477C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264788 size=64
    let mut pc: u32 = 0x83264788;
    'dispatch: loop {
        match pc {
            0x83264788 => {
    //   block [0x83264788..0x832647C8)
	// 83264788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326478C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264794: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264798: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326479C: 388B0598  addi r4, r11, 0x598
	ctx.r[4].s64 = ctx.r[11].s64 + 1432;
	// 832647A0: 386AB620  addi r3, r10, -0x49e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18912;
	// 832647A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832647A8: 4AFC8729  bl 0x8222ced0
	ctx.lr = 0x832647AC;
	sub_8222CED0(ctx, base);
	// 832647AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832647B0: 3869CFE0  addi r3, r9, -0x3020
	ctx.r[3].s64 = ctx.r[9].s64 + -12320;
	// 832647B4: 4BA4576D  bl 0x82ca9f20
	ctx.lr = 0x832647B8;
	sub_82CA9F20(ctx, base);
	// 832647B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832647BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832647C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832647C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832647C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832647C8 size=64
    let mut pc: u32 = 0x832647C8;
    'dispatch: loop {
        match pc {
            0x832647C8 => {
    //   block [0x832647C8..0x83264808)
	// 832647C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832647CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832647D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832647D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832647D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832647DC: 388B05B8  addi r4, r11, 0x5b8
	ctx.r[4].s64 = ctx.r[11].s64 + 1464;
	// 832647E0: 386AB624  addi r3, r10, -0x49dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18908;
	// 832647E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832647E8: 4AFC86E9  bl 0x8222ced0
	ctx.lr = 0x832647EC;
	sub_8222CED0(ctx, base);
	// 832647EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832647F0: 3869CFF0  addi r3, r9, -0x3010
	ctx.r[3].s64 = ctx.r[9].s64 + -12304;
	// 832647F4: 4BA4572D  bl 0x82ca9f20
	ctx.lr = 0x832647F8;
	sub_82CA9F20(ctx, base);
	// 832647F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832647FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264808 size=64
    let mut pc: u32 = 0x83264808;
    'dispatch: loop {
        match pc {
            0x83264808 => {
    //   block [0x83264808..0x83264848)
	// 83264808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264814: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326481C: 388B05DC  addi r4, r11, 0x5dc
	ctx.r[4].s64 = ctx.r[11].s64 + 1500;
	// 83264820: 386AB628  addi r3, r10, -0x49d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18904;
	// 83264824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264828: 4AFC86A9  bl 0x8222ced0
	ctx.lr = 0x8326482C;
	sub_8222CED0(ctx, base);
	// 8326482C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264830: 3869D000  addi r3, r9, -0x3000
	ctx.r[3].s64 = ctx.r[9].s64 + -12288;
	// 83264834: 4BA456ED  bl 0x82ca9f20
	ctx.lr = 0x83264838;
	sub_82CA9F20(ctx, base);
	// 83264838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326483C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264848 size=64
    let mut pc: u32 = 0x83264848;
    'dispatch: loop {
        match pc {
            0x83264848 => {
    //   block [0x83264848..0x83264888)
	// 83264848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326484C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264854: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326485C: 388B3C38  addi r4, r11, 0x3c38
	ctx.r[4].s64 = ctx.r[11].s64 + 15416;
	// 83264860: 386AB62C  addi r3, r10, -0x49d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18900;
	// 83264864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264868: 4AFC8669  bl 0x8222ced0
	ctx.lr = 0x8326486C;
	sub_8222CED0(ctx, base);
	// 8326486C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264870: 3869D010  addi r3, r9, -0x2ff0
	ctx.r[3].s64 = ctx.r[9].s64 + -12272;
	// 83264874: 4BA456AD  bl 0x82ca9f20
	ctx.lr = 0x83264878;
	sub_82CA9F20(ctx, base);
	// 83264878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326487C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264888 size=64
    let mut pc: u32 = 0x83264888;
    'dispatch: loop {
        match pc {
            0x83264888 => {
    //   block [0x83264888..0x832648C8)
	// 83264888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326488C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264894: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264898: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326489C: 388B3B54  addi r4, r11, 0x3b54
	ctx.r[4].s64 = ctx.r[11].s64 + 15188;
	// 832648A0: 386AB630  addi r3, r10, -0x49d0
	ctx.r[3].s64 = ctx.r[10].s64 + -18896;
	// 832648A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832648A8: 4AFC8629  bl 0x8222ced0
	ctx.lr = 0x832648AC;
	sub_8222CED0(ctx, base);
	// 832648AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832648B0: 3869D020  addi r3, r9, -0x2fe0
	ctx.r[3].s64 = ctx.r[9].s64 + -12256;
	// 832648B4: 4BA4566D  bl 0x82ca9f20
	ctx.lr = 0x832648B8;
	sub_82CA9F20(ctx, base);
	// 832648B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832648BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832648C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832648C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832648C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832648C8 size=64
    let mut pc: u32 = 0x832648C8;
    'dispatch: loop {
        match pc {
            0x832648C8 => {
    //   block [0x832648C8..0x83264908)
	// 832648C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832648CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832648D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832648D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832648D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832648DC: 388B05FC  addi r4, r11, 0x5fc
	ctx.r[4].s64 = ctx.r[11].s64 + 1532;
	// 832648E0: 386AB634  addi r3, r10, -0x49cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18892;
	// 832648E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832648E8: 4AFC85E9  bl 0x8222ced0
	ctx.lr = 0x832648EC;
	sub_8222CED0(ctx, base);
	// 832648EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832648F0: 3869D030  addi r3, r9, -0x2fd0
	ctx.r[3].s64 = ctx.r[9].s64 + -12240;
	// 832648F4: 4BA4562D  bl 0x82ca9f20
	ctx.lr = 0x832648F8;
	sub_82CA9F20(ctx, base);
	// 832648F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832648FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264908 size=64
    let mut pc: u32 = 0x83264908;
    'dispatch: loop {
        match pc {
            0x83264908 => {
    //   block [0x83264908..0x83264948)
	// 83264908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326490C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264914: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264918: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326491C: 388B0624  addi r4, r11, 0x624
	ctx.r[4].s64 = ctx.r[11].s64 + 1572;
	// 83264920: 386AB638  addi r3, r10, -0x49c8
	ctx.r[3].s64 = ctx.r[10].s64 + -18888;
	// 83264924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264928: 4AFC85A9  bl 0x8222ced0
	ctx.lr = 0x8326492C;
	sub_8222CED0(ctx, base);
	// 8326492C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264930: 3869D040  addi r3, r9, -0x2fc0
	ctx.r[3].s64 = ctx.r[9].s64 + -12224;
	// 83264934: 4BA455ED  bl 0x82ca9f20
	ctx.lr = 0x83264938;
	sub_82CA9F20(ctx, base);
	// 83264938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326493C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264948 size=64
    let mut pc: u32 = 0x83264948;
    'dispatch: loop {
        match pc {
            0x83264948 => {
    //   block [0x83264948..0x83264988)
	// 83264948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326494C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264954: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264958: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326495C: 388B0634  addi r4, r11, 0x634
	ctx.r[4].s64 = ctx.r[11].s64 + 1588;
	// 83264960: 386AB63C  addi r3, r10, -0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18884;
	// 83264964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264968: 4AFC8569  bl 0x8222ced0
	ctx.lr = 0x8326496C;
	sub_8222CED0(ctx, base);
	// 8326496C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264970: 3869D050  addi r3, r9, -0x2fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -12208;
	// 83264974: 4BA455AD  bl 0x82ca9f20
	ctx.lr = 0x83264978;
	sub_82CA9F20(ctx, base);
	// 83264978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264988 size=64
    let mut pc: u32 = 0x83264988;
    'dispatch: loop {
        match pc {
            0x83264988 => {
    //   block [0x83264988..0x832649C8)
	// 83264988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264994: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326499C: 388B0958  addi r4, r11, 0x958
	ctx.r[4].s64 = ctx.r[11].s64 + 2392;
	// 832649A0: 386AB640  addi r3, r10, -0x49c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18880;
	// 832649A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832649A8: 4AFC8529  bl 0x8222ced0
	ctx.lr = 0x832649AC;
	sub_8222CED0(ctx, base);
	// 832649AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832649B0: 3869D060  addi r3, r9, -0x2fa0
	ctx.r[3].s64 = ctx.r[9].s64 + -12192;
	// 832649B4: 4BA4556D  bl 0x82ca9f20
	ctx.lr = 0x832649B8;
	sub_82CA9F20(ctx, base);
	// 832649B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832649BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832649C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832649C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832649C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832649C8 size=52
    let mut pc: u32 = 0x832649C8;
    'dispatch: loop {
        match pc {
            0x832649C8 => {
    //   block [0x832649C8..0x832649FC)
	// 832649C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832649CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832649D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832649D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832649D8: 386B0980  addi r3, r11, 0x980
	ctx.r[3].s64 = ctx.r[11].s64 + 2432;
	// 832649DC: 4AF24765  bl 0x82189140
	ctx.lr = 0x832649E0;
	sub_82189140(ctx, base);
	// 832649E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832649E4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 832649E8: 916AB644  stw r11, -0x49bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18876 as u32), ctx.r[11].u32 ) };
	// 832649EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832649F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832649F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832649F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264A00 size=52
    let mut pc: u32 = 0x83264A00;
    'dispatch: loop {
        match pc {
            0x83264A00 => {
    //   block [0x83264A00..0x83264A34)
	// 83264A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264A0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264A10: 386B098C  addi r3, r11, 0x98c
	ctx.r[3].s64 = ctx.r[11].s64 + 2444;
	// 83264A14: 4AF2472D  bl 0x82189140
	ctx.lr = 0x83264A18;
	sub_82189140(ctx, base);
	// 83264A18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264A1C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264A20: 916AB648  stw r11, -0x49b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18872 as u32), ctx.r[11].u32 ) };
	// 83264A24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264A38 size=52
    let mut pc: u32 = 0x83264A38;
    'dispatch: loop {
        match pc {
            0x83264A38 => {
    //   block [0x83264A38..0x83264A6C)
	// 83264A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264A44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264A48: 386B0998  addi r3, r11, 0x998
	ctx.r[3].s64 = ctx.r[11].s64 + 2456;
	// 83264A4C: 4AF246F5  bl 0x82189140
	ctx.lr = 0x83264A50;
	sub_82189140(ctx, base);
	// 83264A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264A54: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264A58: 916AB64C  stw r11, -0x49b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18868 as u32), ctx.r[11].u32 ) };
	// 83264A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264A70 size=64
    let mut pc: u32 = 0x83264A70;
    'dispatch: loop {
        match pc {
            0x83264A70 => {
    //   block [0x83264A70..0x83264AB0)
	// 83264A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264A7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264A84: 388B09A4  addi r4, r11, 0x9a4
	ctx.r[4].s64 = ctx.r[11].s64 + 2468;
	// 83264A88: 386AB650  addi r3, r10, -0x49b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18864;
	// 83264A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264A90: 4AFC8441  bl 0x8222ced0
	ctx.lr = 0x83264A94;
	sub_8222CED0(ctx, base);
	// 83264A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264A98: 3869D070  addi r3, r9, -0x2f90
	ctx.r[3].s64 = ctx.r[9].s64 + -12176;
	// 83264A9C: 4BA45485  bl 0x82ca9f20
	ctx.lr = 0x83264AA0;
	sub_82CA9F20(ctx, base);
	// 83264AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264AB0 size=64
    let mut pc: u32 = 0x83264AB0;
    'dispatch: loop {
        match pc {
            0x83264AB0 => {
    //   block [0x83264AB0..0x83264AF0)
	// 83264AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264ABC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264AC4: 388B04EC  addi r4, r11, 0x4ec
	ctx.r[4].s64 = ctx.r[11].s64 + 1260;
	// 83264AC8: 386AB654  addi r3, r10, -0x49ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18860;
	// 83264ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264AD0: 4AFC8401  bl 0x8222ced0
	ctx.lr = 0x83264AD4;
	sub_8222CED0(ctx, base);
	// 83264AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264AD8: 3869D080  addi r3, r9, -0x2f80
	ctx.r[3].s64 = ctx.r[9].s64 + -12160;
	// 83264ADC: 4BA45445  bl 0x82ca9f20
	ctx.lr = 0x83264AE0;
	sub_82CA9F20(ctx, base);
	// 83264AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264AF0 size=192
    let mut pc: u32 = 0x83264AF0;
    'dispatch: loop {
        match pc {
            0x83264AF0 => {
    //   block [0x83264AF0..0x83264B48)
	// 83264AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83264AFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264B00: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264B08: 388B3390  addi r4, r11, 0x3390
	ctx.r[4].s64 = ctx.r[11].s64 + 13200;
	// 83264B0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264B10: 4AFC83C1  bl 0x8222ced0
	ctx.lr = 0x83264B14;
	sub_8222CED0(ctx, base);
	// 83264B14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83264B18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264B1C: 4AF8A01D  bl 0x821eeb38
	ctx.lr = 0x83264B20;
	sub_821EEB38(ctx, base);
	// 83264B20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264B24: 4B99ECCD  bl 0x82c037f0
	ctx.lr = 0x83264B28;
	sub_82C037F0(ctx, base);
	// 83264B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264B2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83264B30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264B34: 916AB658  stw r11, -0x49a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18856 as u32), ctx.r[11].u32 ) };
	// 83264B38: 4AF61C31  bl 0x821c6768
	ctx.lr = 0x83264B3C;
	sub_821C6768(ctx, base);
	// 83264B3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83264B40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83264B44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83264B48; continue 'dispatch;
            }
            0x83264B48 => {
    //   block [0x83264B48..0x83264B74)
	// 83264B48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83264B4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264B50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83264B54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83264B58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83264B5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264B60: 4082FFE8  bne 0x83264b48
	if !ctx.cr[0].eq {
	pc = 0x83264B48; continue 'dispatch;
	}
	// 83264B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83264B68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264B6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83264B70: 4AF61BF9  bl 0x821c6768
	ctx.lr = 0x83264B74;
	sub_821C6768(ctx, base);
	pc = 0x83264B74; continue 'dispatch;
            }
            0x83264B74 => {
    //   block [0x83264B74..0x83264BB0)
	// 83264B74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83264B78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264B7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83264B80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83264B84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83264B88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264B8C: 4082FFE8  bne 0x83264b74
	if !ctx.cr[0].eq {
	pc = 0x83264B74; continue 'dispatch;
	}
	// 83264B90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83264B94: 386BD090  addi r3, r11, -0x2f70
	ctx.r[3].s64 = ctx.r[11].s64 + -12144;
	// 83264B98: 4BA45389  bl 0x82ca9f20
	ctx.lr = 0x83264B9C;
	sub_82CA9F20(ctx, base);
	// 83264B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83264BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264BA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83264BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264BB0 size=52
    let mut pc: u32 = 0x83264BB0;
    'dispatch: loop {
        match pc {
            0x83264BB0 => {
    //   block [0x83264BB0..0x83264BE4)
	// 83264BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264BBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264BC0: 386B04D0  addi r3, r11, 0x4d0
	ctx.r[3].s64 = ctx.r[11].s64 + 1232;
	// 83264BC4: 4AF2457D  bl 0x82189140
	ctx.lr = 0x83264BC8;
	sub_82189140(ctx, base);
	// 83264BC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264BCC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264BD0: 916AB65C  stw r11, -0x49a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18852 as u32), ctx.r[11].u32 ) };
	// 83264BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264BE8 size=52
    let mut pc: u32 = 0x83264BE8;
    'dispatch: loop {
        match pc {
            0x83264BE8 => {
    //   block [0x83264BE8..0x83264C1C)
	// 83264BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264BF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264BF8: 386B04C4  addi r3, r11, 0x4c4
	ctx.r[3].s64 = ctx.r[11].s64 + 1220;
	// 83264BFC: 4AF24545  bl 0x82189140
	ctx.lr = 0x83264C00;
	sub_82189140(ctx, base);
	// 83264C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264C04: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 83264C08: 916AB660  stw r11, -0x49a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18848 as u32), ctx.r[11].u32 ) };
	// 83264C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264C20 size=64
    let mut pc: u32 = 0x83264C20;
    'dispatch: loop {
        match pc {
            0x83264C20 => {
    //   block [0x83264C20..0x83264C60)
	// 83264C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83264C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264C34: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83264C38: 386AB664  addi r3, r10, -0x499c
	ctx.r[3].s64 = ctx.r[10].s64 + -18844;
	// 83264C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264C40: 4AFC8291  bl 0x8222ced0
	ctx.lr = 0x83264C44;
	sub_8222CED0(ctx, base);
	// 83264C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264C48: 3869D098  addi r3, r9, -0x2f68
	ctx.r[3].s64 = ctx.r[9].s64 + -12136;
	// 83264C4C: 4BA452D5  bl 0x82ca9f20
	ctx.lr = 0x83264C50;
	sub_82CA9F20(ctx, base);
	// 83264C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264C60 size=64
    let mut pc: u32 = 0x83264C60;
    'dispatch: loop {
        match pc {
            0x83264C60 => {
    //   block [0x83264C60..0x83264CA0)
	// 83264C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264C6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83264C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264C74: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83264C78: 386AB668  addi r3, r10, -0x4998
	ctx.r[3].s64 = ctx.r[10].s64 + -18840;
	// 83264C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264C80: 4AFC8251  bl 0x8222ced0
	ctx.lr = 0x83264C84;
	sub_8222CED0(ctx, base);
	// 83264C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264C88: 3869D0A8  addi r3, r9, -0x2f58
	ctx.r[3].s64 = ctx.r[9].s64 + -12120;
	// 83264C8C: 4BA45295  bl 0x82ca9f20
	ctx.lr = 0x83264C90;
	sub_82CA9F20(ctx, base);
	// 83264C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264CA0 size=64
    let mut pc: u32 = 0x83264CA0;
    'dispatch: loop {
        match pc {
            0x83264CA0 => {
    //   block [0x83264CA0..0x83264CE0)
	// 83264CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264CAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83264CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264CB4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83264CB8: 386AB66C  addi r3, r10, -0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + -18836;
	// 83264CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264CC0: 4AFC8211  bl 0x8222ced0
	ctx.lr = 0x83264CC4;
	sub_8222CED0(ctx, base);
	// 83264CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264CC8: 3869D0B8  addi r3, r9, -0x2f48
	ctx.r[3].s64 = ctx.r[9].s64 + -12104;
	// 83264CCC: 4BA45255  bl 0x82ca9f20
	ctx.lr = 0x83264CD0;
	sub_82CA9F20(ctx, base);
	// 83264CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264CE0 size=64
    let mut pc: u32 = 0x83264CE0;
    'dispatch: loop {
        match pc {
            0x83264CE0 => {
    //   block [0x83264CE0..0x83264D20)
	// 83264CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264CEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83264CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264CF4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83264CF8: 386AB670  addi r3, r10, -0x4990
	ctx.r[3].s64 = ctx.r[10].s64 + -18832;
	// 83264CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264D00: 4AFC81D1  bl 0x8222ced0
	ctx.lr = 0x83264D04;
	sub_8222CED0(ctx, base);
	// 83264D04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264D08: 3869D0C8  addi r3, r9, -0x2f38
	ctx.r[3].s64 = ctx.r[9].s64 + -12088;
	// 83264D0C: 4BA45215  bl 0x82ca9f20
	ctx.lr = 0x83264D10;
	sub_82CA9F20(ctx, base);
	// 83264D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264D20 size=64
    let mut pc: u32 = 0x83264D20;
    'dispatch: loop {
        match pc {
            0x83264D20 => {
    //   block [0x83264D20..0x83264D60)
	// 83264D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264D2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264D30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264D34: 388B0A08  addi r4, r11, 0xa08
	ctx.r[4].s64 = ctx.r[11].s64 + 2568;
	// 83264D38: 386AB674  addi r3, r10, -0x498c
	ctx.r[3].s64 = ctx.r[10].s64 + -18828;
	// 83264D3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264D40: 4AFC8191  bl 0x8222ced0
	ctx.lr = 0x83264D44;
	sub_8222CED0(ctx, base);
	// 83264D44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264D48: 3869D0D8  addi r3, r9, -0x2f28
	ctx.r[3].s64 = ctx.r[9].s64 + -12072;
	// 83264D4C: 4BA451D5  bl 0x82ca9f20
	ctx.lr = 0x83264D50;
	sub_82CA9F20(ctx, base);
	// 83264D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264D60 size=64
    let mut pc: u32 = 0x83264D60;
    'dispatch: loop {
        match pc {
            0x83264D60 => {
    //   block [0x83264D60..0x83264DA0)
	// 83264D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264D6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264D74: 388B0A18  addi r4, r11, 0xa18
	ctx.r[4].s64 = ctx.r[11].s64 + 2584;
	// 83264D78: 386AB678  addi r3, r10, -0x4988
	ctx.r[3].s64 = ctx.r[10].s64 + -18824;
	// 83264D7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264D80: 4AFC8151  bl 0x8222ced0
	ctx.lr = 0x83264D84;
	sub_8222CED0(ctx, base);
	// 83264D84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264D88: 3869D0E8  addi r3, r9, -0x2f18
	ctx.r[3].s64 = ctx.r[9].s64 + -12056;
	// 83264D8C: 4BA45195  bl 0x82ca9f20
	ctx.lr = 0x83264D90;
	sub_82CA9F20(ctx, base);
	// 83264D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264DA0 size=64
    let mut pc: u32 = 0x83264DA0;
    'dispatch: loop {
        match pc {
            0x83264DA0 => {
    //   block [0x83264DA0..0x83264DE0)
	// 83264DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264DAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264DB4: 388B04EC  addi r4, r11, 0x4ec
	ctx.r[4].s64 = ctx.r[11].s64 + 1260;
	// 83264DB8: 386AB67C  addi r3, r10, -0x4984
	ctx.r[3].s64 = ctx.r[10].s64 + -18820;
	// 83264DBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264DC0: 4AFC8111  bl 0x8222ced0
	ctx.lr = 0x83264DC4;
	sub_8222CED0(ctx, base);
	// 83264DC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264DC8: 3869D0F8  addi r3, r9, -0x2f08
	ctx.r[3].s64 = ctx.r[9].s64 + -12040;
	// 83264DCC: 4BA45155  bl 0x82ca9f20
	ctx.lr = 0x83264DD0;
	sub_82CA9F20(ctx, base);
	// 83264DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264DE0 size=64
    let mut pc: u32 = 0x83264DE0;
    'dispatch: loop {
        match pc {
            0x83264DE0 => {
    //   block [0x83264DE0..0x83264E20)
	// 83264DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264DEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264DF4: 388B0A28  addi r4, r11, 0xa28
	ctx.r[4].s64 = ctx.r[11].s64 + 2600;
	// 83264DF8: 386AB680  addi r3, r10, -0x4980
	ctx.r[3].s64 = ctx.r[10].s64 + -18816;
	// 83264DFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264E00: 4AFC80D1  bl 0x8222ced0
	ctx.lr = 0x83264E04;
	sub_8222CED0(ctx, base);
	// 83264E04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264E08: 3869D108  addi r3, r9, -0x2ef8
	ctx.r[3].s64 = ctx.r[9].s64 + -12024;
	// 83264E0C: 4BA45115  bl 0x82ca9f20
	ctx.lr = 0x83264E10;
	sub_82CA9F20(ctx, base);
	// 83264E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264E20 size=64
    let mut pc: u32 = 0x83264E20;
    'dispatch: loop {
        match pc {
            0x83264E20 => {
    //   block [0x83264E20..0x83264E60)
	// 83264E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264E2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264E30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264E34: 388B0A38  addi r4, r11, 0xa38
	ctx.r[4].s64 = ctx.r[11].s64 + 2616;
	// 83264E38: 386AB684  addi r3, r10, -0x497c
	ctx.r[3].s64 = ctx.r[10].s64 + -18812;
	// 83264E3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264E40: 4AFC8091  bl 0x8222ced0
	ctx.lr = 0x83264E44;
	sub_8222CED0(ctx, base);
	// 83264E44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264E48: 3869D118  addi r3, r9, -0x2ee8
	ctx.r[3].s64 = ctx.r[9].s64 + -12008;
	// 83264E4C: 4BA450D5  bl 0x82ca9f20
	ctx.lr = 0x83264E50;
	sub_82CA9F20(ctx, base);
	// 83264E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264E60 size=64
    let mut pc: u32 = 0x83264E60;
    'dispatch: loop {
        match pc {
            0x83264E60 => {
    //   block [0x83264E60..0x83264EA0)
	// 83264E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264E6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264E74: 388B0A50  addi r4, r11, 0xa50
	ctx.r[4].s64 = ctx.r[11].s64 + 2640;
	// 83264E78: 386AB688  addi r3, r10, -0x4978
	ctx.r[3].s64 = ctx.r[10].s64 + -18808;
	// 83264E7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264E80: 4AFC8051  bl 0x8222ced0
	ctx.lr = 0x83264E84;
	sub_8222CED0(ctx, base);
	// 83264E84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264E88: 3869D128  addi r3, r9, -0x2ed8
	ctx.r[3].s64 = ctx.r[9].s64 + -11992;
	// 83264E8C: 4BA45095  bl 0x82ca9f20
	ctx.lr = 0x83264E90;
	sub_82CA9F20(ctx, base);
	// 83264E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264EA0 size=64
    let mut pc: u32 = 0x83264EA0;
    'dispatch: loop {
        match pc {
            0x83264EA0 => {
    //   block [0x83264EA0..0x83264EE0)
	// 83264EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264EAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264EB4: 388B0A64  addi r4, r11, 0xa64
	ctx.r[4].s64 = ctx.r[11].s64 + 2660;
	// 83264EB8: 386AB68C  addi r3, r10, -0x4974
	ctx.r[3].s64 = ctx.r[10].s64 + -18804;
	// 83264EBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264EC0: 4AFC8011  bl 0x8222ced0
	ctx.lr = 0x83264EC4;
	sub_8222CED0(ctx, base);
	// 83264EC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83264EC8: 3869D138  addi r3, r9, -0x2ec8
	ctx.r[3].s64 = ctx.r[9].s64 + -11976;
	// 83264ECC: 4BA45055  bl 0x82ca9f20
	ctx.lr = 0x83264ED0;
	sub_82CA9F20(ctx, base);
	// 83264ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83264ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264EE0 size=192
    let mut pc: u32 = 0x83264EE0;
    'dispatch: loop {
        match pc {
            0x83264EE0 => {
    //   block [0x83264EE0..0x83264F38)
	// 83264EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264EE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83264EEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264EF0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83264EF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264EF8: 388B3390  addi r4, r11, 0x3390
	ctx.r[4].s64 = ctx.r[11].s64 + 13200;
	// 83264EFC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264F00: 4AFC7FD1  bl 0x8222ced0
	ctx.lr = 0x83264F04;
	sub_8222CED0(ctx, base);
	// 83264F04: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83264F08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264F0C: 4AF89C2D  bl 0x821eeb38
	ctx.lr = 0x83264F10;
	sub_821EEB38(ctx, base);
	// 83264F10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264F14: 4B99E8DD  bl 0x82c037f0
	ctx.lr = 0x83264F18;
	sub_82C037F0(ctx, base);
	// 83264F18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264F1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83264F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264F24: 916AB690  stw r11, -0x4970(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18800 as u32), ctx.r[11].u32 ) };
	// 83264F28: 4AF61841  bl 0x821c6768
	ctx.lr = 0x83264F2C;
	sub_821C6768(ctx, base);
	// 83264F2C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83264F30: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83264F34: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83264F38; continue 'dispatch;
            }
            0x83264F38 => {
    //   block [0x83264F38..0x83264F64)
	// 83264F38: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83264F3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264F40: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83264F44: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83264F48: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83264F4C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264F50: 4082FFE8  bne 0x83264f38
	if !ctx.cr[0].eq {
	pc = 0x83264F38; continue 'dispatch;
	}
	// 83264F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83264F58: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264F5C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83264F60: 4AF61809  bl 0x821c6768
	ctx.lr = 0x83264F64;
	sub_821C6768(ctx, base);
	pc = 0x83264F64; continue 'dispatch;
            }
            0x83264F64 => {
    //   block [0x83264F64..0x83264FA0)
	// 83264F64: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83264F68: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264F6C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83264F70: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83264F74: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83264F78: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83264F7C: 4082FFE8  bne 0x83264f64
	if !ctx.cr[0].eq {
	pc = 0x83264F64; continue 'dispatch;
	}
	// 83264F80: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83264F84: 386BD148  addi r3, r11, -0x2eb8
	ctx.r[3].s64 = ctx.r[11].s64 + -11960;
	// 83264F88: 4BA44F99  bl 0x82ca9f20
	ctx.lr = 0x83264F8C;
	sub_82CA9F20(ctx, base);
	// 83264F8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83264F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83264F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83264F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83264F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83264FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83264FA0 size=192
    let mut pc: u32 = 0x83264FA0;
    'dispatch: loop {
        match pc {
            0x83264FA0 => {
    //   block [0x83264FA0..0x83264FF8)
	// 83264FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83264FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83264FA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83264FAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83264FB0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83264FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83264FB8: 388B04F4  addi r4, r11, 0x4f4
	ctx.r[4].s64 = ctx.r[11].s64 + 1268;
	// 83264FBC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83264FC0: 4AFC7F11  bl 0x8222ced0
	ctx.lr = 0x83264FC4;
	sub_8222CED0(ctx, base);
	// 83264FC4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83264FC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264FCC: 4AF89B6D  bl 0x821eeb38
	ctx.lr = 0x83264FD0;
	sub_821EEB38(ctx, base);
	// 83264FD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264FD4: 4B99E81D  bl 0x82c037f0
	ctx.lr = 0x83264FD8;
	sub_82C037F0(ctx, base);
	// 83264FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83264FDC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83264FE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83264FE4: 916AB694  stw r11, -0x496c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18796 as u32), ctx.r[11].u32 ) };
	// 83264FE8: 4AF61781  bl 0x821c6768
	ctx.lr = 0x83264FEC;
	sub_821C6768(ctx, base);
	// 83264FEC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83264FF0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83264FF4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83264FF8; continue 'dispatch;
            }
            0x83264FF8 => {
    //   block [0x83264FF8..0x83265024)
	// 83264FF8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83264FFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83265000: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83265004: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83265008: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8326500C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83265010: 4082FFE8  bne 0x83264ff8
	if !ctx.cr[0].eq {
	pc = 0x83264FF8; continue 'dispatch;
	}
	// 83265014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83265018: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8326501C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83265020: 4AF61749  bl 0x821c6768
	ctx.lr = 0x83265024;
	sub_821C6768(ctx, base);
	pc = 0x83265024; continue 'dispatch;
            }
            0x83265024 => {
    //   block [0x83265024..0x83265060)
	// 83265024: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83265028: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326502C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83265030: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83265034: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83265038: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8326503C: 4082FFE8  bne 0x83265024
	if !ctx.cr[0].eq {
	pc = 0x83265024; continue 'dispatch;
	}
	// 83265040: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83265044: 386BD150  addi r3, r11, -0x2eb0
	ctx.r[3].s64 = ctx.r[11].s64 + -11952;
	// 83265048: 4BA44ED9  bl 0x82ca9f20
	ctx.lr = 0x8326504C;
	sub_82CA9F20(ctx, base);
	// 8326504C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83265050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326505C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83265060 size=96
    let mut pc: u32 = 0x83265060;
    'dispatch: loop {
        match pc {
            0x83265060 => {
    //   block [0x83265060..0x832650C0)
	// 83265060: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265064: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83265068: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8326506C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83265070: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83265074: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83265078: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 8326507C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83265080: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 83265084: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83265088: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8326508C: 3885B6A0  addi r4, r5, -0x4960
	ctx.r[4].s64 = ctx.r[5].s64 + -18784;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832650C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832650C0 size=96
    let mut pc: u32 = 0x832650C0;
    'dispatch: loop {
        match pc {
            0x832650C0 => {
    //   block [0x832650C0..0x83265120)
	// 832650C0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832650C4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832650C8: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 832650CC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832650D0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832650D4: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832650D8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832650DC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832650E0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83265120 size=56
    let mut pc: u32 = 0x83265120;
    'dispatch: loop {
        match pc {
            0x83265120 => {
    //   block [0x83265120..0x83265158)
	// 83265120: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265124: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83265128: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 8326512C: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83265130: 38E99160  addi r7, r9, -0x6ea0
	ctx.r[7].s64 = ctx.r[9].s64 + -28320;
	// 83265134: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83265138: 38C8B6C0  addi r6, r8, -0x4940
	ctx.r[6].s64 = ctx.r[8].s64 + -18752;
	// 8326513C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265158 size=64
    let mut pc: u32 = 0x83265158;
    'dispatch: loop {
        match pc {
            0x83265158 => {
    //   block [0x83265158..0x83265198)
	// 83265158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326515C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265164: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326516C: 388B0D08  addi r4, r11, 0xd08
	ctx.r[4].s64 = ctx.r[11].s64 + 3336;
	// 83265170: 386AB6D0  addi r3, r10, -0x4930
	ctx.r[3].s64 = ctx.r[10].s64 + -18736;
	// 83265174: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265178: 4AFC7D59  bl 0x8222ced0
	ctx.lr = 0x8326517C;
	sub_8222CED0(ctx, base);
	// 8326517C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265180: 3869D158  addi r3, r9, -0x2ea8
	ctx.r[3].s64 = ctx.r[9].s64 + -11944;
	// 83265184: 4BA44D9D  bl 0x82ca9f20
	ctx.lr = 0x83265188;
	sub_82CA9F20(ctx, base);
	// 83265188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326518C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265198 size=64
    let mut pc: u32 = 0x83265198;
    'dispatch: loop {
        match pc {
            0x83265198 => {
    //   block [0x83265198..0x832651D8)
	// 83265198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326519C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832651A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832651A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832651A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832651AC: 388B0D24  addi r4, r11, 0xd24
	ctx.r[4].s64 = ctx.r[11].s64 + 3364;
	// 832651B0: 386AB6D4  addi r3, r10, -0x492c
	ctx.r[3].s64 = ctx.r[10].s64 + -18732;
	// 832651B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832651B8: 4AFC7D19  bl 0x8222ced0
	ctx.lr = 0x832651BC;
	sub_8222CED0(ctx, base);
	// 832651BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832651C0: 3869D168  addi r3, r9, -0x2e98
	ctx.r[3].s64 = ctx.r[9].s64 + -11928;
	// 832651C4: 4BA44D5D  bl 0x82ca9f20
	ctx.lr = 0x832651C8;
	sub_82CA9F20(ctx, base);
	// 832651C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832651CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832651D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832651D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832651D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832651D8 size=64
    let mut pc: u32 = 0x832651D8;
    'dispatch: loop {
        match pc {
            0x832651D8 => {
    //   block [0x832651D8..0x83265218)
	// 832651D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832651DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832651E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832651E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832651E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832651EC: 388B0D40  addi r4, r11, 0xd40
	ctx.r[4].s64 = ctx.r[11].s64 + 3392;
	// 832651F0: 386AB6D8  addi r3, r10, -0x4928
	ctx.r[3].s64 = ctx.r[10].s64 + -18728;
	// 832651F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832651F8: 4AFC7CD9  bl 0x8222ced0
	ctx.lr = 0x832651FC;
	sub_8222CED0(ctx, base);
	// 832651FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265200: 3869D178  addi r3, r9, -0x2e88
	ctx.r[3].s64 = ctx.r[9].s64 + -11912;
	// 83265204: 4BA44D1D  bl 0x82ca9f20
	ctx.lr = 0x83265208;
	sub_82CA9F20(ctx, base);
	// 83265208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326520C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265218 size=64
    let mut pc: u32 = 0x83265218;
    'dispatch: loop {
        match pc {
            0x83265218 => {
    //   block [0x83265218..0x83265258)
	// 83265218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326521C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265224: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326522C: 388B0D60  addi r4, r11, 0xd60
	ctx.r[4].s64 = ctx.r[11].s64 + 3424;
	// 83265230: 386AB6DC  addi r3, r10, -0x4924
	ctx.r[3].s64 = ctx.r[10].s64 + -18724;
	// 83265234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265238: 4AFC7C99  bl 0x8222ced0
	ctx.lr = 0x8326523C;
	sub_8222CED0(ctx, base);
	// 8326523C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265240: 3869D188  addi r3, r9, -0x2e78
	ctx.r[3].s64 = ctx.r[9].s64 + -11896;
	// 83265244: 4BA44CDD  bl 0x82ca9f20
	ctx.lr = 0x83265248;
	sub_82CA9F20(ctx, base);
	// 83265248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326524C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265258 size=64
    let mut pc: u32 = 0x83265258;
    'dispatch: loop {
        match pc {
            0x83265258 => {
    //   block [0x83265258..0x83265298)
	// 83265258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326525C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265264: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326526C: 388B0D80  addi r4, r11, 0xd80
	ctx.r[4].s64 = ctx.r[11].s64 + 3456;
	// 83265270: 386AB6E0  addi r3, r10, -0x4920
	ctx.r[3].s64 = ctx.r[10].s64 + -18720;
	// 83265274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265278: 4AFC7C59  bl 0x8222ced0
	ctx.lr = 0x8326527C;
	sub_8222CED0(ctx, base);
	// 8326527C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265280: 3869D198  addi r3, r9, -0x2e68
	ctx.r[3].s64 = ctx.r[9].s64 + -11880;
	// 83265284: 4BA44C9D  bl 0x82ca9f20
	ctx.lr = 0x83265288;
	sub_82CA9F20(ctx, base);
	// 83265288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326528C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265298 size=64
    let mut pc: u32 = 0x83265298;
    'dispatch: loop {
        match pc {
            0x83265298 => {
    //   block [0x83265298..0x832652D8)
	// 83265298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832652A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832652A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832652A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832652AC: 388B0D9C  addi r4, r11, 0xd9c
	ctx.r[4].s64 = ctx.r[11].s64 + 3484;
	// 832652B0: 386AB6E4  addi r3, r10, -0x491c
	ctx.r[3].s64 = ctx.r[10].s64 + -18716;
	// 832652B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832652B8: 4AFC7C19  bl 0x8222ced0
	ctx.lr = 0x832652BC;
	sub_8222CED0(ctx, base);
	// 832652BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832652C0: 3869D1A8  addi r3, r9, -0x2e58
	ctx.r[3].s64 = ctx.r[9].s64 + -11864;
	// 832652C4: 4BA44C5D  bl 0x82ca9f20
	ctx.lr = 0x832652C8;
	sub_82CA9F20(ctx, base);
	// 832652C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832652CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832652D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832652D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832652D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832652D8 size=64
    let mut pc: u32 = 0x832652D8;
    'dispatch: loop {
        match pc {
            0x832652D8 => {
    //   block [0x832652D8..0x83265318)
	// 832652D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832652DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832652E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832652E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832652E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832652EC: 388B0DB8  addi r4, r11, 0xdb8
	ctx.r[4].s64 = ctx.r[11].s64 + 3512;
	// 832652F0: 386AB6E8  addi r3, r10, -0x4918
	ctx.r[3].s64 = ctx.r[10].s64 + -18712;
	// 832652F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832652F8: 4AFC7BD9  bl 0x8222ced0
	ctx.lr = 0x832652FC;
	sub_8222CED0(ctx, base);
	// 832652FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265300: 3869D1B8  addi r3, r9, -0x2e48
	ctx.r[3].s64 = ctx.r[9].s64 + -11848;
	// 83265304: 4BA44C1D  bl 0x82ca9f20
	ctx.lr = 0x83265308;
	sub_82CA9F20(ctx, base);
	// 83265308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326530C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83265318 size=12
    let mut pc: u32 = 0x83265318;
    'dispatch: loop {
        match pc {
            0x83265318 => {
    //   block [0x83265318..0x83265324)
	// 83265318: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326531C: 386BD1C8  addi r3, r11, -0x2e38
	ctx.r[3].s64 = ctx.r[11].s64 + -11832;
	// 83265320: 4BA44C00  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265328 size=56
    let mut pc: u32 = 0x83265328;
    'dispatch: loop {
        match pc {
            0x83265328 => {
    //   block [0x83265328..0x83265360)
	// 83265328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326532C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265334: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326533C: 386B0EBC  addi r3, r11, 0xebc
	ctx.r[3].s64 = ctx.r[11].s64 + 3772;
	// 83265340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83265344: 4AF8EA15  bl 0x821f3d58
	ctx.lr = 0x83265348;
	sub_821F3D58(ctx, base);
	// 83265348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326534C: 906AB6F4  stw r3, -0x490c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18700 as u32), ctx.r[3].u32 ) };
	// 83265350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326535C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265360 size=56
    let mut pc: u32 = 0x83265360;
    'dispatch: loop {
        match pc {
            0x83265360 => {
    //   block [0x83265360..0x83265398)
	// 83265360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326536C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83265374: 386B0EC8  addi r3, r11, 0xec8
	ctx.r[3].s64 = ctx.r[11].s64 + 3784;
	// 83265378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326537C: 4AF8E9DD  bl 0x821f3d58
	ctx.lr = 0x83265380;
	sub_821F3D58(ctx, base);
	// 83265380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265384: 906AB6F8  stw r3, -0x4908(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18696 as u32), ctx.r[3].u32 ) };
	// 83265388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326538C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265398 size=144
    let mut pc: u32 = 0x83265398;
    'dispatch: loop {
        match pc {
            0x83265398 => {
    //   block [0x83265398..0x832653BC)
	// 83265398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326539C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832653A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832653A4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 832653A8: 4AFB9EB1  bl 0x8221f258
	ctx.lr = 0x832653AC;
	sub_8221F258(ctx, base);
	// 832653AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832653B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832653B4: 419A0008  beq cr6, 0x832653bc
	if ctx.cr[6].eq {
	pc = 0x832653BC; continue 'dispatch;
	}
	// 832653B8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832653BC; continue 'dispatch;
            }
            0x832653BC => {
    //   block [0x832653BC..0x832653C8)
	// 832653BC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832653C0: 41820008  beq 0x832653c8
	if ctx.cr[0].eq {
	pc = 0x832653C8; continue 'dispatch;
	}
	// 832653C4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832653C8; continue 'dispatch;
            }
            0x832653C8 => {
    //   block [0x832653C8..0x832653D4)
	// 832653C8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832653CC: 41820008  beq 0x832653d4
	if ctx.cr[0].eq {
	pc = 0x832653D4; continue 'dispatch;
	}
	// 832653D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832653D4; continue 'dispatch;
            }
            0x832653D4 => {
    //   block [0x832653D4..0x83265428)
	// 832653D4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832653D8: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 832653DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832653E0: 3909B6FC  addi r8, r9, -0x4904
	ctx.r[8].s64 = ctx.r[9].s64 + -18692;
	// 832653E4: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 832653E8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832653EC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 832653F0: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 832653F4: 3867D268  addi r3, r7, -0x2d98
	ctx.r[3].s64 = ctx.r[7].s64 + -11672;
	// 832653F8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 832653FC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83265400: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83265404: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83265408: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326540C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83265410: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83265414: 4BA44B0D  bl 0x82ca9f20
	ctx.lr = 0x83265418;
	sub_82CA9F20(ctx, base);
	// 83265418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326541C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265428 size=144
    let mut pc: u32 = 0x83265428;
    'dispatch: loop {
        match pc {
            0x83265428 => {
    //   block [0x83265428..0x8326544C)
	// 83265428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326542C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265434: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83265438: 4AFB9E21  bl 0x8221f258
	ctx.lr = 0x8326543C;
	sub_8221F258(ctx, base);
	// 8326543C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83265440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83265444: 419A0008  beq cr6, 0x8326544c
	if ctx.cr[6].eq {
	pc = 0x8326544C; continue 'dispatch;
	}
	// 83265448: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326544C; continue 'dispatch;
            }
            0x8326544C => {
    //   block [0x8326544C..0x83265458)
	// 8326544C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83265450: 41820008  beq 0x83265458
	if ctx.cr[0].eq {
	pc = 0x83265458; continue 'dispatch;
	}
	// 83265454: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83265458; continue 'dispatch;
            }
            0x83265458 => {
    //   block [0x83265458..0x83265464)
	// 83265458: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326545C: 41820008  beq 0x83265464
	if ctx.cr[0].eq {
	pc = 0x83265464; continue 'dispatch;
	}
	// 83265460: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83265464; continue 'dispatch;
            }
            0x83265464 => {
    //   block [0x83265464..0x832654B8)
	// 83265464: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83265468: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 8326546C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83265470: 3909B708  addi r8, r9, -0x48f8
	ctx.r[8].s64 = ctx.r[9].s64 + -18680;
	// 83265474: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83265478: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326547C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83265480: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 83265484: 3867D278  addi r3, r7, -0x2d88
	ctx.r[3].s64 = ctx.r[7].s64 + -11656;
	// 83265488: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326548C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83265490: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83265494: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83265498: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326549C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832654A0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832654A4: 4BA44A7D  bl 0x82ca9f20
	ctx.lr = 0x832654A8;
	sub_82CA9F20(ctx, base);
	// 832654A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832654AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832654B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832654B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832654B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832654B8 size=64
    let mut pc: u32 = 0x832654B8;
    'dispatch: loop {
        match pc {
            0x832654B8 => {
    //   block [0x832654B8..0x832654F8)
	// 832654B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832654BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832654C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832654C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832654C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832654CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832654D0: 386AB714  addi r3, r10, -0x48ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18668;
	// 832654D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832654D8: 4AFC79F9  bl 0x8222ced0
	ctx.lr = 0x832654DC;
	sub_8222CED0(ctx, base);
	// 832654DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832654E0: 3869D288  addi r3, r9, -0x2d78
	ctx.r[3].s64 = ctx.r[9].s64 + -11640;
	// 832654E4: 4BA44A3D  bl 0x82ca9f20
	ctx.lr = 0x832654E8;
	sub_82CA9F20(ctx, base);
	// 832654E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832654EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832654F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832654F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832654F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832654F8 size=64
    let mut pc: u32 = 0x832654F8;
    'dispatch: loop {
        match pc {
            0x832654F8 => {
    //   block [0x832654F8..0x83265538)
	// 832654F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832654FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265504: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326550C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83265510: 386AB718  addi r3, r10, -0x48e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18664;
	// 83265514: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265518: 4AFC79B9  bl 0x8222ced0
	ctx.lr = 0x8326551C;
	sub_8222CED0(ctx, base);
	// 8326551C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265520: 3869D298  addi r3, r9, -0x2d68
	ctx.r[3].s64 = ctx.r[9].s64 + -11624;
	// 83265524: 4BA449FD  bl 0x82ca9f20
	ctx.lr = 0x83265528;
	sub_82CA9F20(ctx, base);
	// 83265528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326552C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265538 size=64
    let mut pc: u32 = 0x83265538;
    'dispatch: loop {
        match pc {
            0x83265538 => {
    //   block [0x83265538..0x83265578)
	// 83265538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326553C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265548: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326554C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83265550: 386AB71C  addi r3, r10, -0x48e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18660;
	// 83265554: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265558: 4AFC7979  bl 0x8222ced0
	ctx.lr = 0x8326555C;
	sub_8222CED0(ctx, base);
	// 8326555C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265560: 3869D2A8  addi r3, r9, -0x2d58
	ctx.r[3].s64 = ctx.r[9].s64 + -11608;
	// 83265564: 4BA449BD  bl 0x82ca9f20
	ctx.lr = 0x83265568;
	sub_82CA9F20(ctx, base);
	// 83265568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326556C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265578 size=64
    let mut pc: u32 = 0x83265578;
    'dispatch: loop {
        match pc {
            0x83265578 => {
    //   block [0x83265578..0x832655B8)
	// 83265578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326557C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265584: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326558C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83265590: 386AB720  addi r3, r10, -0x48e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18656;
	// 83265594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265598: 4AFC7939  bl 0x8222ced0
	ctx.lr = 0x8326559C;
	sub_8222CED0(ctx, base);
	// 8326559C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832655A0: 3869D2B8  addi r3, r9, -0x2d48
	ctx.r[3].s64 = ctx.r[9].s64 + -11592;
	// 832655A4: 4BA4497D  bl 0x82ca9f20
	ctx.lr = 0x832655A8;
	sub_82CA9F20(ctx, base);
	// 832655A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832655AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832655B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832655B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832655B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832655B8 size=64
    let mut pc: u32 = 0x832655B8;
    'dispatch: loop {
        match pc {
            0x832655B8 => {
    //   block [0x832655B8..0x832655F8)
	// 832655B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832655BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832655C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832655C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832655C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832655CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832655D0: 386AB724  addi r3, r10, -0x48dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18652;
	// 832655D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832655D8: 4AFC78F9  bl 0x8222ced0
	ctx.lr = 0x832655DC;
	sub_8222CED0(ctx, base);
	// 832655DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832655E0: 3869D2C8  addi r3, r9, -0x2d38
	ctx.r[3].s64 = ctx.r[9].s64 + -11576;
	// 832655E4: 4BA4493D  bl 0x82ca9f20
	ctx.lr = 0x832655E8;
	sub_82CA9F20(ctx, base);
	// 832655E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832655EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832655F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832655F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832655F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832655F8 size=64
    let mut pc: u32 = 0x832655F8;
    'dispatch: loop {
        match pc {
            0x832655F8 => {
    //   block [0x832655F8..0x83265638)
	// 832655F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832655FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326560C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83265610: 386AB728  addi r3, r10, -0x48d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18648;
	// 83265614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265618: 4AFC78B9  bl 0x8222ced0
	ctx.lr = 0x8326561C;
	sub_8222CED0(ctx, base);
	// 8326561C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265620: 3869D2D8  addi r3, r9, -0x2d28
	ctx.r[3].s64 = ctx.r[9].s64 + -11560;
	// 83265624: 4BA448FD  bl 0x82ca9f20
	ctx.lr = 0x83265628;
	sub_82CA9F20(ctx, base);
	// 83265628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326562C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265638 size=64
    let mut pc: u32 = 0x83265638;
    'dispatch: loop {
        match pc {
            0x83265638 => {
    //   block [0x83265638..0x83265678)
	// 83265638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326563C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265644: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326564C: 388B11C8  addi r4, r11, 0x11c8
	ctx.r[4].s64 = ctx.r[11].s64 + 4552;
	// 83265650: 386AB72C  addi r3, r10, -0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18644;
	// 83265654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265658: 4AFC7879  bl 0x8222ced0
	ctx.lr = 0x8326565C;
	sub_8222CED0(ctx, base);
	// 8326565C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265660: 3869D2E8  addi r3, r9, -0x2d18
	ctx.r[3].s64 = ctx.r[9].s64 + -11544;
	// 83265664: 4BA448BD  bl 0x82ca9f20
	ctx.lr = 0x83265668;
	sub_82CA9F20(ctx, base);
	// 83265668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326566C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265678 size=64
    let mut pc: u32 = 0x83265678;
    'dispatch: loop {
        match pc {
            0x83265678 => {
    //   block [0x83265678..0x832656B8)
	// 83265678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326567C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326568C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83265690: 386AB730  addi r3, r10, -0x48d0
	ctx.r[3].s64 = ctx.r[10].s64 + -18640;
	// 83265694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265698: 4AFC7839  bl 0x8222ced0
	ctx.lr = 0x8326569C;
	sub_8222CED0(ctx, base);
	// 8326569C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832656A0: 3869D2F8  addi r3, r9, -0x2d08
	ctx.r[3].s64 = ctx.r[9].s64 + -11528;
	// 832656A4: 4BA4487D  bl 0x82ca9f20
	ctx.lr = 0x832656A8;
	sub_82CA9F20(ctx, base);
	// 832656A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832656AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832656B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832656B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832656B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832656B8 size=64
    let mut pc: u32 = 0x832656B8;
    'dispatch: loop {
        match pc {
            0x832656B8 => {
    //   block [0x832656B8..0x832656F8)
	// 832656B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832656BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832656C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832656C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832656C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832656CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832656D0: 386AB734  addi r3, r10, -0x48cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18636;
	// 832656D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832656D8: 4AFC77F9  bl 0x8222ced0
	ctx.lr = 0x832656DC;
	sub_8222CED0(ctx, base);
	// 832656DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832656E0: 3869D308  addi r3, r9, -0x2cf8
	ctx.r[3].s64 = ctx.r[9].s64 + -11512;
	// 832656E4: 4BA4483D  bl 0x82ca9f20
	ctx.lr = 0x832656E8;
	sub_82CA9F20(ctx, base);
	// 832656E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832656EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832656F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832656F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832656F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832656F8 size=64
    let mut pc: u32 = 0x832656F8;
    'dispatch: loop {
        match pc {
            0x832656F8 => {
    //   block [0x832656F8..0x83265738)
	// 832656F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832656FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326570C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83265710: 386AB738  addi r3, r10, -0x48c8
	ctx.r[3].s64 = ctx.r[10].s64 + -18632;
	// 83265714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265718: 4AFC77B9  bl 0x8222ced0
	ctx.lr = 0x8326571C;
	sub_8222CED0(ctx, base);
	// 8326571C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265720: 3869D318  addi r3, r9, -0x2ce8
	ctx.r[3].s64 = ctx.r[9].s64 + -11496;
	// 83265724: 4BA447FD  bl 0x82ca9f20
	ctx.lr = 0x83265728;
	sub_82CA9F20(ctx, base);
	// 83265728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326572C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265738 size=64
    let mut pc: u32 = 0x83265738;
    'dispatch: loop {
        match pc {
            0x83265738 => {
    //   block [0x83265738..0x83265778)
	// 83265738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326574C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83265750: 386AB73C  addi r3, r10, -0x48c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18628;
	// 83265754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265758: 4AFC7779  bl 0x8222ced0
	ctx.lr = 0x8326575C;
	sub_8222CED0(ctx, base);
	// 8326575C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265760: 3869D328  addi r3, r9, -0x2cd8
	ctx.r[3].s64 = ctx.r[9].s64 + -11480;
	// 83265764: 4BA447BD  bl 0x82ca9f20
	ctx.lr = 0x83265768;
	sub_82CA9F20(ctx, base);
	// 83265768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326576C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265778 size=64
    let mut pc: u32 = 0x83265778;
    'dispatch: loop {
        match pc {
            0x83265778 => {
    //   block [0x83265778..0x832657B8)
	// 83265778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265784: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265788: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326578C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83265790: 386AB740  addi r3, r10, -0x48c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18624;
	// 83265794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265798: 4AFC7739  bl 0x8222ced0
	ctx.lr = 0x8326579C;
	sub_8222CED0(ctx, base);
	// 8326579C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832657A0: 3869D338  addi r3, r9, -0x2cc8
	ctx.r[3].s64 = ctx.r[9].s64 + -11464;
	// 832657A4: 4BA4477D  bl 0x82ca9f20
	ctx.lr = 0x832657A8;
	sub_82CA9F20(ctx, base);
	// 832657A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832657AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832657B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832657B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832657B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832657B8 size=64
    let mut pc: u32 = 0x832657B8;
    'dispatch: loop {
        match pc {
            0x832657B8 => {
    //   block [0x832657B8..0x832657F8)
	// 832657B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832657BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832657C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832657C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832657C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832657CC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832657D0: 386AB744  addi r3, r10, -0x48bc
	ctx.r[3].s64 = ctx.r[10].s64 + -18620;
	// 832657D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832657D8: 4AFC76F9  bl 0x8222ced0
	ctx.lr = 0x832657DC;
	sub_8222CED0(ctx, base);
	// 832657DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832657E0: 3869D348  addi r3, r9, -0x2cb8
	ctx.r[3].s64 = ctx.r[9].s64 + -11448;
	// 832657E4: 4BA4473D  bl 0x82ca9f20
	ctx.lr = 0x832657E8;
	sub_82CA9F20(ctx, base);
	// 832657E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832657EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832657F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832657F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832657F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832657F8 size=64
    let mut pc: u32 = 0x832657F8;
    'dispatch: loop {
        match pc {
            0x832657F8 => {
    //   block [0x832657F8..0x83265838)
	// 832657F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832657FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265804: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265808: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326580C: 388B1E08  addi r4, r11, 0x1e08
	ctx.r[4].s64 = ctx.r[11].s64 + 7688;
	// 83265810: 386AB748  addi r3, r10, -0x48b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18616;
	// 83265814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265818: 4AFC76B9  bl 0x8222ced0
	ctx.lr = 0x8326581C;
	sub_8222CED0(ctx, base);
	// 8326581C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265820: 3869D358  addi r3, r9, -0x2ca8
	ctx.r[3].s64 = ctx.r[9].s64 + -11432;
	// 83265824: 4BA446FD  bl 0x82ca9f20
	ctx.lr = 0x83265828;
	sub_82CA9F20(ctx, base);
	// 83265828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326582C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265838 size=64
    let mut pc: u32 = 0x83265838;
    'dispatch: loop {
        match pc {
            0x83265838 => {
    //   block [0x83265838..0x83265878)
	// 83265838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326583C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265844: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326584C: 388B1E10  addi r4, r11, 0x1e10
	ctx.r[4].s64 = ctx.r[11].s64 + 7696;
	// 83265850: 386AB74C  addi r3, r10, -0x48b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18612;
	// 83265854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265858: 4AFC7679  bl 0x8222ced0
	ctx.lr = 0x8326585C;
	sub_8222CED0(ctx, base);
	// 8326585C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265860: 3869D368  addi r3, r9, -0x2c98
	ctx.r[3].s64 = ctx.r[9].s64 + -11416;
	// 83265864: 4BA446BD  bl 0x82ca9f20
	ctx.lr = 0x83265868;
	sub_82CA9F20(ctx, base);
	// 83265868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326586C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265878 size=64
    let mut pc: u32 = 0x83265878;
    'dispatch: loop {
        match pc {
            0x83265878 => {
    //   block [0x83265878..0x832658B8)
	// 83265878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265884: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326588C: 388B1E14  addi r4, r11, 0x1e14
	ctx.r[4].s64 = ctx.r[11].s64 + 7700;
	// 83265890: 386AB750  addi r3, r10, -0x48b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18608;
	// 83265894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265898: 4AFC7639  bl 0x8222ced0
	ctx.lr = 0x8326589C;
	sub_8222CED0(ctx, base);
	// 8326589C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832658A0: 3869D378  addi r3, r9, -0x2c88
	ctx.r[3].s64 = ctx.r[9].s64 + -11400;
	// 832658A4: 4BA4467D  bl 0x82ca9f20
	ctx.lr = 0x832658A8;
	sub_82CA9F20(ctx, base);
	// 832658A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832658AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832658B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832658B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832658B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832658B8 size=64
    let mut pc: u32 = 0x832658B8;
    'dispatch: loop {
        match pc {
            0x832658B8 => {
    //   block [0x832658B8..0x832658F8)
	// 832658B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832658BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832658C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832658C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832658C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832658CC: 388B1E20  addi r4, r11, 0x1e20
	ctx.r[4].s64 = ctx.r[11].s64 + 7712;
	// 832658D0: 386AB754  addi r3, r10, -0x48ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18604;
	// 832658D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832658D8: 4AFC75F9  bl 0x8222ced0
	ctx.lr = 0x832658DC;
	sub_8222CED0(ctx, base);
	// 832658DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832658E0: 3869D388  addi r3, r9, -0x2c78
	ctx.r[3].s64 = ctx.r[9].s64 + -11384;
	// 832658E4: 4BA4463D  bl 0x82ca9f20
	ctx.lr = 0x832658E8;
	sub_82CA9F20(ctx, base);
	// 832658E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832658EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832658F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832658F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832658F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832658F8 size=64
    let mut pc: u32 = 0x832658F8;
    'dispatch: loop {
        match pc {
            0x832658F8 => {
    //   block [0x832658F8..0x83265938)
	// 832658F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832658FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265904: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326590C: 388B1E28  addi r4, r11, 0x1e28
	ctx.r[4].s64 = ctx.r[11].s64 + 7720;
	// 83265910: 386AB758  addi r3, r10, -0x48a8
	ctx.r[3].s64 = ctx.r[10].s64 + -18600;
	// 83265914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265918: 4AFC75B9  bl 0x8222ced0
	ctx.lr = 0x8326591C;
	sub_8222CED0(ctx, base);
	// 8326591C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265920: 3869D398  addi r3, r9, -0x2c68
	ctx.r[3].s64 = ctx.r[9].s64 + -11368;
	// 83265924: 4BA445FD  bl 0x82ca9f20
	ctx.lr = 0x83265928;
	sub_82CA9F20(ctx, base);
	// 83265928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326592C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265938 size=64
    let mut pc: u32 = 0x83265938;
    'dispatch: loop {
        match pc {
            0x83265938 => {
    //   block [0x83265938..0x83265978)
	// 83265938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265944: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326594C: 388B1E30  addi r4, r11, 0x1e30
	ctx.r[4].s64 = ctx.r[11].s64 + 7728;
	// 83265950: 386AB75C  addi r3, r10, -0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18596;
	// 83265954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265958: 4AFC7579  bl 0x8222ced0
	ctx.lr = 0x8326595C;
	sub_8222CED0(ctx, base);
	// 8326595C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265960: 3869D3A8  addi r3, r9, -0x2c58
	ctx.r[3].s64 = ctx.r[9].s64 + -11352;
	// 83265964: 4BA445BD  bl 0x82ca9f20
	ctx.lr = 0x83265968;
	sub_82CA9F20(ctx, base);
	// 83265968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326596C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265978 size=64
    let mut pc: u32 = 0x83265978;
    'dispatch: loop {
        match pc {
            0x83265978 => {
    //   block [0x83265978..0x832659B8)
	// 83265978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326597C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265984: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326598C: 388B1E3C  addi r4, r11, 0x1e3c
	ctx.r[4].s64 = ctx.r[11].s64 + 7740;
	// 83265990: 386AB760  addi r3, r10, -0x48a0
	ctx.r[3].s64 = ctx.r[10].s64 + -18592;
	// 83265994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265998: 4AFC7539  bl 0x8222ced0
	ctx.lr = 0x8326599C;
	sub_8222CED0(ctx, base);
	// 8326599C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832659A0: 3869D3B8  addi r3, r9, -0x2c48
	ctx.r[3].s64 = ctx.r[9].s64 + -11336;
	// 832659A4: 4BA4457D  bl 0x82ca9f20
	ctx.lr = 0x832659A8;
	sub_82CA9F20(ctx, base);
	// 832659A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832659AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832659B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832659B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832659B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832659B8 size=64
    let mut pc: u32 = 0x832659B8;
    'dispatch: loop {
        match pc {
            0x832659B8 => {
    //   block [0x832659B8..0x832659F8)
	// 832659B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832659BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832659C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832659C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832659C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832659CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832659D0: 386AB764  addi r3, r10, -0x489c
	ctx.r[3].s64 = ctx.r[10].s64 + -18588;
	// 832659D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832659D8: 4AFC74F9  bl 0x8222ced0
	ctx.lr = 0x832659DC;
	sub_8222CED0(ctx, base);
	// 832659DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832659E0: 3869D3C8  addi r3, r9, -0x2c38
	ctx.r[3].s64 = ctx.r[9].s64 + -11320;
	// 832659E4: 4BA4453D  bl 0x82ca9f20
	ctx.lr = 0x832659E8;
	sub_82CA9F20(ctx, base);
	// 832659E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832659EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832659F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832659F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832659F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832659F8 size=64
    let mut pc: u32 = 0x832659F8;
    'dispatch: loop {
        match pc {
            0x832659F8 => {
    //   block [0x832659F8..0x83265A38)
	// 832659F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832659FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265A0C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83265A10: 386AB768  addi r3, r10, -0x4898
	ctx.r[3].s64 = ctx.r[10].s64 + -18584;
	// 83265A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265A18: 4AFC74B9  bl 0x8222ced0
	ctx.lr = 0x83265A1C;
	sub_8222CED0(ctx, base);
	// 83265A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265A20: 3869D3D8  addi r3, r9, -0x2c28
	ctx.r[3].s64 = ctx.r[9].s64 + -11304;
	// 83265A24: 4BA444FD  bl 0x82ca9f20
	ctx.lr = 0x83265A28;
	sub_82CA9F20(ctx, base);
	// 83265A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265A38 size=64
    let mut pc: u32 = 0x83265A38;
    'dispatch: loop {
        match pc {
            0x83265A38 => {
    //   block [0x83265A38..0x83265A78)
	// 83265A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265A4C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83265A50: 386AB76C  addi r3, r10, -0x4894
	ctx.r[3].s64 = ctx.r[10].s64 + -18580;
	// 83265A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265A58: 4AFC7479  bl 0x8222ced0
	ctx.lr = 0x83265A5C;
	sub_8222CED0(ctx, base);
	// 83265A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265A60: 3869D3E8  addi r3, r9, -0x2c18
	ctx.r[3].s64 = ctx.r[9].s64 + -11288;
	// 83265A64: 4BA444BD  bl 0x82ca9f20
	ctx.lr = 0x83265A68;
	sub_82CA9F20(ctx, base);
	// 83265A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265A78 size=64
    let mut pc: u32 = 0x83265A78;
    'dispatch: loop {
        match pc {
            0x83265A78 => {
    //   block [0x83265A78..0x83265AB8)
	// 83265A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265A8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83265A90: 386AB770  addi r3, r10, -0x4890
	ctx.r[3].s64 = ctx.r[10].s64 + -18576;
	// 83265A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265A98: 4AFC7439  bl 0x8222ced0
	ctx.lr = 0x83265A9C;
	sub_8222CED0(ctx, base);
	// 83265A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265AA0: 3869D400  addi r3, r9, -0x2c00
	ctx.r[3].s64 = ctx.r[9].s64 + -11264;
	// 83265AA4: 4BA4447D  bl 0x82ca9f20
	ctx.lr = 0x83265AA8;
	sub_82CA9F20(ctx, base);
	// 83265AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265AB8 size=64
    let mut pc: u32 = 0x83265AB8;
    'dispatch: loop {
        match pc {
            0x83265AB8 => {
    //   block [0x83265AB8..0x83265AF8)
	// 83265AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265ACC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83265AD0: 386AB774  addi r3, r10, -0x488c
	ctx.r[3].s64 = ctx.r[10].s64 + -18572;
	// 83265AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265AD8: 4AFC73F9  bl 0x8222ced0
	ctx.lr = 0x83265ADC;
	sub_8222CED0(ctx, base);
	// 83265ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265AE0: 3869D410  addi r3, r9, -0x2bf0
	ctx.r[3].s64 = ctx.r[9].s64 + -11248;
	// 83265AE4: 4BA4443D  bl 0x82ca9f20
	ctx.lr = 0x83265AE8;
	sub_82CA9F20(ctx, base);
	// 83265AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265AF8 size=64
    let mut pc: u32 = 0x83265AF8;
    'dispatch: loop {
        match pc {
            0x83265AF8 => {
    //   block [0x83265AF8..0x83265B38)
	// 83265AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83265B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265B0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83265B10: 386AB778  addi r3, r10, -0x4888
	ctx.r[3].s64 = ctx.r[10].s64 + -18568;
	// 83265B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265B18: 4AFC73B9  bl 0x8222ced0
	ctx.lr = 0x83265B1C;
	sub_8222CED0(ctx, base);
	// 83265B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265B20: 3869D420  addi r3, r9, -0x2be0
	ctx.r[3].s64 = ctx.r[9].s64 + -11232;
	// 83265B24: 4BA443FD  bl 0x82ca9f20
	ctx.lr = 0x83265B28;
	sub_82CA9F20(ctx, base);
	// 83265B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265B38 size=64
    let mut pc: u32 = 0x83265B38;
    'dispatch: loop {
        match pc {
            0x83265B38 => {
    //   block [0x83265B38..0x83265B78)
	// 83265B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265B44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265B4C: 388B22E0  addi r4, r11, 0x22e0
	ctx.r[4].s64 = ctx.r[11].s64 + 8928;
	// 83265B50: 386AB77C  addi r3, r10, -0x4884
	ctx.r[3].s64 = ctx.r[10].s64 + -18564;
	// 83265B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265B58: 4AFC7379  bl 0x8222ced0
	ctx.lr = 0x83265B5C;
	sub_8222CED0(ctx, base);
	// 83265B5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265B60: 3869D430  addi r3, r9, -0x2bd0
	ctx.r[3].s64 = ctx.r[9].s64 + -11216;
	// 83265B64: 4BA443BD  bl 0x82ca9f20
	ctx.lr = 0x83265B68;
	sub_82CA9F20(ctx, base);
	// 83265B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265B78 size=88
    let mut pc: u32 = 0x83265B78;
    'dispatch: loop {
        match pc {
            0x83265B78 => {
    //   block [0x83265B78..0x83265BD0)
	// 83265B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265B88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265B8C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265B90: 3BEBB780  addi r31, r11, -0x4880
	ctx.r[31].s64 = ctx.r[11].s64 + -18560;
	// 83265B94: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265B9C: 4AF8A6A5  bl 0x821f0240
	ctx.lr = 0x83265BA0;
	sub_821F0240(ctx, base);
	// 83265BA0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265BA8: 388922F0  addi r4, r9, 0x22f0
	ctx.r[4].s64 = ctx.r[9].s64 + 8944;
	// 83265BAC: 4AF74E15  bl 0x821da9c0
	ctx.lr = 0x83265BB0;
	sub_821DA9C0(ctx, base);
	// 83265BB0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265BB4: 3868D440  addi r3, r8, -0x2bc0
	ctx.r[3].s64 = ctx.r[8].s64 + -11200;
	// 83265BB8: 4BA44369  bl 0x82ca9f20
	ctx.lr = 0x83265BBC;
	sub_82CA9F20(ctx, base);
	// 83265BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265BD0 size=88
    let mut pc: u32 = 0x83265BD0;
    'dispatch: loop {
        match pc {
            0x83265BD0 => {
    //   block [0x83265BD0..0x83265C28)
	// 83265BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265BE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265BE4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265BE8: 3BEBB784  addi r31, r11, -0x487c
	ctx.r[31].s64 = ctx.r[11].s64 + -18556;
	// 83265BEC: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265BF4: 4AF8A64D  bl 0x821f0240
	ctx.lr = 0x83265BF8;
	sub_821F0240(ctx, base);
	// 83265BF8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265C00: 388922FC  addi r4, r9, 0x22fc
	ctx.r[4].s64 = ctx.r[9].s64 + 8956;
	// 83265C04: 4AF74DBD  bl 0x821da9c0
	ctx.lr = 0x83265C08;
	sub_821DA9C0(ctx, base);
	// 83265C08: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265C0C: 3868D450  addi r3, r8, -0x2bb0
	ctx.r[3].s64 = ctx.r[8].s64 + -11184;
	// 83265C10: 4BA44311  bl 0x82ca9f20
	ctx.lr = 0x83265C14;
	sub_82CA9F20(ctx, base);
	// 83265C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265C28 size=88
    let mut pc: u32 = 0x83265C28;
    'dispatch: loop {
        match pc {
            0x83265C28 => {
    //   block [0x83265C28..0x83265C80)
	// 83265C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265C34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265C38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265C3C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265C40: 3BEBB788  addi r31, r11, -0x4878
	ctx.r[31].s64 = ctx.r[11].s64 + -18552;
	// 83265C44: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265C48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265C4C: 4AF8A5F5  bl 0x821f0240
	ctx.lr = 0x83265C50;
	sub_821F0240(ctx, base);
	// 83265C50: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265C58: 38892308  addi r4, r9, 0x2308
	ctx.r[4].s64 = ctx.r[9].s64 + 8968;
	// 83265C5C: 4AF74D65  bl 0x821da9c0
	ctx.lr = 0x83265C60;
	sub_821DA9C0(ctx, base);
	// 83265C60: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265C64: 3868D460  addi r3, r8, -0x2ba0
	ctx.r[3].s64 = ctx.r[8].s64 + -11168;
	// 83265C68: 4BA442B9  bl 0x82ca9f20
	ctx.lr = 0x83265C6C;
	sub_82CA9F20(ctx, base);
	// 83265C6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265C78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265C80 size=88
    let mut pc: u32 = 0x83265C80;
    'dispatch: loop {
        match pc {
            0x83265C80 => {
    //   block [0x83265C80..0x83265CD8)
	// 83265C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265C88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265C8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265C90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265C94: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265C98: 3BEBB78C  addi r31, r11, -0x4874
	ctx.r[31].s64 = ctx.r[11].s64 + -18548;
	// 83265C9C: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265CA4: 4AF8A59D  bl 0x821f0240
	ctx.lr = 0x83265CA8;
	sub_821F0240(ctx, base);
	// 83265CA8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265CB0: 3889231C  addi r4, r9, 0x231c
	ctx.r[4].s64 = ctx.r[9].s64 + 8988;
	// 83265CB4: 4AF74D0D  bl 0x821da9c0
	ctx.lr = 0x83265CB8;
	sub_821DA9C0(ctx, base);
	// 83265CB8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265CBC: 3868D470  addi r3, r8, -0x2b90
	ctx.r[3].s64 = ctx.r[8].s64 + -11152;
	// 83265CC0: 4BA44261  bl 0x82ca9f20
	ctx.lr = 0x83265CC4;
	sub_82CA9F20(ctx, base);
	// 83265CC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265CD8 size=88
    let mut pc: u32 = 0x83265CD8;
    'dispatch: loop {
        match pc {
            0x83265CD8 => {
    //   block [0x83265CD8..0x83265D30)
	// 83265CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265CE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265CE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265CE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265CEC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265CF0: 3BEBB790  addi r31, r11, -0x4870
	ctx.r[31].s64 = ctx.r[11].s64 + -18544;
	// 83265CF4: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265CF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265CFC: 4AF8A545  bl 0x821f0240
	ctx.lr = 0x83265D00;
	sub_821F0240(ctx, base);
	// 83265D00: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265D04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265D08: 38892334  addi r4, r9, 0x2334
	ctx.r[4].s64 = ctx.r[9].s64 + 9012;
	// 83265D0C: 4AF74CB5  bl 0x821da9c0
	ctx.lr = 0x83265D10;
	sub_821DA9C0(ctx, base);
	// 83265D10: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265D14: 3868D480  addi r3, r8, -0x2b80
	ctx.r[3].s64 = ctx.r[8].s64 + -11136;
	// 83265D18: 4BA44209  bl 0x82ca9f20
	ctx.lr = 0x83265D1C;
	sub_82CA9F20(ctx, base);
	// 83265D1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265D30 size=88
    let mut pc: u32 = 0x83265D30;
    'dispatch: loop {
        match pc {
            0x83265D30 => {
    //   block [0x83265D30..0x83265D88)
	// 83265D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265D40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265D44: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265D48: 3BEBB794  addi r31, r11, -0x486c
	ctx.r[31].s64 = ctx.r[11].s64 + -18540;
	// 83265D4C: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265D54: 4AF8A4ED  bl 0x821f0240
	ctx.lr = 0x83265D58;
	sub_821F0240(ctx, base);
	// 83265D58: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265D60: 38892348  addi r4, r9, 0x2348
	ctx.r[4].s64 = ctx.r[9].s64 + 9032;
	// 83265D64: 4AF74C5D  bl 0x821da9c0
	ctx.lr = 0x83265D68;
	sub_821DA9C0(ctx, base);
	// 83265D68: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265D6C: 3868D490  addi r3, r8, -0x2b70
	ctx.r[3].s64 = ctx.r[8].s64 + -11120;
	// 83265D70: 4BA441B1  bl 0x82ca9f20
	ctx.lr = 0x83265D74;
	sub_82CA9F20(ctx, base);
	// 83265D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265D88 size=88
    let mut pc: u32 = 0x83265D88;
    'dispatch: loop {
        match pc {
            0x83265D88 => {
    //   block [0x83265D88..0x83265DE0)
	// 83265D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265D98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265D9C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265DA0: 3BEBB798  addi r31, r11, -0x4868
	ctx.r[31].s64 = ctx.r[11].s64 + -18536;
	// 83265DA4: 388AB77C  addi r4, r10, -0x4884
	ctx.r[4].s64 = ctx.r[10].s64 + -18564;
	// 83265DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265DAC: 4AF8A495  bl 0x821f0240
	ctx.lr = 0x83265DB0;
	sub_821F0240(ctx, base);
	// 83265DB0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265DB8: 3889235C  addi r4, r9, 0x235c
	ctx.r[4].s64 = ctx.r[9].s64 + 9052;
	// 83265DBC: 4AF74C05  bl 0x821da9c0
	ctx.lr = 0x83265DC0;
	sub_821DA9C0(ctx, base);
	// 83265DC0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265DC4: 3868D4A0  addi r3, r8, -0x2b60
	ctx.r[3].s64 = ctx.r[8].s64 + -11104;
	// 83265DC8: 4BA44159  bl 0x82ca9f20
	ctx.lr = 0x83265DCC;
	sub_82CA9F20(ctx, base);
	// 83265DCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265DD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265DE0 size=48
    let mut pc: u32 = 0x83265DE0;
    'dispatch: loop {
        match pc {
            0x83265DE0 => {
    //   block [0x83265DE0..0x83265E10)
	// 83265DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265DEC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265DF0: 386BB780  addi r3, r11, -0x4880
	ctx.r[3].s64 = ctx.r[11].s64 + -18560;
	// 83265DF4: 4B14AC45  bl 0x823b0a38
	ctx.lr = 0x83265DF8;
	sub_823B0A38(ctx, base);
	// 83265DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265DFC: 906AB79C  stw r3, -0x4864(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18532 as u32), ctx.r[3].u32 ) };
	// 83265E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265E10 size=48
    let mut pc: u32 = 0x83265E10;
    'dispatch: loop {
        match pc {
            0x83265E10 => {
    //   block [0x83265E10..0x83265E40)
	// 83265E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265E1C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265E20: 386BB784  addi r3, r11, -0x487c
	ctx.r[3].s64 = ctx.r[11].s64 + -18556;
	// 83265E24: 4B14AC15  bl 0x823b0a38
	ctx.lr = 0x83265E28;
	sub_823B0A38(ctx, base);
	// 83265E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265E2C: 906AB7A0  stw r3, -0x4860(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18528 as u32), ctx.r[3].u32 ) };
	// 83265E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265E40 size=64
    let mut pc: u32 = 0x83265E40;
    'dispatch: loop {
        match pc {
            0x83265E40 => {
    //   block [0x83265E40..0x83265E80)
	// 83265E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265E4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265E54: 388B2364  addi r4, r11, 0x2364
	ctx.r[4].s64 = ctx.r[11].s64 + 9060;
	// 83265E58: 386AB7A4  addi r3, r10, -0x485c
	ctx.r[3].s64 = ctx.r[10].s64 + -18524;
	// 83265E5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265E60: 4AFC7071  bl 0x8222ced0
	ctx.lr = 0x83265E64;
	sub_8222CED0(ctx, base);
	// 83265E64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265E68: 3869D4B0  addi r3, r9, -0x2b50
	ctx.r[3].s64 = ctx.r[9].s64 + -11088;
	// 83265E6C: 4BA440B5  bl 0x82ca9f20
	ctx.lr = 0x83265E70;
	sub_82CA9F20(ctx, base);
	// 83265E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265E80 size=88
    let mut pc: u32 = 0x83265E80;
    'dispatch: loop {
        match pc {
            0x83265E80 => {
    //   block [0x83265E80..0x83265ED8)
	// 83265E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265E8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265E90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265E94: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265E98: 3BEBB7A8  addi r31, r11, -0x4858
	ctx.r[31].s64 = ctx.r[11].s64 + -18520;
	// 83265E9C: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 83265EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265EA4: 4AF8A39D  bl 0x821f0240
	ctx.lr = 0x83265EA8;
	sub_821F0240(ctx, base);
	// 83265EA8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265EB0: 3889235C  addi r4, r9, 0x235c
	ctx.r[4].s64 = ctx.r[9].s64 + 9052;
	// 83265EB4: 4AF74B0D  bl 0x821da9c0
	ctx.lr = 0x83265EB8;
	sub_821DA9C0(ctx, base);
	// 83265EB8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265EBC: 3868D4C0  addi r3, r8, -0x2b40
	ctx.r[3].s64 = ctx.r[8].s64 + -11072;
	// 83265EC0: 4BA44061  bl 0x82ca9f20
	ctx.lr = 0x83265EC4;
	sub_82CA9F20(ctx, base);
	// 83265EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265ED8 size=64
    let mut pc: u32 = 0x83265ED8;
    'dispatch: loop {
        match pc {
            0x83265ED8 => {
    //   block [0x83265ED8..0x83265F18)
	// 83265ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265EE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83265EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265EEC: 388B237C  addi r4, r11, 0x237c
	ctx.r[4].s64 = ctx.r[11].s64 + 9084;
	// 83265EF0: 386AB7AC  addi r3, r10, -0x4854
	ctx.r[3].s64 = ctx.r[10].s64 + -18516;
	// 83265EF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83265EF8: 4AFC6FD9  bl 0x8222ced0
	ctx.lr = 0x83265EFC;
	sub_8222CED0(ctx, base);
	// 83265EFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83265F00: 3869D4D0  addi r3, r9, -0x2b30
	ctx.r[3].s64 = ctx.r[9].s64 + -11056;
	// 83265F04: 4BA4401D  bl 0x82ca9f20
	ctx.lr = 0x83265F08;
	sub_82CA9F20(ctx, base);
	// 83265F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265F18 size=88
    let mut pc: u32 = 0x83265F18;
    'dispatch: loop {
        match pc {
            0x83265F18 => {
    //   block [0x83265F18..0x83265F70)
	// 83265F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265F28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265F2C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265F30: 3BEBB7B0  addi r31, r11, -0x4850
	ctx.r[31].s64 = ctx.r[11].s64 + -18512;
	// 83265F34: 388AB7AC  addi r4, r10, -0x4854
	ctx.r[4].s64 = ctx.r[10].s64 + -18516;
	// 83265F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265F3C: 4AF8A305  bl 0x821f0240
	ctx.lr = 0x83265F40;
	sub_821F0240(ctx, base);
	// 83265F40: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265F48: 3889235C  addi r4, r9, 0x235c
	ctx.r[4].s64 = ctx.r[9].s64 + 9052;
	// 83265F4C: 4AF74A75  bl 0x821da9c0
	ctx.lr = 0x83265F50;
	sub_821DA9C0(ctx, base);
	// 83265F50: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265F54: 3868D4E0  addi r3, r8, -0x2b20
	ctx.r[3].s64 = ctx.r[8].s64 + -11040;
	// 83265F58: 4BA43FC9  bl 0x82ca9f20
	ctx.lr = 0x83265F5C;
	sub_82CA9F20(ctx, base);
	// 83265F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265F68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265F70 size=88
    let mut pc: u32 = 0x83265F70;
    'dispatch: loop {
        match pc {
            0x83265F70 => {
    //   block [0x83265F70..0x83265FC8)
	// 83265F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265F7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265F80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265F84: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265F88: 3BEBB7B4  addi r31, r11, -0x484c
	ctx.r[31].s64 = ctx.r[11].s64 + -18508;
	// 83265F8C: 388AB7AC  addi r4, r10, -0x4854
	ctx.r[4].s64 = ctx.r[10].s64 + -18516;
	// 83265F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265F94: 4AF8A2AD  bl 0x821f0240
	ctx.lr = 0x83265F98;
	sub_821F0240(ctx, base);
	// 83265F98: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265FA0: 3889238C  addi r4, r9, 0x238c
	ctx.r[4].s64 = ctx.r[9].s64 + 9100;
	// 83265FA4: 4AF74A1D  bl 0x821da9c0
	ctx.lr = 0x83265FA8;
	sub_821DA9C0(ctx, base);
	// 83265FA8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83265FAC: 3868D4F0  addi r3, r8, -0x2b10
	ctx.r[3].s64 = ctx.r[8].s64 + -11024;
	// 83265FB0: 4BA43F71  bl 0x82ca9f20
	ctx.lr = 0x83265FB4;
	sub_82CA9F20(ctx, base);
	// 83265FB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83265FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83265FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83265FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83265FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83265FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83265FC8 size=88
    let mut pc: u32 = 0x83265FC8;
    'dispatch: loop {
        match pc {
            0x83265FC8 => {
    //   block [0x83265FC8..0x83266020)
	// 83265FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83265FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83265FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83265FD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83265FD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83265FDC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83265FE0: 3BEBB7B8  addi r31, r11, -0x4848
	ctx.r[31].s64 = ctx.r[11].s64 + -18504;
	// 83265FE4: 388AB7AC  addi r4, r10, -0x4854
	ctx.r[4].s64 = ctx.r[10].s64 + -18516;
	// 83265FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265FEC: 4AF8A255  bl 0x821f0240
	ctx.lr = 0x83265FF0;
	sub_821F0240(ctx, base);
	// 83265FF0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83265FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83265FF8: 38892398  addi r4, r9, 0x2398
	ctx.r[4].s64 = ctx.r[9].s64 + 9112;
	// 83265FFC: 4AF749C5  bl 0x821da9c0
	ctx.lr = 0x83266000;
	sub_821DA9C0(ctx, base);
	// 83266000: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83266004: 3868D500  addi r3, r8, -0x2b00
	ctx.r[3].s64 = ctx.r[8].s64 + -11008;
	// 83266008: 4BA43F19  bl 0x82ca9f20
	ctx.lr = 0x8326600C;
	sub_82CA9F20(ctx, base);
	// 8326600C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326601C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266020 size=88
    let mut pc: u32 = 0x83266020;
    'dispatch: loop {
        match pc {
            0x83266020 => {
    //   block [0x83266020..0x83266078)
	// 83266020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326602C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266030: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83266034: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266038: 3BEBB7BC  addi r31, r11, -0x4844
	ctx.r[31].s64 = ctx.r[11].s64 + -18500;
	// 8326603C: 388AB7AC  addi r4, r10, -0x4854
	ctx.r[4].s64 = ctx.r[10].s64 + -18516;
	// 83266040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266044: 4AF8A1FD  bl 0x821f0240
	ctx.lr = 0x83266048;
	sub_821F0240(ctx, base);
	// 83266048: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8326604C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266050: 388923A4  addi r4, r9, 0x23a4
	ctx.r[4].s64 = ctx.r[9].s64 + 9124;
	// 83266054: 4AF7496D  bl 0x821da9c0
	ctx.lr = 0x83266058;
	sub_821DA9C0(ctx, base);
	// 83266058: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326605C: 3868D510  addi r3, r8, -0x2af0
	ctx.r[3].s64 = ctx.r[8].s64 + -10992;
	// 83266060: 4BA43EC1  bl 0x82ca9f20
	ctx.lr = 0x83266064;
	sub_82CA9F20(ctx, base);
	// 83266064: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326606C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83266074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266078 size=88
    let mut pc: u32 = 0x83266078;
    'dispatch: loop {
        match pc {
            0x83266078 => {
    //   block [0x83266078..0x832660D0)
	// 83266078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83266084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266088: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326608C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266090: 3BEBB7C0  addi r31, r11, -0x4840
	ctx.r[31].s64 = ctx.r[11].s64 + -18496;
	// 83266094: 388AB7AC  addi r4, r10, -0x4854
	ctx.r[4].s64 = ctx.r[10].s64 + -18516;
	// 83266098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326609C: 4AF8A1A5  bl 0x821f0240
	ctx.lr = 0x832660A0;
	sub_821F0240(ctx, base);
	// 832660A0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832660A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832660A8: 388923AC  addi r4, r9, 0x23ac
	ctx.r[4].s64 = ctx.r[9].s64 + 9132;
	// 832660AC: 4AF74915  bl 0x821da9c0
	ctx.lr = 0x832660B0;
	sub_821DA9C0(ctx, base);
	// 832660B0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832660B4: 3868D520  addi r3, r8, -0x2ae0
	ctx.r[3].s64 = ctx.r[8].s64 + -10976;
	// 832660B8: 4BA43E69  bl 0x82ca9f20
	ctx.lr = 0x832660BC;
	sub_82CA9F20(ctx, base);
	// 832660BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832660C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832660C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832660C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832660CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832660D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832660D0 size=48
    let mut pc: u32 = 0x832660D0;
    'dispatch: loop {
        match pc {
            0x832660D0 => {
    //   block [0x832660D0..0x83266100)
	// 832660D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832660D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832660D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832660DC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832660E0: 386BB7B0  addi r3, r11, -0x4850
	ctx.r[3].s64 = ctx.r[11].s64 + -18512;
	// 832660E4: 4B14A955  bl 0x823b0a38
	ctx.lr = 0x832660E8;
	sub_823B0A38(ctx, base);
	// 832660E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832660EC: 906AB7C4  stw r3, -0x483c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18492 as u32), ctx.r[3].u32 ) };
	// 832660F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832660F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832660F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832660FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266100 size=64
    let mut pc: u32 = 0x83266100;
    'dispatch: loop {
        match pc {
            0x83266100 => {
    //   block [0x83266100..0x83266140)
	// 83266100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326610C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266114: 388B23B4  addi r4, r11, 0x23b4
	ctx.r[4].s64 = ctx.r[11].s64 + 9140;
	// 83266118: 386AB7C8  addi r3, r10, -0x4838
	ctx.r[3].s64 = ctx.r[10].s64 + -18488;
	// 8326611C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266120: 4AFC6DB1  bl 0x8222ced0
	ctx.lr = 0x83266124;
	sub_8222CED0(ctx, base);
	// 83266124: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266128: 3869D530  addi r3, r9, -0x2ad0
	ctx.r[3].s64 = ctx.r[9].s64 + -10960;
	// 8326612C: 4BA43DF5  bl 0x82ca9f20
	ctx.lr = 0x83266130;
	sub_82CA9F20(ctx, base);
	// 83266130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326613C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266140 size=64
    let mut pc: u32 = 0x83266140;
    'dispatch: loop {
        match pc {
            0x83266140 => {
    //   block [0x83266140..0x83266180)
	// 83266140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326614C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266154: 388B23C4  addi r4, r11, 0x23c4
	ctx.r[4].s64 = ctx.r[11].s64 + 9156;
	// 83266158: 386AB7CC  addi r3, r10, -0x4834
	ctx.r[3].s64 = ctx.r[10].s64 + -18484;
	// 8326615C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266160: 4AFC6D71  bl 0x8222ced0
	ctx.lr = 0x83266164;
	sub_8222CED0(ctx, base);
	// 83266164: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266168: 3869D540  addi r3, r9, -0x2ac0
	ctx.r[3].s64 = ctx.r[9].s64 + -10944;
	// 8326616C: 4BA43DB5  bl 0x82ca9f20
	ctx.lr = 0x83266170;
	sub_82CA9F20(ctx, base);
	// 83266170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326617C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266180 size=140
    let mut pc: u32 = 0x83266180;
    'dispatch: loop {
        match pc {
            0x83266180 => {
    //   block [0x83266180..0x832661D4)
	// 83266180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326618C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83266190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83266194: 388BB7C8  addi r4, r11, -0x4838
	ctx.r[4].s64 = ctx.r[11].s64 + -18488;
	// 83266198: 4AF8A0A9  bl 0x821f0240
	ctx.lr = 0x8326619C;
	sub_821F0240(ctx, base);
	// 8326619C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832661A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832661A4: 388A230C  addi r4, r10, 0x230c
	ctx.r[4].s64 = ctx.r[10].s64 + 8972;
	// 832661A8: 4AF74819  bl 0x821da9c0
	ctx.lr = 0x832661AC;
	sub_821DA9C0(ctx, base);
	// 832661AC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832661B0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832661B4: 38A9B7CC  addi r5, r9, -0x4834
	ctx.r[5].s64 = ctx.r[9].s64 + -18484;
	// 832661B8: 3868B7D0  addi r3, r8, -0x4830
	ctx.r[3].s64 = ctx.r[8].s64 + -18480;
	// 832661BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 832661C0: 4AF7CF49  bl 0x821e3108
	ctx.lr = 0x832661C4;
	sub_821E3108(ctx, base);
	// 832661C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832661C8: 4AF605A1  bl 0x821c6768
	ctx.lr = 0x832661CC;
	sub_821C6768(ctx, base);
	// 832661CC: 3CE08349  lis r7, -0x7cb7
	ctx.r[7].s64 = -2092367872;
	// 832661D0: 38877088  addi r4, r7, 0x7088
	ctx.r[4].s64 = ctx.r[7].s64 + 28808;
	pc = 0x832661D4; continue 'dispatch;
            }
            0x832661D4 => {
    //   block [0x832661D4..0x8326620C)
	// 832661D4: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 832661D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832661DC: 7CC02028  lwarx r6, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 832661E0: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 832661E4: 7CC0212D  stwcx. r6, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832661E8: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832661EC: 4082FFE8  bne 0x832661d4
	if !ctx.cr[0].eq {
	pc = 0x832661D4; continue 'dispatch;
	}
	// 832661F0: 3C60832B  lis r3, -0x7cd5
	ctx.r[3].s64 = -2094333952;
	// 832661F4: 3863D550  addi r3, r3, -0x2ab0
	ctx.r[3].s64 = ctx.r[3].s64 + -10928;
	// 832661F8: 4BA43D29  bl 0x82ca9f20
	ctx.lr = 0x832661FC;
	sub_82CA9F20(ctx, base);
	// 832661FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266210 size=88
    let mut pc: u32 = 0x83266210;
    'dispatch: loop {
        match pc {
            0x83266210 => {
    //   block [0x83266210..0x83266268)
	// 83266210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326621C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266220: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83266224: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266228: 3BEBB7D4  addi r31, r11, -0x482c
	ctx.r[31].s64 = ctx.r[11].s64 + -18476;
	// 8326622C: 388AB7C8  addi r4, r10, -0x4838
	ctx.r[4].s64 = ctx.r[10].s64 + -18488;
	// 83266230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266234: 4AF8A00D  bl 0x821f0240
	ctx.lr = 0x83266238;
	sub_821F0240(ctx, base);
	// 83266238: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8326623C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266240: 388923CC  addi r4, r9, 0x23cc
	ctx.r[4].s64 = ctx.r[9].s64 + 9164;
	// 83266244: 4AF7477D  bl 0x821da9c0
	ctx.lr = 0x83266248;
	sub_821DA9C0(ctx, base);
	// 83266248: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326624C: 3868D560  addi r3, r8, -0x2aa0
	ctx.r[3].s64 = ctx.r[8].s64 + -10912;
	// 83266250: 4BA43CD1  bl 0x82ca9f20
	ctx.lr = 0x83266254;
	sub_82CA9F20(ctx, base);
	// 83266254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326625C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83266264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266268 size=88
    let mut pc: u32 = 0x83266268;
    'dispatch: loop {
        match pc {
            0x83266268 => {
    //   block [0x83266268..0x832662C0)
	// 83266268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83266274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326627C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266280: 3BEBB7D8  addi r31, r11, -0x4828
	ctx.r[31].s64 = ctx.r[11].s64 + -18472;
	// 83266284: 388AB7C8  addi r4, r10, -0x4838
	ctx.r[4].s64 = ctx.r[10].s64 + -18488;
	// 83266288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326628C: 4AF89FB5  bl 0x821f0240
	ctx.lr = 0x83266290;
	sub_821F0240(ctx, base);
	// 83266290: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83266294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266298: 388923D4  addi r4, r9, 0x23d4
	ctx.r[4].s64 = ctx.r[9].s64 + 9172;
	// 8326629C: 4AF74725  bl 0x821da9c0
	ctx.lr = 0x832662A0;
	sub_821DA9C0(ctx, base);
	// 832662A0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832662A4: 3868D570  addi r3, r8, -0x2a90
	ctx.r[3].s64 = ctx.r[8].s64 + -10896;
	// 832662A8: 4BA43C79  bl 0x82ca9f20
	ctx.lr = 0x832662AC;
	sub_82CA9F20(ctx, base);
	// 832662AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832662B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832662B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832662B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832662BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832662C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832662C0 size=88
    let mut pc: u32 = 0x832662C0;
    'dispatch: loop {
        match pc {
            0x832662C0 => {
    //   block [0x832662C0..0x83266318)
	// 832662C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832662C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832662C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832662CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832662D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832662D4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832662D8: 3BEBB7DC  addi r31, r11, -0x4824
	ctx.r[31].s64 = ctx.r[11].s64 + -18468;
	// 832662DC: 388AB7C8  addi r4, r10, -0x4838
	ctx.r[4].s64 = ctx.r[10].s64 + -18488;
	// 832662E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832662E4: 4AF89F5D  bl 0x821f0240
	ctx.lr = 0x832662E8;
	sub_821F0240(ctx, base);
	// 832662E8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832662EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832662F0: 388923E8  addi r4, r9, 0x23e8
	ctx.r[4].s64 = ctx.r[9].s64 + 9192;
	// 832662F4: 4AF746CD  bl 0x821da9c0
	ctx.lr = 0x832662F8;
	sub_821DA9C0(ctx, base);
	// 832662F8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832662FC: 3868D580  addi r3, r8, -0x2a80
	ctx.r[3].s64 = ctx.r[8].s64 + -10880;
	// 83266300: 4BA43C21  bl 0x82ca9f20
	ctx.lr = 0x83266304;
	sub_82CA9F20(ctx, base);
	// 83266304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326630C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83266314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266318 size=88
    let mut pc: u32 = 0x83266318;
    'dispatch: loop {
        match pc {
            0x83266318 => {
    //   block [0x83266318..0x83266370)
	// 83266318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326631C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83266324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266328: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326632C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266330: 3BEBB7E0  addi r31, r11, -0x4820
	ctx.r[31].s64 = ctx.r[11].s64 + -18464;
	// 83266334: 388AB7C8  addi r4, r10, -0x4838
	ctx.r[4].s64 = ctx.r[10].s64 + -18488;
	// 83266338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326633C: 4AF89F05  bl 0x821f0240
	ctx.lr = 0x83266340;
	sub_821F0240(ctx, base);
	// 83266340: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83266344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266348: 388923F0  addi r4, r9, 0x23f0
	ctx.r[4].s64 = ctx.r[9].s64 + 9200;
	// 8326634C: 4AF74675  bl 0x821da9c0
	ctx.lr = 0x83266350;
	sub_821DA9C0(ctx, base);
	// 83266350: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83266354: 3868D590  addi r3, r8, -0x2a70
	ctx.r[3].s64 = ctx.r[8].s64 + -10864;
	// 83266358: 4BA43BC9  bl 0x82ca9f20
	ctx.lr = 0x8326635C;
	sub_82CA9F20(ctx, base);
	// 8326635C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326636C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266370 size=64
    let mut pc: u32 = 0x83266370;
    'dispatch: loop {
        match pc {
            0x83266370 => {
    //   block [0x83266370..0x832663B0)
	// 83266370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326637C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266384: 388B2404  addi r4, r11, 0x2404
	ctx.r[4].s64 = ctx.r[11].s64 + 9220;
	// 83266388: 386AB7E4  addi r3, r10, -0x481c
	ctx.r[3].s64 = ctx.r[10].s64 + -18460;
	// 8326638C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266390: 4AFC6B41  bl 0x8222ced0
	ctx.lr = 0x83266394;
	sub_8222CED0(ctx, base);
	// 83266394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266398: 3869D5A0  addi r3, r9, -0x2a60
	ctx.r[3].s64 = ctx.r[9].s64 + -10848;
	// 8326639C: 4BA43B85  bl 0x82ca9f20
	ctx.lr = 0x832663A0;
	sub_82CA9F20(ctx, base);
	// 832663A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832663A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832663A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832663AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832663B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832663B0 size=88
    let mut pc: u32 = 0x832663B0;
    'dispatch: loop {
        match pc {
            0x832663B0 => {
    //   block [0x832663B0..0x83266408)
	// 832663B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832663B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832663B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832663BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832663C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832663C4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832663C8: 3BEBB7E8  addi r31, r11, -0x4818
	ctx.r[31].s64 = ctx.r[11].s64 + -18456;
	// 832663CC: 388AB7E4  addi r4, r10, -0x481c
	ctx.r[4].s64 = ctx.r[10].s64 + -18460;
	// 832663D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832663D4: 4AF89E6D  bl 0x821f0240
	ctx.lr = 0x832663D8;
	sub_821F0240(ctx, base);
	// 832663D8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832663DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832663E0: 38892410  addi r4, r9, 0x2410
	ctx.r[4].s64 = ctx.r[9].s64 + 9232;
	// 832663E4: 4AF745DD  bl 0x821da9c0
	ctx.lr = 0x832663E8;
	sub_821DA9C0(ctx, base);
	// 832663E8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832663EC: 3868D5B0  addi r3, r8, -0x2a50
	ctx.r[3].s64 = ctx.r[8].s64 + -10832;
	// 832663F0: 4BA43B31  bl 0x82ca9f20
	ctx.lr = 0x832663F4;
	sub_82CA9F20(ctx, base);
	// 832663F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832663F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832663FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83266404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266408 size=88
    let mut pc: u32 = 0x83266408;
    'dispatch: loop {
        match pc {
            0x83266408 => {
    //   block [0x83266408..0x83266460)
	// 83266408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326640C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266410: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83266414: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266418: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326641C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266420: 3BEBB7EC  addi r31, r11, -0x4814
	ctx.r[31].s64 = ctx.r[11].s64 + -18452;
	// 83266424: 388AB7E4  addi r4, r10, -0x481c
	ctx.r[4].s64 = ctx.r[10].s64 + -18460;
	// 83266428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326642C: 4AF89E15  bl 0x821f0240
	ctx.lr = 0x83266430;
	sub_821F0240(ctx, base);
	// 83266430: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83266434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266438: 38892420  addi r4, r9, 0x2420
	ctx.r[4].s64 = ctx.r[9].s64 + 9248;
	// 8326643C: 4AF74585  bl 0x821da9c0
	ctx.lr = 0x83266440;
	sub_821DA9C0(ctx, base);
	// 83266440: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83266444: 3868D5C0  addi r3, r8, -0x2a40
	ctx.r[3].s64 = ctx.r[8].s64 + -10816;
	// 83266448: 4BA43AD9  bl 0x82ca9f20
	ctx.lr = 0x8326644C;
	sub_82CA9F20(ctx, base);
	// 8326644C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326645C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266460 size=88
    let mut pc: u32 = 0x83266460;
    'dispatch: loop {
        match pc {
            0x83266460 => {
    //   block [0x83266460..0x832664B8)
	// 83266460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326646C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266470: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83266474: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266478: 3BEBB7F0  addi r31, r11, -0x4810
	ctx.r[31].s64 = ctx.r[11].s64 + -18448;
	// 8326647C: 388AB7E4  addi r4, r10, -0x481c
	ctx.r[4].s64 = ctx.r[10].s64 + -18460;
	// 83266480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266484: 4AF89DBD  bl 0x821f0240
	ctx.lr = 0x83266488;
	sub_821F0240(ctx, base);
	// 83266488: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8326648C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266490: 3889235C  addi r4, r9, 0x235c
	ctx.r[4].s64 = ctx.r[9].s64 + 9052;
	// 83266494: 4AF7452D  bl 0x821da9c0
	ctx.lr = 0x83266498;
	sub_821DA9C0(ctx, base);
	// 83266498: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326649C: 3868D5D0  addi r3, r8, -0x2a30
	ctx.r[3].s64 = ctx.r[8].s64 + -10800;
	// 832664A0: 4BA43A81  bl 0x82ca9f20
	ctx.lr = 0x832664A4;
	sub_82CA9F20(ctx, base);
	// 832664A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832664A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832664AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832664B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832664B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832664B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832664B8 size=64
    let mut pc: u32 = 0x832664B8;
    'dispatch: loop {
        match pc {
            0x832664B8 => {
    //   block [0x832664B8..0x832664F8)
	// 832664B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832664BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832664C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832664C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832664C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832664CC: 388B242C  addi r4, r11, 0x242c
	ctx.r[4].s64 = ctx.r[11].s64 + 9260;
	// 832664D0: 386AB7F4  addi r3, r10, -0x480c
	ctx.r[3].s64 = ctx.r[10].s64 + -18444;
	// 832664D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832664D8: 4AFC69F9  bl 0x8222ced0
	ctx.lr = 0x832664DC;
	sub_8222CED0(ctx, base);
	// 832664DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832664E0: 3869D5E0  addi r3, r9, -0x2a20
	ctx.r[3].s64 = ctx.r[9].s64 + -10784;
	// 832664E4: 4BA43A3D  bl 0x82ca9f20
	ctx.lr = 0x832664E8;
	sub_82CA9F20(ctx, base);
	// 832664E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832664EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832664F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832664F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832664F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832664F8 size=64
    let mut pc: u32 = 0x832664F8;
    'dispatch: loop {
        match pc {
            0x832664F8 => {
    //   block [0x832664F8..0x83266538)
	// 832664F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832664FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266504: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326650C: 388B243C  addi r4, r11, 0x243c
	ctx.r[4].s64 = ctx.r[11].s64 + 9276;
	// 83266510: 386AB7F8  addi r3, r10, -0x4808
	ctx.r[3].s64 = ctx.r[10].s64 + -18440;
	// 83266514: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266518: 4AFC69B9  bl 0x8222ced0
	ctx.lr = 0x8326651C;
	sub_8222CED0(ctx, base);
	// 8326651C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266520: 3869D5F0  addi r3, r9, -0x2a10
	ctx.r[3].s64 = ctx.r[9].s64 + -10768;
	// 83266524: 4BA439FD  bl 0x82ca9f20
	ctx.lr = 0x83266528;
	sub_82CA9F20(ctx, base);
	// 83266528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326652C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266538 size=64
    let mut pc: u32 = 0x83266538;
    'dispatch: loop {
        match pc {
            0x83266538 => {
    //   block [0x83266538..0x83266578)
	// 83266538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326653C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266544: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266548: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326654C: 388B2450  addi r4, r11, 0x2450
	ctx.r[4].s64 = ctx.r[11].s64 + 9296;
	// 83266550: 386AB7FC  addi r3, r10, -0x4804
	ctx.r[3].s64 = ctx.r[10].s64 + -18436;
	// 83266554: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266558: 4AFC6979  bl 0x8222ced0
	ctx.lr = 0x8326655C;
	sub_8222CED0(ctx, base);
	// 8326655C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266560: 3869D600  addi r3, r9, -0x2a00
	ctx.r[3].s64 = ctx.r[9].s64 + -10752;
	// 83266564: 4BA439BD  bl 0x82ca9f20
	ctx.lr = 0x83266568;
	sub_82CA9F20(ctx, base);
	// 83266568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266578 size=88
    let mut pc: u32 = 0x83266578;
    'dispatch: loop {
        match pc {
            0x83266578 => {
    //   block [0x83266578..0x832665D0)
	// 83266578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83266584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266588: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326658C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266590: 3BEBB800  addi r31, r11, -0x4800
	ctx.r[31].s64 = ctx.r[11].s64 + -18432;
	// 83266594: 388AB7FC  addi r4, r10, -0x4804
	ctx.r[4].s64 = ctx.r[10].s64 + -18436;
	// 83266598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8326659C: 4AF89CA5  bl 0x821f0240
	ctx.lr = 0x832665A0;
	sub_821F0240(ctx, base);
	// 832665A0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832665A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832665A8: 3889245C  addi r4, r9, 0x245c
	ctx.r[4].s64 = ctx.r[9].s64 + 9308;
	// 832665AC: 4AF74415  bl 0x821da9c0
	ctx.lr = 0x832665B0;
	sub_821DA9C0(ctx, base);
	// 832665B0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832665B4: 3868D610  addi r3, r8, -0x29f0
	ctx.r[3].s64 = ctx.r[8].s64 + -10736;
	// 832665B8: 4BA43969  bl 0x82ca9f20
	ctx.lr = 0x832665BC;
	sub_82CA9F20(ctx, base);
	// 832665BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832665C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832665C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832665C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832665CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832665D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832665D0 size=88
    let mut pc: u32 = 0x832665D0;
    'dispatch: loop {
        match pc {
            0x832665D0 => {
    //   block [0x832665D0..0x83266628)
	// 832665D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832665D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832665D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832665DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832665E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832665E4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832665E8: 3BEBB804  addi r31, r11, -0x47fc
	ctx.r[31].s64 = ctx.r[11].s64 + -18428;
	// 832665EC: 388AB7FC  addi r4, r10, -0x4804
	ctx.r[4].s64 = ctx.r[10].s64 + -18436;
	// 832665F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832665F4: 4AF89C4D  bl 0x821f0240
	ctx.lr = 0x832665F8;
	sub_821F0240(ctx, base);
	// 832665F8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832665FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83266600: 38892468  addi r4, r9, 0x2468
	ctx.r[4].s64 = ctx.r[9].s64 + 9320;
	// 83266604: 4AF743BD  bl 0x821da9c0
	ctx.lr = 0x83266608;
	sub_821DA9C0(ctx, base);
	// 83266608: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326660C: 3868D620  addi r3, r8, -0x29e0
	ctx.r[3].s64 = ctx.r[8].s64 + -10720;
	// 83266610: 4BA43911  bl 0x82ca9f20
	ctx.lr = 0x83266614;
	sub_82CA9F20(ctx, base);
	// 83266614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326661C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83266624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266628 size=64
    let mut pc: u32 = 0x83266628;
    'dispatch: loop {
        match pc {
            0x83266628 => {
    //   block [0x83266628..0x83266668)
	// 83266628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326662C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266634: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266638: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326663C: 388B2470  addi r4, r11, 0x2470
	ctx.r[4].s64 = ctx.r[11].s64 + 9328;
	// 83266640: 386AB808  addi r3, r10, -0x47f8
	ctx.r[3].s64 = ctx.r[10].s64 + -18424;
	// 83266644: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266648: 4AFC6889  bl 0x8222ced0
	ctx.lr = 0x8326664C;
	sub_8222CED0(ctx, base);
	// 8326664C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266650: 3869D630  addi r3, r9, -0x29d0
	ctx.r[3].s64 = ctx.r[9].s64 + -10704;
	// 83266654: 4BA438CD  bl 0x82ca9f20
	ctx.lr = 0x83266658;
	sub_82CA9F20(ctx, base);
	// 83266658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326665C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266668 size=64
    let mut pc: u32 = 0x83266668;
    'dispatch: loop {
        match pc {
            0x83266668 => {
    //   block [0x83266668..0x832666A8)
	// 83266668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266674: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83266678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326667C: 388B3E18  addi r4, r11, 0x3e18
	ctx.r[4].s64 = ctx.r[11].s64 + 15896;
	// 83266680: 386AB80C  addi r3, r10, -0x47f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18420;
	// 83266684: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266688: 4AFC6849  bl 0x8222ced0
	ctx.lr = 0x8326668C;
	sub_8222CED0(ctx, base);
	// 8326668C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266690: 3869D640  addi r3, r9, -0x29c0
	ctx.r[3].s64 = ctx.r[9].s64 + -10688;
	// 83266694: 4BA4388D  bl 0x82ca9f20
	ctx.lr = 0x83266698;
	sub_82CA9F20(ctx, base);
	// 83266698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326669C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832666A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832666A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832666A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832666A8 size=64
    let mut pc: u32 = 0x832666A8;
    'dispatch: loop {
        match pc {
            0x832666A8 => {
    //   block [0x832666A8..0x832666E8)
	// 832666A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832666AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832666B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832666B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832666B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832666BC: 388B2484  addi r4, r11, 0x2484
	ctx.r[4].s64 = ctx.r[11].s64 + 9348;
	// 832666C0: 386AB810  addi r3, r10, -0x47f0
	ctx.r[3].s64 = ctx.r[10].s64 + -18416;
	// 832666C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832666C8: 4AFC6809  bl 0x8222ced0
	ctx.lr = 0x832666CC;
	sub_8222CED0(ctx, base);
	// 832666CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832666D0: 3869D650  addi r3, r9, -0x29b0
	ctx.r[3].s64 = ctx.r[9].s64 + -10672;
	// 832666D4: 4BA4384D  bl 0x82ca9f20
	ctx.lr = 0x832666D8;
	sub_82CA9F20(ctx, base);
	// 832666D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832666DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832666E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832666E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832666E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832666E8 size=64
    let mut pc: u32 = 0x832666E8;
    'dispatch: loop {
        match pc {
            0x832666E8 => {
    //   block [0x832666E8..0x83266728)
	// 832666E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832666EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832666F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832666F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832666F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832666FC: 388B24A4  addi r4, r11, 0x24a4
	ctx.r[4].s64 = ctx.r[11].s64 + 9380;
	// 83266700: 386AB814  addi r3, r10, -0x47ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18412;
	// 83266704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266708: 4AFC67C9  bl 0x8222ced0
	ctx.lr = 0x8326670C;
	sub_8222CED0(ctx, base);
	// 8326670C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266710: 3869D660  addi r3, r9, -0x29a0
	ctx.r[3].s64 = ctx.r[9].s64 + -10656;
	// 83266714: 4BA4380D  bl 0x82ca9f20
	ctx.lr = 0x83266718;
	sub_82CA9F20(ctx, base);
	// 83266718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326671C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266728 size=64
    let mut pc: u32 = 0x83266728;
    'dispatch: loop {
        match pc {
            0x83266728 => {
    //   block [0x83266728..0x83266768)
	// 83266728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266734: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326673C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266740: 386AB818  addi r3, r10, -0x47e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18408;
	// 83266744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266748: 4AFC6789  bl 0x8222ced0
	ctx.lr = 0x8326674C;
	sub_8222CED0(ctx, base);
	// 8326674C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266750: 3869D670  addi r3, r9, -0x2990
	ctx.r[3].s64 = ctx.r[9].s64 + -10640;
	// 83266754: 4BA437CD  bl 0x82ca9f20
	ctx.lr = 0x83266758;
	sub_82CA9F20(ctx, base);
	// 83266758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266768 size=64
    let mut pc: u32 = 0x83266768;
    'dispatch: loop {
        match pc {
            0x83266768 => {
    //   block [0x83266768..0x832667A8)
	// 83266768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326676C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266774: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326677C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266780: 386AB81C  addi r3, r10, -0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18404;
	// 83266784: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266788: 4AFC6749  bl 0x8222ced0
	ctx.lr = 0x8326678C;
	sub_8222CED0(ctx, base);
	// 8326678C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266790: 3869D680  addi r3, r9, -0x2980
	ctx.r[3].s64 = ctx.r[9].s64 + -10624;
	// 83266794: 4BA4378D  bl 0x82ca9f20
	ctx.lr = 0x83266798;
	sub_82CA9F20(ctx, base);
	// 83266798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326679C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832667A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832667A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832667A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832667A8 size=64
    let mut pc: u32 = 0x832667A8;
    'dispatch: loop {
        match pc {
            0x832667A8 => {
    //   block [0x832667A8..0x832667E8)
	// 832667A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832667AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832667B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832667B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832667B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832667BC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832667C0: 386AB820  addi r3, r10, -0x47e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18400;
	// 832667C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832667C8: 4AFC6709  bl 0x8222ced0
	ctx.lr = 0x832667CC;
	sub_8222CED0(ctx, base);
	// 832667CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832667D0: 3869D690  addi r3, r9, -0x2970
	ctx.r[3].s64 = ctx.r[9].s64 + -10608;
	// 832667D4: 4BA4374D  bl 0x82ca9f20
	ctx.lr = 0x832667D8;
	sub_82CA9F20(ctx, base);
	// 832667D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832667DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832667E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832667E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832667E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832667E8 size=12
    let mut pc: u32 = 0x832667E8;
    'dispatch: loop {
        match pc {
            0x832667E8 => {
    //   block [0x832667E8..0x832667F4)
	// 832667E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832667EC: 386BD6A0  addi r3, r11, -0x2960
	ctx.r[3].s64 = ctx.r[11].s64 + -10592;
	// 832667F0: 4BA43730  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832667F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832667F8 size=64
    let mut pc: u32 = 0x832667F8;
    'dispatch: loop {
        match pc {
            0x832667F8 => {
    //   block [0x832667F8..0x83266838)
	// 832667F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832667FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266808: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326680C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266810: 386AB834  addi r3, r10, -0x47cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18380;
	// 83266814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266818: 4AFC66B9  bl 0x8222ced0
	ctx.lr = 0x8326681C;
	sub_8222CED0(ctx, base);
	// 8326681C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266820: 3869D710  addi r3, r9, -0x28f0
	ctx.r[3].s64 = ctx.r[9].s64 + -10480;
	// 83266824: 4BA436FD  bl 0x82ca9f20
	ctx.lr = 0x83266828;
	sub_82CA9F20(ctx, base);
	// 83266828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326682C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266838 size=64
    let mut pc: u32 = 0x83266838;
    'dispatch: loop {
        match pc {
            0x83266838 => {
    //   block [0x83266838..0x83266878)
	// 83266838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326683C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326684C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266850: 386AB838  addi r3, r10, -0x47c8
	ctx.r[3].s64 = ctx.r[10].s64 + -18376;
	// 83266854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266858: 4AFC6679  bl 0x8222ced0
	ctx.lr = 0x8326685C;
	sub_8222CED0(ctx, base);
	// 8326685C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266860: 3869D720  addi r3, r9, -0x28e0
	ctx.r[3].s64 = ctx.r[9].s64 + -10464;
	// 83266864: 4BA436BD  bl 0x82ca9f20
	ctx.lr = 0x83266868;
	sub_82CA9F20(ctx, base);
	// 83266868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326686C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266878 size=64
    let mut pc: u32 = 0x83266878;
    'dispatch: loop {
        match pc {
            0x83266878 => {
    //   block [0x83266878..0x832668B8)
	// 83266878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326687C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326688C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266890: 386AB83C  addi r3, r10, -0x47c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18372;
	// 83266894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266898: 4AFC6639  bl 0x8222ced0
	ctx.lr = 0x8326689C;
	sub_8222CED0(ctx, base);
	// 8326689C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832668A0: 3869D730  addi r3, r9, -0x28d0
	ctx.r[3].s64 = ctx.r[9].s64 + -10448;
	// 832668A4: 4BA4367D  bl 0x82ca9f20
	ctx.lr = 0x832668A8;
	sub_82CA9F20(ctx, base);
	// 832668A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832668AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832668B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832668B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


