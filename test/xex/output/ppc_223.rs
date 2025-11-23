pub fn sub_832813D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832813D0 size=56
    let mut pc: u32 = 0x832813D0;
    'dispatch: loop {
        match pc {
            0x832813D0 => {
    //   block [0x832813D0..0x83281408)
	// 832813D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832813D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832813D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832813DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832813E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832813E4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832813E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832813EC: 4AF7296D  bl 0x821f3d58
	ctx.lr = 0x832813F0;
	sub_821F3D58(ctx, base);
	// 832813F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832813F4: 906AE030  stw r3, -0x1fd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8144 as u32), ctx.r[3].u32 ) };
	// 832813F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832813FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281408 size=56
    let mut pc: u32 = 0x83281408;
    'dispatch: loop {
        match pc {
            0x83281408 => {
    //   block [0x83281408..0x83281440)
	// 83281408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328140C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328141C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83281420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281424: 4AF72935  bl 0x821f3d58
	ctx.lr = 0x83281428;
	sub_821F3D58(ctx, base);
	// 83281428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328142C: 906AE034  stw r3, -0x1fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8140 as u32), ctx.r[3].u32 ) };
	// 83281430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328143C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281440 size=56
    let mut pc: u32 = 0x83281440;
    'dispatch: loop {
        match pc {
            0x83281440 => {
    //   block [0x83281440..0x83281478)
	// 83281440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328144C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281454: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83281458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328145C: 4AF728FD  bl 0x821f3d58
	ctx.lr = 0x83281460;
	sub_821F3D58(ctx, base);
	// 83281460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281464: 906AE038  stw r3, -0x1fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8136 as u32), ctx.r[3].u32 ) };
	// 83281468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328146C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281478 size=56
    let mut pc: u32 = 0x83281478;
    'dispatch: loop {
        match pc {
            0x83281478 => {
    //   block [0x83281478..0x832814B0)
	// 83281478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281484: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328148C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83281490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281494: 4AF728C5  bl 0x821f3d58
	ctx.lr = 0x83281498;
	sub_821F3D58(ctx, base);
	// 83281498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328149C: 906AE03C  stw r3, -0x1fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8132 as u32), ctx.r[3].u32 ) };
	// 832814A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832814A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832814A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832814AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832814B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832814B0 size=56
    let mut pc: u32 = 0x832814B0;
    'dispatch: loop {
        match pc {
            0x832814B0 => {
    //   block [0x832814B0..0x832814E8)
	// 832814B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832814B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832814B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832814BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832814C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832814C4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832814C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832814CC: 4AF7288D  bl 0x821f3d58
	ctx.lr = 0x832814D0;
	sub_821F3D58(ctx, base);
	// 832814D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832814D4: 906AE040  stw r3, -0x1fc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8128 as u32), ctx.r[3].u32 ) };
	// 832814D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832814DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832814E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832814E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832814E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832814E8 size=56
    let mut pc: u32 = 0x832814E8;
    'dispatch: loop {
        match pc {
            0x832814E8 => {
    //   block [0x832814E8..0x83281520)
	// 832814E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832814EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832814F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832814F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832814F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832814FC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83281500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281504: 4AF72855  bl 0x821f3d58
	ctx.lr = 0x83281508;
	sub_821F3D58(ctx, base);
	// 83281508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328150C: 906AE044  stw r3, -0x1fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8124 as u32), ctx.r[3].u32 ) };
	// 83281510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281520 size=56
    let mut pc: u32 = 0x83281520;
    'dispatch: loop {
        match pc {
            0x83281520 => {
    //   block [0x83281520..0x83281558)
	// 83281520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328152C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281534: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83281538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328153C: 4AF7281D  bl 0x821f3d58
	ctx.lr = 0x83281540;
	sub_821F3D58(ctx, base);
	// 83281540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281544: 906AE048  stw r3, -0x1fb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8120 as u32), ctx.r[3].u32 ) };
	// 83281548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328154C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281558 size=56
    let mut pc: u32 = 0x83281558;
    'dispatch: loop {
        match pc {
            0x83281558 => {
    //   block [0x83281558..0x83281590)
	// 83281558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328156C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83281570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281574: 4AF727E5  bl 0x821f3d58
	ctx.lr = 0x83281578;
	sub_821F3D58(ctx, base);
	// 83281578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328157C: 906AE04C  stw r3, -0x1fb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8116 as u32), ctx.r[3].u32 ) };
	// 83281580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281590 size=56
    let mut pc: u32 = 0x83281590;
    'dispatch: loop {
        match pc {
            0x83281590 => {
    //   block [0x83281590..0x832815C8)
	// 83281590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328159C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832815A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832815A4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832815A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832815AC: 4AF727AD  bl 0x821f3d58
	ctx.lr = 0x832815B0;
	sub_821F3D58(ctx, base);
	// 832815B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832815B4: 906AE050  stw r3, -0x1fb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8112 as u32), ctx.r[3].u32 ) };
	// 832815B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832815BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832815C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832815C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832815C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832815C8 size=56
    let mut pc: u32 = 0x832815C8;
    'dispatch: loop {
        match pc {
            0x832815C8 => {
    //   block [0x832815C8..0x83281600)
	// 832815C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832815CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832815D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832815D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832815D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832815DC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832815E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832815E4: 4AF72775  bl 0x821f3d58
	ctx.lr = 0x832815E8;
	sub_821F3D58(ctx, base);
	// 832815E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832815EC: 906AE054  stw r3, -0x1fac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8108 as u32), ctx.r[3].u32 ) };
	// 832815F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832815F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832815F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832815FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281600 size=56
    let mut pc: u32 = 0x83281600;
    'dispatch: loop {
        match pc {
            0x83281600 => {
    //   block [0x83281600..0x83281638)
	// 83281600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328160C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281614: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83281618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328161C: 4AF7273D  bl 0x821f3d58
	ctx.lr = 0x83281620;
	sub_821F3D58(ctx, base);
	// 83281620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281624: 906AE058  stw r3, -0x1fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8104 as u32), ctx.r[3].u32 ) };
	// 83281628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281638 size=56
    let mut pc: u32 = 0x83281638;
    'dispatch: loop {
        match pc {
            0x83281638 => {
    //   block [0x83281638..0x83281670)
	// 83281638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328164C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83281650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281654: 4AF72705  bl 0x821f3d58
	ctx.lr = 0x83281658;
	sub_821F3D58(ctx, base);
	// 83281658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328165C: 906AE05C  stw r3, -0x1fa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8100 as u32), ctx.r[3].u32 ) };
	// 83281660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328166C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281670 size=56
    let mut pc: u32 = 0x83281670;
    'dispatch: loop {
        match pc {
            0x83281670 => {
    //   block [0x83281670..0x832816A8)
	// 83281670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328167C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281684: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83281688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328168C: 4AF726CD  bl 0x821f3d58
	ctx.lr = 0x83281690;
	sub_821F3D58(ctx, base);
	// 83281690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281694: 906AE060  stw r3, -0x1fa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8096 as u32), ctx.r[3].u32 ) };
	// 83281698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832816A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832816A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832816A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832816A8 size=56
    let mut pc: u32 = 0x832816A8;
    'dispatch: loop {
        match pc {
            0x832816A8 => {
    //   block [0x832816A8..0x832816E0)
	// 832816A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832816AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832816B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832816B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832816B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832816BC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832816C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832816C4: 4AF72695  bl 0x821f3d58
	ctx.lr = 0x832816C8;
	sub_821F3D58(ctx, base);
	// 832816C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832816CC: 906AE064  stw r3, -0x1f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8092 as u32), ctx.r[3].u32 ) };
	// 832816D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832816D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832816D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832816DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832816E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832816E0 size=64
    let mut pc: u32 = 0x832816E0;
    'dispatch: loop {
        match pc {
            0x832816E0 => {
    //   block [0x832816E0..0x83281720)
	// 832816E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832816E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832816E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832816EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832816F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832816F4: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 832816F8: 386AE068  addi r3, r10, -0x1f98
	ctx.r[3].s64 = ctx.r[10].s64 + -8088;
	// 832816FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281700: 4AFAB7D1  bl 0x8222ced0
	ctx.lr = 0x83281704;
	sub_8222CED0(ctx, base);
	// 83281704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281708: 386910C8  addi r3, r9, 0x10c8
	ctx.r[3].s64 = ctx.r[9].s64 + 4296;
	// 8328170C: 4BA28815  bl 0x82ca9f20
	ctx.lr = 0x83281710;
	sub_82CA9F20(ctx, base);
	// 83281710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328171C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281720 size=64
    let mut pc: u32 = 0x83281720;
    'dispatch: loop {
        match pc {
            0x83281720 => {
    //   block [0x83281720..0x83281760)
	// 83281720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328172C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281734: 388BFE70  addi r4, r11, -0x190
	ctx.r[4].s64 = ctx.r[11].s64 + -400;
	// 83281738: 386AE06C  addi r3, r10, -0x1f94
	ctx.r[3].s64 = ctx.r[10].s64 + -8084;
	// 8328173C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281740: 4AFAB791  bl 0x8222ced0
	ctx.lr = 0x83281744;
	sub_8222CED0(ctx, base);
	// 83281744: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281748: 386910D8  addi r3, r9, 0x10d8
	ctx.r[3].s64 = ctx.r[9].s64 + 4312;
	// 8328174C: 4BA287D5  bl 0x82ca9f20
	ctx.lr = 0x83281750;
	sub_82CA9F20(ctx, base);
	// 83281750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328175C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281760 size=64
    let mut pc: u32 = 0x83281760;
    'dispatch: loop {
        match pc {
            0x83281760 => {
    //   block [0x83281760..0x832817A0)
	// 83281760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328176C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281774: 388BFEAC  addi r4, r11, -0x154
	ctx.r[4].s64 = ctx.r[11].s64 + -340;
	// 83281778: 386AE070  addi r3, r10, -0x1f90
	ctx.r[3].s64 = ctx.r[10].s64 + -8080;
	// 8328177C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281780: 4AFAB751  bl 0x8222ced0
	ctx.lr = 0x83281784;
	sub_8222CED0(ctx, base);
	// 83281784: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281788: 386910E8  addi r3, r9, 0x10e8
	ctx.r[3].s64 = ctx.r[9].s64 + 4328;
	// 8328178C: 4BA28795  bl 0x82ca9f20
	ctx.lr = 0x83281790;
	sub_82CA9F20(ctx, base);
	// 83281790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328179C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832817A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832817A0 size=64
    let mut pc: u32 = 0x832817A0;
    'dispatch: loop {
        match pc {
            0x832817A0 => {
    //   block [0x832817A0..0x832817E0)
	// 832817A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832817A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832817A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832817AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832817B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832817B4: 388BFEE8  addi r4, r11, -0x118
	ctx.r[4].s64 = ctx.r[11].s64 + -280;
	// 832817B8: 386AE074  addi r3, r10, -0x1f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -8076;
	// 832817BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832817C0: 4AFAB711  bl 0x8222ced0
	ctx.lr = 0x832817C4;
	sub_8222CED0(ctx, base);
	// 832817C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832817C8: 386910F8  addi r3, r9, 0x10f8
	ctx.r[3].s64 = ctx.r[9].s64 + 4344;
	// 832817CC: 4BA28755  bl 0x82ca9f20
	ctx.lr = 0x832817D0;
	sub_82CA9F20(ctx, base);
	// 832817D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832817D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832817D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832817DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832817E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832817E0 size=64
    let mut pc: u32 = 0x832817E0;
    'dispatch: loop {
        match pc {
            0x832817E0 => {
    //   block [0x832817E0..0x83281820)
	// 832817E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832817E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832817E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832817EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832817F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832817F4: 388BFF34  addi r4, r11, -0xcc
	ctx.r[4].s64 = ctx.r[11].s64 + -204;
	// 832817F8: 386AE078  addi r3, r10, -0x1f88
	ctx.r[3].s64 = ctx.r[10].s64 + -8072;
	// 832817FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281800: 4AFAB6D1  bl 0x8222ced0
	ctx.lr = 0x83281804;
	sub_8222CED0(ctx, base);
	// 83281804: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281808: 38691108  addi r3, r9, 0x1108
	ctx.r[3].s64 = ctx.r[9].s64 + 4360;
	// 8328180C: 4BA28715  bl 0x82ca9f20
	ctx.lr = 0x83281810;
	sub_82CA9F20(ctx, base);
	// 83281810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328181C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281820 size=376
    let mut pc: u32 = 0x83281820;
    'dispatch: loop {
        match pc {
            0x83281820 => {
    //   block [0x83281820..0x83281998)
	// 83281820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328182C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281830: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83281834: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83281838: 3BEBE080  addi r31, r11, -0x1f80
	ctx.r[31].s64 = ctx.r[11].s64 + -8064;
	// 8328183C: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 83281840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83281844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281848: 4AFAB689  bl 0x8222ced0
	ctx.lr = 0x8328184C;
	sub_8222CED0(ctx, base);
	// 8328184C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83281850: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281854: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 83281858: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8328185C: 4AFAB675  bl 0x8222ced0
	ctx.lr = 0x83281860;
	sub_8222CED0(ctx, base);
	// 83281860: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83281864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281868: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 8328186C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83281870: 4AFAB661  bl 0x8222ced0
	ctx.lr = 0x83281874;
	sub_8222CED0(ctx, base);
	// 83281874: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83281878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328187C: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 83281880: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83281884: 4AFAB64D  bl 0x8222ced0
	ctx.lr = 0x83281888;
	sub_8222CED0(ctx, base);
	// 83281888: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8328188C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281890: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 83281894: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83281898: 4AFAB639  bl 0x8222ced0
	ctx.lr = 0x8328189C;
	sub_8222CED0(ctx, base);
	// 8328189C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832818A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818A4: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 832818A8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832818AC: 4AFAB625  bl 0x8222ced0
	ctx.lr = 0x832818B0;
	sub_8222CED0(ctx, base);
	// 832818B0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832818B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818B8: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 832818BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832818C0: 4AFAB611  bl 0x8222ced0
	ctx.lr = 0x832818C4;
	sub_8222CED0(ctx, base);
	// 832818C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832818C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818CC: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 832818D0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832818D4: 4AFAB5FD  bl 0x8222ced0
	ctx.lr = 0x832818D8;
	sub_8222CED0(ctx, base);
	// 832818D8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832818DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818E0: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 832818E4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832818E8: 4AFAB5E9  bl 0x8222ced0
	ctx.lr = 0x832818EC;
	sub_8222CED0(ctx, base);
	// 832818EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832818F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832818F4: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 832818F8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832818FC: 4AFAB5D5  bl 0x8222ced0
	ctx.lr = 0x83281900;
	sub_8222CED0(ctx, base);
	// 83281900: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83281904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281908: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 8328190C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83281910: 4AFAB5C1  bl 0x8222ced0
	ctx.lr = 0x83281914;
	sub_8222CED0(ctx, base);
	// 83281914: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83281918: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328191C: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 83281920: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83281924: 4AFAB5AD  bl 0x8222ced0
	ctx.lr = 0x83281928;
	sub_8222CED0(ctx, base);
	// 83281928: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8328192C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281930: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 83281934: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83281938: 4AFAB599  bl 0x8222ced0
	ctx.lr = 0x8328193C;
	sub_8222CED0(ctx, base);
	// 8328193C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83281940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281944: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 83281948: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8328194C: 4AFAB585  bl 0x8222ced0
	ctx.lr = 0x83281950;
	sub_8222CED0(ctx, base);
	// 83281950: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83281954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281958: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 8328195C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83281960: 4AFAB571  bl 0x8222ced0
	ctx.lr = 0x83281964;
	sub_8222CED0(ctx, base);
	// 83281964: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83281968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328196C: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 83281970: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83281974: 4AFAB55D  bl 0x8222ced0
	ctx.lr = 0x83281978;
	sub_8222CED0(ctx, base);
	// 83281978: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328197C: 386A1118  addi r3, r10, 0x1118
	ctx.r[3].s64 = ctx.r[10].s64 + 4376;
	// 83281980: 4BA285A1  bl 0x82ca9f20
	ctx.lr = 0x83281984;
	sub_82CA9F20(ctx, base);
	// 83281984: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328198C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83281994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281998 size=376
    let mut pc: u32 = 0x83281998;
    'dispatch: loop {
        match pc {
            0x83281998 => {
    //   block [0x83281998..0x83281B10)
	// 83281998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832819A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832819A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832819A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832819AC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832819B0: 3BEBE0C0  addi r31, r11, -0x1f40
	ctx.r[31].s64 = ctx.r[11].s64 + -8000;
	// 832819B4: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 832819B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832819BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819C0: 4AFAB511  bl 0x8222ced0
	ctx.lr = 0x832819C4;
	sub_8222CED0(ctx, base);
	// 832819C4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832819C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819CC: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 832819D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832819D4: 4AFAB4FD  bl 0x8222ced0
	ctx.lr = 0x832819D8;
	sub_8222CED0(ctx, base);
	// 832819D8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832819DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819E0: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 832819E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832819E8: 4AFAB4E9  bl 0x8222ced0
	ctx.lr = 0x832819EC;
	sub_8222CED0(ctx, base);
	// 832819EC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832819F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832819F4: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 832819F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832819FC: 4AFAB4D5  bl 0x8222ced0
	ctx.lr = 0x83281A00;
	sub_8222CED0(ctx, base);
	// 83281A00: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83281A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A08: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 83281A0C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83281A10: 4AFAB4C1  bl 0x8222ced0
	ctx.lr = 0x83281A14;
	sub_8222CED0(ctx, base);
	// 83281A14: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83281A18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A1C: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 83281A20: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83281A24: 4AFAB4AD  bl 0x8222ced0
	ctx.lr = 0x83281A28;
	sub_8222CED0(ctx, base);
	// 83281A28: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83281A2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A30: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 83281A34: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83281A38: 4AFAB499  bl 0x8222ced0
	ctx.lr = 0x83281A3C;
	sub_8222CED0(ctx, base);
	// 83281A3C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83281A40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A44: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 83281A48: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83281A4C: 4AFAB485  bl 0x8222ced0
	ctx.lr = 0x83281A50;
	sub_8222CED0(ctx, base);
	// 83281A50: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83281A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A58: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 83281A5C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83281A60: 4AFAB471  bl 0x8222ced0
	ctx.lr = 0x83281A64;
	sub_8222CED0(ctx, base);
	// 83281A64: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83281A68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A6C: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 83281A70: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83281A74: 4AFAB45D  bl 0x8222ced0
	ctx.lr = 0x83281A78;
	sub_8222CED0(ctx, base);
	// 83281A78: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83281A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A80: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 83281A84: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83281A88: 4AFAB449  bl 0x8222ced0
	ctx.lr = 0x83281A8C;
	sub_8222CED0(ctx, base);
	// 83281A8C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83281A90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281A94: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 83281A98: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83281A9C: 4AFAB435  bl 0x8222ced0
	ctx.lr = 0x83281AA0;
	sub_8222CED0(ctx, base);
	// 83281AA0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83281AA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281AA8: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 83281AAC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83281AB0: 4AFAB421  bl 0x8222ced0
	ctx.lr = 0x83281AB4;
	sub_8222CED0(ctx, base);
	// 83281AB4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83281AB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281ABC: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 83281AC0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83281AC4: 4AFAB40D  bl 0x8222ced0
	ctx.lr = 0x83281AC8;
	sub_8222CED0(ctx, base);
	// 83281AC8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83281ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281AD0: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 83281AD4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83281AD8: 4AFAB3F9  bl 0x8222ced0
	ctx.lr = 0x83281ADC;
	sub_8222CED0(ctx, base);
	// 83281ADC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83281AE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281AE4: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 83281AE8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83281AEC: 4AFAB3E5  bl 0x8222ced0
	ctx.lr = 0x83281AF0;
	sub_8222CED0(ctx, base);
	// 83281AF0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83281AF4: 386A1180  addi r3, r10, 0x1180
	ctx.r[3].s64 = ctx.r[10].s64 + 4480;
	// 83281AF8: 4BA28429  bl 0x82ca9f20
	ctx.lr = 0x83281AFC;
	sub_82CA9F20(ctx, base);
	// 83281AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281B08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83281B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281B10 size=56
    let mut pc: u32 = 0x83281B10;
    'dispatch: loop {
        match pc {
            0x83281B10 => {
    //   block [0x83281B10..0x83281B48)
	// 83281B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281B1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281B20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281B24: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83281B28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281B2C: 4AF7222D  bl 0x821f3d58
	ctx.lr = 0x83281B30;
	sub_821F3D58(ctx, base);
	// 83281B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281B34: 906AE07C  stw r3, -0x1f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8068 as u32), ctx.r[3].u32 ) };
	// 83281B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281B48 size=56
    let mut pc: u32 = 0x83281B48;
    'dispatch: loop {
        match pc {
            0x83281B48 => {
    //   block [0x83281B48..0x83281B80)
	// 83281B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281B54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281B58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281B5C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83281B60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281B64: 4AF721F5  bl 0x821f3d58
	ctx.lr = 0x83281B68;
	sub_821F3D58(ctx, base);
	// 83281B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281B6C: 906AE100  stw r3, -0x1f00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7936 as u32), ctx.r[3].u32 ) };
	// 83281B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281B80 size=56
    let mut pc: u32 = 0x83281B80;
    'dispatch: loop {
        match pc {
            0x83281B80 => {
    //   block [0x83281B80..0x83281BB8)
	// 83281B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281B8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281B90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281B94: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83281B98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281B9C: 4AF721BD  bl 0x821f3d58
	ctx.lr = 0x83281BA0;
	sub_821F3D58(ctx, base);
	// 83281BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281BA4: 906AE104  stw r3, -0x1efc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7932 as u32), ctx.r[3].u32 ) };
	// 83281BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281BB8 size=56
    let mut pc: u32 = 0x83281BB8;
    'dispatch: loop {
        match pc {
            0x83281BB8 => {
    //   block [0x83281BB8..0x83281BF0)
	// 83281BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281BCC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83281BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281BD4: 4AF72185  bl 0x821f3d58
	ctx.lr = 0x83281BD8;
	sub_821F3D58(ctx, base);
	// 83281BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281BDC: 906AE108  stw r3, -0x1ef8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7928 as u32), ctx.r[3].u32 ) };
	// 83281BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281BF0 size=56
    let mut pc: u32 = 0x83281BF0;
    'dispatch: loop {
        match pc {
            0x83281BF0 => {
    //   block [0x83281BF0..0x83281C28)
	// 83281BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281C00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281C04: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83281C08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281C0C: 4AF7214D  bl 0x821f3d58
	ctx.lr = 0x83281C10;
	sub_821F3D58(ctx, base);
	// 83281C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281C14: 906AE10C  stw r3, -0x1ef4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7924 as u32), ctx.r[3].u32 ) };
	// 83281C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281C28 size=56
    let mut pc: u32 = 0x83281C28;
    'dispatch: loop {
        match pc {
            0x83281C28 => {
    //   block [0x83281C28..0x83281C60)
	// 83281C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281C38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281C3C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83281C40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281C44: 4AF72115  bl 0x821f3d58
	ctx.lr = 0x83281C48;
	sub_821F3D58(ctx, base);
	// 83281C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281C4C: 906AE110  stw r3, -0x1ef0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7920 as u32), ctx.r[3].u32 ) };
	// 83281C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281C60 size=56
    let mut pc: u32 = 0x83281C60;
    'dispatch: loop {
        match pc {
            0x83281C60 => {
    //   block [0x83281C60..0x83281C98)
	// 83281C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281C6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281C70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281C74: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83281C78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281C7C: 4AF720DD  bl 0x821f3d58
	ctx.lr = 0x83281C80;
	sub_821F3D58(ctx, base);
	// 83281C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281C84: 906AE114  stw r3, -0x1eec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7916 as u32), ctx.r[3].u32 ) };
	// 83281C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281C98 size=56
    let mut pc: u32 = 0x83281C98;
    'dispatch: loop {
        match pc {
            0x83281C98 => {
    //   block [0x83281C98..0x83281CD0)
	// 83281C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281CA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281CA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281CAC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83281CB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281CB4: 4AF720A5  bl 0x821f3d58
	ctx.lr = 0x83281CB8;
	sub_821F3D58(ctx, base);
	// 83281CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281CBC: 906AE118  stw r3, -0x1ee8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7912 as u32), ctx.r[3].u32 ) };
	// 83281CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281CD0 size=56
    let mut pc: u32 = 0x83281CD0;
    'dispatch: loop {
        match pc {
            0x83281CD0 => {
    //   block [0x83281CD0..0x83281D08)
	// 83281CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281CDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281CE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281CE4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83281CE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281CEC: 4AF7206D  bl 0x821f3d58
	ctx.lr = 0x83281CF0;
	sub_821F3D58(ctx, base);
	// 83281CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281CF4: 906AE11C  stw r3, -0x1ee4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7908 as u32), ctx.r[3].u32 ) };
	// 83281CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281D08 size=56
    let mut pc: u32 = 0x83281D08;
    'dispatch: loop {
        match pc {
            0x83281D08 => {
    //   block [0x83281D08..0x83281D40)
	// 83281D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281D14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281D18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281D1C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83281D20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281D24: 4AF72035  bl 0x821f3d58
	ctx.lr = 0x83281D28;
	sub_821F3D58(ctx, base);
	// 83281D28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281D2C: 906AE120  stw r3, -0x1ee0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7904 as u32), ctx.r[3].u32 ) };
	// 83281D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281D40 size=56
    let mut pc: u32 = 0x83281D40;
    'dispatch: loop {
        match pc {
            0x83281D40 => {
    //   block [0x83281D40..0x83281D78)
	// 83281D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281D4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281D50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281D54: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83281D58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281D5C: 4AF71FFD  bl 0x821f3d58
	ctx.lr = 0x83281D60;
	sub_821F3D58(ctx, base);
	// 83281D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281D64: 906AE124  stw r3, -0x1edc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7900 as u32), ctx.r[3].u32 ) };
	// 83281D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281D78 size=56
    let mut pc: u32 = 0x83281D78;
    'dispatch: loop {
        match pc {
            0x83281D78 => {
    //   block [0x83281D78..0x83281DB0)
	// 83281D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281D88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281D8C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83281D90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281D94: 4AF71FC5  bl 0x821f3d58
	ctx.lr = 0x83281D98;
	sub_821F3D58(ctx, base);
	// 83281D98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281D9C: 906AE128  stw r3, -0x1ed8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7896 as u32), ctx.r[3].u32 ) };
	// 83281DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281DB0 size=56
    let mut pc: u32 = 0x83281DB0;
    'dispatch: loop {
        match pc {
            0x83281DB0 => {
    //   block [0x83281DB0..0x83281DE8)
	// 83281DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281DC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281DC4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83281DC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281DCC: 4AF71F8D  bl 0x821f3d58
	ctx.lr = 0x83281DD0;
	sub_821F3D58(ctx, base);
	// 83281DD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281DD4: 906AE12C  stw r3, -0x1ed4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7892 as u32), ctx.r[3].u32 ) };
	// 83281DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281DE8 size=56
    let mut pc: u32 = 0x83281DE8;
    'dispatch: loop {
        match pc {
            0x83281DE8 => {
    //   block [0x83281DE8..0x83281E20)
	// 83281DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281DF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281DF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281DFC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83281E00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281E04: 4AF71F55  bl 0x821f3d58
	ctx.lr = 0x83281E08;
	sub_821F3D58(ctx, base);
	// 83281E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281E0C: 906AE130  stw r3, -0x1ed0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7888 as u32), ctx.r[3].u32 ) };
	// 83281E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281E20 size=56
    let mut pc: u32 = 0x83281E20;
    'dispatch: loop {
        match pc {
            0x83281E20 => {
    //   block [0x83281E20..0x83281E58)
	// 83281E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281E2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281E30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281E34: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83281E38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281E3C: 4AF71F1D  bl 0x821f3d58
	ctx.lr = 0x83281E40;
	sub_821F3D58(ctx, base);
	// 83281E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281E44: 906AE134  stw r3, -0x1ecc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7884 as u32), ctx.r[3].u32 ) };
	// 83281E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281E58 size=56
    let mut pc: u32 = 0x83281E58;
    'dispatch: loop {
        match pc {
            0x83281E58 => {
    //   block [0x83281E58..0x83281E90)
	// 83281E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281E64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281E68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281E6C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83281E70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281E74: 4AF71EE5  bl 0x821f3d58
	ctx.lr = 0x83281E78;
	sub_821F3D58(ctx, base);
	// 83281E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281E7C: 906AE138  stw r3, -0x1ec8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7880 as u32), ctx.r[3].u32 ) };
	// 83281E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281E90 size=56
    let mut pc: u32 = 0x83281E90;
    'dispatch: loop {
        match pc {
            0x83281E90 => {
    //   block [0x83281E90..0x83281EC8)
	// 83281E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281E9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281EA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281EA4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83281EA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281EAC: 4AF71EAD  bl 0x821f3d58
	ctx.lr = 0x83281EB0;
	sub_821F3D58(ctx, base);
	// 83281EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281EB4: 906AE13C  stw r3, -0x1ec4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7876 as u32), ctx.r[3].u32 ) };
	// 83281EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281EC8 size=56
    let mut pc: u32 = 0x83281EC8;
    'dispatch: loop {
        match pc {
            0x83281EC8 => {
    //   block [0x83281EC8..0x83281F00)
	// 83281EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281ED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281ED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281EDC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83281EE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281EE4: 4AF71E75  bl 0x821f3d58
	ctx.lr = 0x83281EE8;
	sub_821F3D58(ctx, base);
	// 83281EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281EEC: 906AE140  stw r3, -0x1ec0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7872 as u32), ctx.r[3].u32 ) };
	// 83281EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281F00 size=56
    let mut pc: u32 = 0x83281F00;
    'dispatch: loop {
        match pc {
            0x83281F00 => {
    //   block [0x83281F00..0x83281F38)
	// 83281F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281F0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281F10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281F14: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83281F18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281F1C: 4AF71E3D  bl 0x821f3d58
	ctx.lr = 0x83281F20;
	sub_821F3D58(ctx, base);
	// 83281F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281F24: 906AE144  stw r3, -0x1ebc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7868 as u32), ctx.r[3].u32 ) };
	// 83281F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281F38 size=56
    let mut pc: u32 = 0x83281F38;
    'dispatch: loop {
        match pc {
            0x83281F38 => {
    //   block [0x83281F38..0x83281F70)
	// 83281F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281F44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281F48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281F4C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83281F50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281F54: 4AF71E05  bl 0x821f3d58
	ctx.lr = 0x83281F58;
	sub_821F3D58(ctx, base);
	// 83281F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281F5C: 906AE148  stw r3, -0x1eb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7864 as u32), ctx.r[3].u32 ) };
	// 83281F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281F70 size=56
    let mut pc: u32 = 0x83281F70;
    'dispatch: loop {
        match pc {
            0x83281F70 => {
    //   block [0x83281F70..0x83281FA8)
	// 83281F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281F80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281F84: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83281F88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281F8C: 4AF71DCD  bl 0x821f3d58
	ctx.lr = 0x83281F90;
	sub_821F3D58(ctx, base);
	// 83281F90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281F94: 906AE14C  stw r3, -0x1eb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7860 as u32), ctx.r[3].u32 ) };
	// 83281F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281FA8 size=64
    let mut pc: u32 = 0x83281FA8;
    'dispatch: loop {
        match pc {
            0x83281FA8 => {
    //   block [0x83281FA8..0x83281FE8)
	// 83281FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281FB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281FBC: 388BFFC8  addi r4, r11, -0x38
	ctx.r[4].s64 = ctx.r[11].s64 + -56;
	// 83281FC0: 386AE150  addi r3, r10, -0x1eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -7856;
	// 83281FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281FC8: 4AFAAF09  bl 0x8222ced0
	ctx.lr = 0x83281FCC;
	sub_8222CED0(ctx, base);
	// 83281FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281FD0: 386911E8  addi r3, r9, 0x11e8
	ctx.r[3].s64 = ctx.r[9].s64 + 4584;
	// 83281FD4: 4BA27F4D  bl 0x82ca9f20
	ctx.lr = 0x83281FD8;
	sub_82CA9F20(ctx, base);
	// 83281FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281FE8 size=64
    let mut pc: u32 = 0x83281FE8;
    'dispatch: loop {
        match pc {
            0x83281FE8 => {
    //   block [0x83281FE8..0x83282028)
	// 83281FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281FF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281FFC: 388B0000  addi r4, r11, 0
	ctx.r[4].s64 = ctx.r[11].s64 + 0;
	// 83282000: 386AE154  addi r3, r10, -0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + -7852;
	// 83282004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282008: 4AFAAEC9  bl 0x8222ced0
	ctx.lr = 0x8328200C;
	sub_8222CED0(ctx, base);
	// 8328200C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282010: 386911F8  addi r3, r9, 0x11f8
	ctx.r[3].s64 = ctx.r[9].s64 + 4600;
	// 83282014: 4BA27F0D  bl 0x82ca9f20
	ctx.lr = 0x83282018;
	sub_82CA9F20(ctx, base);
	// 83282018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328201C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282028 size=64
    let mut pc: u32 = 0x83282028;
    'dispatch: loop {
        match pc {
            0x83282028 => {
    //   block [0x83282028..0x83282068)
	// 83282028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282034: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328203C: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 83282040: 386AE158  addi r3, r10, -0x1ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -7848;
	// 83282044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282048: 4AFAAE89  bl 0x8222ced0
	ctx.lr = 0x8328204C;
	sub_8222CED0(ctx, base);
	// 8328204C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282050: 38691208  addi r3, r9, 0x1208
	ctx.r[3].s64 = ctx.r[9].s64 + 4616;
	// 83282054: 4BA27ECD  bl 0x82ca9f20
	ctx.lr = 0x83282058;
	sub_82CA9F20(ctx, base);
	// 83282058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282068 size=64
    let mut pc: u32 = 0x83282068;
    'dispatch: loop {
        match pc {
            0x83282068 => {
    //   block [0x83282068..0x832820A8)
	// 83282068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282074: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328207C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 83282080: 386AE15C  addi r3, r10, -0x1ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -7844;
	// 83282084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282088: 4AFAAE49  bl 0x8222ced0
	ctx.lr = 0x8328208C;
	sub_8222CED0(ctx, base);
	// 8328208C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282090: 38691218  addi r3, r9, 0x1218
	ctx.r[3].s64 = ctx.r[9].s64 + 4632;
	// 83282094: 4BA27E8D  bl 0x82ca9f20
	ctx.lr = 0x83282098;
	sub_82CA9F20(ctx, base);
	// 83282098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832820A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832820A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832820A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832820A8 size=56
    let mut pc: u32 = 0x832820A8;
    'dispatch: loop {
        match pc {
            0x832820A8 => {
    //   block [0x832820A8..0x832820E0)
	// 832820A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832820AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832820B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832820B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832820B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832820BC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832820C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832820C4: 4AF71C95  bl 0x821f3d58
	ctx.lr = 0x832820C8;
	sub_821F3D58(ctx, base);
	// 832820C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832820CC: 906AE160  stw r3, -0x1ea0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7840 as u32), ctx.r[3].u32 ) };
	// 832820D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832820D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832820D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832820DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832820E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832820E0 size=56
    let mut pc: u32 = 0x832820E0;
    'dispatch: loop {
        match pc {
            0x832820E0 => {
    //   block [0x832820E0..0x83282118)
	// 832820E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832820E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832820E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832820EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832820F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832820F4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832820F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832820FC: 4AF71C5D  bl 0x821f3d58
	ctx.lr = 0x83282100;
	sub_821F3D58(ctx, base);
	// 83282100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282104: 906AE164  stw r3, -0x1e9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7836 as u32), ctx.r[3].u32 ) };
	// 83282108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328210C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282118 size=56
    let mut pc: u32 = 0x83282118;
    'dispatch: loop {
        match pc {
            0x83282118 => {
    //   block [0x83282118..0x83282150)
	// 83282118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328212C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83282130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282134: 4AF71C25  bl 0x821f3d58
	ctx.lr = 0x83282138;
	sub_821F3D58(ctx, base);
	// 83282138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328213C: 906AE168  stw r3, -0x1e98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7832 as u32), ctx.r[3].u32 ) };
	// 83282140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282150 size=56
    let mut pc: u32 = 0x83282150;
    'dispatch: loop {
        match pc {
            0x83282150 => {
    //   block [0x83282150..0x83282188)
	// 83282150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328215C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282164: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83282168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328216C: 4AF71BED  bl 0x821f3d58
	ctx.lr = 0x83282170;
	sub_821F3D58(ctx, base);
	// 83282170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282174: 906AE16C  stw r3, -0x1e94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7828 as u32), ctx.r[3].u32 ) };
	// 83282178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328217C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282188 size=56
    let mut pc: u32 = 0x83282188;
    'dispatch: loop {
        match pc {
            0x83282188 => {
    //   block [0x83282188..0x832821C0)
	// 83282188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282194: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282198: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328219C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832821A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832821A4: 4AF71BB5  bl 0x821f3d58
	ctx.lr = 0x832821A8;
	sub_821F3D58(ctx, base);
	// 832821A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832821AC: 906AE170  stw r3, -0x1e90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7824 as u32), ctx.r[3].u32 ) };
	// 832821B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832821B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832821B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832821BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832821C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832821C0 size=56
    let mut pc: u32 = 0x832821C0;
    'dispatch: loop {
        match pc {
            0x832821C0 => {
    //   block [0x832821C0..0x832821F8)
	// 832821C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832821C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832821C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832821CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832821D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832821D4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832821D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832821DC: 4AF71B7D  bl 0x821f3d58
	ctx.lr = 0x832821E0;
	sub_821F3D58(ctx, base);
	// 832821E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832821E4: 906AE174  stw r3, -0x1e8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7820 as u32), ctx.r[3].u32 ) };
	// 832821E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832821EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832821F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832821F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832821F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832821F8 size=56
    let mut pc: u32 = 0x832821F8;
    'dispatch: loop {
        match pc {
            0x832821F8 => {
    //   block [0x832821F8..0x83282230)
	// 832821F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832821FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282208: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328220C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83282210: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282214: 4AF71B45  bl 0x821f3d58
	ctx.lr = 0x83282218;
	sub_821F3D58(ctx, base);
	// 83282218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328221C: 906AE178  stw r3, -0x1e88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7816 as u32), ctx.r[3].u32 ) };
	// 83282220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328222C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282230 size=56
    let mut pc: u32 = 0x83282230;
    'dispatch: loop {
        match pc {
            0x83282230 => {
    //   block [0x83282230..0x83282268)
	// 83282230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328223C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282240: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282244: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83282248: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328224C: 4AF71B0D  bl 0x821f3d58
	ctx.lr = 0x83282250;
	sub_821F3D58(ctx, base);
	// 83282250: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282254: 906AE17C  stw r3, -0x1e84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7812 as u32), ctx.r[3].u32 ) };
	// 83282258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282268 size=56
    let mut pc: u32 = 0x83282268;
    'dispatch: loop {
        match pc {
            0x83282268 => {
    //   block [0x83282268..0x832822A0)
	// 83282268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282274: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282278: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328227C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83282280: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282284: 4AF71AD5  bl 0x821f3d58
	ctx.lr = 0x83282288;
	sub_821F3D58(ctx, base);
	// 83282288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328228C: 906AE180  stw r3, -0x1e80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7808 as u32), ctx.r[3].u32 ) };
	// 83282290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328229C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832822A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832822A0 size=56
    let mut pc: u32 = 0x832822A0;
    'dispatch: loop {
        match pc {
            0x832822A0 => {
    //   block [0x832822A0..0x832822D8)
	// 832822A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832822A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832822A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832822AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832822B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832822B4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832822B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832822BC: 4AF71A9D  bl 0x821f3d58
	ctx.lr = 0x832822C0;
	sub_821F3D58(ctx, base);
	// 832822C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832822C4: 906AE184  stw r3, -0x1e7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7804 as u32), ctx.r[3].u32 ) };
	// 832822C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832822CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832822D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832822D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832822D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832822D8 size=56
    let mut pc: u32 = 0x832822D8;
    'dispatch: loop {
        match pc {
            0x832822D8 => {
    //   block [0x832822D8..0x83282310)
	// 832822D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832822DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832822E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832822E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832822E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832822EC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832822F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832822F4: 4AF71A65  bl 0x821f3d58
	ctx.lr = 0x832822F8;
	sub_821F3D58(ctx, base);
	// 832822F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832822FC: 906AE188  stw r3, -0x1e78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7800 as u32), ctx.r[3].u32 ) };
	// 83282300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328230C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282310 size=56
    let mut pc: u32 = 0x83282310;
    'dispatch: loop {
        match pc {
            0x83282310 => {
    //   block [0x83282310..0x83282348)
	// 83282310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328231C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282320: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282324: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83282328: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328232C: 4AF71A2D  bl 0x821f3d58
	ctx.lr = 0x83282330;
	sub_821F3D58(ctx, base);
	// 83282330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282334: 906AE18C  stw r3, -0x1e74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7796 as u32), ctx.r[3].u32 ) };
	// 83282338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328233C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282348 size=56
    let mut pc: u32 = 0x83282348;
    'dispatch: loop {
        match pc {
            0x83282348 => {
    //   block [0x83282348..0x83282380)
	// 83282348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282354: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282358: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328235C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83282360: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282364: 4AF719F5  bl 0x821f3d58
	ctx.lr = 0x83282368;
	sub_821F3D58(ctx, base);
	// 83282368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328236C: 906AE190  stw r3, -0x1e70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7792 as u32), ctx.r[3].u32 ) };
	// 83282370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328237C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282380 size=56
    let mut pc: u32 = 0x83282380;
    'dispatch: loop {
        match pc {
            0x83282380 => {
    //   block [0x83282380..0x832823B8)
	// 83282380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328238C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282390: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282394: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83282398: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328239C: 4AF719BD  bl 0x821f3d58
	ctx.lr = 0x832823A0;
	sub_821F3D58(ctx, base);
	// 832823A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832823A4: 906AE194  stw r3, -0x1e6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7788 as u32), ctx.r[3].u32 ) };
	// 832823A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832823AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832823B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832823B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832823B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832823B8 size=56
    let mut pc: u32 = 0x832823B8;
    'dispatch: loop {
        match pc {
            0x832823B8 => {
    //   block [0x832823B8..0x832823F0)
	// 832823B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832823BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832823C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832823C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832823C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832823CC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832823D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832823D4: 4AF71985  bl 0x821f3d58
	ctx.lr = 0x832823D8;
	sub_821F3D58(ctx, base);
	// 832823D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832823DC: 906AE198  stw r3, -0x1e68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7784 as u32), ctx.r[3].u32 ) };
	// 832823E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832823E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832823E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832823EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832823F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832823F0 size=56
    let mut pc: u32 = 0x832823F0;
    'dispatch: loop {
        match pc {
            0x832823F0 => {
    //   block [0x832823F0..0x83282428)
	// 832823F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832823F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832823F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832823FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282404: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83282408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328240C: 4AF7194D  bl 0x821f3d58
	ctx.lr = 0x83282410;
	sub_821F3D58(ctx, base);
	// 83282410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282414: 906AE19C  stw r3, -0x1e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7780 as u32), ctx.r[3].u32 ) };
	// 83282418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328241C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282428 size=56
    let mut pc: u32 = 0x83282428;
    'dispatch: loop {
        match pc {
            0x83282428 => {
    //   block [0x83282428..0x83282460)
	// 83282428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328242C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282434: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328243C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83282440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282444: 4AF71915  bl 0x821f3d58
	ctx.lr = 0x83282448;
	sub_821F3D58(ctx, base);
	// 83282448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328244C: 906AE1A0  stw r3, -0x1e60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7776 as u32), ctx.r[3].u32 ) };
	// 83282450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328245C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282460 size=56
    let mut pc: u32 = 0x83282460;
    'dispatch: loop {
        match pc {
            0x83282460 => {
    //   block [0x83282460..0x83282498)
	// 83282460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328246C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282474: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83282478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328247C: 4AF718DD  bl 0x821f3d58
	ctx.lr = 0x83282480;
	sub_821F3D58(ctx, base);
	// 83282480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282484: 906AE1A4  stw r3, -0x1e5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7772 as u32), ctx.r[3].u32 ) };
	// 83282488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328248C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282498 size=56
    let mut pc: u32 = 0x83282498;
    'dispatch: loop {
        match pc {
            0x83282498 => {
    //   block [0x83282498..0x832824D0)
	// 83282498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328249C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832824A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832824A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832824A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832824AC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832824B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832824B4: 4AF718A5  bl 0x821f3d58
	ctx.lr = 0x832824B8;
	sub_821F3D58(ctx, base);
	// 832824B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832824BC: 906AE1A8  stw r3, -0x1e58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7768 as u32), ctx.r[3].u32 ) };
	// 832824C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832824C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832824C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832824CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832824D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832824D0 size=56
    let mut pc: u32 = 0x832824D0;
    'dispatch: loop {
        match pc {
            0x832824D0 => {
    //   block [0x832824D0..0x83282508)
	// 832824D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832824D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832824D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832824DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832824E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832824E4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832824E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832824EC: 4AF7186D  bl 0x821f3d58
	ctx.lr = 0x832824F0;
	sub_821F3D58(ctx, base);
	// 832824F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832824F4: 906AE1AC  stw r3, -0x1e54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7764 as u32), ctx.r[3].u32 ) };
	// 832824F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832824FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282508 size=56
    let mut pc: u32 = 0x83282508;
    'dispatch: loop {
        match pc {
            0x83282508 => {
    //   block [0x83282508..0x83282540)
	// 83282508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328251C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83282520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282524: 4AF71835  bl 0x821f3d58
	ctx.lr = 0x83282528;
	sub_821F3D58(ctx, base);
	// 83282528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328252C: 906AE1B0  stw r3, -0x1e50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7760 as u32), ctx.r[3].u32 ) };
	// 83282530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328253C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282540 size=64
    let mut pc: u32 = 0x83282540;
    'dispatch: loop {
        match pc {
            0x83282540 => {
    //   block [0x83282540..0x83282580)
	// 83282540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328254C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282554: 388B00B8  addi r4, r11, 0xb8
	ctx.r[4].s64 = ctx.r[11].s64 + 184;
	// 83282558: 386AE1B4  addi r3, r10, -0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7756;
	// 8328255C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282560: 4AFAA971  bl 0x8222ced0
	ctx.lr = 0x83282564;
	sub_8222CED0(ctx, base);
	// 83282564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282568: 38691228  addi r3, r9, 0x1228
	ctx.r[3].s64 = ctx.r[9].s64 + 4648;
	// 8328256C: 4BA279B5  bl 0x82ca9f20
	ctx.lr = 0x83282570;
	sub_82CA9F20(ctx, base);
	// 83282570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328257C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282580 size=64
    let mut pc: u32 = 0x83282580;
    'dispatch: loop {
        match pc {
            0x83282580 => {
    //   block [0x83282580..0x832825C0)
	// 83282580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328258C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282594: 388B00B8  addi r4, r11, 0xb8
	ctx.r[4].s64 = ctx.r[11].s64 + 184;
	// 83282598: 386AE1B8  addi r3, r10, -0x1e48
	ctx.r[3].s64 = ctx.r[10].s64 + -7752;
	// 8328259C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832825A0: 4AFAA931  bl 0x8222ced0
	ctx.lr = 0x832825A4;
	sub_8222CED0(ctx, base);
	// 832825A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832825A8: 38691238  addi r3, r9, 0x1238
	ctx.r[3].s64 = ctx.r[9].s64 + 4664;
	// 832825AC: 4BA27975  bl 0x82ca9f20
	ctx.lr = 0x832825B0;
	sub_82CA9F20(ctx, base);
	// 832825B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832825B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832825B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832825BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832825C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832825C0 size=64
    let mut pc: u32 = 0x832825C0;
    'dispatch: loop {
        match pc {
            0x832825C0 => {
    //   block [0x832825C0..0x83282600)
	// 832825C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832825C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832825C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832825CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832825D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832825D4: 388B00F8  addi r4, r11, 0xf8
	ctx.r[4].s64 = ctx.r[11].s64 + 248;
	// 832825D8: 386AE1BC  addi r3, r10, -0x1e44
	ctx.r[3].s64 = ctx.r[10].s64 + -7748;
	// 832825DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832825E0: 4AFAA8F1  bl 0x8222ced0
	ctx.lr = 0x832825E4;
	sub_8222CED0(ctx, base);
	// 832825E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832825E8: 38691248  addi r3, r9, 0x1248
	ctx.r[3].s64 = ctx.r[9].s64 + 4680;
	// 832825EC: 4BA27935  bl 0x82ca9f20
	ctx.lr = 0x832825F0;
	sub_82CA9F20(ctx, base);
	// 832825F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832825F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832825F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832825FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282600 size=64
    let mut pc: u32 = 0x83282600;
    'dispatch: loop {
        match pc {
            0x83282600 => {
    //   block [0x83282600..0x83282640)
	// 83282600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328260C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282614: 388B013C  addi r4, r11, 0x13c
	ctx.r[4].s64 = ctx.r[11].s64 + 316;
	// 83282618: 386AE1C0  addi r3, r10, -0x1e40
	ctx.r[3].s64 = ctx.r[10].s64 + -7744;
	// 8328261C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282620: 4AFAA8B1  bl 0x8222ced0
	ctx.lr = 0x83282624;
	sub_8222CED0(ctx, base);
	// 83282624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282628: 38691258  addi r3, r9, 0x1258
	ctx.r[3].s64 = ctx.r[9].s64 + 4696;
	// 8328262C: 4BA278F5  bl 0x82ca9f20
	ctx.lr = 0x83282630;
	sub_82CA9F20(ctx, base);
	// 83282630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328263C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282640 size=64
    let mut pc: u32 = 0x83282640;
    'dispatch: loop {
        match pc {
            0x83282640 => {
    //   block [0x83282640..0x83282680)
	// 83282640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328264C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282654: 388B0178  addi r4, r11, 0x178
	ctx.r[4].s64 = ctx.r[11].s64 + 376;
	// 83282658: 386AE1C4  addi r3, r10, -0x1e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7740;
	// 8328265C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282660: 4AFAA871  bl 0x8222ced0
	ctx.lr = 0x83282664;
	sub_8222CED0(ctx, base);
	// 83282664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282668: 38691268  addi r3, r9, 0x1268
	ctx.r[3].s64 = ctx.r[9].s64 + 4712;
	// 8328266C: 4BA278B5  bl 0x82ca9f20
	ctx.lr = 0x83282670;
	sub_82CA9F20(ctx, base);
	// 83282670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328267C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282680 size=64
    let mut pc: u32 = 0x83282680;
    'dispatch: loop {
        match pc {
            0x83282680 => {
    //   block [0x83282680..0x832826C0)
	// 83282680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328268C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282694: 388B01BC  addi r4, r11, 0x1bc
	ctx.r[4].s64 = ctx.r[11].s64 + 444;
	// 83282698: 386AE1C8  addi r3, r10, -0x1e38
	ctx.r[3].s64 = ctx.r[10].s64 + -7736;
	// 8328269C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832826A0: 4AFAA831  bl 0x8222ced0
	ctx.lr = 0x832826A4;
	sub_8222CED0(ctx, base);
	// 832826A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832826A8: 38691278  addi r3, r9, 0x1278
	ctx.r[3].s64 = ctx.r[9].s64 + 4728;
	// 832826AC: 4BA27875  bl 0x82ca9f20
	ctx.lr = 0x832826B0;
	sub_82CA9F20(ctx, base);
	// 832826B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832826B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832826B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832826BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832826C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832826C0 size=64
    let mut pc: u32 = 0x832826C0;
    'dispatch: loop {
        match pc {
            0x832826C0 => {
    //   block [0x832826C0..0x83282700)
	// 832826C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832826C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832826C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832826CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832826D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832826D4: 388B01F8  addi r4, r11, 0x1f8
	ctx.r[4].s64 = ctx.r[11].s64 + 504;
	// 832826D8: 386AE1CC  addi r3, r10, -0x1e34
	ctx.r[3].s64 = ctx.r[10].s64 + -7732;
	// 832826DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832826E0: 4AFAA7F1  bl 0x8222ced0
	ctx.lr = 0x832826E4;
	sub_8222CED0(ctx, base);
	// 832826E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832826E8: 38691288  addi r3, r9, 0x1288
	ctx.r[3].s64 = ctx.r[9].s64 + 4744;
	// 832826EC: 4BA27835  bl 0x82ca9f20
	ctx.lr = 0x832826F0;
	sub_82CA9F20(ctx, base);
	// 832826F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832826F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832826F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832826FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282700 size=64
    let mut pc: u32 = 0x83282700;
    'dispatch: loop {
        match pc {
            0x83282700 => {
    //   block [0x83282700..0x83282740)
	// 83282700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328270C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282714: 388B0240  addi r4, r11, 0x240
	ctx.r[4].s64 = ctx.r[11].s64 + 576;
	// 83282718: 386AE1D0  addi r3, r10, -0x1e30
	ctx.r[3].s64 = ctx.r[10].s64 + -7728;
	// 8328271C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282720: 4AFAA7B1  bl 0x8222ced0
	ctx.lr = 0x83282724;
	sub_8222CED0(ctx, base);
	// 83282724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282728: 38691298  addi r3, r9, 0x1298
	ctx.r[3].s64 = ctx.r[9].s64 + 4760;
	// 8328272C: 4BA277F5  bl 0x82ca9f20
	ctx.lr = 0x83282730;
	sub_82CA9F20(ctx, base);
	// 83282730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328273C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282740 size=64
    let mut pc: u32 = 0x83282740;
    'dispatch: loop {
        match pc {
            0x83282740 => {
    //   block [0x83282740..0x83282780)
	// 83282740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328274C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282754: 388B0280  addi r4, r11, 0x280
	ctx.r[4].s64 = ctx.r[11].s64 + 640;
	// 83282758: 386AE1D4  addi r3, r10, -0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7724;
	// 8328275C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282760: 4AFAA771  bl 0x8222ced0
	ctx.lr = 0x83282764;
	sub_8222CED0(ctx, base);
	// 83282764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282768: 386912A8  addi r3, r9, 0x12a8
	ctx.r[3].s64 = ctx.r[9].s64 + 4776;
	// 8328276C: 4BA277B5  bl 0x82ca9f20
	ctx.lr = 0x83282770;
	sub_82CA9F20(ctx, base);
	// 83282770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282780 size=64
    let mut pc: u32 = 0x83282780;
    'dispatch: loop {
        match pc {
            0x83282780 => {
    //   block [0x83282780..0x832827C0)
	// 83282780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328278C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282794: 388B02C4  addi r4, r11, 0x2c4
	ctx.r[4].s64 = ctx.r[11].s64 + 708;
	// 83282798: 386AE1D8  addi r3, r10, -0x1e28
	ctx.r[3].s64 = ctx.r[10].s64 + -7720;
	// 8328279C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832827A0: 4AFAA731  bl 0x8222ced0
	ctx.lr = 0x832827A4;
	sub_8222CED0(ctx, base);
	// 832827A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832827A8: 386912B8  addi r3, r9, 0x12b8
	ctx.r[3].s64 = ctx.r[9].s64 + 4792;
	// 832827AC: 4BA27775  bl 0x82ca9f20
	ctx.lr = 0x832827B0;
	sub_82CA9F20(ctx, base);
	// 832827B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832827B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832827B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832827BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832827C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832827C0 size=64
    let mut pc: u32 = 0x832827C0;
    'dispatch: loop {
        match pc {
            0x832827C0 => {
    //   block [0x832827C0..0x83282800)
	// 832827C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832827C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832827C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832827CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832827D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832827D4: 388B0300  addi r4, r11, 0x300
	ctx.r[4].s64 = ctx.r[11].s64 + 768;
	// 832827D8: 386AE1DC  addi r3, r10, -0x1e24
	ctx.r[3].s64 = ctx.r[10].s64 + -7716;
	// 832827DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832827E0: 4AFAA6F1  bl 0x8222ced0
	ctx.lr = 0x832827E4;
	sub_8222CED0(ctx, base);
	// 832827E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832827E8: 386912C8  addi r3, r9, 0x12c8
	ctx.r[3].s64 = ctx.r[9].s64 + 4808;
	// 832827EC: 4BA27735  bl 0x82ca9f20
	ctx.lr = 0x832827F0;
	sub_82CA9F20(ctx, base);
	// 832827F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832827F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832827F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832827FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282800 size=156
    let mut pc: u32 = 0x83282800;
    'dispatch: loop {
        match pc {
            0x83282800 => {
    //   block [0x83282800..0x8328289C)
	// 83282800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328280C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83282814: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83282818: 3BEBE1E0  addi r31, r11, -0x1e20
	ctx.r[31].s64 = ctx.r[11].s64 + -7712;
	// 8328281C: 388A0398  addi r4, r10, 0x398
	ctx.r[4].s64 = ctx.r[10].s64 + 920;
	// 83282820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83282824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282828: 4AFAA6A9  bl 0x8222ced0
	ctx.lr = 0x8328282C;
	sub_8222CED0(ctx, base);
	// 8328282C: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83282830: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282834: 38890380  addi r4, r9, 0x380
	ctx.r[4].s64 = ctx.r[9].s64 + 896;
	// 83282838: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8328283C: 4AFAA695  bl 0x8222ced0
	ctx.lr = 0x83282840;
	sub_8222CED0(ctx, base);
	// 83282840: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83282844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282848: 38880364  addi r4, r8, 0x364
	ctx.r[4].s64 = ctx.r[8].s64 + 868;
	// 8328284C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83282850: 4AFAA681  bl 0x8222ced0
	ctx.lr = 0x83282854;
	sub_8222CED0(ctx, base);
	// 83282854: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83282858: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328285C: 3887034C  addi r4, r7, 0x34c
	ctx.r[4].s64 = ctx.r[7].s64 + 844;
	// 83282860: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83282864: 4AFAA66D  bl 0x8222ced0
	ctx.lr = 0x83282868;
	sub_8222CED0(ctx, base);
	// 83282868: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328286C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282870: 38860330  addi r4, r6, 0x330
	ctx.r[4].s64 = ctx.r[6].s64 + 816;
	// 83282874: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83282878: 4AFAA659  bl 0x8222ced0
	ctx.lr = 0x8328287C;
	sub_8222CED0(ctx, base);
	// 8328287C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83282880: 386512D8  addi r3, r5, 0x12d8
	ctx.r[3].s64 = ctx.r[5].s64 + 4824;
	// 83282884: 4BA2769D  bl 0x82ca9f20
	ctx.lr = 0x83282888;
	sub_82CA9F20(ctx, base);
	// 83282888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328288C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282894: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83282898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832828A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832828A0 size=56
    let mut pc: u32 = 0x832828A0;
    'dispatch: loop {
        match pc {
            0x832828A0 => {
    //   block [0x832828A0..0x832828D8)
	// 832828A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832828A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832828A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832828AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832828B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832828B4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832828B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832828BC: 4AF7149D  bl 0x821f3d58
	ctx.lr = 0x832828C0;
	sub_821F3D58(ctx, base);
	// 832828C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832828C4: 906AE1F4  stw r3, -0x1e0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7692 as u32), ctx.r[3].u32 ) };
	// 832828C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832828CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832828D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832828D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832828D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832828D8 size=56
    let mut pc: u32 = 0x832828D8;
    'dispatch: loop {
        match pc {
            0x832828D8 => {
    //   block [0x832828D8..0x83282910)
	// 832828D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832828DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832828E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832828E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832828E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832828EC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832828F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832828F4: 4AF71465  bl 0x821f3d58
	ctx.lr = 0x832828F8;
	sub_821F3D58(ctx, base);
	// 832828F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832828FC: 906AE1F8  stw r3, -0x1e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7688 as u32), ctx.r[3].u32 ) };
	// 83282900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328290C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282910 size=56
    let mut pc: u32 = 0x83282910;
    'dispatch: loop {
        match pc {
            0x83282910 => {
    //   block [0x83282910..0x83282948)
	// 83282910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328291C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282924: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83282928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328292C: 4AF7142D  bl 0x821f3d58
	ctx.lr = 0x83282930;
	sub_821F3D58(ctx, base);
	// 83282930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282934: 906AE1FC  stw r3, -0x1e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7684 as u32), ctx.r[3].u32 ) };
	// 83282938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328293C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282948 size=56
    let mut pc: u32 = 0x83282948;
    'dispatch: loop {
        match pc {
            0x83282948 => {
    //   block [0x83282948..0x83282980)
	// 83282948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328295C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83282960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282964: 4AF713F5  bl 0x821f3d58
	ctx.lr = 0x83282968;
	sub_821F3D58(ctx, base);
	// 83282968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328296C: 906AE200  stw r3, -0x1e00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7680 as u32), ctx.r[3].u32 ) };
	// 83282970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328297C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282980 size=56
    let mut pc: u32 = 0x83282980;
    'dispatch: loop {
        match pc {
            0x83282980 => {
    //   block [0x83282980..0x832829B8)
	// 83282980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328298C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282994: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83282998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328299C: 4AF713BD  bl 0x821f3d58
	ctx.lr = 0x832829A0;
	sub_821F3D58(ctx, base);
	// 832829A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832829A4: 906AE204  stw r3, -0x1dfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7676 as u32), ctx.r[3].u32 ) };
	// 832829A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832829AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832829B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832829B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832829B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832829B8 size=56
    let mut pc: u32 = 0x832829B8;
    'dispatch: loop {
        match pc {
            0x832829B8 => {
    //   block [0x832829B8..0x832829F0)
	// 832829B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832829BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832829C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832829C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832829C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832829CC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832829D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832829D4: 4AF71385  bl 0x821f3d58
	ctx.lr = 0x832829D8;
	sub_821F3D58(ctx, base);
	// 832829D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832829DC: 906AE208  stw r3, -0x1df8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7672 as u32), ctx.r[3].u32 ) };
	// 832829E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832829E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832829E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832829EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832829F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832829F0 size=56
    let mut pc: u32 = 0x832829F0;
    'dispatch: loop {
        match pc {
            0x832829F0 => {
    //   block [0x832829F0..0x83282A28)
	// 832829F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832829F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832829F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832829FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282A00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282A04: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83282A08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282A0C: 4AF7134D  bl 0x821f3d58
	ctx.lr = 0x83282A10;
	sub_821F3D58(ctx, base);
	// 83282A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282A14: 906AE20C  stw r3, -0x1df4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7668 as u32), ctx.r[3].u32 ) };
	// 83282A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282A28 size=56
    let mut pc: u32 = 0x83282A28;
    'dispatch: loop {
        match pc {
            0x83282A28 => {
    //   block [0x83282A28..0x83282A60)
	// 83282A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282A34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282A38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282A3C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83282A40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282A44: 4AF71315  bl 0x821f3d58
	ctx.lr = 0x83282A48;
	sub_821F3D58(ctx, base);
	// 83282A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282A4C: 906AE210  stw r3, -0x1df0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7664 as u32), ctx.r[3].u32 ) };
	// 83282A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282A60 size=56
    let mut pc: u32 = 0x83282A60;
    'dispatch: loop {
        match pc {
            0x83282A60 => {
    //   block [0x83282A60..0x83282A98)
	// 83282A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282A6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282A70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282A74: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83282A78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282A7C: 4AF712DD  bl 0x821f3d58
	ctx.lr = 0x83282A80;
	sub_821F3D58(ctx, base);
	// 83282A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282A84: 906AE214  stw r3, -0x1dec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7660 as u32), ctx.r[3].u32 ) };
	// 83282A88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282A98 size=56
    let mut pc: u32 = 0x83282A98;
    'dispatch: loop {
        match pc {
            0x83282A98 => {
    //   block [0x83282A98..0x83282AD0)
	// 83282A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282AA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282AA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282AAC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83282AB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282AB4: 4AF712A5  bl 0x821f3d58
	ctx.lr = 0x83282AB8;
	sub_821F3D58(ctx, base);
	// 83282AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282ABC: 906AE218  stw r3, -0x1de8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7656 as u32), ctx.r[3].u32 ) };
	// 83282AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282AD0 size=56
    let mut pc: u32 = 0x83282AD0;
    'dispatch: loop {
        match pc {
            0x83282AD0 => {
    //   block [0x83282AD0..0x83282B08)
	// 83282AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282ADC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282AE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282AE4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83282AE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282AEC: 4AF7126D  bl 0x821f3d58
	ctx.lr = 0x83282AF0;
	sub_821F3D58(ctx, base);
	// 83282AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282AF4: 906AE21C  stw r3, -0x1de4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7652 as u32), ctx.r[3].u32 ) };
	// 83282AF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282B08 size=56
    let mut pc: u32 = 0x83282B08;
    'dispatch: loop {
        match pc {
            0x83282B08 => {
    //   block [0x83282B08..0x83282B40)
	// 83282B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282B10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282B14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282B18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282B1C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83282B20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282B24: 4AF71235  bl 0x821f3d58
	ctx.lr = 0x83282B28;
	sub_821F3D58(ctx, base);
	// 83282B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282B2C: 906AE220  stw r3, -0x1de0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7648 as u32), ctx.r[3].u32 ) };
	// 83282B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282B40 size=56
    let mut pc: u32 = 0x83282B40;
    'dispatch: loop {
        match pc {
            0x83282B40 => {
    //   block [0x83282B40..0x83282B78)
	// 83282B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282B4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282B50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282B54: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83282B58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282B5C: 4AF711FD  bl 0x821f3d58
	ctx.lr = 0x83282B60;
	sub_821F3D58(ctx, base);
	// 83282B60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282B64: 906AE224  stw r3, -0x1ddc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7644 as u32), ctx.r[3].u32 ) };
	// 83282B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282B78 size=56
    let mut pc: u32 = 0x83282B78;
    'dispatch: loop {
        match pc {
            0x83282B78 => {
    //   block [0x83282B78..0x83282BB0)
	// 83282B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282B88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282B8C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83282B90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282B94: 4AF711C5  bl 0x821f3d58
	ctx.lr = 0x83282B98;
	sub_821F3D58(ctx, base);
	// 83282B98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282B9C: 906AE228  stw r3, -0x1dd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7640 as u32), ctx.r[3].u32 ) };
	// 83282BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282BB0 size=56
    let mut pc: u32 = 0x83282BB0;
    'dispatch: loop {
        match pc {
            0x83282BB0 => {
    //   block [0x83282BB0..0x83282BE8)
	// 83282BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282BC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282BC4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83282BC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282BCC: 4AF7118D  bl 0x821f3d58
	ctx.lr = 0x83282BD0;
	sub_821F3D58(ctx, base);
	// 83282BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282BD4: 906AE22C  stw r3, -0x1dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7636 as u32), ctx.r[3].u32 ) };
	// 83282BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282BE8 size=56
    let mut pc: u32 = 0x83282BE8;
    'dispatch: loop {
        match pc {
            0x83282BE8 => {
    //   block [0x83282BE8..0x83282C20)
	// 83282BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282BF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282BFC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83282C00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282C04: 4AF71155  bl 0x821f3d58
	ctx.lr = 0x83282C08;
	sub_821F3D58(ctx, base);
	// 83282C08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282C0C: 906AE230  stw r3, -0x1dd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7632 as u32), ctx.r[3].u32 ) };
	// 83282C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282C20 size=56
    let mut pc: u32 = 0x83282C20;
    'dispatch: loop {
        match pc {
            0x83282C20 => {
    //   block [0x83282C20..0x83282C58)
	// 83282C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282C30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282C34: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83282C38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282C3C: 4AF7111D  bl 0x821f3d58
	ctx.lr = 0x83282C40;
	sub_821F3D58(ctx, base);
	// 83282C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282C44: 906AE234  stw r3, -0x1dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7628 as u32), ctx.r[3].u32 ) };
	// 83282C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282C58 size=56
    let mut pc: u32 = 0x83282C58;
    'dispatch: loop {
        match pc {
            0x83282C58 => {
    //   block [0x83282C58..0x83282C90)
	// 83282C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282C64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282C68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282C6C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83282C70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282C74: 4AF710E5  bl 0x821f3d58
	ctx.lr = 0x83282C78;
	sub_821F3D58(ctx, base);
	// 83282C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282C7C: 906AE238  stw r3, -0x1dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7624 as u32), ctx.r[3].u32 ) };
	// 83282C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282C90 size=56
    let mut pc: u32 = 0x83282C90;
    'dispatch: loop {
        match pc {
            0x83282C90 => {
    //   block [0x83282C90..0x83282CC8)
	// 83282C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282C9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282CA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282CA4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83282CA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282CAC: 4AF710AD  bl 0x821f3d58
	ctx.lr = 0x83282CB0;
	sub_821F3D58(ctx, base);
	// 83282CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282CB4: 906AE23C  stw r3, -0x1dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7620 as u32), ctx.r[3].u32 ) };
	// 83282CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282CC8 size=56
    let mut pc: u32 = 0x83282CC8;
    'dispatch: loop {
        match pc {
            0x83282CC8 => {
    //   block [0x83282CC8..0x83282D00)
	// 83282CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282CD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282CD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282CDC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83282CE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282CE4: 4AF71075  bl 0x821f3d58
	ctx.lr = 0x83282CE8;
	sub_821F3D58(ctx, base);
	// 83282CE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282CEC: 906AE240  stw r3, -0x1dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7616 as u32), ctx.r[3].u32 ) };
	// 83282CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282D00 size=56
    let mut pc: u32 = 0x83282D00;
    'dispatch: loop {
        match pc {
            0x83282D00 => {
    //   block [0x83282D00..0x83282D38)
	// 83282D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282D0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282D14: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83282D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282D1C: 4AF7103D  bl 0x821f3d58
	ctx.lr = 0x83282D20;
	sub_821F3D58(ctx, base);
	// 83282D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282D24: 906AE244  stw r3, -0x1dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7612 as u32), ctx.r[3].u32 ) };
	// 83282D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282D38 size=64
    let mut pc: u32 = 0x83282D38;
    'dispatch: loop {
        match pc {
            0x83282D38 => {
    //   block [0x83282D38..0x83282D78)
	// 83282D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282D44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83282D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282D4C: 388B0410  addi r4, r11, 0x410
	ctx.r[4].s64 = ctx.r[11].s64 + 1040;
	// 83282D50: 386AE248  addi r3, r10, -0x1db8
	ctx.r[3].s64 = ctx.r[10].s64 + -7608;
	// 83282D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83282D58: 4AFAA179  bl 0x8222ced0
	ctx.lr = 0x83282D5C;
	sub_8222CED0(ctx, base);
	// 83282D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83282D60: 38691340  addi r3, r9, 0x1340
	ctx.r[3].s64 = ctx.r[9].s64 + 4928;
	// 83282D64: 4BA271BD  bl 0x82ca9f20
	ctx.lr = 0x83282D68;
	sub_82CA9F20(ctx, base);
	// 83282D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282D78 size=56
    let mut pc: u32 = 0x83282D78;
    'dispatch: loop {
        match pc {
            0x83282D78 => {
    //   block [0x83282D78..0x83282DB0)
	// 83282D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282D88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282D8C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83282D90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282D94: 4AF70FC5  bl 0x821f3d58
	ctx.lr = 0x83282D98;
	sub_821F3D58(ctx, base);
	// 83282D98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282D9C: 906AE24C  stw r3, -0x1db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7604 as u32), ctx.r[3].u32 ) };
	// 83282DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282DB0 size=56
    let mut pc: u32 = 0x83282DB0;
    'dispatch: loop {
        match pc {
            0x83282DB0 => {
    //   block [0x83282DB0..0x83282DE8)
	// 83282DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282DC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282DC4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83282DC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282DCC: 4AF70F8D  bl 0x821f3d58
	ctx.lr = 0x83282DD0;
	sub_821F3D58(ctx, base);
	// 83282DD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282DD4: 906AE250  stw r3, -0x1db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7600 as u32), ctx.r[3].u32 ) };
	// 83282DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282DE8 size=56
    let mut pc: u32 = 0x83282DE8;
    'dispatch: loop {
        match pc {
            0x83282DE8 => {
    //   block [0x83282DE8..0x83282E20)
	// 83282DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282DF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282DF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282DFC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83282E00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282E04: 4AF70F55  bl 0x821f3d58
	ctx.lr = 0x83282E08;
	sub_821F3D58(ctx, base);
	// 83282E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282E0C: 906AE254  stw r3, -0x1dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7596 as u32), ctx.r[3].u32 ) };
	// 83282E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282E20 size=56
    let mut pc: u32 = 0x83282E20;
    'dispatch: loop {
        match pc {
            0x83282E20 => {
    //   block [0x83282E20..0x83282E58)
	// 83282E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282E2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282E30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282E34: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83282E38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282E3C: 4AF70F1D  bl 0x821f3d58
	ctx.lr = 0x83282E40;
	sub_821F3D58(ctx, base);
	// 83282E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282E44: 906AE258  stw r3, -0x1da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7592 as u32), ctx.r[3].u32 ) };
	// 83282E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282E58 size=56
    let mut pc: u32 = 0x83282E58;
    'dispatch: loop {
        match pc {
            0x83282E58 => {
    //   block [0x83282E58..0x83282E90)
	// 83282E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282E64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282E68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282E6C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83282E70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282E74: 4AF70EE5  bl 0x821f3d58
	ctx.lr = 0x83282E78;
	sub_821F3D58(ctx, base);
	// 83282E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282E7C: 906AE25C  stw r3, -0x1da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7588 as u32), ctx.r[3].u32 ) };
	// 83282E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282E90 size=56
    let mut pc: u32 = 0x83282E90;
    'dispatch: loop {
        match pc {
            0x83282E90 => {
    //   block [0x83282E90..0x83282EC8)
	// 83282E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282E9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282EA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282EA4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83282EA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282EAC: 4AF70EAD  bl 0x821f3d58
	ctx.lr = 0x83282EB0;
	sub_821F3D58(ctx, base);
	// 83282EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282EB4: 906AE260  stw r3, -0x1da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7584 as u32), ctx.r[3].u32 ) };
	// 83282EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282EC8 size=56
    let mut pc: u32 = 0x83282EC8;
    'dispatch: loop {
        match pc {
            0x83282EC8 => {
    //   block [0x83282EC8..0x83282F00)
	// 83282EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282ED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282ED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282EDC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83282EE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282EE4: 4AF70E75  bl 0x821f3d58
	ctx.lr = 0x83282EE8;
	sub_821F3D58(ctx, base);
	// 83282EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282EEC: 906AE264  stw r3, -0x1d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7580 as u32), ctx.r[3].u32 ) };
	// 83282EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282F00 size=56
    let mut pc: u32 = 0x83282F00;
    'dispatch: loop {
        match pc {
            0x83282F00 => {
    //   block [0x83282F00..0x83282F38)
	// 83282F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282F0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282F10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282F14: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83282F18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282F1C: 4AF70E3D  bl 0x821f3d58
	ctx.lr = 0x83282F20;
	sub_821F3D58(ctx, base);
	// 83282F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282F24: 906AE268  stw r3, -0x1d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7576 as u32), ctx.r[3].u32 ) };
	// 83282F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282F38 size=56
    let mut pc: u32 = 0x83282F38;
    'dispatch: loop {
        match pc {
            0x83282F38 => {
    //   block [0x83282F38..0x83282F70)
	// 83282F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282F44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282F48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282F4C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83282F50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282F54: 4AF70E05  bl 0x821f3d58
	ctx.lr = 0x83282F58;
	sub_821F3D58(ctx, base);
	// 83282F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282F5C: 906AE26C  stw r3, -0x1d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7572 as u32), ctx.r[3].u32 ) };
	// 83282F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282F70 size=56
    let mut pc: u32 = 0x83282F70;
    'dispatch: loop {
        match pc {
            0x83282F70 => {
    //   block [0x83282F70..0x83282FA8)
	// 83282F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282F80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282F84: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83282F88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282F8C: 4AF70DCD  bl 0x821f3d58
	ctx.lr = 0x83282F90;
	sub_821F3D58(ctx, base);
	// 83282F90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282F94: 906AE270  stw r3, -0x1d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7568 as u32), ctx.r[3].u32 ) };
	// 83282F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282FA8 size=56
    let mut pc: u32 = 0x83282FA8;
    'dispatch: loop {
        match pc {
            0x83282FA8 => {
    //   block [0x83282FA8..0x83282FE0)
	// 83282FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282FB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282FB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282FBC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83282FC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282FC4: 4AF70D95  bl 0x821f3d58
	ctx.lr = 0x83282FC8;
	sub_821F3D58(ctx, base);
	// 83282FC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83282FCC: 906AE274  stw r3, -0x1d8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7564 as u32), ctx.r[3].u32 ) };
	// 83282FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83282FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83282FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83282FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83282FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83282FE0 size=56
    let mut pc: u32 = 0x83282FE0;
    'dispatch: loop {
        match pc {
            0x83282FE0 => {
    //   block [0x83282FE0..0x83283018)
	// 83282FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83282FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83282FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83282FEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83282FF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83282FF4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83282FF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83282FFC: 4AF70D5D  bl 0x821f3d58
	ctx.lr = 0x83283000;
	sub_821F3D58(ctx, base);
	// 83283000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283004: 906AE278  stw r3, -0x1d88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7560 as u32), ctx.r[3].u32 ) };
	// 83283008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328300C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283018 size=56
    let mut pc: u32 = 0x83283018;
    'dispatch: loop {
        match pc {
            0x83283018 => {
    //   block [0x83283018..0x83283050)
	// 83283018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283024: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283028: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328302C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83283030: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283034: 4AF70D25  bl 0x821f3d58
	ctx.lr = 0x83283038;
	sub_821F3D58(ctx, base);
	// 83283038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328303C: 906AE27C  stw r3, -0x1d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7556 as u32), ctx.r[3].u32 ) };
	// 83283040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328304C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283050 size=56
    let mut pc: u32 = 0x83283050;
    'dispatch: loop {
        match pc {
            0x83283050 => {
    //   block [0x83283050..0x83283088)
	// 83283050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328305C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283060: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283064: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83283068: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328306C: 4AF70CED  bl 0x821f3d58
	ctx.lr = 0x83283070;
	sub_821F3D58(ctx, base);
	// 83283070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283074: 906AE280  stw r3, -0x1d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7552 as u32), ctx.r[3].u32 ) };
	// 83283078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328307C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283088 size=56
    let mut pc: u32 = 0x83283088;
    'dispatch: loop {
        match pc {
            0x83283088 => {
    //   block [0x83283088..0x832830C0)
	// 83283088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283094: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283098: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328309C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832830A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832830A4: 4AF70CB5  bl 0x821f3d58
	ctx.lr = 0x832830A8;
	sub_821F3D58(ctx, base);
	// 832830A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832830AC: 906AE284  stw r3, -0x1d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7548 as u32), ctx.r[3].u32 ) };
	// 832830B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832830B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832830B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832830BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832830C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832830C0 size=56
    let mut pc: u32 = 0x832830C0;
    'dispatch: loop {
        match pc {
            0x832830C0 => {
    //   block [0x832830C0..0x832830F8)
	// 832830C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832830C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832830C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832830CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832830D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832830D4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832830D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832830DC: 4AF70C7D  bl 0x821f3d58
	ctx.lr = 0x832830E0;
	sub_821F3D58(ctx, base);
	// 832830E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832830E4: 906AE288  stw r3, -0x1d78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7544 as u32), ctx.r[3].u32 ) };
	// 832830E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832830EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832830F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832830F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832830F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832830F8 size=56
    let mut pc: u32 = 0x832830F8;
    'dispatch: loop {
        match pc {
            0x832830F8 => {
    //   block [0x832830F8..0x83283130)
	// 832830F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832830FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283108: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328310C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83283110: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283114: 4AF70C45  bl 0x821f3d58
	ctx.lr = 0x83283118;
	sub_821F3D58(ctx, base);
	// 83283118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328311C: 906AE28C  stw r3, -0x1d74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7540 as u32), ctx.r[3].u32 ) };
	// 83283120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328312C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283130 size=56
    let mut pc: u32 = 0x83283130;
    'dispatch: loop {
        match pc {
            0x83283130 => {
    //   block [0x83283130..0x83283168)
	// 83283130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328313C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283140: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283144: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83283148: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328314C: 4AF70C0D  bl 0x821f3d58
	ctx.lr = 0x83283150;
	sub_821F3D58(ctx, base);
	// 83283150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283154: 906AE290  stw r3, -0x1d70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7536 as u32), ctx.r[3].u32 ) };
	// 83283158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328315C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283168 size=56
    let mut pc: u32 = 0x83283168;
    'dispatch: loop {
        match pc {
            0x83283168 => {
    //   block [0x83283168..0x832831A0)
	// 83283168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283174: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283178: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328317C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83283180: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283184: 4AF70BD5  bl 0x821f3d58
	ctx.lr = 0x83283188;
	sub_821F3D58(ctx, base);
	// 83283188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328318C: 906AE294  stw r3, -0x1d6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7532 as u32), ctx.r[3].u32 ) };
	// 83283190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832831A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832831A0 size=56
    let mut pc: u32 = 0x832831A0;
    'dispatch: loop {
        match pc {
            0x832831A0 => {
    //   block [0x832831A0..0x832831D8)
	// 832831A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832831A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832831A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832831AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832831B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832831B4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832831B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832831BC: 4AF70B9D  bl 0x821f3d58
	ctx.lr = 0x832831C0;
	sub_821F3D58(ctx, base);
	// 832831C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832831C4: 906AE298  stw r3, -0x1d68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7528 as u32), ctx.r[3].u32 ) };
	// 832831C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832831CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832831D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832831D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832831D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832831D8 size=56
    let mut pc: u32 = 0x832831D8;
    'dispatch: loop {
        match pc {
            0x832831D8 => {
    //   block [0x832831D8..0x83283210)
	// 832831D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832831DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832831E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832831E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832831E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832831EC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832831F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832831F4: 4AF70B65  bl 0x821f3d58
	ctx.lr = 0x832831F8;
	sub_821F3D58(ctx, base);
	// 832831F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832831FC: 906AE29C  stw r3, -0x1d64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7524 as u32), ctx.r[3].u32 ) };
	// 83283200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283210 size=64
    let mut pc: u32 = 0x83283210;
    'dispatch: loop {
        match pc {
            0x83283210 => {
    //   block [0x83283210..0x83283250)
	// 83283210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328321C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283224: 388B0464  addi r4, r11, 0x464
	ctx.r[4].s64 = ctx.r[11].s64 + 1124;
	// 83283228: 386AE2A0  addi r3, r10, -0x1d60
	ctx.r[3].s64 = ctx.r[10].s64 + -7520;
	// 8328322C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283230: 4AFA9CA1  bl 0x8222ced0
	ctx.lr = 0x83283234;
	sub_8222CED0(ctx, base);
	// 83283234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283238: 38691350  addi r3, r9, 0x1350
	ctx.r[3].s64 = ctx.r[9].s64 + 4944;
	// 8328323C: 4BA26CE5  bl 0x82ca9f20
	ctx.lr = 0x83283240;
	sub_82CA9F20(ctx, base);
	// 83283240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328324C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283250 size=56
    let mut pc: u32 = 0x83283250;
    'dispatch: loop {
        match pc {
            0x83283250 => {
    //   block [0x83283250..0x83283288)
	// 83283250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328325C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283260: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283264: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83283268: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328326C: 4AF70AED  bl 0x821f3d58
	ctx.lr = 0x83283270;
	sub_821F3D58(ctx, base);
	// 83283270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283274: 906AE2A4  stw r3, -0x1d5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7516 as u32), ctx.r[3].u32 ) };
	// 83283278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328327C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283288 size=56
    let mut pc: u32 = 0x83283288;
    'dispatch: loop {
        match pc {
            0x83283288 => {
    //   block [0x83283288..0x832832C0)
	// 83283288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328328C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283298: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328329C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832832A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832832A4: 4AF70AB5  bl 0x821f3d58
	ctx.lr = 0x832832A8;
	sub_821F3D58(ctx, base);
	// 832832A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832832AC: 906AE2A8  stw r3, -0x1d58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7512 as u32), ctx.r[3].u32 ) };
	// 832832B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832832B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832832B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832832BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832832C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832832C0 size=56
    let mut pc: u32 = 0x832832C0;
    'dispatch: loop {
        match pc {
            0x832832C0 => {
    //   block [0x832832C0..0x832832F8)
	// 832832C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832832C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832832C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832832CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832832D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832832D4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832832D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832832DC: 4AF70A7D  bl 0x821f3d58
	ctx.lr = 0x832832E0;
	sub_821F3D58(ctx, base);
	// 832832E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832832E4: 906AE2AC  stw r3, -0x1d54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7508 as u32), ctx.r[3].u32 ) };
	// 832832E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832832EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832832F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832832F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832832F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832832F8 size=56
    let mut pc: u32 = 0x832832F8;
    'dispatch: loop {
        match pc {
            0x832832F8 => {
    //   block [0x832832F8..0x83283330)
	// 832832F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832832FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283308: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328330C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83283310: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283314: 4AF70A45  bl 0x821f3d58
	ctx.lr = 0x83283318;
	sub_821F3D58(ctx, base);
	// 83283318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328331C: 906AE2B0  stw r3, -0x1d50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7504 as u32), ctx.r[3].u32 ) };
	// 83283320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328332C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283330 size=56
    let mut pc: u32 = 0x83283330;
    'dispatch: loop {
        match pc {
            0x83283330 => {
    //   block [0x83283330..0x83283368)
	// 83283330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328333C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283340: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283344: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83283348: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328334C: 4AF70A0D  bl 0x821f3d58
	ctx.lr = 0x83283350;
	sub_821F3D58(ctx, base);
	// 83283350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283354: 906AE2B4  stw r3, -0x1d4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7500 as u32), ctx.r[3].u32 ) };
	// 83283358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283368 size=56
    let mut pc: u32 = 0x83283368;
    'dispatch: loop {
        match pc {
            0x83283368 => {
    //   block [0x83283368..0x832833A0)
	// 83283368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328336C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283378: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328337C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83283380: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283384: 4AF709D5  bl 0x821f3d58
	ctx.lr = 0x83283388;
	sub_821F3D58(ctx, base);
	// 83283388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328338C: 906AE2B8  stw r3, -0x1d48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7496 as u32), ctx.r[3].u32 ) };
	// 83283390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328339C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832833A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832833A0 size=56
    let mut pc: u32 = 0x832833A0;
    'dispatch: loop {
        match pc {
            0x832833A0 => {
    //   block [0x832833A0..0x832833D8)
	// 832833A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832833A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832833A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832833AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832833B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832833B4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832833B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832833BC: 4AF7099D  bl 0x821f3d58
	ctx.lr = 0x832833C0;
	sub_821F3D58(ctx, base);
	// 832833C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832833C4: 906AE2BC  stw r3, -0x1d44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7492 as u32), ctx.r[3].u32 ) };
	// 832833C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832833CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832833D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832833D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832833D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832833D8 size=56
    let mut pc: u32 = 0x832833D8;
    'dispatch: loop {
        match pc {
            0x832833D8 => {
    //   block [0x832833D8..0x83283410)
	// 832833D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832833DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832833E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832833E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832833E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832833EC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832833F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832833F4: 4AF70965  bl 0x821f3d58
	ctx.lr = 0x832833F8;
	sub_821F3D58(ctx, base);
	// 832833F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832833FC: 906AE2C0  stw r3, -0x1d40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7488 as u32), ctx.r[3].u32 ) };
	// 83283400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328340C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283410 size=56
    let mut pc: u32 = 0x83283410;
    'dispatch: loop {
        match pc {
            0x83283410 => {
    //   block [0x83283410..0x83283448)
	// 83283410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328341C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283420: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283424: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83283428: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328342C: 4AF7092D  bl 0x821f3d58
	ctx.lr = 0x83283430;
	sub_821F3D58(ctx, base);
	// 83283430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283434: 906AE2C4  stw r3, -0x1d3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7484 as u32), ctx.r[3].u32 ) };
	// 83283438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328343C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283448 size=56
    let mut pc: u32 = 0x83283448;
    'dispatch: loop {
        match pc {
            0x83283448 => {
    //   block [0x83283448..0x83283480)
	// 83283448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328344C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283458: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328345C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83283460: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283464: 4AF708F5  bl 0x821f3d58
	ctx.lr = 0x83283468;
	sub_821F3D58(ctx, base);
	// 83283468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328346C: 906AE2C8  stw r3, -0x1d38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7480 as u32), ctx.r[3].u32 ) };
	// 83283470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283480 size=56
    let mut pc: u32 = 0x83283480;
    'dispatch: loop {
        match pc {
            0x83283480 => {
    //   block [0x83283480..0x832834B8)
	// 83283480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328348C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283490: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283494: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83283498: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328349C: 4AF708BD  bl 0x821f3d58
	ctx.lr = 0x832834A0;
	sub_821F3D58(ctx, base);
	// 832834A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832834A4: 906AE2CC  stw r3, -0x1d34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7476 as u32), ctx.r[3].u32 ) };
	// 832834A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832834AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832834B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832834B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832834B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832834B8 size=56
    let mut pc: u32 = 0x832834B8;
    'dispatch: loop {
        match pc {
            0x832834B8 => {
    //   block [0x832834B8..0x832834F0)
	// 832834B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832834BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832834C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832834C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832834C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832834CC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832834D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832834D4: 4AF70885  bl 0x821f3d58
	ctx.lr = 0x832834D8;
	sub_821F3D58(ctx, base);
	// 832834D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832834DC: 906AE2D0  stw r3, -0x1d30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7472 as u32), ctx.r[3].u32 ) };
	// 832834E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832834E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832834E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832834EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832834F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832834F0 size=56
    let mut pc: u32 = 0x832834F0;
    'dispatch: loop {
        match pc {
            0x832834F0 => {
    //   block [0x832834F0..0x83283528)
	// 832834F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832834F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832834F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832834FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283500: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283504: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83283508: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328350C: 4AF7084D  bl 0x821f3d58
	ctx.lr = 0x83283510;
	sub_821F3D58(ctx, base);
	// 83283510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283514: 906AE2D4  stw r3, -0x1d2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7468 as u32), ctx.r[3].u32 ) };
	// 83283518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328351C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283528 size=56
    let mut pc: u32 = 0x83283528;
    'dispatch: loop {
        match pc {
            0x83283528 => {
    //   block [0x83283528..0x83283560)
	// 83283528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328352C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283534: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283538: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328353C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83283540: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283544: 4AF70815  bl 0x821f3d58
	ctx.lr = 0x83283548;
	sub_821F3D58(ctx, base);
	// 83283548: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328354C: 906AE2D8  stw r3, -0x1d28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7464 as u32), ctx.r[3].u32 ) };
	// 83283550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328355C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283560 size=56
    let mut pc: u32 = 0x83283560;
    'dispatch: loop {
        match pc {
            0x83283560 => {
    //   block [0x83283560..0x83283598)
	// 83283560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328356C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283570: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283574: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83283578: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328357C: 4AF707DD  bl 0x821f3d58
	ctx.lr = 0x83283580;
	sub_821F3D58(ctx, base);
	// 83283580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283584: 906AE2DC  stw r3, -0x1d24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7460 as u32), ctx.r[3].u32 ) };
	// 83283588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328358C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283598 size=56
    let mut pc: u32 = 0x83283598;
    'dispatch: loop {
        match pc {
            0x83283598 => {
    //   block [0x83283598..0x832835D0)
	// 83283598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832835A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832835A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832835A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832835AC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832835B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832835B4: 4AF707A5  bl 0x821f3d58
	ctx.lr = 0x832835B8;
	sub_821F3D58(ctx, base);
	// 832835B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832835BC: 906AE2E0  stw r3, -0x1d20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7456 as u32), ctx.r[3].u32 ) };
	// 832835C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832835C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832835C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832835CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832835D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832835D0 size=56
    let mut pc: u32 = 0x832835D0;
    'dispatch: loop {
        match pc {
            0x832835D0 => {
    //   block [0x832835D0..0x83283608)
	// 832835D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832835D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832835D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832835DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832835E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832835E4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832835E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832835EC: 4AF7076D  bl 0x821f3d58
	ctx.lr = 0x832835F0;
	sub_821F3D58(ctx, base);
	// 832835F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832835F4: 906AE2E4  stw r3, -0x1d1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7452 as u32), ctx.r[3].u32 ) };
	// 832835F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832835FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283608 size=56
    let mut pc: u32 = 0x83283608;
    'dispatch: loop {
        match pc {
            0x83283608 => {
    //   block [0x83283608..0x83283640)
	// 83283608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328360C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283614: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283618: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328361C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83283620: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283624: 4AF70735  bl 0x821f3d58
	ctx.lr = 0x83283628;
	sub_821F3D58(ctx, base);
	// 83283628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328362C: 906AE2E8  stw r3, -0x1d18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7448 as u32), ctx.r[3].u32 ) };
	// 83283630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328363C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283640 size=56
    let mut pc: u32 = 0x83283640;
    'dispatch: loop {
        match pc {
            0x83283640 => {
    //   block [0x83283640..0x83283678)
	// 83283640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328364C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283650: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283654: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83283658: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328365C: 4AF706FD  bl 0x821f3d58
	ctx.lr = 0x83283660;
	sub_821F3D58(ctx, base);
	// 83283660: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283664: 906AE2EC  stw r3, -0x1d14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7444 as u32), ctx.r[3].u32 ) };
	// 83283668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328366C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283678 size=56
    let mut pc: u32 = 0x83283678;
    'dispatch: loop {
        match pc {
            0x83283678 => {
    //   block [0x83283678..0x832836B0)
	// 83283678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328367C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328368C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83283690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283694: 4AF706C5  bl 0x821f3d58
	ctx.lr = 0x83283698;
	sub_821F3D58(ctx, base);
	// 83283698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328369C: 906AE2F0  stw r3, -0x1d10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7440 as u32), ctx.r[3].u32 ) };
	// 832836A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832836A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832836A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832836AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832836B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832836B0 size=56
    let mut pc: u32 = 0x832836B0;
    'dispatch: loop {
        match pc {
            0x832836B0 => {
    //   block [0x832836B0..0x832836E8)
	// 832836B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832836B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832836B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832836BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832836C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832836C4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832836C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832836CC: 4AF7068D  bl 0x821f3d58
	ctx.lr = 0x832836D0;
	sub_821F3D58(ctx, base);
	// 832836D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832836D4: 906AE2F4  stw r3, -0x1d0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7436 as u32), ctx.r[3].u32 ) };
	// 832836D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832836DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832836E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832836E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832836E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832836E8 size=64
    let mut pc: u32 = 0x832836E8;
    'dispatch: loop {
        match pc {
            0x832836E8 => {
    //   block [0x832836E8..0x83283728)
	// 832836E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832836EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832836F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832836F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832836F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832836FC: 388B04D4  addi r4, r11, 0x4d4
	ctx.r[4].s64 = ctx.r[11].s64 + 1236;
	// 83283700: 386AE2F8  addi r3, r10, -0x1d08
	ctx.r[3].s64 = ctx.r[10].s64 + -7432;
	// 83283704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283708: 4AFA97C9  bl 0x8222ced0
	ctx.lr = 0x8328370C;
	sub_8222CED0(ctx, base);
	// 8328370C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283710: 38691360  addi r3, r9, 0x1360
	ctx.r[3].s64 = ctx.r[9].s64 + 4960;
	// 83283714: 4BA2680D  bl 0x82ca9f20
	ctx.lr = 0x83283718;
	sub_82CA9F20(ctx, base);
	// 83283718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328371C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283728 size=64
    let mut pc: u32 = 0x83283728;
    'dispatch: loop {
        match pc {
            0x83283728 => {
    //   block [0x83283728..0x83283768)
	// 83283728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328372C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283734: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328373C: 388B0504  addi r4, r11, 0x504
	ctx.r[4].s64 = ctx.r[11].s64 + 1284;
	// 83283740: 386AE2FC  addi r3, r10, -0x1d04
	ctx.r[3].s64 = ctx.r[10].s64 + -7428;
	// 83283744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283748: 4AFA9789  bl 0x8222ced0
	ctx.lr = 0x8328374C;
	sub_8222CED0(ctx, base);
	// 8328374C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283750: 38691370  addi r3, r9, 0x1370
	ctx.r[3].s64 = ctx.r[9].s64 + 4976;
	// 83283754: 4BA267CD  bl 0x82ca9f20
	ctx.lr = 0x83283758;
	sub_82CA9F20(ctx, base);
	// 83283758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328375C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283768 size=64
    let mut pc: u32 = 0x83283768;
    'dispatch: loop {
        match pc {
            0x83283768 => {
    //   block [0x83283768..0x832837A8)
	// 83283768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328376C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283774: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328377C: 388B0534  addi r4, r11, 0x534
	ctx.r[4].s64 = ctx.r[11].s64 + 1332;
	// 83283780: 386AE300  addi r3, r10, -0x1d00
	ctx.r[3].s64 = ctx.r[10].s64 + -7424;
	// 83283784: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283788: 4AFA9749  bl 0x8222ced0
	ctx.lr = 0x8328378C;
	sub_8222CED0(ctx, base);
	// 8328378C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283790: 38691380  addi r3, r9, 0x1380
	ctx.r[3].s64 = ctx.r[9].s64 + 4992;
	// 83283794: 4BA2678D  bl 0x82ca9f20
	ctx.lr = 0x83283798;
	sub_82CA9F20(ctx, base);
	// 83283798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328379C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832837A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832837A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832837A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832837A8 size=64
    let mut pc: u32 = 0x832837A8;
    'dispatch: loop {
        match pc {
            0x832837A8 => {
    //   block [0x832837A8..0x832837E8)
	// 832837A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832837AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832837B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832837B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832837B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832837BC: 388B0564  addi r4, r11, 0x564
	ctx.r[4].s64 = ctx.r[11].s64 + 1380;
	// 832837C0: 386AE304  addi r3, r10, -0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7420;
	// 832837C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832837C8: 4AFA9709  bl 0x8222ced0
	ctx.lr = 0x832837CC;
	sub_8222CED0(ctx, base);
	// 832837CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832837D0: 38691390  addi r3, r9, 0x1390
	ctx.r[3].s64 = ctx.r[9].s64 + 5008;
	// 832837D4: 4BA2674D  bl 0x82ca9f20
	ctx.lr = 0x832837D8;
	sub_82CA9F20(ctx, base);
	// 832837D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832837DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832837E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832837E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832837E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832837E8 size=64
    let mut pc: u32 = 0x832837E8;
    'dispatch: loop {
        match pc {
            0x832837E8 => {
    //   block [0x832837E8..0x83283828)
	// 832837E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832837EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832837F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832837F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832837F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832837FC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83283800: 386AE308  addi r3, r10, -0x1cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -7416;
	// 83283804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283808: 4AFA96C9  bl 0x8222ced0
	ctx.lr = 0x8328380C;
	sub_8222CED0(ctx, base);
	// 8328380C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283810: 386913A0  addi r3, r9, 0x13a0
	ctx.r[3].s64 = ctx.r[9].s64 + 5024;
	// 83283814: 4BA2670D  bl 0x82ca9f20
	ctx.lr = 0x83283818;
	sub_82CA9F20(ctx, base);
	// 83283818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328381C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283828 size=144
    let mut pc: u32 = 0x83283828;
    'dispatch: loop {
        match pc {
            0x83283828 => {
    //   block [0x83283828..0x8328384C)
	// 83283828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283834: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83283838: 4AF9BA21  bl 0x8221f258
	ctx.lr = 0x8328383C;
	sub_8221F258(ctx, base);
	// 8328383C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83283840: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83283844: 419A0008  beq cr6, 0x8328384c
	if ctx.cr[6].eq {
	pc = 0x8328384C; continue 'dispatch;
	}
	// 83283848: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8328384C; continue 'dispatch;
            }
            0x8328384C => {
    //   block [0x8328384C..0x83283858)
	// 8328384C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83283850: 41820008  beq 0x83283858
	if ctx.cr[0].eq {
	pc = 0x83283858; continue 'dispatch;
	}
	// 83283854: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83283858; continue 'dispatch;
            }
            0x83283858 => {
    //   block [0x83283858..0x83283864)
	// 83283858: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8328385C: 41820008  beq 0x83283864
	if ctx.cr[0].eq {
	pc = 0x83283864; continue 'dispatch;
	}
	// 83283860: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83283864; continue 'dispatch;
            }
            0x83283864 => {
    //   block [0x83283864..0x832838B8)
	// 83283864: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83283868: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8328386C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83283870: 3909E30C  addi r8, r9, -0x1cf4
	ctx.r[8].s64 = ctx.r[9].s64 + -7412;
	// 83283874: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83283878: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8328387C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83283880: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83283884: 386713B0  addi r3, r7, 0x13b0
	ctx.r[3].s64 = ctx.r[7].s64 + 5040;
	// 83283888: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328388C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83283890: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83283894: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83283898: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8328389C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832838A0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832838A4: 4BA2667D  bl 0x82ca9f20
	ctx.lr = 0x832838A8;
	sub_82CA9F20(ctx, base);
	// 832838A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832838AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832838B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832838B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832838B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832838B8 size=96
    let mut pc: u32 = 0x832838B8;
    'dispatch: loop {
        match pc {
            0x832838B8 => {
    //   block [0x832838B8..0x83283918)
	// 832838B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832838BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832838C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832838C4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832838C8: C82B0F30  lfd f1, 0xf30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(3888 as u32) ) };
	// 832838CC: 4AFB65E5  bl 0x82239eb0
	ctx.lr = 0x832838D0;
	sub_82239EB0(ctx, base);
	// 832838D0: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 832838D4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832838D8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832838DC: 390A9490  addi r8, r10, -0x6b70
	ctx.r[8].s64 = ctx.r[10].s64 + -27504;
	// 832838E0: 38E9E318  addi r7, r9, -0x1ce8
	ctx.r[7].s64 = ctx.r[9].s64 + -7400;
	// 832838E4: C18A9490  lfs f12, -0x6b70(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832838E8: D009E318  stfs f0, -0x1ce8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7400 as u32), tmp.u32 ) };
	// 832838EC: C0081FF0  lfs f0, 0x1ff0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832838F0: C1A8FFF4  lfs f13, -0xc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832838F4: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832838F8: D1A70008  stfs f13, 8(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832838FC: D1A7000C  stfs f13, 0xc(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 83283900: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83283904: D1870014  stfs f12, 0x14(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83283908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328390C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83283918 size=104
    let mut pc: u32 = 0x83283918;
    'dispatch: loop {
        match pc {
            0x83283918 => {
    //   block [0x83283918..0x83283980)
	// 83283918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328391C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283924: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83283928: C82B1938  lfd f1, 0x1938(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(6456 as u32) ) };
	// 8328392C: 4AFB6585  bl 0x82239eb0
	ctx.lr = 0x83283930;
	sub_82239EB0(ctx, base);
	// 83283930: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83283934: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83283938: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8328393C: 390ABE04  addi r8, r10, -0x41fc
	ctx.r[8].s64 = ctx.r[10].s64 + -16892;
	// 83283940: 38E9E330  addi r7, r9, -0x1cd0
	ctx.r[7].s64 = ctx.r[9].s64 + -7376;
	// 83283944: C14ABE04  lfs f10, -0x41fc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16892 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83283948: D009E330  stfs f0, -0x1cd0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7376 as u32), tmp.u32 ) };
	// 8328394C: C008F67C  lfs f0, -0x984(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2436 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83283950: C1A82094  lfs f13, 0x2094(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83283954: C18822DC  lfs f12, 0x22dc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8924 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83283958: C168D68C  lfs f11, -0x2974(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10612 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8328395C: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83283960: D1A70008  stfs f13, 8(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 83283964: D187000C  stfs f12, 0xc(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 83283968: D1670010  stfs f11, 0x10(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8328396C: D1470014  stfs f10, 0x14(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83283970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328397C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83283980 size=104
    let mut pc: u32 = 0x83283980;
    'dispatch: loop {
        match pc {
            0x83283980 => {
    //   block [0x83283980..0x832839E8)
	// 83283980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328398C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83283990: C82B1A30  lfd f1, 0x1a30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(6704 as u32) ) };
	// 83283994: 4AFB651D  bl 0x82239eb0
	ctx.lr = 0x83283998;
	sub_82239EB0(ctx, base);
	// 83283998: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8328399C: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832839A0: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832839A4: 390ABE04  addi r8, r10, -0x41fc
	ctx.r[8].s64 = ctx.r[10].s64 + -16892;
	// 832839A8: 38E9E348  addi r7, r9, -0x1cb8
	ctx.r[7].s64 = ctx.r[9].s64 + -7352;
	// 832839AC: C14ABE04  lfs f10, -0x41fc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16892 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 832839B0: D009E348  stfs f0, -0x1cb8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7352 as u32), tmp.u32 ) };
	// 832839B4: C008F67C  lfs f0, -0x984(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2436 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832839B8: C1A82094  lfs f13, 0x2094(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832839BC: C18822DC  lfs f12, 0x22dc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8924 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832839C0: C168D68C  lfs f11, -0x2974(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10612 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 832839C4: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 832839C8: D1A70008  stfs f13, 8(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 832839CC: D187000C  stfs f12, 0xc(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 832839D0: D1670010  stfs f11, 0x10(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 832839D4: D1470014  stfs f10, 0x14(r7)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 832839D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832839DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832839E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832839E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832839E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832839E8 size=108
    let mut pc: u32 = 0x832839E8;
    'dispatch: loop {
        match pc {
            0x832839E8 => {
    //   block [0x832839E8..0x83283A54)
	// 832839E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832839EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832839F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832839F4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832839F8: C82B1A38  lfd f1, 0x1a38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(6712 as u32) ) };
	// 832839FC: 4AFB64B5  bl 0x82239eb0
	ctx.lr = 0x83283A00;
	sub_82239EB0(ctx, base);
	// 83283A00: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83283A04: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83283A08: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83283A0C: 390AB7A4  addi r8, r10, -0x485c
	ctx.r[8].s64 = ctx.r[10].s64 + -18524;
	// 83283A10: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 83283A14: 38C9E360  addi r6, r9, -0x1ca0
	ctx.r[6].s64 = ctx.r[9].s64 + -7328;
	// 83283A18: C14AB7A4  lfs f10, -0x485c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18524 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83283A1C: D009E360  stfs f0, -0x1ca0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7328 as u32), tmp.u32 ) };
	// 83283A20: C008FCDC  lfs f0, -0x324(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-804 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83283A24: C1A79A80  lfs f13, -0x6580(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-25984 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83283A28: C188DCE0  lfs f12, -0x2320(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8992 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83283A2C: C168DCEC  lfs f11, -0x2314(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8980 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83283A30: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83283A34: D1A60008  stfs f13, 8(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 83283A38: D186000C  stfs f12, 0xc(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 83283A3C: D1660010  stfs f11, 0x10(r6)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83283A40: D1460014  stfs f10, 0x14(r6)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83283A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83283A58 size=108
    let mut pc: u32 = 0x83283A58;
    'dispatch: loop {
        match pc {
            0x83283A58 => {
    //   block [0x83283A58..0x83283AC4)
	// 83283A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283A64: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83283A68: C82B1488  lfd f1, 0x1488(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(5256 as u32) ) };
	// 83283A6C: 4AFB6445  bl 0x82239eb0
	ctx.lr = 0x83283A70;
	sub_82239EB0(ctx, base);
	// 83283A70: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83283A74: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83283A78: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83283A7C: 390AB7A4  addi r8, r10, -0x485c
	ctx.r[8].s64 = ctx.r[10].s64 + -18524;
	// 83283A80: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83283A84: 38C9E378  addi r6, r9, -0x1c88
	ctx.r[6].s64 = ctx.r[9].s64 + -7304;
	// 83283A88: C14AB7A4  lfs f10, -0x485c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18524 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83283A8C: D009E378  stfs f0, -0x1c88(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7304 as u32), tmp.u32 ) };
	// 83283A90: C008DCEC  lfs f0, -0x2314(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8980 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83283A94: C1A71A40  lfs f13, 0x1a40(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(6720 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83283A98: C188FF8C  lfs f12, -0x74(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-116 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83283A9C: C168FEAC  lfs f11, -0x154(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-340 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83283AA0: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83283AA4: D1A60008  stfs f13, 8(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 83283AA8: D186000C  stfs f12, 0xc(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 83283AAC: D1660010  stfs f11, 0x10(r6)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83283AB0: D1460014  stfs f10, 0x14(r6)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83283AB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83283AC8 size=100
    let mut pc: u32 = 0x83283AC8;
    'dispatch: loop {
        match pc {
            0x83283AC8 => {
    //   block [0x83283AC8..0x83283B2C)
	// 83283AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283AD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283AD4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83283AD8: C82B1938  lfd f1, 0x1938(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(6456 as u32) ) };
	// 83283ADC: 4AFB63D5  bl 0x82239eb0
	ctx.lr = 0x83283AE0;
	sub_82239EB0(ctx, base);
	// 83283AE0: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83283AE4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83283AE8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83283AEC: 390ABE04  addi r8, r10, -0x41fc
	ctx.r[8].s64 = ctx.r[10].s64 + -16892;
	// 83283AF0: 38E9E390  addi r7, r9, -0x1c70
	ctx.r[7].s64 = ctx.r[9].s64 + -7280;
	// 83283AF4: C16ABE04  lfs f11, -0x41fc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16892 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83283AF8: D009E390  stfs f0, -0x1c70(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7280 as u32), tmp.u32 ) };
	// 83283AFC: C008D68C  lfs f0, -0x2974(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-10612 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83283B00: C1A82094  lfs f13, 0x2094(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83283B04: C18822DC  lfs f12, 0x22dc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8924 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83283B08: D0070004  stfs f0, 4(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83283B0C: D1A70008  stfs f13, 8(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 83283B10: D187000C  stfs f12, 0xc(r7)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 83283B14: D0070010  stfs f0, 0x10(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 83283B18: D1670014  stfs f11, 0x14(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 83283B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283B30 size=64
    let mut pc: u32 = 0x83283B30;
    'dispatch: loop {
        match pc {
            0x83283B30 => {
    //   block [0x83283B30..0x83283B70)
	// 83283B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283B3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83283B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283B44: 388B37C8  addi r4, r11, 0x37c8
	ctx.r[4].s64 = ctx.r[11].s64 + 14280;
	// 83283B48: 386AE3A8  addi r3, r10, -0x1c58
	ctx.r[3].s64 = ctx.r[10].s64 + -7256;
	// 83283B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283B50: 4AFA9381  bl 0x8222ced0
	ctx.lr = 0x83283B54;
	sub_8222CED0(ctx, base);
	// 83283B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283B58: 386913C0  addi r3, r9, 0x13c0
	ctx.r[3].s64 = ctx.r[9].s64 + 5056;
	// 83283B5C: 4BA263C5  bl 0x82ca9f20
	ctx.lr = 0x83283B60;
	sub_82CA9F20(ctx, base);
	// 83283B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283B70 size=64
    let mut pc: u32 = 0x83283B70;
    'dispatch: loop {
        match pc {
            0x83283B70 => {
    //   block [0x83283B70..0x83283BB0)
	// 83283B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283B7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83283B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283B84: 388B13B0  addi r4, r11, 0x13b0
	ctx.r[4].s64 = ctx.r[11].s64 + 5040;
	// 83283B88: 386AE3AC  addi r3, r10, -0x1c54
	ctx.r[3].s64 = ctx.r[10].s64 + -7252;
	// 83283B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283B90: 4AFA9341  bl 0x8222ced0
	ctx.lr = 0x83283B94;
	sub_8222CED0(ctx, base);
	// 83283B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283B98: 386913D0  addi r3, r9, 0x13d0
	ctx.r[3].s64 = ctx.r[9].s64 + 5072;
	// 83283B9C: 4BA26385  bl 0x82ca9f20
	ctx.lr = 0x83283BA0;
	sub_82CA9F20(ctx, base);
	// 83283BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283BB0 size=64
    let mut pc: u32 = 0x83283BB0;
    'dispatch: loop {
        match pc {
            0x83283BB0 => {
    //   block [0x83283BB0..0x83283BF0)
	// 83283BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283BBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83283BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283BC4: 388B37D8  addi r4, r11, 0x37d8
	ctx.r[4].s64 = ctx.r[11].s64 + 14296;
	// 83283BC8: 386AE3B0  addi r3, r10, -0x1c50
	ctx.r[3].s64 = ctx.r[10].s64 + -7248;
	// 83283BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283BD0: 4AFA9301  bl 0x8222ced0
	ctx.lr = 0x83283BD4;
	sub_8222CED0(ctx, base);
	// 83283BD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283BD8: 386913E0  addi r3, r9, 0x13e0
	ctx.r[3].s64 = ctx.r[9].s64 + 5088;
	// 83283BDC: 4BA26345  bl 0x82ca9f20
	ctx.lr = 0x83283BE0;
	sub_82CA9F20(ctx, base);
	// 83283BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283BF0 size=64
    let mut pc: u32 = 0x83283BF0;
    'dispatch: loop {
        match pc {
            0x83283BF0 => {
    //   block [0x83283BF0..0x83283C30)
	// 83283BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283BFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83283C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283C04: 388B13A0  addi r4, r11, 0x13a0
	ctx.r[4].s64 = ctx.r[11].s64 + 5024;
	// 83283C08: 386AE3B4  addi r3, r10, -0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7244;
	// 83283C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283C10: 4AFA92C1  bl 0x8222ced0
	ctx.lr = 0x83283C14;
	sub_8222CED0(ctx, base);
	// 83283C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283C18: 386913F0  addi r3, r9, 0x13f0
	ctx.r[3].s64 = ctx.r[9].s64 + 5104;
	// 83283C1C: 4BA26305  bl 0x82ca9f20
	ctx.lr = 0x83283C20;
	sub_82CA9F20(ctx, base);
	// 83283C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83283C30 size=44
    let mut pc: u32 = 0x83283C30;
    'dispatch: loop {
        match pc {
            0x83283C30 => {
    //   block [0x83283C30..0x83283C5C)
	// 83283C30: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83283C34: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83283C38: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83283C3C: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83283C40: C9AA1A28  lfd f13, 0x1a28(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(6696 as u32) ) };
	// 83283C44: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83283C48: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83283C4C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83283C50: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83283C54: 9169E3B8  stw r11, -0x1c48(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7240 as u32), ctx.r[11].u32 ) };
	// 83283C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83283C60 size=44
    let mut pc: u32 = 0x83283C60;
    'dispatch: loop {
        match pc {
            0x83283C60 => {
    //   block [0x83283C60..0x83283C8C)
	// 83283C60: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83283C64: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83283C68: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83283C6C: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83283C70: C9AA1A28  lfd f13, 0x1a28(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(6696 as u32) ) };
	// 83283C74: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83283C78: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83283C7C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83283C80: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83283C84: 9169E3BC  stw r11, -0x1c44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-7236 as u32), ctx.r[11].u32 ) };
	// 83283C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283C90 size=64
    let mut pc: u32 = 0x83283C90;
    'dispatch: loop {
        match pc {
            0x83283C90 => {
    //   block [0x83283C90..0x83283CD0)
	// 83283C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283C9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283CA4: 388B1308  addi r4, r11, 0x1308
	ctx.r[4].s64 = ctx.r[11].s64 + 4872;
	// 83283CA8: 386AE3C0  addi r3, r10, -0x1c40
	ctx.r[3].s64 = ctx.r[10].s64 + -7232;
	// 83283CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283CB0: 4AFA9221  bl 0x8222ced0
	ctx.lr = 0x83283CB4;
	sub_8222CED0(ctx, base);
	// 83283CB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283CB8: 38691400  addi r3, r9, 0x1400
	ctx.r[3].s64 = ctx.r[9].s64 + 5120;
	// 83283CBC: 4BA26265  bl 0x82ca9f20
	ctx.lr = 0x83283CC0;
	sub_82CA9F20(ctx, base);
	// 83283CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283CD0 size=64
    let mut pc: u32 = 0x83283CD0;
    'dispatch: loop {
        match pc {
            0x83283CD0 => {
    //   block [0x83283CD0..0x83283D10)
	// 83283CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283CDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283CE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283CE4: 388B1314  addi r4, r11, 0x1314
	ctx.r[4].s64 = ctx.r[11].s64 + 4884;
	// 83283CE8: 386AE3C4  addi r3, r10, -0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7228;
	// 83283CEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283CF0: 4AFA91E1  bl 0x8222ced0
	ctx.lr = 0x83283CF4;
	sub_8222CED0(ctx, base);
	// 83283CF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283CF8: 38691410  addi r3, r9, 0x1410
	ctx.r[3].s64 = ctx.r[9].s64 + 5136;
	// 83283CFC: 4BA26225  bl 0x82ca9f20
	ctx.lr = 0x83283D00;
	sub_82CA9F20(ctx, base);
	// 83283D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283D10 size=52
    let mut pc: u32 = 0x83283D10;
    'dispatch: loop {
        match pc {
            0x83283D10 => {
    //   block [0x83283D10..0x83283D44)
	// 83283D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283D1C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83283D20: 386BE3C8  addi r3, r11, -0x1c38
	ctx.r[3].s64 = ctx.r[11].s64 + -7224;
	// 83283D24: 4B78688D  bl 0x82a0a5b0
	ctx.lr = 0x83283D28;
	sub_82A0A5B0(ctx, base);
	// 83283D28: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83283D2C: 386A1420  addi r3, r10, 0x1420
	ctx.r[3].s64 = ctx.r[10].s64 + 5152;
	// 83283D30: 4BA261F1  bl 0x82ca9f20
	ctx.lr = 0x83283D34;
	sub_82CA9F20(ctx, base);
	// 83283D34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283D48 size=64
    let mut pc: u32 = 0x83283D48;
    'dispatch: loop {
        match pc {
            0x83283D48 => {
    //   block [0x83283D48..0x83283D88)
	// 83283D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283D54: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283D58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283D5C: 388B1328  addi r4, r11, 0x1328
	ctx.r[4].s64 = ctx.r[11].s64 + 4904;
	// 83283D60: 386AE3D4  addi r3, r10, -0x1c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7212;
	// 83283D64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283D68: 4AFA9169  bl 0x8222ced0
	ctx.lr = 0x83283D6C;
	sub_8222CED0(ctx, base);
	// 83283D6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283D70: 38691430  addi r3, r9, 0x1430
	ctx.r[3].s64 = ctx.r[9].s64 + 5168;
	// 83283D74: 4BA261AD  bl 0x82ca9f20
	ctx.lr = 0x83283D78;
	sub_82CA9F20(ctx, base);
	// 83283D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283D88 size=64
    let mut pc: u32 = 0x83283D88;
    'dispatch: loop {
        match pc {
            0x83283D88 => {
    //   block [0x83283D88..0x83283DC8)
	// 83283D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283D94: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283D98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283D9C: 388B14E4  addi r4, r11, 0x14e4
	ctx.r[4].s64 = ctx.r[11].s64 + 5348;
	// 83283DA0: 386AE3D8  addi r3, r10, -0x1c28
	ctx.r[3].s64 = ctx.r[10].s64 + -7208;
	// 83283DA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283DA8: 4AFA9129  bl 0x8222ced0
	ctx.lr = 0x83283DAC;
	sub_8222CED0(ctx, base);
	// 83283DAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283DB0: 38691440  addi r3, r9, 0x1440
	ctx.r[3].s64 = ctx.r[9].s64 + 5184;
	// 83283DB4: 4BA2616D  bl 0x82ca9f20
	ctx.lr = 0x83283DB8;
	sub_82CA9F20(ctx, base);
	// 83283DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283DC8 size=64
    let mut pc: u32 = 0x83283DC8;
    'dispatch: loop {
        match pc {
            0x83283DC8 => {
    //   block [0x83283DC8..0x83283E08)
	// 83283DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283DD4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83283DD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283DDC: 388B14F0  addi r4, r11, 0x14f0
	ctx.r[4].s64 = ctx.r[11].s64 + 5360;
	// 83283DE0: 386AE3DC  addi r3, r10, -0x1c24
	ctx.r[3].s64 = ctx.r[10].s64 + -7204;
	// 83283DE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83283DE8: 4AFA90E9  bl 0x8222ced0
	ctx.lr = 0x83283DEC;
	sub_8222CED0(ctx, base);
	// 83283DEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83283DF0: 38691450  addi r3, r9, 0x1450
	ctx.r[3].s64 = ctx.r[9].s64 + 5200;
	// 83283DF4: 4BA2612D  bl 0x82ca9f20
	ctx.lr = 0x83283DF8;
	sub_82CA9F20(ctx, base);
	// 83283DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283E08 size=56
    let mut pc: u32 = 0x83283E08;
    'dispatch: loop {
        match pc {
            0x83283E08 => {
    //   block [0x83283E08..0x83283E40)
	// 83283E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283E1C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83283E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283E24: 4AF6FF35  bl 0x821f3d58
	ctx.lr = 0x83283E28;
	sub_821F3D58(ctx, base);
	// 83283E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283E2C: 906AE3E0  stw r3, -0x1c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7200 as u32), ctx.r[3].u32 ) };
	// 83283E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283E40 size=56
    let mut pc: u32 = 0x83283E40;
    'dispatch: loop {
        match pc {
            0x83283E40 => {
    //   block [0x83283E40..0x83283E78)
	// 83283E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283E54: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83283E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283E5C: 4AF6FEFD  bl 0x821f3d58
	ctx.lr = 0x83283E60;
	sub_821F3D58(ctx, base);
	// 83283E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283E64: 906AE3E4  stw r3, -0x1c1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7196 as u32), ctx.r[3].u32 ) };
	// 83283E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283E78 size=56
    let mut pc: u32 = 0x83283E78;
    'dispatch: loop {
        match pc {
            0x83283E78 => {
    //   block [0x83283E78..0x83283EB0)
	// 83283E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283E8C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83283E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283E94: 4AF6FEC5  bl 0x821f3d58
	ctx.lr = 0x83283E98;
	sub_821F3D58(ctx, base);
	// 83283E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283E9C: 906AE3E8  stw r3, -0x1c18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7192 as u32), ctx.r[3].u32 ) };
	// 83283EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283EB0 size=56
    let mut pc: u32 = 0x83283EB0;
    'dispatch: loop {
        match pc {
            0x83283EB0 => {
    //   block [0x83283EB0..0x83283EE8)
	// 83283EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283EC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283EC4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83283EC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283ECC: 4AF6FE8D  bl 0x821f3d58
	ctx.lr = 0x83283ED0;
	sub_821F3D58(ctx, base);
	// 83283ED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283ED4: 906AE3EC  stw r3, -0x1c14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7188 as u32), ctx.r[3].u32 ) };
	// 83283ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283EE8 size=56
    let mut pc: u32 = 0x83283EE8;
    'dispatch: loop {
        match pc {
            0x83283EE8 => {
    //   block [0x83283EE8..0x83283F20)
	// 83283EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283EF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283EF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283EFC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83283F00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283F04: 4AF6FE55  bl 0x821f3d58
	ctx.lr = 0x83283F08;
	sub_821F3D58(ctx, base);
	// 83283F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283F0C: 906AE3F0  stw r3, -0x1c10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7184 as u32), ctx.r[3].u32 ) };
	// 83283F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283F20 size=56
    let mut pc: u32 = 0x83283F20;
    'dispatch: loop {
        match pc {
            0x83283F20 => {
    //   block [0x83283F20..0x83283F58)
	// 83283F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283F2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283F30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283F34: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83283F38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283F3C: 4AF6FE1D  bl 0x821f3d58
	ctx.lr = 0x83283F40;
	sub_821F3D58(ctx, base);
	// 83283F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283F44: 906AE3F4  stw r3, -0x1c0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7180 as u32), ctx.r[3].u32 ) };
	// 83283F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283F58 size=56
    let mut pc: u32 = 0x83283F58;
    'dispatch: loop {
        match pc {
            0x83283F58 => {
    //   block [0x83283F58..0x83283F90)
	// 83283F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283F64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283F68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283F6C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83283F70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283F74: 4AF6FDE5  bl 0x821f3d58
	ctx.lr = 0x83283F78;
	sub_821F3D58(ctx, base);
	// 83283F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283F7C: 906AE3F8  stw r3, -0x1c08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7176 as u32), ctx.r[3].u32 ) };
	// 83283F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283F90 size=56
    let mut pc: u32 = 0x83283F90;
    'dispatch: loop {
        match pc {
            0x83283F90 => {
    //   block [0x83283F90..0x83283FC8)
	// 83283F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283F9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283FA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283FA4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83283FA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283FAC: 4AF6FDAD  bl 0x821f3d58
	ctx.lr = 0x83283FB0;
	sub_821F3D58(ctx, base);
	// 83283FB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283FB4: 906AE3FC  stw r3, -0x1c04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7172 as u32), ctx.r[3].u32 ) };
	// 83283FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83283FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83283FC8 size=56
    let mut pc: u32 = 0x83283FC8;
    'dispatch: loop {
        match pc {
            0x83283FC8 => {
    //   block [0x83283FC8..0x83284000)
	// 83283FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83283FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83283FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83283FD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83283FD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83283FDC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83283FE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83283FE4: 4AF6FD75  bl 0x821f3d58
	ctx.lr = 0x83283FE8;
	sub_821F3D58(ctx, base);
	// 83283FE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83283FEC: 906AE400  stw r3, -0x1c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7168 as u32), ctx.r[3].u32 ) };
	// 83283FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83283FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83283FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83283FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284000 size=56
    let mut pc: u32 = 0x83284000;
    'dispatch: loop {
        match pc {
            0x83284000 => {
    //   block [0x83284000..0x83284038)
	// 83284000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328400C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284010: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83284014: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83284018: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328401C: 4AF6FD3D  bl 0x821f3d58
	ctx.lr = 0x83284020;
	sub_821F3D58(ctx, base);
	// 83284020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284024: 906AE404  stw r3, -0x1bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7164 as u32), ctx.r[3].u32 ) };
	// 83284028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328402C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284038 size=56
    let mut pc: u32 = 0x83284038;
    'dispatch: loop {
        match pc {
            0x83284038 => {
    //   block [0x83284038..0x83284070)
	// 83284038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328403C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284044: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284048: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328404C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83284050: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83284054: 4AF6FD05  bl 0x821f3d58
	ctx.lr = 0x83284058;
	sub_821F3D58(ctx, base);
	// 83284058: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328405C: 906AE408  stw r3, -0x1bf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7160 as u32), ctx.r[3].u32 ) };
	// 83284060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328406C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284070 size=56
    let mut pc: u32 = 0x83284070;
    'dispatch: loop {
        match pc {
            0x83284070 => {
    //   block [0x83284070..0x832840A8)
	// 83284070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328407C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284080: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83284084: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83284088: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328408C: 4AF6FCCD  bl 0x821f3d58
	ctx.lr = 0x83284090;
	sub_821F3D58(ctx, base);
	// 83284090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284094: 906AE40C  stw r3, -0x1bf4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7156 as u32), ctx.r[3].u32 ) };
	// 83284098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328409C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832840A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832840A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832840A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832840A8 size=56
    let mut pc: u32 = 0x832840A8;
    'dispatch: loop {
        match pc {
            0x832840A8 => {
    //   block [0x832840A8..0x832840E0)
	// 832840A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832840AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832840B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832840B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832840B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832840BC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832840C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832840C4: 4AF6FC95  bl 0x821f3d58
	ctx.lr = 0x832840C8;
	sub_821F3D58(ctx, base);
	// 832840C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832840CC: 906AE410  stw r3, -0x1bf0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7152 as u32), ctx.r[3].u32 ) };
	// 832840D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832840D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832840D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832840DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832840E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832840E0 size=56
    let mut pc: u32 = 0x832840E0;
    'dispatch: loop {
        match pc {
            0x832840E0 => {
    //   block [0x832840E0..0x83284118)
	// 832840E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832840E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832840E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832840EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832840F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832840F4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832840F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832840FC: 4AF6FC5D  bl 0x821f3d58
	ctx.lr = 0x83284100;
	sub_821F3D58(ctx, base);
	// 83284100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284104: 906AE414  stw r3, -0x1bec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7148 as u32), ctx.r[3].u32 ) };
	// 83284108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328410C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284118 size=56
    let mut pc: u32 = 0x83284118;
    'dispatch: loop {
        match pc {
            0x83284118 => {
    //   block [0x83284118..0x83284150)
	// 83284118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328412C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83284130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83284134: 4AF6FC25  bl 0x821f3d58
	ctx.lr = 0x83284138;
	sub_821F3D58(ctx, base);
	// 83284138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328413C: 906AE418  stw r3, -0x1be8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7144 as u32), ctx.r[3].u32 ) };
	// 83284140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328414C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284150 size=56
    let mut pc: u32 = 0x83284150;
    'dispatch: loop {
        match pc {
            0x83284150 => {
    //   block [0x83284150..0x83284188)
	// 83284150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328415C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83284164: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83284168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328416C: 4AF6FBED  bl 0x821f3d58
	ctx.lr = 0x83284170;
	sub_821F3D58(ctx, base);
	// 83284170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284174: 906AE41C  stw r3, -0x1be4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7140 as u32), ctx.r[3].u32 ) };
	// 83284178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328417C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284188 size=56
    let mut pc: u32 = 0x83284188;
    'dispatch: loop {
        match pc {
            0x83284188 => {
    //   block [0x83284188..0x832841C0)
	// 83284188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328418C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284194: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284198: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328419C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832841A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832841A4: 4AF6FBB5  bl 0x821f3d58
	ctx.lr = 0x832841A8;
	sub_821F3D58(ctx, base);
	// 832841A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832841AC: 906AE420  stw r3, -0x1be0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7136 as u32), ctx.r[3].u32 ) };
	// 832841B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832841B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832841B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832841BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832841C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832841C0 size=56
    let mut pc: u32 = 0x832841C0;
    'dispatch: loop {
        match pc {
            0x832841C0 => {
    //   block [0x832841C0..0x832841F8)
	// 832841C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832841C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832841C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832841CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832841D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832841D4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832841D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832841DC: 4AF6FB7D  bl 0x821f3d58
	ctx.lr = 0x832841E0;
	sub_821F3D58(ctx, base);
	// 832841E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832841E4: 906AE424  stw r3, -0x1bdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7132 as u32), ctx.r[3].u32 ) };
	// 832841E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832841EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832841F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832841F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832841F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832841F8 size=56
    let mut pc: u32 = 0x832841F8;
    'dispatch: loop {
        match pc {
            0x832841F8 => {
    //   block [0x832841F8..0x83284230)
	// 832841F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832841FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284208: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328420C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83284210: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83284214: 4AF6FB45  bl 0x821f3d58
	ctx.lr = 0x83284218;
	sub_821F3D58(ctx, base);
	// 83284218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328421C: 906AE428  stw r3, -0x1bd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7128 as u32), ctx.r[3].u32 ) };
	// 83284220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328422C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284230 size=56
    let mut pc: u32 = 0x83284230;
    'dispatch: loop {
        match pc {
            0x83284230 => {
    //   block [0x83284230..0x83284268)
	// 83284230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328423C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284240: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83284244: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83284248: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328424C: 4AF6FB0D  bl 0x821f3d58
	ctx.lr = 0x83284250;
	sub_821F3D58(ctx, base);
	// 83284250: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284254: 906AE42C  stw r3, -0x1bd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7124 as u32), ctx.r[3].u32 ) };
	// 83284258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328425C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284268 size=56
    let mut pc: u32 = 0x83284268;
    'dispatch: loop {
        match pc {
            0x83284268 => {
    //   block [0x83284268..0x832842A0)
	// 83284268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328426C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284274: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284278: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328427C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83284280: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83284284: 4AF6FAD5  bl 0x821f3d58
	ctx.lr = 0x83284288;
	sub_821F3D58(ctx, base);
	// 83284288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328428C: 906AE430  stw r3, -0x1bd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7120 as u32), ctx.r[3].u32 ) };
	// 83284290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328429C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832842A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832842A0 size=64
    let mut pc: u32 = 0x832842A0;
    'dispatch: loop {
        match pc {
            0x832842A0 => {
    //   block [0x832842A0..0x832842E0)
	// 832842A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832842A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832842A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832842AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832842B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832842B4: 388B1534  addi r4, r11, 0x1534
	ctx.r[4].s64 = ctx.r[11].s64 + 5428;
	// 832842B8: 386AE434  addi r3, r10, -0x1bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -7116;
	// 832842BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832842C0: 4AFA8C11  bl 0x8222ced0
	ctx.lr = 0x832842C4;
	sub_8222CED0(ctx, base);
	// 832842C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832842C8: 38691460  addi r3, r9, 0x1460
	ctx.r[3].s64 = ctx.r[9].s64 + 5216;
	// 832842CC: 4BA25C55  bl 0x82ca9f20
	ctx.lr = 0x832842D0;
	sub_82CA9F20(ctx, base);
	// 832842D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832842D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832842D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832842DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832842E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832842E0 size=52
    let mut pc: u32 = 0x832842E0;
    'dispatch: loop {
        match pc {
            0x832842E0 => {
    //   block [0x832842E0..0x83284314)
	// 832842E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832842E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832842E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832842EC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832842F0: 386BE438  addi r3, r11, -0x1bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -7112;
	// 832842F4: 48035991  bl 0x832b9c84
	ctx.lr = 0x832842F8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832842F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832842FC: 386A1470  addi r3, r10, 0x1470
	ctx.r[3].s64 = ctx.r[10].s64 + 5232;
	// 83284300: 4BA25C21  bl 0x82ca9f20
	ctx.lr = 0x83284304;
	sub_82CA9F20(ctx, base);
	// 83284304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328430C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284318 size=56
    let mut pc: u32 = 0x83284318;
    'dispatch: loop {
        match pc {
            0x83284318 => {
    //   block [0x83284318..0x83284350)
	// 83284318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284324: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83284328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328432C: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 83284330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83284334: 4AF6FA25  bl 0x821f3d58
	ctx.lr = 0x83284338;
	sub_821F3D58(ctx, base);
	// 83284338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328433C: 906AE454  stw r3, -0x1bac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7084 as u32), ctx.r[3].u32 ) };
	// 83284340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328434C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284350 size=56
    let mut pc: u32 = 0x83284350;
    'dispatch: loop {
        match pc {
            0x83284350 => {
    //   block [0x83284350..0x83284388)
	// 83284350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328435C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83284360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83284364: 386B0E48  addi r3, r11, 0xe48
	ctx.r[3].s64 = ctx.r[11].s64 + 3656;
	// 83284368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328436C: 4AF6F9ED  bl 0x821f3d58
	ctx.lr = 0x83284370;
	sub_821F3D58(ctx, base);
	// 83284370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284374: 906AE458  stw r3, -0x1ba8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7080 as u32), ctx.r[3].u32 ) };
	// 83284378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328437C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284388 size=56
    let mut pc: u32 = 0x83284388;
    'dispatch: loop {
        match pc {
            0x83284388 => {
    //   block [0x83284388..0x832843C0)
	// 83284388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328438C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284394: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83284398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328439C: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 832843A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832843A4: 4AF6F9B5  bl 0x821f3d58
	ctx.lr = 0x832843A8;
	sub_821F3D58(ctx, base);
	// 832843A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832843AC: 906AE45C  stw r3, -0x1ba4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7076 as u32), ctx.r[3].u32 ) };
	// 832843B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832843B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832843B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832843BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832843C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832843C0 size=56
    let mut pc: u32 = 0x832843C0;
    'dispatch: loop {
        match pc {
            0x832843C0 => {
    //   block [0x832843C0..0x832843F8)
	// 832843C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832843C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832843C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832843CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832843D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832843D4: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 832843D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832843DC: 4AF6F97D  bl 0x821f3d58
	ctx.lr = 0x832843E0;
	sub_821F3D58(ctx, base);
	// 832843E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832843E4: 906AE464  stw r3, -0x1b9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7068 as u32), ctx.r[3].u32 ) };
	// 832843E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832843EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832843F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832843F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832843F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832843F8 size=16
    let mut pc: u32 = 0x832843F8;
    'dispatch: loop {
        match pc {
            0x832843F8 => {
    //   block [0x832843F8..0x83284408)
	// 832843F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832843FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83284400: 914BE468  stw r10, -0x1b98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-7064 as u32), ctx.r[10].u32 ) };
	// 83284404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284408 size=64
    let mut pc: u32 = 0x83284408;
    'dispatch: loop {
        match pc {
            0x83284408 => {
    //   block [0x83284408..0x83284448)
	// 83284408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328441C: 388BB3C0  addi r4, r11, -0x4c40
	ctx.r[4].s64 = ctx.r[11].s64 + -19520;
	// 83284420: 386AE46C  addi r3, r10, -0x1b94
	ctx.r[3].s64 = ctx.r[10].s64 + -7060;
	// 83284424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284428: 4AFA8AA9  bl 0x8222ced0
	ctx.lr = 0x8328442C;
	sub_8222CED0(ctx, base);
	// 8328442C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284430: 38691478  addi r3, r9, 0x1478
	ctx.r[3].s64 = ctx.r[9].s64 + 5240;
	// 83284434: 4BA25AED  bl 0x82ca9f20
	ctx.lr = 0x83284438;
	sub_82CA9F20(ctx, base);
	// 83284438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328443C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284448 size=64
    let mut pc: u32 = 0x83284448;
    'dispatch: loop {
        match pc {
            0x83284448 => {
    //   block [0x83284448..0x83284488)
	// 83284448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328445C: 388BB390  addi r4, r11, -0x4c70
	ctx.r[4].s64 = ctx.r[11].s64 + -19568;
	// 83284460: 386AE470  addi r3, r10, -0x1b90
	ctx.r[3].s64 = ctx.r[10].s64 + -7056;
	// 83284464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284468: 4AFA8A69  bl 0x8222ced0
	ctx.lr = 0x8328446C;
	sub_8222CED0(ctx, base);
	// 8328446C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284470: 38691488  addi r3, r9, 0x1488
	ctx.r[3].s64 = ctx.r[9].s64 + 5256;
	// 83284474: 4BA25AAD  bl 0x82ca9f20
	ctx.lr = 0x83284478;
	sub_82CA9F20(ctx, base);
	// 83284478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328447C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284488 size=64
    let mut pc: u32 = 0x83284488;
    'dispatch: loop {
        match pc {
            0x83284488 => {
    //   block [0x83284488..0x832844C8)
	// 83284488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328449C: 388BB398  addi r4, r11, -0x4c68
	ctx.r[4].s64 = ctx.r[11].s64 + -19560;
	// 832844A0: 386AE474  addi r3, r10, -0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7052;
	// 832844A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832844A8: 4AFA8A29  bl 0x8222ced0
	ctx.lr = 0x832844AC;
	sub_8222CED0(ctx, base);
	// 832844AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832844B0: 38691498  addi r3, r9, 0x1498
	ctx.r[3].s64 = ctx.r[9].s64 + 5272;
	// 832844B4: 4BA25A6D  bl 0x82ca9f20
	ctx.lr = 0x832844B8;
	sub_82CA9F20(ctx, base);
	// 832844B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832844BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832844C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832844C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832844C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832844C8 size=64
    let mut pc: u32 = 0x832844C8;
    'dispatch: loop {
        match pc {
            0x832844C8 => {
    //   block [0x832844C8..0x83284508)
	// 832844C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832844CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832844D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832844D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832844D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832844DC: 388BB3AC  addi r4, r11, -0x4c54
	ctx.r[4].s64 = ctx.r[11].s64 + -19540;
	// 832844E0: 386AE478  addi r3, r10, -0x1b88
	ctx.r[3].s64 = ctx.r[10].s64 + -7048;
	// 832844E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832844E8: 4AFA89E9  bl 0x8222ced0
	ctx.lr = 0x832844EC;
	sub_8222CED0(ctx, base);
	// 832844EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832844F0: 386914A8  addi r3, r9, 0x14a8
	ctx.r[3].s64 = ctx.r[9].s64 + 5288;
	// 832844F4: 4BA25A2D  bl 0x82ca9f20
	ctx.lr = 0x832844F8;
	sub_82CA9F20(ctx, base);
	// 832844F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832844FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83284508 size=12
    let mut pc: u32 = 0x83284508;
    'dispatch: loop {
        match pc {
            0x83284508 => {
    //   block [0x83284508..0x83284514)
	// 83284508: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328450C: 386B14B8  addi r3, r11, 0x14b8
	ctx.r[3].s64 = ctx.r[11].s64 + 5304;
	// 83284510: 4BA25A10  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83284518 size=12
    let mut pc: u32 = 0x83284518;
    'dispatch: loop {
        match pc {
            0x83284518 => {
    //   block [0x83284518..0x83284524)
	// 83284518: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328451C: 386B14C8  addi r3, r11, 0x14c8
	ctx.r[3].s64 = ctx.r[11].s64 + 5320;
	// 83284520: 4BA25A00  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83284528 size=12
    let mut pc: u32 = 0x83284528;
    'dispatch: loop {
        match pc {
            0x83284528 => {
    //   block [0x83284528..0x83284534)
	// 83284528: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328452C: 386B14D8  addi r3, r11, 0x14d8
	ctx.r[3].s64 = ctx.r[11].s64 + 5336;
	// 83284530: 4BA259F0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83284538 size=20
    let mut pc: u32 = 0x83284538;
    'dispatch: loop {
        match pc {
            0x83284538 => {
    //   block [0x83284538..0x8328454C)
	// 83284538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328453C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83284540: 394BE4B0  addi r10, r11, -0x1b50
	ctx.r[10].s64 = ctx.r[11].s64 + -6992;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83284550 size=96
    let mut pc: u32 = 0x83284550;
    'dispatch: loop {
        match pc {
            0x83284550 => {
    //   block [0x83284550..0x832845B0)
	// 83284550: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284554: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83284558: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8328455C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83284560: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83284564: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83284568: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8328456C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83284570: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832845B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832845B0 size=96
    let mut pc: u32 = 0x832845B0;
    'dispatch: loop {
        match pc {
            0x832845B0 => {
    //   block [0x832845B0..0x83284610)
	// 832845B0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832845B4: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832845B8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832845BC: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 832845C0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832845C4: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832845C8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832845CC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832845D0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83284610 size=12
    let mut pc: u32 = 0x83284610;
    'dispatch: loop {
        match pc {
            0x83284610 => {
    //   block [0x83284610..0x8328461C)
	// 83284610: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83284614: 386B14E8  addi r3, r11, 0x14e8
	ctx.r[3].s64 = ctx.r[11].s64 + 5352;
	// 83284618: 4BA25908  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83284620 size=12
    let mut pc: u32 = 0x83284620;
    'dispatch: loop {
        match pc {
            0x83284620 => {
    //   block [0x83284620..0x8328462C)
	// 83284620: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83284624: 386B14F8  addi r3, r11, 0x14f8
	ctx.r[3].s64 = ctx.r[11].s64 + 5368;
	// 83284628: 4BA258F8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284630 size=64
    let mut pc: u32 = 0x83284630;
    'dispatch: loop {
        match pc {
            0x83284630 => {
    //   block [0x83284630..0x83284670)
	// 83284630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328463C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284644: 388B921A  addi r4, r11, -0x6de6
	ctx.r[4].s64 = ctx.r[11].s64 + -28134;
	// 83284648: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 8328464C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284650: 4AFA8881  bl 0x8222ced0
	ctx.lr = 0x83284654;
	sub_8222CED0(ctx, base);
	// 83284654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284658: 38691508  addi r3, r9, 0x1508
	ctx.r[3].s64 = ctx.r[9].s64 + 5384;
	// 8328465C: 4BA258C5  bl 0x82ca9f20
	ctx.lr = 0x83284660;
	sub_82CA9F20(ctx, base);
	// 83284660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328466C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284670 size=144
    let mut pc: u32 = 0x83284670;
    'dispatch: loop {
        match pc {
            0x83284670 => {
    //   block [0x83284670..0x83284694)
	// 83284670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328467C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 83284680: 4AF9ABD9  bl 0x8221f258
	ctx.lr = 0x83284684;
	sub_8221F258(ctx, base);
	// 83284684: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83284688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8328468C: 419A0008  beq cr6, 0x83284694
	if ctx.cr[6].eq {
	pc = 0x83284694; continue 'dispatch;
	}
	// 83284690: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83284694; continue 'dispatch;
            }
            0x83284694 => {
    //   block [0x83284694..0x832846A0)
	// 83284694: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83284698: 41820008  beq 0x832846a0
	if ctx.cr[0].eq {
	pc = 0x832846A0; continue 'dispatch;
	}
	// 8328469C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832846A0; continue 'dispatch;
            }
            0x832846A0 => {
    //   block [0x832846A0..0x832846AC)
	// 832846A0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832846A4: 41820008  beq 0x832846ac
	if ctx.cr[0].eq {
	pc = 0x832846AC; continue 'dispatch;
	}
	// 832846A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832846AC; continue 'dispatch;
            }
            0x832846AC => {
    //   block [0x832846AC..0x83284700)
	// 832846AC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832846B0: 99430081  stb r10, 0x81(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(129 as u32), ctx.r[10].u8 ) };
	// 832846B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832846B8: 3909E4F0  addi r8, r9, -0x1b10
	ctx.r[8].s64 = ctx.r[9].s64 + -6928;
	// 832846BC: 99630080  stb r11, 0x80(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 832846C0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832846C4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 832846C8: 99630081  stb r11, 0x81(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 832846CC: 38671518  addi r3, r7, 0x1518
	ctx.r[3].s64 = ctx.r[7].s64 + 5400;
	// 832846D0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 832846D4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832846D8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 832846DC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832846E0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 832846E4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832846E8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832846EC: 4BA25835  bl 0x82ca9f20
	ctx.lr = 0x832846F0;
	sub_82CA9F20(ctx, base);
	// 832846F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832846F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832846F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832846FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284700 size=64
    let mut pc: u32 = 0x83284700;
    'dispatch: loop {
        match pc {
            0x83284700 => {
    //   block [0x83284700..0x83284740)
	// 83284700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328470C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284714: 388BB2D0  addi r4, r11, -0x4d30
	ctx.r[4].s64 = ctx.r[11].s64 + -19760;
	// 83284718: 386AE4FC  addi r3, r10, -0x1b04
	ctx.r[3].s64 = ctx.r[10].s64 + -6916;
	// 8328471C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284720: 4AFA87B1  bl 0x8222ced0
	ctx.lr = 0x83284724;
	sub_8222CED0(ctx, base);
	// 83284724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284728: 38691538  addi r3, r9, 0x1538
	ctx.r[3].s64 = ctx.r[9].s64 + 5432;
	// 8328472C: 4BA257F5  bl 0x82ca9f20
	ctx.lr = 0x83284730;
	sub_82CA9F20(ctx, base);
	// 83284730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328473C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284740 size=64
    let mut pc: u32 = 0x83284740;
    'dispatch: loop {
        match pc {
            0x83284740 => {
    //   block [0x83284740..0x83284780)
	// 83284740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328474C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284754: 388BB2D8  addi r4, r11, -0x4d28
	ctx.r[4].s64 = ctx.r[11].s64 + -19752;
	// 83284758: 386AE500  addi r3, r10, -0x1b00
	ctx.r[3].s64 = ctx.r[10].s64 + -6912;
	// 8328475C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284760: 4AFA8771  bl 0x8222ced0
	ctx.lr = 0x83284764;
	sub_8222CED0(ctx, base);
	// 83284764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284768: 38691548  addi r3, r9, 0x1548
	ctx.r[3].s64 = ctx.r[9].s64 + 5448;
	// 8328476C: 4BA257B5  bl 0x82ca9f20
	ctx.lr = 0x83284770;
	sub_82CA9F20(ctx, base);
	// 83284770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328477C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284780 size=64
    let mut pc: u32 = 0x83284780;
    'dispatch: loop {
        match pc {
            0x83284780 => {
    //   block [0x83284780..0x832847C0)
	// 83284780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328478C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284794: 388BB2EC  addi r4, r11, -0x4d14
	ctx.r[4].s64 = ctx.r[11].s64 + -19732;
	// 83284798: 386AE504  addi r3, r10, -0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + -6908;
	// 8328479C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832847A0: 4AFA8731  bl 0x8222ced0
	ctx.lr = 0x832847A4;
	sub_8222CED0(ctx, base);
	// 832847A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832847A8: 38691558  addi r3, r9, 0x1558
	ctx.r[3].s64 = ctx.r[9].s64 + 5464;
	// 832847AC: 4BA25775  bl 0x82ca9f20
	ctx.lr = 0x832847B0;
	sub_82CA9F20(ctx, base);
	// 832847B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832847B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832847B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832847BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832847C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832847C0 size=64
    let mut pc: u32 = 0x832847C0;
    'dispatch: loop {
        match pc {
            0x832847C0 => {
    //   block [0x832847C0..0x83284800)
	// 832847C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832847C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832847C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832847CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832847D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832847D4: 388BB300  addi r4, r11, -0x4d00
	ctx.r[4].s64 = ctx.r[11].s64 + -19712;
	// 832847D8: 386AE508  addi r3, r10, -0x1af8
	ctx.r[3].s64 = ctx.r[10].s64 + -6904;
	// 832847DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832847E0: 4AFA86F1  bl 0x8222ced0
	ctx.lr = 0x832847E4;
	sub_8222CED0(ctx, base);
	// 832847E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832847E8: 38691568  addi r3, r9, 0x1568
	ctx.r[3].s64 = ctx.r[9].s64 + 5480;
	// 832847EC: 4BA25735  bl 0x82ca9f20
	ctx.lr = 0x832847F0;
	sub_82CA9F20(ctx, base);
	// 832847F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832847F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832847F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832847FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284800 size=64
    let mut pc: u32 = 0x83284800;
    'dispatch: loop {
        match pc {
            0x83284800 => {
    //   block [0x83284800..0x83284840)
	// 83284800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328480C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284814: 388BB324  addi r4, r11, -0x4cdc
	ctx.r[4].s64 = ctx.r[11].s64 + -19676;
	// 83284818: 386AE50C  addi r3, r10, -0x1af4
	ctx.r[3].s64 = ctx.r[10].s64 + -6900;
	// 8328481C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284820: 4AFA86B1  bl 0x8222ced0
	ctx.lr = 0x83284824;
	sub_8222CED0(ctx, base);
	// 83284824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284828: 38691578  addi r3, r9, 0x1578
	ctx.r[3].s64 = ctx.r[9].s64 + 5496;
	// 8328482C: 4BA256F5  bl 0x82ca9f20
	ctx.lr = 0x83284830;
	sub_82CA9F20(ctx, base);
	// 83284830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284840 size=64
    let mut pc: u32 = 0x83284840;
    'dispatch: loop {
        match pc {
            0x83284840 => {
    //   block [0x83284840..0x83284880)
	// 83284840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328484C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284854: 388BB32C  addi r4, r11, -0x4cd4
	ctx.r[4].s64 = ctx.r[11].s64 + -19668;
	// 83284858: 386AE510  addi r3, r10, -0x1af0
	ctx.r[3].s64 = ctx.r[10].s64 + -6896;
	// 8328485C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284860: 4AFA8671  bl 0x8222ced0
	ctx.lr = 0x83284864;
	sub_8222CED0(ctx, base);
	// 83284864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284868: 38691588  addi r3, r9, 0x1588
	ctx.r[3].s64 = ctx.r[9].s64 + 5512;
	// 8328486C: 4BA256B5  bl 0x82ca9f20
	ctx.lr = 0x83284870;
	sub_82CA9F20(ctx, base);
	// 83284870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328487C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284880 size=64
    let mut pc: u32 = 0x83284880;
    'dispatch: loop {
        match pc {
            0x83284880 => {
    //   block [0x83284880..0x832848C0)
	// 83284880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328488C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284894: 388BB354  addi r4, r11, -0x4cac
	ctx.r[4].s64 = ctx.r[11].s64 + -19628;
	// 83284898: 386AE514  addi r3, r10, -0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + -6892;
	// 8328489C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832848A0: 4AFA8631  bl 0x8222ced0
	ctx.lr = 0x832848A4;
	sub_8222CED0(ctx, base);
	// 832848A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832848A8: 38691598  addi r3, r9, 0x1598
	ctx.r[3].s64 = ctx.r[9].s64 + 5528;
	// 832848AC: 4BA25675  bl 0x82ca9f20
	ctx.lr = 0x832848B0;
	sub_82CA9F20(ctx, base);
	// 832848B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832848B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832848B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832848BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832848C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832848C0 size=64
    let mut pc: u32 = 0x832848C0;
    'dispatch: loop {
        match pc {
            0x832848C0 => {
    //   block [0x832848C0..0x83284900)
	// 832848C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832848C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832848C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832848CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832848D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832848D4: 388BB360  addi r4, r11, -0x4ca0
	ctx.r[4].s64 = ctx.r[11].s64 + -19616;
	// 832848D8: 386AE518  addi r3, r10, -0x1ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -6888;
	// 832848DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832848E0: 4AFA85F1  bl 0x8222ced0
	ctx.lr = 0x832848E4;
	sub_8222CED0(ctx, base);
	// 832848E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832848E8: 386915A8  addi r3, r9, 0x15a8
	ctx.r[3].s64 = ctx.r[9].s64 + 5544;
	// 832848EC: 4BA25635  bl 0x82ca9f20
	ctx.lr = 0x832848F0;
	sub_82CA9F20(ctx, base);
	// 832848F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832848F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832848F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832848FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284900 size=64
    let mut pc: u32 = 0x83284900;
    'dispatch: loop {
        match pc {
            0x83284900 => {
    //   block [0x83284900..0x83284940)
	// 83284900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328490C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284914: 388BB368  addi r4, r11, -0x4c98
	ctx.r[4].s64 = ctx.r[11].s64 + -19608;
	// 83284918: 386AE51C  addi r3, r10, -0x1ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -6884;
	// 8328491C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284920: 4AFA85B1  bl 0x8222ced0
	ctx.lr = 0x83284924;
	sub_8222CED0(ctx, base);
	// 83284924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284928: 386915B8  addi r3, r9, 0x15b8
	ctx.r[3].s64 = ctx.r[9].s64 + 5560;
	// 8328492C: 4BA255F5  bl 0x82ca9f20
	ctx.lr = 0x83284930;
	sub_82CA9F20(ctx, base);
	// 83284930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328493C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284940 size=64
    let mut pc: u32 = 0x83284940;
    'dispatch: loop {
        match pc {
            0x83284940 => {
    //   block [0x83284940..0x83284980)
	// 83284940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328494C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284954: 388BB370  addi r4, r11, -0x4c90
	ctx.r[4].s64 = ctx.r[11].s64 + -19600;
	// 83284958: 386AE520  addi r3, r10, -0x1ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -6880;
	// 8328495C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284960: 4AFA8571  bl 0x8222ced0
	ctx.lr = 0x83284964;
	sub_8222CED0(ctx, base);
	// 83284964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284968: 386915C8  addi r3, r9, 0x15c8
	ctx.r[3].s64 = ctx.r[9].s64 + 5576;
	// 8328496C: 4BA255B5  bl 0x82ca9f20
	ctx.lr = 0x83284970;
	sub_82CA9F20(ctx, base);
	// 83284970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328497C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284980 size=64
    let mut pc: u32 = 0x83284980;
    'dispatch: loop {
        match pc {
            0x83284980 => {
    //   block [0x83284980..0x832849C0)
	// 83284980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328498C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284994: 388BB380  addi r4, r11, -0x4c80
	ctx.r[4].s64 = ctx.r[11].s64 + -19584;
	// 83284998: 386AE524  addi r3, r10, -0x1adc
	ctx.r[3].s64 = ctx.r[10].s64 + -6876;
	// 8328499C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832849A0: 4AFA8531  bl 0x8222ced0
	ctx.lr = 0x832849A4;
	sub_8222CED0(ctx, base);
	// 832849A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832849A8: 386915D8  addi r3, r9, 0x15d8
	ctx.r[3].s64 = ctx.r[9].s64 + 5592;
	// 832849AC: 4BA25575  bl 0x82ca9f20
	ctx.lr = 0x832849B0;
	sub_82CA9F20(ctx, base);
	// 832849B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832849B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832849B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832849BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832849C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832849C0 size=20
    let mut pc: u32 = 0x832849C0;
    'dispatch: loop {
        match pc {
            0x832849C0 => {
    //   block [0x832849C0..0x832849D4)
	// 832849C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832849C4: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832849C8: 394BE530  addi r10, r11, -0x1ad0
	ctx.r[10].s64 = ctx.r[11].s64 + -6864;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832849D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832849D8 size=64
    let mut pc: u32 = 0x832849D8;
    'dispatch: loop {
        match pc {
            0x832849D8 => {
    //   block [0x832849D8..0x83284A18)
	// 832849D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832849DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832849E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832849E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832849E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832849EC: 388BB280  addi r4, r11, -0x4d80
	ctx.r[4].s64 = ctx.r[11].s64 + -19840;
	// 832849F0: 386AE540  addi r3, r10, -0x1ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -6848;
	// 832849F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832849F8: 4AFA84D9  bl 0x8222ced0
	ctx.lr = 0x832849FC;
	sub_8222CED0(ctx, base);
	// 832849FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284A00: 386915E8  addi r3, r9, 0x15e8
	ctx.r[3].s64 = ctx.r[9].s64 + 5608;
	// 83284A04: 4BA2551D  bl 0x82ca9f20
	ctx.lr = 0x83284A08;
	sub_82CA9F20(ctx, base);
	// 83284A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284A18 size=64
    let mut pc: u32 = 0x83284A18;
    'dispatch: loop {
        match pc {
            0x83284A18 => {
    //   block [0x83284A18..0x83284A58)
	// 83284A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284A24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284A28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284A2C: 388BB288  addi r4, r11, -0x4d78
	ctx.r[4].s64 = ctx.r[11].s64 + -19832;
	// 83284A30: 386AE544  addi r3, r10, -0x1abc
	ctx.r[3].s64 = ctx.r[10].s64 + -6844;
	// 83284A34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284A38: 4AFA8499  bl 0x8222ced0
	ctx.lr = 0x83284A3C;
	sub_8222CED0(ctx, base);
	// 83284A3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284A40: 386915F8  addi r3, r9, 0x15f8
	ctx.r[3].s64 = ctx.r[9].s64 + 5624;
	// 83284A44: 4BA254DD  bl 0x82ca9f20
	ctx.lr = 0x83284A48;
	sub_82CA9F20(ctx, base);
	// 83284A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284A58 size=64
    let mut pc: u32 = 0x83284A58;
    'dispatch: loop {
        match pc {
            0x83284A58 => {
    //   block [0x83284A58..0x83284A98)
	// 83284A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284A64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284A68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284A6C: 388BB29C  addi r4, r11, -0x4d64
	ctx.r[4].s64 = ctx.r[11].s64 + -19812;
	// 83284A70: 386AE548  addi r3, r10, -0x1ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -6840;
	// 83284A74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284A78: 4AFA8459  bl 0x8222ced0
	ctx.lr = 0x83284A7C;
	sub_8222CED0(ctx, base);
	// 83284A7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284A80: 38691608  addi r3, r9, 0x1608
	ctx.r[3].s64 = ctx.r[9].s64 + 5640;
	// 83284A84: 4BA2549D  bl 0x82ca9f20
	ctx.lr = 0x83284A88;
	sub_82CA9F20(ctx, base);
	// 83284A88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284A98 size=64
    let mut pc: u32 = 0x83284A98;
    'dispatch: loop {
        match pc {
            0x83284A98 => {
    //   block [0x83284A98..0x83284AD8)
	// 83284A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284AA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284AA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284AAC: 388BB2B0  addi r4, r11, -0x4d50
	ctx.r[4].s64 = ctx.r[11].s64 + -19792;
	// 83284AB0: 386AE54C  addi r3, r10, -0x1ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -6836;
	// 83284AB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284AB8: 4AFA8419  bl 0x8222ced0
	ctx.lr = 0x83284ABC;
	sub_8222CED0(ctx, base);
	// 83284ABC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284AC0: 38691618  addi r3, r9, 0x1618
	ctx.r[3].s64 = ctx.r[9].s64 + 5656;
	// 83284AC4: 4BA2545D  bl 0x82ca9f20
	ctx.lr = 0x83284AC8;
	sub_82CA9F20(ctx, base);
	// 83284AC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284AD8 size=64
    let mut pc: u32 = 0x83284AD8;
    'dispatch: loop {
        match pc {
            0x83284AD8 => {
    //   block [0x83284AD8..0x83284B18)
	// 83284AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284AE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284AE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284AEC: 388BB204  addi r4, r11, -0x4dfc
	ctx.r[4].s64 = ctx.r[11].s64 + -19964;
	// 83284AF0: 386AE550  addi r3, r10, -0x1ab0
	ctx.r[3].s64 = ctx.r[10].s64 + -6832;
	// 83284AF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284AF8: 4AFA83D9  bl 0x8222ced0
	ctx.lr = 0x83284AFC;
	sub_8222CED0(ctx, base);
	// 83284AFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284B00: 38691788  addi r3, r9, 0x1788
	ctx.r[3].s64 = ctx.r[9].s64 + 6024;
	// 83284B04: 4BA2541D  bl 0x82ca9f20
	ctx.lr = 0x83284B08;
	sub_82CA9F20(ctx, base);
	// 83284B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284B18 size=64
    let mut pc: u32 = 0x83284B18;
    'dispatch: loop {
        match pc {
            0x83284B18 => {
    //   block [0x83284B18..0x83284B58)
	// 83284B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284B24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284B2C: 388BB20C  addi r4, r11, -0x4df4
	ctx.r[4].s64 = ctx.r[11].s64 + -19956;
	// 83284B30: 386AE554  addi r3, r10, -0x1aac
	ctx.r[3].s64 = ctx.r[10].s64 + -6828;
	// 83284B34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284B38: 4AFA8399  bl 0x8222ced0
	ctx.lr = 0x83284B3C;
	sub_8222CED0(ctx, base);
	// 83284B3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284B40: 38691798  addi r3, r9, 0x1798
	ctx.r[3].s64 = ctx.r[9].s64 + 6040;
	// 83284B44: 4BA253DD  bl 0x82ca9f20
	ctx.lr = 0x83284B48;
	sub_82CA9F20(ctx, base);
	// 83284B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284B58 size=64
    let mut pc: u32 = 0x83284B58;
    'dispatch: loop {
        match pc {
            0x83284B58 => {
    //   block [0x83284B58..0x83284B98)
	// 83284B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284B64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284B6C: 388BB220  addi r4, r11, -0x4de0
	ctx.r[4].s64 = ctx.r[11].s64 + -19936;
	// 83284B70: 386AE558  addi r3, r10, -0x1aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -6824;
	// 83284B74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284B78: 4AFA8359  bl 0x8222ced0
	ctx.lr = 0x83284B7C;
	sub_8222CED0(ctx, base);
	// 83284B7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284B80: 386917A8  addi r3, r9, 0x17a8
	ctx.r[3].s64 = ctx.r[9].s64 + 6056;
	// 83284B84: 4BA2539D  bl 0x82ca9f20
	ctx.lr = 0x83284B88;
	sub_82CA9F20(ctx, base);
	// 83284B88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284B98 size=64
    let mut pc: u32 = 0x83284B98;
    'dispatch: loop {
        match pc {
            0x83284B98 => {
    //   block [0x83284B98..0x83284BD8)
	// 83284B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284BA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284BA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284BAC: 388BB234  addi r4, r11, -0x4dcc
	ctx.r[4].s64 = ctx.r[11].s64 + -19916;
	// 83284BB0: 386AE55C  addi r3, r10, -0x1aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -6820;
	// 83284BB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284BB8: 4AFA8319  bl 0x8222ced0
	ctx.lr = 0x83284BBC;
	sub_8222CED0(ctx, base);
	// 83284BBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284BC0: 386917B8  addi r3, r9, 0x17b8
	ctx.r[3].s64 = ctx.r[9].s64 + 6072;
	// 83284BC4: 4BA2535D  bl 0x82ca9f20
	ctx.lr = 0x83284BC8;
	sub_82CA9F20(ctx, base);
	// 83284BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284BD8 size=64
    let mut pc: u32 = 0x83284BD8;
    'dispatch: loop {
        match pc {
            0x83284BD8 => {
    //   block [0x83284BD8..0x83284C18)
	// 83284BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284BE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284BEC: 388BB260  addi r4, r11, -0x4da0
	ctx.r[4].s64 = ctx.r[11].s64 + -19872;
	// 83284BF0: 386AE560  addi r3, r10, -0x1aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -6816;
	// 83284BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284BF8: 4AFA82D9  bl 0x8222ced0
	ctx.lr = 0x83284BFC;
	sub_8222CED0(ctx, base);
	// 83284BFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284C00: 386917C8  addi r3, r9, 0x17c8
	ctx.r[3].s64 = ctx.r[9].s64 + 6088;
	// 83284C04: 4BA2531D  bl 0x82ca9f20
	ctx.lr = 0x83284C08;
	sub_82CA9F20(ctx, base);
	// 83284C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284C18 size=64
    let mut pc: u32 = 0x83284C18;
    'dispatch: loop {
        match pc {
            0x83284C18 => {
    //   block [0x83284C18..0x83284C58)
	// 83284C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284C24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284C28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284C2C: 388BB278  addi r4, r11, -0x4d88
	ctx.r[4].s64 = ctx.r[11].s64 + -19848;
	// 83284C30: 386AE564  addi r3, r10, -0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -6812;
	// 83284C34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284C38: 4AFA8299  bl 0x8222ced0
	ctx.lr = 0x83284C3C;
	sub_8222CED0(ctx, base);
	// 83284C3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284C40: 386917D8  addi r3, r9, 0x17d8
	ctx.r[3].s64 = ctx.r[9].s64 + 6104;
	// 83284C44: 4BA252DD  bl 0x82ca9f20
	ctx.lr = 0x83284C48;
	sub_82CA9F20(ctx, base);
	// 83284C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284C58 size=52
    let mut pc: u32 = 0x83284C58;
    'dispatch: loop {
        match pc {
            0x83284C58 => {
    //   block [0x83284C58..0x83284C8C)
	// 83284C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284C64: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83284C68: 386BE568  addi r3, r11, -0x1a98
	ctx.r[3].s64 = ctx.r[11].s64 + -6808;
	// 83284C6C: 4BFBC38D  bl 0x83240ff8
	ctx.lr = 0x83284C70;
	sub_83240FF8(ctx, base);
	// 83284C70: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83284C74: 386A17E8  addi r3, r10, 0x17e8
	ctx.r[3].s64 = ctx.r[10].s64 + 6120;
	// 83284C78: 4BA252A9  bl 0x82ca9f20
	ctx.lr = 0x83284C7C;
	sub_82CA9F20(ctx, base);
	// 83284C7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284C90 size=64
    let mut pc: u32 = 0x83284C90;
    'dispatch: loop {
        match pc {
            0x83284C90 => {
    //   block [0x83284C90..0x83284CD0)
	// 83284C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284C9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284CA4: 388BB15C  addi r4, r11, -0x4ea4
	ctx.r[4].s64 = ctx.r[11].s64 + -20132;
	// 83284CA8: 386AE578  addi r3, r10, -0x1a88
	ctx.r[3].s64 = ctx.r[10].s64 + -6792;
	// 83284CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284CB0: 4AFA8221  bl 0x8222ced0
	ctx.lr = 0x83284CB4;
	sub_8222CED0(ctx, base);
	// 83284CB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284CB8: 38691830  addi r3, r9, 0x1830
	ctx.r[3].s64 = ctx.r[9].s64 + 6192;
	// 83284CBC: 4BA25265  bl 0x82ca9f20
	ctx.lr = 0x83284CC0;
	sub_82CA9F20(ctx, base);
	// 83284CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284CD0 size=64
    let mut pc: u32 = 0x83284CD0;
    'dispatch: loop {
        match pc {
            0x83284CD0 => {
    //   block [0x83284CD0..0x83284D10)
	// 83284CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284CDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284CE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284CE4: 388BB164  addi r4, r11, -0x4e9c
	ctx.r[4].s64 = ctx.r[11].s64 + -20124;
	// 83284CE8: 386AE57C  addi r3, r10, -0x1a84
	ctx.r[3].s64 = ctx.r[10].s64 + -6788;
	// 83284CEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284CF0: 4AFA81E1  bl 0x8222ced0
	ctx.lr = 0x83284CF4;
	sub_8222CED0(ctx, base);
	// 83284CF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284CF8: 38691840  addi r3, r9, 0x1840
	ctx.r[3].s64 = ctx.r[9].s64 + 6208;
	// 83284CFC: 4BA25225  bl 0x82ca9f20
	ctx.lr = 0x83284D00;
	sub_82CA9F20(ctx, base);
	// 83284D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284D10 size=64
    let mut pc: u32 = 0x83284D10;
    'dispatch: loop {
        match pc {
            0x83284D10 => {
    //   block [0x83284D10..0x83284D50)
	// 83284D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284D1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284D24: 388BB178  addi r4, r11, -0x4e88
	ctx.r[4].s64 = ctx.r[11].s64 + -20104;
	// 83284D28: 386AE580  addi r3, r10, -0x1a80
	ctx.r[3].s64 = ctx.r[10].s64 + -6784;
	// 83284D2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284D30: 4AFA81A1  bl 0x8222ced0
	ctx.lr = 0x83284D34;
	sub_8222CED0(ctx, base);
	// 83284D34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284D38: 38691850  addi r3, r9, 0x1850
	ctx.r[3].s64 = ctx.r[9].s64 + 6224;
	// 83284D3C: 4BA251E5  bl 0x82ca9f20
	ctx.lr = 0x83284D40;
	sub_82CA9F20(ctx, base);
	// 83284D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284D50 size=64
    let mut pc: u32 = 0x83284D50;
    'dispatch: loop {
        match pc {
            0x83284D50 => {
    //   block [0x83284D50..0x83284D90)
	// 83284D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284D5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284D64: 388BB18C  addi r4, r11, -0x4e74
	ctx.r[4].s64 = ctx.r[11].s64 + -20084;
	// 83284D68: 386AE584  addi r3, r10, -0x1a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -6780;
	// 83284D6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284D70: 4AFA8161  bl 0x8222ced0
	ctx.lr = 0x83284D74;
	sub_8222CED0(ctx, base);
	// 83284D74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284D78: 38691860  addi r3, r9, 0x1860
	ctx.r[3].s64 = ctx.r[9].s64 + 6240;
	// 83284D7C: 4BA251A5  bl 0x82ca9f20
	ctx.lr = 0x83284D80;
	sub_82CA9F20(ctx, base);
	// 83284D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284D90 size=64
    let mut pc: u32 = 0x83284D90;
    'dispatch: loop {
        match pc {
            0x83284D90 => {
    //   block [0x83284D90..0x83284DD0)
	// 83284D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284D98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284D9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284DA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284DA4: 388BB1A8  addi r4, r11, -0x4e58
	ctx.r[4].s64 = ctx.r[11].s64 + -20056;
	// 83284DA8: 386AE588  addi r3, r10, -0x1a78
	ctx.r[3].s64 = ctx.r[10].s64 + -6776;
	// 83284DAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284DB0: 4AFA8121  bl 0x8222ced0
	ctx.lr = 0x83284DB4;
	sub_8222CED0(ctx, base);
	// 83284DB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284DB8: 38691870  addi r3, r9, 0x1870
	ctx.r[3].s64 = ctx.r[9].s64 + 6256;
	// 83284DBC: 4BA25165  bl 0x82ca9f20
	ctx.lr = 0x83284DC0;
	sub_82CA9F20(ctx, base);
	// 83284DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284DD0 size=64
    let mut pc: u32 = 0x83284DD0;
    'dispatch: loop {
        match pc {
            0x83284DD0 => {
    //   block [0x83284DD0..0x83284E10)
	// 83284DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284DE4: 388BB1B8  addi r4, r11, -0x4e48
	ctx.r[4].s64 = ctx.r[11].s64 + -20040;
	// 83284DE8: 386AE58C  addi r3, r10, -0x1a74
	ctx.r[3].s64 = ctx.r[10].s64 + -6772;
	// 83284DEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284DF0: 4AFA80E1  bl 0x8222ced0
	ctx.lr = 0x83284DF4;
	sub_8222CED0(ctx, base);
	// 83284DF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284DF8: 38691880  addi r3, r9, 0x1880
	ctx.r[3].s64 = ctx.r[9].s64 + 6272;
	// 83284DFC: 4BA25125  bl 0x82ca9f20
	ctx.lr = 0x83284E00;
	sub_82CA9F20(ctx, base);
	// 83284E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284E10 size=64
    let mut pc: u32 = 0x83284E10;
    'dispatch: loop {
        match pc {
            0x83284E10 => {
    //   block [0x83284E10..0x83284E50)
	// 83284E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284E1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284E20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284E24: 388BB1CC  addi r4, r11, -0x4e34
	ctx.r[4].s64 = ctx.r[11].s64 + -20020;
	// 83284E28: 386AE590  addi r3, r10, -0x1a70
	ctx.r[3].s64 = ctx.r[10].s64 + -6768;
	// 83284E2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284E30: 4AFA80A1  bl 0x8222ced0
	ctx.lr = 0x83284E34;
	sub_8222CED0(ctx, base);
	// 83284E34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284E38: 38691890  addi r3, r9, 0x1890
	ctx.r[3].s64 = ctx.r[9].s64 + 6288;
	// 83284E3C: 4BA250E5  bl 0x82ca9f20
	ctx.lr = 0x83284E40;
	sub_82CA9F20(ctx, base);
	// 83284E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284E50 size=64
    let mut pc: u32 = 0x83284E50;
    'dispatch: loop {
        match pc {
            0x83284E50 => {
    //   block [0x83284E50..0x83284E90)
	// 83284E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284E5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284E64: 388BB1D4  addi r4, r11, -0x4e2c
	ctx.r[4].s64 = ctx.r[11].s64 + -20012;
	// 83284E68: 386AE594  addi r3, r10, -0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -6764;
	// 83284E6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284E70: 4AFA8061  bl 0x8222ced0
	ctx.lr = 0x83284E74;
	sub_8222CED0(ctx, base);
	// 83284E74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284E78: 386918A0  addi r3, r9, 0x18a0
	ctx.r[3].s64 = ctx.r[9].s64 + 6304;
	// 83284E7C: 4BA250A5  bl 0x82ca9f20
	ctx.lr = 0x83284E80;
	sub_82CA9F20(ctx, base);
	// 83284E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284E90 size=64
    let mut pc: u32 = 0x83284E90;
    'dispatch: loop {
        match pc {
            0x83284E90 => {
    //   block [0x83284E90..0x83284ED0)
	// 83284E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284E9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284EA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284EA4: 388BB1E0  addi r4, r11, -0x4e20
	ctx.r[4].s64 = ctx.r[11].s64 + -20000;
	// 83284EA8: 386AE598  addi r3, r10, -0x1a68
	ctx.r[3].s64 = ctx.r[10].s64 + -6760;
	// 83284EAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284EB0: 4AFA8021  bl 0x8222ced0
	ctx.lr = 0x83284EB4;
	sub_8222CED0(ctx, base);
	// 83284EB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284EB8: 386918B0  addi r3, r9, 0x18b0
	ctx.r[3].s64 = ctx.r[9].s64 + 6320;
	// 83284EBC: 4BA25065  bl 0x82ca9f20
	ctx.lr = 0x83284EC0;
	sub_82CA9F20(ctx, base);
	// 83284EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284ED0 size=64
    let mut pc: u32 = 0x83284ED0;
    'dispatch: loop {
        match pc {
            0x83284ED0 => {
    //   block [0x83284ED0..0x83284F10)
	// 83284ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284EDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284EE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284EE4: 388BB1F4  addi r4, r11, -0x4e0c
	ctx.r[4].s64 = ctx.r[11].s64 + -19980;
	// 83284EE8: 386AE59C  addi r3, r10, -0x1a64
	ctx.r[3].s64 = ctx.r[10].s64 + -6756;
	// 83284EEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284EF0: 4AFA7FE1  bl 0x8222ced0
	ctx.lr = 0x83284EF4;
	sub_8222CED0(ctx, base);
	// 83284EF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284EF8: 386918C0  addi r3, r9, 0x18c0
	ctx.r[3].s64 = ctx.r[9].s64 + 6336;
	// 83284EFC: 4BA25025  bl 0x82ca9f20
	ctx.lr = 0x83284F00;
	sub_82CA9F20(ctx, base);
	// 83284F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284F10 size=64
    let mut pc: u32 = 0x83284F10;
    'dispatch: loop {
        match pc {
            0x83284F10 => {
    //   block [0x83284F10..0x83284F50)
	// 83284F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284F1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284F24: 388BB110  addi r4, r11, -0x4ef0
	ctx.r[4].s64 = ctx.r[11].s64 + -20208;
	// 83284F28: 386AE5A0  addi r3, r10, -0x1a60
	ctx.r[3].s64 = ctx.r[10].s64 + -6752;
	// 83284F2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284F30: 4AFA7FA1  bl 0x8222ced0
	ctx.lr = 0x83284F34;
	sub_8222CED0(ctx, base);
	// 83284F34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284F38: 386918D0  addi r3, r9, 0x18d0
	ctx.r[3].s64 = ctx.r[9].s64 + 6352;
	// 83284F3C: 4BA24FE5  bl 0x82ca9f20
	ctx.lr = 0x83284F40;
	sub_82CA9F20(ctx, base);
	// 83284F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284F50 size=64
    let mut pc: u32 = 0x83284F50;
    'dispatch: loop {
        match pc {
            0x83284F50 => {
    //   block [0x83284F50..0x83284F90)
	// 83284F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284F5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284F60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284F64: 388BB118  addi r4, r11, -0x4ee8
	ctx.r[4].s64 = ctx.r[11].s64 + -20200;
	// 83284F68: 386AE5A4  addi r3, r10, -0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -6748;
	// 83284F6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284F70: 4AFA7F61  bl 0x8222ced0
	ctx.lr = 0x83284F74;
	sub_8222CED0(ctx, base);
	// 83284F74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284F78: 386918E0  addi r3, r9, 0x18e0
	ctx.r[3].s64 = ctx.r[9].s64 + 6368;
	// 83284F7C: 4BA24FA5  bl 0x82ca9f20
	ctx.lr = 0x83284F80;
	sub_82CA9F20(ctx, base);
	// 83284F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284F90 size=64
    let mut pc: u32 = 0x83284F90;
    'dispatch: loop {
        match pc {
            0x83284F90 => {
    //   block [0x83284F90..0x83284FD0)
	// 83284F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284F9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284FA4: 388BB12C  addi r4, r11, -0x4ed4
	ctx.r[4].s64 = ctx.r[11].s64 + -20180;
	// 83284FA8: 386AE5A8  addi r3, r10, -0x1a58
	ctx.r[3].s64 = ctx.r[10].s64 + -6744;
	// 83284FAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284FB0: 4AFA7F21  bl 0x8222ced0
	ctx.lr = 0x83284FB4;
	sub_8222CED0(ctx, base);
	// 83284FB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284FB8: 386918F0  addi r3, r9, 0x18f0
	ctx.r[3].s64 = ctx.r[9].s64 + 6384;
	// 83284FBC: 4BA24F65  bl 0x82ca9f20
	ctx.lr = 0x83284FC0;
	sub_82CA9F20(ctx, base);
	// 83284FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83284FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83284FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83284FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83284FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83284FD0 size=64
    let mut pc: u32 = 0x83284FD0;
    'dispatch: loop {
        match pc {
            0x83284FD0 => {
    //   block [0x83284FD0..0x83285010)
	// 83284FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83284FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83284FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83284FDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83284FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83284FE4: 388BB140  addi r4, r11, -0x4ec0
	ctx.r[4].s64 = ctx.r[11].s64 + -20160;
	// 83284FE8: 386AE5AC  addi r3, r10, -0x1a54
	ctx.r[3].s64 = ctx.r[10].s64 + -6740;
	// 83284FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83284FF0: 4AFA7EE1  bl 0x8222ced0
	ctx.lr = 0x83284FF4;
	sub_8222CED0(ctx, base);
	// 83284FF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83284FF8: 38691900  addi r3, r9, 0x1900
	ctx.r[3].s64 = ctx.r[9].s64 + 6400;
	// 83284FFC: 4BA24F25  bl 0x82ca9f20
	ctx.lr = 0x83285000;
	sub_82CA9F20(ctx, base);
	// 83285000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328500C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285010 size=64
    let mut pc: u32 = 0x83285010;
    'dispatch: loop {
        match pc {
            0x83285010 => {
    //   block [0x83285010..0x83285050)
	// 83285010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328501C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285024: 388BB0B0  addi r4, r11, -0x4f50
	ctx.r[4].s64 = ctx.r[11].s64 + -20304;
	// 83285028: 386AE5B0  addi r3, r10, -0x1a50
	ctx.r[3].s64 = ctx.r[10].s64 + -6736;
	// 8328502C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285030: 4AFA7EA1  bl 0x8222ced0
	ctx.lr = 0x83285034;
	sub_8222CED0(ctx, base);
	// 83285034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285038: 38691910  addi r3, r9, 0x1910
	ctx.r[3].s64 = ctx.r[9].s64 + 6416;
	// 8328503C: 4BA24EE5  bl 0x82ca9f20
	ctx.lr = 0x83285040;
	sub_82CA9F20(ctx, base);
	// 83285040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328504C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285050 size=64
    let mut pc: u32 = 0x83285050;
    'dispatch: loop {
        match pc {
            0x83285050 => {
    //   block [0x83285050..0x83285090)
	// 83285050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328505C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285064: 388BB0B8  addi r4, r11, -0x4f48
	ctx.r[4].s64 = ctx.r[11].s64 + -20296;
	// 83285068: 386AE5B4  addi r3, r10, -0x1a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -6732;
	// 8328506C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285070: 4AFA7E61  bl 0x8222ced0
	ctx.lr = 0x83285074;
	sub_8222CED0(ctx, base);
	// 83285074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285078: 38691920  addi r3, r9, 0x1920
	ctx.r[3].s64 = ctx.r[9].s64 + 6432;
	// 8328507C: 4BA24EA5  bl 0x82ca9f20
	ctx.lr = 0x83285080;
	sub_82CA9F20(ctx, base);
	// 83285080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328508C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285090 size=64
    let mut pc: u32 = 0x83285090;
    'dispatch: loop {
        match pc {
            0x83285090 => {
    //   block [0x83285090..0x832850D0)
	// 83285090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328509C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832850A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832850A4: 388BB0CC  addi r4, r11, -0x4f34
	ctx.r[4].s64 = ctx.r[11].s64 + -20276;
	// 832850A8: 386AE5B8  addi r3, r10, -0x1a48
	ctx.r[3].s64 = ctx.r[10].s64 + -6728;
	// 832850AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832850B0: 4AFA7E21  bl 0x8222ced0
	ctx.lr = 0x832850B4;
	sub_8222CED0(ctx, base);
	// 832850B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832850B8: 38691930  addi r3, r9, 0x1930
	ctx.r[3].s64 = ctx.r[9].s64 + 6448;
	// 832850BC: 4BA24E65  bl 0x82ca9f20
	ctx.lr = 0x832850C0;
	sub_82CA9F20(ctx, base);
	// 832850C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832850C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832850C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832850CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


