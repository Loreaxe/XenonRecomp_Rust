pub fn sub_8325E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E830 size=56
    let mut pc: u32 = 0x8325E830;
    'dispatch: loop {
        match pc {
            0x8325E830 => {
    //   block [0x8325E830..0x8325E868)
	// 8325E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E844: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E84C: 4AF9550D  bl 0x821f3d58
	ctx.lr = 0x8325E850;
	sub_821F3D58(ctx, base);
	// 8325E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E854: 906AAEB8  stw r3, -0x5148(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20808 as u32), ctx.r[3].u32 ) };
	// 8325E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E868 size=56
    let mut pc: u32 = 0x8325E868;
    'dispatch: loop {
        match pc {
            0x8325E868 => {
    //   block [0x8325E868..0x8325E8A0)
	// 8325E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E874: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E87C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E884: 4AF954D5  bl 0x821f3d58
	ctx.lr = 0x8325E888;
	sub_821F3D58(ctx, base);
	// 8325E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E88C: 906AAEBC  stw r3, -0x5144(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20804 as u32), ctx.r[3].u32 ) };
	// 8325E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E8A0 size=56
    let mut pc: u32 = 0x8325E8A0;
    'dispatch: loop {
        match pc {
            0x8325E8A0 => {
    //   block [0x8325E8A0..0x8325E8D8)
	// 8325E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E8AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E8B4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E8BC: 4AF9549D  bl 0x821f3d58
	ctx.lr = 0x8325E8C0;
	sub_821F3D58(ctx, base);
	// 8325E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E8C4: 906AAEC0  stw r3, -0x5140(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20800 as u32), ctx.r[3].u32 ) };
	// 8325E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E8D8 size=56
    let mut pc: u32 = 0x8325E8D8;
    'dispatch: loop {
        match pc {
            0x8325E8D8 => {
    //   block [0x8325E8D8..0x8325E910)
	// 8325E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E8E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E8EC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E8F4: 4AF95465  bl 0x821f3d58
	ctx.lr = 0x8325E8F8;
	sub_821F3D58(ctx, base);
	// 8325E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E8FC: 906AAEC4  stw r3, -0x513c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20796 as u32), ctx.r[3].u32 ) };
	// 8325E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E910 size=56
    let mut pc: u32 = 0x8325E910;
    'dispatch: loop {
        match pc {
            0x8325E910 => {
    //   block [0x8325E910..0x8325E948)
	// 8325E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E91C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E924: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E92C: 4AF9542D  bl 0x821f3d58
	ctx.lr = 0x8325E930;
	sub_821F3D58(ctx, base);
	// 8325E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E934: 906AAEC8  stw r3, -0x5138(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20792 as u32), ctx.r[3].u32 ) };
	// 8325E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E948 size=56
    let mut pc: u32 = 0x8325E948;
    'dispatch: loop {
        match pc {
            0x8325E948 => {
    //   block [0x8325E948..0x8325E980)
	// 8325E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E95C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E964: 4AF953F5  bl 0x821f3d58
	ctx.lr = 0x8325E968;
	sub_821F3D58(ctx, base);
	// 8325E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E96C: 906AAECC  stw r3, -0x5134(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20788 as u32), ctx.r[3].u32 ) };
	// 8325E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E980 size=56
    let mut pc: u32 = 0x8325E980;
    'dispatch: loop {
        match pc {
            0x8325E980 => {
    //   block [0x8325E980..0x8325E9B8)
	// 8325E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E98C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E994: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E99C: 4AF953BD  bl 0x821f3d58
	ctx.lr = 0x8325E9A0;
	sub_821F3D58(ctx, base);
	// 8325E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E9A4: 906AAED0  stw r3, -0x5130(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20784 as u32), ctx.r[3].u32 ) };
	// 8325E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E9B8 size=56
    let mut pc: u32 = 0x8325E9B8;
    'dispatch: loop {
        match pc {
            0x8325E9B8 => {
    //   block [0x8325E9B8..0x8325E9F0)
	// 8325E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E9C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E9CC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325E9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E9D4: 4AF95385  bl 0x821f3d58
	ctx.lr = 0x8325E9D8;
	sub_821F3D58(ctx, base);
	// 8325E9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E9DC: 906AAED4  stw r3, -0x512c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20780 as u32), ctx.r[3].u32 ) };
	// 8325E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E9F0 size=56
    let mut pc: u32 = 0x8325E9F0;
    'dispatch: loop {
        match pc {
            0x8325E9F0 => {
    //   block [0x8325E9F0..0x8325EA28)
	// 8325E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E9FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EA04: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325EA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EA0C: 4AF9534D  bl 0x821f3d58
	ctx.lr = 0x8325EA10;
	sub_821F3D58(ctx, base);
	// 8325EA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EA14: 906AAED8  stw r3, -0x5128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20776 as u32), ctx.r[3].u32 ) };
	// 8325EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EA28 size=56
    let mut pc: u32 = 0x8325EA28;
    'dispatch: loop {
        match pc {
            0x8325EA28 => {
    //   block [0x8325EA28..0x8325EA60)
	// 8325EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EA34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EA3C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325EA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EA44: 4AF95315  bl 0x821f3d58
	ctx.lr = 0x8325EA48;
	sub_821F3D58(ctx, base);
	// 8325EA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EA4C: 906AAEDC  stw r3, -0x5124(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20772 as u32), ctx.r[3].u32 ) };
	// 8325EA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EA60 size=56
    let mut pc: u32 = 0x8325EA60;
    'dispatch: loop {
        match pc {
            0x8325EA60 => {
    //   block [0x8325EA60..0x8325EA98)
	// 8325EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EA6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EA74: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325EA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EA7C: 4AF952DD  bl 0x821f3d58
	ctx.lr = 0x8325EA80;
	sub_821F3D58(ctx, base);
	// 8325EA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EA84: 906AAEE0  stw r3, -0x5120(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20768 as u32), ctx.r[3].u32 ) };
	// 8325EA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EA98 size=56
    let mut pc: u32 = 0x8325EA98;
    'dispatch: loop {
        match pc {
            0x8325EA98 => {
    //   block [0x8325EA98..0x8325EAD0)
	// 8325EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EAA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EAAC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325EAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EAB4: 4AF952A5  bl 0x821f3d58
	ctx.lr = 0x8325EAB8;
	sub_821F3D58(ctx, base);
	// 8325EAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EABC: 906AAEE4  stw r3, -0x511c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20764 as u32), ctx.r[3].u32 ) };
	// 8325EAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EAD0 size=56
    let mut pc: u32 = 0x8325EAD0;
    'dispatch: loop {
        match pc {
            0x8325EAD0 => {
    //   block [0x8325EAD0..0x8325EB08)
	// 8325EAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EADC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EAE4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325EAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EAEC: 4AF9526D  bl 0x821f3d58
	ctx.lr = 0x8325EAF0;
	sub_821F3D58(ctx, base);
	// 8325EAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EAF4: 906AAEE8  stw r3, -0x5118(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20760 as u32), ctx.r[3].u32 ) };
	// 8325EAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EB08 size=56
    let mut pc: u32 = 0x8325EB08;
    'dispatch: loop {
        match pc {
            0x8325EB08 => {
    //   block [0x8325EB08..0x8325EB40)
	// 8325EB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EB14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EB1C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325EB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EB24: 4AF95235  bl 0x821f3d58
	ctx.lr = 0x8325EB28;
	sub_821F3D58(ctx, base);
	// 8325EB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EB2C: 906AAEEC  stw r3, -0x5114(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20756 as u32), ctx.r[3].u32 ) };
	// 8325EB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EB40 size=56
    let mut pc: u32 = 0x8325EB40;
    'dispatch: loop {
        match pc {
            0x8325EB40 => {
    //   block [0x8325EB40..0x8325EB78)
	// 8325EB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EB4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EB54: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325EB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EB5C: 4AF951FD  bl 0x821f3d58
	ctx.lr = 0x8325EB60;
	sub_821F3D58(ctx, base);
	// 8325EB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EB64: 906AAEF0  stw r3, -0x5110(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20752 as u32), ctx.r[3].u32 ) };
	// 8325EB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EB78 size=56
    let mut pc: u32 = 0x8325EB78;
    'dispatch: loop {
        match pc {
            0x8325EB78 => {
    //   block [0x8325EB78..0x8325EBB0)
	// 8325EB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EB84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EB88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EB8C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325EB90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EB94: 4AF951C5  bl 0x821f3d58
	ctx.lr = 0x8325EB98;
	sub_821F3D58(ctx, base);
	// 8325EB98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EB9C: 906AAEF4  stw r3, -0x510c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20748 as u32), ctx.r[3].u32 ) };
	// 8325EBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EBB0 size=64
    let mut pc: u32 = 0x8325EBB0;
    'dispatch: loop {
        match pc {
            0x8325EBB0 => {
    //   block [0x8325EBB0..0x8325EBF0)
	// 8325EBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EBBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EBC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EBC4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325EBC8: 386AAEF8  addi r3, r10, -0x5108
	ctx.r[3].s64 = ctx.r[10].s64 + -20744;
	// 8325EBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EBD0: 4AFCE301  bl 0x8222ced0
	ctx.lr = 0x8325EBD4;
	sub_8222CED0(ctx, base);
	// 8325EBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EBD8: 3869C620  addi r3, r9, -0x39e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14816;
	// 8325EBDC: 4BA4B345  bl 0x82ca9f20
	ctx.lr = 0x8325EBE0;
	sub_82CA9F20(ctx, base);
	// 8325EBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EBF0 size=64
    let mut pc: u32 = 0x8325EBF0;
    'dispatch: loop {
        match pc {
            0x8325EBF0 => {
    //   block [0x8325EBF0..0x8325EC30)
	// 8325EBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EBFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EC00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EC04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325EC08: 386AAEFC  addi r3, r10, -0x5104
	ctx.r[3].s64 = ctx.r[10].s64 + -20740;
	// 8325EC0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EC10: 4AFCE2C1  bl 0x8222ced0
	ctx.lr = 0x8325EC14;
	sub_8222CED0(ctx, base);
	// 8325EC14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EC18: 3869C630  addi r3, r9, -0x39d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14800;
	// 8325EC1C: 4BA4B305  bl 0x82ca9f20
	ctx.lr = 0x8325EC20;
	sub_82CA9F20(ctx, base);
	// 8325EC20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EC30 size=64
    let mut pc: u32 = 0x8325EC30;
    'dispatch: loop {
        match pc {
            0x8325EC30 => {
    //   block [0x8325EC30..0x8325EC70)
	// 8325EC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EC38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EC3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EC44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325EC48: 386AAF00  addi r3, r10, -0x5100
	ctx.r[3].s64 = ctx.r[10].s64 + -20736;
	// 8325EC4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EC50: 4AFCE281  bl 0x8222ced0
	ctx.lr = 0x8325EC54;
	sub_8222CED0(ctx, base);
	// 8325EC54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EC58: 3869C640  addi r3, r9, -0x39c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14784;
	// 8325EC5C: 4BA4B2C5  bl 0x82ca9f20
	ctx.lr = 0x8325EC60;
	sub_82CA9F20(ctx, base);
	// 8325EC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EC70 size=64
    let mut pc: u32 = 0x8325EC70;
    'dispatch: loop {
        match pc {
            0x8325EC70 => {
    //   block [0x8325EC70..0x8325ECB0)
	// 8325EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EC7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325EC80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EC84: 388BED44  addi r4, r11, -0x12bc
	ctx.r[4].s64 = ctx.r[11].s64 + -4796;
	// 8325EC88: 386AAF04  addi r3, r10, -0x50fc
	ctx.r[3].s64 = ctx.r[10].s64 + -20732;
	// 8325EC8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EC90: 4AFCE241  bl 0x8222ced0
	ctx.lr = 0x8325EC94;
	sub_8222CED0(ctx, base);
	// 8325EC94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EC98: 3869C650  addi r3, r9, -0x39b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14768;
	// 8325EC9C: 4BA4B285  bl 0x82ca9f20
	ctx.lr = 0x8325ECA0;
	sub_82CA9F20(ctx, base);
	// 8325ECA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ECA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ECA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ECAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ECB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ECB0 size=64
    let mut pc: u32 = 0x8325ECB0;
    'dispatch: loop {
        match pc {
            0x8325ECB0 => {
    //   block [0x8325ECB0..0x8325ECF0)
	// 8325ECB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ECB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ECB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ECBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325ECC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ECC4: 388BED50  addi r4, r11, -0x12b0
	ctx.r[4].s64 = ctx.r[11].s64 + -4784;
	// 8325ECC8: 386AAF08  addi r3, r10, -0x50f8
	ctx.r[3].s64 = ctx.r[10].s64 + -20728;
	// 8325ECCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ECD0: 4AFCE201  bl 0x8222ced0
	ctx.lr = 0x8325ECD4;
	sub_8222CED0(ctx, base);
	// 8325ECD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ECD8: 3869C660  addi r3, r9, -0x39a0
	ctx.r[3].s64 = ctx.r[9].s64 + -14752;
	// 8325ECDC: 4BA4B245  bl 0x82ca9f20
	ctx.lr = 0x8325ECE0;
	sub_82CA9F20(ctx, base);
	// 8325ECE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ECE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ECE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ECEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ECF0 size=56
    let mut pc: u32 = 0x8325ECF0;
    'dispatch: loop {
        match pc {
            0x8325ECF0 => {
    //   block [0x8325ECF0..0x8325ED28)
	// 8325ECF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ECF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ECF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ECFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325ED00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325ED04: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325ED08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325ED0C: 4AF9504D  bl 0x821f3d58
	ctx.lr = 0x8325ED10;
	sub_821F3D58(ctx, base);
	// 8325ED10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ED14: 906AAF0C  stw r3, -0x50f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20724 as u32), ctx.r[3].u32 ) };
	// 8325ED18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ED1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ED20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ED28 size=56
    let mut pc: u32 = 0x8325ED28;
    'dispatch: loop {
        match pc {
            0x8325ED28 => {
    //   block [0x8325ED28..0x8325ED60)
	// 8325ED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ED30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ED34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325ED38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325ED3C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325ED40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325ED44: 4AF95015  bl 0x821f3d58
	ctx.lr = 0x8325ED48;
	sub_821F3D58(ctx, base);
	// 8325ED48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ED4C: 906AAF10  stw r3, -0x50f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20720 as u32), ctx.r[3].u32 ) };
	// 8325ED50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ED54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ED58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ED5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ED60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ED60 size=56
    let mut pc: u32 = 0x8325ED60;
    'dispatch: loop {
        match pc {
            0x8325ED60 => {
    //   block [0x8325ED60..0x8325ED98)
	// 8325ED60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ED64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ED68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ED6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325ED70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325ED74: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325ED78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325ED7C: 4AF94FDD  bl 0x821f3d58
	ctx.lr = 0x8325ED80;
	sub_821F3D58(ctx, base);
	// 8325ED80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ED84: 906AAF14  stw r3, -0x50ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20716 as u32), ctx.r[3].u32 ) };
	// 8325ED88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ED8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ED90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ED98 size=56
    let mut pc: u32 = 0x8325ED98;
    'dispatch: loop {
        match pc {
            0x8325ED98 => {
    //   block [0x8325ED98..0x8325EDD0)
	// 8325ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EDA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EDA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EDAC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325EDB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EDB4: 4AF94FA5  bl 0x821f3d58
	ctx.lr = 0x8325EDB8;
	sub_821F3D58(ctx, base);
	// 8325EDB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EDBC: 906AAF18  stw r3, -0x50e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20712 as u32), ctx.r[3].u32 ) };
	// 8325EDC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EDD0 size=56
    let mut pc: u32 = 0x8325EDD0;
    'dispatch: loop {
        match pc {
            0x8325EDD0 => {
    //   block [0x8325EDD0..0x8325EE08)
	// 8325EDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EDD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EDDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EDE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EDE4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325EDE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EDEC: 4AF94F6D  bl 0x821f3d58
	ctx.lr = 0x8325EDF0;
	sub_821F3D58(ctx, base);
	// 8325EDF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EDF4: 906AAF1C  stw r3, -0x50e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20708 as u32), ctx.r[3].u32 ) };
	// 8325EDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EE08 size=56
    let mut pc: u32 = 0x8325EE08;
    'dispatch: loop {
        match pc {
            0x8325EE08 => {
    //   block [0x8325EE08..0x8325EE40)
	// 8325EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EE14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EE18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EE1C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325EE20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EE24: 4AF94F35  bl 0x821f3d58
	ctx.lr = 0x8325EE28;
	sub_821F3D58(ctx, base);
	// 8325EE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EE2C: 906AAF20  stw r3, -0x50e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20704 as u32), ctx.r[3].u32 ) };
	// 8325EE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EE40 size=56
    let mut pc: u32 = 0x8325EE40;
    'dispatch: loop {
        match pc {
            0x8325EE40 => {
    //   block [0x8325EE40..0x8325EE78)
	// 8325EE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EE4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EE54: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325EE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EE5C: 4AF94EFD  bl 0x821f3d58
	ctx.lr = 0x8325EE60;
	sub_821F3D58(ctx, base);
	// 8325EE60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EE64: 906AAF24  stw r3, -0x50dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20700 as u32), ctx.r[3].u32 ) };
	// 8325EE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EE78 size=56
    let mut pc: u32 = 0x8325EE78;
    'dispatch: loop {
        match pc {
            0x8325EE78 => {
    //   block [0x8325EE78..0x8325EEB0)
	// 8325EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EE84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EE88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EE8C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325EE90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EE94: 4AF94EC5  bl 0x821f3d58
	ctx.lr = 0x8325EE98;
	sub_821F3D58(ctx, base);
	// 8325EE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EE9C: 906AAF28  stw r3, -0x50d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20696 as u32), ctx.r[3].u32 ) };
	// 8325EEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EEB0 size=56
    let mut pc: u32 = 0x8325EEB0;
    'dispatch: loop {
        match pc {
            0x8325EEB0 => {
    //   block [0x8325EEB0..0x8325EEE8)
	// 8325EEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EEBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EEC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EEC4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325EEC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EECC: 4AF94E8D  bl 0x821f3d58
	ctx.lr = 0x8325EED0;
	sub_821F3D58(ctx, base);
	// 8325EED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EED4: 906AAF2C  stw r3, -0x50d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20692 as u32), ctx.r[3].u32 ) };
	// 8325EED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EEE8 size=56
    let mut pc: u32 = 0x8325EEE8;
    'dispatch: loop {
        match pc {
            0x8325EEE8 => {
    //   block [0x8325EEE8..0x8325EF20)
	// 8325EEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EEF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EEF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EEF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EEFC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325EF00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EF04: 4AF94E55  bl 0x821f3d58
	ctx.lr = 0x8325EF08;
	sub_821F3D58(ctx, base);
	// 8325EF08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EF0C: 906AAF30  stw r3, -0x50d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20688 as u32), ctx.r[3].u32 ) };
	// 8325EF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EF20 size=56
    let mut pc: u32 = 0x8325EF20;
    'dispatch: loop {
        match pc {
            0x8325EF20 => {
    //   block [0x8325EF20..0x8325EF58)
	// 8325EF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EF2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EF30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EF34: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325EF38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EF3C: 4AF94E1D  bl 0x821f3d58
	ctx.lr = 0x8325EF40;
	sub_821F3D58(ctx, base);
	// 8325EF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EF44: 906AAF34  stw r3, -0x50cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20684 as u32), ctx.r[3].u32 ) };
	// 8325EF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EF58 size=56
    let mut pc: u32 = 0x8325EF58;
    'dispatch: loop {
        match pc {
            0x8325EF58 => {
    //   block [0x8325EF58..0x8325EF90)
	// 8325EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EF64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EF68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EF6C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325EF70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EF74: 4AF94DE5  bl 0x821f3d58
	ctx.lr = 0x8325EF78;
	sub_821F3D58(ctx, base);
	// 8325EF78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EF7C: 906AAF38  stw r3, -0x50c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20680 as u32), ctx.r[3].u32 ) };
	// 8325EF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EF90 size=56
    let mut pc: u32 = 0x8325EF90;
    'dispatch: loop {
        match pc {
            0x8325EF90 => {
    //   block [0x8325EF90..0x8325EFC8)
	// 8325EF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EF9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EFA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EFA4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325EFA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EFAC: 4AF94DAD  bl 0x821f3d58
	ctx.lr = 0x8325EFB0;
	sub_821F3D58(ctx, base);
	// 8325EFB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EFB4: 906AAF3C  stw r3, -0x50c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20676 as u32), ctx.r[3].u32 ) };
	// 8325EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EFC8 size=56
    let mut pc: u32 = 0x8325EFC8;
    'dispatch: loop {
        match pc {
            0x8325EFC8 => {
    //   block [0x8325EFC8..0x8325F000)
	// 8325EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EFD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EFD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EFDC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325EFE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EFE4: 4AF94D75  bl 0x821f3d58
	ctx.lr = 0x8325EFE8;
	sub_821F3D58(ctx, base);
	// 8325EFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EFEC: 906AAF40  stw r3, -0x50c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20672 as u32), ctx.r[3].u32 ) };
	// 8325EFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F000 size=56
    let mut pc: u32 = 0x8325F000;
    'dispatch: loop {
        match pc {
            0x8325F000 => {
    //   block [0x8325F000..0x8325F038)
	// 8325F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F00C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F010: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F014: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325F018: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F01C: 4AF94D3D  bl 0x821f3d58
	ctx.lr = 0x8325F020;
	sub_821F3D58(ctx, base);
	// 8325F020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F024: 906AAF44  stw r3, -0x50bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20668 as u32), ctx.r[3].u32 ) };
	// 8325F028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F038 size=56
    let mut pc: u32 = 0x8325F038;
    'dispatch: loop {
        match pc {
            0x8325F038 => {
    //   block [0x8325F038..0x8325F070)
	// 8325F038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F044: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F048: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F04C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325F050: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F054: 4AF94D05  bl 0x821f3d58
	ctx.lr = 0x8325F058;
	sub_821F3D58(ctx, base);
	// 8325F058: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F05C: 906AAF48  stw r3, -0x50b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20664 as u32), ctx.r[3].u32 ) };
	// 8325F060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F070 size=56
    let mut pc: u32 = 0x8325F070;
    'dispatch: loop {
        match pc {
            0x8325F070 => {
    //   block [0x8325F070..0x8325F0A8)
	// 8325F070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F07C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F080: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F084: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325F088: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F08C: 4AF94CCD  bl 0x821f3d58
	ctx.lr = 0x8325F090;
	sub_821F3D58(ctx, base);
	// 8325F090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F094: 906AAF4C  stw r3, -0x50b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20660 as u32), ctx.r[3].u32 ) };
	// 8325F098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F0A8 size=56
    let mut pc: u32 = 0x8325F0A8;
    'dispatch: loop {
        match pc {
            0x8325F0A8 => {
    //   block [0x8325F0A8..0x8325F0E0)
	// 8325F0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F0B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F0B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F0B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F0BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325F0C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F0C4: 4AF94C95  bl 0x821f3d58
	ctx.lr = 0x8325F0C8;
	sub_821F3D58(ctx, base);
	// 8325F0C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F0CC: 906AAF50  stw r3, -0x50b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20656 as u32), ctx.r[3].u32 ) };
	// 8325F0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F0E0 size=56
    let mut pc: u32 = 0x8325F0E0;
    'dispatch: loop {
        match pc {
            0x8325F0E0 => {
    //   block [0x8325F0E0..0x8325F118)
	// 8325F0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F0EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F0F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F0F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325F0F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F0FC: 4AF94C5D  bl 0x821f3d58
	ctx.lr = 0x8325F100;
	sub_821F3D58(ctx, base);
	// 8325F100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F104: 906AAF54  stw r3, -0x50ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20652 as u32), ctx.r[3].u32 ) };
	// 8325F108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F118 size=56
    let mut pc: u32 = 0x8325F118;
    'dispatch: loop {
        match pc {
            0x8325F118 => {
    //   block [0x8325F118..0x8325F150)
	// 8325F118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F12C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325F130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F134: 4AF94C25  bl 0x821f3d58
	ctx.lr = 0x8325F138;
	sub_821F3D58(ctx, base);
	// 8325F138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F13C: 906AAF58  stw r3, -0x50a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20648 as u32), ctx.r[3].u32 ) };
	// 8325F140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F150 size=56
    let mut pc: u32 = 0x8325F150;
    'dispatch: loop {
        match pc {
            0x8325F150 => {
    //   block [0x8325F150..0x8325F188)
	// 8325F150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F15C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F164: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325F168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F16C: 4AF94BED  bl 0x821f3d58
	ctx.lr = 0x8325F170;
	sub_821F3D58(ctx, base);
	// 8325F170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F174: 906AAF5C  stw r3, -0x50a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20644 as u32), ctx.r[3].u32 ) };
	// 8325F178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F188 size=64
    let mut pc: u32 = 0x8325F188;
    'dispatch: loop {
        match pc {
            0x8325F188 => {
    //   block [0x8325F188..0x8325F1C8)
	// 8325F188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F194: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F19C: 388BED98  addi r4, r11, -0x1268
	ctx.r[4].s64 = ctx.r[11].s64 + -4712;
	// 8325F1A0: 386AAF60  addi r3, r10, -0x50a0
	ctx.r[3].s64 = ctx.r[10].s64 + -20640;
	// 8325F1A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F1A8: 4AFCDD29  bl 0x8222ced0
	ctx.lr = 0x8325F1AC;
	sub_8222CED0(ctx, base);
	// 8325F1AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F1B0: 3869C670  addi r3, r9, -0x3990
	ctx.r[3].s64 = ctx.r[9].s64 + -14736;
	// 8325F1B4: 4BA4AD6D  bl 0x82ca9f20
	ctx.lr = 0x8325F1B8;
	sub_82CA9F20(ctx, base);
	// 8325F1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F1C8 size=64
    let mut pc: u32 = 0x8325F1C8;
    'dispatch: loop {
        match pc {
            0x8325F1C8 => {
    //   block [0x8325F1C8..0x8325F208)
	// 8325F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F1D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F1D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F1DC: 388B92D4  addi r4, r11, -0x6d2c
	ctx.r[4].s64 = ctx.r[11].s64 + -27948;
	// 8325F1E0: 386AAF64  addi r3, r10, -0x509c
	ctx.r[3].s64 = ctx.r[10].s64 + -20636;
	// 8325F1E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F1E8: 4AFCDCE9  bl 0x8222ced0
	ctx.lr = 0x8325F1EC;
	sub_8222CED0(ctx, base);
	// 8325F1EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F1F0: 3869C680  addi r3, r9, -0x3980
	ctx.r[3].s64 = ctx.r[9].s64 + -14720;
	// 8325F1F4: 4BA4AD2D  bl 0x82ca9f20
	ctx.lr = 0x8325F1F8;
	sub_82CA9F20(ctx, base);
	// 8325F1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F208 size=60
    let mut pc: u32 = 0x8325F208;
    'dispatch: loop {
        match pc {
            0x8325F208 => {
    //   block [0x8325F208..0x8325F244)
	// 8325F208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F214: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F21C: 388BEE80  addi r4, r11, -0x1180
	ctx.r[4].s64 = ctx.r[11].s64 + -4480;
	// 8325F220: 386AAF68  addi r3, r10, -0x5098
	ctx.r[3].s64 = ctx.r[10].s64 + -20632;
	// 8325F224: 4B0771E5  bl 0x822d6408
	ctx.lr = 0x8325F228;
	sub_822D6408(ctx, base);
	// 8325F228: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F22C: 3869C690  addi r3, r9, -0x3970
	ctx.r[3].s64 = ctx.r[9].s64 + -14704;
	// 8325F230: 4BA4ACF1  bl 0x82ca9f20
	ctx.lr = 0x8325F234;
	sub_82CA9F20(ctx, base);
	// 8325F234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F248 size=64
    let mut pc: u32 = 0x8325F248;
    'dispatch: loop {
        match pc {
            0x8325F248 => {
    //   block [0x8325F248..0x8325F288)
	// 8325F248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F254: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F25C: 388BEE84  addi r4, r11, -0x117c
	ctx.r[4].s64 = ctx.r[11].s64 + -4476;
	// 8325F260: 386AAF6C  addi r3, r10, -0x5094
	ctx.r[3].s64 = ctx.r[10].s64 + -20628;
	// 8325F264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F268: 4AFCDC69  bl 0x8222ced0
	ctx.lr = 0x8325F26C;
	sub_8222CED0(ctx, base);
	// 8325F26C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F270: 3869C6A0  addi r3, r9, -0x3960
	ctx.r[3].s64 = ctx.r[9].s64 + -14688;
	// 8325F274: 4BA4ACAD  bl 0x82ca9f20
	ctx.lr = 0x8325F278;
	sub_82CA9F20(ctx, base);
	// 8325F278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F288 size=64
    let mut pc: u32 = 0x8325F288;
    'dispatch: loop {
        match pc {
            0x8325F288 => {
    //   block [0x8325F288..0x8325F2C8)
	// 8325F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F294: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F29C: 388BEEAC  addi r4, r11, -0x1154
	ctx.r[4].s64 = ctx.r[11].s64 + -4436;
	// 8325F2A0: 386AAF70  addi r3, r10, -0x5090
	ctx.r[3].s64 = ctx.r[10].s64 + -20624;
	// 8325F2A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F2A8: 4AFCDC29  bl 0x8222ced0
	ctx.lr = 0x8325F2AC;
	sub_8222CED0(ctx, base);
	// 8325F2AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F2B0: 3869C6B0  addi r3, r9, -0x3950
	ctx.r[3].s64 = ctx.r[9].s64 + -14672;
	// 8325F2B4: 4BA4AC6D  bl 0x82ca9f20
	ctx.lr = 0x8325F2B8;
	sub_82CA9F20(ctx, base);
	// 8325F2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F2C8 size=64
    let mut pc: u32 = 0x8325F2C8;
    'dispatch: loop {
        match pc {
            0x8325F2C8 => {
    //   block [0x8325F2C8..0x8325F308)
	// 8325F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F2D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F2D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F2DC: 388BEED4  addi r4, r11, -0x112c
	ctx.r[4].s64 = ctx.r[11].s64 + -4396;
	// 8325F2E0: 386AAF74  addi r3, r10, -0x508c
	ctx.r[3].s64 = ctx.r[10].s64 + -20620;
	// 8325F2E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F2E8: 4AFCDBE9  bl 0x8222ced0
	ctx.lr = 0x8325F2EC;
	sub_8222CED0(ctx, base);
	// 8325F2EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F2F0: 3869C6C0  addi r3, r9, -0x3940
	ctx.r[3].s64 = ctx.r[9].s64 + -14656;
	// 8325F2F4: 4BA4AC2D  bl 0x82ca9f20
	ctx.lr = 0x8325F2F8;
	sub_82CA9F20(ctx, base);
	// 8325F2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F308 size=64
    let mut pc: u32 = 0x8325F308;
    'dispatch: loop {
        match pc {
            0x8325F308 => {
    //   block [0x8325F308..0x8325F348)
	// 8325F308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F314: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F31C: 388BEEF8  addi r4, r11, -0x1108
	ctx.r[4].s64 = ctx.r[11].s64 + -4360;
	// 8325F320: 386AAF78  addi r3, r10, -0x5088
	ctx.r[3].s64 = ctx.r[10].s64 + -20616;
	// 8325F324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F328: 4AFCDBA9  bl 0x8222ced0
	ctx.lr = 0x8325F32C;
	sub_8222CED0(ctx, base);
	// 8325F32C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F330: 3869C6D0  addi r3, r9, -0x3930
	ctx.r[3].s64 = ctx.r[9].s64 + -14640;
	// 8325F334: 4BA4ABED  bl 0x82ca9f20
	ctx.lr = 0x8325F338;
	sub_82CA9F20(ctx, base);
	// 8325F338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F348 size=64
    let mut pc: u32 = 0x8325F348;
    'dispatch: loop {
        match pc {
            0x8325F348 => {
    //   block [0x8325F348..0x8325F388)
	// 8325F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F354: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F35C: 388BEF04  addi r4, r11, -0x10fc
	ctx.r[4].s64 = ctx.r[11].s64 + -4348;
	// 8325F360: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 8325F364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F368: 4AFCDB69  bl 0x8222ced0
	ctx.lr = 0x8325F36C;
	sub_8222CED0(ctx, base);
	// 8325F36C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F370: 3869C6E0  addi r3, r9, -0x3920
	ctx.r[3].s64 = ctx.r[9].s64 + -14624;
	// 8325F374: 4BA4ABAD  bl 0x82ca9f20
	ctx.lr = 0x8325F378;
	sub_82CA9F20(ctx, base);
	// 8325F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F388 size=64
    let mut pc: u32 = 0x8325F388;
    'dispatch: loop {
        match pc {
            0x8325F388 => {
    //   block [0x8325F388..0x8325F3C8)
	// 8325F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F394: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F39C: 388BEF14  addi r4, r11, -0x10ec
	ctx.r[4].s64 = ctx.r[11].s64 + -4332;
	// 8325F3A0: 386AAF80  addi r3, r10, -0x5080
	ctx.r[3].s64 = ctx.r[10].s64 + -20608;
	// 8325F3A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F3A8: 4AFCDB29  bl 0x8222ced0
	ctx.lr = 0x8325F3AC;
	sub_8222CED0(ctx, base);
	// 8325F3AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F3B0: 3869C6F0  addi r3, r9, -0x3910
	ctx.r[3].s64 = ctx.r[9].s64 + -14608;
	// 8325F3B4: 4BA4AB6D  bl 0x82ca9f20
	ctx.lr = 0x8325F3B8;
	sub_82CA9F20(ctx, base);
	// 8325F3B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F3C8 size=64
    let mut pc: u32 = 0x8325F3C8;
    'dispatch: loop {
        match pc {
            0x8325F3C8 => {
    //   block [0x8325F3C8..0x8325F408)
	// 8325F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F3D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F3D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325F3D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F3DC: 388BEFA4  addi r4, r11, -0x105c
	ctx.r[4].s64 = ctx.r[11].s64 + -4188;
	// 8325F3E0: 386AAF84  addi r3, r10, -0x507c
	ctx.r[3].s64 = ctx.r[10].s64 + -20604;
	// 8325F3E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F3E8: 4AFCDAE9  bl 0x8222ced0
	ctx.lr = 0x8325F3EC;
	sub_8222CED0(ctx, base);
	// 8325F3EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F3F0: 3869C700  addi r3, r9, -0x3900
	ctx.r[3].s64 = ctx.r[9].s64 + -14592;
	// 8325F3F4: 4BA4AB2D  bl 0x82ca9f20
	ctx.lr = 0x8325F3F8;
	sub_82CA9F20(ctx, base);
	// 8325F3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F408 size=64
    let mut pc: u32 = 0x8325F408;
    'dispatch: loop {
        match pc {
            0x8325F408 => {
    //   block [0x8325F408..0x8325F448)
	// 8325F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F414: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325F418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F41C: 388B1104  addi r4, r11, 0x1104
	ctx.r[4].s64 = ctx.r[11].s64 + 4356;
	// 8325F420: 386AAF88  addi r3, r10, -0x5078
	ctx.r[3].s64 = ctx.r[10].s64 + -20600;
	// 8325F424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F428: 4AFCDAA9  bl 0x8222ced0
	ctx.lr = 0x8325F42C;
	sub_8222CED0(ctx, base);
	// 8325F42C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F430: 3869C710  addi r3, r9, -0x38f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14576;
	// 8325F434: 4BA4AAED  bl 0x82ca9f20
	ctx.lr = 0x8325F438;
	sub_82CA9F20(ctx, base);
	// 8325F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F448 size=64
    let mut pc: u32 = 0x8325F448;
    'dispatch: loop {
        match pc {
            0x8325F448 => {
    //   block [0x8325F448..0x8325F488)
	// 8325F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F45C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325F460: 386AAF8C  addi r3, r10, -0x5074
	ctx.r[3].s64 = ctx.r[10].s64 + -20596;
	// 8325F464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F468: 4AFCDA69  bl 0x8222ced0
	ctx.lr = 0x8325F46C;
	sub_8222CED0(ctx, base);
	// 8325F46C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F470: 3869C720  addi r3, r9, -0x38e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14560;
	// 8325F474: 4BA4AAAD  bl 0x82ca9f20
	ctx.lr = 0x8325F478;
	sub_82CA9F20(ctx, base);
	// 8325F478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8325F488 size=96
    let mut pc: u32 = 0x8325F488;
    'dispatch: loop {
        match pc {
            0x8325F488 => {
    //   block [0x8325F488..0x8325F4E8)
	// 8325F488: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F48C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8325F490: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8325F494: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8325F498: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8325F49C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8325F4A0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8325F4A4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8325F4A8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F4E8 size=56
    let mut pc: u32 = 0x8325F4E8;
    'dispatch: loop {
        match pc {
            0x8325F4E8 => {
    //   block [0x8325F4E8..0x8325F520)
	// 8325F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F4F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F4FC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325F500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F504: 4AF94855  bl 0x821f3d58
	ctx.lr = 0x8325F508;
	sub_821F3D58(ctx, base);
	// 8325F508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F50C: 906AAFA0  stw r3, -0x5060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20576 as u32), ctx.r[3].u32 ) };
	// 8325F510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F520 size=56
    let mut pc: u32 = 0x8325F520;
    'dispatch: loop {
        match pc {
            0x8325F520 => {
    //   block [0x8325F520..0x8325F558)
	// 8325F520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F52C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F534: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325F538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F53C: 4AF9481D  bl 0x821f3d58
	ctx.lr = 0x8325F540;
	sub_821F3D58(ctx, base);
	// 8325F540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F544: 906AAFA4  stw r3, -0x505c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20572 as u32), ctx.r[3].u32 ) };
	// 8325F548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F558 size=56
    let mut pc: u32 = 0x8325F558;
    'dispatch: loop {
        match pc {
            0x8325F558 => {
    //   block [0x8325F558..0x8325F590)
	// 8325F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F56C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325F570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F574: 4AF947E5  bl 0x821f3d58
	ctx.lr = 0x8325F578;
	sub_821F3D58(ctx, base);
	// 8325F578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F57C: 906AAFA8  stw r3, -0x5058(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20568 as u32), ctx.r[3].u32 ) };
	// 8325F580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F590 size=56
    let mut pc: u32 = 0x8325F590;
    'dispatch: loop {
        match pc {
            0x8325F590 => {
    //   block [0x8325F590..0x8325F5C8)
	// 8325F590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F59C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F5A4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325F5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F5AC: 4AF947AD  bl 0x821f3d58
	ctx.lr = 0x8325F5B0;
	sub_821F3D58(ctx, base);
	// 8325F5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F5B4: 906AAFAC  stw r3, -0x5054(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20564 as u32), ctx.r[3].u32 ) };
	// 8325F5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F5C8 size=56
    let mut pc: u32 = 0x8325F5C8;
    'dispatch: loop {
        match pc {
            0x8325F5C8 => {
    //   block [0x8325F5C8..0x8325F600)
	// 8325F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F5D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F5DC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325F5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F5E4: 4AF94775  bl 0x821f3d58
	ctx.lr = 0x8325F5E8;
	sub_821F3D58(ctx, base);
	// 8325F5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F5EC: 906AAFB0  stw r3, -0x5050(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20560 as u32), ctx.r[3].u32 ) };
	// 8325F5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F600 size=56
    let mut pc: u32 = 0x8325F600;
    'dispatch: loop {
        match pc {
            0x8325F600 => {
    //   block [0x8325F600..0x8325F638)
	// 8325F600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F60C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F614: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325F618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F61C: 4AF9473D  bl 0x821f3d58
	ctx.lr = 0x8325F620;
	sub_821F3D58(ctx, base);
	// 8325F620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F624: 906AAFB4  stw r3, -0x504c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20556 as u32), ctx.r[3].u32 ) };
	// 8325F628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F638 size=56
    let mut pc: u32 = 0x8325F638;
    'dispatch: loop {
        match pc {
            0x8325F638 => {
    //   block [0x8325F638..0x8325F670)
	// 8325F638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F64C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325F650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F654: 4AF94705  bl 0x821f3d58
	ctx.lr = 0x8325F658;
	sub_821F3D58(ctx, base);
	// 8325F658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F65C: 906AAFB8  stw r3, -0x5048(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20552 as u32), ctx.r[3].u32 ) };
	// 8325F660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F670 size=56
    let mut pc: u32 = 0x8325F670;
    'dispatch: loop {
        match pc {
            0x8325F670 => {
    //   block [0x8325F670..0x8325F6A8)
	// 8325F670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F684: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325F688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F68C: 4AF946CD  bl 0x821f3d58
	ctx.lr = 0x8325F690;
	sub_821F3D58(ctx, base);
	// 8325F690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F694: 906AAFBC  stw r3, -0x5044(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20548 as u32), ctx.r[3].u32 ) };
	// 8325F698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F6A8 size=56
    let mut pc: u32 = 0x8325F6A8;
    'dispatch: loop {
        match pc {
            0x8325F6A8 => {
    //   block [0x8325F6A8..0x8325F6E0)
	// 8325F6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F6B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F6BC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325F6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F6C4: 4AF94695  bl 0x821f3d58
	ctx.lr = 0x8325F6C8;
	sub_821F3D58(ctx, base);
	// 8325F6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F6CC: 906AAFC0  stw r3, -0x5040(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20544 as u32), ctx.r[3].u32 ) };
	// 8325F6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F6E0 size=56
    let mut pc: u32 = 0x8325F6E0;
    'dispatch: loop {
        match pc {
            0x8325F6E0 => {
    //   block [0x8325F6E0..0x8325F718)
	// 8325F6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F6EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F6F4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325F6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F6FC: 4AF9465D  bl 0x821f3d58
	ctx.lr = 0x8325F700;
	sub_821F3D58(ctx, base);
	// 8325F700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F704: 906AAFC4  stw r3, -0x503c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20540 as u32), ctx.r[3].u32 ) };
	// 8325F708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F718 size=56
    let mut pc: u32 = 0x8325F718;
    'dispatch: loop {
        match pc {
            0x8325F718 => {
    //   block [0x8325F718..0x8325F750)
	// 8325F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F72C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325F730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F734: 4AF94625  bl 0x821f3d58
	ctx.lr = 0x8325F738;
	sub_821F3D58(ctx, base);
	// 8325F738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F73C: 906AAFC8  stw r3, -0x5038(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20536 as u32), ctx.r[3].u32 ) };
	// 8325F740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F750 size=56
    let mut pc: u32 = 0x8325F750;
    'dispatch: loop {
        match pc {
            0x8325F750 => {
    //   block [0x8325F750..0x8325F788)
	// 8325F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F764: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325F768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F76C: 4AF945ED  bl 0x821f3d58
	ctx.lr = 0x8325F770;
	sub_821F3D58(ctx, base);
	// 8325F770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F774: 906AAFCC  stw r3, -0x5034(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20532 as u32), ctx.r[3].u32 ) };
	// 8325F778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F788 size=56
    let mut pc: u32 = 0x8325F788;
    'dispatch: loop {
        match pc {
            0x8325F788 => {
    //   block [0x8325F788..0x8325F7C0)
	// 8325F788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F79C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325F7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F7A4: 4AF945B5  bl 0x821f3d58
	ctx.lr = 0x8325F7A8;
	sub_821F3D58(ctx, base);
	// 8325F7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F7AC: 906AAFD0  stw r3, -0x5030(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20528 as u32), ctx.r[3].u32 ) };
	// 8325F7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F7C0 size=56
    let mut pc: u32 = 0x8325F7C0;
    'dispatch: loop {
        match pc {
            0x8325F7C0 => {
    //   block [0x8325F7C0..0x8325F7F8)
	// 8325F7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F7D4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325F7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F7DC: 4AF9457D  bl 0x821f3d58
	ctx.lr = 0x8325F7E0;
	sub_821F3D58(ctx, base);
	// 8325F7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F7E4: 906AAFD4  stw r3, -0x502c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20524 as u32), ctx.r[3].u32 ) };
	// 8325F7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F7F8 size=56
    let mut pc: u32 = 0x8325F7F8;
    'dispatch: loop {
        match pc {
            0x8325F7F8 => {
    //   block [0x8325F7F8..0x8325F830)
	// 8325F7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F80C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325F810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F814: 4AF94545  bl 0x821f3d58
	ctx.lr = 0x8325F818;
	sub_821F3D58(ctx, base);
	// 8325F818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F81C: 906AAFD8  stw r3, -0x5028(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20520 as u32), ctx.r[3].u32 ) };
	// 8325F820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F830 size=56
    let mut pc: u32 = 0x8325F830;
    'dispatch: loop {
        match pc {
            0x8325F830 => {
    //   block [0x8325F830..0x8325F868)
	// 8325F830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F844: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325F848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F84C: 4AF9450D  bl 0x821f3d58
	ctx.lr = 0x8325F850;
	sub_821F3D58(ctx, base);
	// 8325F850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F854: 906AAFDC  stw r3, -0x5024(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20516 as u32), ctx.r[3].u32 ) };
	// 8325F858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F868 size=56
    let mut pc: u32 = 0x8325F868;
    'dispatch: loop {
        match pc {
            0x8325F868 => {
    //   block [0x8325F868..0x8325F8A0)
	// 8325F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F874: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F87C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325F880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F884: 4AF944D5  bl 0x821f3d58
	ctx.lr = 0x8325F888;
	sub_821F3D58(ctx, base);
	// 8325F888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F88C: 906AAFE0  stw r3, -0x5020(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20512 as u32), ctx.r[3].u32 ) };
	// 8325F890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F8A0 size=56
    let mut pc: u32 = 0x8325F8A0;
    'dispatch: loop {
        match pc {
            0x8325F8A0 => {
    //   block [0x8325F8A0..0x8325F8D8)
	// 8325F8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F8AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F8B4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325F8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F8BC: 4AF9449D  bl 0x821f3d58
	ctx.lr = 0x8325F8C0;
	sub_821F3D58(ctx, base);
	// 8325F8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F8C4: 906AAFE4  stw r3, -0x501c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20508 as u32), ctx.r[3].u32 ) };
	// 8325F8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F8D8 size=56
    let mut pc: u32 = 0x8325F8D8;
    'dispatch: loop {
        match pc {
            0x8325F8D8 => {
    //   block [0x8325F8D8..0x8325F910)
	// 8325F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F8E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F8EC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325F8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F8F4: 4AF94465  bl 0x821f3d58
	ctx.lr = 0x8325F8F8;
	sub_821F3D58(ctx, base);
	// 8325F8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F8FC: 906AAFE8  stw r3, -0x5018(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20504 as u32), ctx.r[3].u32 ) };
	// 8325F900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F910 size=56
    let mut pc: u32 = 0x8325F910;
    'dispatch: loop {
        match pc {
            0x8325F910 => {
    //   block [0x8325F910..0x8325F948)
	// 8325F910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F91C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F924: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325F928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F92C: 4AF9442D  bl 0x821f3d58
	ctx.lr = 0x8325F930;
	sub_821F3D58(ctx, base);
	// 8325F930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F934: 906AAFEC  stw r3, -0x5014(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20500 as u32), ctx.r[3].u32 ) };
	// 8325F938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F948 size=56
    let mut pc: u32 = 0x8325F948;
    'dispatch: loop {
        match pc {
            0x8325F948 => {
    //   block [0x8325F948..0x8325F980)
	// 8325F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F95C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325F960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F964: 4AF943F5  bl 0x821f3d58
	ctx.lr = 0x8325F968;
	sub_821F3D58(ctx, base);
	// 8325F968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F96C: 906AAFF0  stw r3, -0x5010(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20496 as u32), ctx.r[3].u32 ) };
	// 8325F970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F980 size=64
    let mut pc: u32 = 0x8325F980;
    'dispatch: loop {
        match pc {
            0x8325F980 => {
    //   block [0x8325F980..0x8325F9C0)
	// 8325F980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F98C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F994: 388BF1D0  addi r4, r11, -0xe30
	ctx.r[4].s64 = ctx.r[11].s64 + -3632;
	// 8325F998: 386AAFF4  addi r3, r10, -0x500c
	ctx.r[3].s64 = ctx.r[10].s64 + -20492;
	// 8325F99C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F9A0: 4AFCD531  bl 0x8222ced0
	ctx.lr = 0x8325F9A4;
	sub_8222CED0(ctx, base);
	// 8325F9A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F9A8: 3869C730  addi r3, r9, -0x38d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14544;
	// 8325F9AC: 4BA4A575  bl 0x82ca9f20
	ctx.lr = 0x8325F9B0;
	sub_82CA9F20(ctx, base);
	// 8325F9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F9C0 size=64
    let mut pc: u32 = 0x8325F9C0;
    'dispatch: loop {
        match pc {
            0x8325F9C0 => {
    //   block [0x8325F9C0..0x8325FA00)
	// 8325F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F9CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F9D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F9D4: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325F9D8: 386AAFF8  addi r3, r10, -0x5008
	ctx.r[3].s64 = ctx.r[10].s64 + -20488;
	// 8325F9DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F9E0: 4AFCD4F1  bl 0x8222ced0
	ctx.lr = 0x8325F9E4;
	sub_8222CED0(ctx, base);
	// 8325F9E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F9E8: 3869C740  addi r3, r9, -0x38c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14528;
	// 8325F9EC: 4BA4A535  bl 0x82ca9f20
	ctx.lr = 0x8325F9F0;
	sub_82CA9F20(ctx, base);
	// 8325F9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FA00 size=64
    let mut pc: u32 = 0x8325FA00;
    'dispatch: loop {
        match pc {
            0x8325FA00 => {
    //   block [0x8325FA00..0x8325FA40)
	// 8325FA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FA08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FA0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FA14: 388BF1EC  addi r4, r11, -0xe14
	ctx.r[4].s64 = ctx.r[11].s64 + -3604;
	// 8325FA18: 386AAFFC  addi r3, r10, -0x5004
	ctx.r[3].s64 = ctx.r[10].s64 + -20484;
	// 8325FA1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FA20: 4AFCD4B1  bl 0x8222ced0
	ctx.lr = 0x8325FA24;
	sub_8222CED0(ctx, base);
	// 8325FA24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FA28: 3869C750  addi r3, r9, -0x38b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14512;
	// 8325FA2C: 4BA4A4F5  bl 0x82ca9f20
	ctx.lr = 0x8325FA30;
	sub_82CA9F20(ctx, base);
	// 8325FA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FA40 size=64
    let mut pc: u32 = 0x8325FA40;
    'dispatch: loop {
        match pc {
            0x8325FA40 => {
    //   block [0x8325FA40..0x8325FA80)
	// 8325FA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FA4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FA50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FA54: 388BF228  addi r4, r11, -0xdd8
	ctx.r[4].s64 = ctx.r[11].s64 + -3544;
	// 8325FA58: 386AB000  addi r3, r10, -0x5000
	ctx.r[3].s64 = ctx.r[10].s64 + -20480;
	// 8325FA5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FA60: 4AFCD471  bl 0x8222ced0
	ctx.lr = 0x8325FA64;
	sub_8222CED0(ctx, base);
	// 8325FA64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FA68: 3869C760  addi r3, r9, -0x38a0
	ctx.r[3].s64 = ctx.r[9].s64 + -14496;
	// 8325FA6C: 4BA4A4B5  bl 0x82ca9f20
	ctx.lr = 0x8325FA70;
	sub_82CA9F20(ctx, base);
	// 8325FA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FA80 size=64
    let mut pc: u32 = 0x8325FA80;
    'dispatch: loop {
        match pc {
            0x8325FA80 => {
    //   block [0x8325FA80..0x8325FAC0)
	// 8325FA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FA8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FA94: 388BF24C  addi r4, r11, -0xdb4
	ctx.r[4].s64 = ctx.r[11].s64 + -3508;
	// 8325FA98: 386AB004  addi r3, r10, -0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -20476;
	// 8325FA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FAA0: 4AFCD431  bl 0x8222ced0
	ctx.lr = 0x8325FAA4;
	sub_8222CED0(ctx, base);
	// 8325FAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FAA8: 3869C770  addi r3, r9, -0x3890
	ctx.r[3].s64 = ctx.r[9].s64 + -14480;
	// 8325FAAC: 4BA4A475  bl 0x82ca9f20
	ctx.lr = 0x8325FAB0;
	sub_82CA9F20(ctx, base);
	// 8325FAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FAC0 size=64
    let mut pc: u32 = 0x8325FAC0;
    'dispatch: loop {
        match pc {
            0x8325FAC0 => {
    //   block [0x8325FAC0..0x8325FB00)
	// 8325FAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FACC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FAD4: 388BF270  addi r4, r11, -0xd90
	ctx.r[4].s64 = ctx.r[11].s64 + -3472;
	// 8325FAD8: 386AB008  addi r3, r10, -0x4ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -20472;
	// 8325FADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FAE0: 4AFCD3F1  bl 0x8222ced0
	ctx.lr = 0x8325FAE4;
	sub_8222CED0(ctx, base);
	// 8325FAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FAE8: 3869C780  addi r3, r9, -0x3880
	ctx.r[3].s64 = ctx.r[9].s64 + -14464;
	// 8325FAEC: 4BA4A435  bl 0x82ca9f20
	ctx.lr = 0x8325FAF0;
	sub_82CA9F20(ctx, base);
	// 8325FAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FB00 size=64
    let mut pc: u32 = 0x8325FB00;
    'dispatch: loop {
        match pc {
            0x8325FB00 => {
    //   block [0x8325FB00..0x8325FB40)
	// 8325FB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FB0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FB14: 388BF290  addi r4, r11, -0xd70
	ctx.r[4].s64 = ctx.r[11].s64 + -3440;
	// 8325FB18: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 8325FB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FB20: 4AFCD3B1  bl 0x8222ced0
	ctx.lr = 0x8325FB24;
	sub_8222CED0(ctx, base);
	// 8325FB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FB28: 3869C790  addi r3, r9, -0x3870
	ctx.r[3].s64 = ctx.r[9].s64 + -14448;
	// 8325FB2C: 4BA4A3F5  bl 0x82ca9f20
	ctx.lr = 0x8325FB30;
	sub_82CA9F20(ctx, base);
	// 8325FB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FB40 size=64
    let mut pc: u32 = 0x8325FB40;
    'dispatch: loop {
        match pc {
            0x8325FB40 => {
    //   block [0x8325FB40..0x8325FB80)
	// 8325FB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FB4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FB50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FB54: 388BF2B0  addi r4, r11, -0xd50
	ctx.r[4].s64 = ctx.r[11].s64 + -3408;
	// 8325FB58: 386AB010  addi r3, r10, -0x4ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -20464;
	// 8325FB5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FB60: 4AFCD371  bl 0x8222ced0
	ctx.lr = 0x8325FB64;
	sub_8222CED0(ctx, base);
	// 8325FB64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FB68: 3869C7A0  addi r3, r9, -0x3860
	ctx.r[3].s64 = ctx.r[9].s64 + -14432;
	// 8325FB6C: 4BA4A3B5  bl 0x82ca9f20
	ctx.lr = 0x8325FB70;
	sub_82CA9F20(ctx, base);
	// 8325FB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FB80 size=64
    let mut pc: u32 = 0x8325FB80;
    'dispatch: loop {
        match pc {
            0x8325FB80 => {
    //   block [0x8325FB80..0x8325FBC0)
	// 8325FB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FB8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FB90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FB94: 388BF2DC  addi r4, r11, -0xd24
	ctx.r[4].s64 = ctx.r[11].s64 + -3364;
	// 8325FB98: 386AB014  addi r3, r10, -0x4fec
	ctx.r[3].s64 = ctx.r[10].s64 + -20460;
	// 8325FB9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FBA0: 4AFCD331  bl 0x8222ced0
	ctx.lr = 0x8325FBA4;
	sub_8222CED0(ctx, base);
	// 8325FBA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FBA8: 3869C7B0  addi r3, r9, -0x3850
	ctx.r[3].s64 = ctx.r[9].s64 + -14416;
	// 8325FBAC: 4BA4A375  bl 0x82ca9f20
	ctx.lr = 0x8325FBB0;
	sub_82CA9F20(ctx, base);
	// 8325FBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FBC0 size=64
    let mut pc: u32 = 0x8325FBC0;
    'dispatch: loop {
        match pc {
            0x8325FBC0 => {
    //   block [0x8325FBC0..0x8325FC00)
	// 8325FBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FBCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FBD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FBD4: 388BF308  addi r4, r11, -0xcf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3320;
	// 8325FBD8: 386AB018  addi r3, r10, -0x4fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -20456;
	// 8325FBDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FBE0: 4AFCD2F1  bl 0x8222ced0
	ctx.lr = 0x8325FBE4;
	sub_8222CED0(ctx, base);
	// 8325FBE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FBE8: 3869C7C0  addi r3, r9, -0x3840
	ctx.r[3].s64 = ctx.r[9].s64 + -14400;
	// 8325FBEC: 4BA4A335  bl 0x82ca9f20
	ctx.lr = 0x8325FBF0;
	sub_82CA9F20(ctx, base);
	// 8325FBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FC00 size=64
    let mut pc: u32 = 0x8325FC00;
    'dispatch: loop {
        match pc {
            0x8325FC00 => {
    //   block [0x8325FC00..0x8325FC40)
	// 8325FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FC0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FC10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FC14: 388BF334  addi r4, r11, -0xccc
	ctx.r[4].s64 = ctx.r[11].s64 + -3276;
	// 8325FC18: 386AB01C  addi r3, r10, -0x4fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -20452;
	// 8325FC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FC20: 4AFCD2B1  bl 0x8222ced0
	ctx.lr = 0x8325FC24;
	sub_8222CED0(ctx, base);
	// 8325FC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FC28: 3869C7D0  addi r3, r9, -0x3830
	ctx.r[3].s64 = ctx.r[9].s64 + -14384;
	// 8325FC2C: 4BA4A2F5  bl 0x82ca9f20
	ctx.lr = 0x8325FC30;
	sub_82CA9F20(ctx, base);
	// 8325FC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FC40 size=64
    let mut pc: u32 = 0x8325FC40;
    'dispatch: loop {
        match pc {
            0x8325FC40 => {
    //   block [0x8325FC40..0x8325FC80)
	// 8325FC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FC4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FC50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FC54: 388BF35C  addi r4, r11, -0xca4
	ctx.r[4].s64 = ctx.r[11].s64 + -3236;
	// 8325FC58: 386AB020  addi r3, r10, -0x4fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -20448;
	// 8325FC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FC60: 4AFCD271  bl 0x8222ced0
	ctx.lr = 0x8325FC64;
	sub_8222CED0(ctx, base);
	// 8325FC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FC68: 3869C7E0  addi r3, r9, -0x3820
	ctx.r[3].s64 = ctx.r[9].s64 + -14368;
	// 8325FC6C: 4BA4A2B5  bl 0x82ca9f20
	ctx.lr = 0x8325FC70;
	sub_82CA9F20(ctx, base);
	// 8325FC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FC80 size=64
    let mut pc: u32 = 0x8325FC80;
    'dispatch: loop {
        match pc {
            0x8325FC80 => {
    //   block [0x8325FC80..0x8325FCC0)
	// 8325FC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FC8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FC94: 388BF384  addi r4, r11, -0xc7c
	ctx.r[4].s64 = ctx.r[11].s64 + -3196;
	// 8325FC98: 386AB024  addi r3, r10, -0x4fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -20444;
	// 8325FC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FCA0: 4AFCD231  bl 0x8222ced0
	ctx.lr = 0x8325FCA4;
	sub_8222CED0(ctx, base);
	// 8325FCA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FCA8: 3869C7F0  addi r3, r9, -0x3810
	ctx.r[3].s64 = ctx.r[9].s64 + -14352;
	// 8325FCAC: 4BA4A275  bl 0x82ca9f20
	ctx.lr = 0x8325FCB0;
	sub_82CA9F20(ctx, base);
	// 8325FCB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FCB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FCB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FCC0 size=64
    let mut pc: u32 = 0x8325FCC0;
    'dispatch: loop {
        match pc {
            0x8325FCC0 => {
    //   block [0x8325FCC0..0x8325FD00)
	// 8325FCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FCC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FCCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FCD4: 388BF3B0  addi r4, r11, -0xc50
	ctx.r[4].s64 = ctx.r[11].s64 + -3152;
	// 8325FCD8: 386AB028  addi r3, r10, -0x4fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -20440;
	// 8325FCDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FCE0: 4AFCD1F1  bl 0x8222ced0
	ctx.lr = 0x8325FCE4;
	sub_8222CED0(ctx, base);
	// 8325FCE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FCE8: 3869C800  addi r3, r9, -0x3800
	ctx.r[3].s64 = ctx.r[9].s64 + -14336;
	// 8325FCEC: 4BA4A235  bl 0x82ca9f20
	ctx.lr = 0x8325FCF0;
	sub_82CA9F20(ctx, base);
	// 8325FCF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FD00 size=64
    let mut pc: u32 = 0x8325FD00;
    'dispatch: loop {
        match pc {
            0x8325FD00 => {
    //   block [0x8325FD00..0x8325FD40)
	// 8325FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FD0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FD10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FD14: 388BF3DC  addi r4, r11, -0xc24
	ctx.r[4].s64 = ctx.r[11].s64 + -3108;
	// 8325FD18: 386AB02C  addi r3, r10, -0x4fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -20436;
	// 8325FD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FD20: 4AFCD1B1  bl 0x8222ced0
	ctx.lr = 0x8325FD24;
	sub_8222CED0(ctx, base);
	// 8325FD24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FD28: 3869C810  addi r3, r9, -0x37f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14320;
	// 8325FD2C: 4BA4A1F5  bl 0x82ca9f20
	ctx.lr = 0x8325FD30;
	sub_82CA9F20(ctx, base);
	// 8325FD30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FD40 size=64
    let mut pc: u32 = 0x8325FD40;
    'dispatch: loop {
        match pc {
            0x8325FD40 => {
    //   block [0x8325FD40..0x8325FD80)
	// 8325FD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FD4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FD50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FD54: 388BF408  addi r4, r11, -0xbf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3064;
	// 8325FD58: 386AB030  addi r3, r10, -0x4fd0
	ctx.r[3].s64 = ctx.r[10].s64 + -20432;
	// 8325FD5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FD60: 4AFCD171  bl 0x8222ced0
	ctx.lr = 0x8325FD64;
	sub_8222CED0(ctx, base);
	// 8325FD64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FD68: 3869C820  addi r3, r9, -0x37e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14304;
	// 8325FD6C: 4BA4A1B5  bl 0x82ca9f20
	ctx.lr = 0x8325FD70;
	sub_82CA9F20(ctx, base);
	// 8325FD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FD80 size=56
    let mut pc: u32 = 0x8325FD80;
    'dispatch: loop {
        match pc {
            0x8325FD80 => {
    //   block [0x8325FD80..0x8325FDB8)
	// 8325FD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FD8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FD90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FD94: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325FD98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FD9C: 4AF93FBD  bl 0x821f3d58
	ctx.lr = 0x8325FDA0;
	sub_821F3D58(ctx, base);
	// 8325FDA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FDA4: 906AB034  stw r3, -0x4fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20428 as u32), ctx.r[3].u32 ) };
	// 8325FDA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FDB8 size=56
    let mut pc: u32 = 0x8325FDB8;
    'dispatch: loop {
        match pc {
            0x8325FDB8 => {
    //   block [0x8325FDB8..0x8325FDF0)
	// 8325FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FDC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FDC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FDC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FDCC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325FDD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FDD4: 4AF93F85  bl 0x821f3d58
	ctx.lr = 0x8325FDD8;
	sub_821F3D58(ctx, base);
	// 8325FDD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FDDC: 906AB038  stw r3, -0x4fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20424 as u32), ctx.r[3].u32 ) };
	// 8325FDE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FDF0 size=56
    let mut pc: u32 = 0x8325FDF0;
    'dispatch: loop {
        match pc {
            0x8325FDF0 => {
    //   block [0x8325FDF0..0x8325FE28)
	// 8325FDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FDF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FDFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FE00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FE04: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325FE08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FE0C: 4AF93F4D  bl 0x821f3d58
	ctx.lr = 0x8325FE10;
	sub_821F3D58(ctx, base);
	// 8325FE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FE14: 906AB03C  stw r3, -0x4fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20420 as u32), ctx.r[3].u32 ) };
	// 8325FE18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FE28 size=56
    let mut pc: u32 = 0x8325FE28;
    'dispatch: loop {
        match pc {
            0x8325FE28 => {
    //   block [0x8325FE28..0x8325FE60)
	// 8325FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FE30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FE34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FE38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FE3C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325FE40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FE44: 4AF93F15  bl 0x821f3d58
	ctx.lr = 0x8325FE48;
	sub_821F3D58(ctx, base);
	// 8325FE48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FE4C: 906AB040  stw r3, -0x4fc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20416 as u32), ctx.r[3].u32 ) };
	// 8325FE50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FE60 size=56
    let mut pc: u32 = 0x8325FE60;
    'dispatch: loop {
        match pc {
            0x8325FE60 => {
    //   block [0x8325FE60..0x8325FE98)
	// 8325FE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FE68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FE6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FE70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FE74: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325FE78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FE7C: 4AF93EDD  bl 0x821f3d58
	ctx.lr = 0x8325FE80;
	sub_821F3D58(ctx, base);
	// 8325FE80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FE84: 906AB044  stw r3, -0x4fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20412 as u32), ctx.r[3].u32 ) };
	// 8325FE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FE98 size=56
    let mut pc: u32 = 0x8325FE98;
    'dispatch: loop {
        match pc {
            0x8325FE98 => {
    //   block [0x8325FE98..0x8325FED0)
	// 8325FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FEA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FEA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FEAC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325FEB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FEB4: 4AF93EA5  bl 0x821f3d58
	ctx.lr = 0x8325FEB8;
	sub_821F3D58(ctx, base);
	// 8325FEB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FEBC: 906AB048  stw r3, -0x4fb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20408 as u32), ctx.r[3].u32 ) };
	// 8325FEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FED0 size=56
    let mut pc: u32 = 0x8325FED0;
    'dispatch: loop {
        match pc {
            0x8325FED0 => {
    //   block [0x8325FED0..0x8325FF08)
	// 8325FED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FEDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FEE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FEE4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325FEE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FEEC: 4AF93E6D  bl 0x821f3d58
	ctx.lr = 0x8325FEF0;
	sub_821F3D58(ctx, base);
	// 8325FEF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FEF4: 906AB04C  stw r3, -0x4fb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20404 as u32), ctx.r[3].u32 ) };
	// 8325FEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FF08 size=56
    let mut pc: u32 = 0x8325FF08;
    'dispatch: loop {
        match pc {
            0x8325FF08 => {
    //   block [0x8325FF08..0x8325FF40)
	// 8325FF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FF14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FF18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FF1C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325FF20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FF24: 4AF93E35  bl 0x821f3d58
	ctx.lr = 0x8325FF28;
	sub_821F3D58(ctx, base);
	// 8325FF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FF2C: 906AB050  stw r3, -0x4fb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20400 as u32), ctx.r[3].u32 ) };
	// 8325FF30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FF40 size=56
    let mut pc: u32 = 0x8325FF40;
    'dispatch: loop {
        match pc {
            0x8325FF40 => {
    //   block [0x8325FF40..0x8325FF78)
	// 8325FF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FF48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FF4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FF50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FF54: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325FF58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FF5C: 4AF93DFD  bl 0x821f3d58
	ctx.lr = 0x8325FF60;
	sub_821F3D58(ctx, base);
	// 8325FF60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FF64: 906AB054  stw r3, -0x4fac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20396 as u32), ctx.r[3].u32 ) };
	// 8325FF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FF78 size=56
    let mut pc: u32 = 0x8325FF78;
    'dispatch: loop {
        match pc {
            0x8325FF78 => {
    //   block [0x8325FF78..0x8325FFB0)
	// 8325FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FF84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FF88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FF8C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325FF90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FF94: 4AF93DC5  bl 0x821f3d58
	ctx.lr = 0x8325FF98;
	sub_821F3D58(ctx, base);
	// 8325FF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FF9C: 906AB058  stw r3, -0x4fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20392 as u32), ctx.r[3].u32 ) };
	// 8325FFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FFB0 size=56
    let mut pc: u32 = 0x8325FFB0;
    'dispatch: loop {
        match pc {
            0x8325FFB0 => {
    //   block [0x8325FFB0..0x8325FFE8)
	// 8325FFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FFB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FFBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FFC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FFC4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325FFC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FFCC: 4AF93D8D  bl 0x821f3d58
	ctx.lr = 0x8325FFD0;
	sub_821F3D58(ctx, base);
	// 8325FFD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FFD4: 906AB05C  stw r3, -0x4fa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20388 as u32), ctx.r[3].u32 ) };
	// 8325FFD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FFE8 size=56
    let mut pc: u32 = 0x8325FFE8;
    'dispatch: loop {
        match pc {
            0x8325FFE8 => {
    //   block [0x8325FFE8..0x83260020)
	// 8325FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FFF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FFF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FFF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FFFC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83260000: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260004: 4AF93D55  bl 0x821f3d58
	ctx.lr = 0x83260008;
	sub_821F3D58(ctx, base);
	// 83260008: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326000C: 906AB060  stw r3, -0x4fa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20384 as u32), ctx.r[3].u32 ) };
	// 83260010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326001C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260020 size=56
    let mut pc: u32 = 0x83260020;
    'dispatch: loop {
        match pc {
            0x83260020 => {
    //   block [0x83260020..0x83260058)
	// 83260020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326002C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260030: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260034: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83260038: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326003C: 4AF93D1D  bl 0x821f3d58
	ctx.lr = 0x83260040;
	sub_821F3D58(ctx, base);
	// 83260040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260044: 906AB064  stw r3, -0x4f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20380 as u32), ctx.r[3].u32 ) };
	// 83260048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326004C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260058 size=56
    let mut pc: u32 = 0x83260058;
    'dispatch: loop {
        match pc {
            0x83260058 => {
    //   block [0x83260058..0x83260090)
	// 83260058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326005C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260064: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260068: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326006C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83260070: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260074: 4AF93CE5  bl 0x821f3d58
	ctx.lr = 0x83260078;
	sub_821F3D58(ctx, base);
	// 83260078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326007C: 906AB068  stw r3, -0x4f98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20376 as u32), ctx.r[3].u32 ) };
	// 83260080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326008C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260090 size=56
    let mut pc: u32 = 0x83260090;
    'dispatch: loop {
        match pc {
            0x83260090 => {
    //   block [0x83260090..0x832600C8)
	// 83260090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326009C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832600A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832600A4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832600A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832600AC: 4AF93CAD  bl 0x821f3d58
	ctx.lr = 0x832600B0;
	sub_821F3D58(ctx, base);
	// 832600B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832600B4: 906AB06C  stw r3, -0x4f94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20372 as u32), ctx.r[3].u32 ) };
	// 832600B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832600BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832600C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832600C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832600C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832600C8 size=56
    let mut pc: u32 = 0x832600C8;
    'dispatch: loop {
        match pc {
            0x832600C8 => {
    //   block [0x832600C8..0x83260100)
	// 832600C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832600CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832600D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832600D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832600D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832600DC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832600E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832600E4: 4AF93C75  bl 0x821f3d58
	ctx.lr = 0x832600E8;
	sub_821F3D58(ctx, base);
	// 832600E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832600EC: 906AB070  stw r3, -0x4f90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20368 as u32), ctx.r[3].u32 ) };
	// 832600F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832600F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832600F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832600FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260100 size=56
    let mut pc: u32 = 0x83260100;
    'dispatch: loop {
        match pc {
            0x83260100 => {
    //   block [0x83260100..0x83260138)
	// 83260100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326010C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260110: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260114: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83260118: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326011C: 4AF93C3D  bl 0x821f3d58
	ctx.lr = 0x83260120;
	sub_821F3D58(ctx, base);
	// 83260120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260124: 906AB074  stw r3, -0x4f8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20364 as u32), ctx.r[3].u32 ) };
	// 83260128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326012C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260138 size=56
    let mut pc: u32 = 0x83260138;
    'dispatch: loop {
        match pc {
            0x83260138 => {
    //   block [0x83260138..0x83260170)
	// 83260138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260148: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326014C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83260150: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260154: 4AF93C05  bl 0x821f3d58
	ctx.lr = 0x83260158;
	sub_821F3D58(ctx, base);
	// 83260158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326015C: 906AB078  stw r3, -0x4f88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20360 as u32), ctx.r[3].u32 ) };
	// 83260160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326016C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260170 size=56
    let mut pc: u32 = 0x83260170;
    'dispatch: loop {
        match pc {
            0x83260170 => {
    //   block [0x83260170..0x832601A8)
	// 83260170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326017C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260180: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260184: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83260188: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326018C: 4AF93BCD  bl 0x821f3d58
	ctx.lr = 0x83260190;
	sub_821F3D58(ctx, base);
	// 83260190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260194: 906AB07C  stw r3, -0x4f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20356 as u32), ctx.r[3].u32 ) };
	// 83260198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326019C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832601A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832601A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832601A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832601A8 size=56
    let mut pc: u32 = 0x832601A8;
    'dispatch: loop {
        match pc {
            0x832601A8 => {
    //   block [0x832601A8..0x832601E0)
	// 832601A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832601AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832601B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832601B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832601B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832601BC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832601C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832601C4: 4AF93B95  bl 0x821f3d58
	ctx.lr = 0x832601C8;
	sub_821F3D58(ctx, base);
	// 832601C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832601CC: 906AB080  stw r3, -0x4f80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20352 as u32), ctx.r[3].u32 ) };
	// 832601D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832601D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832601D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832601DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832601E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832601E0 size=56
    let mut pc: u32 = 0x832601E0;
    'dispatch: loop {
        match pc {
            0x832601E0 => {
    //   block [0x832601E0..0x83260218)
	// 832601E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832601E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832601E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832601EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832601F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832601F4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832601F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832601FC: 4AF93B5D  bl 0x821f3d58
	ctx.lr = 0x83260200;
	sub_821F3D58(ctx, base);
	// 83260200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260204: 906AB084  stw r3, -0x4f7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20348 as u32), ctx.r[3].u32 ) };
	// 83260208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260218 size=64
    let mut pc: u32 = 0x83260218;
    'dispatch: loop {
        match pc {
            0x83260218 => {
    //   block [0x83260218..0x83260258)
	// 83260218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326021C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260224: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326022C: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83260230: 386AB088  addi r3, r10, -0x4f78
	ctx.r[3].s64 = ctx.r[10].s64 + -20344;
	// 83260234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260238: 4AFCCC99  bl 0x8222ced0
	ctx.lr = 0x8326023C;
	sub_8222CED0(ctx, base);
	// 8326023C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260240: 3869C830  addi r3, r9, -0x37d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14288;
	// 83260244: 4BA49CDD  bl 0x82ca9f20
	ctx.lr = 0x83260248;
	sub_82CA9F20(ctx, base);
	// 83260248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326024C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260258 size=56
    let mut pc: u32 = 0x83260258;
    'dispatch: loop {
        match pc {
            0x83260258 => {
    //   block [0x83260258..0x83260290)
	// 83260258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326026C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83260270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260274: 4AF93AE5  bl 0x821f3d58
	ctx.lr = 0x83260278;
	sub_821F3D58(ctx, base);
	// 83260278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326027C: 906AB08C  stw r3, -0x4f74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20340 as u32), ctx.r[3].u32 ) };
	// 83260280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326028C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260290 size=56
    let mut pc: u32 = 0x83260290;
    'dispatch: loop {
        match pc {
            0x83260290 => {
    //   block [0x83260290..0x832602C8)
	// 83260290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326029C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832602A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832602A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832602A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832602AC: 4AF93AAD  bl 0x821f3d58
	ctx.lr = 0x832602B0;
	sub_821F3D58(ctx, base);
	// 832602B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832602B4: 906AB090  stw r3, -0x4f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20336 as u32), ctx.r[3].u32 ) };
	// 832602B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832602BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832602C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832602C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832602C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832602C8 size=56
    let mut pc: u32 = 0x832602C8;
    'dispatch: loop {
        match pc {
            0x832602C8 => {
    //   block [0x832602C8..0x83260300)
	// 832602C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832602CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832602D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832602D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832602D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832602DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832602E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832602E4: 4AF93A75  bl 0x821f3d58
	ctx.lr = 0x832602E8;
	sub_821F3D58(ctx, base);
	// 832602E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832602EC: 906AB094  stw r3, -0x4f6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20332 as u32), ctx.r[3].u32 ) };
	// 832602F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832602F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832602F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832602FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260300 size=56
    let mut pc: u32 = 0x83260300;
    'dispatch: loop {
        match pc {
            0x83260300 => {
    //   block [0x83260300..0x83260338)
	// 83260300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326030C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260314: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83260318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326031C: 4AF93A3D  bl 0x821f3d58
	ctx.lr = 0x83260320;
	sub_821F3D58(ctx, base);
	// 83260320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260324: 906AB098  stw r3, -0x4f68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20328 as u32), ctx.r[3].u32 ) };
	// 83260328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326032C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260338 size=56
    let mut pc: u32 = 0x83260338;
    'dispatch: loop {
        match pc {
            0x83260338 => {
    //   block [0x83260338..0x83260370)
	// 83260338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326034C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83260350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260354: 4AF93A05  bl 0x821f3d58
	ctx.lr = 0x83260358;
	sub_821F3D58(ctx, base);
	// 83260358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326035C: 906AB09C  stw r3, -0x4f64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20324 as u32), ctx.r[3].u32 ) };
	// 83260360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326036C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260370 size=56
    let mut pc: u32 = 0x83260370;
    'dispatch: loop {
        match pc {
            0x83260370 => {
    //   block [0x83260370..0x832603A8)
	// 83260370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326037C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260384: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83260388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326038C: 4AF939CD  bl 0x821f3d58
	ctx.lr = 0x83260390;
	sub_821F3D58(ctx, base);
	// 83260390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260394: 906AB0A0  stw r3, -0x4f60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20320 as u32), ctx.r[3].u32 ) };
	// 83260398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326039C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832603A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832603A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832603A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832603A8 size=56
    let mut pc: u32 = 0x832603A8;
    'dispatch: loop {
        match pc {
            0x832603A8 => {
    //   block [0x832603A8..0x832603E0)
	// 832603A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832603AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832603B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832603B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832603B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832603BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832603C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832603C4: 4AF93995  bl 0x821f3d58
	ctx.lr = 0x832603C8;
	sub_821F3D58(ctx, base);
	// 832603C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832603CC: 906AB0A4  stw r3, -0x4f5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20316 as u32), ctx.r[3].u32 ) };
	// 832603D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832603D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832603D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832603DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832603E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832603E0 size=56
    let mut pc: u32 = 0x832603E0;
    'dispatch: loop {
        match pc {
            0x832603E0 => {
    //   block [0x832603E0..0x83260418)
	// 832603E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832603E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832603E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832603EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832603F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832603F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832603F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832603FC: 4AF9395D  bl 0x821f3d58
	ctx.lr = 0x83260400;
	sub_821F3D58(ctx, base);
	// 83260400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260404: 906AB0A8  stw r3, -0x4f58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20312 as u32), ctx.r[3].u32 ) };
	// 83260408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260418 size=56
    let mut pc: u32 = 0x83260418;
    'dispatch: loop {
        match pc {
            0x83260418 => {
    //   block [0x83260418..0x83260450)
	// 83260418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260424: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260428: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326042C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83260430: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260434: 4AF93925  bl 0x821f3d58
	ctx.lr = 0x83260438;
	sub_821F3D58(ctx, base);
	// 83260438: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326043C: 906AB0AC  stw r3, -0x4f54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20308 as u32), ctx.r[3].u32 ) };
	// 83260440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326044C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260450 size=56
    let mut pc: u32 = 0x83260450;
    'dispatch: loop {
        match pc {
            0x83260450 => {
    //   block [0x83260450..0x83260488)
	// 83260450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326045C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260460: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260464: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83260468: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326046C: 4AF938ED  bl 0x821f3d58
	ctx.lr = 0x83260470;
	sub_821F3D58(ctx, base);
	// 83260470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260474: 906AB0B0  stw r3, -0x4f50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20304 as u32), ctx.r[3].u32 ) };
	// 83260478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260488 size=56
    let mut pc: u32 = 0x83260488;
    'dispatch: loop {
        match pc {
            0x83260488 => {
    //   block [0x83260488..0x832604C0)
	// 83260488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260498: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326049C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832604A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832604A4: 4AF938B5  bl 0x821f3d58
	ctx.lr = 0x832604A8;
	sub_821F3D58(ctx, base);
	// 832604A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832604AC: 906AB0B4  stw r3, -0x4f4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20300 as u32), ctx.r[3].u32 ) };
	// 832604B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832604B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832604B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832604BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832604C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832604C0 size=56
    let mut pc: u32 = 0x832604C0;
    'dispatch: loop {
        match pc {
            0x832604C0 => {
    //   block [0x832604C0..0x832604F8)
	// 832604C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832604C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832604C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832604CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832604D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832604D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832604D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832604DC: 4AF9387D  bl 0x821f3d58
	ctx.lr = 0x832604E0;
	sub_821F3D58(ctx, base);
	// 832604E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832604E4: 906AB0B8  stw r3, -0x4f48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20296 as u32), ctx.r[3].u32 ) };
	// 832604E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832604EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832604F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832604F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832604F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832604F8 size=56
    let mut pc: u32 = 0x832604F8;
    'dispatch: loop {
        match pc {
            0x832604F8 => {
    //   block [0x832604F8..0x83260530)
	// 832604F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832604FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260504: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260508: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326050C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83260510: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260514: 4AF93845  bl 0x821f3d58
	ctx.lr = 0x83260518;
	sub_821F3D58(ctx, base);
	// 83260518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326051C: 906AB0BC  stw r3, -0x4f44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20292 as u32), ctx.r[3].u32 ) };
	// 83260520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326052C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260530 size=56
    let mut pc: u32 = 0x83260530;
    'dispatch: loop {
        match pc {
            0x83260530 => {
    //   block [0x83260530..0x83260568)
	// 83260530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326053C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260540: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260544: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83260548: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326054C: 4AF9380D  bl 0x821f3d58
	ctx.lr = 0x83260550;
	sub_821F3D58(ctx, base);
	// 83260550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260554: 906AB0C0  stw r3, -0x4f40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20288 as u32), ctx.r[3].u32 ) };
	// 83260558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326055C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260568 size=56
    let mut pc: u32 = 0x83260568;
    'dispatch: loop {
        match pc {
            0x83260568 => {
    //   block [0x83260568..0x832605A0)
	// 83260568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260574: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260578: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326057C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83260580: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260584: 4AF937D5  bl 0x821f3d58
	ctx.lr = 0x83260588;
	sub_821F3D58(ctx, base);
	// 83260588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326058C: 906AB0C4  stw r3, -0x4f3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20284 as u32), ctx.r[3].u32 ) };
	// 83260590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326059C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832605A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832605A0 size=56
    let mut pc: u32 = 0x832605A0;
    'dispatch: loop {
        match pc {
            0x832605A0 => {
    //   block [0x832605A0..0x832605D8)
	// 832605A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832605A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832605A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832605AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832605B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832605B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832605B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832605BC: 4AF9379D  bl 0x821f3d58
	ctx.lr = 0x832605C0;
	sub_821F3D58(ctx, base);
	// 832605C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832605C4: 906AB0C8  stw r3, -0x4f38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20280 as u32), ctx.r[3].u32 ) };
	// 832605C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832605CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832605D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832605D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832605D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832605D8 size=56
    let mut pc: u32 = 0x832605D8;
    'dispatch: loop {
        match pc {
            0x832605D8 => {
    //   block [0x832605D8..0x83260610)
	// 832605D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832605DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832605E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832605E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832605E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832605EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832605F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832605F4: 4AF93765  bl 0x821f3d58
	ctx.lr = 0x832605F8;
	sub_821F3D58(ctx, base);
	// 832605F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832605FC: 906AB0CC  stw r3, -0x4f34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20276 as u32), ctx.r[3].u32 ) };
	// 83260600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326060C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260610 size=56
    let mut pc: u32 = 0x83260610;
    'dispatch: loop {
        match pc {
            0x83260610 => {
    //   block [0x83260610..0x83260648)
	// 83260610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326061C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260624: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83260628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326062C: 4AF9372D  bl 0x821f3d58
	ctx.lr = 0x83260630;
	sub_821F3D58(ctx, base);
	// 83260630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260634: 906AB0D0  stw r3, -0x4f30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20272 as u32), ctx.r[3].u32 ) };
	// 83260638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260648 size=56
    let mut pc: u32 = 0x83260648;
    'dispatch: loop {
        match pc {
            0x83260648 => {
    //   block [0x83260648..0x83260680)
	// 83260648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260654: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260658: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326065C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83260660: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260664: 4AF936F5  bl 0x821f3d58
	ctx.lr = 0x83260668;
	sub_821F3D58(ctx, base);
	// 83260668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326066C: 906AB0D4  stw r3, -0x4f2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20268 as u32), ctx.r[3].u32 ) };
	// 83260670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326067C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260680 size=56
    let mut pc: u32 = 0x83260680;
    'dispatch: loop {
        match pc {
            0x83260680 => {
    //   block [0x83260680..0x832606B8)
	// 83260680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326068C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260690: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260694: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83260698: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326069C: 4AF936BD  bl 0x821f3d58
	ctx.lr = 0x832606A0;
	sub_821F3D58(ctx, base);
	// 832606A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832606A4: 906AB0D8  stw r3, -0x4f28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20264 as u32), ctx.r[3].u32 ) };
	// 832606A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832606AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832606B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832606B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832606B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832606B8 size=56
    let mut pc: u32 = 0x832606B8;
    'dispatch: loop {
        match pc {
            0x832606B8 => {
    //   block [0x832606B8..0x832606F0)
	// 832606B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832606BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832606C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832606C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832606C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832606CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832606D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832606D4: 4AF93685  bl 0x821f3d58
	ctx.lr = 0x832606D8;
	sub_821F3D58(ctx, base);
	// 832606D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832606DC: 906AB0DC  stw r3, -0x4f24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20260 as u32), ctx.r[3].u32 ) };
	// 832606E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832606E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832606E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832606EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832606F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832606F0 size=64
    let mut pc: u32 = 0x832606F0;
    'dispatch: loop {
        match pc {
            0x832606F0 => {
    //   block [0x832606F0..0x83260730)
	// 832606F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832606F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832606F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832606FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260704: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 83260708: 386AB0E0  addi r3, r10, -0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + -20256;
	// 8326070C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260710: 4AFCC7C1  bl 0x8222ced0
	ctx.lr = 0x83260714;
	sub_8222CED0(ctx, base);
	// 83260714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260718: 3869C840  addi r3, r9, -0x37c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14272;
	// 8326071C: 4BA49805  bl 0x82ca9f20
	ctx.lr = 0x83260720;
	sub_82CA9F20(ctx, base);
	// 83260720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326072C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260730 size=64
    let mut pc: u32 = 0x83260730;
    'dispatch: loop {
        match pc {
            0x83260730 => {
    //   block [0x83260730..0x83260770)
	// 83260730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326073C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260744: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83260748: 386AB0E4  addi r3, r10, -0x4f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -20252;
	// 8326074C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260750: 4AFCC781  bl 0x8222ced0
	ctx.lr = 0x83260754;
	sub_8222CED0(ctx, base);
	// 83260754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260758: 3869C850  addi r3, r9, -0x37b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14256;
	// 8326075C: 4BA497C5  bl 0x82ca9f20
	ctx.lr = 0x83260760;
	sub_82CA9F20(ctx, base);
	// 83260760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326076C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260770 size=64
    let mut pc: u32 = 0x83260770;
    'dispatch: loop {
        match pc {
            0x83260770 => {
    //   block [0x83260770..0x832607B0)
	// 83260770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326077C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260784: 388BF5B0  addi r4, r11, -0xa50
	ctx.r[4].s64 = ctx.r[11].s64 + -2640;
	// 83260788: 386AB0E8  addi r3, r10, -0x4f18
	ctx.r[3].s64 = ctx.r[10].s64 + -20248;
	// 8326078C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260790: 4AFCC741  bl 0x8222ced0
	ctx.lr = 0x83260794;
	sub_8222CED0(ctx, base);
	// 83260794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260798: 3869C860  addi r3, r9, -0x37a0
	ctx.r[3].s64 = ctx.r[9].s64 + -14240;
	// 8326079C: 4BA49785  bl 0x82ca9f20
	ctx.lr = 0x832607A0;
	sub_82CA9F20(ctx, base);
	// 832607A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832607A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832607A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832607AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832607B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832607B0 size=64
    let mut pc: u32 = 0x832607B0;
    'dispatch: loop {
        match pc {
            0x832607B0 => {
    //   block [0x832607B0..0x832607F0)
	// 832607B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832607B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832607B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832607BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832607C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832607C4: 388BF5D4  addi r4, r11, -0xa2c
	ctx.r[4].s64 = ctx.r[11].s64 + -2604;
	// 832607C8: 386AB0EC  addi r3, r10, -0x4f14
	ctx.r[3].s64 = ctx.r[10].s64 + -20244;
	// 832607CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832607D0: 4AFCC701  bl 0x8222ced0
	ctx.lr = 0x832607D4;
	sub_8222CED0(ctx, base);
	// 832607D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832607D8: 3869C870  addi r3, r9, -0x3790
	ctx.r[3].s64 = ctx.r[9].s64 + -14224;
	// 832607DC: 4BA49745  bl 0x82ca9f20
	ctx.lr = 0x832607E0;
	sub_82CA9F20(ctx, base);
	// 832607E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832607E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832607E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832607EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832607F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832607F0 size=64
    let mut pc: u32 = 0x832607F0;
    'dispatch: loop {
        match pc {
            0x832607F0 => {
    //   block [0x832607F0..0x83260830)
	// 832607F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832607F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832607F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832607FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260804: 388BF5F4  addi r4, r11, -0xa0c
	ctx.r[4].s64 = ctx.r[11].s64 + -2572;
	// 83260808: 386AB0F0  addi r3, r10, -0x4f10
	ctx.r[3].s64 = ctx.r[10].s64 + -20240;
	// 8326080C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260810: 4AFCC6C1  bl 0x8222ced0
	ctx.lr = 0x83260814;
	sub_8222CED0(ctx, base);
	// 83260814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260818: 3869C880  addi r3, r9, -0x3780
	ctx.r[3].s64 = ctx.r[9].s64 + -14208;
	// 8326081C: 4BA49705  bl 0x82ca9f20
	ctx.lr = 0x83260820;
	sub_82CA9F20(ctx, base);
	// 83260820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326082C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260830 size=64
    let mut pc: u32 = 0x83260830;
    'dispatch: loop {
        match pc {
            0x83260830 => {
    //   block [0x83260830..0x83260870)
	// 83260830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326083C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260844: 388BF614  addi r4, r11, -0x9ec
	ctx.r[4].s64 = ctx.r[11].s64 + -2540;
	// 83260848: 386AB0F4  addi r3, r10, -0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -20236;
	// 8326084C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260850: 4AFCC681  bl 0x8222ced0
	ctx.lr = 0x83260854;
	sub_8222CED0(ctx, base);
	// 83260854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260858: 3869C890  addi r3, r9, -0x3770
	ctx.r[3].s64 = ctx.r[9].s64 + -14192;
	// 8326085C: 4BA496C5  bl 0x82ca9f20
	ctx.lr = 0x83260860;
	sub_82CA9F20(ctx, base);
	// 83260860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260870 size=64
    let mut pc: u32 = 0x83260870;
    'dispatch: loop {
        match pc {
            0x83260870 => {
    //   block [0x83260870..0x832608B0)
	// 83260870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326087C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260884: 388BF638  addi r4, r11, -0x9c8
	ctx.r[4].s64 = ctx.r[11].s64 + -2504;
	// 83260888: 386AB0F8  addi r3, r10, -0x4f08
	ctx.r[3].s64 = ctx.r[10].s64 + -20232;
	// 8326088C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260890: 4AFCC641  bl 0x8222ced0
	ctx.lr = 0x83260894;
	sub_8222CED0(ctx, base);
	// 83260894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260898: 3869C8A0  addi r3, r9, -0x3760
	ctx.r[3].s64 = ctx.r[9].s64 + -14176;
	// 8326089C: 4BA49685  bl 0x82ca9f20
	ctx.lr = 0x832608A0;
	sub_82CA9F20(ctx, base);
	// 832608A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832608A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832608A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832608AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832608B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832608B0 size=64
    let mut pc: u32 = 0x832608B0;
    'dispatch: loop {
        match pc {
            0x832608B0 => {
    //   block [0x832608B0..0x832608F0)
	// 832608B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832608B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832608B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832608BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832608C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832608C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832608C8: 386AB0FC  addi r3, r10, -0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + -20228;
	// 832608CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832608D0: 4AFCC601  bl 0x8222ced0
	ctx.lr = 0x832608D4;
	sub_8222CED0(ctx, base);
	// 832608D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832608D8: 3869C8B0  addi r3, r9, -0x3750
	ctx.r[3].s64 = ctx.r[9].s64 + -14160;
	// 832608DC: 4BA49645  bl 0x82ca9f20
	ctx.lr = 0x832608E0;
	sub_82CA9F20(ctx, base);
	// 832608E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832608E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832608E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832608EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832608F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832608F0 size=64
    let mut pc: u32 = 0x832608F0;
    'dispatch: loop {
        match pc {
            0x832608F0 => {
    //   block [0x832608F0..0x83260930)
	// 832608F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832608F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832608F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832608FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260904: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83260908: 386AB100  addi r3, r10, -0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + -20224;
	// 8326090C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260910: 4AFCC5C1  bl 0x8222ced0
	ctx.lr = 0x83260914;
	sub_8222CED0(ctx, base);
	// 83260914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260918: 3869C8C0  addi r3, r9, -0x3740
	ctx.r[3].s64 = ctx.r[9].s64 + -14144;
	// 8326091C: 4BA49605  bl 0x82ca9f20
	ctx.lr = 0x83260920;
	sub_82CA9F20(ctx, base);
	// 83260920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326092C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260930 size=64
    let mut pc: u32 = 0x83260930;
    'dispatch: loop {
        match pc {
            0x83260930 => {
    //   block [0x83260930..0x83260970)
	// 83260930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326093C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260944: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83260948: 386AB104  addi r3, r10, -0x4efc
	ctx.r[3].s64 = ctx.r[10].s64 + -20220;
	// 8326094C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260950: 4AFCC581  bl 0x8222ced0
	ctx.lr = 0x83260954;
	sub_8222CED0(ctx, base);
	// 83260954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260958: 3869C8D0  addi r3, r9, -0x3730
	ctx.r[3].s64 = ctx.r[9].s64 + -14128;
	// 8326095C: 4BA495C5  bl 0x82ca9f20
	ctx.lr = 0x83260960;
	sub_82CA9F20(ctx, base);
	// 83260960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326096C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260970 size=56
    let mut pc: u32 = 0x83260970;
    'dispatch: loop {
        match pc {
            0x83260970 => {
    //   block [0x83260970..0x832609A8)
	// 83260970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326097C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260980: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260984: 386BF764  addi r3, r11, -0x89c
	ctx.r[3].s64 = ctx.r[11].s64 + -2204;
	// 83260988: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326098C: 4AF933CD  bl 0x821f3d58
	ctx.lr = 0x83260990;
	sub_821F3D58(ctx, base);
	// 83260990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260994: 906AB108  stw r3, -0x4ef8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20216 as u32), ctx.r[3].u32 ) };
	// 83260998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326099C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832609A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832609A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832609A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832609A8 size=56
    let mut pc: u32 = 0x832609A8;
    'dispatch: loop {
        match pc {
            0x832609A8 => {
    //   block [0x832609A8..0x832609E0)
	// 832609A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832609AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832609B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832609B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832609B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832609BC: 386BF770  addi r3, r11, -0x890
	ctx.r[3].s64 = ctx.r[11].s64 + -2192;
	// 832609C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832609C4: 4AF93395  bl 0x821f3d58
	ctx.lr = 0x832609C8;
	sub_821F3D58(ctx, base);
	// 832609C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832609CC: 906AB10C  stw r3, -0x4ef4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20212 as u32), ctx.r[3].u32 ) };
	// 832609D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832609D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832609D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832609DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832609E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832609E0 size=56
    let mut pc: u32 = 0x832609E0;
    'dispatch: loop {
        match pc {
            0x832609E0 => {
    //   block [0x832609E0..0x83260A18)
	// 832609E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832609E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832609E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832609EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832609F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832609F4: 386BF77C  addi r3, r11, -0x884
	ctx.r[3].s64 = ctx.r[11].s64 + -2180;
	// 832609F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832609FC: 4AF9335D  bl 0x821f3d58
	ctx.lr = 0x83260A00;
	sub_821F3D58(ctx, base);
	// 83260A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260A04: 906AB110  stw r3, -0x4ef0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20208 as u32), ctx.r[3].u32 ) };
	// 83260A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260A18 size=56
    let mut pc: u32 = 0x83260A18;
    'dispatch: loop {
        match pc {
            0x83260A18 => {
    //   block [0x83260A18..0x83260A50)
	// 83260A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260A24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260A28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260A2C: 386BF788  addi r3, r11, -0x878
	ctx.r[3].s64 = ctx.r[11].s64 + -2168;
	// 83260A30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260A34: 4AF93325  bl 0x821f3d58
	ctx.lr = 0x83260A38;
	sub_821F3D58(ctx, base);
	// 83260A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260A3C: 906AB114  stw r3, -0x4eec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20204 as u32), ctx.r[3].u32 ) };
	// 83260A40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260A50 size=56
    let mut pc: u32 = 0x83260A50;
    'dispatch: loop {
        match pc {
            0x83260A50 => {
    //   block [0x83260A50..0x83260A88)
	// 83260A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260A5C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260A60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260A64: 386BF794  addi r3, r11, -0x86c
	ctx.r[3].s64 = ctx.r[11].s64 + -2156;
	// 83260A68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260A6C: 4AF932ED  bl 0x821f3d58
	ctx.lr = 0x83260A70;
	sub_821F3D58(ctx, base);
	// 83260A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260A74: 906AB118  stw r3, -0x4ee8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20200 as u32), ctx.r[3].u32 ) };
	// 83260A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260A88 size=56
    let mut pc: u32 = 0x83260A88;
    'dispatch: loop {
        match pc {
            0x83260A88 => {
    //   block [0x83260A88..0x83260AC0)
	// 83260A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260A94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260A98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260A9C: 386BF7A4  addi r3, r11, -0x85c
	ctx.r[3].s64 = ctx.r[11].s64 + -2140;
	// 83260AA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260AA4: 4AF932B5  bl 0x821f3d58
	ctx.lr = 0x83260AA8;
	sub_821F3D58(ctx, base);
	// 83260AA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260AAC: 906AB11C  stw r3, -0x4ee4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20196 as u32), ctx.r[3].u32 ) };
	// 83260AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260AC0 size=56
    let mut pc: u32 = 0x83260AC0;
    'dispatch: loop {
        match pc {
            0x83260AC0 => {
    //   block [0x83260AC0..0x83260AF8)
	// 83260AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260ACC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260AD4: 386BF7B4  addi r3, r11, -0x84c
	ctx.r[3].s64 = ctx.r[11].s64 + -2124;
	// 83260AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260ADC: 4AF9327D  bl 0x821f3d58
	ctx.lr = 0x83260AE0;
	sub_821F3D58(ctx, base);
	// 83260AE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260AE4: 906AB120  stw r3, -0x4ee0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20192 as u32), ctx.r[3].u32 ) };
	// 83260AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260AF8 size=56
    let mut pc: u32 = 0x83260AF8;
    'dispatch: loop {
        match pc {
            0x83260AF8 => {
    //   block [0x83260AF8..0x83260B30)
	// 83260AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260B04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260B0C: 386BF7C8  addi r3, r11, -0x838
	ctx.r[3].s64 = ctx.r[11].s64 + -2104;
	// 83260B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260B14: 4AF93245  bl 0x821f3d58
	ctx.lr = 0x83260B18;
	sub_821F3D58(ctx, base);
	// 83260B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260B1C: 906AB124  stw r3, -0x4edc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20188 as u32), ctx.r[3].u32 ) };
	// 83260B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260B30 size=56
    let mut pc: u32 = 0x83260B30;
    'dispatch: loop {
        match pc {
            0x83260B30 => {
    //   block [0x83260B30..0x83260B68)
	// 83260B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260B3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260B44: 386BF7D8  addi r3, r11, -0x828
	ctx.r[3].s64 = ctx.r[11].s64 + -2088;
	// 83260B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260B4C: 4AF9320D  bl 0x821f3d58
	ctx.lr = 0x83260B50;
	sub_821F3D58(ctx, base);
	// 83260B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260B54: 906AB128  stw r3, -0x4ed8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20184 as u32), ctx.r[3].u32 ) };
	// 83260B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260B68 size=56
    let mut pc: u32 = 0x83260B68;
    'dispatch: loop {
        match pc {
            0x83260B68 => {
    //   block [0x83260B68..0x83260BA0)
	// 83260B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260B74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260B7C: 386BF7E4  addi r3, r11, -0x81c
	ctx.r[3].s64 = ctx.r[11].s64 + -2076;
	// 83260B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260B84: 4AF931D5  bl 0x821f3d58
	ctx.lr = 0x83260B88;
	sub_821F3D58(ctx, base);
	// 83260B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260B8C: 906AB12C  stw r3, -0x4ed4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20180 as u32), ctx.r[3].u32 ) };
	// 83260B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260BA0 size=56
    let mut pc: u32 = 0x83260BA0;
    'dispatch: loop {
        match pc {
            0x83260BA0 => {
    //   block [0x83260BA0..0x83260BD8)
	// 83260BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260BAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83260BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260BB4: 386B260C  addi r3, r11, 0x260c
	ctx.r[3].s64 = ctx.r[11].s64 + 9740;
	// 83260BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260BBC: 4AF9319D  bl 0x821f3d58
	ctx.lr = 0x83260BC0;
	sub_821F3D58(ctx, base);
	// 83260BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260BC4: 906AB130  stw r3, -0x4ed0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20176 as u32), ctx.r[3].u32 ) };
	// 83260BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260BD8 size=56
    let mut pc: u32 = 0x83260BD8;
    'dispatch: loop {
        match pc {
            0x83260BD8 => {
    //   block [0x83260BD8..0x83260C10)
	// 83260BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260BE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260BEC: 386BF7F0  addi r3, r11, -0x810
	ctx.r[3].s64 = ctx.r[11].s64 + -2064;
	// 83260BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260BF4: 4AF93165  bl 0x821f3d58
	ctx.lr = 0x83260BF8;
	sub_821F3D58(ctx, base);
	// 83260BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260BFC: 906AB134  stw r3, -0x4ecc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20172 as u32), ctx.r[3].u32 ) };
	// 83260C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260C10 size=56
    let mut pc: u32 = 0x83260C10;
    'dispatch: loop {
        match pc {
            0x83260C10 => {
    //   block [0x83260C10..0x83260C48)
	// 83260C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260C1C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260C24: 386B26C0  addi r3, r11, 0x26c0
	ctx.r[3].s64 = ctx.r[11].s64 + 9920;
	// 83260C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260C2C: 4AF9312D  bl 0x821f3d58
	ctx.lr = 0x83260C30;
	sub_821F3D58(ctx, base);
	// 83260C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260C34: 906AB138  stw r3, -0x4ec8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20168 as u32), ctx.r[3].u32 ) };
	// 83260C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260C48 size=56
    let mut pc: u32 = 0x83260C48;
    'dispatch: loop {
        match pc {
            0x83260C48 => {
    //   block [0x83260C48..0x83260C80)
	// 83260C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260C54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260C5C: 386BF7FC  addi r3, r11, -0x804
	ctx.r[3].s64 = ctx.r[11].s64 + -2052;
	// 83260C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260C64: 4AF930F5  bl 0x821f3d58
	ctx.lr = 0x83260C68;
	sub_821F3D58(ctx, base);
	// 83260C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260C6C: 906AB13C  stw r3, -0x4ec4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20164 as u32), ctx.r[3].u32 ) };
	// 83260C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260C80 size=56
    let mut pc: u32 = 0x83260C80;
    'dispatch: loop {
        match pc {
            0x83260C80 => {
    //   block [0x83260C80..0x83260CB8)
	// 83260C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260C8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260C94: 386B26D8  addi r3, r11, 0x26d8
	ctx.r[3].s64 = ctx.r[11].s64 + 9944;
	// 83260C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260C9C: 4AF930BD  bl 0x821f3d58
	ctx.lr = 0x83260CA0;
	sub_821F3D58(ctx, base);
	// 83260CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260CA4: 906AB140  stw r3, -0x4ec0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20160 as u32), ctx.r[3].u32 ) };
	// 83260CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260CB8 size=56
    let mut pc: u32 = 0x83260CB8;
    'dispatch: loop {
        match pc {
            0x83260CB8 => {
    //   block [0x83260CB8..0x83260CF0)
	// 83260CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260CC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260CCC: 386BF808  addi r3, r11, -0x7f8
	ctx.r[3].s64 = ctx.r[11].s64 + -2040;
	// 83260CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260CD4: 4AF93085  bl 0x821f3d58
	ctx.lr = 0x83260CD8;
	sub_821F3D58(ctx, base);
	// 83260CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260CDC: 906AB144  stw r3, -0x4ebc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20156 as u32), ctx.r[3].u32 ) };
	// 83260CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260CF0 size=56
    let mut pc: u32 = 0x83260CF0;
    'dispatch: loop {
        match pc {
            0x83260CF0 => {
    //   block [0x83260CF0..0x83260D28)
	// 83260CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260CFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260D04: 386BF818  addi r3, r11, -0x7e8
	ctx.r[3].s64 = ctx.r[11].s64 + -2024;
	// 83260D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260D0C: 4AF9304D  bl 0x821f3d58
	ctx.lr = 0x83260D10;
	sub_821F3D58(ctx, base);
	// 83260D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260D14: 906AB148  stw r3, -0x4eb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20152 as u32), ctx.r[3].u32 ) };
	// 83260D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260D28 size=56
    let mut pc: u32 = 0x83260D28;
    'dispatch: loop {
        match pc {
            0x83260D28 => {
    //   block [0x83260D28..0x83260D60)
	// 83260D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260D34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260D3C: 386BF820  addi r3, r11, -0x7e0
	ctx.r[3].s64 = ctx.r[11].s64 + -2016;
	// 83260D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260D44: 4AF93015  bl 0x821f3d58
	ctx.lr = 0x83260D48;
	sub_821F3D58(ctx, base);
	// 83260D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260D4C: 906AB14C  stw r3, -0x4eb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20148 as u32), ctx.r[3].u32 ) };
	// 83260D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260D60 size=64
    let mut pc: u32 = 0x83260D60;
    'dispatch: loop {
        match pc {
            0x83260D60 => {
    //   block [0x83260D60..0x83260DA0)
	// 83260D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260D6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260D74: 388BF828  addi r4, r11, -0x7d8
	ctx.r[4].s64 = ctx.r[11].s64 + -2008;
	// 83260D78: 386AB150  addi r3, r10, -0x4eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -20144;
	// 83260D7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260D80: 4AFCC151  bl 0x8222ced0
	ctx.lr = 0x83260D84;
	sub_8222CED0(ctx, base);
	// 83260D84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260D88: 3869C8E0  addi r3, r9, -0x3720
	ctx.r[3].s64 = ctx.r[9].s64 + -14112;
	// 83260D8C: 4BA49195  bl 0x82ca9f20
	ctx.lr = 0x83260D90;
	sub_82CA9F20(ctx, base);
	// 83260D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260DA0 size=64
    let mut pc: u32 = 0x83260DA0;
    'dispatch: loop {
        match pc {
            0x83260DA0 => {
    //   block [0x83260DA0..0x83260DE0)
	// 83260DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260DAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260DB4: 388BF84C  addi r4, r11, -0x7b4
	ctx.r[4].s64 = ctx.r[11].s64 + -1972;
	// 83260DB8: 386AB154  addi r3, r10, -0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + -20140;
	// 83260DBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260DC0: 4AFCC111  bl 0x8222ced0
	ctx.lr = 0x83260DC4;
	sub_8222CED0(ctx, base);
	// 83260DC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260DC8: 3869C8F0  addi r3, r9, -0x3710
	ctx.r[3].s64 = ctx.r[9].s64 + -14096;
	// 83260DCC: 4BA49155  bl 0x82ca9f20
	ctx.lr = 0x83260DD0;
	sub_82CA9F20(ctx, base);
	// 83260DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260DE0 size=64
    let mut pc: u32 = 0x83260DE0;
    'dispatch: loop {
        match pc {
            0x83260DE0 => {
    //   block [0x83260DE0..0x83260E20)
	// 83260DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260DEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260DF4: 388BF870  addi r4, r11, -0x790
	ctx.r[4].s64 = ctx.r[11].s64 + -1936;
	// 83260DF8: 386AB158  addi r3, r10, -0x4ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -20136;
	// 83260DFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260E00: 4AFCC0D1  bl 0x8222ced0
	ctx.lr = 0x83260E04;
	sub_8222CED0(ctx, base);
	// 83260E04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260E08: 3869C900  addi r3, r9, -0x3700
	ctx.r[3].s64 = ctx.r[9].s64 + -14080;
	// 83260E0C: 4BA49115  bl 0x82ca9f20
	ctx.lr = 0x83260E10;
	sub_82CA9F20(ctx, base);
	// 83260E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260E20 size=64
    let mut pc: u32 = 0x83260E20;
    'dispatch: loop {
        match pc {
            0x83260E20 => {
    //   block [0x83260E20..0x83260E60)
	// 83260E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260E2C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260E30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260E34: 388BA894  addi r4, r11, -0x576c
	ctx.r[4].s64 = ctx.r[11].s64 + -22380;
	// 83260E38: 386AB15C  addi r3, r10, -0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -20132;
	// 83260E3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260E40: 4AFCC091  bl 0x8222ced0
	ctx.lr = 0x83260E44;
	sub_8222CED0(ctx, base);
	// 83260E44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260E48: 3869C910  addi r3, r9, -0x36f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14064;
	// 83260E4C: 4BA490D5  bl 0x82ca9f20
	ctx.lr = 0x83260E50;
	sub_82CA9F20(ctx, base);
	// 83260E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260E60 size=64
    let mut pc: u32 = 0x83260E60;
    'dispatch: loop {
        match pc {
            0x83260E60 => {
    //   block [0x83260E60..0x83260EA0)
	// 83260E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260E6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260E74: 388BA8B0  addi r4, r11, -0x5750
	ctx.r[4].s64 = ctx.r[11].s64 + -22352;
	// 83260E78: 386AB160  addi r3, r10, -0x4ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -20128;
	// 83260E7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260E80: 4AFCC051  bl 0x8222ced0
	ctx.lr = 0x83260E84;
	sub_8222CED0(ctx, base);
	// 83260E84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260E88: 3869C920  addi r3, r9, -0x36e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14048;
	// 83260E8C: 4BA49095  bl 0x82ca9f20
	ctx.lr = 0x83260E90;
	sub_82CA9F20(ctx, base);
	// 83260E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260EA0 size=64
    let mut pc: u32 = 0x83260EA0;
    'dispatch: loop {
        match pc {
            0x83260EA0 => {
    //   block [0x83260EA0..0x83260EE0)
	// 83260EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260EAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260EB4: 388BF890  addi r4, r11, -0x770
	ctx.r[4].s64 = ctx.r[11].s64 + -1904;
	// 83260EB8: 386AB164  addi r3, r10, -0x4e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -20124;
	// 83260EBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260EC0: 4AFCC011  bl 0x8222ced0
	ctx.lr = 0x83260EC4;
	sub_8222CED0(ctx, base);
	// 83260EC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260EC8: 3869C930  addi r3, r9, -0x36d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14032;
	// 83260ECC: 4BA49055  bl 0x82ca9f20
	ctx.lr = 0x83260ED0;
	sub_82CA9F20(ctx, base);
	// 83260ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260EE0 size=64
    let mut pc: u32 = 0x83260EE0;
    'dispatch: loop {
        match pc {
            0x83260EE0 => {
    //   block [0x83260EE0..0x83260F20)
	// 83260EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260EEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260EF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260EF4: 388BF8B0  addi r4, r11, -0x750
	ctx.r[4].s64 = ctx.r[11].s64 + -1872;
	// 83260EF8: 386AB168  addi r3, r10, -0x4e98
	ctx.r[3].s64 = ctx.r[10].s64 + -20120;
	// 83260EFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260F00: 4AFCBFD1  bl 0x8222ced0
	ctx.lr = 0x83260F04;
	sub_8222CED0(ctx, base);
	// 83260F04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260F08: 3869C940  addi r3, r9, -0x36c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14016;
	// 83260F0C: 4BA49015  bl 0x82ca9f20
	ctx.lr = 0x83260F10;
	sub_82CA9F20(ctx, base);
	// 83260F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260F20 size=64
    let mut pc: u32 = 0x83260F20;
    'dispatch: loop {
        match pc {
            0x83260F20 => {
    //   block [0x83260F20..0x83260F60)
	// 83260F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260F2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260F34: 388BE214  addi r4, r11, -0x1dec
	ctx.r[4].s64 = ctx.r[11].s64 + -7660;
	// 83260F38: 386AB16C  addi r3, r10, -0x4e94
	ctx.r[3].s64 = ctx.r[10].s64 + -20116;
	// 83260F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260F40: 4AFCBF91  bl 0x8222ced0
	ctx.lr = 0x83260F44;
	sub_8222CED0(ctx, base);
	// 83260F44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260F48: 3869C950  addi r3, r9, -0x36b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14000;
	// 83260F4C: 4BA48FD5  bl 0x82ca9f20
	ctx.lr = 0x83260F50;
	sub_82CA9F20(ctx, base);
	// 83260F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260F60 size=64
    let mut pc: u32 = 0x83260F60;
    'dispatch: loop {
        match pc {
            0x83260F60 => {
    //   block [0x83260F60..0x83260FA0)
	// 83260F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260F6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260F74: 388BF8CC  addi r4, r11, -0x734
	ctx.r[4].s64 = ctx.r[11].s64 + -1844;
	// 83260F78: 386AB170  addi r3, r10, -0x4e90
	ctx.r[3].s64 = ctx.r[10].s64 + -20112;
	// 83260F7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260F80: 4AFCBF51  bl 0x8222ced0
	ctx.lr = 0x83260F84;
	sub_8222CED0(ctx, base);
	// 83260F84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260F88: 3869C960  addi r3, r9, -0x36a0
	ctx.r[3].s64 = ctx.r[9].s64 + -13984;
	// 83260F8C: 4BA48F95  bl 0x82ca9f20
	ctx.lr = 0x83260F90;
	sub_82CA9F20(ctx, base);
	// 83260F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260FA0 size=64
    let mut pc: u32 = 0x83260FA0;
    'dispatch: loop {
        match pc {
            0x83260FA0 => {
    //   block [0x83260FA0..0x83260FE0)
	// 83260FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260FAC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260FB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260FB4: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 83260FB8: 386AB174  addi r3, r10, -0x4e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -20108;
	// 83260FBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260FC0: 4AFCBF11  bl 0x8222ced0
	ctx.lr = 0x83260FC4;
	sub_8222CED0(ctx, base);
	// 83260FC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260FC8: 3869C970  addi r3, r9, -0x3690
	ctx.r[3].s64 = ctx.r[9].s64 + -13968;
	// 83260FCC: 4BA48F55  bl 0x82ca9f20
	ctx.lr = 0x83260FD0;
	sub_82CA9F20(ctx, base);
	// 83260FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260FE0 size=64
    let mut pc: u32 = 0x83260FE0;
    'dispatch: loop {
        match pc {
            0x83260FE0 => {
    //   block [0x83260FE0..0x83261020)
	// 83260FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260FEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260FF4: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 83260FF8: 386AB178  addi r3, r10, -0x4e88
	ctx.r[3].s64 = ctx.r[10].s64 + -20104;
	// 83260FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261000: 4AFCBED1  bl 0x8222ced0
	ctx.lr = 0x83261004;
	sub_8222CED0(ctx, base);
	// 83261004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261008: 3869C980  addi r3, r9, -0x3680
	ctx.r[3].s64 = ctx.r[9].s64 + -13952;
	// 8326100C: 4BA48F15  bl 0x82ca9f20
	ctx.lr = 0x83261010;
	sub_82CA9F20(ctx, base);
	// 83261010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261020 size=64
    let mut pc: u32 = 0x83261020;
    'dispatch: loop {
        match pc {
            0x83261020 => {
    //   block [0x83261020..0x83261060)
	// 83261020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326102C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261034: 388B28E4  addi r4, r11, 0x28e4
	ctx.r[4].s64 = ctx.r[11].s64 + 10468;
	// 83261038: 386AB17C  addi r3, r10, -0x4e84
	ctx.r[3].s64 = ctx.r[10].s64 + -20100;
	// 8326103C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261040: 4AFCBE91  bl 0x8222ced0
	ctx.lr = 0x83261044;
	sub_8222CED0(ctx, base);
	// 83261044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261048: 3869C990  addi r3, r9, -0x3670
	ctx.r[3].s64 = ctx.r[9].s64 + -13936;
	// 8326104C: 4BA48ED5  bl 0x82ca9f20
	ctx.lr = 0x83261050;
	sub_82CA9F20(ctx, base);
	// 83261050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261060 size=64
    let mut pc: u32 = 0x83261060;
    'dispatch: loop {
        match pc {
            0x83261060 => {
    //   block [0x83261060..0x832610A0)
	// 83261060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326106C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261074: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 83261078: 386AB180  addi r3, r10, -0x4e80
	ctx.r[3].s64 = ctx.r[10].s64 + -20096;
	// 8326107C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261080: 4AFCBE51  bl 0x8222ced0
	ctx.lr = 0x83261084;
	sub_8222CED0(ctx, base);
	// 83261084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261088: 3869C9A0  addi r3, r9, -0x3660
	ctx.r[3].s64 = ctx.r[9].s64 + -13920;
	// 8326108C: 4BA48E95  bl 0x82ca9f20
	ctx.lr = 0x83261090;
	sub_82CA9F20(ctx, base);
	// 83261090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326109C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832610A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832610A0 size=64
    let mut pc: u32 = 0x832610A0;
    'dispatch: loop {
        match pc {
            0x832610A0 => {
    //   block [0x832610A0..0x832610E0)
	// 832610A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832610A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832610A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832610AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832610B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832610B4: 388B28F8  addi r4, r11, 0x28f8
	ctx.r[4].s64 = ctx.r[11].s64 + 10488;
	// 832610B8: 386AB184  addi r3, r10, -0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -20092;
	// 832610BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832610C0: 4AFCBE11  bl 0x8222ced0
	ctx.lr = 0x832610C4;
	sub_8222CED0(ctx, base);
	// 832610C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832610C8: 3869C9B0  addi r3, r9, -0x3650
	ctx.r[3].s64 = ctx.r[9].s64 + -13904;
	// 832610CC: 4BA48E55  bl 0x82ca9f20
	ctx.lr = 0x832610D0;
	sub_82CA9F20(ctx, base);
	// 832610D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832610D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832610D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832610DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832610E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832610E0 size=64
    let mut pc: u32 = 0x832610E0;
    'dispatch: loop {
        match pc {
            0x832610E0 => {
    //   block [0x832610E0..0x83261120)
	// 832610E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832610E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832610E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832610EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832610F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832610F4: 388B2904  addi r4, r11, 0x2904
	ctx.r[4].s64 = ctx.r[11].s64 + 10500;
	// 832610F8: 386AB188  addi r3, r10, -0x4e78
	ctx.r[3].s64 = ctx.r[10].s64 + -20088;
	// 832610FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261100: 4AFCBDD1  bl 0x8222ced0
	ctx.lr = 0x83261104;
	sub_8222CED0(ctx, base);
	// 83261104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261108: 3869C9C0  addi r3, r9, -0x3640
	ctx.r[3].s64 = ctx.r[9].s64 + -13888;
	// 8326110C: 4BA48E15  bl 0x82ca9f20
	ctx.lr = 0x83261110;
	sub_82CA9F20(ctx, base);
	// 83261110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326111C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261120 size=64
    let mut pc: u32 = 0x83261120;
    'dispatch: loop {
        match pc {
            0x83261120 => {
    //   block [0x83261120..0x83261160)
	// 83261120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326112C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261134: 388B2914  addi r4, r11, 0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + 10516;
	// 83261138: 386AB18C  addi r3, r10, -0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + -20084;
	// 8326113C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261140: 4AFCBD91  bl 0x8222ced0
	ctx.lr = 0x83261144;
	sub_8222CED0(ctx, base);
	// 83261144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261148: 3869C9D0  addi r3, r9, -0x3630
	ctx.r[3].s64 = ctx.r[9].s64 + -13872;
	// 8326114C: 4BA48DD5  bl 0x82ca9f20
	ctx.lr = 0x83261150;
	sub_82CA9F20(ctx, base);
	// 83261150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326115C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261160 size=64
    let mut pc: u32 = 0x83261160;
    'dispatch: loop {
        match pc {
            0x83261160 => {
    //   block [0x83261160..0x832611A0)
	// 83261160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326116C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261174: 388B291C  addi r4, r11, 0x291c
	ctx.r[4].s64 = ctx.r[11].s64 + 10524;
	// 83261178: 386AB190  addi r3, r10, -0x4e70
	ctx.r[3].s64 = ctx.r[10].s64 + -20080;
	// 8326117C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261180: 4AFCBD51  bl 0x8222ced0
	ctx.lr = 0x83261184;
	sub_8222CED0(ctx, base);
	// 83261184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261188: 3869C9E0  addi r3, r9, -0x3620
	ctx.r[3].s64 = ctx.r[9].s64 + -13856;
	// 8326118C: 4BA48D95  bl 0x82ca9f20
	ctx.lr = 0x83261190;
	sub_82CA9F20(ctx, base);
	// 83261190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326119C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832611A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832611A0 size=64
    let mut pc: u32 = 0x832611A0;
    'dispatch: loop {
        match pc {
            0x832611A0 => {
    //   block [0x832611A0..0x832611E0)
	// 832611A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832611A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832611A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832611AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832611B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832611B4: 388B292C  addi r4, r11, 0x292c
	ctx.r[4].s64 = ctx.r[11].s64 + 10540;
	// 832611B8: 386AB194  addi r3, r10, -0x4e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -20076;
	// 832611BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832611C0: 4AFCBD11  bl 0x8222ced0
	ctx.lr = 0x832611C4;
	sub_8222CED0(ctx, base);
	// 832611C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832611C8: 3869C9F0  addi r3, r9, -0x3610
	ctx.r[3].s64 = ctx.r[9].s64 + -13840;
	// 832611CC: 4BA48D55  bl 0x82ca9f20
	ctx.lr = 0x832611D0;
	sub_82CA9F20(ctx, base);
	// 832611D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832611D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832611D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832611DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832611E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832611E0 size=64
    let mut pc: u32 = 0x832611E0;
    'dispatch: loop {
        match pc {
            0x832611E0 => {
    //   block [0x832611E0..0x83261220)
	// 832611E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832611E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832611E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832611EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832611F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832611F4: 388B2934  addi r4, r11, 0x2934
	ctx.r[4].s64 = ctx.r[11].s64 + 10548;
	// 832611F8: 386AB198  addi r3, r10, -0x4e68
	ctx.r[3].s64 = ctx.r[10].s64 + -20072;
	// 832611FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261200: 4AFCBCD1  bl 0x8222ced0
	ctx.lr = 0x83261204;
	sub_8222CED0(ctx, base);
	// 83261204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261208: 3869CA00  addi r3, r9, -0x3600
	ctx.r[3].s64 = ctx.r[9].s64 + -13824;
	// 8326120C: 4BA48D15  bl 0x82ca9f20
	ctx.lr = 0x83261210;
	sub_82CA9F20(ctx, base);
	// 83261210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261220 size=64
    let mut pc: u32 = 0x83261220;
    'dispatch: loop {
        match pc {
            0x83261220 => {
    //   block [0x83261220..0x83261260)
	// 83261220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326122C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261234: 388B293C  addi r4, r11, 0x293c
	ctx.r[4].s64 = ctx.r[11].s64 + 10556;
	// 83261238: 386AB19C  addi r3, r10, -0x4e64
	ctx.r[3].s64 = ctx.r[10].s64 + -20068;
	// 8326123C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261240: 4AFCBC91  bl 0x8222ced0
	ctx.lr = 0x83261244;
	sub_8222CED0(ctx, base);
	// 83261244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261248: 3869CA10  addi r3, r9, -0x35f0
	ctx.r[3].s64 = ctx.r[9].s64 + -13808;
	// 8326124C: 4BA48CD5  bl 0x82ca9f20
	ctx.lr = 0x83261250;
	sub_82CA9F20(ctx, base);
	// 83261250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261260 size=64
    let mut pc: u32 = 0x83261260;
    'dispatch: loop {
        match pc {
            0x83261260 => {
    //   block [0x83261260..0x832612A0)
	// 83261260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326126C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261274: 388B2948  addi r4, r11, 0x2948
	ctx.r[4].s64 = ctx.r[11].s64 + 10568;
	// 83261278: 386AB1A0  addi r3, r10, -0x4e60
	ctx.r[3].s64 = ctx.r[10].s64 + -20064;
	// 8326127C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261280: 4AFCBC51  bl 0x8222ced0
	ctx.lr = 0x83261284;
	sub_8222CED0(ctx, base);
	// 83261284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261288: 3869CA20  addi r3, r9, -0x35e0
	ctx.r[3].s64 = ctx.r[9].s64 + -13792;
	// 8326128C: 4BA48C95  bl 0x82ca9f20
	ctx.lr = 0x83261290;
	sub_82CA9F20(ctx, base);
	// 83261290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326129C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832612A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832612A0 size=64
    let mut pc: u32 = 0x832612A0;
    'dispatch: loop {
        match pc {
            0x832612A0 => {
    //   block [0x832612A0..0x832612E0)
	// 832612A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832612A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832612A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832612AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832612B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832612B4: 388B2950  addi r4, r11, 0x2950
	ctx.r[4].s64 = ctx.r[11].s64 + 10576;
	// 832612B8: 386AB1A4  addi r3, r10, -0x4e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20060;
	// 832612BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832612C0: 4AFCBC11  bl 0x8222ced0
	ctx.lr = 0x832612C4;
	sub_8222CED0(ctx, base);
	// 832612C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832612C8: 3869CA30  addi r3, r9, -0x35d0
	ctx.r[3].s64 = ctx.r[9].s64 + -13776;
	// 832612CC: 4BA48C55  bl 0x82ca9f20
	ctx.lr = 0x832612D0;
	sub_82CA9F20(ctx, base);
	// 832612D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832612D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832612D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832612DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832612E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832612E0 size=64
    let mut pc: u32 = 0x832612E0;
    'dispatch: loop {
        match pc {
            0x832612E0 => {
    //   block [0x832612E0..0x83261320)
	// 832612E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832612E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832612E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832612EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832612F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832612F4: 388B2958  addi r4, r11, 0x2958
	ctx.r[4].s64 = ctx.r[11].s64 + 10584;
	// 832612F8: 386AB1A8  addi r3, r10, -0x4e58
	ctx.r[3].s64 = ctx.r[10].s64 + -20056;
	// 832612FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261300: 4AFCBBD1  bl 0x8222ced0
	ctx.lr = 0x83261304;
	sub_8222CED0(ctx, base);
	// 83261304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261308: 3869CA40  addi r3, r9, -0x35c0
	ctx.r[3].s64 = ctx.r[9].s64 + -13760;
	// 8326130C: 4BA48C15  bl 0x82ca9f20
	ctx.lr = 0x83261310;
	sub_82CA9F20(ctx, base);
	// 83261310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326131C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261320 size=64
    let mut pc: u32 = 0x83261320;
    'dispatch: loop {
        match pc {
            0x83261320 => {
    //   block [0x83261320..0x83261360)
	// 83261320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326132C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261334: 388B2960  addi r4, r11, 0x2960
	ctx.r[4].s64 = ctx.r[11].s64 + 10592;
	// 83261338: 386AB1AC  addi r3, r10, -0x4e54
	ctx.r[3].s64 = ctx.r[10].s64 + -20052;
	// 8326133C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261340: 4AFCBB91  bl 0x8222ced0
	ctx.lr = 0x83261344;
	sub_8222CED0(ctx, base);
	// 83261344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261348: 3869CA50  addi r3, r9, -0x35b0
	ctx.r[3].s64 = ctx.r[9].s64 + -13744;
	// 8326134C: 4BA48BD5  bl 0x82ca9f20
	ctx.lr = 0x83261350;
	sub_82CA9F20(ctx, base);
	// 83261350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261360 size=64
    let mut pc: u32 = 0x83261360;
    'dispatch: loop {
        match pc {
            0x83261360 => {
    //   block [0x83261360..0x832613A0)
	// 83261360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326136C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261374: 388B2968  addi r4, r11, 0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + 10600;
	// 83261378: 386AB1B0  addi r3, r10, -0x4e50
	ctx.r[3].s64 = ctx.r[10].s64 + -20048;
	// 8326137C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261380: 4AFCBB51  bl 0x8222ced0
	ctx.lr = 0x83261384;
	sub_8222CED0(ctx, base);
	// 83261384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261388: 3869CA60  addi r3, r9, -0x35a0
	ctx.r[3].s64 = ctx.r[9].s64 + -13728;
	// 8326138C: 4BA48B95  bl 0x82ca9f20
	ctx.lr = 0x83261390;
	sub_82CA9F20(ctx, base);
	// 83261390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326139C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832613A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832613A0 size=64
    let mut pc: u32 = 0x832613A0;
    'dispatch: loop {
        match pc {
            0x832613A0 => {
    //   block [0x832613A0..0x832613E0)
	// 832613A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832613A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832613A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832613AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832613B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832613B4: 388B2970  addi r4, r11, 0x2970
	ctx.r[4].s64 = ctx.r[11].s64 + 10608;
	// 832613B8: 386AB1B4  addi r3, r10, -0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -20044;
	// 832613BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832613C0: 4AFCBB11  bl 0x8222ced0
	ctx.lr = 0x832613C4;
	sub_8222CED0(ctx, base);
	// 832613C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832613C8: 3869CA70  addi r3, r9, -0x3590
	ctx.r[3].s64 = ctx.r[9].s64 + -13712;
	// 832613CC: 4BA48B55  bl 0x82ca9f20
	ctx.lr = 0x832613D0;
	sub_82CA9F20(ctx, base);
	// 832613D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832613D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832613D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832613DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832613E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832613E0 size=64
    let mut pc: u32 = 0x832613E0;
    'dispatch: loop {
        match pc {
            0x832613E0 => {
    //   block [0x832613E0..0x83261420)
	// 832613E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832613E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832613E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832613EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832613F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832613F4: 388B2978  addi r4, r11, 0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + 10616;
	// 832613F8: 386AB1B8  addi r3, r10, -0x4e48
	ctx.r[3].s64 = ctx.r[10].s64 + -20040;
	// 832613FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261400: 4AFCBAD1  bl 0x8222ced0
	ctx.lr = 0x83261404;
	sub_8222CED0(ctx, base);
	// 83261404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261408: 3869CA80  addi r3, r9, -0x3580
	ctx.r[3].s64 = ctx.r[9].s64 + -13696;
	// 8326140C: 4BA48B15  bl 0x82ca9f20
	ctx.lr = 0x83261410;
	sub_82CA9F20(ctx, base);
	// 83261410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261420 size=64
    let mut pc: u32 = 0x83261420;
    'dispatch: loop {
        match pc {
            0x83261420 => {
    //   block [0x83261420..0x83261460)
	// 83261420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326142C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261434: 388B2980  addi r4, r11, 0x2980
	ctx.r[4].s64 = ctx.r[11].s64 + 10624;
	// 83261438: 386AB1BC  addi r3, r10, -0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + -20036;
	// 8326143C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261440: 4AFCBA91  bl 0x8222ced0
	ctx.lr = 0x83261444;
	sub_8222CED0(ctx, base);
	// 83261444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261448: 3869CA90  addi r3, r9, -0x3570
	ctx.r[3].s64 = ctx.r[9].s64 + -13680;
	// 8326144C: 4BA48AD5  bl 0x82ca9f20
	ctx.lr = 0x83261450;
	sub_82CA9F20(ctx, base);
	// 83261450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326145C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261460 size=64
    let mut pc: u32 = 0x83261460;
    'dispatch: loop {
        match pc {
            0x83261460 => {
    //   block [0x83261460..0x832614A0)
	// 83261460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326146C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261474: 388B298C  addi r4, r11, 0x298c
	ctx.r[4].s64 = ctx.r[11].s64 + 10636;
	// 83261478: 386AB1C0  addi r3, r10, -0x4e40
	ctx.r[3].s64 = ctx.r[10].s64 + -20032;
	// 8326147C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261480: 4AFCBA51  bl 0x8222ced0
	ctx.lr = 0x83261484;
	sub_8222CED0(ctx, base);
	// 83261484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261488: 3869CAA0  addi r3, r9, -0x3560
	ctx.r[3].s64 = ctx.r[9].s64 + -13664;
	// 8326148C: 4BA48A95  bl 0x82ca9f20
	ctx.lr = 0x83261490;
	sub_82CA9F20(ctx, base);
	// 83261490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326149C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832614A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832614A0 size=64
    let mut pc: u32 = 0x832614A0;
    'dispatch: loop {
        match pc {
            0x832614A0 => {
    //   block [0x832614A0..0x832614E0)
	// 832614A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832614A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832614A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832614AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832614B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832614B4: 388B8CD0  addi r4, r11, -0x7330
	ctx.r[4].s64 = ctx.r[11].s64 + -29488;
	// 832614B8: 386AB1C4  addi r3, r10, -0x4e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -20028;
	// 832614BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832614C0: 4AFCBA11  bl 0x8222ced0
	ctx.lr = 0x832614C4;
	sub_8222CED0(ctx, base);
	// 832614C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832614C8: 3869CAB0  addi r3, r9, -0x3550
	ctx.r[3].s64 = ctx.r[9].s64 + -13648;
	// 832614CC: 4BA48A55  bl 0x82ca9f20
	ctx.lr = 0x832614D0;
	sub_82CA9F20(ctx, base);
	// 832614D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832614D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832614D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832614DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832614E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832614E0 size=56
    let mut pc: u32 = 0x832614E0;
    'dispatch: loop {
        match pc {
            0x832614E0 => {
    //   block [0x832614E0..0x83261518)
	// 832614E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832614E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832614E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832614EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832614F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832614F4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832614F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832614FC: 4AF9285D  bl 0x821f3d58
	ctx.lr = 0x83261500;
	sub_821F3D58(ctx, base);
	// 83261500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261504: 906AB1C8  stw r3, -0x4e38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20024 as u32), ctx.r[3].u32 ) };
	// 83261508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261518 size=56
    let mut pc: u32 = 0x83261518;
    'dispatch: loop {
        match pc {
            0x83261518 => {
    //   block [0x83261518..0x83261550)
	// 83261518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326152C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83261530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261534: 4AF92825  bl 0x821f3d58
	ctx.lr = 0x83261538;
	sub_821F3D58(ctx, base);
	// 83261538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326153C: 906AB1CC  stw r3, -0x4e34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20020 as u32), ctx.r[3].u32 ) };
	// 83261540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261550 size=56
    let mut pc: u32 = 0x83261550;
    'dispatch: loop {
        match pc {
            0x83261550 => {
    //   block [0x83261550..0x83261588)
	// 83261550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326155C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261564: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83261568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326156C: 4AF927ED  bl 0x821f3d58
	ctx.lr = 0x83261570;
	sub_821F3D58(ctx, base);
	// 83261570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261574: 906AB1D0  stw r3, -0x4e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20016 as u32), ctx.r[3].u32 ) };
	// 83261578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326157C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261588 size=56
    let mut pc: u32 = 0x83261588;
    'dispatch: loop {
        match pc {
            0x83261588 => {
    //   block [0x83261588..0x832615C0)
	// 83261588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326158C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326159C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832615A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832615A4: 4AF927B5  bl 0x821f3d58
	ctx.lr = 0x832615A8;
	sub_821F3D58(ctx, base);
	// 832615A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832615AC: 906AB1D4  stw r3, -0x4e2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20012 as u32), ctx.r[3].u32 ) };
	// 832615B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832615B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832615B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832615BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832615C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832615C0 size=56
    let mut pc: u32 = 0x832615C0;
    'dispatch: loop {
        match pc {
            0x832615C0 => {
    //   block [0x832615C0..0x832615F8)
	// 832615C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832615C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832615C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832615CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832615D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832615D4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832615D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832615DC: 4AF9277D  bl 0x821f3d58
	ctx.lr = 0x832615E0;
	sub_821F3D58(ctx, base);
	// 832615E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832615E4: 906AB1D8  stw r3, -0x4e28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20008 as u32), ctx.r[3].u32 ) };
	// 832615E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832615EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832615F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832615F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832615F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832615F8 size=56
    let mut pc: u32 = 0x832615F8;
    'dispatch: loop {
        match pc {
            0x832615F8 => {
    //   block [0x832615F8..0x83261630)
	// 832615F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832615FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326160C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83261610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261614: 4AF92745  bl 0x821f3d58
	ctx.lr = 0x83261618;
	sub_821F3D58(ctx, base);
	// 83261618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326161C: 906AB1DC  stw r3, -0x4e24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20004 as u32), ctx.r[3].u32 ) };
	// 83261620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326162C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261630 size=56
    let mut pc: u32 = 0x83261630;
    'dispatch: loop {
        match pc {
            0x83261630 => {
    //   block [0x83261630..0x83261668)
	// 83261630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326163C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261644: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83261648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326164C: 4AF9270D  bl 0x821f3d58
	ctx.lr = 0x83261650;
	sub_821F3D58(ctx, base);
	// 83261650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261654: 906AB1E0  stw r3, -0x4e20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20000 as u32), ctx.r[3].u32 ) };
	// 83261658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326165C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261668 size=56
    let mut pc: u32 = 0x83261668;
    'dispatch: loop {
        match pc {
            0x83261668 => {
    //   block [0x83261668..0x832616A0)
	// 83261668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326166C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326167C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83261680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261684: 4AF926D5  bl 0x821f3d58
	ctx.lr = 0x83261688;
	sub_821F3D58(ctx, base);
	// 83261688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326168C: 906AB1E4  stw r3, -0x4e1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19996 as u32), ctx.r[3].u32 ) };
	// 83261690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326169C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832616A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832616A0 size=56
    let mut pc: u32 = 0x832616A0;
    'dispatch: loop {
        match pc {
            0x832616A0 => {
    //   block [0x832616A0..0x832616D8)
	// 832616A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832616A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832616A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832616AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832616B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832616B4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832616B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832616BC: 4AF9269D  bl 0x821f3d58
	ctx.lr = 0x832616C0;
	sub_821F3D58(ctx, base);
	// 832616C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832616C4: 906AB1E8  stw r3, -0x4e18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19992 as u32), ctx.r[3].u32 ) };
	// 832616C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832616CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832616D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832616D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832616D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832616D8 size=56
    let mut pc: u32 = 0x832616D8;
    'dispatch: loop {
        match pc {
            0x832616D8 => {
    //   block [0x832616D8..0x83261710)
	// 832616D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832616DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832616E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832616E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832616E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832616EC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832616F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832616F4: 4AF92665  bl 0x821f3d58
	ctx.lr = 0x832616F8;
	sub_821F3D58(ctx, base);
	// 832616F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832616FC: 906AB1EC  stw r3, -0x4e14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19988 as u32), ctx.r[3].u32 ) };
	// 83261700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326170C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261710 size=56
    let mut pc: u32 = 0x83261710;
    'dispatch: loop {
        match pc {
            0x83261710 => {
    //   block [0x83261710..0x83261748)
	// 83261710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326171C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261724: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83261728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326172C: 4AF9262D  bl 0x821f3d58
	ctx.lr = 0x83261730;
	sub_821F3D58(ctx, base);
	// 83261730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261734: 906AB1F0  stw r3, -0x4e10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19984 as u32), ctx.r[3].u32 ) };
	// 83261738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326173C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261748 size=56
    let mut pc: u32 = 0x83261748;
    'dispatch: loop {
        match pc {
            0x83261748 => {
    //   block [0x83261748..0x83261780)
	// 83261748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326174C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326175C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83261760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261764: 4AF925F5  bl 0x821f3d58
	ctx.lr = 0x83261768;
	sub_821F3D58(ctx, base);
	// 83261768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326176C: 906AB1F4  stw r3, -0x4e0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19980 as u32), ctx.r[3].u32 ) };
	// 83261770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261780 size=56
    let mut pc: u32 = 0x83261780;
    'dispatch: loop {
        match pc {
            0x83261780 => {
    //   block [0x83261780..0x832617B8)
	// 83261780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326178C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261794: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83261798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326179C: 4AF925BD  bl 0x821f3d58
	ctx.lr = 0x832617A0;
	sub_821F3D58(ctx, base);
	// 832617A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832617A4: 906AB1F8  stw r3, -0x4e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19976 as u32), ctx.r[3].u32 ) };
	// 832617A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832617AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832617B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832617B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832617B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832617B8 size=56
    let mut pc: u32 = 0x832617B8;
    'dispatch: loop {
        match pc {
            0x832617B8 => {
    //   block [0x832617B8..0x832617F0)
	// 832617B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832617BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832617C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832617C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832617C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832617CC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832617D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832617D4: 4AF92585  bl 0x821f3d58
	ctx.lr = 0x832617D8;
	sub_821F3D58(ctx, base);
	// 832617D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832617DC: 906AB1FC  stw r3, -0x4e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19972 as u32), ctx.r[3].u32 ) };
	// 832617E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832617E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832617E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832617EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832617F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832617F0 size=56
    let mut pc: u32 = 0x832617F0;
    'dispatch: loop {
        match pc {
            0x832617F0 => {
    //   block [0x832617F0..0x83261828)
	// 832617F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832617F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832617F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832617FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261804: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83261808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326180C: 4AF9254D  bl 0x821f3d58
	ctx.lr = 0x83261810;
	sub_821F3D58(ctx, base);
	// 83261810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261814: 906AB200  stw r3, -0x4e00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19968 as u32), ctx.r[3].u32 ) };
	// 83261818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326181C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261828 size=56
    let mut pc: u32 = 0x83261828;
    'dispatch: loop {
        match pc {
            0x83261828 => {
    //   block [0x83261828..0x83261860)
	// 83261828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326183C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83261840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261844: 4AF92515  bl 0x821f3d58
	ctx.lr = 0x83261848;
	sub_821F3D58(ctx, base);
	// 83261848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326184C: 906AB204  stw r3, -0x4dfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19964 as u32), ctx.r[3].u32 ) };
	// 83261850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326185C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261860 size=56
    let mut pc: u32 = 0x83261860;
    'dispatch: loop {
        match pc {
            0x83261860 => {
    //   block [0x83261860..0x83261898)
	// 83261860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326186C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261874: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83261878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326187C: 4AF924DD  bl 0x821f3d58
	ctx.lr = 0x83261880;
	sub_821F3D58(ctx, base);
	// 83261880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261884: 906AB208  stw r3, -0x4df8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19960 as u32), ctx.r[3].u32 ) };
	// 83261888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261898 size=56
    let mut pc: u32 = 0x83261898;
    'dispatch: loop {
        match pc {
            0x83261898 => {
    //   block [0x83261898..0x832618D0)
	// 83261898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832618A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832618A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832618A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832618AC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832618B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832618B4: 4AF924A5  bl 0x821f3d58
	ctx.lr = 0x832618B8;
	sub_821F3D58(ctx, base);
	// 832618B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832618BC: 906AB20C  stw r3, -0x4df4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19956 as u32), ctx.r[3].u32 ) };
	// 832618C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832618C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832618C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832618CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832618D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832618D0 size=56
    let mut pc: u32 = 0x832618D0;
    'dispatch: loop {
        match pc {
            0x832618D0 => {
    //   block [0x832618D0..0x83261908)
	// 832618D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832618D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832618D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832618DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832618E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832618E4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832618E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832618EC: 4AF9246D  bl 0x821f3d58
	ctx.lr = 0x832618F0;
	sub_821F3D58(ctx, base);
	// 832618F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832618F4: 906AB210  stw r3, -0x4df0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19952 as u32), ctx.r[3].u32 ) };
	// 832618F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832618FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261908 size=56
    let mut pc: u32 = 0x83261908;
    'dispatch: loop {
        match pc {
            0x83261908 => {
    //   block [0x83261908..0x83261940)
	// 83261908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326191C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83261920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261924: 4AF92435  bl 0x821f3d58
	ctx.lr = 0x83261928;
	sub_821F3D58(ctx, base);
	// 83261928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326192C: 906AB214  stw r3, -0x4dec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19948 as u32), ctx.r[3].u32 ) };
	// 83261930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326193C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261940 size=56
    let mut pc: u32 = 0x83261940;
    'dispatch: loop {
        match pc {
            0x83261940 => {
    //   block [0x83261940..0x83261978)
	// 83261940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326194C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261954: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83261958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326195C: 4AF923FD  bl 0x821f3d58
	ctx.lr = 0x83261960;
	sub_821F3D58(ctx, base);
	// 83261960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261964: 906AB218  stw r3, -0x4de8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19944 as u32), ctx.r[3].u32 ) };
	// 83261968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261978 size=64
    let mut pc: u32 = 0x83261978;
    'dispatch: loop {
        match pc {
            0x83261978 => {
    //   block [0x83261978..0x832619B8)
	// 83261978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326198C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83261990: 386AB21C  addi r3, r10, -0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + -19940;
	// 83261994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261998: 4AFCB539  bl 0x8222ced0
	ctx.lr = 0x8326199C;
	sub_8222CED0(ctx, base);
	// 8326199C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832619A0: 3869CAC0  addi r3, r9, -0x3540
	ctx.r[3].s64 = ctx.r[9].s64 + -13632;
	// 832619A4: 4BA4857D  bl 0x82ca9f20
	ctx.lr = 0x832619A8;
	sub_82CA9F20(ctx, base);
	// 832619A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832619AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832619B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832619B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832619B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832619B8 size=64
    let mut pc: u32 = 0x832619B8;
    'dispatch: loop {
        match pc {
            0x832619B8 => {
    //   block [0x832619B8..0x832619F8)
	// 832619B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832619BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832619C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832619C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832619C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832619CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832619D0: 386AB220  addi r3, r10, -0x4de0
	ctx.r[3].s64 = ctx.r[10].s64 + -19936;
	// 832619D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832619D8: 4AFCB4F9  bl 0x8222ced0
	ctx.lr = 0x832619DC;
	sub_8222CED0(ctx, base);
	// 832619DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832619E0: 3869CAD0  addi r3, r9, -0x3530
	ctx.r[3].s64 = ctx.r[9].s64 + -13616;
	// 832619E4: 4BA4853D  bl 0x82ca9f20
	ctx.lr = 0x832619E8;
	sub_82CA9F20(ctx, base);
	// 832619E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832619EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832619F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832619F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832619F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832619F8 size=64
    let mut pc: u32 = 0x832619F8;
    'dispatch: loop {
        match pc {
            0x832619F8 => {
    //   block [0x832619F8..0x83261A38)
	// 832619F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832619FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261A0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83261A10: 386AB224  addi r3, r10, -0x4ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -19932;
	// 83261A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261A18: 4AFCB4B9  bl 0x8222ced0
	ctx.lr = 0x83261A1C;
	sub_8222CED0(ctx, base);
	// 83261A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261A20: 3869CAE0  addi r3, r9, -0x3520
	ctx.r[3].s64 = ctx.r[9].s64 + -13600;
	// 83261A24: 4BA484FD  bl 0x82ca9f20
	ctx.lr = 0x83261A28;
	sub_82CA9F20(ctx, base);
	// 83261A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261A38 size=64
    let mut pc: u32 = 0x83261A38;
    'dispatch: loop {
        match pc {
            0x83261A38 => {
    //   block [0x83261A38..0x83261A78)
	// 83261A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261A44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261A4C: 388BF970  addi r4, r11, -0x690
	ctx.r[4].s64 = ctx.r[11].s64 + -1680;
	// 83261A50: 386AB228  addi r3, r10, -0x4dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19928;
	// 83261A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261A58: 4AFCB479  bl 0x8222ced0
	ctx.lr = 0x83261A5C;
	sub_8222CED0(ctx, base);
	// 83261A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261A60: 3869CAF0  addi r3, r9, -0x3510
	ctx.r[3].s64 = ctx.r[9].s64 + -13584;
	// 83261A64: 4BA484BD  bl 0x82ca9f20
	ctx.lr = 0x83261A68;
	sub_82CA9F20(ctx, base);
	// 83261A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261A78 size=64
    let mut pc: u32 = 0x83261A78;
    'dispatch: loop {
        match pc {
            0x83261A78 => {
    //   block [0x83261A78..0x83261AB8)
	// 83261A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261A84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261A8C: 388BF990  addi r4, r11, -0x670
	ctx.r[4].s64 = ctx.r[11].s64 + -1648;
	// 83261A90: 386AB22C  addi r3, r10, -0x4dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -19924;
	// 83261A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261A98: 4AFCB439  bl 0x8222ced0
	ctx.lr = 0x83261A9C;
	sub_8222CED0(ctx, base);
	// 83261A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261AA0: 3869CB00  addi r3, r9, -0x3500
	ctx.r[3].s64 = ctx.r[9].s64 + -13568;
	// 83261AA4: 4BA4847D  bl 0x82ca9f20
	ctx.lr = 0x83261AA8;
	sub_82CA9F20(ctx, base);
	// 83261AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261AB8 size=64
    let mut pc: u32 = 0x83261AB8;
    'dispatch: loop {
        match pc {
            0x83261AB8 => {
    //   block [0x83261AB8..0x83261AF8)
	// 83261AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261AC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261ACC: 388BF9A0  addi r4, r11, -0x660
	ctx.r[4].s64 = ctx.r[11].s64 + -1632;
	// 83261AD0: 386AB230  addi r3, r10, -0x4dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -19920;
	// 83261AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261AD8: 4AFCB3F9  bl 0x8222ced0
	ctx.lr = 0x83261ADC;
	sub_8222CED0(ctx, base);
	// 83261ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261AE0: 3869CB10  addi r3, r9, -0x34f0
	ctx.r[3].s64 = ctx.r[9].s64 + -13552;
	// 83261AE4: 4BA4843D  bl 0x82ca9f20
	ctx.lr = 0x83261AE8;
	sub_82CA9F20(ctx, base);
	// 83261AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261AF8 size=56
    let mut pc: u32 = 0x83261AF8;
    'dispatch: loop {
        match pc {
            0x83261AF8 => {
    //   block [0x83261AF8..0x83261B30)
	// 83261AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261B0C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83261B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261B14: 4AF92245  bl 0x821f3d58
	ctx.lr = 0x83261B18;
	sub_821F3D58(ctx, base);
	// 83261B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261B1C: 906AB234  stw r3, -0x4dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19916 as u32), ctx.r[3].u32 ) };
	// 83261B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261B30 size=56
    let mut pc: u32 = 0x83261B30;
    'dispatch: loop {
        match pc {
            0x83261B30 => {
    //   block [0x83261B30..0x83261B68)
	// 83261B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261B44: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83261B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261B4C: 4AF9220D  bl 0x821f3d58
	ctx.lr = 0x83261B50;
	sub_821F3D58(ctx, base);
	// 83261B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261B54: 906AB238  stw r3, -0x4dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19912 as u32), ctx.r[3].u32 ) };
	// 83261B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261B68 size=56
    let mut pc: u32 = 0x83261B68;
    'dispatch: loop {
        match pc {
            0x83261B68 => {
    //   block [0x83261B68..0x83261BA0)
	// 83261B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261B7C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83261B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261B84: 4AF921D5  bl 0x821f3d58
	ctx.lr = 0x83261B88;
	sub_821F3D58(ctx, base);
	// 83261B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261B8C: 906AB23C  stw r3, -0x4dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19908 as u32), ctx.r[3].u32 ) };
	// 83261B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261BA0 size=56
    let mut pc: u32 = 0x83261BA0;
    'dispatch: loop {
        match pc {
            0x83261BA0 => {
    //   block [0x83261BA0..0x83261BD8)
	// 83261BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261BB4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83261BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261BBC: 4AF9219D  bl 0x821f3d58
	ctx.lr = 0x83261BC0;
	sub_821F3D58(ctx, base);
	// 83261BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261BC4: 906AB240  stw r3, -0x4dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19904 as u32), ctx.r[3].u32 ) };
	// 83261BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261BD8 size=56
    let mut pc: u32 = 0x83261BD8;
    'dispatch: loop {
        match pc {
            0x83261BD8 => {
    //   block [0x83261BD8..0x83261C10)
	// 83261BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261BEC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83261BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261BF4: 4AF92165  bl 0x821f3d58
	ctx.lr = 0x83261BF8;
	sub_821F3D58(ctx, base);
	// 83261BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261BFC: 906AB244  stw r3, -0x4dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19900 as u32), ctx.r[3].u32 ) };
	// 83261C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261C10 size=56
    let mut pc: u32 = 0x83261C10;
    'dispatch: loop {
        match pc {
            0x83261C10 => {
    //   block [0x83261C10..0x83261C48)
	// 83261C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261C24: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83261C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261C2C: 4AF9212D  bl 0x821f3d58
	ctx.lr = 0x83261C30;
	sub_821F3D58(ctx, base);
	// 83261C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261C34: 906AB248  stw r3, -0x4db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19896 as u32), ctx.r[3].u32 ) };
	// 83261C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261C48 size=56
    let mut pc: u32 = 0x83261C48;
    'dispatch: loop {
        match pc {
            0x83261C48 => {
    //   block [0x83261C48..0x83261C80)
	// 83261C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261C5C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83261C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261C64: 4AF920F5  bl 0x821f3d58
	ctx.lr = 0x83261C68;
	sub_821F3D58(ctx, base);
	// 83261C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261C6C: 906AB24C  stw r3, -0x4db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19892 as u32), ctx.r[3].u32 ) };
	// 83261C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261C80 size=56
    let mut pc: u32 = 0x83261C80;
    'dispatch: loop {
        match pc {
            0x83261C80 => {
    //   block [0x83261C80..0x83261CB8)
	// 83261C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261C94: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83261C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261C9C: 4AF920BD  bl 0x821f3d58
	ctx.lr = 0x83261CA0;
	sub_821F3D58(ctx, base);
	// 83261CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261CA4: 906AB250  stw r3, -0x4db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19888 as u32), ctx.r[3].u32 ) };
	// 83261CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261CB8 size=56
    let mut pc: u32 = 0x83261CB8;
    'dispatch: loop {
        match pc {
            0x83261CB8 => {
    //   block [0x83261CB8..0x83261CF0)
	// 83261CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261CCC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83261CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261CD4: 4AF92085  bl 0x821f3d58
	ctx.lr = 0x83261CD8;
	sub_821F3D58(ctx, base);
	// 83261CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261CDC: 906AB254  stw r3, -0x4dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19884 as u32), ctx.r[3].u32 ) };
	// 83261CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261CF0 size=56
    let mut pc: u32 = 0x83261CF0;
    'dispatch: loop {
        match pc {
            0x83261CF0 => {
    //   block [0x83261CF0..0x83261D28)
	// 83261CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261D04: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83261D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261D0C: 4AF9204D  bl 0x821f3d58
	ctx.lr = 0x83261D10;
	sub_821F3D58(ctx, base);
	// 83261D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261D14: 906AB258  stw r3, -0x4da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19880 as u32), ctx.r[3].u32 ) };
	// 83261D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261D28 size=56
    let mut pc: u32 = 0x83261D28;
    'dispatch: loop {
        match pc {
            0x83261D28 => {
    //   block [0x83261D28..0x83261D60)
	// 83261D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261D3C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83261D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261D44: 4AF92015  bl 0x821f3d58
	ctx.lr = 0x83261D48;
	sub_821F3D58(ctx, base);
	// 83261D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261D4C: 906AB25C  stw r3, -0x4da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19876 as u32), ctx.r[3].u32 ) };
	// 83261D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261D60 size=56
    let mut pc: u32 = 0x83261D60;
    'dispatch: loop {
        match pc {
            0x83261D60 => {
    //   block [0x83261D60..0x83261D98)
	// 83261D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261D74: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83261D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261D7C: 4AF91FDD  bl 0x821f3d58
	ctx.lr = 0x83261D80;
	sub_821F3D58(ctx, base);
	// 83261D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261D84: 906AB260  stw r3, -0x4da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19872 as u32), ctx.r[3].u32 ) };
	// 83261D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261D98 size=56
    let mut pc: u32 = 0x83261D98;
    'dispatch: loop {
        match pc {
            0x83261D98 => {
    //   block [0x83261D98..0x83261DD0)
	// 83261D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261DAC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83261DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261DB4: 4AF91FA5  bl 0x821f3d58
	ctx.lr = 0x83261DB8;
	sub_821F3D58(ctx, base);
	// 83261DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261DBC: 906AB264  stw r3, -0x4d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19868 as u32), ctx.r[3].u32 ) };
	// 83261DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261DD0 size=56
    let mut pc: u32 = 0x83261DD0;
    'dispatch: loop {
        match pc {
            0x83261DD0 => {
    //   block [0x83261DD0..0x83261E08)
	// 83261DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261DE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261DE4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83261DE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261DEC: 4AF91F6D  bl 0x821f3d58
	ctx.lr = 0x83261DF0;
	sub_821F3D58(ctx, base);
	// 83261DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261DF4: 906AB268  stw r3, -0x4d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19864 as u32), ctx.r[3].u32 ) };
	// 83261DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261E08 size=56
    let mut pc: u32 = 0x83261E08;
    'dispatch: loop {
        match pc {
            0x83261E08 => {
    //   block [0x83261E08..0x83261E40)
	// 83261E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261E1C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83261E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261E24: 4AF91F35  bl 0x821f3d58
	ctx.lr = 0x83261E28;
	sub_821F3D58(ctx, base);
	// 83261E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261E2C: 906AB26C  stw r3, -0x4d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19860 as u32), ctx.r[3].u32 ) };
	// 83261E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261E40 size=56
    let mut pc: u32 = 0x83261E40;
    'dispatch: loop {
        match pc {
            0x83261E40 => {
    //   block [0x83261E40..0x83261E78)
	// 83261E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261E54: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83261E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261E5C: 4AF91EFD  bl 0x821f3d58
	ctx.lr = 0x83261E60;
	sub_821F3D58(ctx, base);
	// 83261E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261E64: 906AB270  stw r3, -0x4d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19856 as u32), ctx.r[3].u32 ) };
	// 83261E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261E78 size=56
    let mut pc: u32 = 0x83261E78;
    'dispatch: loop {
        match pc {
            0x83261E78 => {
    //   block [0x83261E78..0x83261EB0)
	// 83261E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261E8C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83261E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261E94: 4AF91EC5  bl 0x821f3d58
	ctx.lr = 0x83261E98;
	sub_821F3D58(ctx, base);
	// 83261E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261E9C: 906AB274  stw r3, -0x4d8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19852 as u32), ctx.r[3].u32 ) };
	// 83261EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261EB0 size=56
    let mut pc: u32 = 0x83261EB0;
    'dispatch: loop {
        match pc {
            0x83261EB0 => {
    //   block [0x83261EB0..0x83261EE8)
	// 83261EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261EC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261EC4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83261EC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261ECC: 4AF91E8D  bl 0x821f3d58
	ctx.lr = 0x83261ED0;
	sub_821F3D58(ctx, base);
	// 83261ED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261ED4: 906AB278  stw r3, -0x4d88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19848 as u32), ctx.r[3].u32 ) };
	// 83261ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261EE8 size=56
    let mut pc: u32 = 0x83261EE8;
    'dispatch: loop {
        match pc {
            0x83261EE8 => {
    //   block [0x83261EE8..0x83261F20)
	// 83261EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261EF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261EF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261EFC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83261F00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261F04: 4AF91E55  bl 0x821f3d58
	ctx.lr = 0x83261F08;
	sub_821F3D58(ctx, base);
	// 83261F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261F0C: 906AB27C  stw r3, -0x4d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19844 as u32), ctx.r[3].u32 ) };
	// 83261F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261F20 size=56
    let mut pc: u32 = 0x83261F20;
    'dispatch: loop {
        match pc {
            0x83261F20 => {
    //   block [0x83261F20..0x83261F58)
	// 83261F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261F2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261F30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261F34: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83261F38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261F3C: 4AF91E1D  bl 0x821f3d58
	ctx.lr = 0x83261F40;
	sub_821F3D58(ctx, base);
	// 83261F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261F44: 906AB280  stw r3, -0x4d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19840 as u32), ctx.r[3].u32 ) };
	// 83261F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261F58 size=56
    let mut pc: u32 = 0x83261F58;
    'dispatch: loop {
        match pc {
            0x83261F58 => {
    //   block [0x83261F58..0x83261F90)
	// 83261F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261F64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261F68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261F6C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83261F70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261F74: 4AF91DE5  bl 0x821f3d58
	ctx.lr = 0x83261F78;
	sub_821F3D58(ctx, base);
	// 83261F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261F7C: 906AB284  stw r3, -0x4d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19836 as u32), ctx.r[3].u32 ) };
	// 83261F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261F90 size=64
    let mut pc: u32 = 0x83261F90;
    'dispatch: loop {
        match pc {
            0x83261F90 => {
    //   block [0x83261F90..0x83261FD0)
	// 83261F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261F9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261FA4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83261FA8: 386AB288  addi r3, r10, -0x4d78
	ctx.r[3].s64 = ctx.r[10].s64 + -19832;
	// 83261FAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261FB0: 4AFCAF21  bl 0x8222ced0
	ctx.lr = 0x83261FB4;
	sub_8222CED0(ctx, base);
	// 83261FB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261FB8: 3869CB20  addi r3, r9, -0x34e0
	ctx.r[3].s64 = ctx.r[9].s64 + -13536;
	// 83261FBC: 4BA47F65  bl 0x82ca9f20
	ctx.lr = 0x83261FC0;
	sub_82CA9F20(ctx, base);
	// 83261FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261FD0 size=64
    let mut pc: u32 = 0x83261FD0;
    'dispatch: loop {
        match pc {
            0x83261FD0 => {
    //   block [0x83261FD0..0x83262010)
	// 83261FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261FDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261FE4: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83261FE8: 386AB28C  addi r3, r10, -0x4d74
	ctx.r[3].s64 = ctx.r[10].s64 + -19828;
	// 83261FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261FF0: 4AFCAEE1  bl 0x8222ced0
	ctx.lr = 0x83261FF4;
	sub_8222CED0(ctx, base);
	// 83261FF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261FF8: 3869CB30  addi r3, r9, -0x34d0
	ctx.r[3].s64 = ctx.r[9].s64 + -13520;
	// 83261FFC: 4BA47F25  bl 0x82ca9f20
	ctx.lr = 0x83262000;
	sub_82CA9F20(ctx, base);
	// 83262000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326200C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262010 size=64
    let mut pc: u32 = 0x83262010;
    'dispatch: loop {
        match pc {
            0x83262010 => {
    //   block [0x83262010..0x83262050)
	// 83262010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326201C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262024: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 83262028: 386AB290  addi r3, r10, -0x4d70
	ctx.r[3].s64 = ctx.r[10].s64 + -19824;
	// 8326202C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262030: 4AFCAEA1  bl 0x8222ced0
	ctx.lr = 0x83262034;
	sub_8222CED0(ctx, base);
	// 83262034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262038: 3869CB40  addi r3, r9, -0x34c0
	ctx.r[3].s64 = ctx.r[9].s64 + -13504;
	// 8326203C: 4BA47EE5  bl 0x82ca9f20
	ctx.lr = 0x83262040;
	sub_82CA9F20(ctx, base);
	// 83262040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326204C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262050 size=64
    let mut pc: u32 = 0x83262050;
    'dispatch: loop {
        match pc {
            0x83262050 => {
    //   block [0x83262050..0x83262090)
	// 83262050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326205C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262064: 388BFB34  addi r4, r11, -0x4cc
	ctx.r[4].s64 = ctx.r[11].s64 + -1228;
	// 83262068: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 8326206C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262070: 4AFCAE61  bl 0x8222ced0
	ctx.lr = 0x83262074;
	sub_8222CED0(ctx, base);
	// 83262074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262078: 3869CB50  addi r3, r9, -0x34b0
	ctx.r[3].s64 = ctx.r[9].s64 + -13488;
	// 8326207C: 4BA47EA5  bl 0x82ca9f20
	ctx.lr = 0x83262080;
	sub_82CA9F20(ctx, base);
	// 83262080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326208C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262090 size=64
    let mut pc: u32 = 0x83262090;
    'dispatch: loop {
        match pc {
            0x83262090 => {
    //   block [0x83262090..0x832620D0)
	// 83262090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326209C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832620A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832620A4: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 832620A8: 386AB298  addi r3, r10, -0x4d68
	ctx.r[3].s64 = ctx.r[10].s64 + -19816;
	// 832620AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832620B0: 4AFCAE21  bl 0x8222ced0
	ctx.lr = 0x832620B4;
	sub_8222CED0(ctx, base);
	// 832620B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832620B8: 3869CB60  addi r3, r9, -0x34a0
	ctx.r[3].s64 = ctx.r[9].s64 + -13472;
	// 832620BC: 4BA47E65  bl 0x82ca9f20
	ctx.lr = 0x832620C0;
	sub_82CA9F20(ctx, base);
	// 832620C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832620C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832620C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832620CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832620D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832620D0 size=64
    let mut pc: u32 = 0x832620D0;
    'dispatch: loop {
        match pc {
            0x832620D0 => {
    //   block [0x832620D0..0x83262110)
	// 832620D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832620D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832620D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832620DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832620E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832620E4: 388BFB64  addi r4, r11, -0x49c
	ctx.r[4].s64 = ctx.r[11].s64 + -1180;
	// 832620E8: 386AB29C  addi r3, r10, -0x4d64
	ctx.r[3].s64 = ctx.r[10].s64 + -19812;
	// 832620EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832620F0: 4AFCADE1  bl 0x8222ced0
	ctx.lr = 0x832620F4;
	sub_8222CED0(ctx, base);
	// 832620F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832620F8: 3869CB70  addi r3, r9, -0x3490
	ctx.r[3].s64 = ctx.r[9].s64 + -13456;
	// 832620FC: 4BA47E25  bl 0x82ca9f20
	ctx.lr = 0x83262100;
	sub_82CA9F20(ctx, base);
	// 83262100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326210C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262110 size=64
    let mut pc: u32 = 0x83262110;
    'dispatch: loop {
        match pc {
            0x83262110 => {
    //   block [0x83262110..0x83262150)
	// 83262110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326211C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262124: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83262128: 386AB2A0  addi r3, r10, -0x4d60
	ctx.r[3].s64 = ctx.r[10].s64 + -19808;
	// 8326212C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262130: 4AFCADA1  bl 0x8222ced0
	ctx.lr = 0x83262134;
	sub_8222CED0(ctx, base);
	// 83262134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262138: 3869CB80  addi r3, r9, -0x3480
	ctx.r[3].s64 = ctx.r[9].s64 + -13440;
	// 8326213C: 4BA47DE5  bl 0x82ca9f20
	ctx.lr = 0x83262140;
	sub_82CA9F20(ctx, base);
	// 83262140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262150 size=64
    let mut pc: u32 = 0x83262150;
    'dispatch: loop {
        match pc {
            0x83262150 => {
    //   block [0x83262150..0x83262190)
	// 83262150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326215C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262164: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83262168: 386AB2A4  addi r3, r10, -0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19804;
	// 8326216C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262170: 4AFCAD61  bl 0x8222ced0
	ctx.lr = 0x83262174;
	sub_8222CED0(ctx, base);
	// 83262174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262178: 3869CB90  addi r3, r9, -0x3470
	ctx.r[3].s64 = ctx.r[9].s64 + -13424;
	// 8326217C: 4BA47DA5  bl 0x82ca9f20
	ctx.lr = 0x83262180;
	sub_82CA9F20(ctx, base);
	// 83262180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326218C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262190 size=64
    let mut pc: u32 = 0x83262190;
    'dispatch: loop {
        match pc {
            0x83262190 => {
    //   block [0x83262190..0x832621D0)
	// 83262190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326219C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832621A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832621A4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832621A8: 386AB2A8  addi r3, r10, -0x4d58
	ctx.r[3].s64 = ctx.r[10].s64 + -19800;
	// 832621AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832621B0: 4AFCAD21  bl 0x8222ced0
	ctx.lr = 0x832621B4;
	sub_8222CED0(ctx, base);
	// 832621B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832621B8: 3869CBA0  addi r3, r9, -0x3460
	ctx.r[3].s64 = ctx.r[9].s64 + -13408;
	// 832621BC: 4BA47D65  bl 0x82ca9f20
	ctx.lr = 0x832621C0;
	sub_82CA9F20(ctx, base);
	// 832621C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832621C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832621C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832621CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


