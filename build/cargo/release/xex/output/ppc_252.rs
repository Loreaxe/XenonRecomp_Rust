pub fn sub_83277428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277428 size=56
    let mut pc: u32 = 0x83277428;
    'dispatch: loop {
        match pc {
            0x83277428 => {
    //   block [0x83277428..0x83277460)
	// 83277428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327742C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277434: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327743C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83277440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277444: 4AF7C915  bl 0x821f3d58
	ctx.lr = 0x83277448;
	sub_821F3D58(ctx, base);
	// 83277448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327744C: 906AD59C  stw r3, -0x2a64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10852 as u32), ctx.r[3].u32 ) };
	// 83277450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327745C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277460 size=56
    let mut pc: u32 = 0x83277460;
    'dispatch: loop {
        match pc {
            0x83277460 => {
    //   block [0x83277460..0x83277498)
	// 83277460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327746C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277474: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83277478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327747C: 4AF7C8DD  bl 0x821f3d58
	ctx.lr = 0x83277480;
	sub_821F3D58(ctx, base);
	// 83277480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277484: 906AD5A0  stw r3, -0x2a60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10848 as u32), ctx.r[3].u32 ) };
	// 83277488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327748C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277498 size=56
    let mut pc: u32 = 0x83277498;
    'dispatch: loop {
        match pc {
            0x83277498 => {
    //   block [0x83277498..0x832774D0)
	// 83277498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327749C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832774A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832774A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832774A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832774AC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832774B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832774B4: 4AF7C8A5  bl 0x821f3d58
	ctx.lr = 0x832774B8;
	sub_821F3D58(ctx, base);
	// 832774B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832774BC: 906AD5A4  stw r3, -0x2a5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10844 as u32), ctx.r[3].u32 ) };
	// 832774C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832774C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832774C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832774CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832774D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832774D0 size=56
    let mut pc: u32 = 0x832774D0;
    'dispatch: loop {
        match pc {
            0x832774D0 => {
    //   block [0x832774D0..0x83277508)
	// 832774D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832774D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832774D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832774DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832774E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832774E4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832774E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832774EC: 4AF7C86D  bl 0x821f3d58
	ctx.lr = 0x832774F0;
	sub_821F3D58(ctx, base);
	// 832774F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832774F4: 906AD5A8  stw r3, -0x2a58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10840 as u32), ctx.r[3].u32 ) };
	// 832774F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832774FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277508 size=56
    let mut pc: u32 = 0x83277508;
    'dispatch: loop {
        match pc {
            0x83277508 => {
    //   block [0x83277508..0x83277540)
	// 83277508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327750C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327751C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83277520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277524: 4AF7C835  bl 0x821f3d58
	ctx.lr = 0x83277528;
	sub_821F3D58(ctx, base);
	// 83277528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327752C: 906AD5AC  stw r3, -0x2a54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10836 as u32), ctx.r[3].u32 ) };
	// 83277530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327753C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277540 size=56
    let mut pc: u32 = 0x83277540;
    'dispatch: loop {
        match pc {
            0x83277540 => {
    //   block [0x83277540..0x83277578)
	// 83277540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327754C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277550: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277554: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83277558: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327755C: 4AF7C7FD  bl 0x821f3d58
	ctx.lr = 0x83277560;
	sub_821F3D58(ctx, base);
	// 83277560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277564: 906AD5B0  stw r3, -0x2a50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10832 as u32), ctx.r[3].u32 ) };
	// 83277568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327756C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277578 size=56
    let mut pc: u32 = 0x83277578;
    'dispatch: loop {
        match pc {
            0x83277578 => {
    //   block [0x83277578..0x832775B0)
	// 83277578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327757C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277584: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277588: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327758C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83277590: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277594: 4AF7C7C5  bl 0x821f3d58
	ctx.lr = 0x83277598;
	sub_821F3D58(ctx, base);
	// 83277598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327759C: 906AD5B4  stw r3, -0x2a4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10828 as u32), ctx.r[3].u32 ) };
	// 832775A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832775A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832775A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832775AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832775B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832775B0 size=56
    let mut pc: u32 = 0x832775B0;
    'dispatch: loop {
        match pc {
            0x832775B0 => {
    //   block [0x832775B0..0x832775E8)
	// 832775B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832775B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832775B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832775BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832775C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832775C4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832775C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832775CC: 4AF7C78D  bl 0x821f3d58
	ctx.lr = 0x832775D0;
	sub_821F3D58(ctx, base);
	// 832775D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832775D4: 906AD5B8  stw r3, -0x2a48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10824 as u32), ctx.r[3].u32 ) };
	// 832775D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832775DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832775E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832775E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832775E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832775E8 size=56
    let mut pc: u32 = 0x832775E8;
    'dispatch: loop {
        match pc {
            0x832775E8 => {
    //   block [0x832775E8..0x83277620)
	// 832775E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832775EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832775F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832775F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832775F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832775FC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83277600: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277604: 4AF7C755  bl 0x821f3d58
	ctx.lr = 0x83277608;
	sub_821F3D58(ctx, base);
	// 83277608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327760C: 906AD5BC  stw r3, -0x2a44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10820 as u32), ctx.r[3].u32 ) };
	// 83277610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327761C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277620 size=56
    let mut pc: u32 = 0x83277620;
    'dispatch: loop {
        match pc {
            0x83277620 => {
    //   block [0x83277620..0x83277658)
	// 83277620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327762C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277630: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277634: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83277638: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327763C: 4AF7C71D  bl 0x821f3d58
	ctx.lr = 0x83277640;
	sub_821F3D58(ctx, base);
	// 83277640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277644: 906AD5C0  stw r3, -0x2a40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10816 as u32), ctx.r[3].u32 ) };
	// 83277648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327764C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277658 size=56
    let mut pc: u32 = 0x83277658;
    'dispatch: loop {
        match pc {
            0x83277658 => {
    //   block [0x83277658..0x83277690)
	// 83277658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327765C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277664: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277668: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327766C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83277670: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277674: 4AF7C6E5  bl 0x821f3d58
	ctx.lr = 0x83277678;
	sub_821F3D58(ctx, base);
	// 83277678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327767C: 906AD5C4  stw r3, -0x2a3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10812 as u32), ctx.r[3].u32 ) };
	// 83277680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327768C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277690 size=56
    let mut pc: u32 = 0x83277690;
    'dispatch: loop {
        match pc {
            0x83277690 => {
    //   block [0x83277690..0x832776C8)
	// 83277690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327769C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832776A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832776A4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832776A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832776AC: 4AF7C6AD  bl 0x821f3d58
	ctx.lr = 0x832776B0;
	sub_821F3D58(ctx, base);
	// 832776B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832776B4: 906AD5C8  stw r3, -0x2a38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10808 as u32), ctx.r[3].u32 ) };
	// 832776B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832776BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832776C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832776C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832776C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832776C8 size=56
    let mut pc: u32 = 0x832776C8;
    'dispatch: loop {
        match pc {
            0x832776C8 => {
    //   block [0x832776C8..0x83277700)
	// 832776C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832776CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832776D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832776D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832776D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832776DC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832776E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832776E4: 4AF7C675  bl 0x821f3d58
	ctx.lr = 0x832776E8;
	sub_821F3D58(ctx, base);
	// 832776E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832776EC: 906AD5CC  stw r3, -0x2a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10804 as u32), ctx.r[3].u32 ) };
	// 832776F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832776F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832776F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832776FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277700 size=56
    let mut pc: u32 = 0x83277700;
    'dispatch: loop {
        match pc {
            0x83277700 => {
    //   block [0x83277700..0x83277738)
	// 83277700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327770C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277710: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277714: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83277718: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327771C: 4AF7C63D  bl 0x821f3d58
	ctx.lr = 0x83277720;
	sub_821F3D58(ctx, base);
	// 83277720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277724: 906AD5D0  stw r3, -0x2a30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10800 as u32), ctx.r[3].u32 ) };
	// 83277728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327772C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277738 size=56
    let mut pc: u32 = 0x83277738;
    'dispatch: loop {
        match pc {
            0x83277738 => {
    //   block [0x83277738..0x83277770)
	// 83277738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327773C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277748: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327774C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83277750: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277754: 4AF7C605  bl 0x821f3d58
	ctx.lr = 0x83277758;
	sub_821F3D58(ctx, base);
	// 83277758: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327775C: 906AD5D4  stw r3, -0x2a2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10796 as u32), ctx.r[3].u32 ) };
	// 83277760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327776C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277770 size=56
    let mut pc: u32 = 0x83277770;
    'dispatch: loop {
        match pc {
            0x83277770 => {
    //   block [0x83277770..0x832777A8)
	// 83277770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327777C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277780: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277784: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83277788: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327778C: 4AF7C5CD  bl 0x821f3d58
	ctx.lr = 0x83277790;
	sub_821F3D58(ctx, base);
	// 83277790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277794: 906AD5D8  stw r3, -0x2a28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10792 as u32), ctx.r[3].u32 ) };
	// 83277798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327779C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832777A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832777A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832777A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832777A8 size=56
    let mut pc: u32 = 0x832777A8;
    'dispatch: loop {
        match pc {
            0x832777A8 => {
    //   block [0x832777A8..0x832777E0)
	// 832777A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832777AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832777B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832777B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832777B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832777BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832777C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832777C4: 4AF7C595  bl 0x821f3d58
	ctx.lr = 0x832777C8;
	sub_821F3D58(ctx, base);
	// 832777C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832777CC: 906AD5DC  stw r3, -0x2a24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10788 as u32), ctx.r[3].u32 ) };
	// 832777D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832777D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832777D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832777DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832777E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832777E0 size=56
    let mut pc: u32 = 0x832777E0;
    'dispatch: loop {
        match pc {
            0x832777E0 => {
    //   block [0x832777E0..0x83277818)
	// 832777E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832777E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832777E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832777EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832777F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832777F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832777F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832777FC: 4AF7C55D  bl 0x821f3d58
	ctx.lr = 0x83277800;
	sub_821F3D58(ctx, base);
	// 83277800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277804: 906AD5E0  stw r3, -0x2a20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10784 as u32), ctx.r[3].u32 ) };
	// 83277808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327780C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277818 size=56
    let mut pc: u32 = 0x83277818;
    'dispatch: loop {
        match pc {
            0x83277818 => {
    //   block [0x83277818..0x83277850)
	// 83277818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327781C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277824: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277828: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327782C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83277830: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277834: 4AF7C525  bl 0x821f3d58
	ctx.lr = 0x83277838;
	sub_821F3D58(ctx, base);
	// 83277838: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327783C: 906AD5E4  stw r3, -0x2a1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10780 as u32), ctx.r[3].u32 ) };
	// 83277840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327784C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277850 size=56
    let mut pc: u32 = 0x83277850;
    'dispatch: loop {
        match pc {
            0x83277850 => {
    //   block [0x83277850..0x83277888)
	// 83277850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327785C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277860: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277864: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83277868: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327786C: 4AF7C4ED  bl 0x821f3d58
	ctx.lr = 0x83277870;
	sub_821F3D58(ctx, base);
	// 83277870: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277874: 906AD5E8  stw r3, -0x2a18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10776 as u32), ctx.r[3].u32 ) };
	// 83277878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327787C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277888 size=64
    let mut pc: u32 = 0x83277888;
    'dispatch: loop {
        match pc {
            0x83277888 => {
    //   block [0x83277888..0x832778C8)
	// 83277888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327788C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277894: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277898: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327789C: 388BCC94  addi r4, r11, -0x336c
	ctx.r[4].s64 = ctx.r[11].s64 + -13164;
	// 832778A0: 386AD5EC  addi r3, r10, -0x2a14
	ctx.r[3].s64 = ctx.r[10].s64 + -10772;
	// 832778A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832778A8: 4AFB5629  bl 0x8222ced0
	ctx.lr = 0x832778AC;
	sub_8222CED0(ctx, base);
	// 832778AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832778B0: 3869FC10  addi r3, r9, -0x3f0
	ctx.r[3].s64 = ctx.r[9].s64 + -1008;
	// 832778B4: 4BA3266D  bl 0x82ca9f20
	ctx.lr = 0x832778B8;
	sub_82CA9F20(ctx, base);
	// 832778B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832778BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832778C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832778C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832778C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832778C8 size=64
    let mut pc: u32 = 0x832778C8;
    'dispatch: loop {
        match pc {
            0x832778C8 => {
    //   block [0x832778C8..0x83277908)
	// 832778C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832778CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832778D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832778D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832778D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832778DC: 388BCCC8  addi r4, r11, -0x3338
	ctx.r[4].s64 = ctx.r[11].s64 + -13112;
	// 832778E0: 386AD5F0  addi r3, r10, -0x2a10
	ctx.r[3].s64 = ctx.r[10].s64 + -10768;
	// 832778E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832778E8: 4AFB55E9  bl 0x8222ced0
	ctx.lr = 0x832778EC;
	sub_8222CED0(ctx, base);
	// 832778EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832778F0: 3869FC20  addi r3, r9, -0x3e0
	ctx.r[3].s64 = ctx.r[9].s64 + -992;
	// 832778F4: 4BA3262D  bl 0x82ca9f20
	ctx.lr = 0x832778F8;
	sub_82CA9F20(ctx, base);
	// 832778F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832778FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277908 size=64
    let mut pc: u32 = 0x83277908;
    'dispatch: loop {
        match pc {
            0x83277908 => {
    //   block [0x83277908..0x83277948)
	// 83277908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277914: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277918: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327791C: 388BCD00  addi r4, r11, -0x3300
	ctx.r[4].s64 = ctx.r[11].s64 + -13056;
	// 83277920: 386AD5F4  addi r3, r10, -0x2a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -10764;
	// 83277924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277928: 4AFB55A9  bl 0x8222ced0
	ctx.lr = 0x8327792C;
	sub_8222CED0(ctx, base);
	// 8327792C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277930: 3869FC30  addi r3, r9, -0x3d0
	ctx.r[3].s64 = ctx.r[9].s64 + -976;
	// 83277934: 4BA325ED  bl 0x82ca9f20
	ctx.lr = 0x83277938;
	sub_82CA9F20(ctx, base);
	// 83277938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327793C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277948 size=64
    let mut pc: u32 = 0x83277948;
    'dispatch: loop {
        match pc {
            0x83277948 => {
    //   block [0x83277948..0x83277988)
	// 83277948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327794C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277954: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277958: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327795C: 388BCD34  addi r4, r11, -0x32cc
	ctx.r[4].s64 = ctx.r[11].s64 + -13004;
	// 83277960: 386AD5F8  addi r3, r10, -0x2a08
	ctx.r[3].s64 = ctx.r[10].s64 + -10760;
	// 83277964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277968: 4AFB5569  bl 0x8222ced0
	ctx.lr = 0x8327796C;
	sub_8222CED0(ctx, base);
	// 8327796C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277970: 3869FC40  addi r3, r9, -0x3c0
	ctx.r[3].s64 = ctx.r[9].s64 + -960;
	// 83277974: 4BA325AD  bl 0x82ca9f20
	ctx.lr = 0x83277978;
	sub_82CA9F20(ctx, base);
	// 83277978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277988 size=64
    let mut pc: u32 = 0x83277988;
    'dispatch: loop {
        match pc {
            0x83277988 => {
    //   block [0x83277988..0x832779C8)
	// 83277988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327798C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277994: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327799C: 388BCD70  addi r4, r11, -0x3290
	ctx.r[4].s64 = ctx.r[11].s64 + -12944;
	// 832779A0: 386AD5FC  addi r3, r10, -0x2a04
	ctx.r[3].s64 = ctx.r[10].s64 + -10756;
	// 832779A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832779A8: 4AFB5529  bl 0x8222ced0
	ctx.lr = 0x832779AC;
	sub_8222CED0(ctx, base);
	// 832779AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832779B0: 3869FC50  addi r3, r9, -0x3b0
	ctx.r[3].s64 = ctx.r[9].s64 + -944;
	// 832779B4: 4BA3256D  bl 0x82ca9f20
	ctx.lr = 0x832779B8;
	sub_82CA9F20(ctx, base);
	// 832779B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832779BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832779C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832779C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832779C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832779C8 size=56
    let mut pc: u32 = 0x832779C8;
    'dispatch: loop {
        match pc {
            0x832779C8 => {
    //   block [0x832779C8..0x83277A00)
	// 832779C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832779CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832779D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832779D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832779D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832779DC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832779E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832779E4: 4AF7C375  bl 0x821f3d58
	ctx.lr = 0x832779E8;
	sub_821F3D58(ctx, base);
	// 832779E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832779EC: 906AD600  stw r3, -0x2a00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10752 as u32), ctx.r[3].u32 ) };
	// 832779F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832779F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832779F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832779FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277A00 size=56
    let mut pc: u32 = 0x83277A00;
    'dispatch: loop {
        match pc {
            0x83277A00 => {
    //   block [0x83277A00..0x83277A38)
	// 83277A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277A0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277A10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277A14: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83277A18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277A1C: 4AF7C33D  bl 0x821f3d58
	ctx.lr = 0x83277A20;
	sub_821F3D58(ctx, base);
	// 83277A20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277A24: 906AD604  stw r3, -0x29fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10748 as u32), ctx.r[3].u32 ) };
	// 83277A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277A38 size=56
    let mut pc: u32 = 0x83277A38;
    'dispatch: loop {
        match pc {
            0x83277A38 => {
    //   block [0x83277A38..0x83277A70)
	// 83277A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277A48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277A4C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83277A50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277A54: 4AF7C305  bl 0x821f3d58
	ctx.lr = 0x83277A58;
	sub_821F3D58(ctx, base);
	// 83277A58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277A5C: 906AD608  stw r3, -0x29f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10744 as u32), ctx.r[3].u32 ) };
	// 83277A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277A70 size=56
    let mut pc: u32 = 0x83277A70;
    'dispatch: loop {
        match pc {
            0x83277A70 => {
    //   block [0x83277A70..0x83277AA8)
	// 83277A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277A7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277A80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277A84: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83277A88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277A8C: 4AF7C2CD  bl 0x821f3d58
	ctx.lr = 0x83277A90;
	sub_821F3D58(ctx, base);
	// 83277A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277A94: 906AD60C  stw r3, -0x29f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10740 as u32), ctx.r[3].u32 ) };
	// 83277A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277AA8 size=56
    let mut pc: u32 = 0x83277AA8;
    'dispatch: loop {
        match pc {
            0x83277AA8 => {
    //   block [0x83277AA8..0x83277AE0)
	// 83277AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277AB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277AB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277ABC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83277AC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277AC4: 4AF7C295  bl 0x821f3d58
	ctx.lr = 0x83277AC8;
	sub_821F3D58(ctx, base);
	// 83277AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277ACC: 906AD610  stw r3, -0x29f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10736 as u32), ctx.r[3].u32 ) };
	// 83277AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277AE0 size=56
    let mut pc: u32 = 0x83277AE0;
    'dispatch: loop {
        match pc {
            0x83277AE0 => {
    //   block [0x83277AE0..0x83277B18)
	// 83277AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277AEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277AF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277AF4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83277AF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277AFC: 4AF7C25D  bl 0x821f3d58
	ctx.lr = 0x83277B00;
	sub_821F3D58(ctx, base);
	// 83277B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277B04: 906AD614  stw r3, -0x29ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10732 as u32), ctx.r[3].u32 ) };
	// 83277B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277B18 size=56
    let mut pc: u32 = 0x83277B18;
    'dispatch: loop {
        match pc {
            0x83277B18 => {
    //   block [0x83277B18..0x83277B50)
	// 83277B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277B24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277B28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277B2C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83277B30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277B34: 4AF7C225  bl 0x821f3d58
	ctx.lr = 0x83277B38;
	sub_821F3D58(ctx, base);
	// 83277B38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277B3C: 906AD618  stw r3, -0x29e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10728 as u32), ctx.r[3].u32 ) };
	// 83277B40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277B50 size=56
    let mut pc: u32 = 0x83277B50;
    'dispatch: loop {
        match pc {
            0x83277B50 => {
    //   block [0x83277B50..0x83277B88)
	// 83277B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277B58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277B5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277B60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277B64: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83277B68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277B6C: 4AF7C1ED  bl 0x821f3d58
	ctx.lr = 0x83277B70;
	sub_821F3D58(ctx, base);
	// 83277B70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277B74: 906AD61C  stw r3, -0x29e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10724 as u32), ctx.r[3].u32 ) };
	// 83277B78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277B88 size=56
    let mut pc: u32 = 0x83277B88;
    'dispatch: loop {
        match pc {
            0x83277B88 => {
    //   block [0x83277B88..0x83277BC0)
	// 83277B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277B90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277B94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277B98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277B9C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83277BA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277BA4: 4AF7C1B5  bl 0x821f3d58
	ctx.lr = 0x83277BA8;
	sub_821F3D58(ctx, base);
	// 83277BA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277BAC: 906AD620  stw r3, -0x29e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10720 as u32), ctx.r[3].u32 ) };
	// 83277BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277BC0 size=56
    let mut pc: u32 = 0x83277BC0;
    'dispatch: loop {
        match pc {
            0x83277BC0 => {
    //   block [0x83277BC0..0x83277BF8)
	// 83277BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277BCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277BD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277BD4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83277BD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277BDC: 4AF7C17D  bl 0x821f3d58
	ctx.lr = 0x83277BE0;
	sub_821F3D58(ctx, base);
	// 83277BE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277BE4: 906AD624  stw r3, -0x29dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10716 as u32), ctx.r[3].u32 ) };
	// 83277BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277BF8 size=56
    let mut pc: u32 = 0x83277BF8;
    'dispatch: loop {
        match pc {
            0x83277BF8 => {
    //   block [0x83277BF8..0x83277C30)
	// 83277BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277C08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277C0C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83277C10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277C14: 4AF7C145  bl 0x821f3d58
	ctx.lr = 0x83277C18;
	sub_821F3D58(ctx, base);
	// 83277C18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277C1C: 906AD628  stw r3, -0x29d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10712 as u32), ctx.r[3].u32 ) };
	// 83277C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277C30 size=56
    let mut pc: u32 = 0x83277C30;
    'dispatch: loop {
        match pc {
            0x83277C30 => {
    //   block [0x83277C30..0x83277C68)
	// 83277C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277C3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277C40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277C44: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83277C48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277C4C: 4AF7C10D  bl 0x821f3d58
	ctx.lr = 0x83277C50;
	sub_821F3D58(ctx, base);
	// 83277C50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277C54: 906AD62C  stw r3, -0x29d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10708 as u32), ctx.r[3].u32 ) };
	// 83277C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277C68 size=56
    let mut pc: u32 = 0x83277C68;
    'dispatch: loop {
        match pc {
            0x83277C68 => {
    //   block [0x83277C68..0x83277CA0)
	// 83277C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277C74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277C78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277C7C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83277C80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277C84: 4AF7C0D5  bl 0x821f3d58
	ctx.lr = 0x83277C88;
	sub_821F3D58(ctx, base);
	// 83277C88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277C8C: 906AD630  stw r3, -0x29d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10704 as u32), ctx.r[3].u32 ) };
	// 83277C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277CA0 size=56
    let mut pc: u32 = 0x83277CA0;
    'dispatch: loop {
        match pc {
            0x83277CA0 => {
    //   block [0x83277CA0..0x83277CD8)
	// 83277CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277CAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277CB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277CB4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83277CB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277CBC: 4AF7C09D  bl 0x821f3d58
	ctx.lr = 0x83277CC0;
	sub_821F3D58(ctx, base);
	// 83277CC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277CC4: 906AD634  stw r3, -0x29cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10700 as u32), ctx.r[3].u32 ) };
	// 83277CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277CD8 size=56
    let mut pc: u32 = 0x83277CD8;
    'dispatch: loop {
        match pc {
            0x83277CD8 => {
    //   block [0x83277CD8..0x83277D10)
	// 83277CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277CE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277CE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277CEC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83277CF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277CF4: 4AF7C065  bl 0x821f3d58
	ctx.lr = 0x83277CF8;
	sub_821F3D58(ctx, base);
	// 83277CF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277CFC: 906AD638  stw r3, -0x29c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10696 as u32), ctx.r[3].u32 ) };
	// 83277D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277D10 size=56
    let mut pc: u32 = 0x83277D10;
    'dispatch: loop {
        match pc {
            0x83277D10 => {
    //   block [0x83277D10..0x83277D48)
	// 83277D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277D1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277D20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277D24: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83277D28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277D2C: 4AF7C02D  bl 0x821f3d58
	ctx.lr = 0x83277D30;
	sub_821F3D58(ctx, base);
	// 83277D30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277D34: 906AD63C  stw r3, -0x29c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10692 as u32), ctx.r[3].u32 ) };
	// 83277D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277D48 size=56
    let mut pc: u32 = 0x83277D48;
    'dispatch: loop {
        match pc {
            0x83277D48 => {
    //   block [0x83277D48..0x83277D80)
	// 83277D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277D54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277D58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277D5C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83277D60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277D64: 4AF7BFF5  bl 0x821f3d58
	ctx.lr = 0x83277D68;
	sub_821F3D58(ctx, base);
	// 83277D68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277D6C: 906AD640  stw r3, -0x29c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10688 as u32), ctx.r[3].u32 ) };
	// 83277D70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277D80 size=56
    let mut pc: u32 = 0x83277D80;
    'dispatch: loop {
        match pc {
            0x83277D80 => {
    //   block [0x83277D80..0x83277DB8)
	// 83277D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277D8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277D90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277D94: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83277D98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277D9C: 4AF7BFBD  bl 0x821f3d58
	ctx.lr = 0x83277DA0;
	sub_821F3D58(ctx, base);
	// 83277DA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277DA4: 906AD644  stw r3, -0x29bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10684 as u32), ctx.r[3].u32 ) };
	// 83277DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277DB8 size=56
    let mut pc: u32 = 0x83277DB8;
    'dispatch: loop {
        match pc {
            0x83277DB8 => {
    //   block [0x83277DB8..0x83277DF0)
	// 83277DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277DC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277DCC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83277DD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277DD4: 4AF7BF85  bl 0x821f3d58
	ctx.lr = 0x83277DD8;
	sub_821F3D58(ctx, base);
	// 83277DD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277DDC: 906AD648  stw r3, -0x29b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10680 as u32), ctx.r[3].u32 ) };
	// 83277DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277DF0 size=56
    let mut pc: u32 = 0x83277DF0;
    'dispatch: loop {
        match pc {
            0x83277DF0 => {
    //   block [0x83277DF0..0x83277E28)
	// 83277DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277DFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277E00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277E04: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83277E08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277E0C: 4AF7BF4D  bl 0x821f3d58
	ctx.lr = 0x83277E10;
	sub_821F3D58(ctx, base);
	// 83277E10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277E14: 906AD64C  stw r3, -0x29b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10676 as u32), ctx.r[3].u32 ) };
	// 83277E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277E28 size=56
    let mut pc: u32 = 0x83277E28;
    'dispatch: loop {
        match pc {
            0x83277E28 => {
    //   block [0x83277E28..0x83277E60)
	// 83277E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277E34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277E38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277E3C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83277E40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277E44: 4AF7BF15  bl 0x821f3d58
	ctx.lr = 0x83277E48;
	sub_821F3D58(ctx, base);
	// 83277E48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277E4C: 906AD650  stw r3, -0x29b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10672 as u32), ctx.r[3].u32 ) };
	// 83277E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277E60 size=64
    let mut pc: u32 = 0x83277E60;
    'dispatch: loop {
        match pc {
            0x83277E60 => {
    //   block [0x83277E60..0x83277EA0)
	// 83277E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277E6C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277E74: 388BCE90  addi r4, r11, -0x3170
	ctx.r[4].s64 = ctx.r[11].s64 + -12656;
	// 83277E78: 386AD654  addi r3, r10, -0x29ac
	ctx.r[3].s64 = ctx.r[10].s64 + -10668;
	// 83277E7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277E80: 4AFB5051  bl 0x8222ced0
	ctx.lr = 0x83277E84;
	sub_8222CED0(ctx, base);
	// 83277E84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277E88: 3869FC60  addi r3, r9, -0x3a0
	ctx.r[3].s64 = ctx.r[9].s64 + -928;
	// 83277E8C: 4BA32095  bl 0x82ca9f20
	ctx.lr = 0x83277E90;
	sub_82CA9F20(ctx, base);
	// 83277E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277EA0 size=64
    let mut pc: u32 = 0x83277EA0;
    'dispatch: loop {
        match pc {
            0x83277EA0 => {
    //   block [0x83277EA0..0x83277EE0)
	// 83277EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277EAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277EB4: 388BCEC0  addi r4, r11, -0x3140
	ctx.r[4].s64 = ctx.r[11].s64 + -12608;
	// 83277EB8: 386AD658  addi r3, r10, -0x29a8
	ctx.r[3].s64 = ctx.r[10].s64 + -10664;
	// 83277EBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277EC0: 4AFB5011  bl 0x8222ced0
	ctx.lr = 0x83277EC4;
	sub_8222CED0(ctx, base);
	// 83277EC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277EC8: 3869FC70  addi r3, r9, -0x390
	ctx.r[3].s64 = ctx.r[9].s64 + -912;
	// 83277ECC: 4BA32055  bl 0x82ca9f20
	ctx.lr = 0x83277ED0;
	sub_82CA9F20(ctx, base);
	// 83277ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277EE0 size=64
    let mut pc: u32 = 0x83277EE0;
    'dispatch: loop {
        match pc {
            0x83277EE0 => {
    //   block [0x83277EE0..0x83277F20)
	// 83277EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277EEC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277EF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277EF4: 388BCEFC  addi r4, r11, -0x3104
	ctx.r[4].s64 = ctx.r[11].s64 + -12548;
	// 83277EF8: 386AD65C  addi r3, r10, -0x29a4
	ctx.r[3].s64 = ctx.r[10].s64 + -10660;
	// 83277EFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277F00: 4AFB4FD1  bl 0x8222ced0
	ctx.lr = 0x83277F04;
	sub_8222CED0(ctx, base);
	// 83277F04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277F08: 3869FC80  addi r3, r9, -0x380
	ctx.r[3].s64 = ctx.r[9].s64 + -896;
	// 83277F0C: 4BA32015  bl 0x82ca9f20
	ctx.lr = 0x83277F10;
	sub_82CA9F20(ctx, base);
	// 83277F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277F20 size=64
    let mut pc: u32 = 0x83277F20;
    'dispatch: loop {
        match pc {
            0x83277F20 => {
    //   block [0x83277F20..0x83277F60)
	// 83277F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277F2C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277F34: 388BCF38  addi r4, r11, -0x30c8
	ctx.r[4].s64 = ctx.r[11].s64 + -12488;
	// 83277F38: 386AD660  addi r3, r10, -0x29a0
	ctx.r[3].s64 = ctx.r[10].s64 + -10656;
	// 83277F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277F40: 4AFB4F91  bl 0x8222ced0
	ctx.lr = 0x83277F44;
	sub_8222CED0(ctx, base);
	// 83277F44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277F48: 3869FC90  addi r3, r9, -0x370
	ctx.r[3].s64 = ctx.r[9].s64 + -880;
	// 83277F4C: 4BA31FD5  bl 0x82ca9f20
	ctx.lr = 0x83277F50;
	sub_82CA9F20(ctx, base);
	// 83277F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277F60 size=64
    let mut pc: u32 = 0x83277F60;
    'dispatch: loop {
        match pc {
            0x83277F60 => {
    //   block [0x83277F60..0x83277FA0)
	// 83277F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277F6C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277F74: 388BCF70  addi r4, r11, -0x3090
	ctx.r[4].s64 = ctx.r[11].s64 + -12432;
	// 83277F78: 386AD664  addi r3, r10, -0x299c
	ctx.r[3].s64 = ctx.r[10].s64 + -10652;
	// 83277F7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277F80: 4AFB4F51  bl 0x8222ced0
	ctx.lr = 0x83277F84;
	sub_8222CED0(ctx, base);
	// 83277F84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277F88: 3869FCA0  addi r3, r9, -0x360
	ctx.r[3].s64 = ctx.r[9].s64 + -864;
	// 83277F8C: 4BA31F95  bl 0x82ca9f20
	ctx.lr = 0x83277F90;
	sub_82CA9F20(ctx, base);
	// 83277F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277FA0 size=64
    let mut pc: u32 = 0x83277FA0;
    'dispatch: loop {
        match pc {
            0x83277FA0 => {
    //   block [0x83277FA0..0x83277FE0)
	// 83277FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277FAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277FB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277FB4: 388BCFA8  addi r4, r11, -0x3058
	ctx.r[4].s64 = ctx.r[11].s64 + -12376;
	// 83277FB8: 386AD668  addi r3, r10, -0x2998
	ctx.r[3].s64 = ctx.r[10].s64 + -10648;
	// 83277FBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277FC0: 4AFB4F11  bl 0x8222ced0
	ctx.lr = 0x83277FC4;
	sub_8222CED0(ctx, base);
	// 83277FC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277FC8: 3869FCB0  addi r3, r9, -0x350
	ctx.r[3].s64 = ctx.r[9].s64 + -848;
	// 83277FCC: 4BA31F55  bl 0x82ca9f20
	ctx.lr = 0x83277FD0;
	sub_82CA9F20(ctx, base);
	// 83277FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277FE0 size=64
    let mut pc: u32 = 0x83277FE0;
    'dispatch: loop {
        match pc {
            0x83277FE0 => {
    //   block [0x83277FE0..0x83278020)
	// 83277FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277FEC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277FF4: 388BCFE0  addi r4, r11, -0x3020
	ctx.r[4].s64 = ctx.r[11].s64 + -12320;
	// 83277FF8: 386AD66C  addi r3, r10, -0x2994
	ctx.r[3].s64 = ctx.r[10].s64 + -10644;
	// 83277FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278000: 4AFB4ED1  bl 0x8222ced0
	ctx.lr = 0x83278004;
	sub_8222CED0(ctx, base);
	// 83278004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278008: 3869FCC0  addi r3, r9, -0x340
	ctx.r[3].s64 = ctx.r[9].s64 + -832;
	// 8327800C: 4BA31F15  bl 0x82ca9f20
	ctx.lr = 0x83278010;
	sub_82CA9F20(ctx, base);
	// 83278010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327801C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278020 size=64
    let mut pc: u32 = 0x83278020;
    'dispatch: loop {
        match pc {
            0x83278020 => {
    //   block [0x83278020..0x83278060)
	// 83278020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327802C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278034: 388BD018  addi r4, r11, -0x2fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -12264;
	// 83278038: 386AD670  addi r3, r10, -0x2990
	ctx.r[3].s64 = ctx.r[10].s64 + -10640;
	// 8327803C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278040: 4AFB4E91  bl 0x8222ced0
	ctx.lr = 0x83278044;
	sub_8222CED0(ctx, base);
	// 83278044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278048: 3869FCD0  addi r3, r9, -0x330
	ctx.r[3].s64 = ctx.r[9].s64 + -816;
	// 8327804C: 4BA31ED5  bl 0x82ca9f20
	ctx.lr = 0x83278050;
	sub_82CA9F20(ctx, base);
	// 83278050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327805C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278060 size=64
    let mut pc: u32 = 0x83278060;
    'dispatch: loop {
        match pc {
            0x83278060 => {
    //   block [0x83278060..0x832780A0)
	// 83278060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327806C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278074: 388BD050  addi r4, r11, -0x2fb0
	ctx.r[4].s64 = ctx.r[11].s64 + -12208;
	// 83278078: 386AD674  addi r3, r10, -0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + -10636;
	// 8327807C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278080: 4AFB4E51  bl 0x8222ced0
	ctx.lr = 0x83278084;
	sub_8222CED0(ctx, base);
	// 83278084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278088: 3869FCE0  addi r3, r9, -0x320
	ctx.r[3].s64 = ctx.r[9].s64 + -800;
	// 8327808C: 4BA31E95  bl 0x82ca9f20
	ctx.lr = 0x83278090;
	sub_82CA9F20(ctx, base);
	// 83278090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327809C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832780A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832780A0 size=64
    let mut pc: u32 = 0x832780A0;
    'dispatch: loop {
        match pc {
            0x832780A0 => {
    //   block [0x832780A0..0x832780E0)
	// 832780A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832780A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832780A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832780AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832780B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832780B4: 388BD088  addi r4, r11, -0x2f78
	ctx.r[4].s64 = ctx.r[11].s64 + -12152;
	// 832780B8: 386AD678  addi r3, r10, -0x2988
	ctx.r[3].s64 = ctx.r[10].s64 + -10632;
	// 832780BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832780C0: 4AFB4E11  bl 0x8222ced0
	ctx.lr = 0x832780C4;
	sub_8222CED0(ctx, base);
	// 832780C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832780C8: 3869FCF0  addi r3, r9, -0x310
	ctx.r[3].s64 = ctx.r[9].s64 + -784;
	// 832780CC: 4BA31E55  bl 0x82ca9f20
	ctx.lr = 0x832780D0;
	sub_82CA9F20(ctx, base);
	// 832780D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832780D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832780D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832780DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832780E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832780E0 size=64
    let mut pc: u32 = 0x832780E0;
    'dispatch: loop {
        match pc {
            0x832780E0 => {
    //   block [0x832780E0..0x83278120)
	// 832780E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832780E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832780E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832780EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832780F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832780F4: 388BD0D0  addi r4, r11, -0x2f30
	ctx.r[4].s64 = ctx.r[11].s64 + -12080;
	// 832780F8: 386AD67C  addi r3, r10, -0x2984
	ctx.r[3].s64 = ctx.r[10].s64 + -10628;
	// 832780FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278100: 4AFB4DD1  bl 0x8222ced0
	ctx.lr = 0x83278104;
	sub_8222CED0(ctx, base);
	// 83278104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278108: 3869FD00  addi r3, r9, -0x300
	ctx.r[3].s64 = ctx.r[9].s64 + -768;
	// 8327810C: 4BA31E15  bl 0x82ca9f20
	ctx.lr = 0x83278110;
	sub_82CA9F20(ctx, base);
	// 83278110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327811C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278120 size=64
    let mut pc: u32 = 0x83278120;
    'dispatch: loop {
        match pc {
            0x83278120 => {
    //   block [0x83278120..0x83278160)
	// 83278120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327812C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278134: 388BD118  addi r4, r11, -0x2ee8
	ctx.r[4].s64 = ctx.r[11].s64 + -12008;
	// 83278138: 386AD680  addi r3, r10, -0x2980
	ctx.r[3].s64 = ctx.r[10].s64 + -10624;
	// 8327813C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278140: 4AFB4D91  bl 0x8222ced0
	ctx.lr = 0x83278144;
	sub_8222CED0(ctx, base);
	// 83278144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278148: 3869FD10  addi r3, r9, -0x2f0
	ctx.r[3].s64 = ctx.r[9].s64 + -752;
	// 8327814C: 4BA31DD5  bl 0x82ca9f20
	ctx.lr = 0x83278150;
	sub_82CA9F20(ctx, base);
	// 83278150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327815C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278160 size=64
    let mut pc: u32 = 0x83278160;
    'dispatch: loop {
        match pc {
            0x83278160 => {
    //   block [0x83278160..0x832781A0)
	// 83278160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327816C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278174: 388BD158  addi r4, r11, -0x2ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -11944;
	// 83278178: 386AD684  addi r3, r10, -0x297c
	ctx.r[3].s64 = ctx.r[10].s64 + -10620;
	// 8327817C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278180: 4AFB4D51  bl 0x8222ced0
	ctx.lr = 0x83278184;
	sub_8222CED0(ctx, base);
	// 83278184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278188: 3869FD20  addi r3, r9, -0x2e0
	ctx.r[3].s64 = ctx.r[9].s64 + -736;
	// 8327818C: 4BA31D95  bl 0x82ca9f20
	ctx.lr = 0x83278190;
	sub_82CA9F20(ctx, base);
	// 83278190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327819C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832781A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832781A0 size=64
    let mut pc: u32 = 0x832781A0;
    'dispatch: loop {
        match pc {
            0x832781A0 => {
    //   block [0x832781A0..0x832781E0)
	// 832781A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832781A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832781A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832781AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832781B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832781B4: 388BD194  addi r4, r11, -0x2e6c
	ctx.r[4].s64 = ctx.r[11].s64 + -11884;
	// 832781B8: 386AD688  addi r3, r10, -0x2978
	ctx.r[3].s64 = ctx.r[10].s64 + -10616;
	// 832781BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832781C0: 4AFB4D11  bl 0x8222ced0
	ctx.lr = 0x832781C4;
	sub_8222CED0(ctx, base);
	// 832781C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832781C8: 3869FD30  addi r3, r9, -0x2d0
	ctx.r[3].s64 = ctx.r[9].s64 + -720;
	// 832781CC: 4BA31D55  bl 0x82ca9f20
	ctx.lr = 0x832781D0;
	sub_82CA9F20(ctx, base);
	// 832781D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832781D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832781D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832781DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832781E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832781E0 size=64
    let mut pc: u32 = 0x832781E0;
    'dispatch: loop {
        match pc {
            0x832781E0 => {
    //   block [0x832781E0..0x83278220)
	// 832781E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832781E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832781E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832781EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832781F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832781F4: 388BD1CC  addi r4, r11, -0x2e34
	ctx.r[4].s64 = ctx.r[11].s64 + -11828;
	// 832781F8: 386AD68C  addi r3, r10, -0x2974
	ctx.r[3].s64 = ctx.r[10].s64 + -10612;
	// 832781FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278200: 4AFB4CD1  bl 0x8222ced0
	ctx.lr = 0x83278204;
	sub_8222CED0(ctx, base);
	// 83278204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278208: 3869FD40  addi r3, r9, -0x2c0
	ctx.r[3].s64 = ctx.r[9].s64 + -704;
	// 8327820C: 4BA31D15  bl 0x82ca9f20
	ctx.lr = 0x83278210;
	sub_82CA9F20(ctx, base);
	// 83278210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327821C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278220 size=64
    let mut pc: u32 = 0x83278220;
    'dispatch: loop {
        match pc {
            0x83278220 => {
    //   block [0x83278220..0x83278260)
	// 83278220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327822C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278234: 388BD204  addi r4, r11, -0x2dfc
	ctx.r[4].s64 = ctx.r[11].s64 + -11772;
	// 83278238: 386AD690  addi r3, r10, -0x2970
	ctx.r[3].s64 = ctx.r[10].s64 + -10608;
	// 8327823C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278240: 4AFB4C91  bl 0x8222ced0
	ctx.lr = 0x83278244;
	sub_8222CED0(ctx, base);
	// 83278244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278248: 3869FD50  addi r3, r9, -0x2b0
	ctx.r[3].s64 = ctx.r[9].s64 + -688;
	// 8327824C: 4BA31CD5  bl 0x82ca9f20
	ctx.lr = 0x83278250;
	sub_82CA9F20(ctx, base);
	// 83278250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327825C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278260 size=64
    let mut pc: u32 = 0x83278260;
    'dispatch: loop {
        match pc {
            0x83278260 => {
    //   block [0x83278260..0x832782A0)
	// 83278260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327826C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278274: 388BD23C  addi r4, r11, -0x2dc4
	ctx.r[4].s64 = ctx.r[11].s64 + -11716;
	// 83278278: 386AD694  addi r3, r10, -0x296c
	ctx.r[3].s64 = ctx.r[10].s64 + -10604;
	// 8327827C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278280: 4AFB4C51  bl 0x8222ced0
	ctx.lr = 0x83278284;
	sub_8222CED0(ctx, base);
	// 83278284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278288: 3869FD60  addi r3, r9, -0x2a0
	ctx.r[3].s64 = ctx.r[9].s64 + -672;
	// 8327828C: 4BA31C95  bl 0x82ca9f20
	ctx.lr = 0x83278290;
	sub_82CA9F20(ctx, base);
	// 83278290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327829C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832782A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832782A0 size=64
    let mut pc: u32 = 0x832782A0;
    'dispatch: loop {
        match pc {
            0x832782A0 => {
    //   block [0x832782A0..0x832782E0)
	// 832782A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832782A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832782A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832782AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832782B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832782B4: 388BD274  addi r4, r11, -0x2d8c
	ctx.r[4].s64 = ctx.r[11].s64 + -11660;
	// 832782B8: 386AD698  addi r3, r10, -0x2968
	ctx.r[3].s64 = ctx.r[10].s64 + -10600;
	// 832782BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832782C0: 4AFB4C11  bl 0x8222ced0
	ctx.lr = 0x832782C4;
	sub_8222CED0(ctx, base);
	// 832782C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832782C8: 3869FD70  addi r3, r9, -0x290
	ctx.r[3].s64 = ctx.r[9].s64 + -656;
	// 832782CC: 4BA31C55  bl 0x82ca9f20
	ctx.lr = 0x832782D0;
	sub_82CA9F20(ctx, base);
	// 832782D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832782D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832782D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832782DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832782E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832782E0 size=64
    let mut pc: u32 = 0x832782E0;
    'dispatch: loop {
        match pc {
            0x832782E0 => {
    //   block [0x832782E0..0x83278320)
	// 832782E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832782E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832782E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832782EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832782F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832782F4: 388BD2B0  addi r4, r11, -0x2d50
	ctx.r[4].s64 = ctx.r[11].s64 + -11600;
	// 832782F8: 386AD69C  addi r3, r10, -0x2964
	ctx.r[3].s64 = ctx.r[10].s64 + -10596;
	// 832782FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278300: 4AFB4BD1  bl 0x8222ced0
	ctx.lr = 0x83278304;
	sub_8222CED0(ctx, base);
	// 83278304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278308: 3869FD80  addi r3, r9, -0x280
	ctx.r[3].s64 = ctx.r[9].s64 + -640;
	// 8327830C: 4BA31C15  bl 0x82ca9f20
	ctx.lr = 0x83278310;
	sub_82CA9F20(ctx, base);
	// 83278310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327831C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278320 size=64
    let mut pc: u32 = 0x83278320;
    'dispatch: loop {
        match pc {
            0x83278320 => {
    //   block [0x83278320..0x83278360)
	// 83278320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327832C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278334: 388BD2EC  addi r4, r11, -0x2d14
	ctx.r[4].s64 = ctx.r[11].s64 + -11540;
	// 83278338: 386AD6A0  addi r3, r10, -0x2960
	ctx.r[3].s64 = ctx.r[10].s64 + -10592;
	// 8327833C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278340: 4AFB4B91  bl 0x8222ced0
	ctx.lr = 0x83278344;
	sub_8222CED0(ctx, base);
	// 83278344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278348: 3869FD90  addi r3, r9, -0x270
	ctx.r[3].s64 = ctx.r[9].s64 + -624;
	// 8327834C: 4BA31BD5  bl 0x82ca9f20
	ctx.lr = 0x83278350;
	sub_82CA9F20(ctx, base);
	// 83278350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327835C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278360 size=64
    let mut pc: u32 = 0x83278360;
    'dispatch: loop {
        match pc {
            0x83278360 => {
    //   block [0x83278360..0x832783A0)
	// 83278360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327836C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278374: 388BD32C  addi r4, r11, -0x2cd4
	ctx.r[4].s64 = ctx.r[11].s64 + -11476;
	// 83278378: 386AD6A4  addi r3, r10, -0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	// 8327837C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278380: 4AFB4B51  bl 0x8222ced0
	ctx.lr = 0x83278384;
	sub_8222CED0(ctx, base);
	// 83278384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278388: 3869FDA0  addi r3, r9, -0x260
	ctx.r[3].s64 = ctx.r[9].s64 + -608;
	// 8327838C: 4BA31B95  bl 0x82ca9f20
	ctx.lr = 0x83278390;
	sub_82CA9F20(ctx, base);
	// 83278390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327839C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832783A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832783A0 size=64
    let mut pc: u32 = 0x832783A0;
    'dispatch: loop {
        match pc {
            0x832783A0 => {
    //   block [0x832783A0..0x832783E0)
	// 832783A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832783A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832783A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832783AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832783B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832783B4: 388BD368  addi r4, r11, -0x2c98
	ctx.r[4].s64 = ctx.r[11].s64 + -11416;
	// 832783B8: 386AD6A8  addi r3, r10, -0x2958
	ctx.r[3].s64 = ctx.r[10].s64 + -10584;
	// 832783BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832783C0: 4AFB4B11  bl 0x8222ced0
	ctx.lr = 0x832783C4;
	sub_8222CED0(ctx, base);
	// 832783C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832783C8: 3869FDB0  addi r3, r9, -0x250
	ctx.r[3].s64 = ctx.r[9].s64 + -592;
	// 832783CC: 4BA31B55  bl 0x82ca9f20
	ctx.lr = 0x832783D0;
	sub_82CA9F20(ctx, base);
	// 832783D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832783D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832783D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832783DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832783E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832783E0 size=64
    let mut pc: u32 = 0x832783E0;
    'dispatch: loop {
        match pc {
            0x832783E0 => {
    //   block [0x832783E0..0x83278420)
	// 832783E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832783E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832783E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832783EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832783F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832783F4: 388BD3A0  addi r4, r11, -0x2c60
	ctx.r[4].s64 = ctx.r[11].s64 + -11360;
	// 832783F8: 386AD6AC  addi r3, r10, -0x2954
	ctx.r[3].s64 = ctx.r[10].s64 + -10580;
	// 832783FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278400: 4AFB4AD1  bl 0x8222ced0
	ctx.lr = 0x83278404;
	sub_8222CED0(ctx, base);
	// 83278404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278408: 3869FDC0  addi r3, r9, -0x240
	ctx.r[3].s64 = ctx.r[9].s64 + -576;
	// 8327840C: 4BA31B15  bl 0x82ca9f20
	ctx.lr = 0x83278410;
	sub_82CA9F20(ctx, base);
	// 83278410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327841C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278420 size=64
    let mut pc: u32 = 0x83278420;
    'dispatch: loop {
        match pc {
            0x83278420 => {
    //   block [0x83278420..0x83278460)
	// 83278420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327842C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278434: 388BD3D8  addi r4, r11, -0x2c28
	ctx.r[4].s64 = ctx.r[11].s64 + -11304;
	// 83278438: 386AD6B0  addi r3, r10, -0x2950
	ctx.r[3].s64 = ctx.r[10].s64 + -10576;
	// 8327843C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278440: 4AFB4A91  bl 0x8222ced0
	ctx.lr = 0x83278444;
	sub_8222CED0(ctx, base);
	// 83278444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278448: 3869FDD0  addi r3, r9, -0x230
	ctx.r[3].s64 = ctx.r[9].s64 + -560;
	// 8327844C: 4BA31AD5  bl 0x82ca9f20
	ctx.lr = 0x83278450;
	sub_82CA9F20(ctx, base);
	// 83278450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327845C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278460 size=64
    let mut pc: u32 = 0x83278460;
    'dispatch: loop {
        match pc {
            0x83278460 => {
    //   block [0x83278460..0x832784A0)
	// 83278460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327846C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278474: 388B9ED8  addi r4, r11, -0x6128
	ctx.r[4].s64 = ctx.r[11].s64 + -24872;
	// 83278478: 386AD6B4  addi r3, r10, -0x294c
	ctx.r[3].s64 = ctx.r[10].s64 + -10572;
	// 8327847C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278480: 4AFB4A51  bl 0x8222ced0
	ctx.lr = 0x83278484;
	sub_8222CED0(ctx, base);
	// 83278484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278488: 3869FDE0  addi r3, r9, -0x220
	ctx.r[3].s64 = ctx.r[9].s64 + -544;
	// 8327848C: 4BA31A95  bl 0x82ca9f20
	ctx.lr = 0x83278490;
	sub_82CA9F20(ctx, base);
	// 83278490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327849C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832784A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832784A0 size=56
    let mut pc: u32 = 0x832784A0;
    'dispatch: loop {
        match pc {
            0x832784A0 => {
    //   block [0x832784A0..0x832784D8)
	// 832784A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832784A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832784A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832784AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832784B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832784B4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832784B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832784BC: 4AF7B89D  bl 0x821f3d58
	ctx.lr = 0x832784C0;
	sub_821F3D58(ctx, base);
	// 832784C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832784C4: 906AD6B8  stw r3, -0x2948(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10568 as u32), ctx.r[3].u32 ) };
	// 832784C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832784CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832784D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832784D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832784D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832784D8 size=56
    let mut pc: u32 = 0x832784D8;
    'dispatch: loop {
        match pc {
            0x832784D8 => {
    //   block [0x832784D8..0x83278510)
	// 832784D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832784DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832784E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832784E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832784E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832784EC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832784F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832784F4: 4AF7B865  bl 0x821f3d58
	ctx.lr = 0x832784F8;
	sub_821F3D58(ctx, base);
	// 832784F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832784FC: 906AD6BC  stw r3, -0x2944(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10564 as u32), ctx.r[3].u32 ) };
	// 83278500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327850C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278510 size=56
    let mut pc: u32 = 0x83278510;
    'dispatch: loop {
        match pc {
            0x83278510 => {
    //   block [0x83278510..0x83278548)
	// 83278510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327851C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278524: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83278528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327852C: 4AF7B82D  bl 0x821f3d58
	ctx.lr = 0x83278530;
	sub_821F3D58(ctx, base);
	// 83278530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278534: 906AD6C0  stw r3, -0x2940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10560 as u32), ctx.r[3].u32 ) };
	// 83278538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327853C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278548 size=56
    let mut pc: u32 = 0x83278548;
    'dispatch: loop {
        match pc {
            0x83278548 => {
    //   block [0x83278548..0x83278580)
	// 83278548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327854C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327855C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83278560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83278564: 4AF7B7F5  bl 0x821f3d58
	ctx.lr = 0x83278568;
	sub_821F3D58(ctx, base);
	// 83278568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327856C: 906AD6C4  stw r3, -0x293c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10556 as u32), ctx.r[3].u32 ) };
	// 83278570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327857C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278580 size=56
    let mut pc: u32 = 0x83278580;
    'dispatch: loop {
        match pc {
            0x83278580 => {
    //   block [0x83278580..0x832785B8)
	// 83278580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327858C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278594: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83278598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327859C: 4AF7B7BD  bl 0x821f3d58
	ctx.lr = 0x832785A0;
	sub_821F3D58(ctx, base);
	// 832785A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832785A4: 906AD6C8  stw r3, -0x2938(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10552 as u32), ctx.r[3].u32 ) };
	// 832785A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832785AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832785B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832785B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832785B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832785B8 size=56
    let mut pc: u32 = 0x832785B8;
    'dispatch: loop {
        match pc {
            0x832785B8 => {
    //   block [0x832785B8..0x832785F0)
	// 832785B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832785BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832785C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832785C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832785C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832785CC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832785D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832785D4: 4AF7B785  bl 0x821f3d58
	ctx.lr = 0x832785D8;
	sub_821F3D58(ctx, base);
	// 832785D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832785DC: 906AD6CC  stw r3, -0x2934(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10548 as u32), ctx.r[3].u32 ) };
	// 832785E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832785E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832785E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832785EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832785F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832785F0 size=56
    let mut pc: u32 = 0x832785F0;
    'dispatch: loop {
        match pc {
            0x832785F0 => {
    //   block [0x832785F0..0x83278628)
	// 832785F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832785F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832785F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832785FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278600: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278604: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83278608: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327860C: 4AF7B74D  bl 0x821f3d58
	ctx.lr = 0x83278610;
	sub_821F3D58(ctx, base);
	// 83278610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278614: 906AD6D0  stw r3, -0x2930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10544 as u32), ctx.r[3].u32 ) };
	// 83278618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327861C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278628 size=56
    let mut pc: u32 = 0x83278628;
    'dispatch: loop {
        match pc {
            0x83278628 => {
    //   block [0x83278628..0x83278660)
	// 83278628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327862C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278634: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278638: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327863C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83278640: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83278644: 4AF7B715  bl 0x821f3d58
	ctx.lr = 0x83278648;
	sub_821F3D58(ctx, base);
	// 83278648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327864C: 906AD6D4  stw r3, -0x292c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10540 as u32), ctx.r[3].u32 ) };
	// 83278650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327865C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278660 size=56
    let mut pc: u32 = 0x83278660;
    'dispatch: loop {
        match pc {
            0x83278660 => {
    //   block [0x83278660..0x83278698)
	// 83278660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327866C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278670: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278674: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83278678: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327867C: 4AF7B6DD  bl 0x821f3d58
	ctx.lr = 0x83278680;
	sub_821F3D58(ctx, base);
	// 83278680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278684: 906AD6D8  stw r3, -0x2928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10536 as u32), ctx.r[3].u32 ) };
	// 83278688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327868C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278698 size=56
    let mut pc: u32 = 0x83278698;
    'dispatch: loop {
        match pc {
            0x83278698 => {
    //   block [0x83278698..0x832786D0)
	// 83278698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327869C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832786A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832786A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832786A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832786AC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832786B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832786B4: 4AF7B6A5  bl 0x821f3d58
	ctx.lr = 0x832786B8;
	sub_821F3D58(ctx, base);
	// 832786B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832786BC: 906AD6DC  stw r3, -0x2924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10532 as u32), ctx.r[3].u32 ) };
	// 832786C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832786C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832786C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832786CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832786D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832786D0 size=56
    let mut pc: u32 = 0x832786D0;
    'dispatch: loop {
        match pc {
            0x832786D0 => {
    //   block [0x832786D0..0x83278708)
	// 832786D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832786D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832786D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832786DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832786E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832786E4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832786E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832786EC: 4AF7B66D  bl 0x821f3d58
	ctx.lr = 0x832786F0;
	sub_821F3D58(ctx, base);
	// 832786F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832786F4: 906AD6E0  stw r3, -0x2920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10528 as u32), ctx.r[3].u32 ) };
	// 832786F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832786FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278708 size=56
    let mut pc: u32 = 0x83278708;
    'dispatch: loop {
        match pc {
            0x83278708 => {
    //   block [0x83278708..0x83278740)
	// 83278708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327870C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278714: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278718: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327871C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83278720: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83278724: 4AF7B635  bl 0x821f3d58
	ctx.lr = 0x83278728;
	sub_821F3D58(ctx, base);
	// 83278728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327872C: 906AD6E4  stw r3, -0x291c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10524 as u32), ctx.r[3].u32 ) };
	// 83278730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327873C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278740 size=56
    let mut pc: u32 = 0x83278740;
    'dispatch: loop {
        match pc {
            0x83278740 => {
    //   block [0x83278740..0x83278778)
	// 83278740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327874C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278750: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278754: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83278758: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327875C: 4AF7B5FD  bl 0x821f3d58
	ctx.lr = 0x83278760;
	sub_821F3D58(ctx, base);
	// 83278760: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278764: 906AD6E8  stw r3, -0x2918(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10520 as u32), ctx.r[3].u32 ) };
	// 83278768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327876C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278778 size=56
    let mut pc: u32 = 0x83278778;
    'dispatch: loop {
        match pc {
            0x83278778 => {
    //   block [0x83278778..0x832787B0)
	// 83278778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327877C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278784: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278788: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327878C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83278790: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83278794: 4AF7B5C5  bl 0x821f3d58
	ctx.lr = 0x83278798;
	sub_821F3D58(ctx, base);
	// 83278798: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327879C: 906AD6EC  stw r3, -0x2914(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10516 as u32), ctx.r[3].u32 ) };
	// 832787A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832787A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832787A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832787AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832787B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832787B0 size=56
    let mut pc: u32 = 0x832787B0;
    'dispatch: loop {
        match pc {
            0x832787B0 => {
    //   block [0x832787B0..0x832787E8)
	// 832787B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832787B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832787B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832787BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832787C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832787C4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832787C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832787CC: 4AF7B58D  bl 0x821f3d58
	ctx.lr = 0x832787D0;
	sub_821F3D58(ctx, base);
	// 832787D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832787D4: 906AD6F0  stw r3, -0x2910(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10512 as u32), ctx.r[3].u32 ) };
	// 832787D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832787DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832787E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832787E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832787E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832787E8 size=56
    let mut pc: u32 = 0x832787E8;
    'dispatch: loop {
        match pc {
            0x832787E8 => {
    //   block [0x832787E8..0x83278820)
	// 832787E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832787EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832787F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832787F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832787F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832787FC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83278800: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83278804: 4AF7B555  bl 0x821f3d58
	ctx.lr = 0x83278808;
	sub_821F3D58(ctx, base);
	// 83278808: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327880C: 906AD6F4  stw r3, -0x290c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10508 as u32), ctx.r[3].u32 ) };
	// 83278810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327881C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278820 size=56
    let mut pc: u32 = 0x83278820;
    'dispatch: loop {
        match pc {
            0x83278820 => {
    //   block [0x83278820..0x83278858)
	// 83278820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327882C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278830: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278834: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83278838: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327883C: 4AF7B51D  bl 0x821f3d58
	ctx.lr = 0x83278840;
	sub_821F3D58(ctx, base);
	// 83278840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278844: 906AD6F8  stw r3, -0x2908(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10504 as u32), ctx.r[3].u32 ) };
	// 83278848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327884C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278858 size=56
    let mut pc: u32 = 0x83278858;
    'dispatch: loop {
        match pc {
            0x83278858 => {
    //   block [0x83278858..0x83278890)
	// 83278858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327885C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278864: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278868: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327886C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83278870: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83278874: 4AF7B4E5  bl 0x821f3d58
	ctx.lr = 0x83278878;
	sub_821F3D58(ctx, base);
	// 83278878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327887C: 906AD6FC  stw r3, -0x2904(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10500 as u32), ctx.r[3].u32 ) };
	// 83278880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327888C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278890 size=56
    let mut pc: u32 = 0x83278890;
    'dispatch: loop {
        match pc {
            0x83278890 => {
    //   block [0x83278890..0x832788C8)
	// 83278890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327889C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832788A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832788A4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832788A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832788AC: 4AF7B4AD  bl 0x821f3d58
	ctx.lr = 0x832788B0;
	sub_821F3D58(ctx, base);
	// 832788B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832788B4: 906AD700  stw r3, -0x2900(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10496 as u32), ctx.r[3].u32 ) };
	// 832788B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832788BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832788C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832788C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832788C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832788C8 size=56
    let mut pc: u32 = 0x832788C8;
    'dispatch: loop {
        match pc {
            0x832788C8 => {
    //   block [0x832788C8..0x83278900)
	// 832788C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832788CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832788D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832788D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832788D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832788DC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832788E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832788E4: 4AF7B475  bl 0x821f3d58
	ctx.lr = 0x832788E8;
	sub_821F3D58(ctx, base);
	// 832788E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832788EC: 906AD704  stw r3, -0x28fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10492 as u32), ctx.r[3].u32 ) };
	// 832788F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832788F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832788F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832788FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278900 size=56
    let mut pc: u32 = 0x83278900;
    'dispatch: loop {
        match pc {
            0x83278900 => {
    //   block [0x83278900..0x83278938)
	// 83278900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327890C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83278910: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83278914: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83278918: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327891C: 4AF7B43D  bl 0x821f3d58
	ctx.lr = 0x83278920;
	sub_821F3D58(ctx, base);
	// 83278920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278924: 906AD708  stw r3, -0x28f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10488 as u32), ctx.r[3].u32 ) };
	// 83278928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327892C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278938 size=64
    let mut pc: u32 = 0x83278938;
    'dispatch: loop {
        match pc {
            0x83278938 => {
    //   block [0x83278938..0x83278978)
	// 83278938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327893C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278944: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327894C: 388BD4A8  addi r4, r11, -0x2b58
	ctx.r[4].s64 = ctx.r[11].s64 + -11096;
	// 83278950: 386AD70C  addi r3, r10, -0x28f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10484;
	// 83278954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278958: 4AFB4579  bl 0x8222ced0
	ctx.lr = 0x8327895C;
	sub_8222CED0(ctx, base);
	// 8327895C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278960: 3869FDF0  addi r3, r9, -0x210
	ctx.r[3].s64 = ctx.r[9].s64 + -528;
	// 83278964: 4BA315BD  bl 0x82ca9f20
	ctx.lr = 0x83278968;
	sub_82CA9F20(ctx, base);
	// 83278968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327896C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278978 size=64
    let mut pc: u32 = 0x83278978;
    'dispatch: loop {
        match pc {
            0x83278978 => {
    //   block [0x83278978..0x832789B8)
	// 83278978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327897C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278984: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327898C: 388BD4D8  addi r4, r11, -0x2b28
	ctx.r[4].s64 = ctx.r[11].s64 + -11048;
	// 83278990: 386AD710  addi r3, r10, -0x28f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10480;
	// 83278994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278998: 4AFB4539  bl 0x8222ced0
	ctx.lr = 0x8327899C;
	sub_8222CED0(ctx, base);
	// 8327899C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832789A0: 3869FE00  addi r3, r9, -0x200
	ctx.r[3].s64 = ctx.r[9].s64 + -512;
	// 832789A4: 4BA3157D  bl 0x82ca9f20
	ctx.lr = 0x832789A8;
	sub_82CA9F20(ctx, base);
	// 832789A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832789AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832789B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832789B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832789B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832789B8 size=64
    let mut pc: u32 = 0x832789B8;
    'dispatch: loop {
        match pc {
            0x832789B8 => {
    //   block [0x832789B8..0x832789F8)
	// 832789B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832789BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832789C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832789C4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832789C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832789CC: 388BD500  addi r4, r11, -0x2b00
	ctx.r[4].s64 = ctx.r[11].s64 + -11008;
	// 832789D0: 386AD714  addi r3, r10, -0x28ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10476;
	// 832789D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832789D8: 4AFB44F9  bl 0x8222ced0
	ctx.lr = 0x832789DC;
	sub_8222CED0(ctx, base);
	// 832789DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832789E0: 3869FE10  addi r3, r9, -0x1f0
	ctx.r[3].s64 = ctx.r[9].s64 + -496;
	// 832789E4: 4BA3153D  bl 0x82ca9f20
	ctx.lr = 0x832789E8;
	sub_82CA9F20(ctx, base);
	// 832789E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832789EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832789F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832789F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832789F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832789F8 size=64
    let mut pc: u32 = 0x832789F8;
    'dispatch: loop {
        match pc {
            0x832789F8 => {
    //   block [0x832789F8..0x83278A38)
	// 832789F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832789FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278A04: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278A0C: 388BD534  addi r4, r11, -0x2acc
	ctx.r[4].s64 = ctx.r[11].s64 + -10956;
	// 83278A10: 386AD718  addi r3, r10, -0x28e8
	ctx.r[3].s64 = ctx.r[10].s64 + -10472;
	// 83278A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278A18: 4AFB44B9  bl 0x8222ced0
	ctx.lr = 0x83278A1C;
	sub_8222CED0(ctx, base);
	// 83278A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278A20: 3869FE20  addi r3, r9, -0x1e0
	ctx.r[3].s64 = ctx.r[9].s64 + -480;
	// 83278A24: 4BA314FD  bl 0x82ca9f20
	ctx.lr = 0x83278A28;
	sub_82CA9F20(ctx, base);
	// 83278A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278A38 size=64
    let mut pc: u32 = 0x83278A38;
    'dispatch: loop {
        match pc {
            0x83278A38 => {
    //   block [0x83278A38..0x83278A78)
	// 83278A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278A44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83278A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278A4C: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83278A50: 386AD71C  addi r3, r10, -0x28e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10468;
	// 83278A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278A58: 4AFB4479  bl 0x8222ced0
	ctx.lr = 0x83278A5C;
	sub_8222CED0(ctx, base);
	// 83278A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83278A60: 3869FE30  addi r3, r9, -0x1d0
	ctx.r[3].s64 = ctx.r[9].s64 + -464;
	// 83278A64: 4BA314BD  bl 0x82ca9f20
	ctx.lr = 0x83278A68;
	sub_82CA9F20(ctx, base);
	// 83278A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83278A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278A78 size=192
    let mut pc: u32 = 0x83278A78;
    'dispatch: loop {
        match pc {
            0x83278A78 => {
    //   block [0x83278A78..0x83278B38)
	// 83278A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278A84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278A88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278A90: 388BD244  addi r4, r11, -0x2dbc
	ctx.r[4].s64 = ctx.r[11].s64 + -11708;
	// 83278A94: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278A98: 4AFB4439  bl 0x8222ced0
	ctx.lr = 0x83278A9C;
	sub_8222CED0(ctx, base);
	// 83278A9C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278AA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278AA4: 4AF76095  bl 0x821eeb38
	ctx.lr = 0x83278AA8;
	sub_821EEB38(ctx, base);
	// 83278AA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278AAC: 4B98AD45  bl 0x82c037f0
	ctx.lr = 0x83278AB0;
	sub_82C037F0(ctx, base);
	// 83278AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278AB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278AB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278ABC: 916AD720  stw r11, -0x28e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10464 as u32), ctx.r[11].u32 ) };
	// 83278AC0: 4AF4DCA9  bl 0x821c6768
	ctx.lr = 0x83278AC4;
	sub_821C6768(ctx, base);
	// 83278AC4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278AC8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278ACC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278AD0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278AD4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278AD8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278ADC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278AE0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278AE4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278AE8: 4082FFE8  bne 0x83278ad0
	if !ctx.cr[0].eq {
	pc = 0x83278AD0; continue 'dispatch;
	}
	// 83278AEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278AF0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278AF4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278AF8: 4AF4DC71  bl 0x821c6768
	ctx.lr = 0x83278AFC;
	sub_821C6768(ctx, base);
	// 83278AFC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278B00: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278B04: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278B08: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278B0C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278B10: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278B14: 4082FFE8  bne 0x83278afc
	if !ctx.cr[0].eq {
	pc = 0x83278AFC; continue 'dispatch;
	}
	// 83278B18: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278B1C: 386BFE40  addi r3, r11, -0x1c0
	ctx.r[3].s64 = ctx.r[11].s64 + -448;
	// 83278B20: 4BA31401  bl 0x82ca9f20
	ctx.lr = 0x83278B24;
	sub_82CA9F20(ctx, base);
	// 83278B24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278B38 size=192
    let mut pc: u32 = 0x83278B38;
    'dispatch: loop {
        match pc {
            0x83278B38 => {
    //   block [0x83278B38..0x83278BF8)
	// 83278B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278B40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278B44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278B48: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278B50: 388BD270  addi r4, r11, -0x2d90
	ctx.r[4].s64 = ctx.r[11].s64 + -11664;
	// 83278B54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278B58: 4AFB4379  bl 0x8222ced0
	ctx.lr = 0x83278B5C;
	sub_8222CED0(ctx, base);
	// 83278B5C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278B60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278B64: 4AF75FD5  bl 0x821eeb38
	ctx.lr = 0x83278B68;
	sub_821EEB38(ctx, base);
	// 83278B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278B6C: 4B98AC85  bl 0x82c037f0
	ctx.lr = 0x83278B70;
	sub_82C037F0(ctx, base);
	// 83278B70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278B74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278B78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278B7C: 916AD724  stw r11, -0x28dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10460 as u32), ctx.r[11].u32 ) };
	// 83278B80: 4AF4DBE9  bl 0x821c6768
	ctx.lr = 0x83278B84;
	sub_821C6768(ctx, base);
	// 83278B84: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278B88: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278B8C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278B90: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278B94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278B98: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278B9C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278BA0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278BA4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278BA8: 4082FFE8  bne 0x83278b90
	if !ctx.cr[0].eq {
	pc = 0x83278B90; continue 'dispatch;
	}
	// 83278BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278BB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278BB4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278BB8: 4AF4DBB1  bl 0x821c6768
	ctx.lr = 0x83278BBC;
	sub_821C6768(ctx, base);
	// 83278BBC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278BC0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278BC4: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278BC8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278BCC: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278BD0: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278BD4: 4082FFE8  bne 0x83278bbc
	if !ctx.cr[0].eq {
	pc = 0x83278BBC; continue 'dispatch;
	}
	// 83278BD8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278BDC: 386BFE48  addi r3, r11, -0x1b8
	ctx.r[3].s64 = ctx.r[11].s64 + -440;
	// 83278BE0: 4BA31341  bl 0x82ca9f20
	ctx.lr = 0x83278BE4;
	sub_82CA9F20(ctx, base);
	// 83278BE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278BF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278BF8 size=192
    let mut pc: u32 = 0x83278BF8;
    'dispatch: loop {
        match pc {
            0x83278BF8 => {
    //   block [0x83278BF8..0x83278CB8)
	// 83278BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278C04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278C08: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278C10: 388BD29C  addi r4, r11, -0x2d64
	ctx.r[4].s64 = ctx.r[11].s64 + -11620;
	// 83278C14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278C18: 4AFB42B9  bl 0x8222ced0
	ctx.lr = 0x83278C1C;
	sub_8222CED0(ctx, base);
	// 83278C1C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278C24: 4AF75F15  bl 0x821eeb38
	ctx.lr = 0x83278C28;
	sub_821EEB38(ctx, base);
	// 83278C28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278C2C: 4B98ABC5  bl 0x82c037f0
	ctx.lr = 0x83278C30;
	sub_82C037F0(ctx, base);
	// 83278C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278C34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278C38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278C3C: 916AD728  stw r11, -0x28d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10456 as u32), ctx.r[11].u32 ) };
	// 83278C40: 4AF4DB29  bl 0x821c6768
	ctx.lr = 0x83278C44;
	sub_821C6768(ctx, base);
	// 83278C44: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278C48: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278C4C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278C50: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278C54: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278C58: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278C5C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278C60: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278C64: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278C68: 4082FFE8  bne 0x83278c50
	if !ctx.cr[0].eq {
	pc = 0x83278C50; continue 'dispatch;
	}
	// 83278C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278C70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278C74: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278C78: 4AF4DAF1  bl 0x821c6768
	ctx.lr = 0x83278C7C;
	sub_821C6768(ctx, base);
	// 83278C7C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278C80: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278C84: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278C88: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278C8C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278C90: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278C94: 4082FFE8  bne 0x83278c7c
	if !ctx.cr[0].eq {
	pc = 0x83278C7C; continue 'dispatch;
	}
	// 83278C98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278C9C: 386BFE50  addi r3, r11, -0x1b0
	ctx.r[3].s64 = ctx.r[11].s64 + -432;
	// 83278CA0: 4BA31281  bl 0x82ca9f20
	ctx.lr = 0x83278CA4;
	sub_82CA9F20(ctx, base);
	// 83278CA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278CB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278CB8 size=192
    let mut pc: u32 = 0x83278CB8;
    'dispatch: loop {
        match pc {
            0x83278CB8 => {
    //   block [0x83278CB8..0x83278D78)
	// 83278CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278CC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278CC8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278CD0: 388BD2C4  addi r4, r11, -0x2d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -11580;
	// 83278CD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278CD8: 4AFB41F9  bl 0x8222ced0
	ctx.lr = 0x83278CDC;
	sub_8222CED0(ctx, base);
	// 83278CDC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278CE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278CE4: 4AF75E55  bl 0x821eeb38
	ctx.lr = 0x83278CE8;
	sub_821EEB38(ctx, base);
	// 83278CE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278CEC: 4B98AB05  bl 0x82c037f0
	ctx.lr = 0x83278CF0;
	sub_82C037F0(ctx, base);
	// 83278CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278CF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278CFC: 916AD72C  stw r11, -0x28d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10452 as u32), ctx.r[11].u32 ) };
	// 83278D00: 4AF4DA69  bl 0x821c6768
	ctx.lr = 0x83278D04;
	sub_821C6768(ctx, base);
	// 83278D04: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278D08: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278D0C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278D10: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278D14: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278D18: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278D1C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278D20: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278D24: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278D28: 4082FFE8  bne 0x83278d10
	if !ctx.cr[0].eq {
	pc = 0x83278D10; continue 'dispatch;
	}
	// 83278D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278D30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278D34: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278D38: 4AF4DA31  bl 0x821c6768
	ctx.lr = 0x83278D3C;
	sub_821C6768(ctx, base);
	// 83278D3C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278D40: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278D44: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278D48: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278D4C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278D50: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278D54: 4082FFE8  bne 0x83278d3c
	if !ctx.cr[0].eq {
	pc = 0x83278D3C; continue 'dispatch;
	}
	// 83278D58: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278D5C: 386BFE58  addi r3, r11, -0x1a8
	ctx.r[3].s64 = ctx.r[11].s64 + -424;
	// 83278D60: 4BA311C1  bl 0x82ca9f20
	ctx.lr = 0x83278D64;
	sub_82CA9F20(ctx, base);
	// 83278D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278D70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278D78 size=192
    let mut pc: u32 = 0x83278D78;
    'dispatch: loop {
        match pc {
            0x83278D78 => {
    //   block [0x83278D78..0x83278E38)
	// 83278D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278D80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278D84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278D88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278D90: 388BD2F0  addi r4, r11, -0x2d10
	ctx.r[4].s64 = ctx.r[11].s64 + -11536;
	// 83278D94: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278D98: 4AFB4139  bl 0x8222ced0
	ctx.lr = 0x83278D9C;
	sub_8222CED0(ctx, base);
	// 83278D9C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278DA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278DA4: 4AF75D95  bl 0x821eeb38
	ctx.lr = 0x83278DA8;
	sub_821EEB38(ctx, base);
	// 83278DA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278DAC: 4B98AA45  bl 0x82c037f0
	ctx.lr = 0x83278DB0;
	sub_82C037F0(ctx, base);
	// 83278DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278DB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278DB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278DBC: 916AD730  stw r11, -0x28d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10448 as u32), ctx.r[11].u32 ) };
	// 83278DC0: 4AF4D9A9  bl 0x821c6768
	ctx.lr = 0x83278DC4;
	sub_821C6768(ctx, base);
	// 83278DC4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278DC8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278DCC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278DD0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278DD4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278DD8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278DDC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278DE0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278DE4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278DE8: 4082FFE8  bne 0x83278dd0
	if !ctx.cr[0].eq {
	pc = 0x83278DD0; continue 'dispatch;
	}
	// 83278DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278DF0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278DF4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278DF8: 4AF4D971  bl 0x821c6768
	ctx.lr = 0x83278DFC;
	sub_821C6768(ctx, base);
	// 83278DFC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278E00: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278E04: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278E08: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278E0C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278E10: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278E14: 4082FFE8  bne 0x83278dfc
	if !ctx.cr[0].eq {
	pc = 0x83278DFC; continue 'dispatch;
	}
	// 83278E18: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278E1C: 386BFE60  addi r3, r11, -0x1a0
	ctx.r[3].s64 = ctx.r[11].s64 + -416;
	// 83278E20: 4BA31101  bl 0x82ca9f20
	ctx.lr = 0x83278E24;
	sub_82CA9F20(ctx, base);
	// 83278E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278E38 size=192
    let mut pc: u32 = 0x83278E38;
    'dispatch: loop {
        match pc {
            0x83278E38 => {
    //   block [0x83278E38..0x83278EF8)
	// 83278E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278E44: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278E48: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278E50: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 83278E54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278E58: 4AFB4079  bl 0x8222ced0
	ctx.lr = 0x83278E5C;
	sub_8222CED0(ctx, base);
	// 83278E5C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278E60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278E64: 4AF75CD5  bl 0x821eeb38
	ctx.lr = 0x83278E68;
	sub_821EEB38(ctx, base);
	// 83278E68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278E6C: 4B98A985  bl 0x82c037f0
	ctx.lr = 0x83278E70;
	sub_82C037F0(ctx, base);
	// 83278E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278E74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278E78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278E7C: 916AD734  stw r11, -0x28cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10444 as u32), ctx.r[11].u32 ) };
	// 83278E80: 4AF4D8E9  bl 0x821c6768
	ctx.lr = 0x83278E84;
	sub_821C6768(ctx, base);
	// 83278E84: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278E88: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278E8C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278E90: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278E94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278E98: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278E9C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278EA0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278EA4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278EA8: 4082FFE8  bne 0x83278e90
	if !ctx.cr[0].eq {
	pc = 0x83278E90; continue 'dispatch;
	}
	// 83278EAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278EB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278EB4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278EB8: 4AF4D8B1  bl 0x821c6768
	ctx.lr = 0x83278EBC;
	sub_821C6768(ctx, base);
	// 83278EBC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278EC0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278EC4: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278EC8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278ECC: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278ED0: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278ED4: 4082FFE8  bne 0x83278ebc
	if !ctx.cr[0].eq {
	pc = 0x83278EBC; continue 'dispatch;
	}
	// 83278ED8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278EDC: 386BFE68  addi r3, r11, -0x198
	ctx.r[3].s64 = ctx.r[11].s64 + -408;
	// 83278EE0: 4BA31041  bl 0x82ca9f20
	ctx.lr = 0x83278EE4;
	sub_82CA9F20(ctx, base);
	// 83278EE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278EF8 size=192
    let mut pc: u32 = 0x83278EF8;
    'dispatch: loop {
        match pc {
            0x83278EF8 => {
    //   block [0x83278EF8..0x83278FB8)
	// 83278EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278F00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278F04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278F08: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278F10: 388BD344  addi r4, r11, -0x2cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -11452;
	// 83278F14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278F18: 4AFB3FB9  bl 0x8222ced0
	ctx.lr = 0x83278F1C;
	sub_8222CED0(ctx, base);
	// 83278F1C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278F24: 4AF75C15  bl 0x821eeb38
	ctx.lr = 0x83278F28;
	sub_821EEB38(ctx, base);
	// 83278F28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278F2C: 4B98A8C5  bl 0x82c037f0
	ctx.lr = 0x83278F30;
	sub_82C037F0(ctx, base);
	// 83278F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278F34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278F38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278F3C: 916AD738  stw r11, -0x28c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10440 as u32), ctx.r[11].u32 ) };
	// 83278F40: 4AF4D829  bl 0x821c6768
	ctx.lr = 0x83278F44;
	sub_821C6768(ctx, base);
	// 83278F44: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83278F48: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83278F4C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83278F50: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83278F54: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278F58: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83278F5C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83278F60: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278F64: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278F68: 4082FFE8  bne 0x83278f50
	if !ctx.cr[0].eq {
	pc = 0x83278F50; continue 'dispatch;
	}
	// 83278F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83278F70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278F74: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83278F78: 4AF4D7F1  bl 0x821c6768
	ctx.lr = 0x83278F7C;
	sub_821C6768(ctx, base);
	// 83278F7C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83278F80: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278F84: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83278F88: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83278F8C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83278F90: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83278F94: 4082FFE8  bne 0x83278f7c
	if !ctx.cr[0].eq {
	pc = 0x83278F7C; continue 'dispatch;
	}
	// 83278F98: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83278F9C: 386BFE70  addi r3, r11, -0x190
	ctx.r[3].s64 = ctx.r[11].s64 + -400;
	// 83278FA0: 4BA30F81  bl 0x82ca9f20
	ctx.lr = 0x83278FA4;
	sub_82CA9F20(ctx, base);
	// 83278FA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83278FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83278FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83278FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83278FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83278FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83278FB8 size=192
    let mut pc: u32 = 0x83278FB8;
    'dispatch: loop {
        match pc {
            0x83278FB8 => {
    //   block [0x83278FB8..0x83279078)
	// 83278FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83278FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83278FC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83278FC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83278FC8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83278FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83278FD0: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 83278FD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83278FD8: 4AFB3EF9  bl 0x8222ced0
	ctx.lr = 0x83278FDC;
	sub_8222CED0(ctx, base);
	// 83278FDC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83278FE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278FE4: 4AF75B55  bl 0x821eeb38
	ctx.lr = 0x83278FE8;
	sub_821EEB38(ctx, base);
	// 83278FE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278FEC: 4B98A805  bl 0x82c037f0
	ctx.lr = 0x83278FF0;
	sub_82C037F0(ctx, base);
	// 83278FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83278FF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83278FF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83278FFC: 916AD73C  stw r11, -0x28c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10436 as u32), ctx.r[11].u32 ) };
	// 83279000: 4AF4D769  bl 0x821c6768
	ctx.lr = 0x83279004;
	sub_821C6768(ctx, base);
	// 83279004: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83279008: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8327900C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83279010: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83279014: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279018: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8327901C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83279020: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83279024: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279028: 4082FFE8  bne 0x83279010
	if !ctx.cr[0].eq {
	pc = 0x83279010; continue 'dispatch;
	}
	// 8327902C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83279030: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83279034: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83279038: 4AF4D731  bl 0x821c6768
	ctx.lr = 0x8327903C;
	sub_821C6768(ctx, base);
	// 8327903C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83279040: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279044: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83279048: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8327904C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83279050: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279054: 4082FFE8  bne 0x8327903c
	if !ctx.cr[0].eq {
	pc = 0x8327903C; continue 'dispatch;
	}
	// 83279058: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8327905C: 386BFE78  addi r3, r11, -0x188
	ctx.r[3].s64 = ctx.r[11].s64 + -392;
	// 83279060: 4BA30EC1  bl 0x82ca9f20
	ctx.lr = 0x83279064;
	sub_82CA9F20(ctx, base);
	// 83279064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83279068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327906C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83279074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279078 size=192
    let mut pc: u32 = 0x83279078;
    'dispatch: loop {
        match pc {
            0x83279078 => {
    //   block [0x83279078..0x83279138)
	// 83279078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83279084: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279088: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327908C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279090: 388BD370  addi r4, r11, -0x2c90
	ctx.r[4].s64 = ctx.r[11].s64 + -11408;
	// 83279094: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83279098: 4AFB3E39  bl 0x8222ced0
	ctx.lr = 0x8327909C;
	sub_8222CED0(ctx, base);
	// 8327909C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832790A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832790A4: 4AF75A95  bl 0x821eeb38
	ctx.lr = 0x832790A8;
	sub_821EEB38(ctx, base);
	// 832790A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832790AC: 4B98A745  bl 0x82c037f0
	ctx.lr = 0x832790B0;
	sub_82C037F0(ctx, base);
	// 832790B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832790B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832790B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832790BC: 916AD740  stw r11, -0x28c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10432 as u32), ctx.r[11].u32 ) };
	// 832790C0: 4AF4D6A9  bl 0x821c6768
	ctx.lr = 0x832790C4;
	sub_821C6768(ctx, base);
	// 832790C4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832790C8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832790CC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 832790D0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832790D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832790D8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832790DC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832790E0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832790E4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832790E8: 4082FFE8  bne 0x832790d0
	if !ctx.cr[0].eq {
	pc = 0x832790D0; continue 'dispatch;
	}
	// 832790EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832790F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832790F4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832790F8: 4AF4D671  bl 0x821c6768
	ctx.lr = 0x832790FC;
	sub_821C6768(ctx, base);
	// 832790FC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83279100: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279104: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83279108: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8327910C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83279110: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279114: 4082FFE8  bne 0x832790fc
	if !ctx.cr[0].eq {
	pc = 0x832790FC; continue 'dispatch;
	}
	// 83279118: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8327911C: 386BFE80  addi r3, r11, -0x180
	ctx.r[3].s64 = ctx.r[11].s64 + -384;
	// 83279120: 4BA30E01  bl 0x82ca9f20
	ctx.lr = 0x83279124;
	sub_82CA9F20(ctx, base);
	// 83279124: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83279128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327912C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83279134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279138 size=192
    let mut pc: u32 = 0x83279138;
    'dispatch: loop {
        match pc {
            0x83279138 => {
    //   block [0x83279138..0x832791F8)
	// 83279138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327913C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83279144: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279148: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327914C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279150: 388BD39C  addi r4, r11, -0x2c64
	ctx.r[4].s64 = ctx.r[11].s64 + -11364;
	// 83279154: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83279158: 4AFB3D79  bl 0x8222ced0
	ctx.lr = 0x8327915C;
	sub_8222CED0(ctx, base);
	// 8327915C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83279160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83279164: 4AF759D5  bl 0x821eeb38
	ctx.lr = 0x83279168;
	sub_821EEB38(ctx, base);
	// 83279168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8327916C: 4B98A685  bl 0x82c037f0
	ctx.lr = 0x83279170;
	sub_82C037F0(ctx, base);
	// 83279170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279174: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83279178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8327917C: 916AD744  stw r11, -0x28bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10428 as u32), ctx.r[11].u32 ) };
	// 83279180: 4AF4D5E9  bl 0x821c6768
	ctx.lr = 0x83279184;
	sub_821C6768(ctx, base);
	// 83279184: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83279188: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8327918C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83279190: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83279194: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279198: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8327919C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832791A0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832791A4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832791A8: 4082FFE8  bne 0x83279190
	if !ctx.cr[0].eq {
	pc = 0x83279190; continue 'dispatch;
	}
	// 832791AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832791B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832791B4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832791B8: 4AF4D5B1  bl 0x821c6768
	ctx.lr = 0x832791BC;
	sub_821C6768(ctx, base);
	// 832791BC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 832791C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832791C4: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832791C8: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832791CC: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832791D0: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832791D4: 4082FFE8  bne 0x832791bc
	if !ctx.cr[0].eq {
	pc = 0x832791BC; continue 'dispatch;
	}
	// 832791D8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832791DC: 386BFE88  addi r3, r11, -0x178
	ctx.r[3].s64 = ctx.r[11].s64 + -376;
	// 832791E0: 4BA30D41  bl 0x82ca9f20
	ctx.lr = 0x832791E4;
	sub_82CA9F20(ctx, base);
	// 832791E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832791E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832791EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832791F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832791F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832791F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832791F8 size=192
    let mut pc: u32 = 0x832791F8;
    'dispatch: loop {
        match pc {
            0x832791F8 => {
    //   block [0x832791F8..0x832792B8)
	// 832791F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832791FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83279204: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279208: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327920C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279210: 388BD3C8  addi r4, r11, -0x2c38
	ctx.r[4].s64 = ctx.r[11].s64 + -11320;
	// 83279214: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83279218: 4AFB3CB9  bl 0x8222ced0
	ctx.lr = 0x8327921C;
	sub_8222CED0(ctx, base);
	// 8327921C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83279220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83279224: 4AF75915  bl 0x821eeb38
	ctx.lr = 0x83279228;
	sub_821EEB38(ctx, base);
	// 83279228: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8327922C: 4B98A5C5  bl 0x82c037f0
	ctx.lr = 0x83279230;
	sub_82C037F0(ctx, base);
	// 83279230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279234: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83279238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8327923C: 916AD748  stw r11, -0x28b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10424 as u32), ctx.r[11].u32 ) };
	// 83279240: 4AF4D529  bl 0x821c6768
	ctx.lr = 0x83279244;
	sub_821C6768(ctx, base);
	// 83279244: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83279248: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8327924C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83279250: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83279254: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279258: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8327925C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83279260: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83279264: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279268: 4082FFE8  bne 0x83279250
	if !ctx.cr[0].eq {
	pc = 0x83279250; continue 'dispatch;
	}
	// 8327926C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83279270: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83279274: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83279278: 4AF4D4F1  bl 0x821c6768
	ctx.lr = 0x8327927C;
	sub_821C6768(ctx, base);
	// 8327927C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83279280: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279284: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83279288: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8327928C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83279290: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83279294: 4082FFE8  bne 0x8327927c
	if !ctx.cr[0].eq {
	pc = 0x8327927C; continue 'dispatch;
	}
	// 83279298: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8327929C: 386BFE90  addi r3, r11, -0x170
	ctx.r[3].s64 = ctx.r[11].s64 + -368;
	// 832792A0: 4BA30C81  bl 0x82ca9f20
	ctx.lr = 0x832792A4;
	sub_82CA9F20(ctx, base);
	// 832792A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832792A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832792AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832792B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832792B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832792B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832792B8 size=56
    let mut pc: u32 = 0x832792B8;
    'dispatch: loop {
        match pc {
            0x832792B8 => {
    //   block [0x832792B8..0x832792F0)
	// 832792B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832792BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832792C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832792C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832792C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832792CC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832792D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832792D4: 4AF7AA85  bl 0x821f3d58
	ctx.lr = 0x832792D8;
	sub_821F3D58(ctx, base);
	// 832792D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832792DC: 906AD74C  stw r3, -0x28b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10420 as u32), ctx.r[3].u32 ) };
	// 832792E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832792E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832792E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832792EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832792F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832792F0 size=56
    let mut pc: u32 = 0x832792F0;
    'dispatch: loop {
        match pc {
            0x832792F0 => {
    //   block [0x832792F0..0x83279328)
	// 832792F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832792F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832792F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832792FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279300: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279304: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83279308: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327930C: 4AF7AA4D  bl 0x821f3d58
	ctx.lr = 0x83279310;
	sub_821F3D58(ctx, base);
	// 83279310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279314: 906AD750  stw r3, -0x28b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10416 as u32), ctx.r[3].u32 ) };
	// 83279318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327931C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279328 size=56
    let mut pc: u32 = 0x83279328;
    'dispatch: loop {
        match pc {
            0x83279328 => {
    //   block [0x83279328..0x83279360)
	// 83279328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327932C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279334: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327933C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83279340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279344: 4AF7AA15  bl 0x821f3d58
	ctx.lr = 0x83279348;
	sub_821F3D58(ctx, base);
	// 83279348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327934C: 906AD754  stw r3, -0x28ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10412 as u32), ctx.r[3].u32 ) };
	// 83279350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327935C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279360 size=56
    let mut pc: u32 = 0x83279360;
    'dispatch: loop {
        match pc {
            0x83279360 => {
    //   block [0x83279360..0x83279398)
	// 83279360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327936C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279374: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83279378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327937C: 4AF7A9DD  bl 0x821f3d58
	ctx.lr = 0x83279380;
	sub_821F3D58(ctx, base);
	// 83279380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279384: 906AD758  stw r3, -0x28a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10408 as u32), ctx.r[3].u32 ) };
	// 83279388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327938C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279398 size=56
    let mut pc: u32 = 0x83279398;
    'dispatch: loop {
        match pc {
            0x83279398 => {
    //   block [0x83279398..0x832793D0)
	// 83279398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832793A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832793A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832793A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832793AC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832793B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832793B4: 4AF7A9A5  bl 0x821f3d58
	ctx.lr = 0x832793B8;
	sub_821F3D58(ctx, base);
	// 832793B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832793BC: 906AD75C  stw r3, -0x28a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10404 as u32), ctx.r[3].u32 ) };
	// 832793C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832793C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832793C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832793CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832793D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832793D0 size=56
    let mut pc: u32 = 0x832793D0;
    'dispatch: loop {
        match pc {
            0x832793D0 => {
    //   block [0x832793D0..0x83279408)
	// 832793D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832793D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832793D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832793DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832793E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832793E4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832793E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832793EC: 4AF7A96D  bl 0x821f3d58
	ctx.lr = 0x832793F0;
	sub_821F3D58(ctx, base);
	// 832793F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832793F4: 906AD760  stw r3, -0x28a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10400 as u32), ctx.r[3].u32 ) };
	// 832793F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832793FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279408 size=56
    let mut pc: u32 = 0x83279408;
    'dispatch: loop {
        match pc {
            0x83279408 => {
    //   block [0x83279408..0x83279440)
	// 83279408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327940C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327941C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83279420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279424: 4AF7A935  bl 0x821f3d58
	ctx.lr = 0x83279428;
	sub_821F3D58(ctx, base);
	// 83279428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327942C: 906AD764  stw r3, -0x289c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10396 as u32), ctx.r[3].u32 ) };
	// 83279430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327943C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279440 size=56
    let mut pc: u32 = 0x83279440;
    'dispatch: loop {
        match pc {
            0x83279440 => {
    //   block [0x83279440..0x83279478)
	// 83279440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327944C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279454: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83279458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327945C: 4AF7A8FD  bl 0x821f3d58
	ctx.lr = 0x83279460;
	sub_821F3D58(ctx, base);
	// 83279460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279464: 906AD768  stw r3, -0x2898(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10392 as u32), ctx.r[3].u32 ) };
	// 83279468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279478 size=56
    let mut pc: u32 = 0x83279478;
    'dispatch: loop {
        match pc {
            0x83279478 => {
    //   block [0x83279478..0x832794B0)
	// 83279478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279484: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327948C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83279490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279494: 4AF7A8C5  bl 0x821f3d58
	ctx.lr = 0x83279498;
	sub_821F3D58(ctx, base);
	// 83279498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327949C: 906AD76C  stw r3, -0x2894(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10388 as u32), ctx.r[3].u32 ) };
	// 832794A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832794A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832794A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832794AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832794B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832794B0 size=56
    let mut pc: u32 = 0x832794B0;
    'dispatch: loop {
        match pc {
            0x832794B0 => {
    //   block [0x832794B0..0x832794E8)
	// 832794B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832794B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832794B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832794BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832794C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832794C4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832794C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832794CC: 4AF7A88D  bl 0x821f3d58
	ctx.lr = 0x832794D0;
	sub_821F3D58(ctx, base);
	// 832794D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832794D4: 906AD770  stw r3, -0x2890(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10384 as u32), ctx.r[3].u32 ) };
	// 832794D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832794DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832794E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832794E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832794E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832794E8 size=56
    let mut pc: u32 = 0x832794E8;
    'dispatch: loop {
        match pc {
            0x832794E8 => {
    //   block [0x832794E8..0x83279520)
	// 832794E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832794EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832794F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832794F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832794F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832794FC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83279500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279504: 4AF7A855  bl 0x821f3d58
	ctx.lr = 0x83279508;
	sub_821F3D58(ctx, base);
	// 83279508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327950C: 906AD774  stw r3, -0x288c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10380 as u32), ctx.r[3].u32 ) };
	// 83279510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327951C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279520 size=56
    let mut pc: u32 = 0x83279520;
    'dispatch: loop {
        match pc {
            0x83279520 => {
    //   block [0x83279520..0x83279558)
	// 83279520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327952C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279534: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83279538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327953C: 4AF7A81D  bl 0x821f3d58
	ctx.lr = 0x83279540;
	sub_821F3D58(ctx, base);
	// 83279540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279544: 906AD778  stw r3, -0x2888(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10376 as u32), ctx.r[3].u32 ) };
	// 83279548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327954C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279558 size=56
    let mut pc: u32 = 0x83279558;
    'dispatch: loop {
        match pc {
            0x83279558 => {
    //   block [0x83279558..0x83279590)
	// 83279558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327956C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83279570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279574: 4AF7A7E5  bl 0x821f3d58
	ctx.lr = 0x83279578;
	sub_821F3D58(ctx, base);
	// 83279578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327957C: 906AD77C  stw r3, -0x2884(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10372 as u32), ctx.r[3].u32 ) };
	// 83279580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327958C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279590 size=56
    let mut pc: u32 = 0x83279590;
    'dispatch: loop {
        match pc {
            0x83279590 => {
    //   block [0x83279590..0x832795C8)
	// 83279590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327959C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832795A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832795A4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832795A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832795AC: 4AF7A7AD  bl 0x821f3d58
	ctx.lr = 0x832795B0;
	sub_821F3D58(ctx, base);
	// 832795B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832795B4: 906AD780  stw r3, -0x2880(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10368 as u32), ctx.r[3].u32 ) };
	// 832795B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832795BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832795C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832795C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832795C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832795C8 size=56
    let mut pc: u32 = 0x832795C8;
    'dispatch: loop {
        match pc {
            0x832795C8 => {
    //   block [0x832795C8..0x83279600)
	// 832795C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832795CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832795D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832795D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832795D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832795DC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832795E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832795E4: 4AF7A775  bl 0x821f3d58
	ctx.lr = 0x832795E8;
	sub_821F3D58(ctx, base);
	// 832795E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832795EC: 906AD784  stw r3, -0x287c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10364 as u32), ctx.r[3].u32 ) };
	// 832795F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832795F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832795F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832795FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279600 size=56
    let mut pc: u32 = 0x83279600;
    'dispatch: loop {
        match pc {
            0x83279600 => {
    //   block [0x83279600..0x83279638)
	// 83279600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327960C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279614: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83279618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327961C: 4AF7A73D  bl 0x821f3d58
	ctx.lr = 0x83279620;
	sub_821F3D58(ctx, base);
	// 83279620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279624: 906AD788  stw r3, -0x2878(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10360 as u32), ctx.r[3].u32 ) };
	// 83279628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327962C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279638 size=56
    let mut pc: u32 = 0x83279638;
    'dispatch: loop {
        match pc {
            0x83279638 => {
    //   block [0x83279638..0x83279670)
	// 83279638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327963C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327964C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83279650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279654: 4AF7A705  bl 0x821f3d58
	ctx.lr = 0x83279658;
	sub_821F3D58(ctx, base);
	// 83279658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327965C: 906AD78C  stw r3, -0x2874(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10356 as u32), ctx.r[3].u32 ) };
	// 83279660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327966C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279670 size=56
    let mut pc: u32 = 0x83279670;
    'dispatch: loop {
        match pc {
            0x83279670 => {
    //   block [0x83279670..0x832796A8)
	// 83279670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327967C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279684: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83279688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327968C: 4AF7A6CD  bl 0x821f3d58
	ctx.lr = 0x83279690;
	sub_821F3D58(ctx, base);
	// 83279690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279694: 906AD790  stw r3, -0x2870(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10352 as u32), ctx.r[3].u32 ) };
	// 83279698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327969C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832796A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832796A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832796A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832796A8 size=56
    let mut pc: u32 = 0x832796A8;
    'dispatch: loop {
        match pc {
            0x832796A8 => {
    //   block [0x832796A8..0x832796E0)
	// 832796A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832796AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832796B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832796B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832796B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832796BC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832796C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832796C4: 4AF7A695  bl 0x821f3d58
	ctx.lr = 0x832796C8;
	sub_821F3D58(ctx, base);
	// 832796C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832796CC: 906AD794  stw r3, -0x286c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10348 as u32), ctx.r[3].u32 ) };
	// 832796D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832796D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832796D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832796DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832796E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832796E0 size=56
    let mut pc: u32 = 0x832796E0;
    'dispatch: loop {
        match pc {
            0x832796E0 => {
    //   block [0x832796E0..0x83279718)
	// 832796E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832796E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832796E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832796EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832796F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832796F4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832796F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832796FC: 4AF7A65D  bl 0x821f3d58
	ctx.lr = 0x83279700;
	sub_821F3D58(ctx, base);
	// 83279700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279704: 906AD798  stw r3, -0x2868(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10344 as u32), ctx.r[3].u32 ) };
	// 83279708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327970C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279718 size=56
    let mut pc: u32 = 0x83279718;
    'dispatch: loop {
        match pc {
            0x83279718 => {
    //   block [0x83279718..0x83279750)
	// 83279718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327972C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83279730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279734: 4AF7A625  bl 0x821f3d58
	ctx.lr = 0x83279738;
	sub_821F3D58(ctx, base);
	// 83279738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327973C: 906AD79C  stw r3, -0x2864(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10340 as u32), ctx.r[3].u32 ) };
	// 83279740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327974C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279750 size=64
    let mut pc: u32 = 0x83279750;
    'dispatch: loop {
        match pc {
            0x83279750 => {
    //   block [0x83279750..0x83279790)
	// 83279750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327975C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279760: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279764: 388BD5B4  addi r4, r11, -0x2a4c
	ctx.r[4].s64 = ctx.r[11].s64 + -10828;
	// 83279768: 386AD7A0  addi r3, r10, -0x2860
	ctx.r[3].s64 = ctx.r[10].s64 + -10336;
	// 8327976C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279770: 4AFB3761  bl 0x8222ced0
	ctx.lr = 0x83279774;
	sub_8222CED0(ctx, base);
	// 83279774: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279778: 3869FE98  addi r3, r9, -0x168
	ctx.r[3].s64 = ctx.r[9].s64 + -360;
	// 8327977C: 4BA307A5  bl 0x82ca9f20
	ctx.lr = 0x83279780;
	sub_82CA9F20(ctx, base);
	// 83279780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327978C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279790 size=64
    let mut pc: u32 = 0x83279790;
    'dispatch: loop {
        match pc {
            0x83279790 => {
    //   block [0x83279790..0x832797D0)
	// 83279790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327979C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832797A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832797A4: 388BD5E8  addi r4, r11, -0x2a18
	ctx.r[4].s64 = ctx.r[11].s64 + -10776;
	// 832797A8: 386AD7A4  addi r3, r10, -0x285c
	ctx.r[3].s64 = ctx.r[10].s64 + -10332;
	// 832797AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832797B0: 4AFB3721  bl 0x8222ced0
	ctx.lr = 0x832797B4;
	sub_8222CED0(ctx, base);
	// 832797B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832797B8: 3869FEA8  addi r3, r9, -0x158
	ctx.r[3].s64 = ctx.r[9].s64 + -344;
	// 832797BC: 4BA30765  bl 0x82ca9f20
	ctx.lr = 0x832797C0;
	sub_82CA9F20(ctx, base);
	// 832797C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832797C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832797C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832797CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832797D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832797D0 size=64
    let mut pc: u32 = 0x832797D0;
    'dispatch: loop {
        match pc {
            0x832797D0 => {
    //   block [0x832797D0..0x83279810)
	// 832797D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832797D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832797D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832797DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832797E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832797E4: 388BD620  addi r4, r11, -0x29e0
	ctx.r[4].s64 = ctx.r[11].s64 + -10720;
	// 832797E8: 386AD7A8  addi r3, r10, -0x2858
	ctx.r[3].s64 = ctx.r[10].s64 + -10328;
	// 832797EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832797F0: 4AFB36E1  bl 0x8222ced0
	ctx.lr = 0x832797F4;
	sub_8222CED0(ctx, base);
	// 832797F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832797F8: 3869FEB8  addi r3, r9, -0x148
	ctx.r[3].s64 = ctx.r[9].s64 + -328;
	// 832797FC: 4BA30725  bl 0x82ca9f20
	ctx.lr = 0x83279800;
	sub_82CA9F20(ctx, base);
	// 83279800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327980C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279810 size=64
    let mut pc: u32 = 0x83279810;
    'dispatch: loop {
        match pc {
            0x83279810 => {
    //   block [0x83279810..0x83279850)
	// 83279810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327981C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279824: 388BD654  addi r4, r11, -0x29ac
	ctx.r[4].s64 = ctx.r[11].s64 + -10668;
	// 83279828: 386AD7AC  addi r3, r10, -0x2854
	ctx.r[3].s64 = ctx.r[10].s64 + -10324;
	// 8327982C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279830: 4AFB36A1  bl 0x8222ced0
	ctx.lr = 0x83279834;
	sub_8222CED0(ctx, base);
	// 83279834: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279838: 3869FEC8  addi r3, r9, -0x138
	ctx.r[3].s64 = ctx.r[9].s64 + -312;
	// 8327983C: 4BA306E5  bl 0x82ca9f20
	ctx.lr = 0x83279840;
	sub_82CA9F20(ctx, base);
	// 83279840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327984C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279850 size=64
    let mut pc: u32 = 0x83279850;
    'dispatch: loop {
        match pc {
            0x83279850 => {
    //   block [0x83279850..0x83279890)
	// 83279850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327985C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279864: 388BD694  addi r4, r11, -0x296c
	ctx.r[4].s64 = ctx.r[11].s64 + -10604;
	// 83279868: 386AD7B0  addi r3, r10, -0x2850
	ctx.r[3].s64 = ctx.r[10].s64 + -10320;
	// 8327986C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279870: 4AFB3661  bl 0x8222ced0
	ctx.lr = 0x83279874;
	sub_8222CED0(ctx, base);
	// 83279874: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279878: 3869FED8  addi r3, r9, -0x128
	ctx.r[3].s64 = ctx.r[9].s64 + -296;
	// 8327987C: 4BA306A5  bl 0x82ca9f20
	ctx.lr = 0x83279880;
	sub_82CA9F20(ctx, base);
	// 83279880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279890 size=56
    let mut pc: u32 = 0x83279890;
    'dispatch: loop {
        match pc {
            0x83279890 => {
    //   block [0x83279890..0x832798C8)
	// 83279890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327989C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832798A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832798A4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832798A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832798AC: 4AF7A4AD  bl 0x821f3d58
	ctx.lr = 0x832798B0;
	sub_821F3D58(ctx, base);
	// 832798B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832798B4: 906AD7B4  stw r3, -0x284c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10316 as u32), ctx.r[3].u32 ) };
	// 832798B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832798BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832798C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832798C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832798C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832798C8 size=56
    let mut pc: u32 = 0x832798C8;
    'dispatch: loop {
        match pc {
            0x832798C8 => {
    //   block [0x832798C8..0x83279900)
	// 832798C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832798CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832798D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832798D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832798D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832798DC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832798E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832798E4: 4AF7A475  bl 0x821f3d58
	ctx.lr = 0x832798E8;
	sub_821F3D58(ctx, base);
	// 832798E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832798EC: 906AD7B8  stw r3, -0x2848(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10312 as u32), ctx.r[3].u32 ) };
	// 832798F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832798F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832798F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832798FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279900 size=56
    let mut pc: u32 = 0x83279900;
    'dispatch: loop {
        match pc {
            0x83279900 => {
    //   block [0x83279900..0x83279938)
	// 83279900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327990C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279910: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279914: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83279918: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327991C: 4AF7A43D  bl 0x821f3d58
	ctx.lr = 0x83279920;
	sub_821F3D58(ctx, base);
	// 83279920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279924: 906AD7BC  stw r3, -0x2844(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10308 as u32), ctx.r[3].u32 ) };
	// 83279928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327992C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279938 size=56
    let mut pc: u32 = 0x83279938;
    'dispatch: loop {
        match pc {
            0x83279938 => {
    //   block [0x83279938..0x83279970)
	// 83279938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279948: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327994C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83279950: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279954: 4AF7A405  bl 0x821f3d58
	ctx.lr = 0x83279958;
	sub_821F3D58(ctx, base);
	// 83279958: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327995C: 906AD7C0  stw r3, -0x2840(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10304 as u32), ctx.r[3].u32 ) };
	// 83279960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327996C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279970 size=56
    let mut pc: u32 = 0x83279970;
    'dispatch: loop {
        match pc {
            0x83279970 => {
    //   block [0x83279970..0x832799A8)
	// 83279970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327997C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279980: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279984: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83279988: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327998C: 4AF7A3CD  bl 0x821f3d58
	ctx.lr = 0x83279990;
	sub_821F3D58(ctx, base);
	// 83279990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279994: 906AD7C4  stw r3, -0x283c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10300 as u32), ctx.r[3].u32 ) };
	// 83279998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327999C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832799A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832799A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832799A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832799A8 size=56
    let mut pc: u32 = 0x832799A8;
    'dispatch: loop {
        match pc {
            0x832799A8 => {
    //   block [0x832799A8..0x832799E0)
	// 832799A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832799AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832799B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832799B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832799B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832799BC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832799C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832799C4: 4AF7A395  bl 0x821f3d58
	ctx.lr = 0x832799C8;
	sub_821F3D58(ctx, base);
	// 832799C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832799CC: 906AD7C8  stw r3, -0x2838(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10296 as u32), ctx.r[3].u32 ) };
	// 832799D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832799D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832799D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832799DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832799E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832799E0 size=56
    let mut pc: u32 = 0x832799E0;
    'dispatch: loop {
        match pc {
            0x832799E0 => {
    //   block [0x832799E0..0x83279A18)
	// 832799E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832799E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832799E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832799EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832799F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832799F4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832799F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832799FC: 4AF7A35D  bl 0x821f3d58
	ctx.lr = 0x83279A00;
	sub_821F3D58(ctx, base);
	// 83279A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279A04: 906AD7CC  stw r3, -0x2834(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10292 as u32), ctx.r[3].u32 ) };
	// 83279A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279A18 size=56
    let mut pc: u32 = 0x83279A18;
    'dispatch: loop {
        match pc {
            0x83279A18 => {
    //   block [0x83279A18..0x83279A50)
	// 83279A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279A24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279A28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279A2C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83279A30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279A34: 4AF7A325  bl 0x821f3d58
	ctx.lr = 0x83279A38;
	sub_821F3D58(ctx, base);
	// 83279A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279A3C: 906AD7D0  stw r3, -0x2830(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10288 as u32), ctx.r[3].u32 ) };
	// 83279A40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279A50 size=56
    let mut pc: u32 = 0x83279A50;
    'dispatch: loop {
        match pc {
            0x83279A50 => {
    //   block [0x83279A50..0x83279A88)
	// 83279A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279A5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279A60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279A64: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83279A68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279A6C: 4AF7A2ED  bl 0x821f3d58
	ctx.lr = 0x83279A70;
	sub_821F3D58(ctx, base);
	// 83279A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279A74: 906AD7D4  stw r3, -0x282c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10284 as u32), ctx.r[3].u32 ) };
	// 83279A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279A88 size=56
    let mut pc: u32 = 0x83279A88;
    'dispatch: loop {
        match pc {
            0x83279A88 => {
    //   block [0x83279A88..0x83279AC0)
	// 83279A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279A94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279A98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279A9C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83279AA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279AA4: 4AF7A2B5  bl 0x821f3d58
	ctx.lr = 0x83279AA8;
	sub_821F3D58(ctx, base);
	// 83279AA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279AAC: 906AD7D8  stw r3, -0x2828(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10280 as u32), ctx.r[3].u32 ) };
	// 83279AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279AC0 size=56
    let mut pc: u32 = 0x83279AC0;
    'dispatch: loop {
        match pc {
            0x83279AC0 => {
    //   block [0x83279AC0..0x83279AF8)
	// 83279AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279AD4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83279AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279ADC: 4AF7A27D  bl 0x821f3d58
	ctx.lr = 0x83279AE0;
	sub_821F3D58(ctx, base);
	// 83279AE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279AE4: 906AD7DC  stw r3, -0x2824(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10276 as u32), ctx.r[3].u32 ) };
	// 83279AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279AF8 size=56
    let mut pc: u32 = 0x83279AF8;
    'dispatch: loop {
        match pc {
            0x83279AF8 => {
    //   block [0x83279AF8..0x83279B30)
	// 83279AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279B0C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83279B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279B14: 4AF7A245  bl 0x821f3d58
	ctx.lr = 0x83279B18;
	sub_821F3D58(ctx, base);
	// 83279B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279B1C: 906AD7E0  stw r3, -0x2820(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10272 as u32), ctx.r[3].u32 ) };
	// 83279B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279B30 size=56
    let mut pc: u32 = 0x83279B30;
    'dispatch: loop {
        match pc {
            0x83279B30 => {
    //   block [0x83279B30..0x83279B68)
	// 83279B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279B44: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83279B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279B4C: 4AF7A20D  bl 0x821f3d58
	ctx.lr = 0x83279B50;
	sub_821F3D58(ctx, base);
	// 83279B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279B54: 906AD7E4  stw r3, -0x281c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10268 as u32), ctx.r[3].u32 ) };
	// 83279B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279B68 size=56
    let mut pc: u32 = 0x83279B68;
    'dispatch: loop {
        match pc {
            0x83279B68 => {
    //   block [0x83279B68..0x83279BA0)
	// 83279B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279B7C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83279B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279B84: 4AF7A1D5  bl 0x821f3d58
	ctx.lr = 0x83279B88;
	sub_821F3D58(ctx, base);
	// 83279B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279B8C: 906AD7E8  stw r3, -0x2818(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10264 as u32), ctx.r[3].u32 ) };
	// 83279B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279BA0 size=56
    let mut pc: u32 = 0x83279BA0;
    'dispatch: loop {
        match pc {
            0x83279BA0 => {
    //   block [0x83279BA0..0x83279BD8)
	// 83279BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279BB4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83279BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279BBC: 4AF7A19D  bl 0x821f3d58
	ctx.lr = 0x83279BC0;
	sub_821F3D58(ctx, base);
	// 83279BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279BC4: 906AD7EC  stw r3, -0x2814(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10260 as u32), ctx.r[3].u32 ) };
	// 83279BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279BD8 size=56
    let mut pc: u32 = 0x83279BD8;
    'dispatch: loop {
        match pc {
            0x83279BD8 => {
    //   block [0x83279BD8..0x83279C10)
	// 83279BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279BEC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83279BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279BF4: 4AF7A165  bl 0x821f3d58
	ctx.lr = 0x83279BF8;
	sub_821F3D58(ctx, base);
	// 83279BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279BFC: 906AD7F0  stw r3, -0x2810(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10256 as u32), ctx.r[3].u32 ) };
	// 83279C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279C10 size=56
    let mut pc: u32 = 0x83279C10;
    'dispatch: loop {
        match pc {
            0x83279C10 => {
    //   block [0x83279C10..0x83279C48)
	// 83279C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279C24: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83279C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279C2C: 4AF7A12D  bl 0x821f3d58
	ctx.lr = 0x83279C30;
	sub_821F3D58(ctx, base);
	// 83279C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279C34: 906AD7F4  stw r3, -0x280c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10252 as u32), ctx.r[3].u32 ) };
	// 83279C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279C48 size=56
    let mut pc: u32 = 0x83279C48;
    'dispatch: loop {
        match pc {
            0x83279C48 => {
    //   block [0x83279C48..0x83279C80)
	// 83279C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279C5C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83279C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279C64: 4AF7A0F5  bl 0x821f3d58
	ctx.lr = 0x83279C68;
	sub_821F3D58(ctx, base);
	// 83279C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279C6C: 906AD7F8  stw r3, -0x2808(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10248 as u32), ctx.r[3].u32 ) };
	// 83279C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279C80 size=56
    let mut pc: u32 = 0x83279C80;
    'dispatch: loop {
        match pc {
            0x83279C80 => {
    //   block [0x83279C80..0x83279CB8)
	// 83279C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279C94: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83279C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279C9C: 4AF7A0BD  bl 0x821f3d58
	ctx.lr = 0x83279CA0;
	sub_821F3D58(ctx, base);
	// 83279CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279CA4: 906AD7FC  stw r3, -0x2804(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10244 as u32), ctx.r[3].u32 ) };
	// 83279CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279CB8 size=56
    let mut pc: u32 = 0x83279CB8;
    'dispatch: loop {
        match pc {
            0x83279CB8 => {
    //   block [0x83279CB8..0x83279CF0)
	// 83279CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279CCC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83279CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279CD4: 4AF7A085  bl 0x821f3d58
	ctx.lr = 0x83279CD8;
	sub_821F3D58(ctx, base);
	// 83279CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279CDC: 906AD800  stw r3, -0x2800(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10240 as u32), ctx.r[3].u32 ) };
	// 83279CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279CF0 size=56
    let mut pc: u32 = 0x83279CF0;
    'dispatch: loop {
        match pc {
            0x83279CF0 => {
    //   block [0x83279CF0..0x83279D28)
	// 83279CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83279D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83279D04: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83279D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83279D0C: 4AF7A04D  bl 0x821f3d58
	ctx.lr = 0x83279D10;
	sub_821F3D58(ctx, base);
	// 83279D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279D14: 906AD804  stw r3, -0x27fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10236 as u32), ctx.r[3].u32 ) };
	// 83279D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279D28 size=64
    let mut pc: u32 = 0x83279D28;
    'dispatch: loop {
        match pc {
            0x83279D28 => {
    //   block [0x83279D28..0x83279D68)
	// 83279D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279D34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279D3C: 388BD874  addi r4, r11, -0x278c
	ctx.r[4].s64 = ctx.r[11].s64 + -10124;
	// 83279D40: 386AD808  addi r3, r10, -0x27f8
	ctx.r[3].s64 = ctx.r[10].s64 + -10232;
	// 83279D44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279D48: 4AFB3189  bl 0x8222ced0
	ctx.lr = 0x83279D4C;
	sub_8222CED0(ctx, base);
	// 83279D4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279D50: 3869FEE8  addi r3, r9, -0x118
	ctx.r[3].s64 = ctx.r[9].s64 + -280;
	// 83279D54: 4BA301CD  bl 0x82ca9f20
	ctx.lr = 0x83279D58;
	sub_82CA9F20(ctx, base);
	// 83279D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279D68 size=64
    let mut pc: u32 = 0x83279D68;
    'dispatch: loop {
        match pc {
            0x83279D68 => {
    //   block [0x83279D68..0x83279DA8)
	// 83279D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279D74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279D7C: 388BD8A8  addi r4, r11, -0x2758
	ctx.r[4].s64 = ctx.r[11].s64 + -10072;
	// 83279D80: 386AD80C  addi r3, r10, -0x27f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10228;
	// 83279D84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279D88: 4AFB3149  bl 0x8222ced0
	ctx.lr = 0x83279D8C;
	sub_8222CED0(ctx, base);
	// 83279D8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279D90: 3869FEF8  addi r3, r9, -0x108
	ctx.r[3].s64 = ctx.r[9].s64 + -264;
	// 83279D94: 4BA3018D  bl 0x82ca9f20
	ctx.lr = 0x83279D98;
	sub_82CA9F20(ctx, base);
	// 83279D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279DA8 size=64
    let mut pc: u32 = 0x83279DA8;
    'dispatch: loop {
        match pc {
            0x83279DA8 => {
    //   block [0x83279DA8..0x83279DE8)
	// 83279DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279DB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279DBC: 388BD8DC  addi r4, r11, -0x2724
	ctx.r[4].s64 = ctx.r[11].s64 + -10020;
	// 83279DC0: 386AD810  addi r3, r10, -0x27f0
	ctx.r[3].s64 = ctx.r[10].s64 + -10224;
	// 83279DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279DC8: 4AFB3109  bl 0x8222ced0
	ctx.lr = 0x83279DCC;
	sub_8222CED0(ctx, base);
	// 83279DCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279DD0: 3869FF08  addi r3, r9, -0xf8
	ctx.r[3].s64 = ctx.r[9].s64 + -248;
	// 83279DD4: 4BA3014D  bl 0x82ca9f20
	ctx.lr = 0x83279DD8;
	sub_82CA9F20(ctx, base);
	// 83279DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279DE8 size=64
    let mut pc: u32 = 0x83279DE8;
    'dispatch: loop {
        match pc {
            0x83279DE8 => {
    //   block [0x83279DE8..0x83279E28)
	// 83279DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279DF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279DFC: 388BD910  addi r4, r11, -0x26f0
	ctx.r[4].s64 = ctx.r[11].s64 + -9968;
	// 83279E00: 386AD814  addi r3, r10, -0x27ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10220;
	// 83279E04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279E08: 4AFB30C9  bl 0x8222ced0
	ctx.lr = 0x83279E0C;
	sub_8222CED0(ctx, base);
	// 83279E0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279E10: 3869FF18  addi r3, r9, -0xe8
	ctx.r[3].s64 = ctx.r[9].s64 + -232;
	// 83279E14: 4BA3010D  bl 0x82ca9f20
	ctx.lr = 0x83279E18;
	sub_82CA9F20(ctx, base);
	// 83279E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279E28 size=64
    let mut pc: u32 = 0x83279E28;
    'dispatch: loop {
        match pc {
            0x83279E28 => {
    //   block [0x83279E28..0x83279E68)
	// 83279E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279E34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279E3C: 388BD944  addi r4, r11, -0x26bc
	ctx.r[4].s64 = ctx.r[11].s64 + -9916;
	// 83279E40: 386AD818  addi r3, r10, -0x27e8
	ctx.r[3].s64 = ctx.r[10].s64 + -10216;
	// 83279E44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279E48: 4AFB3089  bl 0x8222ced0
	ctx.lr = 0x83279E4C;
	sub_8222CED0(ctx, base);
	// 83279E4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279E50: 3869FF28  addi r3, r9, -0xd8
	ctx.r[3].s64 = ctx.r[9].s64 + -216;
	// 83279E54: 4BA300CD  bl 0x82ca9f20
	ctx.lr = 0x83279E58;
	sub_82CA9F20(ctx, base);
	// 83279E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279E68 size=64
    let mut pc: u32 = 0x83279E68;
    'dispatch: loop {
        match pc {
            0x83279E68 => {
    //   block [0x83279E68..0x83279EA8)
	// 83279E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279E74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279E7C: 388BD978  addi r4, r11, -0x2688
	ctx.r[4].s64 = ctx.r[11].s64 + -9864;
	// 83279E80: 386AD81C  addi r3, r10, -0x27e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10212;
	// 83279E84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279E88: 4AFB3049  bl 0x8222ced0
	ctx.lr = 0x83279E8C;
	sub_8222CED0(ctx, base);
	// 83279E8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279E90: 3869FF38  addi r3, r9, -0xc8
	ctx.r[3].s64 = ctx.r[9].s64 + -200;
	// 83279E94: 4BA3008D  bl 0x82ca9f20
	ctx.lr = 0x83279E98;
	sub_82CA9F20(ctx, base);
	// 83279E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279EA8 size=64
    let mut pc: u32 = 0x83279EA8;
    'dispatch: loop {
        match pc {
            0x83279EA8 => {
    //   block [0x83279EA8..0x83279EE8)
	// 83279EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279EB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279EB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279EBC: 388BD9B4  addi r4, r11, -0x264c
	ctx.r[4].s64 = ctx.r[11].s64 + -9804;
	// 83279EC0: 386AD820  addi r3, r10, -0x27e0
	ctx.r[3].s64 = ctx.r[10].s64 + -10208;
	// 83279EC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279EC8: 4AFB3009  bl 0x8222ced0
	ctx.lr = 0x83279ECC;
	sub_8222CED0(ctx, base);
	// 83279ECC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279ED0: 3869FF48  addi r3, r9, -0xb8
	ctx.r[3].s64 = ctx.r[9].s64 + -184;
	// 83279ED4: 4BA3004D  bl 0x82ca9f20
	ctx.lr = 0x83279ED8;
	sub_82CA9F20(ctx, base);
	// 83279ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279EE8 size=64
    let mut pc: u32 = 0x83279EE8;
    'dispatch: loop {
        match pc {
            0x83279EE8 => {
    //   block [0x83279EE8..0x83279F28)
	// 83279EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279EF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279EFC: 388BD9F4  addi r4, r11, -0x260c
	ctx.r[4].s64 = ctx.r[11].s64 + -9740;
	// 83279F00: 386AD824  addi r3, r10, -0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	// 83279F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279F08: 4AFB2FC9  bl 0x8222ced0
	ctx.lr = 0x83279F0C;
	sub_8222CED0(ctx, base);
	// 83279F0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279F10: 3869FF58  addi r3, r9, -0xa8
	ctx.r[3].s64 = ctx.r[9].s64 + -168;
	// 83279F14: 4BA3000D  bl 0x82ca9f20
	ctx.lr = 0x83279F18;
	sub_82CA9F20(ctx, base);
	// 83279F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279F28 size=64
    let mut pc: u32 = 0x83279F28;
    'dispatch: loop {
        match pc {
            0x83279F28 => {
    //   block [0x83279F28..0x83279F68)
	// 83279F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279F34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279F3C: 388BDA30  addi r4, r11, -0x25d0
	ctx.r[4].s64 = ctx.r[11].s64 + -9680;
	// 83279F40: 386AD828  addi r3, r10, -0x27d8
	ctx.r[3].s64 = ctx.r[10].s64 + -10200;
	// 83279F44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279F48: 4AFB2F89  bl 0x8222ced0
	ctx.lr = 0x83279F4C;
	sub_8222CED0(ctx, base);
	// 83279F4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279F50: 3869FF68  addi r3, r9, -0x98
	ctx.r[3].s64 = ctx.r[9].s64 + -152;
	// 83279F54: 4BA2FFCD  bl 0x82ca9f20
	ctx.lr = 0x83279F58;
	sub_82CA9F20(ctx, base);
	// 83279F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279F68 size=64
    let mut pc: u32 = 0x83279F68;
    'dispatch: loop {
        match pc {
            0x83279F68 => {
    //   block [0x83279F68..0x83279FA8)
	// 83279F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279F74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279F7C: 388BDA70  addi r4, r11, -0x2590
	ctx.r[4].s64 = ctx.r[11].s64 + -9616;
	// 83279F80: 386AD82C  addi r3, r10, -0x27d4
	ctx.r[3].s64 = ctx.r[10].s64 + -10196;
	// 83279F84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279F88: 4AFB2F49  bl 0x8222ced0
	ctx.lr = 0x83279F8C;
	sub_8222CED0(ctx, base);
	// 83279F8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279F90: 3869FF78  addi r3, r9, -0x88
	ctx.r[3].s64 = ctx.r[9].s64 + -136;
	// 83279F94: 4BA2FF8D  bl 0x82ca9f20
	ctx.lr = 0x83279F98;
	sub_82CA9F20(ctx, base);
	// 83279F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279FA8 size=64
    let mut pc: u32 = 0x83279FA8;
    'dispatch: loop {
        match pc {
            0x83279FA8 => {
    //   block [0x83279FA8..0x83279FE8)
	// 83279FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279FB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279FBC: 388BDAA0  addi r4, r11, -0x2560
	ctx.r[4].s64 = ctx.r[11].s64 + -9568;
	// 83279FC0: 386AD830  addi r3, r10, -0x27d0
	ctx.r[3].s64 = ctx.r[10].s64 + -10192;
	// 83279FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83279FC8: 4AFB2F09  bl 0x8222ced0
	ctx.lr = 0x83279FCC;
	sub_8222CED0(ctx, base);
	// 83279FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83279FD0: 3869FF88  addi r3, r9, -0x78
	ctx.r[3].s64 = ctx.r[9].s64 + -120;
	// 83279FD4: 4BA2FF4D  bl 0x82ca9f20
	ctx.lr = 0x83279FD8;
	sub_82CA9F20(ctx, base);
	// 83279FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83279FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83279FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83279FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83279FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83279FE8 size=64
    let mut pc: u32 = 0x83279FE8;
    'dispatch: loop {
        match pc {
            0x83279FE8 => {
    //   block [0x83279FE8..0x8327A028)
	// 83279FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83279FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83279FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83279FF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83279FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83279FFC: 388BDAD0  addi r4, r11, -0x2530
	ctx.r[4].s64 = ctx.r[11].s64 + -9520;
	// 8327A000: 386AD834  addi r3, r10, -0x27cc
	ctx.r[3].s64 = ctx.r[10].s64 + -10188;
	// 8327A004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A008: 4AFB2EC9  bl 0x8222ced0
	ctx.lr = 0x8327A00C;
	sub_8222CED0(ctx, base);
	// 8327A00C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A010: 3869FF98  addi r3, r9, -0x68
	ctx.r[3].s64 = ctx.r[9].s64 + -104;
	// 8327A014: 4BA2FF0D  bl 0x82ca9f20
	ctx.lr = 0x8327A018;
	sub_82CA9F20(ctx, base);
	// 8327A018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A028 size=56
    let mut pc: u32 = 0x8327A028;
    'dispatch: loop {
        match pc {
            0x8327A028 => {
    //   block [0x8327A028..0x8327A060)
	// 8327A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A034: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A038: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A03C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327A040: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A044: 4AF79D15  bl 0x821f3d58
	ctx.lr = 0x8327A048;
	sub_821F3D58(ctx, base);
	// 8327A048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A04C: 906AD838  stw r3, -0x27c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10184 as u32), ctx.r[3].u32 ) };
	// 8327A050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A060 size=56
    let mut pc: u32 = 0x8327A060;
    'dispatch: loop {
        match pc {
            0x8327A060 => {
    //   block [0x8327A060..0x8327A098)
	// 8327A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A06C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A070: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A074: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327A078: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A07C: 4AF79CDD  bl 0x821f3d58
	ctx.lr = 0x8327A080;
	sub_821F3D58(ctx, base);
	// 8327A080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A084: 906AD83C  stw r3, -0x27c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10180 as u32), ctx.r[3].u32 ) };
	// 8327A088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A098 size=56
    let mut pc: u32 = 0x8327A098;
    'dispatch: loop {
        match pc {
            0x8327A098 => {
    //   block [0x8327A098..0x8327A0D0)
	// 8327A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A0A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A0A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A0AC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327A0B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A0B4: 4AF79CA5  bl 0x821f3d58
	ctx.lr = 0x8327A0B8;
	sub_821F3D58(ctx, base);
	// 8327A0B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A0BC: 906AD840  stw r3, -0x27c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10176 as u32), ctx.r[3].u32 ) };
	// 8327A0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A0D0 size=56
    let mut pc: u32 = 0x8327A0D0;
    'dispatch: loop {
        match pc {
            0x8327A0D0 => {
    //   block [0x8327A0D0..0x8327A108)
	// 8327A0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A0DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A0E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A0E4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327A0E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A0EC: 4AF79C6D  bl 0x821f3d58
	ctx.lr = 0x8327A0F0;
	sub_821F3D58(ctx, base);
	// 8327A0F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A0F4: 906AD844  stw r3, -0x27bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10172 as u32), ctx.r[3].u32 ) };
	// 8327A0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A108 size=56
    let mut pc: u32 = 0x8327A108;
    'dispatch: loop {
        match pc {
            0x8327A108 => {
    //   block [0x8327A108..0x8327A140)
	// 8327A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A114: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A118: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A11C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327A120: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A124: 4AF79C35  bl 0x821f3d58
	ctx.lr = 0x8327A128;
	sub_821F3D58(ctx, base);
	// 8327A128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A12C: 906AD848  stw r3, -0x27b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10168 as u32), ctx.r[3].u32 ) };
	// 8327A130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A140 size=56
    let mut pc: u32 = 0x8327A140;
    'dispatch: loop {
        match pc {
            0x8327A140 => {
    //   block [0x8327A140..0x8327A178)
	// 8327A140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A14C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A150: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A154: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327A158: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A15C: 4AF79BFD  bl 0x821f3d58
	ctx.lr = 0x8327A160;
	sub_821F3D58(ctx, base);
	// 8327A160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A164: 906AD84C  stw r3, -0x27b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10164 as u32), ctx.r[3].u32 ) };
	// 8327A168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A178 size=56
    let mut pc: u32 = 0x8327A178;
    'dispatch: loop {
        match pc {
            0x8327A178 => {
    //   block [0x8327A178..0x8327A1B0)
	// 8327A178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A18C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327A190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A194: 4AF79BC5  bl 0x821f3d58
	ctx.lr = 0x8327A198;
	sub_821F3D58(ctx, base);
	// 8327A198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A19C: 906AD850  stw r3, -0x27b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10160 as u32), ctx.r[3].u32 ) };
	// 8327A1A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A1B0 size=56
    let mut pc: u32 = 0x8327A1B0;
    'dispatch: loop {
        match pc {
            0x8327A1B0 => {
    //   block [0x8327A1B0..0x8327A1E8)
	// 8327A1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A1BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A1C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A1C4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327A1C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A1CC: 4AF79B8D  bl 0x821f3d58
	ctx.lr = 0x8327A1D0;
	sub_821F3D58(ctx, base);
	// 8327A1D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A1D4: 906AD854  stw r3, -0x27ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10156 as u32), ctx.r[3].u32 ) };
	// 8327A1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A1E8 size=56
    let mut pc: u32 = 0x8327A1E8;
    'dispatch: loop {
        match pc {
            0x8327A1E8 => {
    //   block [0x8327A1E8..0x8327A220)
	// 8327A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A1F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A1F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A1F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A1FC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327A200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A204: 4AF79B55  bl 0x821f3d58
	ctx.lr = 0x8327A208;
	sub_821F3D58(ctx, base);
	// 8327A208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A20C: 906AD858  stw r3, -0x27a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10152 as u32), ctx.r[3].u32 ) };
	// 8327A210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A220 size=56
    let mut pc: u32 = 0x8327A220;
    'dispatch: loop {
        match pc {
            0x8327A220 => {
    //   block [0x8327A220..0x8327A258)
	// 8327A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A22C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A234: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327A238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A23C: 4AF79B1D  bl 0x821f3d58
	ctx.lr = 0x8327A240;
	sub_821F3D58(ctx, base);
	// 8327A240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A244: 906AD85C  stw r3, -0x27a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10148 as u32), ctx.r[3].u32 ) };
	// 8327A248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A258 size=56
    let mut pc: u32 = 0x8327A258;
    'dispatch: loop {
        match pc {
            0x8327A258 => {
    //   block [0x8327A258..0x8327A290)
	// 8327A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A26C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327A270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A274: 4AF79AE5  bl 0x821f3d58
	ctx.lr = 0x8327A278;
	sub_821F3D58(ctx, base);
	// 8327A278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A27C: 906AD860  stw r3, -0x27a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10144 as u32), ctx.r[3].u32 ) };
	// 8327A280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A290 size=56
    let mut pc: u32 = 0x8327A290;
    'dispatch: loop {
        match pc {
            0x8327A290 => {
    //   block [0x8327A290..0x8327A2C8)
	// 8327A290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A29C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A2A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A2A4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327A2A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A2AC: 4AF79AAD  bl 0x821f3d58
	ctx.lr = 0x8327A2B0;
	sub_821F3D58(ctx, base);
	// 8327A2B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A2B4: 906AD864  stw r3, -0x279c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10140 as u32), ctx.r[3].u32 ) };
	// 8327A2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A2C8 size=56
    let mut pc: u32 = 0x8327A2C8;
    'dispatch: loop {
        match pc {
            0x8327A2C8 => {
    //   block [0x8327A2C8..0x8327A300)
	// 8327A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A2D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A2D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A2DC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327A2E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A2E4: 4AF79A75  bl 0x821f3d58
	ctx.lr = 0x8327A2E8;
	sub_821F3D58(ctx, base);
	// 8327A2E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A2EC: 906AD868  stw r3, -0x2798(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10136 as u32), ctx.r[3].u32 ) };
	// 8327A2F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A300 size=56
    let mut pc: u32 = 0x8327A300;
    'dispatch: loop {
        match pc {
            0x8327A300 => {
    //   block [0x8327A300..0x8327A338)
	// 8327A300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A30C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A314: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327A318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A31C: 4AF79A3D  bl 0x821f3d58
	ctx.lr = 0x8327A320;
	sub_821F3D58(ctx, base);
	// 8327A320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A324: 906AD86C  stw r3, -0x2794(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10132 as u32), ctx.r[3].u32 ) };
	// 8327A328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A338 size=56
    let mut pc: u32 = 0x8327A338;
    'dispatch: loop {
        match pc {
            0x8327A338 => {
    //   block [0x8327A338..0x8327A370)
	// 8327A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A34C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327A350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A354: 4AF79A05  bl 0x821f3d58
	ctx.lr = 0x8327A358;
	sub_821F3D58(ctx, base);
	// 8327A358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A35C: 906AD870  stw r3, -0x2790(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10128 as u32), ctx.r[3].u32 ) };
	// 8327A360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A370 size=56
    let mut pc: u32 = 0x8327A370;
    'dispatch: loop {
        match pc {
            0x8327A370 => {
    //   block [0x8327A370..0x8327A3A8)
	// 8327A370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A37C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A384: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327A388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A38C: 4AF799CD  bl 0x821f3d58
	ctx.lr = 0x8327A390;
	sub_821F3D58(ctx, base);
	// 8327A390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A394: 906AD874  stw r3, -0x278c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10124 as u32), ctx.r[3].u32 ) };
	// 8327A398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A3A8 size=56
    let mut pc: u32 = 0x8327A3A8;
    'dispatch: loop {
        match pc {
            0x8327A3A8 => {
    //   block [0x8327A3A8..0x8327A3E0)
	// 8327A3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A3B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A3B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A3B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A3BC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327A3C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A3C4: 4AF79995  bl 0x821f3d58
	ctx.lr = 0x8327A3C8;
	sub_821F3D58(ctx, base);
	// 8327A3C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A3CC: 906AD878  stw r3, -0x2788(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10120 as u32), ctx.r[3].u32 ) };
	// 8327A3D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A3E0 size=56
    let mut pc: u32 = 0x8327A3E0;
    'dispatch: loop {
        match pc {
            0x8327A3E0 => {
    //   block [0x8327A3E0..0x8327A418)
	// 8327A3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A3E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A3EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A3F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A3F4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327A3F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A3FC: 4AF7995D  bl 0x821f3d58
	ctx.lr = 0x8327A400;
	sub_821F3D58(ctx, base);
	// 8327A400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A404: 906AD87C  stw r3, -0x2784(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10116 as u32), ctx.r[3].u32 ) };
	// 8327A408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A418 size=56
    let mut pc: u32 = 0x8327A418;
    'dispatch: loop {
        match pc {
            0x8327A418 => {
    //   block [0x8327A418..0x8327A450)
	// 8327A418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A424: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A428: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A42C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327A430: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A434: 4AF79925  bl 0x821f3d58
	ctx.lr = 0x8327A438;
	sub_821F3D58(ctx, base);
	// 8327A438: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A43C: 906AD880  stw r3, -0x2780(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10112 as u32), ctx.r[3].u32 ) };
	// 8327A440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A450 size=56
    let mut pc: u32 = 0x8327A450;
    'dispatch: loop {
        match pc {
            0x8327A450 => {
    //   block [0x8327A450..0x8327A488)
	// 8327A450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A45C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A460: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A464: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327A468: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A46C: 4AF798ED  bl 0x821f3d58
	ctx.lr = 0x8327A470;
	sub_821F3D58(ctx, base);
	// 8327A470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A474: 906AD884  stw r3, -0x277c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10108 as u32), ctx.r[3].u32 ) };
	// 8327A478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A488 size=56
    let mut pc: u32 = 0x8327A488;
    'dispatch: loop {
        match pc {
            0x8327A488 => {
    //   block [0x8327A488..0x8327A4C0)
	// 8327A488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327A498: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327A49C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327A4A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327A4A4: 4AF798B5  bl 0x821f3d58
	ctx.lr = 0x8327A4A8;
	sub_821F3D58(ctx, base);
	// 8327A4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A4AC: 906AD888  stw r3, -0x2778(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10104 as u32), ctx.r[3].u32 ) };
	// 8327A4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A4C0 size=64
    let mut pc: u32 = 0x8327A4C0;
    'dispatch: loop {
        match pc {
            0x8327A4C0 => {
    //   block [0x8327A4C0..0x8327A500)
	// 8327A4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A4CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A4D4: 388BDB50  addi r4, r11, -0x24b0
	ctx.r[4].s64 = ctx.r[11].s64 + -9392;
	// 8327A4D8: 386AD88C  addi r3, r10, -0x2774
	ctx.r[3].s64 = ctx.r[10].s64 + -10100;
	// 8327A4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A4E0: 4AFB29F1  bl 0x8222ced0
	ctx.lr = 0x8327A4E4;
	sub_8222CED0(ctx, base);
	// 8327A4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A4E8: 3869FFA8  addi r3, r9, -0x58
	ctx.r[3].s64 = ctx.r[9].s64 + -88;
	// 8327A4EC: 4BA2FA35  bl 0x82ca9f20
	ctx.lr = 0x8327A4F0;
	sub_82CA9F20(ctx, base);
	// 8327A4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A500 size=64
    let mut pc: u32 = 0x8327A500;
    'dispatch: loop {
        match pc {
            0x8327A500 => {
    //   block [0x8327A500..0x8327A540)
	// 8327A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A50C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A514: 388BDB84  addi r4, r11, -0x247c
	ctx.r[4].s64 = ctx.r[11].s64 + -9340;
	// 8327A518: 386AD890  addi r3, r10, -0x2770
	ctx.r[3].s64 = ctx.r[10].s64 + -10096;
	// 8327A51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A520: 4AFB29B1  bl 0x8222ced0
	ctx.lr = 0x8327A524;
	sub_8222CED0(ctx, base);
	// 8327A524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A528: 3869FFB8  addi r3, r9, -0x48
	ctx.r[3].s64 = ctx.r[9].s64 + -72;
	// 8327A52C: 4BA2F9F5  bl 0x82ca9f20
	ctx.lr = 0x8327A530;
	sub_82CA9F20(ctx, base);
	// 8327A530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A540 size=64
    let mut pc: u32 = 0x8327A540;
    'dispatch: loop {
        match pc {
            0x8327A540 => {
    //   block [0x8327A540..0x8327A580)
	// 8327A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A54C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A554: 388BDBC4  addi r4, r11, -0x243c
	ctx.r[4].s64 = ctx.r[11].s64 + -9276;
	// 8327A558: 386AD894  addi r3, r10, -0x276c
	ctx.r[3].s64 = ctx.r[10].s64 + -10092;
	// 8327A55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A560: 4AFB2971  bl 0x8222ced0
	ctx.lr = 0x8327A564;
	sub_8222CED0(ctx, base);
	// 8327A564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A568: 3869FFC8  addi r3, r9, -0x38
	ctx.r[3].s64 = ctx.r[9].s64 + -56;
	// 8327A56C: 4BA2F9B5  bl 0x82ca9f20
	ctx.lr = 0x8327A570;
	sub_82CA9F20(ctx, base);
	// 8327A570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A580 size=64
    let mut pc: u32 = 0x8327A580;
    'dispatch: loop {
        match pc {
            0x8327A580 => {
    //   block [0x8327A580..0x8327A5C0)
	// 8327A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A58C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A594: 388BDC00  addi r4, r11, -0x2400
	ctx.r[4].s64 = ctx.r[11].s64 + -9216;
	// 8327A598: 386AD898  addi r3, r10, -0x2768
	ctx.r[3].s64 = ctx.r[10].s64 + -10088;
	// 8327A59C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A5A0: 4AFB2931  bl 0x8222ced0
	ctx.lr = 0x8327A5A4;
	sub_8222CED0(ctx, base);
	// 8327A5A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A5A8: 3869FFD8  addi r3, r9, -0x28
	ctx.r[3].s64 = ctx.r[9].s64 + -40;
	// 8327A5AC: 4BA2F975  bl 0x82ca9f20
	ctx.lr = 0x8327A5B0;
	sub_82CA9F20(ctx, base);
	// 8327A5B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A5C0 size=64
    let mut pc: u32 = 0x8327A5C0;
    'dispatch: loop {
        match pc {
            0x8327A5C0 => {
    //   block [0x8327A5C0..0x8327A600)
	// 8327A5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A5CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A5D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A5D4: 388BDC48  addi r4, r11, -0x23b8
	ctx.r[4].s64 = ctx.r[11].s64 + -9144;
	// 8327A5D8: 386AD89C  addi r3, r10, -0x2764
	ctx.r[3].s64 = ctx.r[10].s64 + -10084;
	// 8327A5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A5E0: 4AFB28F1  bl 0x8222ced0
	ctx.lr = 0x8327A5E4;
	sub_8222CED0(ctx, base);
	// 8327A5E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A5E8: 3869FFE8  addi r3, r9, -0x18
	ctx.r[3].s64 = ctx.r[9].s64 + -24;
	// 8327A5EC: 4BA2F935  bl 0x82ca9f20
	ctx.lr = 0x8327A5F0;
	sub_82CA9F20(ctx, base);
	// 8327A5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A600 size=64
    let mut pc: u32 = 0x8327A600;
    'dispatch: loop {
        match pc {
            0x8327A600 => {
    //   block [0x8327A600..0x8327A640)
	// 8327A600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A60C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A614: 388BDC90  addi r4, r11, -0x2370
	ctx.r[4].s64 = ctx.r[11].s64 + -9072;
	// 8327A618: 386AD8A0  addi r3, r10, -0x2760
	ctx.r[3].s64 = ctx.r[10].s64 + -10080;
	// 8327A61C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A620: 4AFB28B1  bl 0x8222ced0
	ctx.lr = 0x8327A624;
	sub_8222CED0(ctx, base);
	// 8327A624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A628: 3869FFF8  addi r3, r9, -8
	ctx.r[3].s64 = ctx.r[9].s64 + -8;
	// 8327A62C: 4BA2F8F5  bl 0x82ca9f20
	ctx.lr = 0x8327A630;
	sub_82CA9F20(ctx, base);
	// 8327A630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A640 size=64
    let mut pc: u32 = 0x8327A640;
    'dispatch: loop {
        match pc {
            0x8327A640 => {
    //   block [0x8327A640..0x8327A680)
	// 8327A640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A64C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A654: 388BDCD4  addi r4, r11, -0x232c
	ctx.r[4].s64 = ctx.r[11].s64 + -9004;
	// 8327A658: 386AD8A4  addi r3, r10, -0x275c
	ctx.r[3].s64 = ctx.r[10].s64 + -10076;
	// 8327A65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A660: 4AFB2871  bl 0x8222ced0
	ctx.lr = 0x8327A664;
	sub_8222CED0(ctx, base);
	// 8327A664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A668: 38690008  addi r3, r9, 8
	ctx.r[3].s64 = ctx.r[9].s64 + 8;
	// 8327A66C: 4BA2F8B5  bl 0x82ca9f20
	ctx.lr = 0x8327A670;
	sub_82CA9F20(ctx, base);
	// 8327A670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A680 size=64
    let mut pc: u32 = 0x8327A680;
    'dispatch: loop {
        match pc {
            0x8327A680 => {
    //   block [0x8327A680..0x8327A6C0)
	// 8327A680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A68C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A694: 388BDD10  addi r4, r11, -0x22f0
	ctx.r[4].s64 = ctx.r[11].s64 + -8944;
	// 8327A698: 386AD8A8  addi r3, r10, -0x2758
	ctx.r[3].s64 = ctx.r[10].s64 + -10072;
	// 8327A69C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A6A0: 4AFB2831  bl 0x8222ced0
	ctx.lr = 0x8327A6A4;
	sub_8222CED0(ctx, base);
	// 8327A6A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A6A8: 38690018  addi r3, r9, 0x18
	ctx.r[3].s64 = ctx.r[9].s64 + 24;
	// 8327A6AC: 4BA2F875  bl 0x82ca9f20
	ctx.lr = 0x8327A6B0;
	sub_82CA9F20(ctx, base);
	// 8327A6B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A6C0 size=64
    let mut pc: u32 = 0x8327A6C0;
    'dispatch: loop {
        match pc {
            0x8327A6C0 => {
    //   block [0x8327A6C0..0x8327A700)
	// 8327A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A6C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A6CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A6D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A6D4: 388BDD58  addi r4, r11, -0x22a8
	ctx.r[4].s64 = ctx.r[11].s64 + -8872;
	// 8327A6D8: 386AD8AC  addi r3, r10, -0x2754
	ctx.r[3].s64 = ctx.r[10].s64 + -10068;
	// 8327A6DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A6E0: 4AFB27F1  bl 0x8222ced0
	ctx.lr = 0x8327A6E4;
	sub_8222CED0(ctx, base);
	// 8327A6E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A6E8: 38690028  addi r3, r9, 0x28
	ctx.r[3].s64 = ctx.r[9].s64 + 40;
	// 8327A6EC: 4BA2F835  bl 0x82ca9f20
	ctx.lr = 0x8327A6F0;
	sub_82CA9F20(ctx, base);
	// 8327A6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A700 size=64
    let mut pc: u32 = 0x8327A700;
    'dispatch: loop {
        match pc {
            0x8327A700 => {
    //   block [0x8327A700..0x8327A740)
	// 8327A700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A70C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A714: 388BDDA0  addi r4, r11, -0x2260
	ctx.r[4].s64 = ctx.r[11].s64 + -8800;
	// 8327A718: 386AD8B0  addi r3, r10, -0x2750
	ctx.r[3].s64 = ctx.r[10].s64 + -10064;
	// 8327A71C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A720: 4AFB27B1  bl 0x8222ced0
	ctx.lr = 0x8327A724;
	sub_8222CED0(ctx, base);
	// 8327A724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A728: 38690038  addi r3, r9, 0x38
	ctx.r[3].s64 = ctx.r[9].s64 + 56;
	// 8327A72C: 4BA2F7F5  bl 0x82ca9f20
	ctx.lr = 0x8327A730;
	sub_82CA9F20(ctx, base);
	// 8327A730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A740 size=64
    let mut pc: u32 = 0x8327A740;
    'dispatch: loop {
        match pc {
            0x8327A740 => {
    //   block [0x8327A740..0x8327A780)
	// 8327A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A74C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A754: 388BDDF0  addi r4, r11, -0x2210
	ctx.r[4].s64 = ctx.r[11].s64 + -8720;
	// 8327A758: 386AD8B4  addi r3, r10, -0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + -10060;
	// 8327A75C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A760: 4AFB2771  bl 0x8222ced0
	ctx.lr = 0x8327A764;
	sub_8222CED0(ctx, base);
	// 8327A764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A768: 38690048  addi r3, r9, 0x48
	ctx.r[3].s64 = ctx.r[9].s64 + 72;
	// 8327A76C: 4BA2F7B5  bl 0x82ca9f20
	ctx.lr = 0x8327A770;
	sub_82CA9F20(ctx, base);
	// 8327A770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A780 size=64
    let mut pc: u32 = 0x8327A780;
    'dispatch: loop {
        match pc {
            0x8327A780 => {
    //   block [0x8327A780..0x8327A7C0)
	// 8327A780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A78C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A794: 388BDE40  addi r4, r11, -0x21c0
	ctx.r[4].s64 = ctx.r[11].s64 + -8640;
	// 8327A798: 386AD8B8  addi r3, r10, -0x2748
	ctx.r[3].s64 = ctx.r[10].s64 + -10056;
	// 8327A79C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A7A0: 4AFB2731  bl 0x8222ced0
	ctx.lr = 0x8327A7A4;
	sub_8222CED0(ctx, base);
	// 8327A7A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A7A8: 38690058  addi r3, r9, 0x58
	ctx.r[3].s64 = ctx.r[9].s64 + 88;
	// 8327A7AC: 4BA2F775  bl 0x82ca9f20
	ctx.lr = 0x8327A7B0;
	sub_82CA9F20(ctx, base);
	// 8327A7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A7C0 size=64
    let mut pc: u32 = 0x8327A7C0;
    'dispatch: loop {
        match pc {
            0x8327A7C0 => {
    //   block [0x8327A7C0..0x8327A800)
	// 8327A7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A7CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A7D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A7D4: 388BDE8C  addi r4, r11, -0x2174
	ctx.r[4].s64 = ctx.r[11].s64 + -8564;
	// 8327A7D8: 386AD8BC  addi r3, r10, -0x2744
	ctx.r[3].s64 = ctx.r[10].s64 + -10052;
	// 8327A7DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A7E0: 4AFB26F1  bl 0x8222ced0
	ctx.lr = 0x8327A7E4;
	sub_8222CED0(ctx, base);
	// 8327A7E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A7E8: 38690068  addi r3, r9, 0x68
	ctx.r[3].s64 = ctx.r[9].s64 + 104;
	// 8327A7EC: 4BA2F735  bl 0x82ca9f20
	ctx.lr = 0x8327A7F0;
	sub_82CA9F20(ctx, base);
	// 8327A7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A800 size=64
    let mut pc: u32 = 0x8327A800;
    'dispatch: loop {
        match pc {
            0x8327A800 => {
    //   block [0x8327A800..0x8327A840)
	// 8327A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A80C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A814: 388BDEC0  addi r4, r11, -0x2140
	ctx.r[4].s64 = ctx.r[11].s64 + -8512;
	// 8327A818: 386AD8C0  addi r3, r10, -0x2740
	ctx.r[3].s64 = ctx.r[10].s64 + -10048;
	// 8327A81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A820: 4AFB26B1  bl 0x8222ced0
	ctx.lr = 0x8327A824;
	sub_8222CED0(ctx, base);
	// 8327A824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A828: 38690078  addi r3, r9, 0x78
	ctx.r[3].s64 = ctx.r[9].s64 + 120;
	// 8327A82C: 4BA2F6F5  bl 0x82ca9f20
	ctx.lr = 0x8327A830;
	sub_82CA9F20(ctx, base);
	// 8327A830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A840 size=64
    let mut pc: u32 = 0x8327A840;
    'dispatch: loop {
        match pc {
            0x8327A840 => {
    //   block [0x8327A840..0x8327A880)
	// 8327A840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A84C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327A850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A854: 388BDF00  addi r4, r11, -0x2100
	ctx.r[4].s64 = ctx.r[11].s64 + -8448;
	// 8327A858: 386AD8C4  addi r3, r10, -0x273c
	ctx.r[3].s64 = ctx.r[10].s64 + -10044;
	// 8327A85C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A860: 4AFB2671  bl 0x8222ced0
	ctx.lr = 0x8327A864;
	sub_8222CED0(ctx, base);
	// 8327A864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A868: 38690088  addi r3, r9, 0x88
	ctx.r[3].s64 = ctx.r[9].s64 + 136;
	// 8327A86C: 4BA2F6B5  bl 0x82ca9f20
	ctx.lr = 0x8327A870;
	sub_82CA9F20(ctx, base);
	// 8327A870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A880 size=64
    let mut pc: u32 = 0x8327A880;
    'dispatch: loop {
        match pc {
            0x8327A880 => {
    //   block [0x8327A880..0x8327A8C0)
	// 8327A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A88C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327A890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A894: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 8327A898: 386AD8C8  addi r3, r10, -0x2738
	ctx.r[3].s64 = ctx.r[10].s64 + -10040;
	// 8327A89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A8A0: 4AFB2631  bl 0x8222ced0
	ctx.lr = 0x8327A8A4;
	sub_8222CED0(ctx, base);
	// 8327A8A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A8A8: 38690098  addi r3, r9, 0x98
	ctx.r[3].s64 = ctx.r[9].s64 + 152;
	// 8327A8AC: 4BA2F675  bl 0x82ca9f20
	ctx.lr = 0x8327A8B0;
	sub_82CA9F20(ctx, base);
	// 8327A8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A8C0 size=64
    let mut pc: u32 = 0x8327A8C0;
    'dispatch: loop {
        match pc {
            0x8327A8C0 => {
    //   block [0x8327A8C0..0x8327A900)
	// 8327A8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A8CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327A8D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A8D4: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 8327A8D8: 386AD8CC  addi r3, r10, -0x2734
	ctx.r[3].s64 = ctx.r[10].s64 + -10036;
	// 8327A8DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A8E0: 4AFB25F1  bl 0x8222ced0
	ctx.lr = 0x8327A8E4;
	sub_8222CED0(ctx, base);
	// 8327A8E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A8E8: 386900A8  addi r3, r9, 0xa8
	ctx.r[3].s64 = ctx.r[9].s64 + 168;
	// 8327A8EC: 4BA2F635  bl 0x82ca9f20
	ctx.lr = 0x8327A8F0;
	sub_82CA9F20(ctx, base);
	// 8327A8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A900 size=64
    let mut pc: u32 = 0x8327A900;
    'dispatch: loop {
        match pc {
            0x8327A900 => {
    //   block [0x8327A900..0x8327A940)
	// 8327A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A90C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327A910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A914: 388B28E4  addi r4, r11, 0x28e4
	ctx.r[4].s64 = ctx.r[11].s64 + 10468;
	// 8327A918: 386AD8D0  addi r3, r10, -0x2730
	ctx.r[3].s64 = ctx.r[10].s64 + -10032;
	// 8327A91C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A920: 4AFB25B1  bl 0x8222ced0
	ctx.lr = 0x8327A924;
	sub_8222CED0(ctx, base);
	// 8327A924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A928: 386900B8  addi r3, r9, 0xb8
	ctx.r[3].s64 = ctx.r[9].s64 + 184;
	// 8327A92C: 4BA2F5F5  bl 0x82ca9f20
	ctx.lr = 0x8327A930;
	sub_82CA9F20(ctx, base);
	// 8327A930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A940 size=64
    let mut pc: u32 = 0x8327A940;
    'dispatch: loop {
        match pc {
            0x8327A940 => {
    //   block [0x8327A940..0x8327A980)
	// 8327A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A94C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327A950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A954: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8327A958: 386AD8D4  addi r3, r10, -0x272c
	ctx.r[3].s64 = ctx.r[10].s64 + -10028;
	// 8327A95C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A960: 4AFB2571  bl 0x8222ced0
	ctx.lr = 0x8327A964;
	sub_8222CED0(ctx, base);
	// 8327A964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A968: 386900C8  addi r3, r9, 0xc8
	ctx.r[3].s64 = ctx.r[9].s64 + 200;
	// 8327A96C: 4BA2F5B5  bl 0x82ca9f20
	ctx.lr = 0x8327A970;
	sub_82CA9F20(ctx, base);
	// 8327A970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A980 size=64
    let mut pc: u32 = 0x8327A980;
    'dispatch: loop {
        match pc {
            0x8327A980 => {
    //   block [0x8327A980..0x8327A9C0)
	// 8327A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A98C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327A990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A994: 388B28F8  addi r4, r11, 0x28f8
	ctx.r[4].s64 = ctx.r[11].s64 + 10488;
	// 8327A998: 386AD8D8  addi r3, r10, -0x2728
	ctx.r[3].s64 = ctx.r[10].s64 + -10024;
	// 8327A99C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A9A0: 4AFB2531  bl 0x8222ced0
	ctx.lr = 0x8327A9A4;
	sub_8222CED0(ctx, base);
	// 8327A9A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A9A8: 386900D8  addi r3, r9, 0xd8
	ctx.r[3].s64 = ctx.r[9].s64 + 216;
	// 8327A9AC: 4BA2F575  bl 0x82ca9f20
	ctx.lr = 0x8327A9B0;
	sub_82CA9F20(ctx, base);
	// 8327A9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327A9C0 size=64
    let mut pc: u32 = 0x8327A9C0;
    'dispatch: loop {
        match pc {
            0x8327A9C0 => {
    //   block [0x8327A9C0..0x8327AA00)
	// 8327A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327A9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327A9CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327A9D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327A9D4: 388B2904  addi r4, r11, 0x2904
	ctx.r[4].s64 = ctx.r[11].s64 + 10500;
	// 8327A9D8: 386AD8DC  addi r3, r10, -0x2724
	ctx.r[3].s64 = ctx.r[10].s64 + -10020;
	// 8327A9DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327A9E0: 4AFB24F1  bl 0x8222ced0
	ctx.lr = 0x8327A9E4;
	sub_8222CED0(ctx, base);
	// 8327A9E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327A9E8: 386900E8  addi r3, r9, 0xe8
	ctx.r[3].s64 = ctx.r[9].s64 + 232;
	// 8327A9EC: 4BA2F535  bl 0x82ca9f20
	ctx.lr = 0x8327A9F0;
	sub_82CA9F20(ctx, base);
	// 8327A9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327A9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327A9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327A9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AA00 size=64
    let mut pc: u32 = 0x8327AA00;
    'dispatch: loop {
        match pc {
            0x8327AA00 => {
    //   block [0x8327AA00..0x8327AA40)
	// 8327AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AA08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AA0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AA14: 388B2914  addi r4, r11, 0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + 10516;
	// 8327AA18: 386AD8E0  addi r3, r10, -0x2720
	ctx.r[3].s64 = ctx.r[10].s64 + -10016;
	// 8327AA1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AA20: 4AFB24B1  bl 0x8222ced0
	ctx.lr = 0x8327AA24;
	sub_8222CED0(ctx, base);
	// 8327AA24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AA28: 386900F8  addi r3, r9, 0xf8
	ctx.r[3].s64 = ctx.r[9].s64 + 248;
	// 8327AA2C: 4BA2F4F5  bl 0x82ca9f20
	ctx.lr = 0x8327AA30;
	sub_82CA9F20(ctx, base);
	// 8327AA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AA40 size=64
    let mut pc: u32 = 0x8327AA40;
    'dispatch: loop {
        match pc {
            0x8327AA40 => {
    //   block [0x8327AA40..0x8327AA80)
	// 8327AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AA4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AA50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AA54: 388B291C  addi r4, r11, 0x291c
	ctx.r[4].s64 = ctx.r[11].s64 + 10524;
	// 8327AA58: 386AD8E4  addi r3, r10, -0x271c
	ctx.r[3].s64 = ctx.r[10].s64 + -10012;
	// 8327AA5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AA60: 4AFB2471  bl 0x8222ced0
	ctx.lr = 0x8327AA64;
	sub_8222CED0(ctx, base);
	// 8327AA64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AA68: 38690108  addi r3, r9, 0x108
	ctx.r[3].s64 = ctx.r[9].s64 + 264;
	// 8327AA6C: 4BA2F4B5  bl 0x82ca9f20
	ctx.lr = 0x8327AA70;
	sub_82CA9F20(ctx, base);
	// 8327AA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AA80 size=64
    let mut pc: u32 = 0x8327AA80;
    'dispatch: loop {
        match pc {
            0x8327AA80 => {
    //   block [0x8327AA80..0x8327AAC0)
	// 8327AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AA8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AA94: 388B292C  addi r4, r11, 0x292c
	ctx.r[4].s64 = ctx.r[11].s64 + 10540;
	// 8327AA98: 386AD8E8  addi r3, r10, -0x2718
	ctx.r[3].s64 = ctx.r[10].s64 + -10008;
	// 8327AA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AAA0: 4AFB2431  bl 0x8222ced0
	ctx.lr = 0x8327AAA4;
	sub_8222CED0(ctx, base);
	// 8327AAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AAA8: 38690118  addi r3, r9, 0x118
	ctx.r[3].s64 = ctx.r[9].s64 + 280;
	// 8327AAAC: 4BA2F475  bl 0x82ca9f20
	ctx.lr = 0x8327AAB0;
	sub_82CA9F20(ctx, base);
	// 8327AAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AAC0 size=64
    let mut pc: u32 = 0x8327AAC0;
    'dispatch: loop {
        match pc {
            0x8327AAC0 => {
    //   block [0x8327AAC0..0x8327AB00)
	// 8327AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AACC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AAD4: 388B2934  addi r4, r11, 0x2934
	ctx.r[4].s64 = ctx.r[11].s64 + 10548;
	// 8327AAD8: 386AD8EC  addi r3, r10, -0x2714
	ctx.r[3].s64 = ctx.r[10].s64 + -10004;
	// 8327AADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AAE0: 4AFB23F1  bl 0x8222ced0
	ctx.lr = 0x8327AAE4;
	sub_8222CED0(ctx, base);
	// 8327AAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AAE8: 38690128  addi r3, r9, 0x128
	ctx.r[3].s64 = ctx.r[9].s64 + 296;
	// 8327AAEC: 4BA2F435  bl 0x82ca9f20
	ctx.lr = 0x8327AAF0;
	sub_82CA9F20(ctx, base);
	// 8327AAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AB00 size=64
    let mut pc: u32 = 0x8327AB00;
    'dispatch: loop {
        match pc {
            0x8327AB00 => {
    //   block [0x8327AB00..0x8327AB40)
	// 8327AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AB0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AB14: 388B293C  addi r4, r11, 0x293c
	ctx.r[4].s64 = ctx.r[11].s64 + 10556;
	// 8327AB18: 386AD8F0  addi r3, r10, -0x2710
	ctx.r[3].s64 = ctx.r[10].s64 + -10000;
	// 8327AB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AB20: 4AFB23B1  bl 0x8222ced0
	ctx.lr = 0x8327AB24;
	sub_8222CED0(ctx, base);
	// 8327AB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AB28: 38690138  addi r3, r9, 0x138
	ctx.r[3].s64 = ctx.r[9].s64 + 312;
	// 8327AB2C: 4BA2F3F5  bl 0x82ca9f20
	ctx.lr = 0x8327AB30;
	sub_82CA9F20(ctx, base);
	// 8327AB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AB40 size=64
    let mut pc: u32 = 0x8327AB40;
    'dispatch: loop {
        match pc {
            0x8327AB40 => {
    //   block [0x8327AB40..0x8327AB80)
	// 8327AB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AB4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AB50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AB54: 388B2948  addi r4, r11, 0x2948
	ctx.r[4].s64 = ctx.r[11].s64 + 10568;
	// 8327AB58: 386AD8F4  addi r3, r10, -0x270c
	ctx.r[3].s64 = ctx.r[10].s64 + -9996;
	// 8327AB5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AB60: 4AFB2371  bl 0x8222ced0
	ctx.lr = 0x8327AB64;
	sub_8222CED0(ctx, base);
	// 8327AB64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AB68: 38690148  addi r3, r9, 0x148
	ctx.r[3].s64 = ctx.r[9].s64 + 328;
	// 8327AB6C: 4BA2F3B5  bl 0x82ca9f20
	ctx.lr = 0x8327AB70;
	sub_82CA9F20(ctx, base);
	// 8327AB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AB80 size=64
    let mut pc: u32 = 0x8327AB80;
    'dispatch: loop {
        match pc {
            0x8327AB80 => {
    //   block [0x8327AB80..0x8327ABC0)
	// 8327AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AB8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AB90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AB94: 388B2950  addi r4, r11, 0x2950
	ctx.r[4].s64 = ctx.r[11].s64 + 10576;
	// 8327AB98: 386AD8F8  addi r3, r10, -0x2708
	ctx.r[3].s64 = ctx.r[10].s64 + -9992;
	// 8327AB9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ABA0: 4AFB2331  bl 0x8222ced0
	ctx.lr = 0x8327ABA4;
	sub_8222CED0(ctx, base);
	// 8327ABA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ABA8: 38690158  addi r3, r9, 0x158
	ctx.r[3].s64 = ctx.r[9].s64 + 344;
	// 8327ABAC: 4BA2F375  bl 0x82ca9f20
	ctx.lr = 0x8327ABB0;
	sub_82CA9F20(ctx, base);
	// 8327ABB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ABB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ABB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ABBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ABC0 size=64
    let mut pc: u32 = 0x8327ABC0;
    'dispatch: loop {
        match pc {
            0x8327ABC0 => {
    //   block [0x8327ABC0..0x8327AC00)
	// 8327ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ABC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ABCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327ABD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ABD4: 388B2958  addi r4, r11, 0x2958
	ctx.r[4].s64 = ctx.r[11].s64 + 10584;
	// 8327ABD8: 386AD8FC  addi r3, r10, -0x2704
	ctx.r[3].s64 = ctx.r[10].s64 + -9988;
	// 8327ABDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ABE0: 4AFB22F1  bl 0x8222ced0
	ctx.lr = 0x8327ABE4;
	sub_8222CED0(ctx, base);
	// 8327ABE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ABE8: 38690168  addi r3, r9, 0x168
	ctx.r[3].s64 = ctx.r[9].s64 + 360;
	// 8327ABEC: 4BA2F335  bl 0x82ca9f20
	ctx.lr = 0x8327ABF0;
	sub_82CA9F20(ctx, base);
	// 8327ABF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ABF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ABF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ABFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AC00 size=64
    let mut pc: u32 = 0x8327AC00;
    'dispatch: loop {
        match pc {
            0x8327AC00 => {
    //   block [0x8327AC00..0x8327AC40)
	// 8327AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AC0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AC10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AC14: 388B2960  addi r4, r11, 0x2960
	ctx.r[4].s64 = ctx.r[11].s64 + 10592;
	// 8327AC18: 386AD900  addi r3, r10, -0x2700
	ctx.r[3].s64 = ctx.r[10].s64 + -9984;
	// 8327AC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AC20: 4AFB22B1  bl 0x8222ced0
	ctx.lr = 0x8327AC24;
	sub_8222CED0(ctx, base);
	// 8327AC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AC28: 38690178  addi r3, r9, 0x178
	ctx.r[3].s64 = ctx.r[9].s64 + 376;
	// 8327AC2C: 4BA2F2F5  bl 0x82ca9f20
	ctx.lr = 0x8327AC30;
	sub_82CA9F20(ctx, base);
	// 8327AC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AC40 size=64
    let mut pc: u32 = 0x8327AC40;
    'dispatch: loop {
        match pc {
            0x8327AC40 => {
    //   block [0x8327AC40..0x8327AC80)
	// 8327AC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AC4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AC50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AC54: 388B2968  addi r4, r11, 0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + 10600;
	// 8327AC58: 386AD904  addi r3, r10, -0x26fc
	ctx.r[3].s64 = ctx.r[10].s64 + -9980;
	// 8327AC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AC60: 4AFB2271  bl 0x8222ced0
	ctx.lr = 0x8327AC64;
	sub_8222CED0(ctx, base);
	// 8327AC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AC68: 38690188  addi r3, r9, 0x188
	ctx.r[3].s64 = ctx.r[9].s64 + 392;
	// 8327AC6C: 4BA2F2B5  bl 0x82ca9f20
	ctx.lr = 0x8327AC70;
	sub_82CA9F20(ctx, base);
	// 8327AC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AC80 size=64
    let mut pc: u32 = 0x8327AC80;
    'dispatch: loop {
        match pc {
            0x8327AC80 => {
    //   block [0x8327AC80..0x8327ACC0)
	// 8327AC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AC8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AC94: 388B2970  addi r4, r11, 0x2970
	ctx.r[4].s64 = ctx.r[11].s64 + 10608;
	// 8327AC98: 386AD908  addi r3, r10, -0x26f8
	ctx.r[3].s64 = ctx.r[10].s64 + -9976;
	// 8327AC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ACA0: 4AFB2231  bl 0x8222ced0
	ctx.lr = 0x8327ACA4;
	sub_8222CED0(ctx, base);
	// 8327ACA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ACA8: 38690198  addi r3, r9, 0x198
	ctx.r[3].s64 = ctx.r[9].s64 + 408;
	// 8327ACAC: 4BA2F275  bl 0x82ca9f20
	ctx.lr = 0x8327ACB0;
	sub_82CA9F20(ctx, base);
	// 8327ACB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ACC0 size=64
    let mut pc: u32 = 0x8327ACC0;
    'dispatch: loop {
        match pc {
            0x8327ACC0 => {
    //   block [0x8327ACC0..0x8327AD00)
	// 8327ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ACC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ACCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327ACD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ACD4: 388B2978  addi r4, r11, 0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + 10616;
	// 8327ACD8: 386AD90C  addi r3, r10, -0x26f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9972;
	// 8327ACDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ACE0: 4AFB21F1  bl 0x8222ced0
	ctx.lr = 0x8327ACE4;
	sub_8222CED0(ctx, base);
	// 8327ACE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ACE8: 386901A8  addi r3, r9, 0x1a8
	ctx.r[3].s64 = ctx.r[9].s64 + 424;
	// 8327ACEC: 4BA2F235  bl 0x82ca9f20
	ctx.lr = 0x8327ACF0;
	sub_82CA9F20(ctx, base);
	// 8327ACF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ACF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ACF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ACFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AD00 size=64
    let mut pc: u32 = 0x8327AD00;
    'dispatch: loop {
        match pc {
            0x8327AD00 => {
    //   block [0x8327AD00..0x8327AD40)
	// 8327AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AD0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AD10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AD14: 388B2980  addi r4, r11, 0x2980
	ctx.r[4].s64 = ctx.r[11].s64 + 10624;
	// 8327AD18: 386AD910  addi r3, r10, -0x26f0
	ctx.r[3].s64 = ctx.r[10].s64 + -9968;
	// 8327AD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AD20: 4AFB21B1  bl 0x8222ced0
	ctx.lr = 0x8327AD24;
	sub_8222CED0(ctx, base);
	// 8327AD24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AD28: 386901B8  addi r3, r9, 0x1b8
	ctx.r[3].s64 = ctx.r[9].s64 + 440;
	// 8327AD2C: 4BA2F1F5  bl 0x82ca9f20
	ctx.lr = 0x8327AD30;
	sub_82CA9F20(ctx, base);
	// 8327AD30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AD40 size=64
    let mut pc: u32 = 0x8327AD40;
    'dispatch: loop {
        match pc {
            0x8327AD40 => {
    //   block [0x8327AD40..0x8327AD80)
	// 8327AD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AD4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AD50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AD54: 388B298C  addi r4, r11, 0x298c
	ctx.r[4].s64 = ctx.r[11].s64 + 10636;
	// 8327AD58: 386AD914  addi r3, r10, -0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9964;
	// 8327AD5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AD60: 4AFB2171  bl 0x8222ced0
	ctx.lr = 0x8327AD64;
	sub_8222CED0(ctx, base);
	// 8327AD64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AD68: 386901C8  addi r3, r9, 0x1c8
	ctx.r[3].s64 = ctx.r[9].s64 + 456;
	// 8327AD6C: 4BA2F1B5  bl 0x82ca9f20
	ctx.lr = 0x8327AD70;
	sub_82CA9F20(ctx, base);
	// 8327AD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AD80 size=64
    let mut pc: u32 = 0x8327AD80;
    'dispatch: loop {
        match pc {
            0x8327AD80 => {
    //   block [0x8327AD80..0x8327ADC0)
	// 8327AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AD8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AD94: 388B8CD0  addi r4, r11, -0x7330
	ctx.r[4].s64 = ctx.r[11].s64 + -29488;
	// 8327AD98: 386AD918  addi r3, r10, -0x26e8
	ctx.r[3].s64 = ctx.r[10].s64 + -9960;
	// 8327AD9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ADA0: 4AFB2131  bl 0x8222ced0
	ctx.lr = 0x8327ADA4;
	sub_8222CED0(ctx, base);
	// 8327ADA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ADA8: 386901D8  addi r3, r9, 0x1d8
	ctx.r[3].s64 = ctx.r[9].s64 + 472;
	// 8327ADAC: 4BA2F175  bl 0x82ca9f20
	ctx.lr = 0x8327ADB0;
	sub_82CA9F20(ctx, base);
	// 8327ADB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ADB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ADB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ADBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ADC0 size=64
    let mut pc: u32 = 0x8327ADC0;
    'dispatch: loop {
        match pc {
            0x8327ADC0 => {
    //   block [0x8327ADC0..0x8327AE00)
	// 8327ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ADC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ADCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ADD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ADD4: 388BDF94  addi r4, r11, -0x206c
	ctx.r[4].s64 = ctx.r[11].s64 + -8300;
	// 8327ADD8: 386AD91C  addi r3, r10, -0x26e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9956;
	// 8327ADDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ADE0: 4AFB20F1  bl 0x8222ced0
	ctx.lr = 0x8327ADE4;
	sub_8222CED0(ctx, base);
	// 8327ADE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ADE8: 386901E8  addi r3, r9, 0x1e8
	ctx.r[3].s64 = ctx.r[9].s64 + 488;
	// 8327ADEC: 4BA2F135  bl 0x82ca9f20
	ctx.lr = 0x8327ADF0;
	sub_82CA9F20(ctx, base);
	// 8327ADF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ADF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ADF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ADFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AE00 size=64
    let mut pc: u32 = 0x8327AE00;
    'dispatch: loop {
        match pc {
            0x8327AE00 => {
    //   block [0x8327AE00..0x8327AE40)
	// 8327AE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AE0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327AE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AE14: 388BDFA4  addi r4, r11, -0x205c
	ctx.r[4].s64 = ctx.r[11].s64 + -8284;
	// 8327AE18: 386AD920  addi r3, r10, -0x26e0
	ctx.r[3].s64 = ctx.r[10].s64 + -9952;
	// 8327AE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AE20: 4AFB20B1  bl 0x8222ced0
	ctx.lr = 0x8327AE24;
	sub_8222CED0(ctx, base);
	// 8327AE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AE28: 386901F8  addi r3, r9, 0x1f8
	ctx.r[3].s64 = ctx.r[9].s64 + 504;
	// 8327AE2C: 4BA2F0F5  bl 0x82ca9f20
	ctx.lr = 0x8327AE30;
	sub_82CA9F20(ctx, base);
	// 8327AE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AE40 size=64
    let mut pc: u32 = 0x8327AE40;
    'dispatch: loop {
        match pc {
            0x8327AE40 => {
    //   block [0x8327AE40..0x8327AE80)
	// 8327AE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AE4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327AE50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AE54: 388BDFB0  addi r4, r11, -0x2050
	ctx.r[4].s64 = ctx.r[11].s64 + -8272;
	// 8327AE58: 386AD924  addi r3, r10, -0x26dc
	ctx.r[3].s64 = ctx.r[10].s64 + -9948;
	// 8327AE5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AE60: 4AFB2071  bl 0x8222ced0
	ctx.lr = 0x8327AE64;
	sub_8222CED0(ctx, base);
	// 8327AE64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AE68: 38690208  addi r3, r9, 0x208
	ctx.r[3].s64 = ctx.r[9].s64 + 520;
	// 8327AE6C: 4BA2F0B5  bl 0x82ca9f20
	ctx.lr = 0x8327AE70;
	sub_82CA9F20(ctx, base);
	// 8327AE70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AE80 size=64
    let mut pc: u32 = 0x8327AE80;
    'dispatch: loop {
        match pc {
            0x8327AE80 => {
    //   block [0x8327AE80..0x8327AEC0)
	// 8327AE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AE88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AE8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327AE90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AE94: 388BDFBC  addi r4, r11, -0x2044
	ctx.r[4].s64 = ctx.r[11].s64 + -8260;
	// 8327AE98: 386AD928  addi r3, r10, -0x26d8
	ctx.r[3].s64 = ctx.r[10].s64 + -9944;
	// 8327AE9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AEA0: 4AFB2031  bl 0x8222ced0
	ctx.lr = 0x8327AEA4;
	sub_8222CED0(ctx, base);
	// 8327AEA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AEA8: 38690218  addi r3, r9, 0x218
	ctx.r[3].s64 = ctx.r[9].s64 + 536;
	// 8327AEAC: 4BA2F075  bl 0x82ca9f20
	ctx.lr = 0x8327AEB0;
	sub_82CA9F20(ctx, base);
	// 8327AEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AEC0 size=64
    let mut pc: u32 = 0x8327AEC0;
    'dispatch: loop {
        match pc {
            0x8327AEC0 => {
    //   block [0x8327AEC0..0x8327AF00)
	// 8327AEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AEC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AECC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327AED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AED4: 388BDFC4  addi r4, r11, -0x203c
	ctx.r[4].s64 = ctx.r[11].s64 + -8252;
	// 8327AED8: 386AD92C  addi r3, r10, -0x26d4
	ctx.r[3].s64 = ctx.r[10].s64 + -9940;
	// 8327AEDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AEE0: 4AFB1FF1  bl 0x8222ced0
	ctx.lr = 0x8327AEE4;
	sub_8222CED0(ctx, base);
	// 8327AEE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AEE8: 38690228  addi r3, r9, 0x228
	ctx.r[3].s64 = ctx.r[9].s64 + 552;
	// 8327AEEC: 4BA2F035  bl 0x82ca9f20
	ctx.lr = 0x8327AEF0;
	sub_82CA9F20(ctx, base);
	// 8327AEF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AF00 size=64
    let mut pc: u32 = 0x8327AF00;
    'dispatch: loop {
        match pc {
            0x8327AF00 => {
    //   block [0x8327AF00..0x8327AF40)
	// 8327AF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AF08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AF0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327AF10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AF14: 388B29F4  addi r4, r11, 0x29f4
	ctx.r[4].s64 = ctx.r[11].s64 + 10740;
	// 8327AF18: 386AD930  addi r3, r10, -0x26d0
	ctx.r[3].s64 = ctx.r[10].s64 + -9936;
	// 8327AF1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327AF20: 4AFB1FB1  bl 0x8222ced0
	ctx.lr = 0x8327AF24;
	sub_8222CED0(ctx, base);
	// 8327AF24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327AF28: 38690238  addi r3, r9, 0x238
	ctx.r[3].s64 = ctx.r[9].s64 + 568;
	// 8327AF2C: 4BA2EFF5  bl 0x82ca9f20
	ctx.lr = 0x8327AF30;
	sub_82CA9F20(ctx, base);
	// 8327AF30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AF40 size=56
    let mut pc: u32 = 0x8327AF40;
    'dispatch: loop {
        match pc {
            0x8327AF40 => {
    //   block [0x8327AF40..0x8327AF78)
	// 8327AF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AF48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AF4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327AF50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327AF54: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327AF58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327AF5C: 4AF78DFD  bl 0x821f3d58
	ctx.lr = 0x8327AF60;
	sub_821F3D58(ctx, base);
	// 8327AF60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AF64: 906AD934  stw r3, -0x26cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9932 as u32), ctx.r[3].u32 ) };
	// 8327AF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AF78 size=56
    let mut pc: u32 = 0x8327AF78;
    'dispatch: loop {
        match pc {
            0x8327AF78 => {
    //   block [0x8327AF78..0x8327AFB0)
	// 8327AF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AF84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327AF88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327AF8C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327AF90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327AF94: 4AF78DC5  bl 0x821f3d58
	ctx.lr = 0x8327AF98;
	sub_821F3D58(ctx, base);
	// 8327AF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AF9C: 906AD938  stw r3, -0x26c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9928 as u32), ctx.r[3].u32 ) };
	// 8327AFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AFB0 size=56
    let mut pc: u32 = 0x8327AFB0;
    'dispatch: loop {
        match pc {
            0x8327AFB0 => {
    //   block [0x8327AFB0..0x8327AFE8)
	// 8327AFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AFB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AFBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327AFC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327AFC4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327AFC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327AFCC: 4AF78D8D  bl 0x821f3d58
	ctx.lr = 0x8327AFD0;
	sub_821F3D58(ctx, base);
	// 8327AFD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327AFD4: 906AD93C  stw r3, -0x26c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9924 as u32), ctx.r[3].u32 ) };
	// 8327AFD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327AFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327AFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327AFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327AFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327AFE8 size=56
    let mut pc: u32 = 0x8327AFE8;
    'dispatch: loop {
        match pc {
            0x8327AFE8 => {
    //   block [0x8327AFE8..0x8327B020)
	// 8327AFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327AFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327AFF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327AFF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327AFF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327AFFC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327B000: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B004: 4AF78D55  bl 0x821f3d58
	ctx.lr = 0x8327B008;
	sub_821F3D58(ctx, base);
	// 8327B008: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B00C: 906AD940  stw r3, -0x26c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9920 as u32), ctx.r[3].u32 ) };
	// 8327B010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B020 size=56
    let mut pc: u32 = 0x8327B020;
    'dispatch: loop {
        match pc {
            0x8327B020 => {
    //   block [0x8327B020..0x8327B058)
	// 8327B020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B02C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B030: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B034: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327B038: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B03C: 4AF78D1D  bl 0x821f3d58
	ctx.lr = 0x8327B040;
	sub_821F3D58(ctx, base);
	// 8327B040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B044: 906AD944  stw r3, -0x26bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9916 as u32), ctx.r[3].u32 ) };
	// 8327B048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B058 size=56
    let mut pc: u32 = 0x8327B058;
    'dispatch: loop {
        match pc {
            0x8327B058 => {
    //   block [0x8327B058..0x8327B090)
	// 8327B058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B064: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B068: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B06C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327B070: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B074: 4AF78CE5  bl 0x821f3d58
	ctx.lr = 0x8327B078;
	sub_821F3D58(ctx, base);
	// 8327B078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B07C: 906AD948  stw r3, -0x26b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9912 as u32), ctx.r[3].u32 ) };
	// 8327B080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B090 size=56
    let mut pc: u32 = 0x8327B090;
    'dispatch: loop {
        match pc {
            0x8327B090 => {
    //   block [0x8327B090..0x8327B0C8)
	// 8327B090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B09C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B0A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B0A4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327B0A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B0AC: 4AF78CAD  bl 0x821f3d58
	ctx.lr = 0x8327B0B0;
	sub_821F3D58(ctx, base);
	// 8327B0B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B0B4: 906AD94C  stw r3, -0x26b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9908 as u32), ctx.r[3].u32 ) };
	// 8327B0B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B0C8 size=56
    let mut pc: u32 = 0x8327B0C8;
    'dispatch: loop {
        match pc {
            0x8327B0C8 => {
    //   block [0x8327B0C8..0x8327B100)
	// 8327B0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B0D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B0D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B0D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B0DC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327B0E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B0E4: 4AF78C75  bl 0x821f3d58
	ctx.lr = 0x8327B0E8;
	sub_821F3D58(ctx, base);
	// 8327B0E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B0EC: 906AD950  stw r3, -0x26b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9904 as u32), ctx.r[3].u32 ) };
	// 8327B0F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B100 size=56
    let mut pc: u32 = 0x8327B100;
    'dispatch: loop {
        match pc {
            0x8327B100 => {
    //   block [0x8327B100..0x8327B138)
	// 8327B100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B10C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B110: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B114: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327B118: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B11C: 4AF78C3D  bl 0x821f3d58
	ctx.lr = 0x8327B120;
	sub_821F3D58(ctx, base);
	// 8327B120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B124: 906AD954  stw r3, -0x26ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9900 as u32), ctx.r[3].u32 ) };
	// 8327B128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B138 size=56
    let mut pc: u32 = 0x8327B138;
    'dispatch: loop {
        match pc {
            0x8327B138 => {
    //   block [0x8327B138..0x8327B170)
	// 8327B138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B148: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B14C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327B150: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B154: 4AF78C05  bl 0x821f3d58
	ctx.lr = 0x8327B158;
	sub_821F3D58(ctx, base);
	// 8327B158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B15C: 906AD958  stw r3, -0x26a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9896 as u32), ctx.r[3].u32 ) };
	// 8327B160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B170 size=56
    let mut pc: u32 = 0x8327B170;
    'dispatch: loop {
        match pc {
            0x8327B170 => {
    //   block [0x8327B170..0x8327B1A8)
	// 8327B170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B17C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B180: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B184: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327B188: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B18C: 4AF78BCD  bl 0x821f3d58
	ctx.lr = 0x8327B190;
	sub_821F3D58(ctx, base);
	// 8327B190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B194: 906AD95C  stw r3, -0x26a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9892 as u32), ctx.r[3].u32 ) };
	// 8327B198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B1A8 size=56
    let mut pc: u32 = 0x8327B1A8;
    'dispatch: loop {
        match pc {
            0x8327B1A8 => {
    //   block [0x8327B1A8..0x8327B1E0)
	// 8327B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B1B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B1B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B1B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B1BC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327B1C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B1C4: 4AF78B95  bl 0x821f3d58
	ctx.lr = 0x8327B1C8;
	sub_821F3D58(ctx, base);
	// 8327B1C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B1CC: 906AD960  stw r3, -0x26a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9888 as u32), ctx.r[3].u32 ) };
	// 8327B1D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B1E0 size=56
    let mut pc: u32 = 0x8327B1E0;
    'dispatch: loop {
        match pc {
            0x8327B1E0 => {
    //   block [0x8327B1E0..0x8327B218)
	// 8327B1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B1E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B1EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B1F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B1F4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327B1F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B1FC: 4AF78B5D  bl 0x821f3d58
	ctx.lr = 0x8327B200;
	sub_821F3D58(ctx, base);
	// 8327B200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B204: 906AD964  stw r3, -0x269c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9884 as u32), ctx.r[3].u32 ) };
	// 8327B208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B218 size=56
    let mut pc: u32 = 0x8327B218;
    'dispatch: loop {
        match pc {
            0x8327B218 => {
    //   block [0x8327B218..0x8327B250)
	// 8327B218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B224: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B228: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B22C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327B230: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B234: 4AF78B25  bl 0x821f3d58
	ctx.lr = 0x8327B238;
	sub_821F3D58(ctx, base);
	// 8327B238: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B23C: 906AD968  stw r3, -0x2698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9880 as u32), ctx.r[3].u32 ) };
	// 8327B240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B250 size=56
    let mut pc: u32 = 0x8327B250;
    'dispatch: loop {
        match pc {
            0x8327B250 => {
    //   block [0x8327B250..0x8327B288)
	// 8327B250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B25C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B260: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B264: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327B268: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B26C: 4AF78AED  bl 0x821f3d58
	ctx.lr = 0x8327B270;
	sub_821F3D58(ctx, base);
	// 8327B270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B274: 906AD96C  stw r3, -0x2694(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9876 as u32), ctx.r[3].u32 ) };
	// 8327B278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B288 size=56
    let mut pc: u32 = 0x8327B288;
    'dispatch: loop {
        match pc {
            0x8327B288 => {
    //   block [0x8327B288..0x8327B2C0)
	// 8327B288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B298: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B29C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327B2A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B2A4: 4AF78AB5  bl 0x821f3d58
	ctx.lr = 0x8327B2A8;
	sub_821F3D58(ctx, base);
	// 8327B2A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B2AC: 906AD970  stw r3, -0x2690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9872 as u32), ctx.r[3].u32 ) };
	// 8327B2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B2C0 size=56
    let mut pc: u32 = 0x8327B2C0;
    'dispatch: loop {
        match pc {
            0x8327B2C0 => {
    //   block [0x8327B2C0..0x8327B2F8)
	// 8327B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B2C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B2CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B2D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B2D4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327B2D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B2DC: 4AF78A7D  bl 0x821f3d58
	ctx.lr = 0x8327B2E0;
	sub_821F3D58(ctx, base);
	// 8327B2E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B2E4: 906AD974  stw r3, -0x268c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9868 as u32), ctx.r[3].u32 ) };
	// 8327B2E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B2F8 size=56
    let mut pc: u32 = 0x8327B2F8;
    'dispatch: loop {
        match pc {
            0x8327B2F8 => {
    //   block [0x8327B2F8..0x8327B330)
	// 8327B2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B308: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B30C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327B310: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B314: 4AF78A45  bl 0x821f3d58
	ctx.lr = 0x8327B318;
	sub_821F3D58(ctx, base);
	// 8327B318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B31C: 906AD978  stw r3, -0x2688(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9864 as u32), ctx.r[3].u32 ) };
	// 8327B320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B330 size=56
    let mut pc: u32 = 0x8327B330;
    'dispatch: loop {
        match pc {
            0x8327B330 => {
    //   block [0x8327B330..0x8327B368)
	// 8327B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B33C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B340: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B344: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327B348: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B34C: 4AF78A0D  bl 0x821f3d58
	ctx.lr = 0x8327B350;
	sub_821F3D58(ctx, base);
	// 8327B350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B354: 906AD97C  stw r3, -0x2684(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9860 as u32), ctx.r[3].u32 ) };
	// 8327B358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B368 size=56
    let mut pc: u32 = 0x8327B368;
    'dispatch: loop {
        match pc {
            0x8327B368 => {
    //   block [0x8327B368..0x8327B3A0)
	// 8327B368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B378: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B37C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327B380: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B384: 4AF789D5  bl 0x821f3d58
	ctx.lr = 0x8327B388;
	sub_821F3D58(ctx, base);
	// 8327B388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B38C: 906AD980  stw r3, -0x2680(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9856 as u32), ctx.r[3].u32 ) };
	// 8327B390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


