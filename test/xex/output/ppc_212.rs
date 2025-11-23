pub fn sub_832563F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832563F0 size=56
    let mut pc: u32 = 0x832563F0;
    'dispatch: loop {
        match pc {
            0x832563F0 => {
    //   block [0x832563F0..0x83256428)
	// 832563F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832563F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832563F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832563FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256404: 386B1068  addi r3, r11, 0x1068
	ctx.r[3].s64 = ctx.r[11].s64 + 4200;
	// 83256408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325640C: 4AF9D94D  bl 0x821f3d58
	ctx.lr = 0x83256410;
	sub_821F3D58(ctx, base);
	// 83256410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256414: 906A873C  stw r3, -0x78c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30916 as u32), ctx.r[3].u32 ) };
	// 83256418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256428 size=56
    let mut pc: u32 = 0x83256428;
    'dispatch: loop {
        match pc {
            0x83256428 => {
    //   block [0x83256428..0x83256460)
	// 83256428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256434: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83256438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325643C: 386BABCC  addi r3, r11, -0x5434
	ctx.r[3].s64 = ctx.r[11].s64 + -21556;
	// 83256440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256444: 4AF9D915  bl 0x821f3d58
	ctx.lr = 0x83256448;
	sub_821F3D58(ctx, base);
	// 83256448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325644C: 906A8740  stw r3, -0x78c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30912 as u32), ctx.r[3].u32 ) };
	// 83256450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325645C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256460 size=56
    let mut pc: u32 = 0x83256460;
    'dispatch: loop {
        match pc {
            0x83256460 => {
    //   block [0x83256460..0x83256498)
	// 83256460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325646C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83256470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256474: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 83256478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325647C: 4AF9D8DD  bl 0x821f3d58
	ctx.lr = 0x83256480;
	sub_821F3D58(ctx, base);
	// 83256480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256484: 906A8744  stw r3, -0x78bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30908 as u32), ctx.r[3].u32 ) };
	// 83256488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325648C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256498 size=56
    let mut pc: u32 = 0x83256498;
    'dispatch: loop {
        match pc {
            0x83256498 => {
    //   block [0x83256498..0x832564D0)
	// 83256498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832564A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832564A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832564A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832564AC: 386BABD8  addi r3, r11, -0x5428
	ctx.r[3].s64 = ctx.r[11].s64 + -21544;
	// 832564B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832564B4: 4AF9D8A5  bl 0x821f3d58
	ctx.lr = 0x832564B8;
	sub_821F3D58(ctx, base);
	// 832564B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832564BC: 906A8748  stw r3, -0x78b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30904 as u32), ctx.r[3].u32 ) };
	// 832564C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832564C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832564C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832564CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832564D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832564D0 size=56
    let mut pc: u32 = 0x832564D0;
    'dispatch: loop {
        match pc {
            0x832564D0 => {
    //   block [0x832564D0..0x83256508)
	// 832564D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832564D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832564D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832564DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832564E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832564E4: 386B1074  addi r3, r11, 0x1074
	ctx.r[3].s64 = ctx.r[11].s64 + 4212;
	// 832564E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832564EC: 4AF9D86D  bl 0x821f3d58
	ctx.lr = 0x832564F0;
	sub_821F3D58(ctx, base);
	// 832564F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832564F4: 906A874C  stw r3, -0x78b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30900 as u32), ctx.r[3].u32 ) };
	// 832564F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832564FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256508 size=56
    let mut pc: u32 = 0x83256508;
    'dispatch: loop {
        match pc {
            0x83256508 => {
    //   block [0x83256508..0x83256540)
	// 83256508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256514: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325651C: 386B1080  addi r3, r11, 0x1080
	ctx.r[3].s64 = ctx.r[11].s64 + 4224;
	// 83256520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256524: 4AF9D835  bl 0x821f3d58
	ctx.lr = 0x83256528;
	sub_821F3D58(ctx, base);
	// 83256528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325652C: 906A8750  stw r3, -0x78b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30896 as u32), ctx.r[3].u32 ) };
	// 83256530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325653C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256540 size=56
    let mut pc: u32 = 0x83256540;
    'dispatch: loop {
        match pc {
            0x83256540 => {
    //   block [0x83256540..0x83256578)
	// 83256540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325654C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256550: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256554: 386B108C  addi r3, r11, 0x108c
	ctx.r[3].s64 = ctx.r[11].s64 + 4236;
	// 83256558: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325655C: 4AF9D7FD  bl 0x821f3d58
	ctx.lr = 0x83256560;
	sub_821F3D58(ctx, base);
	// 83256560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256564: 906A8754  stw r3, -0x78ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30892 as u32), ctx.r[3].u32 ) };
	// 83256568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256578 size=56
    let mut pc: u32 = 0x83256578;
    'dispatch: loop {
        match pc {
            0x83256578 => {
    //   block [0x83256578..0x832565B0)
	// 83256578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256584: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256588: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325658C: 386B109C  addi r3, r11, 0x109c
	ctx.r[3].s64 = ctx.r[11].s64 + 4252;
	// 83256590: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256594: 4AF9D7C5  bl 0x821f3d58
	ctx.lr = 0x83256598;
	sub_821F3D58(ctx, base);
	// 83256598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325659C: 906A8758  stw r3, -0x78a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30888 as u32), ctx.r[3].u32 ) };
	// 832565A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832565A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832565A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832565AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832565B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832565B0 size=56
    let mut pc: u32 = 0x832565B0;
    'dispatch: loop {
        match pc {
            0x832565B0 => {
    //   block [0x832565B0..0x832565E8)
	// 832565B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832565B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832565B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832565BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832565C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832565C4: 386B10AC  addi r3, r11, 0x10ac
	ctx.r[3].s64 = ctx.r[11].s64 + 4268;
	// 832565C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832565CC: 4AF9D78D  bl 0x821f3d58
	ctx.lr = 0x832565D0;
	sub_821F3D58(ctx, base);
	// 832565D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832565D4: 906A875C  stw r3, -0x78a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30884 as u32), ctx.r[3].u32 ) };
	// 832565D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832565DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832565E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832565E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832565E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832565E8 size=56
    let mut pc: u32 = 0x832565E8;
    'dispatch: loop {
        match pc {
            0x832565E8 => {
    //   block [0x832565E8..0x83256620)
	// 832565E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832565EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832565F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832565F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832565F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832565FC: 386B10B8  addi r3, r11, 0x10b8
	ctx.r[3].s64 = ctx.r[11].s64 + 4280;
	// 83256600: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256604: 4AF9D755  bl 0x821f3d58
	ctx.lr = 0x83256608;
	sub_821F3D58(ctx, base);
	// 83256608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325660C: 906A8760  stw r3, -0x78a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30880 as u32), ctx.r[3].u32 ) };
	// 83256610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325661C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256620 size=56
    let mut pc: u32 = 0x83256620;
    'dispatch: loop {
        match pc {
            0x83256620 => {
    //   block [0x83256620..0x83256658)
	// 83256620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325662C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256630: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256634: 386B10D0  addi r3, r11, 0x10d0
	ctx.r[3].s64 = ctx.r[11].s64 + 4304;
	// 83256638: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325663C: 4AF9D71D  bl 0x821f3d58
	ctx.lr = 0x83256640;
	sub_821F3D58(ctx, base);
	// 83256640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256644: 906A8764  stw r3, -0x789c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30876 as u32), ctx.r[3].u32 ) };
	// 83256648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325664C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256658 size=56
    let mut pc: u32 = 0x83256658;
    'dispatch: loop {
        match pc {
            0x83256658 => {
    //   block [0x83256658..0x83256690)
	// 83256658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256664: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256668: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325666C: 386B10E4  addi r3, r11, 0x10e4
	ctx.r[3].s64 = ctx.r[11].s64 + 4324;
	// 83256670: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256674: 4AF9D6E5  bl 0x821f3d58
	ctx.lr = 0x83256678;
	sub_821F3D58(ctx, base);
	// 83256678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325667C: 906A8768  stw r3, -0x7898(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30872 as u32), ctx.r[3].u32 ) };
	// 83256680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325668C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256690 size=64
    let mut pc: u32 = 0x83256690;
    'dispatch: loop {
        match pc {
            0x83256690 => {
    //   block [0x83256690..0x832566D0)
	// 83256690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325669C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832566A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832566A4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832566A8: 386A876C  addi r3, r10, -0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + -30868;
	// 832566AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832566B0: 4AFD6821  bl 0x8222ced0
	ctx.lr = 0x832566B4;
	sub_8222CED0(ctx, base);
	// 832566B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832566B8: 3869A888  addi r3, r9, -0x5778
	ctx.r[3].s64 = ctx.r[9].s64 + -22392;
	// 832566BC: 4BA53865  bl 0x82ca9f20
	ctx.lr = 0x832566C0;
	sub_82CA9F20(ctx, base);
	// 832566C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832566C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832566C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832566CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832566D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832566D0 size=64
    let mut pc: u32 = 0x832566D0;
    'dispatch: loop {
        match pc {
            0x832566D0 => {
    //   block [0x832566D0..0x83256710)
	// 832566D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832566D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832566D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832566DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832566E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832566E4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832566E8: 386A8770  addi r3, r10, -0x7890
	ctx.r[3].s64 = ctx.r[10].s64 + -30864;
	// 832566EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832566F0: 4AFD67E1  bl 0x8222ced0
	ctx.lr = 0x832566F4;
	sub_8222CED0(ctx, base);
	// 832566F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832566F8: 3869A898  addi r3, r9, -0x5768
	ctx.r[3].s64 = ctx.r[9].s64 + -22376;
	// 832566FC: 4BA53825  bl 0x82ca9f20
	ctx.lr = 0x83256700;
	sub_82CA9F20(ctx, base);
	// 83256700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325670C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256710 size=64
    let mut pc: u32 = 0x83256710;
    'dispatch: loop {
        match pc {
            0x83256710 => {
    //   block [0x83256710..0x83256750)
	// 83256710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325671C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256724: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256728: 386A8774  addi r3, r10, -0x788c
	ctx.r[3].s64 = ctx.r[10].s64 + -30860;
	// 8325672C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256730: 4AFD67A1  bl 0x8222ced0
	ctx.lr = 0x83256734;
	sub_8222CED0(ctx, base);
	// 83256734: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256738: 3869A8A8  addi r3, r9, -0x5758
	ctx.r[3].s64 = ctx.r[9].s64 + -22360;
	// 8325673C: 4BA537E5  bl 0x82ca9f20
	ctx.lr = 0x83256740;
	sub_82CA9F20(ctx, base);
	// 83256740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325674C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256750 size=64
    let mut pc: u32 = 0x83256750;
    'dispatch: loop {
        match pc {
            0x83256750 => {
    //   block [0x83256750..0x83256790)
	// 83256750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325675C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256760: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256764: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256768: 386A8778  addi r3, r10, -0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + -30856;
	// 8325676C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256770: 4AFD6761  bl 0x8222ced0
	ctx.lr = 0x83256774;
	sub_8222CED0(ctx, base);
	// 83256774: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256778: 3869A8B8  addi r3, r9, -0x5748
	ctx.r[3].s64 = ctx.r[9].s64 + -22344;
	// 8325677C: 4BA537A5  bl 0x82ca9f20
	ctx.lr = 0x83256780;
	sub_82CA9F20(ctx, base);
	// 83256780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325678C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256790 size=64
    let mut pc: u32 = 0x83256790;
    'dispatch: loop {
        match pc {
            0x83256790 => {
    //   block [0x83256790..0x832567D0)
	// 83256790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325679C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832567A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832567A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832567A8: 386A877C  addi r3, r10, -0x7884
	ctx.r[3].s64 = ctx.r[10].s64 + -30852;
	// 832567AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832567B0: 4AFD6721  bl 0x8222ced0
	ctx.lr = 0x832567B4;
	sub_8222CED0(ctx, base);
	// 832567B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832567B8: 3869A8C8  addi r3, r9, -0x5738
	ctx.r[3].s64 = ctx.r[9].s64 + -22328;
	// 832567BC: 4BA53765  bl 0x82ca9f20
	ctx.lr = 0x832567C0;
	sub_82CA9F20(ctx, base);
	// 832567C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832567C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832567C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832567CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832567D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832567D0 size=64
    let mut pc: u32 = 0x832567D0;
    'dispatch: loop {
        match pc {
            0x832567D0 => {
    //   block [0x832567D0..0x83256810)
	// 832567D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832567D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832567D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832567DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832567E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832567E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832567E8: 386A8780  addi r3, r10, -0x7880
	ctx.r[3].s64 = ctx.r[10].s64 + -30848;
	// 832567EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832567F0: 4AFD66E1  bl 0x8222ced0
	ctx.lr = 0x832567F4;
	sub_8222CED0(ctx, base);
	// 832567F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832567F8: 3869A8D8  addi r3, r9, -0x5728
	ctx.r[3].s64 = ctx.r[9].s64 + -22312;
	// 832567FC: 4BA53725  bl 0x82ca9f20
	ctx.lr = 0x83256800;
	sub_82CA9F20(ctx, base);
	// 83256800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325680C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256810 size=52
    let mut pc: u32 = 0x83256810;
    'dispatch: loop {
        match pc {
            0x83256810 => {
    //   block [0x83256810..0x83256844)
	// 83256810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325681C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256820: 386B8784  addi r3, r11, -0x787c
	ctx.r[3].s64 = ctx.r[11].s64 + -30844;
	// 83256824: 4B7B3D8D  bl 0x82a0a5b0
	ctx.lr = 0x83256828;
	sub_82A0A5B0(ctx, base);
	// 83256828: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325682C: 386AA8E8  addi r3, r10, -0x5718
	ctx.r[3].s64 = ctx.r[10].s64 + -22296;
	// 83256830: 4BA536F1  bl 0x82ca9f20
	ctx.lr = 0x83256834;
	sub_82CA9F20(ctx, base);
	// 83256834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256848 size=52
    let mut pc: u32 = 0x83256848;
    'dispatch: loop {
        match pc {
            0x83256848 => {
    //   block [0x83256848..0x8325687C)
	// 83256848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256854: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256858: 386B8790  addi r3, r11, -0x7870
	ctx.r[3].s64 = ctx.r[11].s64 + -30832;
	// 8325685C: 4B7B3D55  bl 0x82a0a5b0
	ctx.lr = 0x83256860;
	sub_82A0A5B0(ctx, base);
	// 83256860: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256864: 386AA8F8  addi r3, r10, -0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + -22280;
	// 83256868: 4BA536B9  bl 0x82ca9f20
	ctx.lr = 0x8325686C;
	sub_82CA9F20(ctx, base);
	// 8325686C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256880 size=52
    let mut pc: u32 = 0x83256880;
    'dispatch: loop {
        match pc {
            0x83256880 => {
    //   block [0x83256880..0x832568B4)
	// 83256880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325688C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256890: 386B879C  addi r3, r11, -0x7864
	ctx.r[3].s64 = ctx.r[11].s64 + -30820;
	// 83256894: 4B7B3D1D  bl 0x82a0a5b0
	ctx.lr = 0x83256898;
	sub_82A0A5B0(ctx, base);
	// 83256898: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325689C: 386AA908  addi r3, r10, -0x56f8
	ctx.r[3].s64 = ctx.r[10].s64 + -22264;
	// 832568A0: 4BA53681  bl 0x82ca9f20
	ctx.lr = 0x832568A4;
	sub_82CA9F20(ctx, base);
	// 832568A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832568A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832568AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832568B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832568B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832568B8 size=20
    let mut pc: u32 = 0x832568B8;
    'dispatch: loop {
        match pc {
            0x832568B8 => {
    //   block [0x832568B8..0x832568CC)
	// 832568B8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832568BC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832568C0: C00B9F18  lfs f0, -0x60e8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832568C4: D00A87A8  stfs f0, -0x7858(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30808 as u32), tmp.u32 ) };
	// 832568C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832568D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832568D0 size=20
    let mut pc: u32 = 0x832568D0;
    'dispatch: loop {
        match pc {
            0x832568D0 => {
    //   block [0x832568D0..0x832568E4)
	// 832568D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832568D4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832568D8: C00B9F1C  lfs f0, -0x60e4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24804 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832568DC: D00A87AC  stfs f0, -0x7854(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30804 as u32), tmp.u32 ) };
	// 832568E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832568E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832568E8 size=20
    let mut pc: u32 = 0x832568E8;
    'dispatch: loop {
        match pc {
            0x832568E8 => {
    //   block [0x832568E8..0x832568FC)
	// 832568E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832568EC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832568F0: 394B87B0  addi r10, r11, -0x7850
	ctx.r[10].s64 = ctx.r[11].s64 + -30800;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256900 size=20
    let mut pc: u32 = 0x83256900;
    'dispatch: loop {
        match pc {
            0x83256900 => {
    //   block [0x83256900..0x83256914)
	// 83256900: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256904: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83256908: 394B87C0  addi r10, r11, -0x7840
	ctx.r[10].s64 = ctx.r[11].s64 + -30784;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256918 size=20
    let mut pc: u32 = 0x83256918;
    'dispatch: loop {
        match pc {
            0x83256918 => {
    //   block [0x83256918..0x8325692C)
	// 83256918: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325691C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83256920: 394B87D0  addi r10, r11, -0x7830
	ctx.r[10].s64 = ctx.r[11].s64 + -30768;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256930 size=20
    let mut pc: u32 = 0x83256930;
    'dispatch: loop {
        match pc {
            0x83256930 => {
    //   block [0x83256930..0x83256944)
	// 83256930: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256934: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83256938: 394B87E0  addi r10, r11, -0x7820
	ctx.r[10].s64 = ctx.r[11].s64 + -30752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256948 size=96
    let mut pc: u32 = 0x83256948;
    'dispatch: loop {
        match pc {
            0x83256948 => {
    //   block [0x83256948..0x8325696C)
	// 83256948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325694C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256954: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 83256958: 4AFC8901  bl 0x8221f258
	ctx.lr = 0x8325695C;
	sub_8221F258(ctx, base);
	// 8325695C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83256960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83256964: 419A0008  beq cr6, 0x8325696c
	if ctx.cr[6].eq {
	pc = 0x8325696C; continue 'dispatch;
	}
	// 83256968: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325696C; continue 'dispatch;
            }
            0x8325696C => {
    //   block [0x8325696C..0x83256978)
	// 8325696C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83256970: 41820008  beq 0x83256978
	if ctx.cr[0].eq {
	pc = 0x83256978; continue 'dispatch;
	}
	// 83256974: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83256978; continue 'dispatch;
            }
            0x83256978 => {
    //   block [0x83256978..0x832569A8)
	// 83256978: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325697C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83256980: 390987F0  addi r8, r9, -0x7810
	ctx.r[8].s64 = ctx.r[9].s64 + -30736;
	// 83256984: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83256988: 3867A918  addi r3, r7, -0x56e8
	ctx.r[3].s64 = ctx.r[7].s64 + -22248;
	// 8325698C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83256990: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83256994: 4BA5358D  bl 0x82ca9f20
	ctx.lr = 0x83256998;
	sub_82CA9F20(ctx, base);
	// 83256998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325699C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832569A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832569A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832569A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832569A8 size=12
    let mut pc: u32 = 0x832569A8;
    'dispatch: loop {
        match pc {
            0x832569A8 => {
    //   block [0x832569A8..0x832569B4)
	// 832569A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832569AC: 386BA9B0  addi r3, r11, -0x5650
	ctx.r[3].s64 = ctx.r[11].s64 + -22096;
	// 832569B0: 4BA53570  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832569B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832569B8 size=64
    let mut pc: u32 = 0x832569B8;
    'dispatch: loop {
        match pc {
            0x832569B8 => {
    //   block [0x832569B8..0x832569F8)
	// 832569B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832569BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832569C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832569C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832569C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832569CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832569D0: 386A880C  addi r3, r10, -0x77f4
	ctx.r[3].s64 = ctx.r[10].s64 + -30708;
	// 832569D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832569D8: 4AFD64F9  bl 0x8222ced0
	ctx.lr = 0x832569DC;
	sub_8222CED0(ctx, base);
	// 832569DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832569E0: 3869AA08  addi r3, r9, -0x55f8
	ctx.r[3].s64 = ctx.r[9].s64 + -22008;
	// 832569E4: 4BA5353D  bl 0x82ca9f20
	ctx.lr = 0x832569E8;
	sub_82CA9F20(ctx, base);
	// 832569E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832569EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832569F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832569F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832569F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832569F8 size=64
    let mut pc: u32 = 0x832569F8;
    'dispatch: loop {
        match pc {
            0x832569F8 => {
    //   block [0x832569F8..0x83256A38)
	// 832569F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832569FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256A0C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256A10: 386A8810  addi r3, r10, -0x77f0
	ctx.r[3].s64 = ctx.r[10].s64 + -30704;
	// 83256A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256A18: 4AFD64B9  bl 0x8222ced0
	ctx.lr = 0x83256A1C;
	sub_8222CED0(ctx, base);
	// 83256A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256A20: 3869AA18  addi r3, r9, -0x55e8
	ctx.r[3].s64 = ctx.r[9].s64 + -21992;
	// 83256A24: 4BA534FD  bl 0x82ca9f20
	ctx.lr = 0x83256A28;
	sub_82CA9F20(ctx, base);
	// 83256A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256A38 size=64
    let mut pc: u32 = 0x83256A38;
    'dispatch: loop {
        match pc {
            0x83256A38 => {
    //   block [0x83256A38..0x83256A78)
	// 83256A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256A4C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256A50: 386A8814  addi r3, r10, -0x77ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30700;
	// 83256A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256A58: 4AFD6479  bl 0x8222ced0
	ctx.lr = 0x83256A5C;
	sub_8222CED0(ctx, base);
	// 83256A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256A60: 3869AA28  addi r3, r9, -0x55d8
	ctx.r[3].s64 = ctx.r[9].s64 + -21976;
	// 83256A64: 4BA534BD  bl 0x82ca9f20
	ctx.lr = 0x83256A68;
	sub_82CA9F20(ctx, base);
	// 83256A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256A78 size=64
    let mut pc: u32 = 0x83256A78;
    'dispatch: loop {
        match pc {
            0x83256A78 => {
    //   block [0x83256A78..0x83256AB8)
	// 83256A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256A8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256A90: 386A8818  addi r3, r10, -0x77e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30696;
	// 83256A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256A98: 4AFD6439  bl 0x8222ced0
	ctx.lr = 0x83256A9C;
	sub_8222CED0(ctx, base);
	// 83256A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256AA0: 3869AA38  addi r3, r9, -0x55c8
	ctx.r[3].s64 = ctx.r[9].s64 + -21960;
	// 83256AA4: 4BA5347D  bl 0x82ca9f20
	ctx.lr = 0x83256AA8;
	sub_82CA9F20(ctx, base);
	// 83256AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256AB8 size=64
    let mut pc: u32 = 0x83256AB8;
    'dispatch: loop {
        match pc {
            0x83256AB8 => {
    //   block [0x83256AB8..0x83256AF8)
	// 83256AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256ACC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256AD0: 386A881C  addi r3, r10, -0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30692;
	// 83256AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256AD8: 4AFD63F9  bl 0x8222ced0
	ctx.lr = 0x83256ADC;
	sub_8222CED0(ctx, base);
	// 83256ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256AE0: 3869AA48  addi r3, r9, -0x55b8
	ctx.r[3].s64 = ctx.r[9].s64 + -21944;
	// 83256AE4: 4BA5343D  bl 0x82ca9f20
	ctx.lr = 0x83256AE8;
	sub_82CA9F20(ctx, base);
	// 83256AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256AF8 size=64
    let mut pc: u32 = 0x83256AF8;
    'dispatch: loop {
        match pc {
            0x83256AF8 => {
    //   block [0x83256AF8..0x83256B38)
	// 83256AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256B0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256B10: 386A8820  addi r3, r10, -0x77e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30688;
	// 83256B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256B18: 4AFD63B9  bl 0x8222ced0
	ctx.lr = 0x83256B1C;
	sub_8222CED0(ctx, base);
	// 83256B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256B20: 3869AA58  addi r3, r9, -0x55a8
	ctx.r[3].s64 = ctx.r[9].s64 + -21928;
	// 83256B24: 4BA533FD  bl 0x82ca9f20
	ctx.lr = 0x83256B28;
	sub_82CA9F20(ctx, base);
	// 83256B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256B38 size=44
    let mut pc: u32 = 0x83256B38;
    'dispatch: loop {
        match pc {
            0x83256B38 => {
    //   block [0x83256B38..0x83256B64)
	// 83256B38: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256B3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83256B40: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83256B44: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256B48: C9AA0D30  lfd f13, 0xd30(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3376 as u32) ) };
	// 83256B4C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83256B50: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256B54: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83256B58: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256B5C: 91698824  stw r11, -0x77dc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30684 as u32), ctx.r[11].u32 ) };
	// 83256B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256B68 size=44
    let mut pc: u32 = 0x83256B68;
    'dispatch: loop {
        match pc {
            0x83256B68 => {
    //   block [0x83256B68..0x83256B94)
	// 83256B68: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256B6C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83256B70: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83256B74: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256B78: C9AA1718  lfd f13, 0x1718(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(5912 as u32) ) };
	// 83256B7C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83256B80: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256B84: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83256B88: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256B8C: 91698828  stw r11, -0x77d8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30680 as u32), ctx.r[11].u32 ) };
	// 83256B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256B98 size=32
    let mut pc: u32 = 0x83256B98;
    'dispatch: loop {
        match pc {
            0x83256B98 => {
    //   block [0x83256B98..0x83256BB8)
	// 83256B98: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256B9C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256BA0: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256BA4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256BA8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 83256BAC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256BB0: 916A882C  stw r11, -0x77d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30676 as u32), ctx.r[11].u32 ) };
	// 83256BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256BB8 size=52
    let mut pc: u32 = 0x83256BB8;
    'dispatch: loop {
        match pc {
            0x83256BB8 => {
    //   block [0x83256BB8..0x83256BEC)
	// 83256BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256BC4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256BC8: 386B9F34  addi r3, r11, -0x60cc
	ctx.r[3].s64 = ctx.r[11].s64 + -24780;
	// 83256BCC: 4AF3D26D  bl 0x82193e38
	ctx.lr = 0x83256BD0;
	sub_82193E38(ctx, base);
	// 83256BD0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256BD4: 386AAA68  addi r3, r10, -0x5598
	ctx.r[3].s64 = ctx.r[10].s64 + -21912;
	// 83256BD8: 4BA53349  bl 0x82ca9f20
	ctx.lr = 0x83256BDC;
	sub_82CA9F20(ctx, base);
	// 83256BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256BF0 size=12
    let mut pc: u32 = 0x83256BF0;
    'dispatch: loop {
        match pc {
            0x83256BF0 => {
    //   block [0x83256BF0..0x83256BFC)
	// 83256BF0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256BF4: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 83256BF8: 4BA53328  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256C00 size=12
    let mut pc: u32 = 0x83256C00;
    'dispatch: loop {
        match pc {
            0x83256C00 => {
    //   block [0x83256C00..0x83256C0C)
	// 83256C00: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256C04: 386BAAF8  addi r3, r11, -0x5508
	ctx.r[3].s64 = ctx.r[11].s64 + -21768;
	// 83256C08: 4BA53318  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256C10 size=12
    let mut pc: u32 = 0x83256C10;
    'dispatch: loop {
        match pc {
            0x83256C10 => {
    //   block [0x83256C10..0x83256C1C)
	// 83256C10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256C14: 386BAB08  addi r3, r11, -0x54f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21752;
	// 83256C18: 4BA53308  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256C20 size=64
    let mut pc: u32 = 0x83256C20;
    'dispatch: loop {
        match pc {
            0x83256C20 => {
    //   block [0x83256C20..0x83256C60)
	// 83256C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256C2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256C34: 388B0A88  addi r4, r11, 0xa88
	ctx.r[4].s64 = ctx.r[11].s64 + 2696;
	// 83256C38: 386A883C  addi r3, r10, -0x77c4
	ctx.r[3].s64 = ctx.r[10].s64 + -30660;
	// 83256C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256C40: 4AFD6291  bl 0x8222ced0
	ctx.lr = 0x83256C44;
	sub_8222CED0(ctx, base);
	// 83256C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256C48: 3869AB80  addi r3, r9, -0x5480
	ctx.r[3].s64 = ctx.r[9].s64 + -21632;
	// 83256C4C: 4BA532D5  bl 0x82ca9f20
	ctx.lr = 0x83256C50;
	sub_82CA9F20(ctx, base);
	// 83256C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256C60 size=64
    let mut pc: u32 = 0x83256C60;
    'dispatch: loop {
        match pc {
            0x83256C60 => {
    //   block [0x83256C60..0x83256CA0)
	// 83256C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256C6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256C74: 388B0E18  addi r4, r11, 0xe18
	ctx.r[4].s64 = ctx.r[11].s64 + 3608;
	// 83256C78: 386A8840  addi r3, r10, -0x77c0
	ctx.r[3].s64 = ctx.r[10].s64 + -30656;
	// 83256C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256C80: 4AFD6251  bl 0x8222ced0
	ctx.lr = 0x83256C84;
	sub_8222CED0(ctx, base);
	// 83256C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256C88: 3869AB90  addi r3, r9, -0x5470
	ctx.r[3].s64 = ctx.r[9].s64 + -21616;
	// 83256C8C: 4BA53295  bl 0x82ca9f20
	ctx.lr = 0x83256C90;
	sub_82CA9F20(ctx, base);
	// 83256C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256CA0 size=64
    let mut pc: u32 = 0x83256CA0;
    'dispatch: loop {
        match pc {
            0x83256CA0 => {
    //   block [0x83256CA0..0x83256CE0)
	// 83256CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256CAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256CB4: 388B2EE0  addi r4, r11, 0x2ee0
	ctx.r[4].s64 = ctx.r[11].s64 + 12000;
	// 83256CB8: 386A8844  addi r3, r10, -0x77bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30652;
	// 83256CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256CC0: 4AFD6211  bl 0x8222ced0
	ctx.lr = 0x83256CC4;
	sub_8222CED0(ctx, base);
	// 83256CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256CC8: 3869ABA0  addi r3, r9, -0x5460
	ctx.r[3].s64 = ctx.r[9].s64 + -21600;
	// 83256CCC: 4BA53255  bl 0x82ca9f20
	ctx.lr = 0x83256CD0;
	sub_82CA9F20(ctx, base);
	// 83256CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256CE0 size=64
    let mut pc: u32 = 0x83256CE0;
    'dispatch: loop {
        match pc {
            0x83256CE0 => {
    //   block [0x83256CE0..0x83256D20)
	// 83256CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256CEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256CF4: 388B0E58  addi r4, r11, 0xe58
	ctx.r[4].s64 = ctx.r[11].s64 + 3672;
	// 83256CF8: 386A8848  addi r3, r10, -0x77b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30648;
	// 83256CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256D00: 4AFD61D1  bl 0x8222ced0
	ctx.lr = 0x83256D04;
	sub_8222CED0(ctx, base);
	// 83256D04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256D08: 3869ABB0  addi r3, r9, -0x5450
	ctx.r[3].s64 = ctx.r[9].s64 + -21584;
	// 83256D0C: 4BA53215  bl 0x82ca9f20
	ctx.lr = 0x83256D10;
	sub_82CA9F20(ctx, base);
	// 83256D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D20 size=52
    let mut pc: u32 = 0x83256D20;
    'dispatch: loop {
        match pc {
            0x83256D20 => {
    //   block [0x83256D20..0x83256D54)
	// 83256D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256D2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256D30: 386B884C  addi r3, r11, -0x77b4
	ctx.r[3].s64 = ctx.r[11].s64 + -30644;
	// 83256D34: 4B229505  bl 0x82480238
	ctx.lr = 0x83256D38;
	sub_82480238(ctx, base);
	// 83256D38: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256D3C: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 83256D40: 4BA531E1  bl 0x82ca9f20
	ctx.lr = 0x83256D44;
	sub_82CA9F20(ctx, base);
	// 83256D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D58 size=64
    let mut pc: u32 = 0x83256D58;
    'dispatch: loop {
        match pc {
            0x83256D58 => {
    //   block [0x83256D58..0x83256D98)
	// 83256D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256D64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256D68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256D6C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256D70: 386A8858  addi r3, r10, -0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30632;
	// 83256D74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256D78: 4AFD6159  bl 0x8222ced0
	ctx.lr = 0x83256D7C;
	sub_8222CED0(ctx, base);
	// 83256D7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256D80: 3869ABD0  addi r3, r9, -0x5430
	ctx.r[3].s64 = ctx.r[9].s64 + -21552;
	// 83256D84: 4BA5319D  bl 0x82ca9f20
	ctx.lr = 0x83256D88;
	sub_82CA9F20(ctx, base);
	// 83256D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D98 size=64
    let mut pc: u32 = 0x83256D98;
    'dispatch: loop {
        match pc {
            0x83256D98 => {
    //   block [0x83256D98..0x83256DD8)
	// 83256D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256DAC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256DB0: 386A885C  addi r3, r10, -0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + -30628;
	// 83256DB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256DB8: 4AFD6119  bl 0x8222ced0
	ctx.lr = 0x83256DBC;
	sub_8222CED0(ctx, base);
	// 83256DBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256DC0: 3869ABE0  addi r3, r9, -0x5420
	ctx.r[3].s64 = ctx.r[9].s64 + -21536;
	// 83256DC4: 4BA5315D  bl 0x82ca9f20
	ctx.lr = 0x83256DC8;
	sub_82CA9F20(ctx, base);
	// 83256DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256DD8 size=64
    let mut pc: u32 = 0x83256DD8;
    'dispatch: loop {
        match pc {
            0x83256DD8 => {
    //   block [0x83256DD8..0x83256E18)
	// 83256DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256DE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256DEC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256DF0: 386A8860  addi r3, r10, -0x77a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30624;
	// 83256DF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256DF8: 4AFD60D9  bl 0x8222ced0
	ctx.lr = 0x83256DFC;
	sub_8222CED0(ctx, base);
	// 83256DFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256E00: 3869ABF0  addi r3, r9, -0x5410
	ctx.r[3].s64 = ctx.r[9].s64 + -21520;
	// 83256E04: 4BA5311D  bl 0x82ca9f20
	ctx.lr = 0x83256E08;
	sub_82CA9F20(ctx, base);
	// 83256E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256E18 size=44
    let mut pc: u32 = 0x83256E18;
    'dispatch: loop {
        match pc {
            0x83256E18 => {
    //   block [0x83256E18..0x83256E44)
	// 83256E18: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256E1C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83256E20: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83256E24: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256E28: C9AA11A8  lfd f13, 0x11a8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4520 as u32) ) };
	// 83256E2C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83256E30: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256E34: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83256E38: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256E3C: 91698864  stw r11, -0x779c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30620 as u32), ctx.r[11].u32 ) };
	// 83256E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256E48 size=64
    let mut pc: u32 = 0x83256E48;
    'dispatch: loop {
        match pc {
            0x83256E48 => {
    //   block [0x83256E48..0x83256E88)
	// 83256E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256E54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256E5C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256E60: 386A8868  addi r3, r10, -0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + -30616;
	// 83256E64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256E68: 4AFD6069  bl 0x8222ced0
	ctx.lr = 0x83256E6C;
	sub_8222CED0(ctx, base);
	// 83256E6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256E70: 3869AC00  addi r3, r9, -0x5400
	ctx.r[3].s64 = ctx.r[9].s64 + -21504;
	// 83256E74: 4BA530AD  bl 0x82ca9f20
	ctx.lr = 0x83256E78;
	sub_82CA9F20(ctx, base);
	// 83256E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256E88 size=64
    let mut pc: u32 = 0x83256E88;
    'dispatch: loop {
        match pc {
            0x83256E88 => {
    //   block [0x83256E88..0x83256EC8)
	// 83256E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256E94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256E9C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256EA0: 386A886C  addi r3, r10, -0x7794
	ctx.r[3].s64 = ctx.r[10].s64 + -30612;
	// 83256EA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256EA8: 4AFD6029  bl 0x8222ced0
	ctx.lr = 0x83256EAC;
	sub_8222CED0(ctx, base);
	// 83256EAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256EB0: 3869AC10  addi r3, r9, -0x53f0
	ctx.r[3].s64 = ctx.r[9].s64 + -21488;
	// 83256EB4: 4BA5306D  bl 0x82ca9f20
	ctx.lr = 0x83256EB8;
	sub_82CA9F20(ctx, base);
	// 83256EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256EC8 size=64
    let mut pc: u32 = 0x83256EC8;
    'dispatch: loop {
        match pc {
            0x83256EC8 => {
    //   block [0x83256EC8..0x83256F08)
	// 83256EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256ED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256ED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256EDC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256EE0: 386A8870  addi r3, r10, -0x7790
	ctx.r[3].s64 = ctx.r[10].s64 + -30608;
	// 83256EE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256EE8: 4AFD5FE9  bl 0x8222ced0
	ctx.lr = 0x83256EEC;
	sub_8222CED0(ctx, base);
	// 83256EEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256EF0: 3869AC20  addi r3, r9, -0x53e0
	ctx.r[3].s64 = ctx.r[9].s64 + -21472;
	// 83256EF4: 4BA5302D  bl 0x82ca9f20
	ctx.lr = 0x83256EF8;
	sub_82CA9F20(ctx, base);
	// 83256EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F08 size=64
    let mut pc: u32 = 0x83256F08;
    'dispatch: loop {
        match pc {
            0x83256F08 => {
    //   block [0x83256F08..0x83256F48)
	// 83256F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256F14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256F18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256F1C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83256F20: 386A8874  addi r3, r10, -0x778c
	ctx.r[3].s64 = ctx.r[10].s64 + -30604;
	// 83256F24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256F28: 4AFD5FA9  bl 0x8222ced0
	ctx.lr = 0x83256F2C;
	sub_8222CED0(ctx, base);
	// 83256F2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256F30: 3869AC30  addi r3, r9, -0x53d0
	ctx.r[3].s64 = ctx.r[9].s64 + -21456;
	// 83256F34: 4BA52FED  bl 0x82ca9f20
	ctx.lr = 0x83256F38;
	sub_82CA9F20(ctx, base);
	// 83256F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F48 size=64
    let mut pc: u32 = 0x83256F48;
    'dispatch: loop {
        match pc {
            0x83256F48 => {
    //   block [0x83256F48..0x83256F88)
	// 83256F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256F54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256F5C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256F60: 386A8878  addi r3, r10, -0x7788
	ctx.r[3].s64 = ctx.r[10].s64 + -30600;
	// 83256F64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256F68: 4AFD5F69  bl 0x8222ced0
	ctx.lr = 0x83256F6C;
	sub_8222CED0(ctx, base);
	// 83256F6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256F70: 3869AC48  addi r3, r9, -0x53b8
	ctx.r[3].s64 = ctx.r[9].s64 + -21432;
	// 83256F74: 4BA52FAD  bl 0x82ca9f20
	ctx.lr = 0x83256F78;
	sub_82CA9F20(ctx, base);
	// 83256F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F88 size=64
    let mut pc: u32 = 0x83256F88;
    'dispatch: loop {
        match pc {
            0x83256F88 => {
    //   block [0x83256F88..0x83256FC8)
	// 83256F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256F94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256F98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256F9C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256FA0: 386A887C  addi r3, r10, -0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + -30596;
	// 83256FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256FA8: 4AFD5F29  bl 0x8222ced0
	ctx.lr = 0x83256FAC;
	sub_8222CED0(ctx, base);
	// 83256FAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256FB0: 3869AC58  addi r3, r9, -0x53a8
	ctx.r[3].s64 = ctx.r[9].s64 + -21416;
	// 83256FB4: 4BA52F6D  bl 0x82ca9f20
	ctx.lr = 0x83256FB8;
	sub_82CA9F20(ctx, base);
	// 83256FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256FC8 size=64
    let mut pc: u32 = 0x83256FC8;
    'dispatch: loop {
        match pc {
            0x83256FC8 => {
    //   block [0x83256FC8..0x83257008)
	// 83256FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256FD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256FDC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256FE0: 386A8880  addi r3, r10, -0x7780
	ctx.r[3].s64 = ctx.r[10].s64 + -30592;
	// 83256FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256FE8: 4AFD5EE9  bl 0x8222ced0
	ctx.lr = 0x83256FEC;
	sub_8222CED0(ctx, base);
	// 83256FEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256FF0: 3869AC68  addi r3, r9, -0x5398
	ctx.r[3].s64 = ctx.r[9].s64 + -21400;
	// 83256FF4: 4BA52F2D  bl 0x82ca9f20
	ctx.lr = 0x83256FF8;
	sub_82CA9F20(ctx, base);
	// 83256FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257008 size=52
    let mut pc: u32 = 0x83257008;
    'dispatch: loop {
        match pc {
            0x83257008 => {
    //   block [0x83257008..0x8325703C)
	// 83257008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257014: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83257018: 386B8884  addi r3, r11, -0x777c
	ctx.r[3].s64 = ctx.r[11].s64 + -30588;
	// 8325701C: 4B22921D  bl 0x82480238
	ctx.lr = 0x83257020;
	sub_82480238(ctx, base);
	// 83257020: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83257024: 386AAC88  addi r3, r10, -0x5378
	ctx.r[3].s64 = ctx.r[10].s64 + -21368;
	// 83257028: 4BA52EF9  bl 0x82ca9f20
	ctx.lr = 0x8325702C;
	sub_82CA9F20(ctx, base);
	// 8325702C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257040 size=64
    let mut pc: u32 = 0x83257040;
    'dispatch: loop {
        match pc {
            0x83257040 => {
    //   block [0x83257040..0x83257080)
	// 83257040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325704C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257054: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257058: 386A8890  addi r3, r10, -0x7770
	ctx.r[3].s64 = ctx.r[10].s64 + -30576;
	// 8325705C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257060: 4AFD5E71  bl 0x8222ced0
	ctx.lr = 0x83257064;
	sub_8222CED0(ctx, base);
	// 83257064: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257068: 3869ACB8  addi r3, r9, -0x5348
	ctx.r[3].s64 = ctx.r[9].s64 + -21320;
	// 8325706C: 4BA52EB5  bl 0x82ca9f20
	ctx.lr = 0x83257070;
	sub_82CA9F20(ctx, base);
	// 83257070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325707C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257080 size=64
    let mut pc: u32 = 0x83257080;
    'dispatch: loop {
        match pc {
            0x83257080 => {
    //   block [0x83257080..0x832570C0)
	// 83257080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325708C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257094: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257098: 386A8894  addi r3, r10, -0x776c
	ctx.r[3].s64 = ctx.r[10].s64 + -30572;
	// 8325709C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832570A0: 4AFD5E31  bl 0x8222ced0
	ctx.lr = 0x832570A4;
	sub_8222CED0(ctx, base);
	// 832570A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832570A8: 3869ACC8  addi r3, r9, -0x5338
	ctx.r[3].s64 = ctx.r[9].s64 + -21304;
	// 832570AC: 4BA52E75  bl 0x82ca9f20
	ctx.lr = 0x832570B0;
	sub_82CA9F20(ctx, base);
	// 832570B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832570B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832570B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832570BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832570C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832570C0 size=64
    let mut pc: u32 = 0x832570C0;
    'dispatch: loop {
        match pc {
            0x832570C0 => {
    //   block [0x832570C0..0x83257100)
	// 832570C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832570C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832570C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832570CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832570D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832570D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832570D8: 386A8898  addi r3, r10, -0x7768
	ctx.r[3].s64 = ctx.r[10].s64 + -30568;
	// 832570DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832570E0: 4AFD5DF1  bl 0x8222ced0
	ctx.lr = 0x832570E4;
	sub_8222CED0(ctx, base);
	// 832570E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832570E8: 3869ACD8  addi r3, r9, -0x5328
	ctx.r[3].s64 = ctx.r[9].s64 + -21288;
	// 832570EC: 4BA52E35  bl 0x82ca9f20
	ctx.lr = 0x832570F0;
	sub_82CA9F20(ctx, base);
	// 832570F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832570F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832570F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832570FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257100 size=64
    let mut pc: u32 = 0x83257100;
    'dispatch: loop {
        match pc {
            0x83257100 => {
    //   block [0x83257100..0x83257140)
	// 83257100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325710C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257114: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257118: 386A889C  addi r3, r10, -0x7764
	ctx.r[3].s64 = ctx.r[10].s64 + -30564;
	// 8325711C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257120: 4AFD5DB1  bl 0x8222ced0
	ctx.lr = 0x83257124;
	sub_8222CED0(ctx, base);
	// 83257124: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257128: 3869ACE8  addi r3, r9, -0x5318
	ctx.r[3].s64 = ctx.r[9].s64 + -21272;
	// 8325712C: 4BA52DF5  bl 0x82ca9f20
	ctx.lr = 0x83257130;
	sub_82CA9F20(ctx, base);
	// 83257130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325713C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257140 size=64
    let mut pc: u32 = 0x83257140;
    'dispatch: loop {
        match pc {
            0x83257140 => {
    //   block [0x83257140..0x83257180)
	// 83257140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325714C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257154: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257158: 386A88A0  addi r3, r10, -0x7760
	ctx.r[3].s64 = ctx.r[10].s64 + -30560;
	// 8325715C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257160: 4AFD5D71  bl 0x8222ced0
	ctx.lr = 0x83257164;
	sub_8222CED0(ctx, base);
	// 83257164: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257168: 3869ACF8  addi r3, r9, -0x5308
	ctx.r[3].s64 = ctx.r[9].s64 + -21256;
	// 8325716C: 4BA52DB5  bl 0x82ca9f20
	ctx.lr = 0x83257170;
	sub_82CA9F20(ctx, base);
	// 83257170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257180 size=64
    let mut pc: u32 = 0x83257180;
    'dispatch: loop {
        match pc {
            0x83257180 => {
    //   block [0x83257180..0x832571C0)
	// 83257180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325718C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257194: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257198: 386A88A4  addi r3, r10, -0x775c
	ctx.r[3].s64 = ctx.r[10].s64 + -30556;
	// 8325719C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832571A0: 4AFD5D31  bl 0x8222ced0
	ctx.lr = 0x832571A4;
	sub_8222CED0(ctx, base);
	// 832571A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832571A8: 3869AD08  addi r3, r9, -0x52f8
	ctx.r[3].s64 = ctx.r[9].s64 + -21240;
	// 832571AC: 4BA52D75  bl 0x82ca9f20
	ctx.lr = 0x832571B0;
	sub_82CA9F20(ctx, base);
	// 832571B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832571B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832571B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832571BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832571C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832571C0 size=64
    let mut pc: u32 = 0x832571C0;
    'dispatch: loop {
        match pc {
            0x832571C0 => {
    //   block [0x832571C0..0x83257200)
	// 832571C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832571C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832571C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832571CC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832571D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832571D4: 388B3C3C  addi r4, r11, 0x3c3c
	ctx.r[4].s64 = ctx.r[11].s64 + 15420;
	// 832571D8: 386A88A8  addi r3, r10, -0x7758
	ctx.r[3].s64 = ctx.r[10].s64 + -30552;
	// 832571DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832571E0: 4AFD5CF1  bl 0x8222ced0
	ctx.lr = 0x832571E4;
	sub_8222CED0(ctx, base);
	// 832571E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832571E8: 3869AD18  addi r3, r9, -0x52e8
	ctx.r[3].s64 = ctx.r[9].s64 + -21224;
	// 832571EC: 4BA52D35  bl 0x82ca9f20
	ctx.lr = 0x832571F0;
	sub_82CA9F20(ctx, base);
	// 832571F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832571F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832571F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832571FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257200 size=64
    let mut pc: u32 = 0x83257200;
    'dispatch: loop {
        match pc {
            0x83257200 => {
    //   block [0x83257200..0x83257240)
	// 83257200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325720C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257214: 388B3C48  addi r4, r11, 0x3c48
	ctx.r[4].s64 = ctx.r[11].s64 + 15432;
	// 83257218: 386A88AC  addi r3, r10, -0x7754
	ctx.r[3].s64 = ctx.r[10].s64 + -30548;
	// 8325721C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257220: 4AFD5CB1  bl 0x8222ced0
	ctx.lr = 0x83257224;
	sub_8222CED0(ctx, base);
	// 83257224: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257228: 3869AD28  addi r3, r9, -0x52d8
	ctx.r[3].s64 = ctx.r[9].s64 + -21208;
	// 8325722C: 4BA52CF5  bl 0x82ca9f20
	ctx.lr = 0x83257230;
	sub_82CA9F20(ctx, base);
	// 83257230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325723C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257240 size=96
    let mut pc: u32 = 0x83257240;
    'dispatch: loop {
        match pc {
            0x83257240 => {
    //   block [0x83257240..0x83257264)
	// 83257240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325724C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83257250: 4AFC8009  bl 0x8221f258
	ctx.lr = 0x83257254;
	sub_8221F258(ctx, base);
	// 83257254: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83257258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8325725C: 419A0008  beq cr6, 0x83257264
	if ctx.cr[6].eq {
	pc = 0x83257264; continue 'dispatch;
	}
	// 83257260: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83257264; continue 'dispatch;
            }
            0x83257264 => {
    //   block [0x83257264..0x83257270)
	// 83257264: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83257268: 41820008  beq 0x83257270
	if ctx.cr[0].eq {
	pc = 0x83257270; continue 'dispatch;
	}
	// 8325726C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83257270; continue 'dispatch;
            }
            0x83257270 => {
    //   block [0x83257270..0x832572A0)
	// 83257270: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83257274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83257278: 390988B0  addi r8, r9, -0x7750
	ctx.r[8].s64 = ctx.r[9].s64 + -30544;
	// 8325727C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83257280: 3867AD38  addi r3, r7, -0x52c8
	ctx.r[3].s64 = ctx.r[7].s64 + -21192;
	// 83257284: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83257288: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325728C: 4BA52C95  bl 0x82ca9f20
	ctx.lr = 0x83257290;
	sub_82CA9F20(ctx, base);
	// 83257290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325729C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832572A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832572A0 size=64
    let mut pc: u32 = 0x832572A0;
    'dispatch: loop {
        match pc {
            0x832572A0 => {
    //   block [0x832572A0..0x832572E0)
	// 832572A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832572A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832572A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832572AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832572B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832572B4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832572B8: 386A88BC  addi r3, r10, -0x7744
	ctx.r[3].s64 = ctx.r[10].s64 + -30532;
	// 832572BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832572C0: 4AFD5C11  bl 0x8222ced0
	ctx.lr = 0x832572C4;
	sub_8222CED0(ctx, base);
	// 832572C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832572C8: 3869ADC0  addi r3, r9, -0x5240
	ctx.r[3].s64 = ctx.r[9].s64 + -21056;
	// 832572CC: 4BA52C55  bl 0x82ca9f20
	ctx.lr = 0x832572D0;
	sub_82CA9F20(ctx, base);
	// 832572D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832572D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832572D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832572DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832572E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832572E0 size=64
    let mut pc: u32 = 0x832572E0;
    'dispatch: loop {
        match pc {
            0x832572E0 => {
    //   block [0x832572E0..0x83257320)
	// 832572E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832572E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832572E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832572EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832572F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832572F4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832572F8: 386A88C0  addi r3, r10, -0x7740
	ctx.r[3].s64 = ctx.r[10].s64 + -30528;
	// 832572FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257300: 4AFD5BD1  bl 0x8222ced0
	ctx.lr = 0x83257304;
	sub_8222CED0(ctx, base);
	// 83257304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257308: 3869ADD0  addi r3, r9, -0x5230
	ctx.r[3].s64 = ctx.r[9].s64 + -21040;
	// 8325730C: 4BA52C15  bl 0x82ca9f20
	ctx.lr = 0x83257310;
	sub_82CA9F20(ctx, base);
	// 83257310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325731C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257320 size=64
    let mut pc: u32 = 0x83257320;
    'dispatch: loop {
        match pc {
            0x83257320 => {
    //   block [0x83257320..0x83257360)
	// 83257320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325732C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257334: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257338: 386A88C4  addi r3, r10, -0x773c
	ctx.r[3].s64 = ctx.r[10].s64 + -30524;
	// 8325733C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257340: 4AFD5B91  bl 0x8222ced0
	ctx.lr = 0x83257344;
	sub_8222CED0(ctx, base);
	// 83257344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257348: 3869ADE0  addi r3, r9, -0x5220
	ctx.r[3].s64 = ctx.r[9].s64 + -21024;
	// 8325734C: 4BA52BD5  bl 0x82ca9f20
	ctx.lr = 0x83257350;
	sub_82CA9F20(ctx, base);
	// 83257350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257360 size=64
    let mut pc: u32 = 0x83257360;
    'dispatch: loop {
        match pc {
            0x83257360 => {
    //   block [0x83257360..0x832573A0)
	// 83257360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325736C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257374: 388B4034  addi r4, r11, 0x4034
	ctx.r[4].s64 = ctx.r[11].s64 + 16436;
	// 83257378: 386A88C8  addi r3, r10, -0x7738
	ctx.r[3].s64 = ctx.r[10].s64 + -30520;
	// 8325737C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257380: 4AFD5B51  bl 0x8222ced0
	ctx.lr = 0x83257384;
	sub_8222CED0(ctx, base);
	// 83257384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257388: 3869ADF0  addi r3, r9, -0x5210
	ctx.r[3].s64 = ctx.r[9].s64 + -21008;
	// 8325738C: 4BA52B95  bl 0x82ca9f20
	ctx.lr = 0x83257390;
	sub_82CA9F20(ctx, base);
	// 83257390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325739C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832573A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832573A0 size=64
    let mut pc: u32 = 0x832573A0;
    'dispatch: loop {
        match pc {
            0x832573A0 => {
    //   block [0x832573A0..0x832573E0)
	// 832573A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832573A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832573A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832573AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832573B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832573B4: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 832573B8: 386A88CC  addi r3, r10, -0x7734
	ctx.r[3].s64 = ctx.r[10].s64 + -30516;
	// 832573BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832573C0: 4AFD5B11  bl 0x8222ced0
	ctx.lr = 0x832573C4;
	sub_8222CED0(ctx, base);
	// 832573C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832573C8: 3869AE00  addi r3, r9, -0x5200
	ctx.r[3].s64 = ctx.r[9].s64 + -20992;
	// 832573CC: 4BA52B55  bl 0x82ca9f20
	ctx.lr = 0x832573D0;
	sub_82CA9F20(ctx, base);
	// 832573D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832573D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832573D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832573DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832573E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832573E0 size=64
    let mut pc: u32 = 0x832573E0;
    'dispatch: loop {
        match pc {
            0x832573E0 => {
    //   block [0x832573E0..0x83257420)
	// 832573E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832573E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832573E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832573EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832573F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832573F4: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 832573F8: 386A88D0  addi r3, r10, -0x7730
	ctx.r[3].s64 = ctx.r[10].s64 + -30512;
	// 832573FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257400: 4AFD5AD1  bl 0x8222ced0
	ctx.lr = 0x83257404;
	sub_8222CED0(ctx, base);
	// 83257404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257408: 3869AE10  addi r3, r9, -0x51f0
	ctx.r[3].s64 = ctx.r[9].s64 + -20976;
	// 8325740C: 4BA52B15  bl 0x82ca9f20
	ctx.lr = 0x83257410;
	sub_82CA9F20(ctx, base);
	// 83257410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325741C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257420 size=64
    let mut pc: u32 = 0x83257420;
    'dispatch: loop {
        match pc {
            0x83257420 => {
    //   block [0x83257420..0x83257460)
	// 83257420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325742C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257434: 388B403C  addi r4, r11, 0x403c
	ctx.r[4].s64 = ctx.r[11].s64 + 16444;
	// 83257438: 386A88D4  addi r3, r10, -0x772c
	ctx.r[3].s64 = ctx.r[10].s64 + -30508;
	// 8325743C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257440: 4AFD5A91  bl 0x8222ced0
	ctx.lr = 0x83257444;
	sub_8222CED0(ctx, base);
	// 83257444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257448: 3869AE20  addi r3, r9, -0x51e0
	ctx.r[3].s64 = ctx.r[9].s64 + -20960;
	// 8325744C: 4BA52AD5  bl 0x82ca9f20
	ctx.lr = 0x83257450;
	sub_82CA9F20(ctx, base);
	// 83257450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325745C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257460 size=64
    let mut pc: u32 = 0x83257460;
    'dispatch: loop {
        match pc {
            0x83257460 => {
    //   block [0x83257460..0x832574A0)
	// 83257460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325746C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257474: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257478: 386A88D8  addi r3, r10, -0x7728
	ctx.r[3].s64 = ctx.r[10].s64 + -30504;
	// 8325747C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257480: 4AFD5A51  bl 0x8222ced0
	ctx.lr = 0x83257484;
	sub_8222CED0(ctx, base);
	// 83257484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257488: 3869AE30  addi r3, r9, -0x51d0
	ctx.r[3].s64 = ctx.r[9].s64 + -20944;
	// 8325748C: 4BA52A95  bl 0x82ca9f20
	ctx.lr = 0x83257490;
	sub_82CA9F20(ctx, base);
	// 83257490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325749C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832574A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832574A0 size=64
    let mut pc: u32 = 0x832574A0;
    'dispatch: loop {
        match pc {
            0x832574A0 => {
    //   block [0x832574A0..0x832574E0)
	// 832574A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832574A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832574A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832574AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832574B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832574B4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832574B8: 386A88DC  addi r3, r10, -0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + -30500;
	// 832574BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832574C0: 4AFD5A11  bl 0x8222ced0
	ctx.lr = 0x832574C4;
	sub_8222CED0(ctx, base);
	// 832574C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832574C8: 3869AE40  addi r3, r9, -0x51c0
	ctx.r[3].s64 = ctx.r[9].s64 + -20928;
	// 832574CC: 4BA52A55  bl 0x82ca9f20
	ctx.lr = 0x832574D0;
	sub_82CA9F20(ctx, base);
	// 832574D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832574D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832574D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832574DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832574E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832574E0 size=64
    let mut pc: u32 = 0x832574E0;
    'dispatch: loop {
        match pc {
            0x832574E0 => {
    //   block [0x832574E0..0x83257520)
	// 832574E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832574E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832574E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832574EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832574F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832574F4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832574F8: 386A88E0  addi r3, r10, -0x7720
	ctx.r[3].s64 = ctx.r[10].s64 + -30496;
	// 832574FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257500: 4AFD59D1  bl 0x8222ced0
	ctx.lr = 0x83257504;
	sub_8222CED0(ctx, base);
	// 83257504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257508: 3869AE50  addi r3, r9, -0x51b0
	ctx.r[3].s64 = ctx.r[9].s64 + -20912;
	// 8325750C: 4BA52A15  bl 0x82ca9f20
	ctx.lr = 0x83257510;
	sub_82CA9F20(ctx, base);
	// 83257510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257520 size=64
    let mut pc: u32 = 0x83257520;
    'dispatch: loop {
        match pc {
            0x83257520 => {
    //   block [0x83257520..0x83257560)
	// 83257520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325752C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257534: 388B475C  addi r4, r11, 0x475c
	ctx.r[4].s64 = ctx.r[11].s64 + 18268;
	// 83257538: 386A88E4  addi r3, r10, -0x771c
	ctx.r[3].s64 = ctx.r[10].s64 + -30492;
	// 8325753C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257540: 4AFD5991  bl 0x8222ced0
	ctx.lr = 0x83257544;
	sub_8222CED0(ctx, base);
	// 83257544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257548: 3869AE60  addi r3, r9, -0x51a0
	ctx.r[3].s64 = ctx.r[9].s64 + -20896;
	// 8325754C: 4BA529D5  bl 0x82ca9f20
	ctx.lr = 0x83257550;
	sub_82CA9F20(ctx, base);
	// 83257550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325755C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257560 size=64
    let mut pc: u32 = 0x83257560;
    'dispatch: loop {
        match pc {
            0x83257560 => {
    //   block [0x83257560..0x832575A0)
	// 83257560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325756C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257574: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257578: 386A88E8  addi r3, r10, -0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + -30488;
	// 8325757C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257580: 4AFD5951  bl 0x8222ced0
	ctx.lr = 0x83257584;
	sub_8222CED0(ctx, base);
	// 83257584: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257588: 3869AE70  addi r3, r9, -0x5190
	ctx.r[3].s64 = ctx.r[9].s64 + -20880;
	// 8325758C: 4BA52995  bl 0x82ca9f20
	ctx.lr = 0x83257590;
	sub_82CA9F20(ctx, base);
	// 83257590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325759C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832575A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832575A0 size=64
    let mut pc: u32 = 0x832575A0;
    'dispatch: loop {
        match pc {
            0x832575A0 => {
    //   block [0x832575A0..0x832575E0)
	// 832575A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832575A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832575A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832575AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832575B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832575B4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832575B8: 386A88EC  addi r3, r10, -0x7714
	ctx.r[3].s64 = ctx.r[10].s64 + -30484;
	// 832575BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832575C0: 4AFD5911  bl 0x8222ced0
	ctx.lr = 0x832575C4;
	sub_8222CED0(ctx, base);
	// 832575C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832575C8: 3869AE80  addi r3, r9, -0x5180
	ctx.r[3].s64 = ctx.r[9].s64 + -20864;
	// 832575CC: 4BA52955  bl 0x82ca9f20
	ctx.lr = 0x832575D0;
	sub_82CA9F20(ctx, base);
	// 832575D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832575D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832575D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832575DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832575E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832575E0 size=64
    let mut pc: u32 = 0x832575E0;
    'dispatch: loop {
        match pc {
            0x832575E0 => {
    //   block [0x832575E0..0x83257620)
	// 832575E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832575E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832575E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832575EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832575F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832575F4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832575F8: 386A88F0  addi r3, r10, -0x7710
	ctx.r[3].s64 = ctx.r[10].s64 + -30480;
	// 832575FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257600: 4AFD58D1  bl 0x8222ced0
	ctx.lr = 0x83257604;
	sub_8222CED0(ctx, base);
	// 83257604: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257608: 3869AE90  addi r3, r9, -0x5170
	ctx.r[3].s64 = ctx.r[9].s64 + -20848;
	// 8325760C: 4BA52915  bl 0x82ca9f20
	ctx.lr = 0x83257610;
	sub_82CA9F20(ctx, base);
	// 83257610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325761C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257620 size=64
    let mut pc: u32 = 0x83257620;
    'dispatch: loop {
        match pc {
            0x83257620 => {
    //   block [0x83257620..0x83257660)
	// 83257620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325762C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257634: 388B47A0  addi r4, r11, 0x47a0
	ctx.r[4].s64 = ctx.r[11].s64 + 18336;
	// 83257638: 386A88F4  addi r3, r10, -0x770c
	ctx.r[3].s64 = ctx.r[10].s64 + -30476;
	// 8325763C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257640: 4AFD5891  bl 0x8222ced0
	ctx.lr = 0x83257644;
	sub_8222CED0(ctx, base);
	// 83257644: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257648: 3869AEA0  addi r3, r9, -0x5160
	ctx.r[3].s64 = ctx.r[9].s64 + -20832;
	// 8325764C: 4BA528D5  bl 0x82ca9f20
	ctx.lr = 0x83257650;
	sub_82CA9F20(ctx, base);
	// 83257650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257660 size=64
    let mut pc: u32 = 0x83257660;
    'dispatch: loop {
        match pc {
            0x83257660 => {
    //   block [0x83257660..0x832576A0)
	// 83257660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325766C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257670: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257674: 388B47B0  addi r4, r11, 0x47b0
	ctx.r[4].s64 = ctx.r[11].s64 + 18352;
	// 83257678: 386A88F8  addi r3, r10, -0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + -30472;
	// 8325767C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257680: 4AFD5851  bl 0x8222ced0
	ctx.lr = 0x83257684;
	sub_8222CED0(ctx, base);
	// 83257684: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257688: 3869AEB0  addi r3, r9, -0x5150
	ctx.r[3].s64 = ctx.r[9].s64 + -20816;
	// 8325768C: 4BA52895  bl 0x82ca9f20
	ctx.lr = 0x83257690;
	sub_82CA9F20(ctx, base);
	// 83257690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325769C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832576A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832576A0 size=64
    let mut pc: u32 = 0x832576A0;
    'dispatch: loop {
        match pc {
            0x832576A0 => {
    //   block [0x832576A0..0x832576E0)
	// 832576A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832576A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832576A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832576AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832576B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832576B4: 388B47C0  addi r4, r11, 0x47c0
	ctx.r[4].s64 = ctx.r[11].s64 + 18368;
	// 832576B8: 386A88FC  addi r3, r10, -0x7704
	ctx.r[3].s64 = ctx.r[10].s64 + -30468;
	// 832576BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832576C0: 4AFD5811  bl 0x8222ced0
	ctx.lr = 0x832576C4;
	sub_8222CED0(ctx, base);
	// 832576C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832576C8: 3869AEC0  addi r3, r9, -0x5140
	ctx.r[3].s64 = ctx.r[9].s64 + -20800;
	// 832576CC: 4BA52855  bl 0x82ca9f20
	ctx.lr = 0x832576D0;
	sub_82CA9F20(ctx, base);
	// 832576D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832576D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832576D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832576DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832576E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832576E0 size=64
    let mut pc: u32 = 0x832576E0;
    'dispatch: loop {
        match pc {
            0x832576E0 => {
    //   block [0x832576E0..0x83257720)
	// 832576E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832576E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832576E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832576EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832576F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832576F4: 388B47D4  addi r4, r11, 0x47d4
	ctx.r[4].s64 = ctx.r[11].s64 + 18388;
	// 832576F8: 386A8900  addi r3, r10, -0x7700
	ctx.r[3].s64 = ctx.r[10].s64 + -30464;
	// 832576FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257700: 4AFD57D1  bl 0x8222ced0
	ctx.lr = 0x83257704;
	sub_8222CED0(ctx, base);
	// 83257704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257708: 3869AED0  addi r3, r9, -0x5130
	ctx.r[3].s64 = ctx.r[9].s64 + -20784;
	// 8325770C: 4BA52815  bl 0x82ca9f20
	ctx.lr = 0x83257710;
	sub_82CA9F20(ctx, base);
	// 83257710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325771C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257720 size=64
    let mut pc: u32 = 0x83257720;
    'dispatch: loop {
        match pc {
            0x83257720 => {
    //   block [0x83257720..0x83257760)
	// 83257720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325772C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257734: 388B47E0  addi r4, r11, 0x47e0
	ctx.r[4].s64 = ctx.r[11].s64 + 18400;
	// 83257738: 386A8904  addi r3, r10, -0x76fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30460;
	// 8325773C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257740: 4AFD5791  bl 0x8222ced0
	ctx.lr = 0x83257744;
	sub_8222CED0(ctx, base);
	// 83257744: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257748: 3869AEE0  addi r3, r9, -0x5120
	ctx.r[3].s64 = ctx.r[9].s64 + -20768;
	// 8325774C: 4BA527D5  bl 0x82ca9f20
	ctx.lr = 0x83257750;
	sub_82CA9F20(ctx, base);
	// 83257750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325775C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257760 size=64
    let mut pc: u32 = 0x83257760;
    'dispatch: loop {
        match pc {
            0x83257760 => {
    //   block [0x83257760..0x832577A0)
	// 83257760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325776C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257774: 388B47F4  addi r4, r11, 0x47f4
	ctx.r[4].s64 = ctx.r[11].s64 + 18420;
	// 83257778: 386A8908  addi r3, r10, -0x76f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30456;
	// 8325777C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257780: 4AFD5751  bl 0x8222ced0
	ctx.lr = 0x83257784;
	sub_8222CED0(ctx, base);
	// 83257784: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257788: 3869AEF0  addi r3, r9, -0x5110
	ctx.r[3].s64 = ctx.r[9].s64 + -20752;
	// 8325778C: 4BA52795  bl 0x82ca9f20
	ctx.lr = 0x83257790;
	sub_82CA9F20(ctx, base);
	// 83257790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325779C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832577A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832577A0 size=64
    let mut pc: u32 = 0x832577A0;
    'dispatch: loop {
        match pc {
            0x832577A0 => {
    //   block [0x832577A0..0x832577E0)
	// 832577A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832577A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832577A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832577AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832577B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832577B4: 388B4808  addi r4, r11, 0x4808
	ctx.r[4].s64 = ctx.r[11].s64 + 18440;
	// 832577B8: 386A890C  addi r3, r10, -0x76f4
	ctx.r[3].s64 = ctx.r[10].s64 + -30452;
	// 832577BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832577C0: 4AFD5711  bl 0x8222ced0
	ctx.lr = 0x832577C4;
	sub_8222CED0(ctx, base);
	// 832577C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832577C8: 3869AF00  addi r3, r9, -0x5100
	ctx.r[3].s64 = ctx.r[9].s64 + -20736;
	// 832577CC: 4BA52755  bl 0x82ca9f20
	ctx.lr = 0x832577D0;
	sub_82CA9F20(ctx, base);
	// 832577D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832577D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832577D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832577DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832577E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832577E0 size=64
    let mut pc: u32 = 0x832577E0;
    'dispatch: loop {
        match pc {
            0x832577E0 => {
    //   block [0x832577E0..0x83257820)
	// 832577E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832577E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832577E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832577EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832577F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832577F4: 388B4820  addi r4, r11, 0x4820
	ctx.r[4].s64 = ctx.r[11].s64 + 18464;
	// 832577F8: 386A8910  addi r3, r10, -0x76f0
	ctx.r[3].s64 = ctx.r[10].s64 + -30448;
	// 832577FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257800: 4AFD56D1  bl 0x8222ced0
	ctx.lr = 0x83257804;
	sub_8222CED0(ctx, base);
	// 83257804: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257808: 3869AF10  addi r3, r9, -0x50f0
	ctx.r[3].s64 = ctx.r[9].s64 + -20720;
	// 8325780C: 4BA52715  bl 0x82ca9f20
	ctx.lr = 0x83257810;
	sub_82CA9F20(ctx, base);
	// 83257810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257820 size=64
    let mut pc: u32 = 0x83257820;
    'dispatch: loop {
        match pc {
            0x83257820 => {
    //   block [0x83257820..0x83257860)
	// 83257820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325782C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257830: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257834: 388B4838  addi r4, r11, 0x4838
	ctx.r[4].s64 = ctx.r[11].s64 + 18488;
	// 83257838: 386A8914  addi r3, r10, -0x76ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30444;
	// 8325783C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257840: 4AFD5691  bl 0x8222ced0
	ctx.lr = 0x83257844;
	sub_8222CED0(ctx, base);
	// 83257844: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257848: 3869AF20  addi r3, r9, -0x50e0
	ctx.r[3].s64 = ctx.r[9].s64 + -20704;
	// 8325784C: 4BA526D5  bl 0x82ca9f20
	ctx.lr = 0x83257850;
	sub_82CA9F20(ctx, base);
	// 83257850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325785C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257860 size=64
    let mut pc: u32 = 0x83257860;
    'dispatch: loop {
        match pc {
            0x83257860 => {
    //   block [0x83257860..0x832578A0)
	// 83257860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325786C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257870: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257874: 388B4850  addi r4, r11, 0x4850
	ctx.r[4].s64 = ctx.r[11].s64 + 18512;
	// 83257878: 386A8918  addi r3, r10, -0x76e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30440;
	// 8325787C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257880: 4AFD5651  bl 0x8222ced0
	ctx.lr = 0x83257884;
	sub_8222CED0(ctx, base);
	// 83257884: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257888: 3869AF30  addi r3, r9, -0x50d0
	ctx.r[3].s64 = ctx.r[9].s64 + -20688;
	// 8325788C: 4BA52695  bl 0x82ca9f20
	ctx.lr = 0x83257890;
	sub_82CA9F20(ctx, base);
	// 83257890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325789C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832578A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832578A0 size=64
    let mut pc: u32 = 0x832578A0;
    'dispatch: loop {
        match pc {
            0x832578A0 => {
    //   block [0x832578A0..0x832578E0)
	// 832578A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832578A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832578A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832578AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832578B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832578B4: 388B4868  addi r4, r11, 0x4868
	ctx.r[4].s64 = ctx.r[11].s64 + 18536;
	// 832578B8: 386A891C  addi r3, r10, -0x76e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30436;
	// 832578BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832578C0: 4AFD5611  bl 0x8222ced0
	ctx.lr = 0x832578C4;
	sub_8222CED0(ctx, base);
	// 832578C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832578C8: 3869AF40  addi r3, r9, -0x50c0
	ctx.r[3].s64 = ctx.r[9].s64 + -20672;
	// 832578CC: 4BA52655  bl 0x82ca9f20
	ctx.lr = 0x832578D0;
	sub_82CA9F20(ctx, base);
	// 832578D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832578D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832578D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832578DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832578E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832578E0 size=64
    let mut pc: u32 = 0x832578E0;
    'dispatch: loop {
        match pc {
            0x832578E0 => {
    //   block [0x832578E0..0x83257920)
	// 832578E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832578E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832578E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832578EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832578F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832578F4: 388B4874  addi r4, r11, 0x4874
	ctx.r[4].s64 = ctx.r[11].s64 + 18548;
	// 832578F8: 386A8920  addi r3, r10, -0x76e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30432;
	// 832578FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257900: 4AFD55D1  bl 0x8222ced0
	ctx.lr = 0x83257904;
	sub_8222CED0(ctx, base);
	// 83257904: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257908: 3869AF50  addi r3, r9, -0x50b0
	ctx.r[3].s64 = ctx.r[9].s64 + -20656;
	// 8325790C: 4BA52615  bl 0x82ca9f20
	ctx.lr = 0x83257910;
	sub_82CA9F20(ctx, base);
	// 83257910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325791C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257920 size=64
    let mut pc: u32 = 0x83257920;
    'dispatch: loop {
        match pc {
            0x83257920 => {
    //   block [0x83257920..0x83257960)
	// 83257920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325792C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257934: 388B4878  addi r4, r11, 0x4878
	ctx.r[4].s64 = ctx.r[11].s64 + 18552;
	// 83257938: 386A8924  addi r3, r10, -0x76dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30428;
	// 8325793C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257940: 4AFD5591  bl 0x8222ced0
	ctx.lr = 0x83257944;
	sub_8222CED0(ctx, base);
	// 83257944: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257948: 3869AF60  addi r3, r9, -0x50a0
	ctx.r[3].s64 = ctx.r[9].s64 + -20640;
	// 8325794C: 4BA525D5  bl 0x82ca9f20
	ctx.lr = 0x83257950;
	sub_82CA9F20(ctx, base);
	// 83257950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325795C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257960 size=64
    let mut pc: u32 = 0x83257960;
    'dispatch: loop {
        match pc {
            0x83257960 => {
    //   block [0x83257960..0x832579A0)
	// 83257960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325796C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257974: 388B4880  addi r4, r11, 0x4880
	ctx.r[4].s64 = ctx.r[11].s64 + 18560;
	// 83257978: 386A8928  addi r3, r10, -0x76d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30424;
	// 8325797C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257980: 4AFD5551  bl 0x8222ced0
	ctx.lr = 0x83257984;
	sub_8222CED0(ctx, base);
	// 83257984: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257988: 3869AF70  addi r3, r9, -0x5090
	ctx.r[3].s64 = ctx.r[9].s64 + -20624;
	// 8325798C: 4BA52595  bl 0x82ca9f20
	ctx.lr = 0x83257990;
	sub_82CA9F20(ctx, base);
	// 83257990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325799C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832579A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832579A0 size=64
    let mut pc: u32 = 0x832579A0;
    'dispatch: loop {
        match pc {
            0x832579A0 => {
    //   block [0x832579A0..0x832579E0)
	// 832579A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832579A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832579A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832579AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832579B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832579B4: 388B489C  addi r4, r11, 0x489c
	ctx.r[4].s64 = ctx.r[11].s64 + 18588;
	// 832579B8: 386A892C  addi r3, r10, -0x76d4
	ctx.r[3].s64 = ctx.r[10].s64 + -30420;
	// 832579BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832579C0: 4AFD5511  bl 0x8222ced0
	ctx.lr = 0x832579C4;
	sub_8222CED0(ctx, base);
	// 832579C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832579C8: 3869AF80  addi r3, r9, -0x5080
	ctx.r[3].s64 = ctx.r[9].s64 + -20608;
	// 832579CC: 4BA52555  bl 0x82ca9f20
	ctx.lr = 0x832579D0;
	sub_82CA9F20(ctx, base);
	// 832579D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832579D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832579D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832579DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832579E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832579E0 size=64
    let mut pc: u32 = 0x832579E0;
    'dispatch: loop {
        match pc {
            0x832579E0 => {
    //   block [0x832579E0..0x83257A20)
	// 832579E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832579E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832579E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832579EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832579F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832579F4: 388B48B8  addi r4, r11, 0x48b8
	ctx.r[4].s64 = ctx.r[11].s64 + 18616;
	// 832579F8: 386A8930  addi r3, r10, -0x76d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30416;
	// 832579FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257A00: 4AFD54D1  bl 0x8222ced0
	ctx.lr = 0x83257A04;
	sub_8222CED0(ctx, base);
	// 83257A04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257A08: 3869AF90  addi r3, r9, -0x5070
	ctx.r[3].s64 = ctx.r[9].s64 + -20592;
	// 83257A0C: 4BA52515  bl 0x82ca9f20
	ctx.lr = 0x83257A10;
	sub_82CA9F20(ctx, base);
	// 83257A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257A20 size=64
    let mut pc: u32 = 0x83257A20;
    'dispatch: loop {
        match pc {
            0x83257A20 => {
    //   block [0x83257A20..0x83257A60)
	// 83257A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257A2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257A30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257A34: 388B48C4  addi r4, r11, 0x48c4
	ctx.r[4].s64 = ctx.r[11].s64 + 18628;
	// 83257A38: 386A8934  addi r3, r10, -0x76cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30412;
	// 83257A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257A40: 4AFD5491  bl 0x8222ced0
	ctx.lr = 0x83257A44;
	sub_8222CED0(ctx, base);
	// 83257A44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257A48: 3869AFA0  addi r3, r9, -0x5060
	ctx.r[3].s64 = ctx.r[9].s64 + -20576;
	// 83257A4C: 4BA524D5  bl 0x82ca9f20
	ctx.lr = 0x83257A50;
	sub_82CA9F20(ctx, base);
	// 83257A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257A60 size=64
    let mut pc: u32 = 0x83257A60;
    'dispatch: loop {
        match pc {
            0x83257A60 => {
    //   block [0x83257A60..0x83257AA0)
	// 83257A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257A6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257A74: 388B48D4  addi r4, r11, 0x48d4
	ctx.r[4].s64 = ctx.r[11].s64 + 18644;
	// 83257A78: 386A8938  addi r3, r10, -0x76c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30408;
	// 83257A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257A80: 4AFD5451  bl 0x8222ced0
	ctx.lr = 0x83257A84;
	sub_8222CED0(ctx, base);
	// 83257A84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257A88: 3869AFB0  addi r3, r9, -0x5050
	ctx.r[3].s64 = ctx.r[9].s64 + -20560;
	// 83257A8C: 4BA52495  bl 0x82ca9f20
	ctx.lr = 0x83257A90;
	sub_82CA9F20(ctx, base);
	// 83257A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257AA0 size=64
    let mut pc: u32 = 0x83257AA0;
    'dispatch: loop {
        match pc {
            0x83257AA0 => {
    //   block [0x83257AA0..0x83257AE0)
	// 83257AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257AAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257AB4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257AB8: 386A893C  addi r3, r10, -0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + -30404;
	// 83257ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257AC0: 4AFD5411  bl 0x8222ced0
	ctx.lr = 0x83257AC4;
	sub_8222CED0(ctx, base);
	// 83257AC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257AC8: 3869AFC0  addi r3, r9, -0x5040
	ctx.r[3].s64 = ctx.r[9].s64 + -20544;
	// 83257ACC: 4BA52455  bl 0x82ca9f20
	ctx.lr = 0x83257AD0;
	sub_82CA9F20(ctx, base);
	// 83257AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257AE0 size=64
    let mut pc: u32 = 0x83257AE0;
    'dispatch: loop {
        match pc {
            0x83257AE0 => {
    //   block [0x83257AE0..0x83257B20)
	// 83257AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257AEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257AF4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257AF8: 386A8940  addi r3, r10, -0x76c0
	ctx.r[3].s64 = ctx.r[10].s64 + -30400;
	// 83257AFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257B00: 4AFD53D1  bl 0x8222ced0
	ctx.lr = 0x83257B04;
	sub_8222CED0(ctx, base);
	// 83257B04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257B08: 3869AFD0  addi r3, r9, -0x5030
	ctx.r[3].s64 = ctx.r[9].s64 + -20528;
	// 83257B0C: 4BA52415  bl 0x82ca9f20
	ctx.lr = 0x83257B10;
	sub_82CA9F20(ctx, base);
	// 83257B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257B20 size=64
    let mut pc: u32 = 0x83257B20;
    'dispatch: loop {
        match pc {
            0x83257B20 => {
    //   block [0x83257B20..0x83257B60)
	// 83257B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257B2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257B34: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257B38: 386A8944  addi r3, r10, -0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30396;
	// 83257B3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257B40: 4AFD5391  bl 0x8222ced0
	ctx.lr = 0x83257B44;
	sub_8222CED0(ctx, base);
	// 83257B44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257B48: 3869AFE0  addi r3, r9, -0x5020
	ctx.r[3].s64 = ctx.r[9].s64 + -20512;
	// 83257B4C: 4BA523D5  bl 0x82ca9f20
	ctx.lr = 0x83257B50;
	sub_82CA9F20(ctx, base);
	// 83257B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257B60 size=64
    let mut pc: u32 = 0x83257B60;
    'dispatch: loop {
        match pc {
            0x83257B60 => {
    //   block [0x83257B60..0x83257BA0)
	// 83257B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257B6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257B70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257B74: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257B78: 386A8948  addi r3, r10, -0x76b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30392;
	// 83257B7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257B80: 4AFD5351  bl 0x8222ced0
	ctx.lr = 0x83257B84;
	sub_8222CED0(ctx, base);
	// 83257B84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257B88: 3869AFF0  addi r3, r9, -0x5010
	ctx.r[3].s64 = ctx.r[9].s64 + -20496;
	// 83257B8C: 4BA52395  bl 0x82ca9f20
	ctx.lr = 0x83257B90;
	sub_82CA9F20(ctx, base);
	// 83257B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257BA0 size=64
    let mut pc: u32 = 0x83257BA0;
    'dispatch: loop {
        match pc {
            0x83257BA0 => {
    //   block [0x83257BA0..0x83257BE0)
	// 83257BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257BB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257BB4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257BB8: 386A894C  addi r3, r10, -0x76b4
	ctx.r[3].s64 = ctx.r[10].s64 + -30388;
	// 83257BBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257BC0: 4AFD5311  bl 0x8222ced0
	ctx.lr = 0x83257BC4;
	sub_8222CED0(ctx, base);
	// 83257BC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257BC8: 3869B000  addi r3, r9, -0x5000
	ctx.r[3].s64 = ctx.r[9].s64 + -20480;
	// 83257BCC: 4BA52355  bl 0x82ca9f20
	ctx.lr = 0x83257BD0;
	sub_82CA9F20(ctx, base);
	// 83257BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257BE0 size=64
    let mut pc: u32 = 0x83257BE0;
    'dispatch: loop {
        match pc {
            0x83257BE0 => {
    //   block [0x83257BE0..0x83257C20)
	// 83257BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257BEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257BF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257BF4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257BF8: 386A8950  addi r3, r10, -0x76b0
	ctx.r[3].s64 = ctx.r[10].s64 + -30384;
	// 83257BFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257C00: 4AFD52D1  bl 0x8222ced0
	ctx.lr = 0x83257C04;
	sub_8222CED0(ctx, base);
	// 83257C04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257C08: 3869B010  addi r3, r9, -0x4ff0
	ctx.r[3].s64 = ctx.r[9].s64 + -20464;
	// 83257C0C: 4BA52315  bl 0x82ca9f20
	ctx.lr = 0x83257C10;
	sub_82CA9F20(ctx, base);
	// 83257C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257C20 size=64
    let mut pc: u32 = 0x83257C20;
    'dispatch: loop {
        match pc {
            0x83257C20 => {
    //   block [0x83257C20..0x83257C60)
	// 83257C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257C34: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257C38: 386A8954  addi r3, r10, -0x76ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30380;
	// 83257C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257C40: 4AFD5291  bl 0x8222ced0
	ctx.lr = 0x83257C44;
	sub_8222CED0(ctx, base);
	// 83257C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257C48: 3869B020  addi r3, r9, -0x4fe0
	ctx.r[3].s64 = ctx.r[9].s64 + -20448;
	// 83257C4C: 4BA522D5  bl 0x82ca9f20
	ctx.lr = 0x83257C50;
	sub_82CA9F20(ctx, base);
	// 83257C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257C60 size=64
    let mut pc: u32 = 0x83257C60;
    'dispatch: loop {
        match pc {
            0x83257C60 => {
    //   block [0x83257C60..0x83257CA0)
	// 83257C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257C6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257C74: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257C78: 386A8958  addi r3, r10, -0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30376;
	// 83257C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257C80: 4AFD5251  bl 0x8222ced0
	ctx.lr = 0x83257C84;
	sub_8222CED0(ctx, base);
	// 83257C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257C88: 3869B030  addi r3, r9, -0x4fd0
	ctx.r[3].s64 = ctx.r[9].s64 + -20432;
	// 83257C8C: 4BA52295  bl 0x82ca9f20
	ctx.lr = 0x83257C90;
	sub_82CA9F20(ctx, base);
	// 83257C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257CA0 size=64
    let mut pc: u32 = 0x83257CA0;
    'dispatch: loop {
        match pc {
            0x83257CA0 => {
    //   block [0x83257CA0..0x83257CE0)
	// 83257CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257CAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257CB4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257CB8: 386A895C  addi r3, r10, -0x76a4
	ctx.r[3].s64 = ctx.r[10].s64 + -30372;
	// 83257CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257CC0: 4AFD5211  bl 0x8222ced0
	ctx.lr = 0x83257CC4;
	sub_8222CED0(ctx, base);
	// 83257CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257CC8: 3869B040  addi r3, r9, -0x4fc0
	ctx.r[3].s64 = ctx.r[9].s64 + -20416;
	// 83257CCC: 4BA52255  bl 0x82ca9f20
	ctx.lr = 0x83257CD0;
	sub_82CA9F20(ctx, base);
	// 83257CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257CE0 size=56
    let mut pc: u32 = 0x83257CE0;
    'dispatch: loop {
        match pc {
            0x83257CE0 => {
    //   block [0x83257CE0..0x83257D18)
	// 83257CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257CEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257CF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257CF4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83257CF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257CFC: 4AF9C05D  bl 0x821f3d58
	ctx.lr = 0x83257D00;
	sub_821F3D58(ctx, base);
	// 83257D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D04: 906A8960  stw r3, -0x76a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30368 as u32), ctx.r[3].u32 ) };
	// 83257D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D18 size=56
    let mut pc: u32 = 0x83257D18;
    'dispatch: loop {
        match pc {
            0x83257D18 => {
    //   block [0x83257D18..0x83257D50)
	// 83257D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D24: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257D28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D2C: 386B4DBC  addi r3, r11, 0x4dbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19900;
	// 83257D30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257D34: 4AF9C025  bl 0x821f3d58
	ctx.lr = 0x83257D38;
	sub_821F3D58(ctx, base);
	// 83257D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D3C: 906A8964  stw r3, -0x769c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30364 as u32), ctx.r[3].u32 ) };
	// 83257D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D50 size=56
    let mut pc: u32 = 0x83257D50;
    'dispatch: loop {
        match pc {
            0x83257D50 => {
    //   block [0x83257D50..0x83257D88)
	// 83257D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257D60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D64: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83257D68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257D6C: 4AF9BFED  bl 0x821f3d58
	ctx.lr = 0x83257D70;
	sub_821F3D58(ctx, base);
	// 83257D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D74: 906A8968  stw r3, -0x7698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30360 as u32), ctx.r[3].u32 ) };
	// 83257D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D88 size=56
    let mut pc: u32 = 0x83257D88;
    'dispatch: loop {
        match pc {
            0x83257D88 => {
    //   block [0x83257D88..0x83257DC0)
	// 83257D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257D98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D9C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83257DA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257DA4: 4AF9BFB5  bl 0x821f3d58
	ctx.lr = 0x83257DA8;
	sub_821F3D58(ctx, base);
	// 83257DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257DAC: 906A896C  stw r3, -0x7694(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30356 as u32), ctx.r[3].u32 ) };
	// 83257DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257DC0 size=56
    let mut pc: u32 = 0x83257DC0;
    'dispatch: loop {
        match pc {
            0x83257DC0 => {
    //   block [0x83257DC0..0x83257DF8)
	// 83257DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257DCC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257DD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257DD4: 386B4DD4  addi r3, r11, 0x4dd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19924;
	// 83257DD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257DDC: 4AF9BF7D  bl 0x821f3d58
	ctx.lr = 0x83257DE0;
	sub_821F3D58(ctx, base);
	// 83257DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257DE4: 906A8970  stw r3, -0x7690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30352 as u32), ctx.r[3].u32 ) };
	// 83257DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257DF8 size=56
    let mut pc: u32 = 0x83257DF8;
    'dispatch: loop {
        match pc {
            0x83257DF8 => {
    //   block [0x83257DF8..0x83257E30)
	// 83257DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257E08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257E0C: 386B4DEC  addi r3, r11, 0x4dec
	ctx.r[3].s64 = ctx.r[11].s64 + 19948;
	// 83257E10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257E14: 4AF9BF45  bl 0x821f3d58
	ctx.lr = 0x83257E18;
	sub_821F3D58(ctx, base);
	// 83257E18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E1C: 906A8974  stw r3, -0x768c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30348 as u32), ctx.r[3].u32 ) };
	// 83257E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257E30 size=64
    let mut pc: u32 = 0x83257E30;
    'dispatch: loop {
        match pc {
            0x83257E30 => {
    //   block [0x83257E30..0x83257E70)
	// 83257E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E44: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257E48: 386A8978  addi r3, r10, -0x7688
	ctx.r[3].s64 = ctx.r[10].s64 + -30344;
	// 83257E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257E50: 4AFD5081  bl 0x8222ced0
	ctx.lr = 0x83257E54;
	sub_8222CED0(ctx, base);
	// 83257E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257E58: 3869B050  addi r3, r9, -0x4fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -20400;
	// 83257E5C: 4BA520C5  bl 0x82ca9f20
	ctx.lr = 0x83257E60;
	sub_82CA9F20(ctx, base);
	// 83257E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257E70 size=64
    let mut pc: u32 = 0x83257E70;
    'dispatch: loop {
        match pc {
            0x83257E70 => {
    //   block [0x83257E70..0x83257EB0)
	// 83257E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257E80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E84: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257E88: 386A897C  addi r3, r10, -0x7684
	ctx.r[3].s64 = ctx.r[10].s64 + -30340;
	// 83257E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257E90: 4AFD5041  bl 0x8222ced0
	ctx.lr = 0x83257E94;
	sub_8222CED0(ctx, base);
	// 83257E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257E98: 3869B060  addi r3, r9, -0x4fa0
	ctx.r[3].s64 = ctx.r[9].s64 + -20384;
	// 83257E9C: 4BA52085  bl 0x82ca9f20
	ctx.lr = 0x83257EA0;
	sub_82CA9F20(ctx, base);
	// 83257EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257EB0 size=64
    let mut pc: u32 = 0x83257EB0;
    'dispatch: loop {
        match pc {
            0x83257EB0 => {
    //   block [0x83257EB0..0x83257EF0)
	// 83257EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257EC4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257EC8: 386A8980  addi r3, r10, -0x7680
	ctx.r[3].s64 = ctx.r[10].s64 + -30336;
	// 83257ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257ED0: 4AFD5001  bl 0x8222ced0
	ctx.lr = 0x83257ED4;
	sub_8222CED0(ctx, base);
	// 83257ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257ED8: 3869B070  addi r3, r9, -0x4f90
	ctx.r[3].s64 = ctx.r[9].s64 + -20368;
	// 83257EDC: 4BA52045  bl 0x82ca9f20
	ctx.lr = 0x83257EE0;
	sub_82CA9F20(ctx, base);
	// 83257EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257EF0 size=64
    let mut pc: u32 = 0x83257EF0;
    'dispatch: loop {
        match pc {
            0x83257EF0 => {
    //   block [0x83257EF0..0x83257F30)
	// 83257EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257EFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257F04: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257F08: 386A8984  addi r3, r10, -0x767c
	ctx.r[3].s64 = ctx.r[10].s64 + -30332;
	// 83257F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257F10: 4AFD4FC1  bl 0x8222ced0
	ctx.lr = 0x83257F14;
	sub_8222CED0(ctx, base);
	// 83257F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257F18: 3869B080  addi r3, r9, -0x4f80
	ctx.r[3].s64 = ctx.r[9].s64 + -20352;
	// 83257F1C: 4BA52005  bl 0x82ca9f20
	ctx.lr = 0x83257F20;
	sub_82CA9F20(ctx, base);
	// 83257F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257F30 size=64
    let mut pc: u32 = 0x83257F30;
    'dispatch: loop {
        match pc {
            0x83257F30 => {
    //   block [0x83257F30..0x83257F70)
	// 83257F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257F3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257F44: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257F48: 386A8988  addi r3, r10, -0x7678
	ctx.r[3].s64 = ctx.r[10].s64 + -30328;
	// 83257F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257F50: 4AFD4F81  bl 0x8222ced0
	ctx.lr = 0x83257F54;
	sub_8222CED0(ctx, base);
	// 83257F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257F58: 3869B090  addi r3, r9, -0x4f70
	ctx.r[3].s64 = ctx.r[9].s64 + -20336;
	// 83257F5C: 4BA51FC5  bl 0x82ca9f20
	ctx.lr = 0x83257F60;
	sub_82CA9F20(ctx, base);
	// 83257F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257F70 size=64
    let mut pc: u32 = 0x83257F70;
    'dispatch: loop {
        match pc {
            0x83257F70 => {
    //   block [0x83257F70..0x83257FB0)
	// 83257F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257F84: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257F88: 386A898C  addi r3, r10, -0x7674
	ctx.r[3].s64 = ctx.r[10].s64 + -30324;
	// 83257F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257F90: 4AFD4F41  bl 0x8222ced0
	ctx.lr = 0x83257F94;
	sub_8222CED0(ctx, base);
	// 83257F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257F98: 3869B0A0  addi r3, r9, -0x4f60
	ctx.r[3].s64 = ctx.r[9].s64 + -20320;
	// 83257F9C: 4BA51F85  bl 0x82ca9f20
	ctx.lr = 0x83257FA0;
	sub_82CA9F20(ctx, base);
	// 83257FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257FB0 size=64
    let mut pc: u32 = 0x83257FB0;
    'dispatch: loop {
        match pc {
            0x83257FB0 => {
    //   block [0x83257FB0..0x83257FF0)
	// 83257FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257FBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257FC4: 388B5304  addi r4, r11, 0x5304
	ctx.r[4].s64 = ctx.r[11].s64 + 21252;
	// 83257FC8: 386A8990  addi r3, r10, -0x7670
	ctx.r[3].s64 = ctx.r[10].s64 + -30320;
	// 83257FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257FD0: 4AFD4F01  bl 0x8222ced0
	ctx.lr = 0x83257FD4;
	sub_8222CED0(ctx, base);
	// 83257FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257FD8: 3869B0B0  addi r3, r9, -0x4f50
	ctx.r[3].s64 = ctx.r[9].s64 + -20304;
	// 83257FDC: 4BA51F45  bl 0x82ca9f20
	ctx.lr = 0x83257FE0;
	sub_82CA9F20(ctx, base);
	// 83257FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257FF0 size=64
    let mut pc: u32 = 0x83257FF0;
    'dispatch: loop {
        match pc {
            0x83257FF0 => {
    //   block [0x83257FF0..0x83258030)
	// 83257FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258004: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258008: 386A8994  addi r3, r10, -0x766c
	ctx.r[3].s64 = ctx.r[10].s64 + -30316;
	// 8325800C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258010: 4AFD4EC1  bl 0x8222ced0
	ctx.lr = 0x83258014;
	sub_8222CED0(ctx, base);
	// 83258014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258018: 3869B0C0  addi r3, r9, -0x4f40
	ctx.r[3].s64 = ctx.r[9].s64 + -20288;
	// 8325801C: 4BA51F05  bl 0x82ca9f20
	ctx.lr = 0x83258020;
	sub_82CA9F20(ctx, base);
	// 83258020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258030 size=64
    let mut pc: u32 = 0x83258030;
    'dispatch: loop {
        match pc {
            0x83258030 => {
    //   block [0x83258030..0x83258070)
	// 83258030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325803C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258044: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258048: 386A8998  addi r3, r10, -0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + -30312;
	// 8325804C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258050: 4AFD4E81  bl 0x8222ced0
	ctx.lr = 0x83258054;
	sub_8222CED0(ctx, base);
	// 83258054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258058: 3869B0D0  addi r3, r9, -0x4f30
	ctx.r[3].s64 = ctx.r[9].s64 + -20272;
	// 8325805C: 4BA51EC5  bl 0x82ca9f20
	ctx.lr = 0x83258060;
	sub_82CA9F20(ctx, base);
	// 83258060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258070 size=64
    let mut pc: u32 = 0x83258070;
    'dispatch: loop {
        match pc {
            0x83258070 => {
    //   block [0x83258070..0x832580B0)
	// 83258070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325807C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258084: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258088: 386A899C  addi r3, r10, -0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + -30308;
	// 8325808C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258090: 4AFD4E41  bl 0x8222ced0
	ctx.lr = 0x83258094;
	sub_8222CED0(ctx, base);
	// 83258094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258098: 3869B0E0  addi r3, r9, -0x4f20
	ctx.r[3].s64 = ctx.r[9].s64 + -20256;
	// 8325809C: 4BA51E85  bl 0x82ca9f20
	ctx.lr = 0x832580A0;
	sub_82CA9F20(ctx, base);
	// 832580A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832580A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832580A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832580AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832580B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832580B0 size=12
    let mut pc: u32 = 0x832580B0;
    'dispatch: loop {
        match pc {
            0x832580B0 => {
    //   block [0x832580B0..0x832580BC)
	// 832580B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832580B4: 386B89A0  addi r3, r11, -0x7660
	ctx.r[3].s64 = ctx.r[11].s64 + -30304;
	// 832580B8: 4AFE3CF8  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832580C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832580C0 size=64
    let mut pc: u32 = 0x832580C0;
    'dispatch: loop {
        match pc {
            0x832580C0 => {
    //   block [0x832580C0..0x83258100)
	// 832580C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832580C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832580C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832580CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832580D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832580D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832580D8: 386A89E0  addi r3, r10, -0x7620
	ctx.r[3].s64 = ctx.r[10].s64 + -30240;
	// 832580DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832580E0: 4AFD4DF1  bl 0x8222ced0
	ctx.lr = 0x832580E4;
	sub_8222CED0(ctx, base);
	// 832580E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832580E8: 3869B0F0  addi r3, r9, -0x4f10
	ctx.r[3].s64 = ctx.r[9].s64 + -20240;
	// 832580EC: 4BA51E35  bl 0x82ca9f20
	ctx.lr = 0x832580F0;
	sub_82CA9F20(ctx, base);
	// 832580F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832580F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832580F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832580FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258100 size=64
    let mut pc: u32 = 0x83258100;
    'dispatch: loop {
        match pc {
            0x83258100 => {
    //   block [0x83258100..0x83258140)
	// 83258100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325810C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258114: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258118: 386A89E4  addi r3, r10, -0x761c
	ctx.r[3].s64 = ctx.r[10].s64 + -30236;
	// 8325811C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258120: 4AFD4DB1  bl 0x8222ced0
	ctx.lr = 0x83258124;
	sub_8222CED0(ctx, base);
	// 83258124: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258128: 3869B100  addi r3, r9, -0x4f00
	ctx.r[3].s64 = ctx.r[9].s64 + -20224;
	// 8325812C: 4BA51DF5  bl 0x82ca9f20
	ctx.lr = 0x83258130;
	sub_82CA9F20(ctx, base);
	// 83258130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258140 size=64
    let mut pc: u32 = 0x83258140;
    'dispatch: loop {
        match pc {
            0x83258140 => {
    //   block [0x83258140..0x83258180)
	// 83258140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325814C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258154: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258158: 386A89E8  addi r3, r10, -0x7618
	ctx.r[3].s64 = ctx.r[10].s64 + -30232;
	// 8325815C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258160: 4AFD4D71  bl 0x8222ced0
	ctx.lr = 0x83258164;
	sub_8222CED0(ctx, base);
	// 83258164: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258168: 3869B110  addi r3, r9, -0x4ef0
	ctx.r[3].s64 = ctx.r[9].s64 + -20208;
	// 8325816C: 4BA51DB5  bl 0x82ca9f20
	ctx.lr = 0x83258170;
	sub_82CA9F20(ctx, base);
	// 83258170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325817C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258180 size=12
    let mut pc: u32 = 0x83258180;
    'dispatch: loop {
        match pc {
            0x83258180 => {
    //   block [0x83258180..0x8325818C)
	// 83258180: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83258184: 386BB120  addi r3, r11, -0x4ee0
	ctx.r[3].s64 = ctx.r[11].s64 + -20192;
	// 83258188: 4BA51D98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258190 size=12
    let mut pc: u32 = 0x83258190;
    'dispatch: loop {
        match pc {
            0x83258190 => {
    //   block [0x83258190..0x8325819C)
	// 83258190: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83258194: 386BB188  addi r3, r11, -0x4e78
	ctx.r[3].s64 = ctx.r[11].s64 + -20088;
	// 83258198: 4BA51D88  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832581A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832581A0 size=12
    let mut pc: u32 = 0x832581A0;
    'dispatch: loop {
        match pc {
            0x832581A0 => {
    //   block [0x832581A0..0x832581AC)
	// 832581A0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832581A4: 386BB1F0  addi r3, r11, -0x4e10
	ctx.r[3].s64 = ctx.r[11].s64 + -19984;
	// 832581A8: 4BA51D78  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832581B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832581B0 size=64
    let mut pc: u32 = 0x832581B0;
    'dispatch: loop {
        match pc {
            0x832581B0 => {
    //   block [0x832581B0..0x832581F0)
	// 832581B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832581B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832581B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832581BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832581C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832581C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832581C8: 386A8A1C  addi r3, r10, -0x75e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30180;
	// 832581CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832581D0: 4AFD4D01  bl 0x8222ced0
	ctx.lr = 0x832581D4;
	sub_8222CED0(ctx, base);
	// 832581D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832581D8: 3869B248  addi r3, r9, -0x4db8
	ctx.r[3].s64 = ctx.r[9].s64 + -19896;
	// 832581DC: 4BA51D45  bl 0x82ca9f20
	ctx.lr = 0x832581E0;
	sub_82CA9F20(ctx, base);
	// 832581E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832581E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832581E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832581EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832581F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832581F0 size=64
    let mut pc: u32 = 0x832581F0;
    'dispatch: loop {
        match pc {
            0x832581F0 => {
    //   block [0x832581F0..0x83258230)
	// 832581F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832581F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832581F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832581FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258204: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258208: 386A8A20  addi r3, r10, -0x75e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30176;
	// 8325820C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258210: 4AFD4CC1  bl 0x8222ced0
	ctx.lr = 0x83258214;
	sub_8222CED0(ctx, base);
	// 83258214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258218: 3869B258  addi r3, r9, -0x4da8
	ctx.r[3].s64 = ctx.r[9].s64 + -19880;
	// 8325821C: 4BA51D05  bl 0x82ca9f20
	ctx.lr = 0x83258220;
	sub_82CA9F20(ctx, base);
	// 83258220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325822C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258230 size=64
    let mut pc: u32 = 0x83258230;
    'dispatch: loop {
        match pc {
            0x83258230 => {
    //   block [0x83258230..0x83258270)
	// 83258230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325823C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258244: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258248: 386A8A24  addi r3, r10, -0x75dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30172;
	// 8325824C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258250: 4AFD4C81  bl 0x8222ced0
	ctx.lr = 0x83258254;
	sub_8222CED0(ctx, base);
	// 83258254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258258: 3869B268  addi r3, r9, -0x4d98
	ctx.r[3].s64 = ctx.r[9].s64 + -19864;
	// 8325825C: 4BA51CC5  bl 0x82ca9f20
	ctx.lr = 0x83258260;
	sub_82CA9F20(ctx, base);
	// 83258260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325826C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258270 size=64
    let mut pc: u32 = 0x83258270;
    'dispatch: loop {
        match pc {
            0x83258270 => {
    //   block [0x83258270..0x832582B0)
	// 83258270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325827C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258284: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258288: 386A8A28  addi r3, r10, -0x75d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30168;
	// 8325828C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258290: 4AFD4C41  bl 0x8222ced0
	ctx.lr = 0x83258294;
	sub_8222CED0(ctx, base);
	// 83258294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258298: 3869B278  addi r3, r9, -0x4d88
	ctx.r[3].s64 = ctx.r[9].s64 + -19848;
	// 8325829C: 4BA51C85  bl 0x82ca9f20
	ctx.lr = 0x832582A0;
	sub_82CA9F20(ctx, base);
	// 832582A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832582A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832582A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832582AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832582B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832582B0 size=64
    let mut pc: u32 = 0x832582B0;
    'dispatch: loop {
        match pc {
            0x832582B0 => {
    //   block [0x832582B0..0x832582F0)
	// 832582B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832582B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832582B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832582BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832582C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832582C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832582C8: 386A8A2C  addi r3, r10, -0x75d4
	ctx.r[3].s64 = ctx.r[10].s64 + -30164;
	// 832582CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832582D0: 4AFD4C01  bl 0x8222ced0
	ctx.lr = 0x832582D4;
	sub_8222CED0(ctx, base);
	// 832582D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832582D8: 3869B288  addi r3, r9, -0x4d78
	ctx.r[3].s64 = ctx.r[9].s64 + -19832;
	// 832582DC: 4BA51C45  bl 0x82ca9f20
	ctx.lr = 0x832582E0;
	sub_82CA9F20(ctx, base);
	// 832582E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832582E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832582E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832582EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832582F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832582F0 size=64
    let mut pc: u32 = 0x832582F0;
    'dispatch: loop {
        match pc {
            0x832582F0 => {
    //   block [0x832582F0..0x83258330)
	// 832582F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832582F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832582F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832582FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258304: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258308: 386A8A30  addi r3, r10, -0x75d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30160;
	// 8325830C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258310: 4AFD4BC1  bl 0x8222ced0
	ctx.lr = 0x83258314;
	sub_8222CED0(ctx, base);
	// 83258314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258318: 3869B298  addi r3, r9, -0x4d68
	ctx.r[3].s64 = ctx.r[9].s64 + -19816;
	// 8325831C: 4BA51C05  bl 0x82ca9f20
	ctx.lr = 0x83258320;
	sub_82CA9F20(ctx, base);
	// 83258320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325832C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258330 size=64
    let mut pc: u32 = 0x83258330;
    'dispatch: loop {
        match pc {
            0x83258330 => {
    //   block [0x83258330..0x83258370)
	// 83258330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325833C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258344: 388B2EF4  addi r4, r11, 0x2ef4
	ctx.r[4].s64 = ctx.r[11].s64 + 12020;
	// 83258348: 386A8A34  addi r3, r10, -0x75cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30156;
	// 8325834C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258350: 4AFD4B81  bl 0x8222ced0
	ctx.lr = 0x83258354;
	sub_8222CED0(ctx, base);
	// 83258354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258358: 3869B2A8  addi r3, r9, -0x4d58
	ctx.r[3].s64 = ctx.r[9].s64 + -19800;
	// 8325835C: 4BA51BC5  bl 0x82ca9f20
	ctx.lr = 0x83258360;
	sub_82CA9F20(ctx, base);
	// 83258360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258370 size=696
    let mut pc: u32 = 0x83258370;
    'dispatch: loop {
        match pc {
            0x83258370 => {
    //   block [0x83258370..0x83258628)
	// 83258370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325837C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258384: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83258388: 3BEB8A38  addi r31, r11, -0x75c8
	ctx.r[31].s64 = ctx.r[11].s64 + -30152;
	// 8325838C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83258390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83258394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258398: 4AFD4B39  bl 0x8222ced0
	ctx.lr = 0x8325839C;
	sub_8222CED0(ctx, base);
	// 8325839C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832583A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583A4: 3889674C  addi r4, r9, 0x674c
	ctx.r[4].s64 = ctx.r[9].s64 + 26444;
	// 832583A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832583AC: 4AFD4B25  bl 0x8222ced0
	ctx.lr = 0x832583B0;
	sub_8222CED0(ctx, base);
	// 832583B0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832583B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583B8: 38886748  addi r4, r8, 0x6748
	ctx.r[4].s64 = ctx.r[8].s64 + 26440;
	// 832583BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832583C0: 4AFD4B11  bl 0x8222ced0
	ctx.lr = 0x832583C4;
	sub_8222CED0(ctx, base);
	// 832583C4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832583C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583CC: 38876744  addi r4, r7, 0x6744
	ctx.r[4].s64 = ctx.r[7].s64 + 26436;
	// 832583D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832583D4: 4AFD4AFD  bl 0x8222ced0
	ctx.lr = 0x832583D8;
	sub_8222CED0(ctx, base);
	// 832583D8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832583DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583E0: 38866740  addi r4, r6, 0x6740
	ctx.r[4].s64 = ctx.r[6].s64 + 26432;
	// 832583E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832583E8: 4AFD4AE9  bl 0x8222ced0
	ctx.lr = 0x832583EC;
	sub_8222CED0(ctx, base);
	// 832583EC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832583F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583F4: 3884673C  addi r4, r4, 0x673c
	ctx.r[4].s64 = ctx.r[4].s64 + 26428;
	// 832583F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832583FC: 4AFD4AD5  bl 0x8222ced0
	ctx.lr = 0x83258400;
	sub_8222CED0(ctx, base);
	// 83258400: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258408: 38836738  addi r4, r3, 0x6738
	ctx.r[4].s64 = ctx.r[3].s64 + 26424;
	// 8325840C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83258410: 4AFD4AC1  bl 0x8222ced0
	ctx.lr = 0x83258414;
	sub_8222CED0(ctx, base);
	// 83258414: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258418: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325841C: 388B6734  addi r4, r11, 0x6734
	ctx.r[4].s64 = ctx.r[11].s64 + 26420;
	// 83258420: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83258424: 4AFD4AAD  bl 0x8222ced0
	ctx.lr = 0x83258428;
	sub_8222CED0(ctx, base);
	// 83258428: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325842C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258430: 388A6730  addi r4, r10, 0x6730
	ctx.r[4].s64 = ctx.r[10].s64 + 26416;
	// 83258434: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83258438: 4AFD4A99  bl 0x8222ced0
	ctx.lr = 0x8325843C;
	sub_8222CED0(ctx, base);
	// 8325843C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258440: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258444: 3889672C  addi r4, r9, 0x672c
	ctx.r[4].s64 = ctx.r[9].s64 + 26412;
	// 83258448: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325844C: 4AFD4A85  bl 0x8222ced0
	ctx.lr = 0x83258450;
	sub_8222CED0(ctx, base);
	// 83258450: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258458: 38886728  addi r4, r8, 0x6728
	ctx.r[4].s64 = ctx.r[8].s64 + 26408;
	// 8325845C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83258460: 4AFD4A71  bl 0x8222ced0
	ctx.lr = 0x83258464;
	sub_8222CED0(ctx, base);
	// 83258464: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258468: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325846C: 38876724  addi r4, r7, 0x6724
	ctx.r[4].s64 = ctx.r[7].s64 + 26404;
	// 83258470: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83258474: 4AFD4A5D  bl 0x8222ced0
	ctx.lr = 0x83258478;
	sub_8222CED0(ctx, base);
	// 83258478: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325847C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258480: 38866720  addi r4, r6, 0x6720
	ctx.r[4].s64 = ctx.r[6].s64 + 26400;
	// 83258484: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83258488: 4AFD4A49  bl 0x8222ced0
	ctx.lr = 0x8325848C;
	sub_8222CED0(ctx, base);
	// 8325848C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258490: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258494: 3884671C  addi r4, r4, 0x671c
	ctx.r[4].s64 = ctx.r[4].s64 + 26396;
	// 83258498: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8325849C: 4AFD4A35  bl 0x8222ced0
	ctx.lr = 0x832584A0;
	sub_8222CED0(ctx, base);
	// 832584A0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832584A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584A8: 38836718  addi r4, r3, 0x6718
	ctx.r[4].s64 = ctx.r[3].s64 + 26392;
	// 832584AC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832584B0: 4AFD4A21  bl 0x8222ced0
	ctx.lr = 0x832584B4;
	sub_8222CED0(ctx, base);
	// 832584B4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832584B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584BC: 388B6714  addi r4, r11, 0x6714
	ctx.r[4].s64 = ctx.r[11].s64 + 26388;
	// 832584C0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832584C4: 4AFD4A0D  bl 0x8222ced0
	ctx.lr = 0x832584C8;
	sub_8222CED0(ctx, base);
	// 832584C8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832584CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584D0: 388A6710  addi r4, r10, 0x6710
	ctx.r[4].s64 = ctx.r[10].s64 + 26384;
	// 832584D4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832584D8: 4AFD49F9  bl 0x8222ced0
	ctx.lr = 0x832584DC;
	sub_8222CED0(ctx, base);
	// 832584DC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832584E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584E4: 38896708  addi r4, r9, 0x6708
	ctx.r[4].s64 = ctx.r[9].s64 + 26376;
	// 832584E8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 832584EC: 4AFD49E5  bl 0x8222ced0
	ctx.lr = 0x832584F0;
	sub_8222CED0(ctx, base);
	// 832584F0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832584F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584F8: 38886700  addi r4, r8, 0x6700
	ctx.r[4].s64 = ctx.r[8].s64 + 26368;
	// 832584FC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83258500: 4AFD49D1  bl 0x8222ced0
	ctx.lr = 0x83258504;
	sub_8222CED0(ctx, base);
	// 83258504: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258508: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325850C: 388766F8  addi r4, r7, 0x66f8
	ctx.r[4].s64 = ctx.r[7].s64 + 26360;
	// 83258510: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83258514: 4AFD49BD  bl 0x8222ced0
	ctx.lr = 0x83258518;
	sub_8222CED0(ctx, base);
	// 83258518: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325851C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258520: 388666F0  addi r4, r6, 0x66f0
	ctx.r[4].s64 = ctx.r[6].s64 + 26352;
	// 83258524: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83258528: 4AFD49A9  bl 0x8222ced0
	ctx.lr = 0x8325852C;
	sub_8222CED0(ctx, base);
	// 8325852C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258530: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258534: 388466E8  addi r4, r4, 0x66e8
	ctx.r[4].s64 = ctx.r[4].s64 + 26344;
	// 83258538: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8325853C: 4AFD4995  bl 0x8222ced0
	ctx.lr = 0x83258540;
	sub_8222CED0(ctx, base);
	// 83258540: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258548: 388366E0  addi r4, r3, 0x66e0
	ctx.r[4].s64 = ctx.r[3].s64 + 26336;
	// 8325854C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83258550: 4AFD4981  bl 0x8222ced0
	ctx.lr = 0x83258554;
	sub_8222CED0(ctx, base);
	// 83258554: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258558: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325855C: 388B66D8  addi r4, r11, 0x66d8
	ctx.r[4].s64 = ctx.r[11].s64 + 26328;
	// 83258560: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83258564: 4AFD496D  bl 0x8222ced0
	ctx.lr = 0x83258568;
	sub_8222CED0(ctx, base);
	// 83258568: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325856C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258570: 388A66D0  addi r4, r10, 0x66d0
	ctx.r[4].s64 = ctx.r[10].s64 + 26320;
	// 83258574: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83258578: 4AFD4959  bl 0x8222ced0
	ctx.lr = 0x8325857C;
	sub_8222CED0(ctx, base);
	// 8325857C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258580: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258584: 388966C8  addi r4, r9, 0x66c8
	ctx.r[4].s64 = ctx.r[9].s64 + 26312;
	// 83258588: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8325858C: 4AFD4945  bl 0x8222ced0
	ctx.lr = 0x83258590;
	sub_8222CED0(ctx, base);
	// 83258590: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258598: 388866C0  addi r4, r8, 0x66c0
	ctx.r[4].s64 = ctx.r[8].s64 + 26304;
	// 8325859C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 832585A0: 4AFD4931  bl 0x8222ced0
	ctx.lr = 0x832585A4;
	sub_8222CED0(ctx, base);
	// 832585A4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832585A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585AC: 388766B8  addi r4, r7, 0x66b8
	ctx.r[4].s64 = ctx.r[7].s64 + 26296;
	// 832585B0: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 832585B4: 4AFD491D  bl 0x8222ced0
	ctx.lr = 0x832585B8;
	sub_8222CED0(ctx, base);
	// 832585B8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832585BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585C0: 388666B0  addi r4, r6, 0x66b0
	ctx.r[4].s64 = ctx.r[6].s64 + 26288;
	// 832585C4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 832585C8: 4AFD4909  bl 0x8222ced0
	ctx.lr = 0x832585CC;
	sub_8222CED0(ctx, base);
	// 832585CC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832585D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585D4: 388466A8  addi r4, r4, 0x66a8
	ctx.r[4].s64 = ctx.r[4].s64 + 26280;
	// 832585D8: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 832585DC: 4AFD48F5  bl 0x8222ced0
	ctx.lr = 0x832585E0;
	sub_8222CED0(ctx, base);
	// 832585E0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832585E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585E8: 388366A0  addi r4, r3, 0x66a0
	ctx.r[4].s64 = ctx.r[3].s64 + 26272;
	// 832585EC: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 832585F0: 4AFD48E1  bl 0x8222ced0
	ctx.lr = 0x832585F4;
	sub_8222CED0(ctx, base);
	// 832585F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832585F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585FC: 388B6698  addi r4, r11, 0x6698
	ctx.r[4].s64 = ctx.r[11].s64 + 26264;
	// 83258600: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83258604: 4AFD48CD  bl 0x8222ced0
	ctx.lr = 0x83258608;
	sub_8222CED0(ctx, base);
	// 83258608: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325860C: 386AB2B8  addi r3, r10, -0x4d48
	ctx.r[3].s64 = ctx.r[10].s64 + -19784;
	// 83258610: 4BA51911  bl 0x82ca9f20
	ctx.lr = 0x83258614;
	sub_82CA9F20(ctx, base);
	// 83258614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325861C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258628 size=12
    let mut pc: u32 = 0x83258628;
    'dispatch: loop {
        match pc {
            0x83258628 => {
    //   block [0x83258628..0x83258634)
	// 83258628: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325862C: 386BB320  addi r3, r11, -0x4ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -19680;
	// 83258630: 4BA518F0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258638 size=56
    let mut pc: u32 = 0x83258638;
    'dispatch: loop {
        match pc {
            0x83258638 => {
    //   block [0x83258638..0x83258670)
	// 83258638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325863C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258644: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325864C: 386B6E20  addi r3, r11, 0x6e20
	ctx.r[3].s64 = ctx.r[11].s64 + 28192;
	// 83258650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258654: 4AF9B705  bl 0x821f3d58
	ctx.lr = 0x83258658;
	sub_821F3D58(ctx, base);
	// 83258658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325865C: 906A8ACC  stw r3, -0x7534(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30004 as u32), ctx.r[3].u32 ) };
	// 83258660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325866C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258670 size=64
    let mut pc: u32 = 0x83258670;
    'dispatch: loop {
        match pc {
            0x83258670 => {
    //   block [0x83258670..0x832586B0)
	// 83258670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325867C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258684: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258688: 386A8AD0  addi r3, r10, -0x7530
	ctx.r[3].s64 = ctx.r[10].s64 + -30000;
	// 8325868C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258690: 4AFD4841  bl 0x8222ced0
	ctx.lr = 0x83258694;
	sub_8222CED0(ctx, base);
	// 83258694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258698: 3869B378  addi r3, r9, -0x4c88
	ctx.r[3].s64 = ctx.r[9].s64 + -19592;
	// 8325869C: 4BA51885  bl 0x82ca9f20
	ctx.lr = 0x832586A0;
	sub_82CA9F20(ctx, base);
	// 832586A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832586A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832586A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832586AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832586B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832586B0 size=64
    let mut pc: u32 = 0x832586B0;
    'dispatch: loop {
        match pc {
            0x832586B0 => {
    //   block [0x832586B0..0x832586F0)
	// 832586B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832586B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832586B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832586BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832586C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832586C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832586C8: 386A8AD4  addi r3, r10, -0x752c
	ctx.r[3].s64 = ctx.r[10].s64 + -29996;
	// 832586CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832586D0: 4AFD4801  bl 0x8222ced0
	ctx.lr = 0x832586D4;
	sub_8222CED0(ctx, base);
	// 832586D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832586D8: 3869B388  addi r3, r9, -0x4c78
	ctx.r[3].s64 = ctx.r[9].s64 + -19576;
	// 832586DC: 4BA51845  bl 0x82ca9f20
	ctx.lr = 0x832586E0;
	sub_82CA9F20(ctx, base);
	// 832586E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832586E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832586E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832586EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832586F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832586F0 size=64
    let mut pc: u32 = 0x832586F0;
    'dispatch: loop {
        match pc {
            0x832586F0 => {
    //   block [0x832586F0..0x83258730)
	// 832586F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832586F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832586F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832586FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258704: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258708: 386A8AD8  addi r3, r10, -0x7528
	ctx.r[3].s64 = ctx.r[10].s64 + -29992;
	// 8325870C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258710: 4AFD47C1  bl 0x8222ced0
	ctx.lr = 0x83258714;
	sub_8222CED0(ctx, base);
	// 83258714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258718: 3869B398  addi r3, r9, -0x4c68
	ctx.r[3].s64 = ctx.r[9].s64 + -19560;
	// 8325871C: 4BA51805  bl 0x82ca9f20
	ctx.lr = 0x83258720;
	sub_82CA9F20(ctx, base);
	// 83258720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325872C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258730 size=64
    let mut pc: u32 = 0x83258730;
    'dispatch: loop {
        match pc {
            0x83258730 => {
    //   block [0x83258730..0x83258770)
	// 83258730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325873C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258744: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258748: 386A8ADC  addi r3, r10, -0x7524
	ctx.r[3].s64 = ctx.r[10].s64 + -29988;
	// 8325874C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258750: 4AFD4781  bl 0x8222ced0
	ctx.lr = 0x83258754;
	sub_8222CED0(ctx, base);
	// 83258754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258758: 3869B3A8  addi r3, r9, -0x4c58
	ctx.r[3].s64 = ctx.r[9].s64 + -19544;
	// 8325875C: 4BA517C5  bl 0x82ca9f20
	ctx.lr = 0x83258760;
	sub_82CA9F20(ctx, base);
	// 83258760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325876C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258770 size=64
    let mut pc: u32 = 0x83258770;
    'dispatch: loop {
        match pc {
            0x83258770 => {
    //   block [0x83258770..0x832587B0)
	// 83258770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325877C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258784: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258788: 386A8AE0  addi r3, r10, -0x7520
	ctx.r[3].s64 = ctx.r[10].s64 + -29984;
	// 8325878C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258790: 4AFD4741  bl 0x8222ced0
	ctx.lr = 0x83258794;
	sub_8222CED0(ctx, base);
	// 83258794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258798: 3869B3B8  addi r3, r9, -0x4c48
	ctx.r[3].s64 = ctx.r[9].s64 + -19528;
	// 8325879C: 4BA51785  bl 0x82ca9f20
	ctx.lr = 0x832587A0;
	sub_82CA9F20(ctx, base);
	// 832587A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832587A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832587A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832587AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832587B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832587B0 size=64
    let mut pc: u32 = 0x832587B0;
    'dispatch: loop {
        match pc {
            0x832587B0 => {
    //   block [0x832587B0..0x832587F0)
	// 832587B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832587B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832587B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832587BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832587C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832587C4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832587C8: 386A8AE4  addi r3, r10, -0x751c
	ctx.r[3].s64 = ctx.r[10].s64 + -29980;
	// 832587CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832587D0: 4AFD4701  bl 0x8222ced0
	ctx.lr = 0x832587D4;
	sub_8222CED0(ctx, base);
	// 832587D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832587D8: 3869B3C8  addi r3, r9, -0x4c38
	ctx.r[3].s64 = ctx.r[9].s64 + -19512;
	// 832587DC: 4BA51745  bl 0x82ca9f20
	ctx.lr = 0x832587E0;
	sub_82CA9F20(ctx, base);
	// 832587E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832587E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832587E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832587EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832587F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832587F0 size=56
    let mut pc: u32 = 0x832587F0;
    'dispatch: loop {
        match pc {
            0x832587F0 => {
    //   block [0x832587F0..0x83258828)
	// 832587F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832587F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832587F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832587FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258804: 386B7198  addi r3, r11, 0x7198
	ctx.r[3].s64 = ctx.r[11].s64 + 29080;
	// 83258808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325880C: 4AF9B54D  bl 0x821f3d58
	ctx.lr = 0x83258810;
	sub_821F3D58(ctx, base);
	// 83258810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258814: 906A8AE8  stw r3, -0x7518(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29976 as u32), ctx.r[3].u32 ) };
	// 83258818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325881C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258828 size=56
    let mut pc: u32 = 0x83258828;
    'dispatch: loop {
        match pc {
            0x83258828 => {
    //   block [0x83258828..0x83258860)
	// 83258828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325882C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258834: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325883C: 386B71A8  addi r3, r11, 0x71a8
	ctx.r[3].s64 = ctx.r[11].s64 + 29096;
	// 83258840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258844: 4AF9B515  bl 0x821f3d58
	ctx.lr = 0x83258848;
	sub_821F3D58(ctx, base);
	// 83258848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325884C: 906A8AEC  stw r3, -0x7514(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29972 as u32), ctx.r[3].u32 ) };
	// 83258850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325885C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258860 size=56
    let mut pc: u32 = 0x83258860;
    'dispatch: loop {
        match pc {
            0x83258860 => {
    //   block [0x83258860..0x83258898)
	// 83258860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325886C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258874: 386B71B8  addi r3, r11, 0x71b8
	ctx.r[3].s64 = ctx.r[11].s64 + 29112;
	// 83258878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325887C: 4AF9B4DD  bl 0x821f3d58
	ctx.lr = 0x83258880;
	sub_821F3D58(ctx, base);
	// 83258880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258884: 906A8AF0  stw r3, -0x7510(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29968 as u32), ctx.r[3].u32 ) };
	// 83258888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325888C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258898 size=56
    let mut pc: u32 = 0x83258898;
    'dispatch: loop {
        match pc {
            0x83258898 => {
    //   block [0x83258898..0x832588D0)
	// 83258898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325889C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832588A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832588A4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832588A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832588AC: 386B71D0  addi r3, r11, 0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29136;
	// 832588B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832588B4: 4AF9B4A5  bl 0x821f3d58
	ctx.lr = 0x832588B8;
	sub_821F3D58(ctx, base);
	// 832588B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832588BC: 906A8AF4  stw r3, -0x750c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29964 as u32), ctx.r[3].u32 ) };
	// 832588C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832588C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832588C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832588CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832588D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832588D0 size=56
    let mut pc: u32 = 0x832588D0;
    'dispatch: loop {
        match pc {
            0x832588D0 => {
    //   block [0x832588D0..0x83258908)
	// 832588D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832588D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832588D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832588DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832588E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832588E4: 386B71DC  addi r3, r11, 0x71dc
	ctx.r[3].s64 = ctx.r[11].s64 + 29148;
	// 832588E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832588EC: 4AF9B46D  bl 0x821f3d58
	ctx.lr = 0x832588F0;
	sub_821F3D58(ctx, base);
	// 832588F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832588F4: 906A8AF8  stw r3, -0x7508(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29960 as u32), ctx.r[3].u32 ) };
	// 832588F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832588FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258908 size=56
    let mut pc: u32 = 0x83258908;
    'dispatch: loop {
        match pc {
            0x83258908 => {
    //   block [0x83258908..0x83258940)
	// 83258908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325890C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258914: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325891C: 386B71EC  addi r3, r11, 0x71ec
	ctx.r[3].s64 = ctx.r[11].s64 + 29164;
	// 83258920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258924: 4AF9B435  bl 0x821f3d58
	ctx.lr = 0x83258928;
	sub_821F3D58(ctx, base);
	// 83258928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325892C: 906A8AFC  stw r3, -0x7504(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29956 as u32), ctx.r[3].u32 ) };
	// 83258930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325893C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258940 size=56
    let mut pc: u32 = 0x83258940;
    'dispatch: loop {
        match pc {
            0x83258940 => {
    //   block [0x83258940..0x83258978)
	// 83258940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325894C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258954: 386B7200  addi r3, r11, 0x7200
	ctx.r[3].s64 = ctx.r[11].s64 + 29184;
	// 83258958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325895C: 4AF9B3FD  bl 0x821f3d58
	ctx.lr = 0x83258960;
	sub_821F3D58(ctx, base);
	// 83258960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258964: 906A8B00  stw r3, -0x7500(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29952 as u32), ctx.r[3].u32 ) };
	// 83258968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325896C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258978 size=56
    let mut pc: u32 = 0x83258978;
    'dispatch: loop {
        match pc {
            0x83258978 => {
    //   block [0x83258978..0x832589B0)
	// 83258978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325897C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258984: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325898C: 386B7214  addi r3, r11, 0x7214
	ctx.r[3].s64 = ctx.r[11].s64 + 29204;
	// 83258990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258994: 4AF9B3C5  bl 0x821f3d58
	ctx.lr = 0x83258998;
	sub_821F3D58(ctx, base);
	// 83258998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325899C: 906A8B04  stw r3, -0x74fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29948 as u32), ctx.r[3].u32 ) };
	// 832589A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832589A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832589A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832589AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832589B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832589B0 size=64
    let mut pc: u32 = 0x832589B0;
    'dispatch: loop {
        match pc {
            0x832589B0 => {
    //   block [0x832589B0..0x832589F0)
	// 832589B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832589B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832589B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832589BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832589C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832589C4: 388B722C  addi r4, r11, 0x722c
	ctx.r[4].s64 = ctx.r[11].s64 + 29228;
	// 832589C8: 386A8B08  addi r3, r10, -0x74f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29944;
	// 832589CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832589D0: 4AFD4501  bl 0x8222ced0
	ctx.lr = 0x832589D4;
	sub_8222CED0(ctx, base);
	// 832589D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832589D8: 3869B3D8  addi r3, r9, -0x4c28
	ctx.r[3].s64 = ctx.r[9].s64 + -19496;
	// 832589DC: 4BA51545  bl 0x82ca9f20
	ctx.lr = 0x832589E0;
	sub_82CA9F20(ctx, base);
	// 832589E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832589E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832589E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832589EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832589F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832589F0 size=64
    let mut pc: u32 = 0x832589F0;
    'dispatch: loop {
        match pc {
            0x832589F0 => {
    //   block [0x832589F0..0x83258A30)
	// 832589F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832589F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832589F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832589FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258A04: 388B7240  addi r4, r11, 0x7240
	ctx.r[4].s64 = ctx.r[11].s64 + 29248;
	// 83258A08: 386A8B0C  addi r3, r10, -0x74f4
	ctx.r[3].s64 = ctx.r[10].s64 + -29940;
	// 83258A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258A10: 4AFD44C1  bl 0x8222ced0
	ctx.lr = 0x83258A14;
	sub_8222CED0(ctx, base);
	// 83258A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258A18: 3869B3E8  addi r3, r9, -0x4c18
	ctx.r[3].s64 = ctx.r[9].s64 + -19480;
	// 83258A1C: 4BA51505  bl 0x82ca9f20
	ctx.lr = 0x83258A20;
	sub_82CA9F20(ctx, base);
	// 83258A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258A30 size=64
    let mut pc: u32 = 0x83258A30;
    'dispatch: loop {
        match pc {
            0x83258A30 => {
    //   block [0x83258A30..0x83258A70)
	// 83258A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258A44: 388B3FBC  addi r4, r11, 0x3fbc
	ctx.r[4].s64 = ctx.r[11].s64 + 16316;
	// 83258A48: 386A8B10  addi r3, r10, -0x74f0
	ctx.r[3].s64 = ctx.r[10].s64 + -29936;
	// 83258A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258A50: 4AFD4481  bl 0x8222ced0
	ctx.lr = 0x83258A54;
	sub_8222CED0(ctx, base);
	// 83258A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258A58: 3869B3F8  addi r3, r9, -0x4c08
	ctx.r[3].s64 = ctx.r[9].s64 + -19464;
	// 83258A5C: 4BA514C5  bl 0x82ca9f20
	ctx.lr = 0x83258A60;
	sub_82CA9F20(ctx, base);
	// 83258A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258A70 size=64
    let mut pc: u32 = 0x83258A70;
    'dispatch: loop {
        match pc {
            0x83258A70 => {
    //   block [0x83258A70..0x83258AB0)
	// 83258A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258A7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258A84: 388B7254  addi r4, r11, 0x7254
	ctx.r[4].s64 = ctx.r[11].s64 + 29268;
	// 83258A88: 386A8B14  addi r3, r10, -0x74ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29932;
	// 83258A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258A90: 4AFD4441  bl 0x8222ced0
	ctx.lr = 0x83258A94;
	sub_8222CED0(ctx, base);
	// 83258A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258A98: 3869B408  addi r3, r9, -0x4bf8
	ctx.r[3].s64 = ctx.r[9].s64 + -19448;
	// 83258A9C: 4BA51485  bl 0x82ca9f20
	ctx.lr = 0x83258AA0;
	sub_82CA9F20(ctx, base);
	// 83258AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258AB0 size=64
    let mut pc: u32 = 0x83258AB0;
    'dispatch: loop {
        match pc {
            0x83258AB0 => {
    //   block [0x83258AB0..0x83258AF0)
	// 83258AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258ABC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258AC4: 388B726C  addi r4, r11, 0x726c
	ctx.r[4].s64 = ctx.r[11].s64 + 29292;
	// 83258AC8: 386A8B18  addi r3, r10, -0x74e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29928;
	// 83258ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258AD0: 4AFD4401  bl 0x8222ced0
	ctx.lr = 0x83258AD4;
	sub_8222CED0(ctx, base);
	// 83258AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258AD8: 3869B418  addi r3, r9, -0x4be8
	ctx.r[3].s64 = ctx.r[9].s64 + -19432;
	// 83258ADC: 4BA51445  bl 0x82ca9f20
	ctx.lr = 0x83258AE0;
	sub_82CA9F20(ctx, base);
	// 83258AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258AF0 size=64
    let mut pc: u32 = 0x83258AF0;
    'dispatch: loop {
        match pc {
            0x83258AF0 => {
    //   block [0x83258AF0..0x83258B30)
	// 83258AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258AFC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258B04: 388B7280  addi r4, r11, 0x7280
	ctx.r[4].s64 = ctx.r[11].s64 + 29312;
	// 83258B08: 386A8B1C  addi r3, r10, -0x74e4
	ctx.r[3].s64 = ctx.r[10].s64 + -29924;
	// 83258B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258B10: 4AFD43C1  bl 0x8222ced0
	ctx.lr = 0x83258B14;
	sub_8222CED0(ctx, base);
	// 83258B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258B18: 3869B428  addi r3, r9, -0x4bd8
	ctx.r[3].s64 = ctx.r[9].s64 + -19416;
	// 83258B1C: 4BA51405  bl 0x82ca9f20
	ctx.lr = 0x83258B20;
	sub_82CA9F20(ctx, base);
	// 83258B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258B30 size=64
    let mut pc: u32 = 0x83258B30;
    'dispatch: loop {
        match pc {
            0x83258B30 => {
    //   block [0x83258B30..0x83258B70)
	// 83258B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258B3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258B44: 388B72A0  addi r4, r11, 0x72a0
	ctx.r[4].s64 = ctx.r[11].s64 + 29344;
	// 83258B48: 386A8B20  addi r3, r10, -0x74e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29920;
	// 83258B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258B50: 4AFD4381  bl 0x8222ced0
	ctx.lr = 0x83258B54;
	sub_8222CED0(ctx, base);
	// 83258B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258B58: 3869B438  addi r3, r9, -0x4bc8
	ctx.r[3].s64 = ctx.r[9].s64 + -19400;
	// 83258B5C: 4BA513C5  bl 0x82ca9f20
	ctx.lr = 0x83258B60;
	sub_82CA9F20(ctx, base);
	// 83258B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258B70 size=64
    let mut pc: u32 = 0x83258B70;
    'dispatch: loop {
        match pc {
            0x83258B70 => {
    //   block [0x83258B70..0x83258BB0)
	// 83258B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258B7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258B84: 388B72C0  addi r4, r11, 0x72c0
	ctx.r[4].s64 = ctx.r[11].s64 + 29376;
	// 83258B88: 386A8B24  addi r3, r10, -0x74dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29916;
	// 83258B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258B90: 4AFD4341  bl 0x8222ced0
	ctx.lr = 0x83258B94;
	sub_8222CED0(ctx, base);
	// 83258B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258B98: 3869B448  addi r3, r9, -0x4bb8
	ctx.r[3].s64 = ctx.r[9].s64 + -19384;
	// 83258B9C: 4BA51385  bl 0x82ca9f20
	ctx.lr = 0x83258BA0;
	sub_82CA9F20(ctx, base);
	// 83258BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258BB0 size=216
    let mut pc: u32 = 0x83258BB0;
    'dispatch: loop {
        match pc {
            0x83258BB0 => {
    //   block [0x83258BB0..0x83258C88)
	// 83258BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83258BBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258BC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258BC4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83258BC8: 3BEB8B28  addi r31, r11, -0x74d8
	ctx.r[31].s64 = ctx.r[11].s64 + -29912;
	// 83258BCC: 388A7358  addi r4, r10, 0x7358
	ctx.r[4].s64 = ctx.r[10].s64 + 29528;
	// 83258BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83258BD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258BD8: 4AFD42F9  bl 0x8222ced0
	ctx.lr = 0x83258BDC;
	sub_8222CED0(ctx, base);
	// 83258BDC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258BE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258BE4: 3889734C  addi r4, r9, 0x734c
	ctx.r[4].s64 = ctx.r[9].s64 + 29516;
	// 83258BE8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83258BEC: 4AFD42E5  bl 0x8222ced0
	ctx.lr = 0x83258BF0;
	sub_8222CED0(ctx, base);
	// 83258BF0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258BF8: 38887334  addi r4, r8, 0x7334
	ctx.r[4].s64 = ctx.r[8].s64 + 29492;
	// 83258BFC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83258C00: 4AFD42D1  bl 0x8222ced0
	ctx.lr = 0x83258C04;
	sub_8222CED0(ctx, base);
	// 83258C04: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258C08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C0C: 3887731C  addi r4, r7, 0x731c
	ctx.r[4].s64 = ctx.r[7].s64 + 29468;
	// 83258C10: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83258C14: 4AFD42BD  bl 0x8222ced0
	ctx.lr = 0x83258C18;
	sub_8222CED0(ctx, base);
	// 83258C18: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83258C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C20: 38867310  addi r4, r6, 0x7310
	ctx.r[4].s64 = ctx.r[6].s64 + 29456;
	// 83258C24: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83258C28: 4AFD42A9  bl 0x8222ced0
	ctx.lr = 0x83258C2C;
	sub_8222CED0(ctx, base);
	// 83258C2C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258C30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C34: 38847304  addi r4, r4, 0x7304
	ctx.r[4].s64 = ctx.r[4].s64 + 29444;
	// 83258C38: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83258C3C: 4AFD4295  bl 0x8222ced0
	ctx.lr = 0x83258C40;
	sub_8222CED0(ctx, base);
	// 83258C40: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C48: 388372F4  addi r4, r3, 0x72f4
	ctx.r[4].s64 = ctx.r[3].s64 + 29428;
	// 83258C4C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83258C50: 4AFD4281  bl 0x8222ced0
	ctx.lr = 0x83258C54;
	sub_8222CED0(ctx, base);
	// 83258C54: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258C58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258C5C: 388B72E0  addi r4, r11, 0x72e0
	ctx.r[4].s64 = ctx.r[11].s64 + 29408;
	// 83258C60: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83258C64: 4AFD426D  bl 0x8222ced0
	ctx.lr = 0x83258C68;
	sub_8222CED0(ctx, base);
	// 83258C68: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83258C6C: 386AB458  addi r3, r10, -0x4ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -19368;
	// 83258C70: 4BA512B1  bl 0x82ca9f20
	ctx.lr = 0x83258C74;
	sub_82CA9F20(ctx, base);
	// 83258C74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258C80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258C88 size=12
    let mut pc: u32 = 0x83258C88;
    'dispatch: loop {
        match pc {
            0x83258C88 => {
    //   block [0x83258C88..0x83258C94)
	// 83258C88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258C8C: 386B8B50  addi r3, r11, -0x74b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29872;
	// 83258C90: 4AFE3120  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258C98 size=96
    let mut pc: u32 = 0x83258C98;
    'dispatch: loop {
        match pc {
            0x83258C98 => {
    //   block [0x83258C98..0x83258CBC)
	// 83258C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258CA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83258CA8: 4AFC65B1  bl 0x8221f258
	ctx.lr = 0x83258CAC;
	sub_8221F258(ctx, base);
	// 83258CAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83258CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83258CB4: 419A0008  beq cr6, 0x83258cbc
	if ctx.cr[6].eq {
	pc = 0x83258CBC; continue 'dispatch;
	}
	// 83258CB8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83258CBC; continue 'dispatch;
            }
            0x83258CBC => {
    //   block [0x83258CBC..0x83258CC8)
	// 83258CBC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83258CC0: 41820008  beq 0x83258cc8
	if ctx.cr[0].eq {
	pc = 0x83258CC8; continue 'dispatch;
	}
	// 83258CC4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83258CC8; continue 'dispatch;
            }
            0x83258CC8 => {
    //   block [0x83258CC8..0x83258CF8)
	// 83258CC8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83258CCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83258CD0: 39098B90  addi r8, r9, -0x7470
	ctx.r[8].s64 = ctx.r[9].s64 + -29808;
	// 83258CD4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83258CD8: 3867B4C0  addi r3, r7, -0x4b40
	ctx.r[3].s64 = ctx.r[7].s64 + -19264;
	// 83258CDC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83258CE0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83258CE4: 4BA5123D  bl 0x82ca9f20
	ctx.lr = 0x83258CE8;
	sub_82CA9F20(ctx, base);
	// 83258CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258CF8 size=64
    let mut pc: u32 = 0x83258CF8;
    'dispatch: loop {
        match pc {
            0x83258CF8 => {
    //   block [0x83258CF8..0x83258D38)
	// 83258CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258D04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258D08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258D0C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258D10: 386A8B9C  addi r3, r10, -0x7464
	ctx.r[3].s64 = ctx.r[10].s64 + -29796;
	// 83258D14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258D18: 4AFD41B9  bl 0x8222ced0
	ctx.lr = 0x83258D1C;
	sub_8222CED0(ctx, base);
	// 83258D1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258D20: 3869B508  addi r3, r9, -0x4af8
	ctx.r[3].s64 = ctx.r[9].s64 + -19192;
	// 83258D24: 4BA511FD  bl 0x82ca9f20
	ctx.lr = 0x83258D28;
	sub_82CA9F20(ctx, base);
	// 83258D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258D38 size=64
    let mut pc: u32 = 0x83258D38;
    'dispatch: loop {
        match pc {
            0x83258D38 => {
    //   block [0x83258D38..0x83258D78)
	// 83258D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258D44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258D4C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258D50: 386A8BA0  addi r3, r10, -0x7460
	ctx.r[3].s64 = ctx.r[10].s64 + -29792;
	// 83258D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258D58: 4AFD4179  bl 0x8222ced0
	ctx.lr = 0x83258D5C;
	sub_8222CED0(ctx, base);
	// 83258D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258D60: 3869B518  addi r3, r9, -0x4ae8
	ctx.r[3].s64 = ctx.r[9].s64 + -19176;
	// 83258D64: 4BA511BD  bl 0x82ca9f20
	ctx.lr = 0x83258D68;
	sub_82CA9F20(ctx, base);
	// 83258D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258D78 size=64
    let mut pc: u32 = 0x83258D78;
    'dispatch: loop {
        match pc {
            0x83258D78 => {
    //   block [0x83258D78..0x83258DB8)
	// 83258D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258D8C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258D90: 386A8BA4  addi r3, r10, -0x745c
	ctx.r[3].s64 = ctx.r[10].s64 + -29788;
	// 83258D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258D98: 4AFD4139  bl 0x8222ced0
	ctx.lr = 0x83258D9C;
	sub_8222CED0(ctx, base);
	// 83258D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258DA0: 3869B528  addi r3, r9, -0x4ad8
	ctx.r[3].s64 = ctx.r[9].s64 + -19160;
	// 83258DA4: 4BA5117D  bl 0x82ca9f20
	ctx.lr = 0x83258DA8;
	sub_82CA9F20(ctx, base);
	// 83258DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258DB8 size=64
    let mut pc: u32 = 0x83258DB8;
    'dispatch: loop {
        match pc {
            0x83258DB8 => {
    //   block [0x83258DB8..0x83258DF8)
	// 83258DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258DCC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258DD0: 386A8BA8  addi r3, r10, -0x7458
	ctx.r[3].s64 = ctx.r[10].s64 + -29784;
	// 83258DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258DD8: 4AFD40F9  bl 0x8222ced0
	ctx.lr = 0x83258DDC;
	sub_8222CED0(ctx, base);
	// 83258DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258DE0: 3869B538  addi r3, r9, -0x4ac8
	ctx.r[3].s64 = ctx.r[9].s64 + -19144;
	// 83258DE4: 4BA5113D  bl 0x82ca9f20
	ctx.lr = 0x83258DE8;
	sub_82CA9F20(ctx, base);
	// 83258DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258DF8 size=64
    let mut pc: u32 = 0x83258DF8;
    'dispatch: loop {
        match pc {
            0x83258DF8 => {
    //   block [0x83258DF8..0x83258E38)
	// 83258DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258E0C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258E10: 386A8BAC  addi r3, r10, -0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + -29780;
	// 83258E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258E18: 4AFD40B9  bl 0x8222ced0
	ctx.lr = 0x83258E1C;
	sub_8222CED0(ctx, base);
	// 83258E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258E20: 3869B548  addi r3, r9, -0x4ab8
	ctx.r[3].s64 = ctx.r[9].s64 + -19128;
	// 83258E24: 4BA510FD  bl 0x82ca9f20
	ctx.lr = 0x83258E28;
	sub_82CA9F20(ctx, base);
	// 83258E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258E38 size=64
    let mut pc: u32 = 0x83258E38;
    'dispatch: loop {
        match pc {
            0x83258E38 => {
    //   block [0x83258E38..0x83258E78)
	// 83258E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258E48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258E4C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258E50: 386A8BB0  addi r3, r10, -0x7450
	ctx.r[3].s64 = ctx.r[10].s64 + -29776;
	// 83258E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258E58: 4AFD4079  bl 0x8222ced0
	ctx.lr = 0x83258E5C;
	sub_8222CED0(ctx, base);
	// 83258E5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258E60: 3869B558  addi r3, r9, -0x4aa8
	ctx.r[3].s64 = ctx.r[9].s64 + -19112;
	// 83258E64: 4BA510BD  bl 0x82ca9f20
	ctx.lr = 0x83258E68;
	sub_82CA9F20(ctx, base);
	// 83258E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258E78 size=56
    let mut pc: u32 = 0x83258E78;
    'dispatch: loop {
        match pc {
            0x83258E78 => {
    //   block [0x83258E78..0x83258EB0)
	// 83258E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258E84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83258E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258E8C: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 83258E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258E94: 4AF9AEC5  bl 0x821f3d58
	ctx.lr = 0x83258E98;
	sub_821F3D58(ctx, base);
	// 83258E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258E9C: 906A8BB4  stw r3, -0x744c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29772 as u32), ctx.r[3].u32 ) };
	// 83258EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258EB0 size=164
    let mut pc: u32 = 0x83258EB0;
    'dispatch: loop {
        match pc {
            0x83258EB0 => {
    //   block [0x83258EB0..0x83258F54)
	// 83258EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83258EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83258EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258EC4: 3D60811C  lis r11, -0x7ee4
	ctx.r[11].s64 = -2128871424;
	// 83258EC8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83258ECC: 617F9DC5  ori r31, r11, 0x9dc5
	ctx.r[31].u64 = ctx.r[11].u64 | 40389;
	// 83258ED0: 386A75F8  addi r3, r10, 0x75f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30200;
	// 83258ED4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258ED8: 4AF9AE81  bl 0x821f3d58
	ctx.lr = 0x83258EDC;
	sub_821F3D58(ctx, base);
	// 83258EDC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83258EE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83258EE4: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258EE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258EEC: 386875F0  addi r3, r8, 0x75f0
	ctx.r[3].s64 = ctx.r[8].s64 + 30192;
	// 83258EF0: 91698BB8  stw r11, -0x7448(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-29768 as u32), ctx.r[11].u32 ) };
	// 83258EF4: 3BC98BB8  addi r30, r9, -0x7448
	ctx.r[30].s64 = ctx.r[9].s64 + -29768;
	// 83258EF8: 4AF9AE61  bl 0x821f3d58
	ctx.lr = 0x83258EFC;
	sub_821F3D58(ctx, base);
	// 83258EFC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83258F00: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258F04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258F08: 386775E8  addi r3, r7, 0x75e8
	ctx.r[3].s64 = ctx.r[7].s64 + 30184;
	// 83258F0C: 4AF9AE4D  bl 0x821f3d58
	ctx.lr = 0x83258F10;
	sub_821F3D58(ctx, base);
	// 83258F10: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83258F14: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83258F18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258F1C: 386675E0  addi r3, r6, 0x75e0
	ctx.r[3].s64 = ctx.r[6].s64 + 30176;
	// 83258F20: 4AF9AE39  bl 0x821f3d58
	ctx.lr = 0x83258F24;
	sub_821F3D58(ctx, base);
	// 83258F24: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83258F28: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 83258F2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83258F30: 386575D8  addi r3, r5, 0x75d8
	ctx.r[3].s64 = ctx.r[5].s64 + 30168;
	// 83258F34: 4AF9AE25  bl 0x821f3d58
	ctx.lr = 0x83258F38;
	sub_821F3D58(ctx, base);
	// 83258F38: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83258F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83258F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83258F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258F58 size=64
    let mut pc: u32 = 0x83258F58;
    'dispatch: loop {
        match pc {
            0x83258F58 => {
    //   block [0x83258F58..0x83258F98)
	// 83258F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258F64: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258F68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258F6C: 388B7DF0  addi r4, r11, 0x7df0
	ctx.r[4].s64 = ctx.r[11].s64 + 32240;
	// 83258F70: 386A8BCC  addi r3, r10, -0x7434
	ctx.r[3].s64 = ctx.r[10].s64 + -29748;
	// 83258F74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258F78: 4AFD3F59  bl 0x8222ced0
	ctx.lr = 0x83258F7C;
	sub_8222CED0(ctx, base);
	// 83258F7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258F80: 3869B5D0  addi r3, r9, -0x4a30
	ctx.r[3].s64 = ctx.r[9].s64 + -18992;
	// 83258F84: 4BA50F9D  bl 0x82ca9f20
	ctx.lr = 0x83258F88;
	sub_82CA9F20(ctx, base);
	// 83258F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258F98 size=12
    let mut pc: u32 = 0x83258F98;
    'dispatch: loop {
        match pc {
            0x83258F98 => {
    //   block [0x83258F98..0x83258FA4)
	// 83258F98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83258F9C: 386BB5E0  addi r3, r11, -0x4a20
	ctx.r[3].s64 = ctx.r[11].s64 + -18976;
	// 83258FA0: 4BA50F80  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258FA8 size=64
    let mut pc: u32 = 0x83258FA8;
    'dispatch: loop {
        match pc {
            0x83258FA8 => {
    //   block [0x83258FA8..0x83258FE8)
	// 83258FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258FB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258FBC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258FC0: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 83258FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258FC8: 4AFD3F09  bl 0x8222ced0
	ctx.lr = 0x83258FCC;
	sub_8222CED0(ctx, base);
	// 83258FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258FD0: 3869B638  addi r3, r9, -0x49c8
	ctx.r[3].s64 = ctx.r[9].s64 + -18888;
	// 83258FD4: 4BA50F4D  bl 0x82ca9f20
	ctx.lr = 0x83258FD8;
	sub_82CA9F20(ctx, base);
	// 83258FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258FE8 size=64
    let mut pc: u32 = 0x83258FE8;
    'dispatch: loop {
        match pc {
            0x83258FE8 => {
    //   block [0x83258FE8..0x83259028)
	// 83258FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258FF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258FFC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83259000: 386A8BE8  addi r3, r10, -0x7418
	ctx.r[3].s64 = ctx.r[10].s64 + -29720;
	// 83259004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259008: 4AFD3EC9  bl 0x8222ced0
	ctx.lr = 0x8325900C;
	sub_8222CED0(ctx, base);
	// 8325900C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259010: 3869B648  addi r3, r9, -0x49b8
	ctx.r[3].s64 = ctx.r[9].s64 + -18872;
	// 83259014: 4BA50F0D  bl 0x82ca9f20
	ctx.lr = 0x83259018;
	sub_82CA9F20(ctx, base);
	// 83259018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325901C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259028 size=64
    let mut pc: u32 = 0x83259028;
    'dispatch: loop {
        match pc {
            0x83259028 => {
    //   block [0x83259028..0x83259068)
	// 83259028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325902C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259034: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325903C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83259040: 386A8BEC  addi r3, r10, -0x7414
	ctx.r[3].s64 = ctx.r[10].s64 + -29716;
	// 83259044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259048: 4AFD3E89  bl 0x8222ced0
	ctx.lr = 0x8325904C;
	sub_8222CED0(ctx, base);
	// 8325904C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259050: 3869B658  addi r3, r9, -0x49a8
	ctx.r[3].s64 = ctx.r[9].s64 + -18856;
	// 83259054: 4BA50ECD  bl 0x82ca9f20
	ctx.lr = 0x83259058;
	sub_82CA9F20(ctx, base);
	// 83259058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325905C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259068 size=64
    let mut pc: u32 = 0x83259068;
    'dispatch: loop {
        match pc {
            0x83259068 => {
    //   block [0x83259068..0x832590A8)
	// 83259068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259074: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83259078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325907C: 388B7E70  addi r4, r11, 0x7e70
	ctx.r[4].s64 = ctx.r[11].s64 + 32368;
	// 83259080: 386A8BF0  addi r3, r10, -0x7410
	ctx.r[3].s64 = ctx.r[10].s64 + -29712;
	// 83259084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259088: 4AFD3E49  bl 0x8222ced0
	ctx.lr = 0x8325908C;
	sub_8222CED0(ctx, base);
	// 8325908C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259090: 3869B668  addi r3, r9, -0x4998
	ctx.r[3].s64 = ctx.r[9].s64 + -18840;
	// 83259094: 4BA50E8D  bl 0x82ca9f20
	ctx.lr = 0x83259098;
	sub_82CA9F20(ctx, base);
	// 83259098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325909C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832590A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832590A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832590A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832590A8 size=144
    let mut pc: u32 = 0x832590A8;
    'dispatch: loop {
        match pc {
            0x832590A8 => {
    //   block [0x832590A8..0x832590CC)
	// 832590A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832590AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832590B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832590B4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 832590B8: 4AFC61A1  bl 0x8221f258
	ctx.lr = 0x832590BC;
	sub_8221F258(ctx, base);
	// 832590BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832590C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832590C4: 419A0008  beq cr6, 0x832590cc
	if ctx.cr[6].eq {
	pc = 0x832590CC; continue 'dispatch;
	}
	// 832590C8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832590CC; continue 'dispatch;
            }
            0x832590CC => {
    //   block [0x832590CC..0x832590D8)
	// 832590CC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832590D0: 41820008  beq 0x832590d8
	if ctx.cr[0].eq {
	pc = 0x832590D8; continue 'dispatch;
	}
	// 832590D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832590D8; continue 'dispatch;
            }
            0x832590D8 => {
    //   block [0x832590D8..0x832590E4)
	// 832590D8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832590DC: 41820008  beq 0x832590e4
	if ctx.cr[0].eq {
	pc = 0x832590E4; continue 'dispatch;
	}
	// 832590E0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x832590E4; continue 'dispatch;
            }
            0x832590E4 => {
    //   block [0x832590E4..0x83259138)
	// 832590E4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832590E8: 99430059  stb r10, 0x59(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 832590EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832590F0: 39098BF4  addi r8, r9, -0x740c
	ctx.r[8].s64 = ctx.r[9].s64 + -29708;
	// 832590F4: 99630058  stb r11, 0x58(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 832590F8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832590FC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83259100: 99630059  stb r11, 0x59(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 83259104: 3867B678  addi r3, r7, -0x4988
	ctx.r[3].s64 = ctx.r[7].s64 + -18824;
	// 83259108: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325910C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83259110: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83259114: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83259118: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325911C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83259120: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83259124: 4BA50DFD  bl 0x82ca9f20
	ctx.lr = 0x83259128;
	sub_82CA9F20(ctx, base);
	// 83259128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325912C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259138 size=176
    let mut pc: u32 = 0x83259138;
    'dispatch: loop {
        match pc {
            0x83259138 => {
    //   block [0x83259138..0x8325915C)
	// 83259138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325913C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83259144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259148: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325914C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 83259150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83259154: 394A833C  addi r10, r10, -0x7cc4
	ctx.r[10].s64 = ctx.r[10].s64 + -31940;
	// 83259158: 3BE9A0F4  addi r31, r9, -0x5f0c
	ctx.r[31].s64 = ctx.r[9].s64 + -24332;
	pc = 0x8325915C; continue 'dispatch;
            }
            0x8325915C => {
    //   block [0x8325915C..0x83259190)
	// 8325915C: 7D2B50AE  lbzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83259160: 391F000C  addi r8, r31, 0xc
	ctx.r[8].s64 = ctx.r[31].s64 + 12;
	// 83259164: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83259168: 7D2B41AE  stbx r9, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u8) };
	// 8325916C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83259170: 409AFFEC  bne cr6, 0x8325915c
	if !ctx.cr[6].eq {
	pc = 0x8325915C; continue 'dispatch;
	}
	// 83259174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83259178: 4B3BD6A9  bl 0x82616820
	ctx.lr = 0x8325917C;
	sub_82616820(ctx, base);
	// 8325917C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 83259180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83259184: 409A000C  bne cr6, 0x83259190
	if !ctx.cr[6].eq {
	pc = 0x83259190; continue 'dispatch;
	}
	// 83259188: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8325918C: 48000010  b 0x8325919c
	pc = 0x8325919C; continue 'dispatch;
            }
            0x83259190 => {
    //   block [0x83259190..0x8325919C)
	// 83259190: 4B3BD5F9  bl 0x82616788
	ctx.lr = 0x83259194;
	sub_82616788(ctx, base);
	// 83259194: 81630400  lwz r11, 0x400(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1024 as u32) ) } as u64;
	// 83259198: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x8325919C; continue 'dispatch;
            }
            0x8325919C => {
    //   block [0x8325919C..0x832591E8)
	// 8325919C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832591A0: 3D608261  lis r11, -0x7d9f
	ctx.r[11].s64 = -2107572224;
	// 832591A4: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832591A8: 396B4F18  addi r11, r11, 0x4f18
	ctx.r[11].s64 = ctx.r[11].s64 + 20248;
	// 832591AC: 392A2B90  addi r9, r10, 0x2b90
	ctx.r[9].s64 = ctx.r[10].s64 + 11152;
	// 832591B0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832591B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832591B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832591BC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832591C0: 915F010C  stw r10, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[10].u32 ) };
	// 832591C4: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832591C8: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 832591CC: 386AB688  addi r3, r10, -0x4978
	ctx.r[3].s64 = ctx.r[10].s64 + -18808;
	// 832591D0: 4BA50D51  bl 0x82ca9f20
	ctx.lr = 0x832591D4;
	sub_82CA9F20(ctx, base);
	// 832591D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832591D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832591DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832591E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832591E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832591E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832591E8 size=76
    let mut pc: u32 = 0x832591E8;
    'dispatch: loop {
        match pc {
            0x832591E8 => {
    //   block [0x832591E8..0x83259234)
	// 832591E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832591EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832591F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832591F4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832591F8: 3D408261  lis r10, -0x7d9f
	ctx.r[10].s64 = -2107572224;
	// 832591FC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259200: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259204: 38CBB6AC  addi r6, r11, -0x4954
	ctx.r[6].s64 = ctx.r[11].s64 + -18772;
	// 83259208: 388983EC  addi r4, r9, -0x7c14
	ctx.r[4].s64 = ctx.r[9].s64 + -31764;
	// 8325920C: 38688C00  addi r3, r8, -0x7400
	ctx.r[3].s64 = ctx.r[8].s64 + -29696;
	// 83259210: 38AA69A8  addi r5, r10, 0x69a8
	ctx.r[5].s64 = ctx.r[10].s64 + 27048;
	// 83259214: 4BC3355D  bl 0x82e8c770
	ctx.lr = 0x83259218;
	sub_82E8C770(ctx, base);
	// 83259218: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325921C: 3867B6B8  addi r3, r7, -0x4948
	ctx.r[3].s64 = ctx.r[7].s64 + -18760;
	// 83259220: 4BA50D01  bl 0x82ca9f20
	ctx.lr = 0x83259224;
	sub_82CA9F20(ctx, base);
	// 83259224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325922C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83259238 size=108
    let mut pc: u32 = 0x83259238;
    'dispatch: loop {
        match pc {
            0x83259238 => {
    //   block [0x83259238..0x832592A4)
	// 83259238: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325923C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83259240: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83259244: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83259248: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 8325924C: 3CE0820A  lis r7, -0x7df6
	ctx.r[7].s64 = -2113273856;
	// 83259250: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83259254: 38C89140  addi r6, r8, -0x6ec0
	ctx.r[6].s64 = ctx.r[8].s64 + -28352;
	// 83259258: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8325925C: 38A79150  addi r5, r7, -0x6eb0
	ctx.r[5].s64 = ctx.r[7].s64 + -28336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832592A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832592A8 size=52
    let mut pc: u32 = 0x832592A8;
    'dispatch: loop {
        match pc {
            0x832592A8 => {
    //   block [0x832592A8..0x832592DC)
	// 832592A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832592AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832592B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832592B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832592B8: 386B8D14  addi r3, r11, -0x72ec
	ctx.r[3].s64 = ctx.r[11].s64 + -29420;
	// 832592BC: 480609C9  bl 0x832b9c84
	ctx.lr = 0x832592C0;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832592C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832592C4: 386AB6D0  addi r3, r10, -0x4930
	ctx.r[3].s64 = ctx.r[10].s64 + -18736;
	// 832592C8: 4BA50C59  bl 0x82ca9f20
	ctx.lr = 0x832592CC;
	sub_82CA9F20(ctx, base);
	// 832592CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832592D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832592D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832592D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832592E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832592E0 size=52
    let mut pc: u32 = 0x832592E0;
    'dispatch: loop {
        match pc {
            0x832592E0 => {
    //   block [0x832592E0..0x83259314)
	// 832592E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832592E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832592E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832592EC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832592F0: 386B8D30  addi r3, r11, -0x72d0
	ctx.r[3].s64 = ctx.r[11].s64 + -29392;
	// 832592F4: 48060991  bl 0x832b9c84
	ctx.lr = 0x832592F8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832592F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832592FC: 386AB6D8  addi r3, r10, -0x4928
	ctx.r[3].s64 = ctx.r[10].s64 + -18728;
	// 83259300: 4BA50C21  bl 0x82ca9f20
	ctx.lr = 0x83259304;
	sub_82CA9F20(ctx, base);
	// 83259304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325930C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259318 size=84
    let mut pc: u32 = 0x83259318;
    'dispatch: loop {
        match pc {
            0x83259318 => {
    //   block [0x83259318..0x8325936C)
	// 83259318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325931C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259324: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259328: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8325932C: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 83259330: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259334: 38688D4C  addi r3, r8, -0x72b4
	ctx.r[3].s64 = ctx.r[8].s64 + -29364;
	// 83259338: 38CB8C64  addi r6, r11, -0x739c
	ctx.r[6].s64 = ctx.r[11].s64 + -29596;
	// 8325933C: 388984D4  addi r4, r9, -0x7b2c
	ctx.r[4].s64 = ctx.r[9].s64 + -31532;
	// 83259340: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83259344: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83259348: 38AA8EA0  addi r5, r10, -0x7160
	ctx.r[5].s64 = ctx.r[10].s64 + -29024;
	// 8325934C: 4BC2C435  bl 0x82e85780
	ctx.lr = 0x83259350;
	sub_82E85780(ctx, base);
	// 83259350: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259354: 3867B6E0  addi r3, r7, -0x4920
	ctx.r[3].s64 = ctx.r[7].s64 + -18720;
	// 83259358: 4BA50BC9  bl 0x82ca9f20
	ctx.lr = 0x8325935C;
	sub_82CA9F20(ctx, base);
	// 8325935C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259370 size=80
    let mut pc: u32 = 0x83259370;
    'dispatch: loop {
        match pc {
            0x83259370 => {
    //   block [0x83259370..0x832593C0)
	// 83259370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325937C: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259380: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259384: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259388: 388A84E8  addi r4, r10, -0x7b18
	ctx.r[4].s64 = ctx.r[10].s64 + -31512;
	// 8325938C: 38698E60  addi r3, r9, -0x71a0
	ctx.r[3].s64 = ctx.r[9].s64 + -29088;
	// 83259390: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83259394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83259398: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8325939C: 38AB9218  addi r5, r11, -0x6de8
	ctx.r[5].s64 = ctx.r[11].s64 + -28136;
	// 832593A0: 4BC2C421  bl 0x82e857c0
	ctx.lr = 0x832593A4;
	sub_82E857C0(ctx, base);
	// 832593A4: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832593A8: 3868B6F8  addi r3, r8, -0x4908
	ctx.r[3].s64 = ctx.r[8].s64 + -18696;
	// 832593AC: 4BA50B75  bl 0x82ca9f20
	ctx.lr = 0x832593B0;
	sub_82CA9F20(ctx, base);
	// 832593B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832593B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832593B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832593BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832593C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832593C0 size=72
    let mut pc: u32 = 0x832593C0;
    'dispatch: loop {
        match pc {
            0x832593C0 => {
    //   block [0x832593C0..0x83259408)
	// 832593C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832593C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832593C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832593CC: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832593D0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832593D4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832593D8: 388A8528  addi r4, r10, -0x7ad8
	ctx.r[4].s64 = ctx.r[10].s64 + -31448;
	// 832593DC: 38698F74  addi r3, r9, -0x708c
	ctx.r[3].s64 = ctx.r[9].s64 + -28812;
	// 832593E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832593E4: 38AB96B8  addi r5, r11, -0x6948
	ctx.r[5].s64 = ctx.r[11].s64 + -26952;
	// 832593E8: 4BC26179  bl 0x82e7f560
	ctx.lr = 0x832593EC;
	sub_82E7F560(ctx, base);
	// 832593EC: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832593F0: 3868B710  addi r3, r8, -0x48f0
	ctx.r[3].s64 = ctx.r[8].s64 + -18672;
	// 832593F4: 4BA50B2D  bl 0x82ca9f20
	ctx.lr = 0x832593F8;
	sub_82CA9F20(ctx, base);
	// 832593F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832593FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259408 size=72
    let mut pc: u32 = 0x83259408;
    'dispatch: loop {
        match pc {
            0x83259408 => {
    //   block [0x83259408..0x83259450)
	// 83259408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325940C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259414: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259418: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325941C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259420: 388A8538  addi r4, r10, -0x7ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -31432;
	// 83259424: 38699088  addi r3, r9, -0x6f78
	ctx.r[3].s64 = ctx.r[9].s64 + -28536;
	// 83259428: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8325942C: 38AB9710  addi r5, r11, -0x68f0
	ctx.r[5].s64 = ctx.r[11].s64 + -26864;
	// 83259430: 4BC26131  bl 0x82e7f560
	ctx.lr = 0x83259434;
	sub_82E7F560(ctx, base);
	// 83259434: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259438: 3868B728  addi r3, r8, -0x48d8
	ctx.r[3].s64 = ctx.r[8].s64 + -18648;
	// 8325943C: 4BA50AE5  bl 0x82ca9f20
	ctx.lr = 0x83259440;
	sub_82CA9F20(ctx, base);
	// 83259440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325944C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259450 size=72
    let mut pc: u32 = 0x83259450;
    'dispatch: loop {
        match pc {
            0x83259450 => {
    //   block [0x83259450..0x83259498)
	// 83259450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325945C: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259460: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259464: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259468: 388A8590  addi r4, r10, -0x7a70
	ctx.r[4].s64 = ctx.r[10].s64 + -31344;
	// 8325946C: 3869919C  addi r3, r9, -0x6e64
	ctx.r[3].s64 = ctx.r[9].s64 + -28260;
	// 83259470: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83259474: 38AB9CC0  addi r5, r11, -0x6340
	ctx.r[5].s64 = ctx.r[11].s64 + -25408;
	// 83259478: 4BC260E9  bl 0x82e7f560
	ctx.lr = 0x8325947C;
	sub_82E7F560(ctx, base);
	// 8325947C: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259480: 3868B740  addi r3, r8, -0x48c0
	ctx.r[3].s64 = ctx.r[8].s64 + -18624;
	// 83259484: 4BA50A9D  bl 0x82ca9f20
	ctx.lr = 0x83259488;
	sub_82CA9F20(ctx, base);
	// 83259488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325948C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259498 size=96
    let mut pc: u32 = 0x83259498;
    'dispatch: loop {
        match pc {
            0x83259498 => {
    //   block [0x83259498..0x832594BC)
	// 83259498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832594A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832594A4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 832594A8: 4AFC5DB1  bl 0x8221f258
	ctx.lr = 0x832594AC;
	sub_8221F258(ctx, base);
	// 832594AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832594B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832594B4: 419A0008  beq cr6, 0x832594bc
	if ctx.cr[6].eq {
	pc = 0x832594BC; continue 'dispatch;
	}
	// 832594B8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832594BC; continue 'dispatch;
            }
            0x832594BC => {
    //   block [0x832594BC..0x832594C8)
	// 832594BC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832594C0: 41820008  beq 0x832594c8
	if ctx.cr[0].eq {
	pc = 0x832594C8; continue 'dispatch;
	}
	// 832594C4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832594C8; continue 'dispatch;
            }
            0x832594C8 => {
    //   block [0x832594C8..0x832594F8)
	// 832594C8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832594CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832594D0: 390992B0  addi r8, r9, -0x6d50
	ctx.r[8].s64 = ctx.r[9].s64 + -27984;
	// 832594D4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832594D8: 3867B758  addi r3, r7, -0x48a8
	ctx.r[3].s64 = ctx.r[7].s64 + -18600;
	// 832594DC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832594E0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832594E4: 4BA50A3D  bl 0x82ca9f20
	ctx.lr = 0x832594E8;
	sub_82CA9F20(ctx, base);
	// 832594E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832594EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832594F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832594F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832594F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832594F8 size=72
    let mut pc: u32 = 0x832594F8;
    'dispatch: loop {
        match pc {
            0x832594F8 => {
    //   block [0x832594F8..0x83259540)
	// 832594F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832594FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259504: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259508: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325950C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259510: 388A8CC8  addi r4, r10, -0x7338
	ctx.r[4].s64 = ctx.r[10].s64 + -29496;
	// 83259514: 386992BC  addi r3, r9, -0x6d44
	ctx.r[3].s64 = ctx.r[9].s64 + -27972;
	// 83259518: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8325951C: 38ABE058  addi r5, r11, -0x1fa8
	ctx.r[5].s64 = ctx.r[11].s64 + -8104;
	// 83259520: 4BC26041  bl 0x82e7f560
	ctx.lr = 0x83259524;
	sub_82E7F560(ctx, base);
	// 83259524: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259528: 3868B7A0  addi r3, r8, -0x4860
	ctx.r[3].s64 = ctx.r[8].s64 + -18528;
	// 8325952C: 4BA509F5  bl 0x82ca9f20
	ctx.lr = 0x83259530;
	sub_82CA9F20(ctx, base);
	// 83259530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325953C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259540 size=72
    let mut pc: u32 = 0x83259540;
    'dispatch: loop {
        match pc {
            0x83259540 => {
    //   block [0x83259540..0x83259588)
	// 83259540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325954C: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259550: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259554: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259558: 388A8CDC  addi r4, r10, -0x7324
	ctx.r[4].s64 = ctx.r[10].s64 + -29476;
	// 8325955C: 386993D0  addi r3, r9, -0x6c30
	ctx.r[3].s64 = ctx.r[9].s64 + -27696;
	// 83259560: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83259564: 38ABE1A8  addi r5, r11, -0x1e58
	ctx.r[5].s64 = ctx.r[11].s64 + -7768;
	// 83259568: 4BC24759  bl 0x82e7dcc0
	ctx.lr = 0x8325956C;
	sub_82E7DCC0(ctx, base);
	// 8325956C: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259570: 3868B7B8  addi r3, r8, -0x4848
	ctx.r[3].s64 = ctx.r[8].s64 + -18504;
	// 83259574: 4BA509AD  bl 0x82ca9f20
	ctx.lr = 0x83259578;
	sub_82CA9F20(ctx, base);
	// 83259578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325957C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259588 size=80
    let mut pc: u32 = 0x83259588;
    'dispatch: loop {
        match pc {
            0x83259588 => {
    //   block [0x83259588..0x832595D8)
	// 83259588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325958C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259594: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259598: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325959C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832595A0: 388A8CE8  addi r4, r10, -0x7318
	ctx.r[4].s64 = ctx.r[10].s64 + -29464;
	// 832595A4: 386994E4  addi r3, r9, -0x6b1c
	ctx.r[3].s64 = ctx.r[9].s64 + -27420;
	// 832595A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832595AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832595B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832595B4: 38ABE3A0  addi r5, r11, -0x1c60
	ctx.r[5].s64 = ctx.r[11].s64 + -7264;
	// 832595B8: 4BC25369  bl 0x82e7e920
	ctx.lr = 0x832595BC;
	sub_82E7E920(ctx, base);
	// 832595BC: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832595C0: 3868B7D0  addi r3, r8, -0x4830
	ctx.r[3].s64 = ctx.r[8].s64 + -18480;
	// 832595C4: 4BA5095D  bl 0x82ca9f20
	ctx.lr = 0x832595C8;
	sub_82CA9F20(ctx, base);
	// 832595C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832595CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832595D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832595D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832595D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832595D8 size=76
    let mut pc: u32 = 0x832595D8;
    'dispatch: loop {
        match pc {
            0x832595D8 => {
    //   block [0x832595D8..0x83259624)
	// 832595D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832595DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832595E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832595E4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832595E8: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 832595EC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832595F0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832595F4: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 832595F8: 38898D28  addi r4, r9, -0x72d8
	ctx.r[4].s64 = ctx.r[9].s64 + -29400;
	// 832595FC: 386895F8  addi r3, r8, -0x6a08
	ctx.r[3].s64 = ctx.r[8].s64 + -27144;
	// 83259600: 38AAE8F0  addi r5, r10, -0x1710
	ctx.r[5].s64 = ctx.r[10].s64 + -5904;
	// 83259604: 4BC25F5D  bl 0x82e7f560
	ctx.lr = 0x83259608;
	sub_82E7F560(ctx, base);
	// 83259608: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325960C: 3867B7E8  addi r3, r7, -0x4818
	ctx.r[3].s64 = ctx.r[7].s64 + -18456;
	// 83259610: 4BA50911  bl 0x82ca9f20
	ctx.lr = 0x83259614;
	sub_82CA9F20(ctx, base);
	// 83259614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325961C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259628 size=76
    let mut pc: u32 = 0x83259628;
    'dispatch: loop {
        match pc {
            0x83259628 => {
    //   block [0x83259628..0x83259674)
	// 83259628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259634: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259638: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 8325963C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259640: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259644: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 83259648: 38898D44  addi r4, r9, -0x72bc
	ctx.r[4].s64 = ctx.r[9].s64 + -29372;
	// 8325964C: 3868970C  addi r3, r8, -0x68f4
	ctx.r[3].s64 = ctx.r[8].s64 + -26868;
	// 83259650: 38AAFF08  addi r5, r10, -0xf8
	ctx.r[5].s64 = ctx.r[10].s64 + -248;
	// 83259654: 4BC25F0D  bl 0x82e7f560
	ctx.lr = 0x83259658;
	sub_82E7F560(ctx, base);
	// 83259658: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325965C: 3867B800  addi r3, r7, -0x4800
	ctx.r[3].s64 = ctx.r[7].s64 + -18432;
	// 83259660: 4BA508C1  bl 0x82ca9f20
	ctx.lr = 0x83259664;
	sub_82CA9F20(ctx, base);
	// 83259664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325966C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259678 size=56
    let mut pc: u32 = 0x83259678;
    'dispatch: loop {
        match pc {
            0x83259678 => {
    //   block [0x83259678..0x832596B0)
	// 83259678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325967C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259684: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83259688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325968C: 386B4034  addi r3, r11, 0x4034
	ctx.r[3].s64 = ctx.r[11].s64 + 16436;
	// 83259690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83259694: 4AF9A6C5  bl 0x821f3d58
	ctx.lr = 0x83259698;
	sub_821F3D58(ctx, base);
	// 83259698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325969C: 906A9820  stw r3, -0x67e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26592 as u32), ctx.r[3].u32 ) };
	// 832596A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832596A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832596A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832596AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832596B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832596B0 size=76
    let mut pc: u32 = 0x832596B0;
    'dispatch: loop {
        match pc {
            0x832596B0 => {
    //   block [0x832596B0..0x832596FC)
	// 832596B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832596B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832596B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832596BC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832596C0: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 832596C4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832596C8: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832596CC: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 832596D0: 38898DF4  addi r4, r9, -0x720c
	ctx.r[4].s64 = ctx.r[9].s64 + -29196;
	// 832596D4: 38689824  addi r3, r8, -0x67dc
	ctx.r[3].s64 = ctx.r[8].s64 + -26588;
	// 832596D8: 38AA2E88  addi r5, r10, 0x2e88
	ctx.r[5].s64 = ctx.r[10].s64 + 11912;
	// 832596DC: 4BC25E85  bl 0x82e7f560
	ctx.lr = 0x832596E0;
	sub_82E7F560(ctx, base);
	// 832596E0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832596E4: 3867B818  addi r3, r7, -0x47e8
	ctx.r[3].s64 = ctx.r[7].s64 + -18408;
	// 832596E8: 4BA50839  bl 0x82ca9f20
	ctx.lr = 0x832596EC;
	sub_82CA9F20(ctx, base);
	// 832596EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832596F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832596F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832596F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259700 size=76
    let mut pc: u32 = 0x83259700;
    'dispatch: loop {
        match pc {
            0x83259700 => {
    //   block [0x83259700..0x8325974C)
	// 83259700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325970C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259710: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 83259714: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259718: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8325971C: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 83259720: 38898E6C  addi r4, r9, -0x7194
	ctx.r[4].s64 = ctx.r[9].s64 + -29076;
	// 83259724: 38689938  addi r3, r8, -0x66c8
	ctx.r[3].s64 = ctx.r[8].s64 + -26312;
	// 83259728: 38AA33C0  addi r5, r10, 0x33c0
	ctx.r[5].s64 = ctx.r[10].s64 + 13248;
	// 8325972C: 4BC25E35  bl 0x82e7f560
	ctx.lr = 0x83259730;
	sub_82E7F560(ctx, base);
	// 83259730: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259734: 3867B830  addi r3, r7, -0x47d0
	ctx.r[3].s64 = ctx.r[7].s64 + -18384;
	// 83259738: 4BA507E9  bl 0x82ca9f20
	ctx.lr = 0x8325973C;
	sub_82CA9F20(ctx, base);
	// 8325973C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259750 size=76
    let mut pc: u32 = 0x83259750;
    'dispatch: loop {
        match pc {
            0x83259750 => {
    //   block [0x83259750..0x8325979C)
	// 83259750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325975C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259760: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 83259764: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259768: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8325976C: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 83259770: 38898EEC  addi r4, r9, -0x7114
	ctx.r[4].s64 = ctx.r[9].s64 + -28948;
	// 83259774: 38689A4C  addi r3, r8, -0x65b4
	ctx.r[3].s64 = ctx.r[8].s64 + -26036;
	// 83259778: 38AA3DC0  addi r5, r10, 0x3dc0
	ctx.r[5].s64 = ctx.r[10].s64 + 15808;
	// 8325977C: 4BC25DE5  bl 0x82e7f560
	ctx.lr = 0x83259780;
	sub_82E7F560(ctx, base);
	// 83259780: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259784: 3867B848  addi r3, r7, -0x47b8
	ctx.r[3].s64 = ctx.r[7].s64 + -18360;
	// 83259788: 4BA50799  bl 0x82ca9f20
	ctx.lr = 0x8325978C;
	sub_82CA9F20(ctx, base);
	// 8325978C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832597A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832597A0 size=76
    let mut pc: u32 = 0x832597A0;
    'dispatch: loop {
        match pc {
            0x832597A0 => {
    //   block [0x832597A0..0x832597EC)
	// 832597A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832597A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832597A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832597AC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832597B0: 3D408262  lis r10, -0x7d9e
	ctx.r[10].s64 = -2107506688;
	// 832597B4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 832597B8: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832597BC: 38CB5C6C  addi r6, r11, 0x5c6c
	ctx.r[6].s64 = ctx.r[11].s64 + 23660;
	// 832597C0: 38898F64  addi r4, r9, -0x709c
	ctx.r[4].s64 = ctx.r[9].s64 + -28828;
	// 832597C4: 38689B60  addi r3, r8, -0x64a0
	ctx.r[3].s64 = ctx.r[8].s64 + -25760;
	// 832597C8: 38AA4948  addi r5, r10, 0x4948
	ctx.r[5].s64 = ctx.r[10].s64 + 18760;
	// 832597CC: 4BC25D95  bl 0x82e7f560
	ctx.lr = 0x832597D0;
	sub_82E7F560(ctx, base);
	// 832597D0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 832597D4: 3867B860  addi r3, r7, -0x47a0
	ctx.r[3].s64 = ctx.r[7].s64 + -18336;
	// 832597D8: 4BA50749  bl 0x82ca9f20
	ctx.lr = 0x832597DC;
	sub_82CA9F20(ctx, base);
	// 832597DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832597E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832597E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832597E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832597F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832597F0 size=24
    let mut pc: u32 = 0x832597F0;
    'dispatch: loop {
        match pc {
            0x832597F0 => {
    //   block [0x832597F0..0x83259808)
	// 832597F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832597F4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832597F8: C00B9038  lfs f0, -0x6fc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28616 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832597FC: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 83259800: D00A9C74  stfs f0, -0x638c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25484 as u32), tmp.u32 ) };
	// 83259804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259808 size=144
    let mut pc: u32 = 0x83259808;
    'dispatch: loop {
        match pc {
            0x83259808 => {
    //   block [0x83259808..0x8325982C)
	// 83259808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325980C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259814: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 83259818: 4AFC5A41  bl 0x8221f258
	ctx.lr = 0x8325981C;
	sub_8221F258(ctx, base);
	// 8325981C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83259820: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83259824: 419A0008  beq cr6, 0x8325982c
	if ctx.cr[6].eq {
	pc = 0x8325982C; continue 'dispatch;
	}
	// 83259828: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325982C; continue 'dispatch;
            }
            0x8325982C => {
    //   block [0x8325982C..0x83259838)
	// 8325982C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83259830: 41820008  beq 0x83259838
	if ctx.cr[0].eq {
	pc = 0x83259838; continue 'dispatch;
	}
	// 83259834: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83259838; continue 'dispatch;
            }
            0x83259838 => {
    //   block [0x83259838..0x83259844)
	// 83259838: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325983C: 41820008  beq 0x83259844
	if ctx.cr[0].eq {
	pc = 0x83259844; continue 'dispatch;
	}
	// 83259840: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83259844; continue 'dispatch;
            }
            0x83259844 => {
    //   block [0x83259844..0x83259898)
	// 83259844: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259848: 99430061  stb r10, 0x61(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(97 as u32), ctx.r[10].u8 ) };
	// 8325984C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83259850: 39099C78  addi r8, r9, -0x6388
	ctx.r[8].s64 = ctx.r[9].s64 + -25480;
	// 83259854: 99630060  stb r11, 0x60(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 83259858: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325985C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83259860: 99630061  stb r11, 0x61(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 83259864: 3867B878  addi r3, r7, -0x4788
	ctx.r[3].s64 = ctx.r[7].s64 + -18312;
	// 83259868: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325986C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83259870: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83259874: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83259878: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325987C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83259880: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83259884: 4BA5069D  bl 0x82ca9f20
	ctx.lr = 0x83259888;
	sub_82CA9F20(ctx, base);
	// 83259888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325988C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259898 size=72
    let mut pc: u32 = 0x83259898;
    'dispatch: loop {
        match pc {
            0x83259898 => {
    //   block [0x83259898..0x832598E0)
	// 83259898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325989C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832598A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832598A4: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832598A8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832598AC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832598B0: 388A9000  addi r4, r10, -0x7000
	ctx.r[4].s64 = ctx.r[10].s64 + -28672;
	// 832598B4: 38699C84  addi r3, r9, -0x637c
	ctx.r[3].s64 = ctx.r[9].s64 + -25468;
	// 832598B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832598BC: 38AB67E0  addi r5, r11, 0x67e0
	ctx.r[5].s64 = ctx.r[11].s64 + 26592;
	// 832598C0: 4BC2CA11  bl 0x82e862d0
	ctx.lr = 0x832598C4;
	sub_82E862D0(ctx, base);
	// 832598C4: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832598C8: 3868B898  addi r3, r8, -0x4768
	ctx.r[3].s64 = ctx.r[8].s64 + -18280;
	// 832598CC: 4BA50655  bl 0x82ca9f20
	ctx.lr = 0x832598D0;
	sub_82CA9F20(ctx, base);
	// 832598D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832598D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832598D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832598DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832598E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832598E0 size=72
    let mut pc: u32 = 0x832598E0;
    'dispatch: loop {
        match pc {
            0x832598E0 => {
    //   block [0x832598E0..0x83259928)
	// 832598E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832598E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832598E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832598EC: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832598F0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832598F4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832598F8: 388A9020  addi r4, r10, -0x6fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -28640;
	// 832598FC: 38699D98  addi r3, r9, -0x6268
	ctx.r[3].s64 = ctx.r[9].s64 + -25192;
	// 83259900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83259904: 38AB6A90  addi r5, r11, 0x6a90
	ctx.r[5].s64 = ctx.r[11].s64 + 27280;
	// 83259908: 4BC2C9C9  bl 0x82e862d0
	ctx.lr = 0x8325990C;
	sub_82E862D0(ctx, base);
	// 8325990C: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259910: 3868B8B0  addi r3, r8, -0x4750
	ctx.r[3].s64 = ctx.r[8].s64 + -18256;
	// 83259914: 4BA5060D  bl 0x82ca9f20
	ctx.lr = 0x83259918;
	sub_82CA9F20(ctx, base);
	// 83259918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325991C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259928 size=96
    let mut pc: u32 = 0x83259928;
    'dispatch: loop {
        match pc {
            0x83259928 => {
    //   block [0x83259928..0x8325994C)
	// 83259928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325992C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259934: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83259938: 4AFC5921  bl 0x8221f258
	ctx.lr = 0x8325993C;
	sub_8221F258(ctx, base);
	// 8325993C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83259940: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83259944: 419A0008  beq cr6, 0x8325994c
	if ctx.cr[6].eq {
	pc = 0x8325994C; continue 'dispatch;
	}
	// 83259948: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325994C; continue 'dispatch;
            }
            0x8325994C => {
    //   block [0x8325994C..0x83259958)
	// 8325994C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83259950: 41820008  beq 0x83259958
	if ctx.cr[0].eq {
	pc = 0x83259958; continue 'dispatch;
	}
	// 83259954: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83259958; continue 'dispatch;
            }
            0x83259958 => {
    //   block [0x83259958..0x83259988)
	// 83259958: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325995C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83259960: 39099EAC  addi r8, r9, -0x6154
	ctx.r[8].s64 = ctx.r[9].s64 + -24916;
	// 83259964: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259968: 3867B8C8  addi r3, r7, -0x4738
	ctx.r[3].s64 = ctx.r[7].s64 + -18232;
	// 8325996C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83259970: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83259974: 4BA505AD  bl 0x82ca9f20
	ctx.lr = 0x83259978;
	sub_82CA9F20(ctx, base);
	// 83259978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325997C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259988 size=68
    let mut pc: u32 = 0x83259988;
    'dispatch: loop {
        match pc {
            0x83259988 => {
    //   block [0x83259988..0x832599CC)
	// 83259988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259994: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 83259998: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325999C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832599A0: 388A9098  addi r4, r10, -0x6f68
	ctx.r[4].s64 = ctx.r[10].s64 + -28520;
	// 832599A4: 38699EB8  addi r3, r9, -0x6148
	ctx.r[3].s64 = ctx.r[9].s64 + -24904;
	// 832599A8: 38AB70F8  addi r5, r11, 0x70f8
	ctx.r[5].s64 = ctx.r[11].s64 + 28920;
	// 832599AC: 4B3D45C5  bl 0x8262df70
	ctx.lr = 0x832599B0;
	sub_8262DF70(ctx, base);
	// 832599B0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832599B4: 3868B910  addi r3, r8, -0x46f0
	ctx.r[3].s64 = ctx.r[8].s64 + -18160;
	// 832599B8: 4BA50569  bl 0x82ca9f20
	ctx.lr = 0x832599BC;
	sub_82CA9F20(ctx, base);
	// 832599BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832599C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832599C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832599C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832599D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832599D0 size=68
    let mut pc: u32 = 0x832599D0;
    'dispatch: loop {
        match pc {
            0x832599D0 => {
    //   block [0x832599D0..0x83259A14)
	// 832599D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832599D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832599D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832599DC: 3D608262  lis r11, -0x7d9e
	ctx.r[11].s64 = -2107506688;
	// 832599E0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 832599E4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832599E8: 388A90A0  addi r4, r10, -0x6f60
	ctx.r[4].s64 = ctx.r[10].s64 + -28512;
	// 832599EC: 38699FC4  addi r3, r9, -0x603c
	ctx.r[3].s64 = ctx.r[9].s64 + -24636;
	// 832599F0: 38AB72D8  addi r5, r11, 0x72d8
	ctx.r[5].s64 = ctx.r[11].s64 + 29400;
	// 832599F4: 4B3D479D  bl 0x8262e190
	ctx.lr = 0x832599F8;
	sub_8262E190(ctx, base);
	// 832599F8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 832599FC: 3868B928  addi r3, r8, -0x46d8
	ctx.r[3].s64 = ctx.r[8].s64 + -18136;
	// 83259A00: 4BA50521  bl 0x82ca9f20
	ctx.lr = 0x83259A04;
	sub_82CA9F20(ctx, base);
	// 83259A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259A18 size=76
    let mut pc: u32 = 0x83259A18;
    'dispatch: loop {
        match pc {
            0x83259A18 => {
    //   block [0x83259A18..0x83259A64)
	// 83259A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259A24: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83259A28: 3D408263  lis r10, -0x7d9d
	ctx.r[10].s64 = -2107441152;
	// 83259A2C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259A30: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259A34: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 83259A38: 388990B0  addi r4, r9, -0x6f50
	ctx.r[4].s64 = ctx.r[9].s64 + -28496;
	// 83259A3C: 3868A0D0  addi r3, r8, -0x5f30
	ctx.r[3].s64 = ctx.r[8].s64 + -24368;
	// 83259A40: 38AAB0A0  addi r5, r10, -0x4f60
	ctx.r[5].s64 = ctx.r[10].s64 + -20320;
	// 83259A44: 4BC2C1B5  bl 0x82e85bf8
	ctx.lr = 0x83259A48;
	sub_82E85BF8(ctx, base);
	// 83259A48: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259A4C: 3867B940  addi r3, r7, -0x46c0
	ctx.r[3].s64 = ctx.r[7].s64 + -18112;
	// 83259A50: 4BA504D1  bl 0x82ca9f20
	ctx.lr = 0x83259A54;
	sub_82CA9F20(ctx, base);
	// 83259A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259A68 size=68
    let mut pc: u32 = 0x83259A68;
    'dispatch: loop {
        match pc {
            0x83259A68 => {
    //   block [0x83259A68..0x83259AAC)
	// 83259A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259A74: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 83259A78: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259A7C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259A80: 388A90D0  addi r4, r10, -0x6f30
	ctx.r[4].s64 = ctx.r[10].s64 + -28464;
	// 83259A84: 3869A1E4  addi r3, r9, -0x5e1c
	ctx.r[3].s64 = ctx.r[9].s64 + -24092;
	// 83259A88: 38AB88D0  addi r5, r11, -0x7730
	ctx.r[5].s64 = ctx.r[11].s64 + -30512;
	// 83259A8C: 4B3D45F5  bl 0x8262e080
	ctx.lr = 0x83259A90;
	sub_8262E080(ctx, base);
	// 83259A90: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259A94: 3868B958  addi r3, r8, -0x46a8
	ctx.r[3].s64 = ctx.r[8].s64 + -18088;
	// 83259A98: 4BA50489  bl 0x82ca9f20
	ctx.lr = 0x83259A9C;
	sub_82CA9F20(ctx, base);
	// 83259A9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259AB0 size=68
    let mut pc: u32 = 0x83259AB0;
    'dispatch: loop {
        match pc {
            0x83259AB0 => {
    //   block [0x83259AB0..0x83259AF4)
	// 83259AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259ABC: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 83259AC0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259AC4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259AC8: 388A90EC  addi r4, r10, -0x6f14
	ctx.r[4].s64 = ctx.r[10].s64 + -28436;
	// 83259ACC: 3869A2F0  addi r3, r9, -0x5d10
	ctx.r[3].s64 = ctx.r[9].s64 + -23824;
	// 83259AD0: 38AB9A10  addi r5, r11, -0x65f0
	ctx.r[5].s64 = ctx.r[11].s64 + -26096;
	// 83259AD4: 4B3D4635  bl 0x8262e108
	ctx.lr = 0x83259AD8;
	sub_8262E108(ctx, base);
	// 83259AD8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259ADC: 3868B970  addi r3, r8, -0x4690
	ctx.r[3].s64 = ctx.r[8].s64 + -18064;
	// 83259AE0: 4BA50441  bl 0x82ca9f20
	ctx.lr = 0x83259AE4;
	sub_82CA9F20(ctx, base);
	// 83259AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259AF8 size=68
    let mut pc: u32 = 0x83259AF8;
    'dispatch: loop {
        match pc {
            0x83259AF8 => {
    //   block [0x83259AF8..0x83259B3C)
	// 83259AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259B04: 3D608263  lis r11, -0x7d9d
	ctx.r[11].s64 = -2107441152;
	// 83259B08: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83259B0C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259B10: 388A9108  addi r4, r10, -0x6ef8
	ctx.r[4].s64 = ctx.r[10].s64 + -28408;
	// 83259B14: 3869A3FC  addi r3, r9, -0x5c04
	ctx.r[3].s64 = ctx.r[9].s64 + -23556;
	// 83259B18: 38ABA4F8  addi r5, r11, -0x5b08
	ctx.r[5].s64 = ctx.r[11].s64 + -23304;
	// 83259B1C: 4B3D44DD  bl 0x8262dff8
	ctx.lr = 0x83259B20;
	sub_8262DFF8(ctx, base);
	// 83259B20: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 83259B24: 3868B988  addi r3, r8, -0x4678
	ctx.r[3].s64 = ctx.r[8].s64 + -18040;
	// 83259B28: 4BA503F9  bl 0x82ca9f20
	ctx.lr = 0x83259B2C;
	sub_82CA9F20(ctx, base);
	// 83259B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259B40 size=84
    let mut pc: u32 = 0x83259B40;
    'dispatch: loop {
        match pc {
            0x83259B40 => {
    //   block [0x83259B40..0x83259B94)
	// 83259B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259B4C: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83259B50: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83259B54: 3D408263  lis r10, -0x7d9d
	ctx.r[10].s64 = -2107441152;
	// 83259B58: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83259B5C: 3868A508  addi r3, r8, -0x5af8
	ctx.r[3].s64 = ctx.r[8].s64 + -23288;
	// 83259B60: 38CB8C64  addi r6, r11, -0x739c
	ctx.r[6].s64 = ctx.r[11].s64 + -29596;
	// 83259B64: 38899164  addi r4, r9, -0x6e9c
	ctx.r[4].s64 = ctx.r[9].s64 + -28316;
	// 83259B68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83259B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83259B70: 38AAF5F8  addi r5, r10, -0xa08
	ctx.r[5].s64 = ctx.r[10].s64 + -2568;
	// 83259B74: 4BC2BC0D  bl 0x82e85780
	ctx.lr = 0x83259B78;
	sub_82E85780(ctx, base);
	// 83259B78: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259B7C: 3867BA40  addi r3, r7, -0x45c0
	ctx.r[3].s64 = ctx.r[7].s64 + -17856;
	// 83259B80: 4BA503A1  bl 0x82ca9f20
	ctx.lr = 0x83259B84;
	sub_82CA9F20(ctx, base);
	// 83259B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259B98 size=64
    let mut pc: u32 = 0x83259B98;
    'dispatch: loop {
        match pc {
            0x83259B98 => {
    //   block [0x83259B98..0x83259BD8)
	// 83259B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259BA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259BA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259BAC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83259BB0: 386AA61C  addi r3, r10, -0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + -23012;
	// 83259BB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259BB8: 4AFD3319  bl 0x8222ced0
	ctx.lr = 0x83259BBC;
	sub_8222CED0(ctx, base);
	// 83259BBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259BC0: 3869BA58  addi r3, r9, -0x45a8
	ctx.r[3].s64 = ctx.r[9].s64 + -17832;
	// 83259BC4: 4BA5035D  bl 0x82ca9f20
	ctx.lr = 0x83259BC8;
	sub_82CA9F20(ctx, base);
	// 83259BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259BD8 size=64
    let mut pc: u32 = 0x83259BD8;
    'dispatch: loop {
        match pc {
            0x83259BD8 => {
    //   block [0x83259BD8..0x83259C18)
	// 83259BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259BE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259BEC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83259BF0: 386AA620  addi r3, r10, -0x59e0
	ctx.r[3].s64 = ctx.r[10].s64 + -23008;
	// 83259BF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259BF8: 4AFD32D9  bl 0x8222ced0
	ctx.lr = 0x83259BFC;
	sub_8222CED0(ctx, base);
	// 83259BFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259C00: 3869BA68  addi r3, r9, -0x4598
	ctx.r[3].s64 = ctx.r[9].s64 + -17816;
	// 83259C04: 4BA5031D  bl 0x82ca9f20
	ctx.lr = 0x83259C08;
	sub_82CA9F20(ctx, base);
	// 83259C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259C18 size=64
    let mut pc: u32 = 0x83259C18;
    'dispatch: loop {
        match pc {
            0x83259C18 => {
    //   block [0x83259C18..0x83259C58)
	// 83259C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259C24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259C28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259C2C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83259C30: 386AA624  addi r3, r10, -0x59dc
	ctx.r[3].s64 = ctx.r[10].s64 + -23004;
	// 83259C34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259C38: 4AFD3299  bl 0x8222ced0
	ctx.lr = 0x83259C3C;
	sub_8222CED0(ctx, base);
	// 83259C3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259C40: 3869BA78  addi r3, r9, -0x4588
	ctx.r[3].s64 = ctx.r[9].s64 + -17800;
	// 83259C44: 4BA502DD  bl 0x82ca9f20
	ctx.lr = 0x83259C48;
	sub_82CA9F20(ctx, base);
	// 83259C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259C58 size=64
    let mut pc: u32 = 0x83259C58;
    'dispatch: loop {
        match pc {
            0x83259C58 => {
    //   block [0x83259C58..0x83259C98)
	// 83259C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259C64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259C6C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83259C70: 386AA628  addi r3, r10, -0x59d8
	ctx.r[3].s64 = ctx.r[10].s64 + -23000;
	// 83259C74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259C78: 4AFD3259  bl 0x8222ced0
	ctx.lr = 0x83259C7C;
	sub_8222CED0(ctx, base);
	// 83259C7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259C80: 3869BA88  addi r3, r9, -0x4578
	ctx.r[3].s64 = ctx.r[9].s64 + -17784;
	// 83259C84: 4BA5029D  bl 0x82ca9f20
	ctx.lr = 0x83259C88;
	sub_82CA9F20(ctx, base);
	// 83259C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259C98 size=64
    let mut pc: u32 = 0x83259C98;
    'dispatch: loop {
        match pc {
            0x83259C98 => {
    //   block [0x83259C98..0x83259CD8)
	// 83259C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259CA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259CA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259CAC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83259CB0: 386AA62C  addi r3, r10, -0x59d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22996;
	// 83259CB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259CB8: 4AFD3219  bl 0x8222ced0
	ctx.lr = 0x83259CBC;
	sub_8222CED0(ctx, base);
	// 83259CBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259CC0: 3869BA98  addi r3, r9, -0x4568
	ctx.r[3].s64 = ctx.r[9].s64 + -17768;
	// 83259CC4: 4BA5025D  bl 0x82ca9f20
	ctx.lr = 0x83259CC8;
	sub_82CA9F20(ctx, base);
	// 83259CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259CD8 size=64
    let mut pc: u32 = 0x83259CD8;
    'dispatch: loop {
        match pc {
            0x83259CD8 => {
    //   block [0x83259CD8..0x83259D18)
	// 83259CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259CE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259CE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259CEC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83259CF0: 386AA630  addi r3, r10, -0x59d0
	ctx.r[3].s64 = ctx.r[10].s64 + -22992;
	// 83259CF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259CF8: 4AFD31D9  bl 0x8222ced0
	ctx.lr = 0x83259CFC;
	sub_8222CED0(ctx, base);
	// 83259CFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259D00: 3869BAA8  addi r3, r9, -0x4558
	ctx.r[3].s64 = ctx.r[9].s64 + -17752;
	// 83259D04: 4BA5021D  bl 0x82ca9f20
	ctx.lr = 0x83259D08;
	sub_82CA9F20(ctx, base);
	// 83259D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259D18 size=88
    let mut pc: u32 = 0x83259D18;
    'dispatch: loop {
        match pc {
            0x83259D18 => {
    //   block [0x83259D18..0x83259D38)
	// 83259D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83259D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83259D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259D2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83259D30: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83259D34: 3BCBA638  addi r30, r11, -0x59c8
	ctx.r[30].s64 = ctx.r[11].s64 + -22984;
	pc = 0x83259D38; continue 'dispatch;
            }
            0x83259D38 => {
    //   block [0x83259D38..0x83259D70)
	// 83259D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83259D3C: 4B400DE5  bl 0x8265ab20
	ctx.lr = 0x83259D40;
	sub_8265AB20(ctx, base);
	// 83259D40: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83259D44: 3BDE00B0  addi r30, r30, 0xb0
	ctx.r[30].s64 = ctx.r[30].s64 + 176;
	// 83259D48: 4080FFF0  bge 0x83259d38
	if !ctx.cr[0].lt {
	pc = 0x83259D38; continue 'dispatch;
	}
	// 83259D4C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83259D50: 386BBAB8  addi r3, r11, -0x4548
	ctx.r[3].s64 = ctx.r[11].s64 + -17736;
	// 83259D54: 4BA501CD  bl 0x82ca9f20
	ctx.lr = 0x83259D58;
	sub_82CA9F20(ctx, base);
	// 83259D58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83259D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259D64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83259D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83259D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83259D70 size=12
    let mut pc: u32 = 0x83259D70;
    'dispatch: loop {
        match pc {
            0x83259D70 => {
    //   block [0x83259D70..0x83259D7C)
	// 83259D70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83259D74: 386BBB08  addi r3, r11, -0x44f8
	ctx.r[3].s64 = ctx.r[11].s64 + -17656;
	// 83259D78: 4BA501A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259D80 size=64
    let mut pc: u32 = 0x83259D80;
    'dispatch: loop {
        match pc {
            0x83259D80 => {
    //   block [0x83259D80..0x83259DC0)
	// 83259D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259D8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83259D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259D94: 388B6E58  addi r4, r11, 0x6e58
	ctx.r[4].s64 = ctx.r[11].s64 + 28248;
	// 83259D98: 386AA90C  addi r3, r10, -0x56f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22260;
	// 83259D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259DA0: 4AFD3131  bl 0x8222ced0
	ctx.lr = 0x83259DA4;
	sub_8222CED0(ctx, base);
	// 83259DA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259DA8: 3869BB18  addi r3, r9, -0x44e8
	ctx.r[3].s64 = ctx.r[9].s64 + -17640;
	// 83259DAC: 4BA50175  bl 0x82ca9f20
	ctx.lr = 0x83259DB0;
	sub_82CA9F20(ctx, base);
	// 83259DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83259DC0 size=20
    let mut pc: u32 = 0x83259DC0;
    'dispatch: loop {
        match pc {
            0x83259DC0 => {
    //   block [0x83259DC0..0x83259DD4)
	// 83259DC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83259DC4: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83259DC8: 394BA910  addi r10, r11, -0x56f0
	ctx.r[10].s64 = ctx.r[11].s64 + -22256;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83259DD8 size=12
    let mut pc: u32 = 0x83259DD8;
    'dispatch: loop {
        match pc {
            0x83259DD8 => {
    //   block [0x83259DD8..0x83259DE4)
	// 83259DD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83259DDC: 386BBB28  addi r3, r11, -0x44d8
	ctx.r[3].s64 = ctx.r[11].s64 + -17624;
	// 83259DE0: 4BA50140  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259DE8 size=96
    let mut pc: u32 = 0x83259DE8;
    'dispatch: loop {
        match pc {
            0x83259DE8 => {
    //   block [0x83259DE8..0x83259E0C)
	// 83259DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259DF4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83259DF8: 4AFC5461  bl 0x8221f258
	ctx.lr = 0x83259DFC;
	sub_8221F258(ctx, base);
	// 83259DFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83259E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83259E04: 419A0008  beq cr6, 0x83259e0c
	if ctx.cr[6].eq {
	pc = 0x83259E0C; continue 'dispatch;
	}
	// 83259E08: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83259E0C; continue 'dispatch;
            }
            0x83259E0C => {
    //   block [0x83259E0C..0x83259E18)
	// 83259E0C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83259E10: 41820008  beq 0x83259e18
	if ctx.cr[0].eq {
	pc = 0x83259E18; continue 'dispatch;
	}
	// 83259E14: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83259E18; continue 'dispatch;
            }
            0x83259E18 => {
    //   block [0x83259E18..0x83259E48)
	// 83259E18: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83259E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83259E20: 3909A930  addi r8, r9, -0x56d0
	ctx.r[8].s64 = ctx.r[9].s64 + -22224;
	// 83259E24: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83259E28: 3867BB80  addi r3, r7, -0x4480
	ctx.r[3].s64 = ctx.r[7].s64 + -17536;
	// 83259E2C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83259E30: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83259E34: 4BA500ED  bl 0x82ca9f20
	ctx.lr = 0x83259E38;
	sub_82CA9F20(ctx, base);
	// 83259E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259E48 size=64
    let mut pc: u32 = 0x83259E48;
    'dispatch: loop {
        match pc {
            0x83259E48 => {
    //   block [0x83259E48..0x83259E88)
	// 83259E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259E54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83259E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259E5C: 388BA5A8  addi r4, r11, -0x5a58
	ctx.r[4].s64 = ctx.r[11].s64 + -23128;
	// 83259E60: 386AA93C  addi r3, r10, -0x56c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22212;
	// 83259E64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259E68: 4AFD3069  bl 0x8222ced0
	ctx.lr = 0x83259E6C;
	sub_8222CED0(ctx, base);
	// 83259E6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259E70: 3869BBC8  addi r3, r9, -0x4438
	ctx.r[3].s64 = ctx.r[9].s64 + -17464;
	// 83259E74: 4BA500AD  bl 0x82ca9f20
	ctx.lr = 0x83259E78;
	sub_82CA9F20(ctx, base);
	// 83259E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83259E88 size=96
    let mut pc: u32 = 0x83259E88;
    'dispatch: loop {
        match pc {
            0x83259E88 => {
    //   block [0x83259E88..0x83259EE8)
	// 83259E88: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83259E8C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83259E90: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83259E94: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83259E98: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83259E9C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83259EA0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83259EA4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83259EA8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83259EE8 size=20
    let mut pc: u32 = 0x83259EE8;
    'dispatch: loop {
        match pc {
            0x83259EE8 => {
    //   block [0x83259EE8..0x83259EFC)
	// 83259EE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83259EEC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83259EF0: 394BA950  addi r10, r11, -0x56b0
	ctx.r[10].s64 = ctx.r[11].s64 + -22192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259F00 size=64
    let mut pc: u32 = 0x83259F00;
    'dispatch: loop {
        match pc {
            0x83259F00 => {
    //   block [0x83259F00..0x83259F40)
	// 83259F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259F0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83259F10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259F14: 388BA8E8  addi r4, r11, -0x5718
	ctx.r[4].s64 = ctx.r[11].s64 + -22296;
	// 83259F18: 386AA960  addi r3, r10, -0x56a0
	ctx.r[3].s64 = ctx.r[10].s64 + -22176;
	// 83259F1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259F20: 4AFD2FB1  bl 0x8222ced0
	ctx.lr = 0x83259F24;
	sub_8222CED0(ctx, base);
	// 83259F24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259F28: 3869BBE8  addi r3, r9, -0x4418
	ctx.r[3].s64 = ctx.r[9].s64 + -17432;
	// 83259F2C: 4BA4FFF5  bl 0x82ca9f20
	ctx.lr = 0x83259F30;
	sub_82CA9F20(ctx, base);
	// 83259F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259F40 size=64
    let mut pc: u32 = 0x83259F40;
    'dispatch: loop {
        match pc {
            0x83259F40 => {
    //   block [0x83259F40..0x83259F80)
	// 83259F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259F4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83259F50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259F54: 388BA8F4  addi r4, r11, -0x570c
	ctx.r[4].s64 = ctx.r[11].s64 + -22284;
	// 83259F58: 386AA964  addi r3, r10, -0x569c
	ctx.r[3].s64 = ctx.r[10].s64 + -22172;
	// 83259F5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259F60: 4AFD2F71  bl 0x8222ced0
	ctx.lr = 0x83259F64;
	sub_8222CED0(ctx, base);
	// 83259F64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259F68: 3869BBF8  addi r3, r9, -0x4408
	ctx.r[3].s64 = ctx.r[9].s64 + -17416;
	// 83259F6C: 4BA4FFB5  bl 0x82ca9f20
	ctx.lr = 0x83259F70;
	sub_82CA9F20(ctx, base);
	// 83259F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259F80 size=64
    let mut pc: u32 = 0x83259F80;
    'dispatch: loop {
        match pc {
            0x83259F80 => {
    //   block [0x83259F80..0x83259FC0)
	// 83259F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259F8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83259F90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259F94: 388BA8FC  addi r4, r11, -0x5704
	ctx.r[4].s64 = ctx.r[11].s64 + -22276;
	// 83259F98: 386AA968  addi r3, r10, -0x5698
	ctx.r[3].s64 = ctx.r[10].s64 + -22168;
	// 83259F9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259FA0: 4AFD2F31  bl 0x8222ced0
	ctx.lr = 0x83259FA4;
	sub_8222CED0(ctx, base);
	// 83259FA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259FA8: 3869BC08  addi r3, r9, -0x43f8
	ctx.r[3].s64 = ctx.r[9].s64 + -17400;
	// 83259FAC: 4BA4FF75  bl 0x82ca9f20
	ctx.lr = 0x83259FB0;
	sub_82CA9F20(ctx, base);
	// 83259FB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83259FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83259FC0 size=64
    let mut pc: u32 = 0x83259FC0;
    'dispatch: loop {
        match pc {
            0x83259FC0 => {
    //   block [0x83259FC0..0x8325A000)
	// 83259FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83259FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83259FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83259FCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83259FD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83259FD4: 388BA904  addi r4, r11, -0x56fc
	ctx.r[4].s64 = ctx.r[11].s64 + -22268;
	// 83259FD8: 386AA96C  addi r3, r10, -0x5694
	ctx.r[3].s64 = ctx.r[10].s64 + -22164;
	// 83259FDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83259FE0: 4AFD2EF1  bl 0x8222ced0
	ctx.lr = 0x83259FE4;
	sub_8222CED0(ctx, base);
	// 83259FE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83259FE8: 3869BC18  addi r3, r9, -0x43e8
	ctx.r[3].s64 = ctx.r[9].s64 + -17384;
	// 83259FEC: 4BA4FF35  bl 0x82ca9f20
	ctx.lr = 0x83259FF0;
	sub_82CA9F20(ctx, base);
	// 83259FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83259FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83259FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83259FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A000 size=64
    let mut pc: u32 = 0x8325A000;
    'dispatch: loop {
        match pc {
            0x8325A000 => {
    //   block [0x8325A000..0x8325A040)
	// 8325A000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A00C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A010: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A014: 388BA914  addi r4, r11, -0x56ec
	ctx.r[4].s64 = ctx.r[11].s64 + -22252;
	// 8325A018: 386AA970  addi r3, r10, -0x5690
	ctx.r[3].s64 = ctx.r[10].s64 + -22160;
	// 8325A01C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A020: 4AFD2EB1  bl 0x8222ced0
	ctx.lr = 0x8325A024;
	sub_8222CED0(ctx, base);
	// 8325A024: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A028: 3869BC28  addi r3, r9, -0x43d8
	ctx.r[3].s64 = ctx.r[9].s64 + -17368;
	// 8325A02C: 4BA4FEF5  bl 0x82ca9f20
	ctx.lr = 0x8325A030;
	sub_82CA9F20(ctx, base);
	// 8325A030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A040 size=64
    let mut pc: u32 = 0x8325A040;
    'dispatch: loop {
        match pc {
            0x8325A040 => {
    //   block [0x8325A040..0x8325A080)
	// 8325A040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A04C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A054: 388BA928  addi r4, r11, -0x56d8
	ctx.r[4].s64 = ctx.r[11].s64 + -22232;
	// 8325A058: 386AA974  addi r3, r10, -0x568c
	ctx.r[3].s64 = ctx.r[10].s64 + -22156;
	// 8325A05C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A060: 4AFD2E71  bl 0x8222ced0
	ctx.lr = 0x8325A064;
	sub_8222CED0(ctx, base);
	// 8325A064: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A068: 3869BC38  addi r3, r9, -0x43c8
	ctx.r[3].s64 = ctx.r[9].s64 + -17352;
	// 8325A06C: 4BA4FEB5  bl 0x82ca9f20
	ctx.lr = 0x8325A070;
	sub_82CA9F20(ctx, base);
	// 8325A070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A080 size=64
    let mut pc: u32 = 0x8325A080;
    'dispatch: loop {
        match pc {
            0x8325A080 => {
    //   block [0x8325A080..0x8325A0C0)
	// 8325A080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A08C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A094: 388BA93C  addi r4, r11, -0x56c4
	ctx.r[4].s64 = ctx.r[11].s64 + -22212;
	// 8325A098: 386AA978  addi r3, r10, -0x5688
	ctx.r[3].s64 = ctx.r[10].s64 + -22152;
	// 8325A09C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A0A0: 4AFD2E31  bl 0x8222ced0
	ctx.lr = 0x8325A0A4;
	sub_8222CED0(ctx, base);
	// 8325A0A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A0A8: 3869BC48  addi r3, r9, -0x43b8
	ctx.r[3].s64 = ctx.r[9].s64 + -17336;
	// 8325A0AC: 4BA4FE75  bl 0x82ca9f20
	ctx.lr = 0x8325A0B0;
	sub_82CA9F20(ctx, base);
	// 8325A0B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A0C0 size=64
    let mut pc: u32 = 0x8325A0C0;
    'dispatch: loop {
        match pc {
            0x8325A0C0 => {
    //   block [0x8325A0C0..0x8325A100)
	// 8325A0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A0C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A0CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A0D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A0D4: 388BA950  addi r4, r11, -0x56b0
	ctx.r[4].s64 = ctx.r[11].s64 + -22192;
	// 8325A0D8: 386AA97C  addi r3, r10, -0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + -22148;
	// 8325A0DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A0E0: 4AFD2DF1  bl 0x8222ced0
	ctx.lr = 0x8325A0E4;
	sub_8222CED0(ctx, base);
	// 8325A0E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A0E8: 3869BC58  addi r3, r9, -0x43a8
	ctx.r[3].s64 = ctx.r[9].s64 + -17320;
	// 8325A0EC: 4BA4FE35  bl 0x82ca9f20
	ctx.lr = 0x8325A0F0;
	sub_82CA9F20(ctx, base);
	// 8325A0F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A100 size=144
    let mut pc: u32 = 0x8325A100;
    'dispatch: loop {
        match pc {
            0x8325A100 => {
    //   block [0x8325A100..0x8325A124)
	// 8325A100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A10C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8325A110: 4AFC5149  bl 0x8221f258
	ctx.lr = 0x8325A114;
	sub_8221F258(ctx, base);
	// 8325A114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A118: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8325A11C: 419A0008  beq cr6, 0x8325a124
	if ctx.cr[6].eq {
	pc = 0x8325A124; continue 'dispatch;
	}
	// 8325A120: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A124; continue 'dispatch;
            }
            0x8325A124 => {
    //   block [0x8325A124..0x8325A130)
	// 8325A124: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A128: 41820008  beq 0x8325a130
	if ctx.cr[0].eq {
	pc = 0x8325A130; continue 'dispatch;
	}
	// 8325A12C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A130; continue 'dispatch;
            }
            0x8325A130 => {
    //   block [0x8325A130..0x8325A13C)
	// 8325A130: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A134: 41820008  beq 0x8325a13c
	if ctx.cr[0].eq {
	pc = 0x8325A13C; continue 'dispatch;
	}
	// 8325A138: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A13C; continue 'dispatch;
            }
            0x8325A13C => {
    //   block [0x8325A13C..0x8325A190)
	// 8325A13C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A140: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8325A144: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8325A148: 3909A980  addi r8, r9, -0x5680
	ctx.r[8].s64 = ctx.r[9].s64 + -22144;
	// 8325A14C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8325A150: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A154: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325A158: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8325A15C: 3867BC68  addi r3, r7, -0x4398
	ctx.r[3].s64 = ctx.r[7].s64 + -17304;
	// 8325A160: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A164: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A168: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A16C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325A170: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A174: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325A178: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A17C: 4BA4FDA5  bl 0x82ca9f20
	ctx.lr = 0x8325A180;
	sub_82CA9F20(ctx, base);
	// 8325A180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A190 size=144
    let mut pc: u32 = 0x8325A190;
    'dispatch: loop {
        match pc {
            0x8325A190 => {
    //   block [0x8325A190..0x8325A1B4)
	// 8325A190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A19C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8325A1A0: 4AFC50B9  bl 0x8221f258
	ctx.lr = 0x8325A1A4;
	sub_8221F258(ctx, base);
	// 8325A1A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A1A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8325A1AC: 419A0008  beq cr6, 0x8325a1b4
	if ctx.cr[6].eq {
	pc = 0x8325A1B4; continue 'dispatch;
	}
	// 8325A1B0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A1B4; continue 'dispatch;
            }
            0x8325A1B4 => {
    //   block [0x8325A1B4..0x8325A1C0)
	// 8325A1B4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A1B8: 41820008  beq 0x8325a1c0
	if ctx.cr[0].eq {
	pc = 0x8325A1C0; continue 'dispatch;
	}
	// 8325A1BC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A1C0; continue 'dispatch;
            }
            0x8325A1C0 => {
    //   block [0x8325A1C0..0x8325A1CC)
	// 8325A1C0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A1C4: 41820008  beq 0x8325a1cc
	if ctx.cr[0].eq {
	pc = 0x8325A1CC; continue 'dispatch;
	}
	// 8325A1C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A1CC; continue 'dispatch;
            }
            0x8325A1CC => {
    //   block [0x8325A1CC..0x8325A220)
	// 8325A1CC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A1D0: 99430021  stb r10, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 8325A1D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8325A1D8: 3909A98C  addi r8, r9, -0x5674
	ctx.r[8].s64 = ctx.r[9].s64 + -22132;
	// 8325A1DC: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8325A1E0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A1E4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325A1E8: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8325A1EC: 3867BC78  addi r3, r7, -0x4388
	ctx.r[3].s64 = ctx.r[7].s64 + -17288;
	// 8325A1F0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A1F4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A1F8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A1FC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325A200: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A204: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325A208: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A20C: 4BA4FD15  bl 0x82ca9f20
	ctx.lr = 0x8325A210;
	sub_82CA9F20(ctx, base);
	// 8325A210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A220 size=144
    let mut pc: u32 = 0x8325A220;
    'dispatch: loop {
        match pc {
            0x8325A220 => {
    //   block [0x8325A220..0x8325A244)
	// 8325A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A22C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8325A230: 4AFC5029  bl 0x8221f258
	ctx.lr = 0x8325A234;
	sub_8221F258(ctx, base);
	// 8325A234: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8325A23C: 419A0008  beq cr6, 0x8325a244
	if ctx.cr[6].eq {
	pc = 0x8325A244; continue 'dispatch;
	}
	// 8325A240: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A244; continue 'dispatch;
            }
            0x8325A244 => {
    //   block [0x8325A244..0x8325A250)
	// 8325A244: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A248: 41820008  beq 0x8325a250
	if ctx.cr[0].eq {
	pc = 0x8325A250; continue 'dispatch;
	}
	// 8325A24C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A250; continue 'dispatch;
            }
            0x8325A250 => {
    //   block [0x8325A250..0x8325A25C)
	// 8325A250: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8325A254: 41820008  beq 0x8325a25c
	if ctx.cr[0].eq {
	pc = 0x8325A25C; continue 'dispatch;
	}
	// 8325A258: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8325A25C; continue 'dispatch;
            }
            0x8325A25C => {
    //   block [0x8325A25C..0x8325A2B0)
	// 8325A25C: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A260: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 8325A264: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8325A268: 3909A998  addi r8, r9, -0x5668
	ctx.r[8].s64 = ctx.r[9].s64 + -22120;
	// 8325A26C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8325A270: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A274: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325A278: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8325A27C: 3867BC88  addi r3, r7, -0x4378
	ctx.r[3].s64 = ctx.r[7].s64 + -17272;
	// 8325A280: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A284: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A288: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A28C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325A290: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325A294: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325A298: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A29C: 4BA4FC85  bl 0x82ca9f20
	ctx.lr = 0x8325A2A0;
	sub_82CA9F20(ctx, base);
	// 8325A2A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A2B0 size=96
    let mut pc: u32 = 0x8325A2B0;
    'dispatch: loop {
        match pc {
            0x8325A2B0 => {
    //   block [0x8325A2B0..0x8325A2D4)
	// 8325A2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A2BC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8325A2C0: 4AFC4F99  bl 0x8221f258
	ctx.lr = 0x8325A2C4;
	sub_8221F258(ctx, base);
	// 8325A2C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325A2C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8325A2CC: 419A0008  beq cr6, 0x8325a2d4
	if ctx.cr[6].eq {
	pc = 0x8325A2D4; continue 'dispatch;
	}
	// 8325A2D0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325A2D4; continue 'dispatch;
            }
            0x8325A2D4 => {
    //   block [0x8325A2D4..0x8325A2E0)
	// 8325A2D4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8325A2D8: 41820008  beq 0x8325a2e0
	if ctx.cr[0].eq {
	pc = 0x8325A2E0; continue 'dispatch;
	}
	// 8325A2DC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325A2E0; continue 'dispatch;
            }
            0x8325A2E0 => {
    //   block [0x8325A2E0..0x8325A310)
	// 8325A2E0: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A2E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A2E8: 3909A9A4  addi r8, r9, -0x565c
	ctx.r[8].s64 = ctx.r[9].s64 + -22108;
	// 8325A2EC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A2F0: 3867BC98  addi r3, r7, -0x4368
	ctx.r[3].s64 = ctx.r[7].s64 + -17256;
	// 8325A2F4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A2F8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A2FC: 4BA4FC25  bl 0x82ca9f20
	ctx.lr = 0x8325A300;
	sub_82CA9F20(ctx, base);
	// 8325A300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8325A310 size=12
    let mut pc: u32 = 0x8325A310;
    'dispatch: loop {
        match pc {
            0x8325A310 => {
    //   block [0x8325A310..0x8325A31C)
	// 8325A310: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325A314: 386BBCE0  addi r3, r11, -0x4320
	ctx.r[3].s64 = ctx.r[11].s64 + -17184;
	// 8325A318: 4BA4FC08  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8325A320 size=20
    let mut pc: u32 = 0x8325A320;
    'dispatch: loop {
        match pc {
            0x8325A320 => {
    //   block [0x8325A320..0x8325A334)
	// 8325A320: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325A324: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 8325A328: 394BA9C0  addi r10, r11, -0x5640
	ctx.r[10].s64 = ctx.r[11].s64 + -22080;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A338 size=96
    let mut pc: u32 = 0x8325A338;
    'dispatch: loop {
        match pc {
            0x8325A338 => {
    //   block [0x8325A338..0x8325A35C)
	// 8325A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A344: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8325A348: 4AFC4F11  bl 0x8221f258
	ctx.lr = 0x8325A34C;
	sub_8221F258(ctx, base);
	// 8325A34C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325A350: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8325A354: 419A0008  beq cr6, 0x8325a35c
	if ctx.cr[6].eq {
	pc = 0x8325A35C; continue 'dispatch;
	}
	// 8325A358: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325A35C; continue 'dispatch;
            }
            0x8325A35C => {
    //   block [0x8325A35C..0x8325A368)
	// 8325A35C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8325A360: 41820008  beq 0x8325a368
	if ctx.cr[0].eq {
	pc = 0x8325A368; continue 'dispatch;
	}
	// 8325A364: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325A368; continue 'dispatch;
            }
            0x8325A368 => {
    //   block [0x8325A368..0x8325A398)
	// 8325A368: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A36C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A370: 3909A9D0  addi r8, r9, -0x5630
	ctx.r[8].s64 = ctx.r[9].s64 + -22064;
	// 8325A374: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A378: 3867BD38  addi r3, r7, -0x42c8
	ctx.r[3].s64 = ctx.r[7].s64 + -17096;
	// 8325A37C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A380: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A384: 4BA4FB9D  bl 0x82ca9f20
	ctx.lr = 0x8325A388;
	sub_82CA9F20(ctx, base);
	// 8325A388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A398 size=96
    let mut pc: u32 = 0x8325A398;
    'dispatch: loop {
        match pc {
            0x8325A398 => {
    //   block [0x8325A398..0x8325A3BC)
	// 8325A398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A3A4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8325A3A8: 4AFC4EB1  bl 0x8221f258
	ctx.lr = 0x8325A3AC;
	sub_8221F258(ctx, base);
	// 8325A3AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325A3B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8325A3B4: 419A0008  beq cr6, 0x8325a3bc
	if ctx.cr[6].eq {
	pc = 0x8325A3BC; continue 'dispatch;
	}
	// 8325A3B8: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325A3BC; continue 'dispatch;
            }
            0x8325A3BC => {
    //   block [0x8325A3BC..0x8325A3C8)
	// 8325A3BC: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8325A3C0: 41820008  beq 0x8325a3c8
	if ctx.cr[0].eq {
	pc = 0x8325A3C8; continue 'dispatch;
	}
	// 8325A3C4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325A3C8; continue 'dispatch;
            }
            0x8325A3C8 => {
    //   block [0x8325A3C8..0x8325A3F8)
	// 8325A3C8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A3CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325A3D0: 3909A9DC  addi r8, r9, -0x5624
	ctx.r[8].s64 = ctx.r[9].s64 + -22052;
	// 8325A3D4: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A3D8: 3867BD80  addi r3, r7, -0x4280
	ctx.r[3].s64 = ctx.r[7].s64 + -17024;
	// 8325A3DC: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A3E0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325A3E4: 4BA4FB3D  bl 0x82ca9f20
	ctx.lr = 0x8325A3E8;
	sub_82CA9F20(ctx, base);
	// 8325A3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A3F8 size=64
    let mut pc: u32 = 0x8325A3F8;
    'dispatch: loop {
        match pc {
            0x8325A3F8 => {
    //   block [0x8325A3F8..0x8325A438)
	// 8325A3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A404: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A40C: 388BA5A8  addi r4, r11, -0x5a58
	ctx.r[4].s64 = ctx.r[11].s64 + -23128;
	// 8325A410: 386AA9E8  addi r3, r10, -0x5618
	ctx.r[3].s64 = ctx.r[10].s64 + -22040;
	// 8325A414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A418: 4AFD2AB9  bl 0x8222ced0
	ctx.lr = 0x8325A41C;
	sub_8222CED0(ctx, base);
	// 8325A41C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A420: 3869BDC8  addi r3, r9, -0x4238
	ctx.r[3].s64 = ctx.r[9].s64 + -16952;
	// 8325A424: 4BA4FAFD  bl 0x82ca9f20
	ctx.lr = 0x8325A428;
	sub_82CA9F20(ctx, base);
	// 8325A428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8325A438 size=12
    let mut pc: u32 = 0x8325A438;
    'dispatch: loop {
        match pc {
            0x8325A438 => {
    //   block [0x8325A438..0x8325A444)
	// 8325A438: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325A43C: 386BBDD8  addi r3, r11, -0x4228
	ctx.r[3].s64 = ctx.r[11].s64 + -16936;
	// 8325A440: 4BA4FAE0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A448 size=332
    let mut pc: u32 = 0x8325A448;
    'dispatch: loop {
        match pc {
            0x8325A448 => {
    //   block [0x8325A448..0x8325A4CC)
	// 8325A448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8325A454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325A458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A45C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A460: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A464: 388B04E8  addi r4, r11, 0x4e8
	ctx.r[4].s64 = ctx.r[11].s64 + 1256;
	// 8325A468: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8325A46C: 4AFD2A65  bl 0x8222ced0
	ctx.lr = 0x8325A470;
	sub_8222CED0(ctx, base);
	// 8325A470: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8325A478: 388AB1F0  addi r4, r10, -0x4e10
	ctx.r[4].s64 = ctx.r[10].s64 + -19984;
	// 8325A47C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A480: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8325A484: 4AFD2A4D  bl 0x8222ced0
	ctx.lr = 0x8325A488;
	sub_8222CED0(ctx, base);
	// 8325A488: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8325A48C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325A490: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8325A494: 4AF88C75  bl 0x821e3108
	ctx.lr = 0x8325A498;
	sub_821E3108(ctx, base);
	// 8325A498: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8325A49C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325A4A0: 4AF94699  bl 0x821eeb38
	ctx.lr = 0x8325A4A4;
	sub_821EEB38(ctx, base);
	// 8325A4A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325A4A8: 4B9A9349  bl 0x82c037f0
	ctx.lr = 0x8325A4AC;
	sub_82C037F0(ctx, base);
	// 8325A4AC: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325A4B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325A4B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325A4B8: 9169AA04  stw r11, -0x55fc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-22012 as u32), ctx.r[11].u32 ) };
	// 8325A4BC: 4AF6C2AD  bl 0x821c6768
	ctx.lr = 0x8325A4C0;
	sub_821C6768(ctx, base);
	// 8325A4C0: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 8325A4C4: 3BC87088  addi r30, r8, 0x7088
	ctx.r[30].s64 = ctx.r[8].s64 + 28808;
	// 8325A4C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	pc = 0x8325A4CC; continue 'dispatch;
            }
            0x8325A4CC => {
    //   block [0x8325A4CC..0x8325A4FC)
	// 8325A4CC: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8325A4D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A4D4: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8325A4D8: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8325A4DC: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A4E0: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A4E4: 4082FFE8  bne 0x8325a4cc
	if !ctx.cr[0].eq {
	pc = 0x8325A4CC; continue 'dispatch;
	}
	// 8325A4E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8325A4EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325A4F0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8325A4F4: 4AF6C275  bl 0x821c6768
	ctx.lr = 0x8325A4F8;
	sub_821C6768(ctx, base);
	// 8325A4F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8325A4FC; continue 'dispatch;
            }
            0x8325A4FC => {
    //   block [0x8325A4FC..0x8325A528)
	// 8325A4FC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325A500: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A504: 7C805828  lwarx r4, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325A508: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325A50C: 7C80592D  stwcx. r4, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A510: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A514: 4082FFE8  bne 0x8325a4fc
	if !ctx.cr[0].eq {
	pc = 0x8325A4FC; continue 'dispatch;
	}
	// 8325A518: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8325A51C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8325A520: 4AF6C249  bl 0x821c6768
	ctx.lr = 0x8325A524;
	sub_821C6768(ctx, base);
	// 8325A524: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	pc = 0x8325A528; continue 'dispatch;
            }
            0x8325A528 => {
    //   block [0x8325A528..0x8325A550)
	// 8325A528: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8325A52C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A530: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8325A534: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8325A538: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A53C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A540: 4082FFE8  bne 0x8325a528
	if !ctx.cr[0].eq {
	pc = 0x8325A528; continue 'dispatch;
	}
	// 8325A544: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8325A548: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8325A54C: 4AF6C21D  bl 0x821c6768
	ctx.lr = 0x8325A550;
	sub_821C6768(ctx, base);
	pc = 0x8325A550; continue 'dispatch;
            }
            0x8325A550 => {
    //   block [0x8325A550..0x8325A594)
	// 8325A550: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8325A554: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A558: 7CE0F028  lwarx r7, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8325A55C: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8325A560: 7CE0F12D  stwcx. r7, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325A564: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325A568: 4082FFE8  bne 0x8325a550
	if !ctx.cr[0].eq {
	pc = 0x8325A550; continue 'dispatch;
	}
	// 8325A56C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8325A570: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 8325A574: 3865BDE8  addi r3, r5, -0x4218
	ctx.r[3].s64 = ctx.r[5].s64 + -16920;
	// 8325A578: 4BA4F9A9  bl 0x82ca9f20
	ctx.lr = 0x8325A57C;
	sub_82CA9F20(ctx, base);
	// 8325A57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8325A580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8325A58C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325A590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A598 size=532
    let mut pc: u32 = 0x8325A598;
    'dispatch: loop {
        match pc {
            0x8325A598 => {
    //   block [0x8325A598..0x8325A7AC)
	// 8325A598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A5A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325A5A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A5A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325A5AC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A5B0: 3BEBAA08  addi r31, r11, -0x55f8
	ctx.r[31].s64 = ctx.r[11].s64 + -22008;
	// 8325A5B4: 388AB560  addi r4, r10, -0x4aa0
	ctx.r[4].s64 = ctx.r[10].s64 + -19104;
	// 8325A5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325A5BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A5C0: 4AFD2911  bl 0x8222ced0
	ctx.lr = 0x8325A5C4;
	sub_8222CED0(ctx, base);
	// 8325A5C4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8325A5C8: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325A5CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325A5D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A5D4: 3889B54C  addi r4, r9, -0x4ab4
	ctx.r[4].s64 = ctx.r[9].s64 + -19124;
	// 8325A5D8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325A5DC: 4AFD28F5  bl 0x8222ced0
	ctx.lr = 0x8325A5E0;
	sub_8222CED0(ctx, base);
	// 8325A5E0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8325A5E4: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325A5E8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8325A5EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A5F0: 3888B534  addi r4, r8, -0x4acc
	ctx.r[4].s64 = ctx.r[8].s64 + -19148;
	// 8325A5F4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8325A5F8: 4AFD28D9  bl 0x8222ced0
	ctx.lr = 0x8325A5FC;
	sub_8222CED0(ctx, base);
	// 8325A5FC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8325A600: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325A604: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8325A608: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A60C: 3887B51C  addi r4, r7, -0x4ae4
	ctx.r[4].s64 = ctx.r[7].s64 + -19172;
	// 8325A610: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325A614: 4AFD28BD  bl 0x8222ced0
	ctx.lr = 0x8325A618;
	sub_8222CED0(ctx, base);
	// 8325A618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8325A61C: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325A620: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8325A624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A628: 3886B50C  addi r4, r6, -0x4af4
	ctx.r[4].s64 = ctx.r[6].s64 + -19188;
	// 8325A62C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8325A630: 4AFD28A1  bl 0x8222ced0
	ctx.lr = 0x8325A634;
	sub_8222CED0(ctx, base);
	// 8325A634: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8325A638: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325A63C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8325A640: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A644: 3884B4F8  addi r4, r4, -0x4b08
	ctx.r[4].s64 = ctx.r[4].s64 + -19208;
	// 8325A648: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8325A64C: 4AFD2885  bl 0x8222ced0
	ctx.lr = 0x8325A650;
	sub_8222CED0(ctx, base);
	// 8325A650: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8325A654: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8325A658: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8325A65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A660: 38833E18  addi r4, r3, 0x3e18
	ctx.r[4].s64 = ctx.r[3].s64 + 15896;
	// 8325A664: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8325A668: 4AFD2869  bl 0x8222ced0
	ctx.lr = 0x8325A66C;
	sub_8222CED0(ctx, base);
	// 8325A66C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8325A670: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A674: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8325A678: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A67C: 388AB4EC  addi r4, r10, -0x4b14
	ctx.r[4].s64 = ctx.r[10].s64 + -19220;
	// 8325A680: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8325A684: 4AFD284D  bl 0x8222ced0
	ctx.lr = 0x8325A688;
	sub_8222CED0(ctx, base);
	// 8325A688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8325A68C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325A690: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8325A694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A698: 3889B4CC  addi r4, r9, -0x4b34
	ctx.r[4].s64 = ctx.r[9].s64 + -19252;
	// 8325A69C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8325A6A0: 4AFD2831  bl 0x8222ced0
	ctx.lr = 0x8325A6A4;
	sub_8222CED0(ctx, base);
	// 8325A6A4: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 8325A6A8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325A6AC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8325A6B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A6B4: 3888B4AC  addi r4, r8, -0x4b54
	ctx.r[4].s64 = ctx.r[8].s64 + -19284;
	// 8325A6B8: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8325A6BC: 4AFD2815  bl 0x8222ced0
	ctx.lr = 0x8325A6C0;
	sub_8222CED0(ctx, base);
	// 8325A6C0: 39600029  li r11, 0x29
	ctx.r[11].s64 = 41;
	// 8325A6C4: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325A6C8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8325A6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A6D0: 38878D58  addi r4, r7, -0x72a8
	ctx.r[4].s64 = ctx.r[7].s64 + -29352;
	// 8325A6D4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8325A6D8: 4AFD27F9  bl 0x8222ced0
	ctx.lr = 0x8325A6DC;
	sub_8222CED0(ctx, base);
	// 8325A6DC: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 8325A6E0: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325A6E4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8325A6E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A6EC: 3886B498  addi r4, r6, -0x4b68
	ctx.r[4].s64 = ctx.r[6].s64 + -19304;
	// 8325A6F0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8325A6F4: 4AFD27DD  bl 0x8222ced0
	ctx.lr = 0x8325A6F8;
	sub_8222CED0(ctx, base);
	// 8325A6F8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8325A6FC: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325A700: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8325A704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A708: 3884B484  addi r4, r4, -0x4b7c
	ctx.r[4].s64 = ctx.r[4].s64 + -19324;
	// 8325A70C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8325A710: 4AFD27C1  bl 0x8222ced0
	ctx.lr = 0x8325A714;
	sub_8222CED0(ctx, base);
	// 8325A714: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 8325A718: 3C60820D  lis r3, -0x7df3
	ctx.r[3].s64 = -2113077248;
	// 8325A71C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8325A720: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A724: 3883B470  addi r4, r3, -0x4b90
	ctx.r[4].s64 = ctx.r[3].s64 + -19344;
	// 8325A728: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8325A72C: 4AFD27A5  bl 0x8222ced0
	ctx.lr = 0x8325A730;
	sub_8222CED0(ctx, base);
	// 8325A730: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8325A734: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325A738: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8325A73C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A740: 388AB458  addi r4, r10, -0x4ba8
	ctx.r[4].s64 = ctx.r[10].s64 + -19368;
	// 8325A744: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8325A748: 4AFD2789  bl 0x8222ced0
	ctx.lr = 0x8325A74C;
	sub_8222CED0(ctx, base);
	// 8325A74C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8325A750: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325A754: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8325A758: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A75C: 3889B440  addi r4, r9, -0x4bc0
	ctx.r[4].s64 = ctx.r[9].s64 + -19392;
	// 8325A760: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8325A764: 4AFD276D  bl 0x8222ced0
	ctx.lr = 0x8325A768;
	sub_8222CED0(ctx, base);
	// 8325A768: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8325A76C: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325A770: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8325A774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A778: 3888B428  addi r4, r8, -0x4bd8
	ctx.r[4].s64 = ctx.r[8].s64 + -19416;
	// 8325A77C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8325A780: 4AFD2751  bl 0x8222ced0
	ctx.lr = 0x8325A784;
	sub_8222CED0(ctx, base);
	// 8325A784: 3960002A  li r11, 0x2a
	ctx.r[11].s64 = 42;
	// 8325A788: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8325A78C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8325A790: 3867BDF0  addi r3, r7, -0x4210
	ctx.r[3].s64 = ctx.r[7].s64 + -16912;
	// 8325A794: 4BA4F78D  bl 0x82ca9f20
	ctx.lr = 0x8325A798;
	sub_82CA9F20(ctx, base);
	// 8325A798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A7A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325A7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


