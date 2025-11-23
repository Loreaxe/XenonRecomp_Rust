pub fn sub_832B9398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9398 size=40
    let mut pc: u32 = 0x832B9398;
    'dispatch: loop {
        match pc {
            0x832B9398 => {
    //   block [0x832B9398..0x832B93C0)
	// 832B9398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B93A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B93A4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B93A8: 386B9870  addi r3, r11, -0x6790
	ctx.r[3].s64 = ctx.r[11].s64 + -26512;
	// 832B93AC: 4BD4A21D  bl 0x830035c8
	ctx.lr = 0x832B93B0;
	sub_830035C8(ctx, base);
	// 832B93B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B93B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B93B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B93BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B93C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B93C0 size=40
    let mut pc: u32 = 0x832B93C0;
    'dispatch: loop {
        match pc {
            0x832B93C0 => {
    //   block [0x832B93C0..0x832B93E8)
	// 832B93C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B93C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B93C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B93CC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B93D0: 386B9864  addi r3, r11, -0x679c
	ctx.r[3].s64 = ctx.r[11].s64 + -26524;
	// 832B93D4: 4BD4A2C5  bl 0x83003698
	ctx.lr = 0x832B93D8;
	sub_83003698(ctx, base);
	// 832B93D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B93DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B93E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B93E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B93E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B93E8 size=40
    let mut pc: u32 = 0x832B93E8;
    'dispatch: loop {
        match pc {
            0x832B93E8 => {
    //   block [0x832B93E8..0x832B9410)
	// 832B93E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B93EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B93F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B93F4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B93F8: 386B98A0  addi r3, r11, -0x6760
	ctx.r[3].s64 = ctx.r[11].s64 + -26464;
	// 832B93FC: 4BD4A1CD  bl 0x830035c8
	ctx.lr = 0x832B9400;
	sub_830035C8(ctx, base);
	// 832B9400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9410 size=40
    let mut pc: u32 = 0x832B9410;
    'dispatch: loop {
        match pc {
            0x832B9410 => {
    //   block [0x832B9410..0x832B9438)
	// 832B9410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B941C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9420: 386B9894  addi r3, r11, -0x676c
	ctx.r[3].s64 = ctx.r[11].s64 + -26476;
	// 832B9424: 4BD4A275  bl 0x83003698
	ctx.lr = 0x832B9428;
	sub_83003698(ctx, base);
	// 832B9428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B942C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9438 size=40
    let mut pc: u32 = 0x832B9438;
    'dispatch: loop {
        match pc {
            0x832B9438 => {
    //   block [0x832B9438..0x832B9460)
	// 832B9438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9444: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9448: 386B98D0  addi r3, r11, -0x6730
	ctx.r[3].s64 = ctx.r[11].s64 + -26416;
	// 832B944C: 4BD4A17D  bl 0x830035c8
	ctx.lr = 0x832B9450;
	sub_830035C8(ctx, base);
	// 832B9450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B945C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9460 size=40
    let mut pc: u32 = 0x832B9460;
    'dispatch: loop {
        match pc {
            0x832B9460 => {
    //   block [0x832B9460..0x832B9488)
	// 832B9460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B946C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9470: 386B98C4  addi r3, r11, -0x673c
	ctx.r[3].s64 = ctx.r[11].s64 + -26428;
	// 832B9474: 4BD4A225  bl 0x83003698
	ctx.lr = 0x832B9478;
	sub_83003698(ctx, base);
	// 832B9478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B947C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9488 size=40
    let mut pc: u32 = 0x832B9488;
    'dispatch: loop {
        match pc {
            0x832B9488 => {
    //   block [0x832B9488..0x832B94B0)
	// 832B9488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B948C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9494: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9498: 386B9940  addi r3, r11, -0x66c0
	ctx.r[3].s64 = ctx.r[11].s64 + -26304;
	// 832B949C: 4BD4A12D  bl 0x830035c8
	ctx.lr = 0x832B94A0;
	sub_830035C8(ctx, base);
	// 832B94A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B94A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B94A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B94AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B94B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B94B0 size=40
    let mut pc: u32 = 0x832B94B0;
    'dispatch: loop {
        match pc {
            0x832B94B0 => {
    //   block [0x832B94B0..0x832B94D8)
	// 832B94B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B94B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B94B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B94BC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B94C0: 386B9934  addi r3, r11, -0x66cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26316;
	// 832B94C4: 4BD4A1D5  bl 0x83003698
	ctx.lr = 0x832B94C8;
	sub_83003698(ctx, base);
	// 832B94C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B94CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B94D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B94D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B94D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B94D8 size=40
    let mut pc: u32 = 0x832B94D8;
    'dispatch: loop {
        match pc {
            0x832B94D8 => {
    //   block [0x832B94D8..0x832B9500)
	// 832B94D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B94DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B94E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B94E4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B94E8: 386B9A00  addi r3, r11, -0x6600
	ctx.r[3].s64 = ctx.r[11].s64 + -26112;
	// 832B94EC: 4BD4A0DD  bl 0x830035c8
	ctx.lr = 0x832B94F0;
	sub_830035C8(ctx, base);
	// 832B94F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B94F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B94F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B94FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9500 size=40
    let mut pc: u32 = 0x832B9500;
    'dispatch: loop {
        match pc {
            0x832B9500 => {
    //   block [0x832B9500..0x832B9528)
	// 832B9500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B950C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9510: 386B99F4  addi r3, r11, -0x660c
	ctx.r[3].s64 = ctx.r[11].s64 + -26124;
	// 832B9514: 4BD4A185  bl 0x83003698
	ctx.lr = 0x832B9518;
	sub_83003698(ctx, base);
	// 832B9518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B951C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9528 size=40
    let mut pc: u32 = 0x832B9528;
    'dispatch: loop {
        match pc {
            0x832B9528 => {
    //   block [0x832B9528..0x832B9550)
	// 832B9528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9534: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9538: 386B99D0  addi r3, r11, -0x6630
	ctx.r[3].s64 = ctx.r[11].s64 + -26160;
	// 832B953C: 4BD4A08D  bl 0x830035c8
	ctx.lr = 0x832B9540;
	sub_830035C8(ctx, base);
	// 832B9540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B954C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9550 size=40
    let mut pc: u32 = 0x832B9550;
    'dispatch: loop {
        match pc {
            0x832B9550 => {
    //   block [0x832B9550..0x832B9578)
	// 832B9550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B955C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9560: 386B99C4  addi r3, r11, -0x663c
	ctx.r[3].s64 = ctx.r[11].s64 + -26172;
	// 832B9564: 4BD4A135  bl 0x83003698
	ctx.lr = 0x832B9568;
	sub_83003698(ctx, base);
	// 832B9568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B956C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9578 size=40
    let mut pc: u32 = 0x832B9578;
    'dispatch: loop {
        match pc {
            0x832B9578 => {
    //   block [0x832B9578..0x832B95A0)
	// 832B9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9584: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9588: 386B99A0  addi r3, r11, -0x6660
	ctx.r[3].s64 = ctx.r[11].s64 + -26208;
	// 832B958C: 4BD4A03D  bl 0x830035c8
	ctx.lr = 0x832B9590;
	sub_830035C8(ctx, base);
	// 832B9590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B959C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B95A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B95A0 size=40
    let mut pc: u32 = 0x832B95A0;
    'dispatch: loop {
        match pc {
            0x832B95A0 => {
    //   block [0x832B95A0..0x832B95C8)
	// 832B95A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B95A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B95A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B95AC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B95B0: 386B9994  addi r3, r11, -0x666c
	ctx.r[3].s64 = ctx.r[11].s64 + -26220;
	// 832B95B4: 4BD4A0E5  bl 0x83003698
	ctx.lr = 0x832B95B8;
	sub_83003698(ctx, base);
	// 832B95B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B95BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B95C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B95C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B95C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B95C8 size=40
    let mut pc: u32 = 0x832B95C8;
    'dispatch: loop {
        match pc {
            0x832B95C8 => {
    //   block [0x832B95C8..0x832B95F0)
	// 832B95C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B95CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B95D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B95D4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B95D8: 386B9970  addi r3, r11, -0x6690
	ctx.r[3].s64 = ctx.r[11].s64 + -26256;
	// 832B95DC: 4BD49FED  bl 0x830035c8
	ctx.lr = 0x832B95E0;
	sub_830035C8(ctx, base);
	// 832B95E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B95E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B95E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B95EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B95F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B95F0 size=40
    let mut pc: u32 = 0x832B95F0;
    'dispatch: loop {
        match pc {
            0x832B95F0 => {
    //   block [0x832B95F0..0x832B9618)
	// 832B95F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B95F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B95F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B95FC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9600: 386B9964  addi r3, r11, -0x669c
	ctx.r[3].s64 = ctx.r[11].s64 + -26268;
	// 832B9604: 4BD4A095  bl 0x83003698
	ctx.lr = 0x832B9608;
	sub_83003698(ctx, base);
	// 832B9608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B960C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9618 size=40
    let mut pc: u32 = 0x832B9618;
    'dispatch: loop {
        match pc {
            0x832B9618 => {
    //   block [0x832B9618..0x832B9640)
	// 832B9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9624: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9628: 386B9A34  addi r3, r11, -0x65cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26060;
	// 832B962C: 4BD49F9D  bl 0x830035c8
	ctx.lr = 0x832B9630;
	sub_830035C8(ctx, base);
	// 832B9630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B963C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9640 size=40
    let mut pc: u32 = 0x832B9640;
    'dispatch: loop {
        match pc {
            0x832B9640 => {
    //   block [0x832B9640..0x832B9668)
	// 832B9640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B964C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9650: 386B9A28  addi r3, r11, -0x65d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26072;
	// 832B9654: 4BD4A045  bl 0x83003698
	ctx.lr = 0x832B9658;
	sub_83003698(ctx, base);
	// 832B9658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B965C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9668 size=40
    let mut pc: u32 = 0x832B9668;
    'dispatch: loop {
        match pc {
            0x832B9668 => {
    //   block [0x832B9668..0x832B9690)
	// 832B9668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B966C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9674: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9678: 386B9A64  addi r3, r11, -0x659c
	ctx.r[3].s64 = ctx.r[11].s64 + -26012;
	// 832B967C: 4BD49F4D  bl 0x830035c8
	ctx.lr = 0x832B9680;
	sub_830035C8(ctx, base);
	// 832B9680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B968C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9690 size=40
    let mut pc: u32 = 0x832B9690;
    'dispatch: loop {
        match pc {
            0x832B9690 => {
    //   block [0x832B9690..0x832B96B8)
	// 832B9690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B969C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B96A0: 386B9A58  addi r3, r11, -0x65a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26024;
	// 832B96A4: 4BD49FF5  bl 0x83003698
	ctx.lr = 0x832B96A8;
	sub_83003698(ctx, base);
	// 832B96A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B96AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B96B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B96B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B96B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B96B8 size=40
    let mut pc: u32 = 0x832B96B8;
    'dispatch: loop {
        match pc {
            0x832B96B8 => {
    //   block [0x832B96B8..0x832B96E0)
	// 832B96B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B96BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B96C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B96C4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B96C8: 386B9AC4  addi r3, r11, -0x653c
	ctx.r[3].s64 = ctx.r[11].s64 + -25916;
	// 832B96CC: 4BD49EFD  bl 0x830035c8
	ctx.lr = 0x832B96D0;
	sub_830035C8(ctx, base);
	// 832B96D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B96D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B96D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B96DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B96E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B96E0 size=40
    let mut pc: u32 = 0x832B96E0;
    'dispatch: loop {
        match pc {
            0x832B96E0 => {
    //   block [0x832B96E0..0x832B9708)
	// 832B96E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B96E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B96E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B96EC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B96F0: 386B9AB8  addi r3, r11, -0x6548
	ctx.r[3].s64 = ctx.r[11].s64 + -25928;
	// 832B96F4: 4BD49FA5  bl 0x83003698
	ctx.lr = 0x832B96F8;
	sub_83003698(ctx, base);
	// 832B96F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B96FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9708 size=40
    let mut pc: u32 = 0x832B9708;
    'dispatch: loop {
        match pc {
            0x832B9708 => {
    //   block [0x832B9708..0x832B9730)
	// 832B9708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9714: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9718: 386B9A94  addi r3, r11, -0x656c
	ctx.r[3].s64 = ctx.r[11].s64 + -25964;
	// 832B971C: 4BD49EAD  bl 0x830035c8
	ctx.lr = 0x832B9720;
	sub_830035C8(ctx, base);
	// 832B9720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9730 size=40
    let mut pc: u32 = 0x832B9730;
    'dispatch: loop {
        match pc {
            0x832B9730 => {
    //   block [0x832B9730..0x832B9758)
	// 832B9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B973C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9740: 386B9A88  addi r3, r11, -0x6578
	ctx.r[3].s64 = ctx.r[11].s64 + -25976;
	// 832B9744: 4BD49F55  bl 0x83003698
	ctx.lr = 0x832B9748;
	sub_83003698(ctx, base);
	// 832B9748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B974C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B9754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9758 size=40
    let mut pc: u32 = 0x832B9758;
    'dispatch: loop {
        match pc {
            0x832B9758 => {
    //   block [0x832B9758..0x832B9780)
	// 832B9758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B9764: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9768: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 832B976C: 4BD4BA05  bl 0x83005170
	ctx.lr = 0x832B9770;
	sub_83005170(ctx, base);
	// 832B9770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B9774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B9778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B977C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B9780 size=40
    let mut pc: u32 = 0x832B9780;
    'dispatch: loop {
        match pc {
            0x832B9780 => {
    //   block [0x832B9780..0x832B97A8)
	// 832B9780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B9784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B9788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B978C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9790: 386B9B44  addi r3, r11, -0x64bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25788;
	// 832B9794: 4BD49E35  bl 0x830035c8
	ctx.lr = 0x832B9798;
	sub_830035C8(ctx, base);
	// 832B9798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B979C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B97A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B97A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B97A8 size=40
    let mut pc: u32 = 0x832B97A8;
    'dispatch: loop {
        match pc {
            0x832B97A8 => {
    //   block [0x832B97A8..0x832B97D0)
	// 832B97A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B97AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B97B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B97B4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B97B8: 386B9B68  addi r3, r11, -0x6498
	ctx.r[3].s64 = ctx.r[11].s64 + -25752;
	// 832B97BC: 4BD4CA7D  bl 0x83006238
	ctx.lr = 0x832B97C0;
	sub_83006238(ctx, base);
	// 832B97C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B97C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B97C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B97CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97D0 size=16
    let mut pc: u32 = 0x832B97D0;
    'dispatch: loop {
        match pc {
            0x832B97D0 => {
    //   block [0x832B97D0..0x832B97E0)
	// 832B97D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B97D4: 396BB0A0  addi r11, r11, -0x4f60
	ctx.r[11].s64 = ctx.r[11].s64 + -20320;
	// 832B97D8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 832B97DC: 4BAC5F2C  b 0x82d7f708
	sub_82D7F708(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97E0 size=20
    let mut pc: u32 = 0x832B97E0;
    'dispatch: loop {
        match pc {
            0x832B97E0 => {
    //   block [0x832B97E0..0x832B97F4)
	// 832B97E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832B97E4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B97E8: 396B9128  addi r11, r11, -0x6ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -28376;
	// 832B97EC: 916A1F68  stw r11, 0x1f68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8040 as u32), ctx.r[11].u32 ) };
	// 832B97F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B97F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B97F8 size=52
    let mut pc: u32 = 0x832B97F8;
    'dispatch: loop {
        match pc {
            0x832B97F8 => {
    //   block [0x832B97F8..0x832B9810)
	// 832B97F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B97FC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832B9800: 390BB4A0  addi r8, r11, -0x4b60
	ctx.r[8].s64 = ctx.r[11].s64 + -19296;
	// 832B9804: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 832B9808: 39680284  addi r11, r8, 0x284
	ctx.r[11].s64 = ctx.r[8].s64 + 644;
	// 832B980C: 39299128  addi r9, r9, -0x6ed8
	ctx.r[9].s64 = ctx.r[9].s64 + -28376;
	pc = 0x832B9810; continue 'dispatch;
            }
            0x832B9810 => {
    //   block [0x832B9810..0x832B982C)
	// 832B9810: 396BFFD8  addi r11, r11, -0x28
	ctx.r[11].s64 = ctx.r[11].s64 + -40;
	// 832B9814: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832B9818: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832B981C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9820: 4080FFF0  bge 0x832b9810
	if !ctx.cr[0].lt {
	pc = 0x832B9810; continue 'dispatch;
	}
	// 832B9824: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9830 size=20
    let mut pc: u32 = 0x832B9830;
    'dispatch: loop {
        match pc {
            0x832B9830 => {
    //   block [0x832B9830..0x832B9844)
	// 832B9830: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B9834: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B9838: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B983C: 916AB38C  stw r11, -0x4c74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19572 as u32), ctx.r[11].u32 ) };
	// 832B9840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9848 size=52
    let mut pc: u32 = 0x832B9848;
    'dispatch: loop {
        match pc {
            0x832B9848 => {
    //   block [0x832B9848..0x832B9860)
	// 832B9848: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B984C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832B9850: 390BB738  addi r8, r11, -0x48c8
	ctx.r[8].s64 = ctx.r[11].s64 + -18632;
	// 832B9854: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 832B9858: 39680284  addi r11, r8, 0x284
	ctx.r[11].s64 = ctx.r[8].s64 + 644;
	// 832B985C: 39299128  addi r9, r9, -0x6ed8
	ctx.r[9].s64 = ctx.r[9].s64 + -28376;
	pc = 0x832B9860; continue 'dispatch;
            }
            0x832B9860 => {
    //   block [0x832B9860..0x832B987C)
	// 832B9860: 396BFFD8  addi r11, r11, -0x28
	ctx.r[11].s64 = ctx.r[11].s64 + -40;
	// 832B9864: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832B9868: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832B986C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9870: 4080FFF0  bge 0x832b9860
	if !ctx.cr[0].lt {
	pc = 0x832B9860; continue 'dispatch;
	}
	// 832B9874: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B9878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9880 size=48
    let mut pc: u32 = 0x832B9880;
    'dispatch: loop {
        match pc {
            0x832B9880 => {
    //   block [0x832B9880..0x832B9898)
	// 832B9880: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B9884: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832B9888: 390BB9D0  addi r8, r11, -0x4630
	ctx.r[8].s64 = ctx.r[11].s64 + -17968;
	// 832B988C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 832B9890: 396800A4  addi r11, r8, 0xa4
	ctx.r[11].s64 = ctx.r[8].s64 + 164;
	// 832B9894: 39299128  addi r9, r9, -0x6ed8
	ctx.r[9].s64 = ctx.r[9].s64 + -28376;
	pc = 0x832B9898; continue 'dispatch;
            }
            0x832B9898 => {
    //   block [0x832B9898..0x832B98B0)
	// 832B9898: 396BFFEC  addi r11, r11, -0x14
	ctx.r[11].s64 = ctx.r[11].s64 + -20;
	// 832B989C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832B98A0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B98A4: 4080FFF4  bge 0x832b9898
	if !ctx.cr[0].lt {
	pc = 0x832B9898; continue 'dispatch;
	}
	// 832B98A8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B98AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98B0 size=20
    let mut pc: u32 = 0x832B98B0;
    'dispatch: loop {
        match pc {
            0x832B98B0 => {
    //   block [0x832B98B0..0x832B98C4)
	// 832B98B0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B98B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832B98B8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B98BC: 916ABA88  stw r11, -0x4578(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17784 as u32), ctx.r[11].u32 ) };
	// 832B98C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98C8 size=20
    let mut pc: u32 = 0x832B98C8;
    'dispatch: loop {
        match pc {
            0x832B98C8 => {
    //   block [0x832B98C8..0x832B98DC)
	// 832B98C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832B98CC: 806BBBA4  lwz r3, -0x445c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17500 as u32) ) } as u64;
	// 832B98D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B98D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B98D8: 4B8E8940  b 0x82ba2218
	sub_82BA2218(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98DC size=4
    let mut pc: u32 = 0x832B98DC;
    'dispatch: loop {
        match pc {
            0x832B98DC => {
    //   block [0x832B98DC..0x832B98E0)
	// 832B98DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98E0 size=4
    let mut pc: u32 = 0x832B98E0;
    'dispatch: loop {
        match pc {
            0x832B98E0 => {
    //   block [0x832B98E0..0x832B98E4)
	// 832B98E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98E4 size=16
    let mut pc: u32 = 0x832B98E4;
    'dispatch: loop {
        match pc {
            0x832B98E4 => {
    //   block [0x832B98E4..0x832B98F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B98F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B98F4 size=16
    let mut pc: u32 = 0x832B98F4;
    'dispatch: loop {
        match pc {
            0x832B98F4 => {
    //   block [0x832B98F4..0x832B9904)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9904 size=16
    let mut pc: u32 = 0x832B9904;
    'dispatch: loop {
        match pc {
            0x832B9904 => {
    //   block [0x832B9904..0x832B9914)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9914 size=16
    let mut pc: u32 = 0x832B9914;
    'dispatch: loop {
        match pc {
            0x832B9914 => {
    //   block [0x832B9914..0x832B9924)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9924(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9924 size=16
    let mut pc: u32 = 0x832B9924;
    'dispatch: loop {
        match pc {
            0x832B9924 => {
    //   block [0x832B9924..0x832B9934)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9934 size=16
    let mut pc: u32 = 0x832B9934;
    'dispatch: loop {
        match pc {
            0x832B9934 => {
    //   block [0x832B9934..0x832B9944)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9944 size=16
    let mut pc: u32 = 0x832B9944;
    'dispatch: loop {
        match pc {
            0x832B9944 => {
    //   block [0x832B9944..0x832B9954)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9954 size=16
    let mut pc: u32 = 0x832B9954;
    'dispatch: loop {
        match pc {
            0x832B9954 => {
    //   block [0x832B9954..0x832B9964)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9964(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9964 size=16
    let mut pc: u32 = 0x832B9964;
    'dispatch: loop {
        match pc {
            0x832B9964 => {
    //   block [0x832B9964..0x832B9974)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9974(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9974 size=16
    let mut pc: u32 = 0x832B9974;
    'dispatch: loop {
        match pc {
            0x832B9974 => {
    //   block [0x832B9974..0x832B9984)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9984 size=16
    let mut pc: u32 = 0x832B9984;
    'dispatch: loop {
        match pc {
            0x832B9984 => {
    //   block [0x832B9984..0x832B9994)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9994 size=16
    let mut pc: u32 = 0x832B9994;
    'dispatch: loop {
        match pc {
            0x832B9994 => {
    //   block [0x832B9994..0x832B99A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99A4 size=16
    let mut pc: u32 = 0x832B99A4;
    'dispatch: loop {
        match pc {
            0x832B99A4 => {
    //   block [0x832B99A4..0x832B99B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99B4 size=16
    let mut pc: u32 = 0x832B99B4;
    'dispatch: loop {
        match pc {
            0x832B99B4 => {
    //   block [0x832B99B4..0x832B99C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99C4 size=16
    let mut pc: u32 = 0x832B99C4;
    'dispatch: loop {
        match pc {
            0x832B99C4 => {
    //   block [0x832B99C4..0x832B99D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99D4 size=16
    let mut pc: u32 = 0x832B99D4;
    'dispatch: loop {
        match pc {
            0x832B99D4 => {
    //   block [0x832B99D4..0x832B99E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99E4 size=16
    let mut pc: u32 = 0x832B99E4;
    'dispatch: loop {
        match pc {
            0x832B99E4 => {
    //   block [0x832B99E4..0x832B99F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B99F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B99F4 size=16
    let mut pc: u32 = 0x832B99F4;
    'dispatch: loop {
        match pc {
            0x832B99F4 => {
    //   block [0x832B99F4..0x832B9A04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A04 size=16
    let mut pc: u32 = 0x832B9A04;
    'dispatch: loop {
        match pc {
            0x832B9A04 => {
    //   block [0x832B9A04..0x832B9A14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A24 size=16
    let mut pc: u32 = 0x832B9A24;
    'dispatch: loop {
        match pc {
            0x832B9A24 => {
    //   block [0x832B9A24..0x832B9A34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A34 size=16
    let mut pc: u32 = 0x832B9A34;
    'dispatch: loop {
        match pc {
            0x832B9A34 => {
    //   block [0x832B9A34..0x832B9A44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A44 size=16
    let mut pc: u32 = 0x832B9A44;
    'dispatch: loop {
        match pc {
            0x832B9A44 => {
    //   block [0x832B9A44..0x832B9A54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A54 size=16
    let mut pc: u32 = 0x832B9A54;
    'dispatch: loop {
        match pc {
            0x832B9A54 => {
    //   block [0x832B9A54..0x832B9A64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A64 size=16
    let mut pc: u32 = 0x832B9A64;
    'dispatch: loop {
        match pc {
            0x832B9A64 => {
    //   block [0x832B9A64..0x832B9A74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A74 size=16
    let mut pc: u32 = 0x832B9A74;
    'dispatch: loop {
        match pc {
            0x832B9A74 => {
    //   block [0x832B9A74..0x832B9A84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A84 size=16
    let mut pc: u32 = 0x832B9A84;
    'dispatch: loop {
        match pc {
            0x832B9A84 => {
    //   block [0x832B9A84..0x832B9A94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9A94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9A94 size=16
    let mut pc: u32 = 0x832B9A94;
    'dispatch: loop {
        match pc {
            0x832B9A94 => {
    //   block [0x832B9A94..0x832B9AA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AA4 size=16
    let mut pc: u32 = 0x832B9AA4;
    'dispatch: loop {
        match pc {
            0x832B9AA4 => {
    //   block [0x832B9AA4..0x832B9AB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AB4 size=16
    let mut pc: u32 = 0x832B9AB4;
    'dispatch: loop {
        match pc {
            0x832B9AB4 => {
    //   block [0x832B9AB4..0x832B9AC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AC4 size=16
    let mut pc: u32 = 0x832B9AC4;
    'dispatch: loop {
        match pc {
            0x832B9AC4 => {
    //   block [0x832B9AC4..0x832B9AD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AD4 size=16
    let mut pc: u32 = 0x832B9AD4;
    'dispatch: loop {
        match pc {
            0x832B9AD4 => {
    //   block [0x832B9AD4..0x832B9AE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AE4 size=16
    let mut pc: u32 = 0x832B9AE4;
    'dispatch: loop {
        match pc {
            0x832B9AE4 => {
    //   block [0x832B9AE4..0x832B9AF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9AF4 size=16
    let mut pc: u32 = 0x832B9AF4;
    'dispatch: loop {
        match pc {
            0x832B9AF4 => {
    //   block [0x832B9AF4..0x832B9B04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B04 size=16
    let mut pc: u32 = 0x832B9B04;
    'dispatch: loop {
        match pc {
            0x832B9B04 => {
    //   block [0x832B9B04..0x832B9B14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B14 size=16
    let mut pc: u32 = 0x832B9B14;
    'dispatch: loop {
        match pc {
            0x832B9B14 => {
    //   block [0x832B9B14..0x832B9B24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B24 size=16
    let mut pc: u32 = 0x832B9B24;
    'dispatch: loop {
        match pc {
            0x832B9B24 => {
    //   block [0x832B9B24..0x832B9B34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B34 size=16
    let mut pc: u32 = 0x832B9B34;
    'dispatch: loop {
        match pc {
            0x832B9B34 => {
    //   block [0x832B9B34..0x832B9B44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B44 size=16
    let mut pc: u32 = 0x832B9B44;
    'dispatch: loop {
        match pc {
            0x832B9B44 => {
    //   block [0x832B9B44..0x832B9B54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B74 size=16
    let mut pc: u32 = 0x832B9B74;
    'dispatch: loop {
        match pc {
            0x832B9B74 => {
    //   block [0x832B9B74..0x832B9B84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9B94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9B94 size=16
    let mut pc: u32 = 0x832B9B94;
    'dispatch: loop {
        match pc {
            0x832B9B94 => {
    //   block [0x832B9B94..0x832B9BA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BA4 size=16
    let mut pc: u32 = 0x832B9BA4;
    'dispatch: loop {
        match pc {
            0x832B9BA4 => {
    //   block [0x832B9BA4..0x832B9BB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BB4 size=16
    let mut pc: u32 = 0x832B9BB4;
    'dispatch: loop {
        match pc {
            0x832B9BB4 => {
    //   block [0x832B9BB4..0x832B9BC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BC4 size=16
    let mut pc: u32 = 0x832B9BC4;
    'dispatch: loop {
        match pc {
            0x832B9BC4 => {
    //   block [0x832B9BC4..0x832B9BD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BD4 size=16
    let mut pc: u32 = 0x832B9BD4;
    'dispatch: loop {
        match pc {
            0x832B9BD4 => {
    //   block [0x832B9BD4..0x832B9BE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BE4 size=16
    let mut pc: u32 = 0x832B9BE4;
    'dispatch: loop {
        match pc {
            0x832B9BE4 => {
    //   block [0x832B9BE4..0x832B9BF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9BF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9BF4 size=16
    let mut pc: u32 = 0x832B9BF4;
    'dispatch: loop {
        match pc {
            0x832B9BF4 => {
    //   block [0x832B9BF4..0x832B9C04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C04 size=16
    let mut pc: u32 = 0x832B9C04;
    'dispatch: loop {
        match pc {
            0x832B9C04 => {
    //   block [0x832B9C04..0x832B9C14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C14 size=16
    let mut pc: u32 = 0x832B9C14;
    'dispatch: loop {
        match pc {
            0x832B9C14 => {
    //   block [0x832B9C14..0x832B9C24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C24 size=16
    let mut pc: u32 = 0x832B9C24;
    'dispatch: loop {
        match pc {
            0x832B9C24 => {
    //   block [0x832B9C24..0x832B9C34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C34 size=16
    let mut pc: u32 = 0x832B9C34;
    'dispatch: loop {
        match pc {
            0x832B9C34 => {
    //   block [0x832B9C34..0x832B9C44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C44 size=16
    let mut pc: u32 = 0x832B9C44;
    'dispatch: loop {
        match pc {
            0x832B9C44 => {
    //   block [0x832B9C44..0x832B9C54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C54 size=16
    let mut pc: u32 = 0x832B9C54;
    'dispatch: loop {
        match pc {
            0x832B9C54 => {
    //   block [0x832B9C54..0x832B9C64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C64 size=16
    let mut pc: u32 = 0x832B9C64;
    'dispatch: loop {
        match pc {
            0x832B9C64 => {
    //   block [0x832B9C64..0x832B9C74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C74 size=16
    let mut pc: u32 = 0x832B9C74;
    'dispatch: loop {
        match pc {
            0x832B9C74 => {
    //   block [0x832B9C74..0x832B9C84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C84 size=16
    let mut pc: u32 = 0x832B9C84;
    'dispatch: loop {
        match pc {
            0x832B9C84 => {
    //   block [0x832B9C84..0x832B9C94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9C94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9C94 size=16
    let mut pc: u32 = 0x832B9C94;
    'dispatch: loop {
        match pc {
            0x832B9C94 => {
    //   block [0x832B9C94..0x832B9CA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CB4 size=16
    let mut pc: u32 = 0x832B9CB4;
    'dispatch: loop {
        match pc {
            0x832B9CB4 => {
    //   block [0x832B9CB4..0x832B9CC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CC4 size=16
    let mut pc: u32 = 0x832B9CC4;
    'dispatch: loop {
        match pc {
            0x832B9CC4 => {
    //   block [0x832B9CC4..0x832B9CD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CD4 size=16
    let mut pc: u32 = 0x832B9CD4;
    'dispatch: loop {
        match pc {
            0x832B9CD4 => {
    //   block [0x832B9CD4..0x832B9CE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CE4 size=16
    let mut pc: u32 = 0x832B9CE4;
    'dispatch: loop {
        match pc {
            0x832B9CE4 => {
    //   block [0x832B9CE4..0x832B9CF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9CF4 size=16
    let mut pc: u32 = 0x832B9CF4;
    'dispatch: loop {
        match pc {
            0x832B9CF4 => {
    //   block [0x832B9CF4..0x832B9D04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D04 size=16
    let mut pc: u32 = 0x832B9D04;
    'dispatch: loop {
        match pc {
            0x832B9D04 => {
    //   block [0x832B9D04..0x832B9D14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D14 size=16
    let mut pc: u32 = 0x832B9D14;
    'dispatch: loop {
        match pc {
            0x832B9D14 => {
    //   block [0x832B9D14..0x832B9D24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D24 size=16
    let mut pc: u32 = 0x832B9D24;
    'dispatch: loop {
        match pc {
            0x832B9D24 => {
    //   block [0x832B9D24..0x832B9D34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D34 size=16
    let mut pc: u32 = 0x832B9D34;
    'dispatch: loop {
        match pc {
            0x832B9D34 => {
    //   block [0x832B9D34..0x832B9D44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D64 size=16
    let mut pc: u32 = 0x832B9D64;
    'dispatch: loop {
        match pc {
            0x832B9D64 => {
    //   block [0x832B9D64..0x832B9D74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D74 size=16
    let mut pc: u32 = 0x832B9D74;
    'dispatch: loop {
        match pc {
            0x832B9D74 => {
    //   block [0x832B9D74..0x832B9D84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D84 size=16
    let mut pc: u32 = 0x832B9D84;
    'dispatch: loop {
        match pc {
            0x832B9D84 => {
    //   block [0x832B9D84..0x832B9D94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9D94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9D94 size=16
    let mut pc: u32 = 0x832B9D94;
    'dispatch: loop {
        match pc {
            0x832B9D94 => {
    //   block [0x832B9D94..0x832B9DA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DA4 size=16
    let mut pc: u32 = 0x832B9DA4;
    'dispatch: loop {
        match pc {
            0x832B9DA4 => {
    //   block [0x832B9DA4..0x832B9DB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DB4 size=16
    let mut pc: u32 = 0x832B9DB4;
    'dispatch: loop {
        match pc {
            0x832B9DB4 => {
    //   block [0x832B9DB4..0x832B9DC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DC4 size=16
    let mut pc: u32 = 0x832B9DC4;
    'dispatch: loop {
        match pc {
            0x832B9DC4 => {
    //   block [0x832B9DC4..0x832B9DD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DD4 size=16
    let mut pc: u32 = 0x832B9DD4;
    'dispatch: loop {
        match pc {
            0x832B9DD4 => {
    //   block [0x832B9DD4..0x832B9DE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DE4 size=16
    let mut pc: u32 = 0x832B9DE4;
    'dispatch: loop {
        match pc {
            0x832B9DE4 => {
    //   block [0x832B9DE4..0x832B9DF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9DF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9DF4 size=16
    let mut pc: u32 = 0x832B9DF4;
    'dispatch: loop {
        match pc {
            0x832B9DF4 => {
    //   block [0x832B9DF4..0x832B9E04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E04 size=16
    let mut pc: u32 = 0x832B9E04;
    'dispatch: loop {
        match pc {
            0x832B9E04 => {
    //   block [0x832B9E04..0x832B9E14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E14 size=16
    let mut pc: u32 = 0x832B9E14;
    'dispatch: loop {
        match pc {
            0x832B9E14 => {
    //   block [0x832B9E14..0x832B9E24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E24 size=16
    let mut pc: u32 = 0x832B9E24;
    'dispatch: loop {
        match pc {
            0x832B9E24 => {
    //   block [0x832B9E24..0x832B9E34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E34 size=16
    let mut pc: u32 = 0x832B9E34;
    'dispatch: loop {
        match pc {
            0x832B9E34 => {
    //   block [0x832B9E34..0x832B9E44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E44 size=16
    let mut pc: u32 = 0x832B9E44;
    'dispatch: loop {
        match pc {
            0x832B9E44 => {
    //   block [0x832B9E44..0x832B9E54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E54 size=16
    let mut pc: u32 = 0x832B9E54;
    'dispatch: loop {
        match pc {
            0x832B9E54 => {
    //   block [0x832B9E54..0x832B9E64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E64 size=16
    let mut pc: u32 = 0x832B9E64;
    'dispatch: loop {
        match pc {
            0x832B9E64 => {
    //   block [0x832B9E64..0x832B9E74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E74 size=16
    let mut pc: u32 = 0x832B9E74;
    'dispatch: loop {
        match pc {
            0x832B9E74 => {
    //   block [0x832B9E74..0x832B9E84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E84 size=16
    let mut pc: u32 = 0x832B9E84;
    'dispatch: loop {
        match pc {
            0x832B9E84 => {
    //   block [0x832B9E84..0x832B9E94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9E94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9E94 size=16
    let mut pc: u32 = 0x832B9E94;
    'dispatch: loop {
        match pc {
            0x832B9E94 => {
    //   block [0x832B9E94..0x832B9EA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9EA4 size=16
    let mut pc: u32 = 0x832B9EA4;
    'dispatch: loop {
        match pc {
            0x832B9EA4 => {
    //   block [0x832B9EA4..0x832B9EB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9EB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9EB4 size=16
    let mut pc: u32 = 0x832B9EB4;
    'dispatch: loop {
        match pc {
            0x832B9EB4 => {
    //   block [0x832B9EB4..0x832B9EC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9EC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9EC4 size=16
    let mut pc: u32 = 0x832B9EC4;
    'dispatch: loop {
        match pc {
            0x832B9EC4 => {
    //   block [0x832B9EC4..0x832B9ED4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9ED4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9ED4 size=16
    let mut pc: u32 = 0x832B9ED4;
    'dispatch: loop {
        match pc {
            0x832B9ED4 => {
    //   block [0x832B9ED4..0x832B9EE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9EE4 size=16
    let mut pc: u32 = 0x832B9EE4;
    'dispatch: loop {
        match pc {
            0x832B9EE4 => {
    //   block [0x832B9EE4..0x832B9EF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9EF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9EF4 size=16
    let mut pc: u32 = 0x832B9EF4;
    'dispatch: loop {
        match pc {
            0x832B9EF4 => {
    //   block [0x832B9EF4..0x832B9F04)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F04 size=16
    let mut pc: u32 = 0x832B9F04;
    'dispatch: loop {
        match pc {
            0x832B9F04 => {
    //   block [0x832B9F04..0x832B9F14)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F14 size=16
    let mut pc: u32 = 0x832B9F14;
    'dispatch: loop {
        match pc {
            0x832B9F14 => {
    //   block [0x832B9F14..0x832B9F24)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F24 size=16
    let mut pc: u32 = 0x832B9F24;
    'dispatch: loop {
        match pc {
            0x832B9F24 => {
    //   block [0x832B9F24..0x832B9F34)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F34 size=16
    let mut pc: u32 = 0x832B9F34;
    'dispatch: loop {
        match pc {
            0x832B9F34 => {
    //   block [0x832B9F34..0x832B9F44)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F44 size=16
    let mut pc: u32 = 0x832B9F44;
    'dispatch: loop {
        match pc {
            0x832B9F44 => {
    //   block [0x832B9F44..0x832B9F54)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F54 size=16
    let mut pc: u32 = 0x832B9F54;
    'dispatch: loop {
        match pc {
            0x832B9F54 => {
    //   block [0x832B9F54..0x832B9F64)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F64 size=16
    let mut pc: u32 = 0x832B9F64;
    'dispatch: loop {
        match pc {
            0x832B9F64 => {
    //   block [0x832B9F64..0x832B9F74)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F74 size=16
    let mut pc: u32 = 0x832B9F74;
    'dispatch: loop {
        match pc {
            0x832B9F74 => {
    //   block [0x832B9F74..0x832B9F84)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F84 size=16
    let mut pc: u32 = 0x832B9F84;
    'dispatch: loop {
        match pc {
            0x832B9F84 => {
    //   block [0x832B9F84..0x832B9F94)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9F94 size=16
    let mut pc: u32 = 0x832B9F94;
    'dispatch: loop {
        match pc {
            0x832B9F94 => {
    //   block [0x832B9F94..0x832B9FA4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9FA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9FA4 size=16
    let mut pc: u32 = 0x832B9FA4;
    'dispatch: loop {
        match pc {
            0x832B9FA4 => {
    //   block [0x832B9FA4..0x832B9FB4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9FB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9FB4 size=16
    let mut pc: u32 = 0x832B9FB4;
    'dispatch: loop {
        match pc {
            0x832B9FB4 => {
    //   block [0x832B9FB4..0x832B9FC4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9FC4 size=16
    let mut pc: u32 = 0x832B9FC4;
    'dispatch: loop {
        match pc {
            0x832B9FC4 => {
    //   block [0x832B9FC4..0x832B9FD4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9FD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9FD4 size=16
    let mut pc: u32 = 0x832B9FD4;
    'dispatch: loop {
        match pc {
            0x832B9FD4 => {
    //   block [0x832B9FD4..0x832B9FE4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9FE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9FE4 size=16
    let mut pc: u32 = 0x832B9FE4;
    'dispatch: loop {
        match pc {
            0x832B9FE4 => {
    //   block [0x832B9FE4..0x832B9FF4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B9FF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B9FF4 size=16
    let mut pc: u32 = 0x832B9FF4;
    'dispatch: loop {
        match pc {
            0x832B9FF4 => {
    //   block [0x832B9FF4..0x832BA004)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA004 size=16
    let mut pc: u32 = 0x832BA004;
    'dispatch: loop {
        match pc {
            0x832BA004 => {
    //   block [0x832BA004..0x832BA014)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA014 size=16
    let mut pc: u32 = 0x832BA014;
    'dispatch: loop {
        match pc {
            0x832BA014 => {
    //   block [0x832BA014..0x832BA024)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA024 size=16
    let mut pc: u32 = 0x832BA024;
    'dispatch: loop {
        match pc {
            0x832BA024 => {
    //   block [0x832BA024..0x832BA034)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA034 size=16
    let mut pc: u32 = 0x832BA034;
    'dispatch: loop {
        match pc {
            0x832BA034 => {
    //   block [0x832BA034..0x832BA044)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA044(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA044 size=16
    let mut pc: u32 = 0x832BA044;
    'dispatch: loop {
        match pc {
            0x832BA044 => {
    //   block [0x832BA044..0x832BA054)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA054(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA054 size=16
    let mut pc: u32 = 0x832BA054;
    'dispatch: loop {
        match pc {
            0x832BA054 => {
    //   block [0x832BA054..0x832BA064)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA064(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA064 size=16
    let mut pc: u32 = 0x832BA064;
    'dispatch: loop {
        match pc {
            0x832BA064 => {
    //   block [0x832BA064..0x832BA074)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA074(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA074 size=16
    let mut pc: u32 = 0x832BA074;
    'dispatch: loop {
        match pc {
            0x832BA074 => {
    //   block [0x832BA074..0x832BA084)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA084(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA084 size=16
    let mut pc: u32 = 0x832BA084;
    'dispatch: loop {
        match pc {
            0x832BA084 => {
    //   block [0x832BA084..0x832BA094)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA094(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA094 size=16
    let mut pc: u32 = 0x832BA094;
    'dispatch: loop {
        match pc {
            0x832BA094 => {
    //   block [0x832BA094..0x832BA0A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA0A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA0A4 size=16
    let mut pc: u32 = 0x832BA0A4;
    'dispatch: loop {
        match pc {
            0x832BA0A4 => {
    //   block [0x832BA0A4..0x832BA0B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA0B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA0B4 size=16
    let mut pc: u32 = 0x832BA0B4;
    'dispatch: loop {
        match pc {
            0x832BA0B4 => {
    //   block [0x832BA0B4..0x832BA0C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA0C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA0C4 size=16
    let mut pc: u32 = 0x832BA0C4;
    'dispatch: loop {
        match pc {
            0x832BA0C4 => {
    //   block [0x832BA0C4..0x832BA0D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA0D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA0D4 size=16
    let mut pc: u32 = 0x832BA0D4;
    'dispatch: loop {
        match pc {
            0x832BA0D4 => {
    //   block [0x832BA0D4..0x832BA0E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA0E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA0E4 size=16
    let mut pc: u32 = 0x832BA0E4;
    'dispatch: loop {
        match pc {
            0x832BA0E4 => {
    //   block [0x832BA0E4..0x832BA0F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA0F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA0F4 size=16
    let mut pc: u32 = 0x832BA0F4;
    'dispatch: loop {
        match pc {
            0x832BA0F4 => {
    //   block [0x832BA0F4..0x832BA104)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA104 size=16
    let mut pc: u32 = 0x832BA104;
    'dispatch: loop {
        match pc {
            0x832BA104 => {
    //   block [0x832BA104..0x832BA114)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA114 size=16
    let mut pc: u32 = 0x832BA114;
    'dispatch: loop {
        match pc {
            0x832BA114 => {
    //   block [0x832BA114..0x832BA124)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA124 size=16
    let mut pc: u32 = 0x832BA124;
    'dispatch: loop {
        match pc {
            0x832BA124 => {
    //   block [0x832BA124..0x832BA134)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA134 size=16
    let mut pc: u32 = 0x832BA134;
    'dispatch: loop {
        match pc {
            0x832BA134 => {
    //   block [0x832BA134..0x832BA144)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA144(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA144 size=16
    let mut pc: u32 = 0x832BA144;
    'dispatch: loop {
        match pc {
            0x832BA144 => {
    //   block [0x832BA144..0x832BA154)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA154(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA154 size=16
    let mut pc: u32 = 0x832BA154;
    'dispatch: loop {
        match pc {
            0x832BA154 => {
    //   block [0x832BA154..0x832BA164)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA164 size=16
    let mut pc: u32 = 0x832BA164;
    'dispatch: loop {
        match pc {
            0x832BA164 => {
    //   block [0x832BA164..0x832BA174)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA174 size=16
    let mut pc: u32 = 0x832BA174;
    'dispatch: loop {
        match pc {
            0x832BA174 => {
    //   block [0x832BA174..0x832BA184)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA184 size=16
    let mut pc: u32 = 0x832BA184;
    'dispatch: loop {
        match pc {
            0x832BA184 => {
    //   block [0x832BA184..0x832BA194)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA194 size=16
    let mut pc: u32 = 0x832BA194;
    'dispatch: loop {
        match pc {
            0x832BA194 => {
    //   block [0x832BA194..0x832BA1A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA1A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA1A4 size=16
    let mut pc: u32 = 0x832BA1A4;
    'dispatch: loop {
        match pc {
            0x832BA1A4 => {
    //   block [0x832BA1A4..0x832BA1B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA1B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA1B4 size=16
    let mut pc: u32 = 0x832BA1B4;
    'dispatch: loop {
        match pc {
            0x832BA1B4 => {
    //   block [0x832BA1B4..0x832BA1C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA1C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA1C4 size=16
    let mut pc: u32 = 0x832BA1C4;
    'dispatch: loop {
        match pc {
            0x832BA1C4 => {
    //   block [0x832BA1C4..0x832BA1D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA1D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA1D4 size=16
    let mut pc: u32 = 0x832BA1D4;
    'dispatch: loop {
        match pc {
            0x832BA1D4 => {
    //   block [0x832BA1D4..0x832BA1E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA1E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA1E4 size=16
    let mut pc: u32 = 0x832BA1E4;
    'dispatch: loop {
        match pc {
            0x832BA1E4 => {
    //   block [0x832BA1E4..0x832BA1F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA1F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA1F4 size=16
    let mut pc: u32 = 0x832BA1F4;
    'dispatch: loop {
        match pc {
            0x832BA1F4 => {
    //   block [0x832BA1F4..0x832BA204)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA204 size=16
    let mut pc: u32 = 0x832BA204;
    'dispatch: loop {
        match pc {
            0x832BA204 => {
    //   block [0x832BA204..0x832BA214)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA214 size=16
    let mut pc: u32 = 0x832BA214;
    'dispatch: loop {
        match pc {
            0x832BA214 => {
    //   block [0x832BA214..0x832BA224)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA224(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA224 size=16
    let mut pc: u32 = 0x832BA224;
    'dispatch: loop {
        match pc {
            0x832BA224 => {
    //   block [0x832BA224..0x832BA234)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA234(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA234 size=16
    let mut pc: u32 = 0x832BA234;
    'dispatch: loop {
        match pc {
            0x832BA234 => {
    //   block [0x832BA234..0x832BA244)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA244(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA244 size=16
    let mut pc: u32 = 0x832BA244;
    'dispatch: loop {
        match pc {
            0x832BA244 => {
    //   block [0x832BA244..0x832BA254)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA254 size=16
    let mut pc: u32 = 0x832BA254;
    'dispatch: loop {
        match pc {
            0x832BA254 => {
    //   block [0x832BA254..0x832BA264)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA264 size=16
    let mut pc: u32 = 0x832BA264;
    'dispatch: loop {
        match pc {
            0x832BA264 => {
    //   block [0x832BA264..0x832BA274)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA274 size=16
    let mut pc: u32 = 0x832BA274;
    'dispatch: loop {
        match pc {
            0x832BA274 => {
    //   block [0x832BA274..0x832BA284)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA284 size=16
    let mut pc: u32 = 0x832BA284;
    'dispatch: loop {
        match pc {
            0x832BA284 => {
    //   block [0x832BA284..0x832BA294)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA294(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA294 size=16
    let mut pc: u32 = 0x832BA294;
    'dispatch: loop {
        match pc {
            0x832BA294 => {
    //   block [0x832BA294..0x832BA2A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA2A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA2A4 size=16
    let mut pc: u32 = 0x832BA2A4;
    'dispatch: loop {
        match pc {
            0x832BA2A4 => {
    //   block [0x832BA2A4..0x832BA2B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA2E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA2E4 size=16
    let mut pc: u32 = 0x832BA2E4;
    'dispatch: loop {
        match pc {
            0x832BA2E4 => {
    //   block [0x832BA2E4..0x832BA2F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA2F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA2F4 size=16
    let mut pc: u32 = 0x832BA2F4;
    'dispatch: loop {
        match pc {
            0x832BA2F4 => {
    //   block [0x832BA2F4..0x832BA304)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA304(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA304 size=16
    let mut pc: u32 = 0x832BA304;
    'dispatch: loop {
        match pc {
            0x832BA304 => {
    //   block [0x832BA304..0x832BA314)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA314 size=16
    let mut pc: u32 = 0x832BA314;
    'dispatch: loop {
        match pc {
            0x832BA314 => {
    //   block [0x832BA314..0x832BA324)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA324(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA324 size=16
    let mut pc: u32 = 0x832BA324;
    'dispatch: loop {
        match pc {
            0x832BA324 => {
    //   block [0x832BA324..0x832BA334)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA334 size=16
    let mut pc: u32 = 0x832BA334;
    'dispatch: loop {
        match pc {
            0x832BA334 => {
    //   block [0x832BA334..0x832BA344)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA344 size=16
    let mut pc: u32 = 0x832BA344;
    'dispatch: loop {
        match pc {
            0x832BA344 => {
    //   block [0x832BA344..0x832BA354)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA354 size=16
    let mut pc: u32 = 0x832BA354;
    'dispatch: loop {
        match pc {
            0x832BA354 => {
    //   block [0x832BA354..0x832BA364)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA364 size=16
    let mut pc: u32 = 0x832BA364;
    'dispatch: loop {
        match pc {
            0x832BA364 => {
    //   block [0x832BA364..0x832BA374)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA374(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA374 size=16
    let mut pc: u32 = 0x832BA374;
    'dispatch: loop {
        match pc {
            0x832BA374 => {
    //   block [0x832BA374..0x832BA384)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA384 size=16
    let mut pc: u32 = 0x832BA384;
    'dispatch: loop {
        match pc {
            0x832BA384 => {
    //   block [0x832BA384..0x832BA394)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA394 size=16
    let mut pc: u32 = 0x832BA394;
    'dispatch: loop {
        match pc {
            0x832BA394 => {
    //   block [0x832BA394..0x832BA3A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA3A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA3A4 size=16
    let mut pc: u32 = 0x832BA3A4;
    'dispatch: loop {
        match pc {
            0x832BA3A4 => {
    //   block [0x832BA3A4..0x832BA3B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA3B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA3B4 size=16
    let mut pc: u32 = 0x832BA3B4;
    'dispatch: loop {
        match pc {
            0x832BA3B4 => {
    //   block [0x832BA3B4..0x832BA3C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA3C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA3C4 size=16
    let mut pc: u32 = 0x832BA3C4;
    'dispatch: loop {
        match pc {
            0x832BA3C4 => {
    //   block [0x832BA3C4..0x832BA3D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA3D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA3D4 size=16
    let mut pc: u32 = 0x832BA3D4;
    'dispatch: loop {
        match pc {
            0x832BA3D4 => {
    //   block [0x832BA3D4..0x832BA3E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA3E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA3E4 size=16
    let mut pc: u32 = 0x832BA3E4;
    'dispatch: loop {
        match pc {
            0x832BA3E4 => {
    //   block [0x832BA3E4..0x832BA3F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA3F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA3F4 size=16
    let mut pc: u32 = 0x832BA3F4;
    'dispatch: loop {
        match pc {
            0x832BA3F4 => {
    //   block [0x832BA3F4..0x832BA404)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA404 size=16
    let mut pc: u32 = 0x832BA404;
    'dispatch: loop {
        match pc {
            0x832BA404 => {
    //   block [0x832BA404..0x832BA414)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA414 size=16
    let mut pc: u32 = 0x832BA414;
    'dispatch: loop {
        match pc {
            0x832BA414 => {
    //   block [0x832BA414..0x832BA424)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA424(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA424 size=16
    let mut pc: u32 = 0x832BA424;
    'dispatch: loop {
        match pc {
            0x832BA424 => {
    //   block [0x832BA424..0x832BA434)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA434 size=16
    let mut pc: u32 = 0x832BA434;
    'dispatch: loop {
        match pc {
            0x832BA434 => {
    //   block [0x832BA434..0x832BA444)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA444(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA444 size=16
    let mut pc: u32 = 0x832BA444;
    'dispatch: loop {
        match pc {
            0x832BA444 => {
    //   block [0x832BA444..0x832BA454)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA454 size=16
    let mut pc: u32 = 0x832BA454;
    'dispatch: loop {
        match pc {
            0x832BA454 => {
    //   block [0x832BA454..0x832BA464)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA464(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA464 size=16
    let mut pc: u32 = 0x832BA464;
    'dispatch: loop {
        match pc {
            0x832BA464 => {
    //   block [0x832BA464..0x832BA474)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA474 size=16
    let mut pc: u32 = 0x832BA474;
    'dispatch: loop {
        match pc {
            0x832BA474 => {
    //   block [0x832BA474..0x832BA484)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA484(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA484 size=16
    let mut pc: u32 = 0x832BA484;
    'dispatch: loop {
        match pc {
            0x832BA484 => {
    //   block [0x832BA484..0x832BA494)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA494 size=16
    let mut pc: u32 = 0x832BA494;
    'dispatch: loop {
        match pc {
            0x832BA494 => {
    //   block [0x832BA494..0x832BA4A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA4A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA4A4 size=16
    let mut pc: u32 = 0x832BA4A4;
    'dispatch: loop {
        match pc {
            0x832BA4A4 => {
    //   block [0x832BA4A4..0x832BA4B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA4B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA4B4 size=16
    let mut pc: u32 = 0x832BA4B4;
    'dispatch: loop {
        match pc {
            0x832BA4B4 => {
    //   block [0x832BA4B4..0x832BA4C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA4C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA4C4 size=16
    let mut pc: u32 = 0x832BA4C4;
    'dispatch: loop {
        match pc {
            0x832BA4C4 => {
    //   block [0x832BA4C4..0x832BA4D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA4D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA4D4 size=16
    let mut pc: u32 = 0x832BA4D4;
    'dispatch: loop {
        match pc {
            0x832BA4D4 => {
    //   block [0x832BA4D4..0x832BA4E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA4E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA4E4 size=16
    let mut pc: u32 = 0x832BA4E4;
    'dispatch: loop {
        match pc {
            0x832BA4E4 => {
    //   block [0x832BA4E4..0x832BA4F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA4F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA4F4 size=16
    let mut pc: u32 = 0x832BA4F4;
    'dispatch: loop {
        match pc {
            0x832BA4F4 => {
    //   block [0x832BA4F4..0x832BA504)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA504 size=16
    let mut pc: u32 = 0x832BA504;
    'dispatch: loop {
        match pc {
            0x832BA504 => {
    //   block [0x832BA504..0x832BA514)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA514(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA514 size=16
    let mut pc: u32 = 0x832BA514;
    'dispatch: loop {
        match pc {
            0x832BA514 => {
    //   block [0x832BA514..0x832BA524)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA544 size=16
    let mut pc: u32 = 0x832BA544;
    'dispatch: loop {
        match pc {
            0x832BA544 => {
    //   block [0x832BA544..0x832BA554)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA584 size=16
    let mut pc: u32 = 0x832BA584;
    'dispatch: loop {
        match pc {
            0x832BA584 => {
    //   block [0x832BA584..0x832BA594)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA594(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA594 size=16
    let mut pc: u32 = 0x832BA594;
    'dispatch: loop {
        match pc {
            0x832BA594 => {
    //   block [0x832BA594..0x832BA5A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA5A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA5A4 size=16
    let mut pc: u32 = 0x832BA5A4;
    'dispatch: loop {
        match pc {
            0x832BA5A4 => {
    //   block [0x832BA5A4..0x832BA5B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA5B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA5B4 size=16
    let mut pc: u32 = 0x832BA5B4;
    'dispatch: loop {
        match pc {
            0x832BA5B4 => {
    //   block [0x832BA5B4..0x832BA5C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA5C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA5C4 size=16
    let mut pc: u32 = 0x832BA5C4;
    'dispatch: loop {
        match pc {
            0x832BA5C4 => {
    //   block [0x832BA5C4..0x832BA5D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA5D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA5D4 size=16
    let mut pc: u32 = 0x832BA5D4;
    'dispatch: loop {
        match pc {
            0x832BA5D4 => {
    //   block [0x832BA5D4..0x832BA5E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA5E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA5E4 size=16
    let mut pc: u32 = 0x832BA5E4;
    'dispatch: loop {
        match pc {
            0x832BA5E4 => {
    //   block [0x832BA5E4..0x832BA5F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA5F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA5F4 size=16
    let mut pc: u32 = 0x832BA5F4;
    'dispatch: loop {
        match pc {
            0x832BA5F4 => {
    //   block [0x832BA5F4..0x832BA604)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA604(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA604 size=16
    let mut pc: u32 = 0x832BA604;
    'dispatch: loop {
        match pc {
            0x832BA604 => {
    //   block [0x832BA604..0x832BA614)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA614 size=16
    let mut pc: u32 = 0x832BA614;
    'dispatch: loop {
        match pc {
            0x832BA614 => {
    //   block [0x832BA614..0x832BA624)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA624(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA624 size=16
    let mut pc: u32 = 0x832BA624;
    'dispatch: loop {
        match pc {
            0x832BA624 => {
    //   block [0x832BA624..0x832BA634)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA634 size=16
    let mut pc: u32 = 0x832BA634;
    'dispatch: loop {
        match pc {
            0x832BA634 => {
    //   block [0x832BA634..0x832BA644)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA644 size=16
    let mut pc: u32 = 0x832BA644;
    'dispatch: loop {
        match pc {
            0x832BA644 => {
    //   block [0x832BA644..0x832BA654)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA654 size=16
    let mut pc: u32 = 0x832BA654;
    'dispatch: loop {
        match pc {
            0x832BA654 => {
    //   block [0x832BA654..0x832BA664)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA664 size=16
    let mut pc: u32 = 0x832BA664;
    'dispatch: loop {
        match pc {
            0x832BA664 => {
    //   block [0x832BA664..0x832BA674)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA674 size=16
    let mut pc: u32 = 0x832BA674;
    'dispatch: loop {
        match pc {
            0x832BA674 => {
    //   block [0x832BA674..0x832BA684)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA684(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA684 size=16
    let mut pc: u32 = 0x832BA684;
    'dispatch: loop {
        match pc {
            0x832BA684 => {
    //   block [0x832BA684..0x832BA694)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA694 size=16
    let mut pc: u32 = 0x832BA694;
    'dispatch: loop {
        match pc {
            0x832BA694 => {
    //   block [0x832BA694..0x832BA6A4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA6A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA6A4 size=16
    let mut pc: u32 = 0x832BA6A4;
    'dispatch: loop {
        match pc {
            0x832BA6A4 => {
    //   block [0x832BA6A4..0x832BA6B4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA6B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA6B4 size=16
    let mut pc: u32 = 0x832BA6B4;
    'dispatch: loop {
        match pc {
            0x832BA6B4 => {
    //   block [0x832BA6B4..0x832BA6C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA6C4 size=16
    let mut pc: u32 = 0x832BA6C4;
    'dispatch: loop {
        match pc {
            0x832BA6C4 => {
    //   block [0x832BA6C4..0x832BA6D4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA6D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA6D4 size=16
    let mut pc: u32 = 0x832BA6D4;
    'dispatch: loop {
        match pc {
            0x832BA6D4 => {
    //   block [0x832BA6D4..0x832BA6E4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA6E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA6E4 size=16
    let mut pc: u32 = 0x832BA6E4;
    'dispatch: loop {
        match pc {
            0x832BA6E4 => {
    //   block [0x832BA6E4..0x832BA6F4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA6F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA6F4 size=16
    let mut pc: u32 = 0x832BA6F4;
    'dispatch: loop {
        match pc {
            0x832BA6F4 => {
    //   block [0x832BA6F4..0x832BA704)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA704 size=16
    let mut pc: u32 = 0x832BA704;
    'dispatch: loop {
        match pc {
            0x832BA704 => {
    //   block [0x832BA704..0x832BA714)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832BA714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832BA714 size=16
    let mut pc: u32 = 0x832BA714;
    'dispatch: loop {
        match pc {
            0x832BA714 => {
    //   block [0x832BA714..0x832BA724)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


