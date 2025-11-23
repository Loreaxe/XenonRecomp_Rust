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


pub fn sub_8325A7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A7B0 size=64
    let mut pc: u32 = 0x8325A7B0;
    'dispatch: loop {
        match pc {
            0x8325A7B0 => {
    //   block [0x8325A7B0..0x8325A7F0)
	// 8325A7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A7BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A7C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A7C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325A7C8: 386AAA90  addi r3, r10, -0x5570
	ctx.r[3].s64 = ctx.r[10].s64 + -21872;
	// 8325A7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A7D0: 4AFD2701  bl 0x8222ced0
	ctx.lr = 0x8325A7D4;
	sub_8222CED0(ctx, base);
	// 8325A7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A7D8: 3869BE58  addi r3, r9, -0x41a8
	ctx.r[3].s64 = ctx.r[9].s64 + -16808;
	// 8325A7DC: 4BA4F745  bl 0x82ca9f20
	ctx.lr = 0x8325A7E0;
	sub_82CA9F20(ctx, base);
	// 8325A7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A7F0 size=64
    let mut pc: u32 = 0x8325A7F0;
    'dispatch: loop {
        match pc {
            0x8325A7F0 => {
    //   block [0x8325A7F0..0x8325A830)
	// 8325A7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A7FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A804: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325A808: 386AAA94  addi r3, r10, -0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + -21868;
	// 8325A80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A810: 4AFD26C1  bl 0x8222ced0
	ctx.lr = 0x8325A814;
	sub_8222CED0(ctx, base);
	// 8325A814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A818: 3869BE68  addi r3, r9, -0x4198
	ctx.r[3].s64 = ctx.r[9].s64 + -16792;
	// 8325A81C: 4BA4F705  bl 0x82ca9f20
	ctx.lr = 0x8325A820;
	sub_82CA9F20(ctx, base);
	// 8325A820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A830 size=64
    let mut pc: u32 = 0x8325A830;
    'dispatch: loop {
        match pc {
            0x8325A830 => {
    //   block [0x8325A830..0x8325A870)
	// 8325A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A844: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325A848: 386AAA98  addi r3, r10, -0x5568
	ctx.r[3].s64 = ctx.r[10].s64 + -21864;
	// 8325A84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A850: 4AFD2681  bl 0x8222ced0
	ctx.lr = 0x8325A854;
	sub_8222CED0(ctx, base);
	// 8325A854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A858: 3869BE78  addi r3, r9, -0x4188
	ctx.r[3].s64 = ctx.r[9].s64 + -16776;
	// 8325A85C: 4BA4F6C5  bl 0x82ca9f20
	ctx.lr = 0x8325A860;
	sub_82CA9F20(ctx, base);
	// 8325A860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8325A870 size=12
    let mut pc: u32 = 0x8325A870;
    'dispatch: loop {
        match pc {
            0x8325A870 => {
    //   block [0x8325A870..0x8325A87C)
	// 8325A870: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325A874: 386BBE88  addi r3, r11, -0x4178
	ctx.r[3].s64 = ctx.r[11].s64 + -16760;
	// 8325A878: 4BA4F6A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A880 size=64
    let mut pc: u32 = 0x8325A880;
    'dispatch: loop {
        match pc {
            0x8325A880 => {
    //   block [0x8325A880..0x8325A8C0)
	// 8325A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A88C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A894: 388BBA64  addi r4, r11, -0x459c
	ctx.r[4].s64 = ctx.r[11].s64 + -17820;
	// 8325A898: 386AAAB0  addi r3, r10, -0x5550
	ctx.r[3].s64 = ctx.r[10].s64 + -21840;
	// 8325A89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A8A0: 4AFD2631  bl 0x8222ced0
	ctx.lr = 0x8325A8A4;
	sub_8222CED0(ctx, base);
	// 8325A8A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A8A8: 3869BEF0  addi r3, r9, -0x4110
	ctx.r[3].s64 = ctx.r[9].s64 + -16656;
	// 8325A8AC: 4BA4F675  bl 0x82ca9f20
	ctx.lr = 0x8325A8B0;
	sub_82CA9F20(ctx, base);
	// 8325A8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A8C0 size=64
    let mut pc: u32 = 0x8325A8C0;
    'dispatch: loop {
        match pc {
            0x8325A8C0 => {
    //   block [0x8325A8C0..0x8325A900)
	// 8325A8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A8CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8325A8D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A8D4: 388B0E20  addi r4, r11, 0xe20
	ctx.r[4].s64 = ctx.r[11].s64 + 3616;
	// 8325A8D8: 386AAAB4  addi r3, r10, -0x554c
	ctx.r[3].s64 = ctx.r[10].s64 + -21836;
	// 8325A8DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A8E0: 4AFD25F1  bl 0x8222ced0
	ctx.lr = 0x8325A8E4;
	sub_8222CED0(ctx, base);
	// 8325A8E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A8E8: 3869BF00  addi r3, r9, -0x4100
	ctx.r[3].s64 = ctx.r[9].s64 + -16640;
	// 8325A8EC: 4BA4F635  bl 0x82ca9f20
	ctx.lr = 0x8325A8F0;
	sub_82CA9F20(ctx, base);
	// 8325A8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A900 size=64
    let mut pc: u32 = 0x8325A900;
    'dispatch: loop {
        match pc {
            0x8325A900 => {
    //   block [0x8325A900..0x8325A940)
	// 8325A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A90C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A914: 388BBA78  addi r4, r11, -0x4588
	ctx.r[4].s64 = ctx.r[11].s64 + -17800;
	// 8325A918: 386AAAB8  addi r3, r10, -0x5548
	ctx.r[3].s64 = ctx.r[10].s64 + -21832;
	// 8325A91C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A920: 4AFD25B1  bl 0x8222ced0
	ctx.lr = 0x8325A924;
	sub_8222CED0(ctx, base);
	// 8325A924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A928: 3869BF10  addi r3, r9, -0x40f0
	ctx.r[3].s64 = ctx.r[9].s64 + -16624;
	// 8325A92C: 4BA4F5F5  bl 0x82ca9f20
	ctx.lr = 0x8325A930;
	sub_82CA9F20(ctx, base);
	// 8325A930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A940 size=64
    let mut pc: u32 = 0x8325A940;
    'dispatch: loop {
        match pc {
            0x8325A940 => {
    //   block [0x8325A940..0x8325A980)
	// 8325A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A94C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A954: 388BBA9C  addi r4, r11, -0x4564
	ctx.r[4].s64 = ctx.r[11].s64 + -17764;
	// 8325A958: 386AAABC  addi r3, r10, -0x5544
	ctx.r[3].s64 = ctx.r[10].s64 + -21828;
	// 8325A95C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A960: 4AFD2571  bl 0x8222ced0
	ctx.lr = 0x8325A964;
	sub_8222CED0(ctx, base);
	// 8325A964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A968: 3869BF20  addi r3, r9, -0x40e0
	ctx.r[3].s64 = ctx.r[9].s64 + -16608;
	// 8325A96C: 4BA4F5B5  bl 0x82ca9f20
	ctx.lr = 0x8325A970;
	sub_82CA9F20(ctx, base);
	// 8325A970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A980 size=64
    let mut pc: u32 = 0x8325A980;
    'dispatch: loop {
        match pc {
            0x8325A980 => {
    //   block [0x8325A980..0x8325A9C0)
	// 8325A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A98C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A994: 388BBABC  addi r4, r11, -0x4544
	ctx.r[4].s64 = ctx.r[11].s64 + -17732;
	// 8325A998: 386AAAC0  addi r3, r10, -0x5540
	ctx.r[3].s64 = ctx.r[10].s64 + -21824;
	// 8325A99C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A9A0: 4AFD2531  bl 0x8222ced0
	ctx.lr = 0x8325A9A4;
	sub_8222CED0(ctx, base);
	// 8325A9A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A9A8: 3869BF30  addi r3, r9, -0x40d0
	ctx.r[3].s64 = ctx.r[9].s64 + -16592;
	// 8325A9AC: 4BA4F575  bl 0x82ca9f20
	ctx.lr = 0x8325A9B0;
	sub_82CA9F20(ctx, base);
	// 8325A9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A9C0 size=64
    let mut pc: u32 = 0x8325A9C0;
    'dispatch: loop {
        match pc {
            0x8325A9C0 => {
    //   block [0x8325A9C0..0x8325AA00)
	// 8325A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A9CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A9D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A9D4: 388BBAF4  addi r4, r11, -0x450c
	ctx.r[4].s64 = ctx.r[11].s64 + -17676;
	// 8325A9D8: 386AAAC4  addi r3, r10, -0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + -21820;
	// 8325A9DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A9E0: 4AFD24F1  bl 0x8222ced0
	ctx.lr = 0x8325A9E4;
	sub_8222CED0(ctx, base);
	// 8325A9E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A9E8: 3869BF40  addi r3, r9, -0x40c0
	ctx.r[3].s64 = ctx.r[9].s64 + -16576;
	// 8325A9EC: 4BA4F535  bl 0x82ca9f20
	ctx.lr = 0x8325A9F0;
	sub_82CA9F20(ctx, base);
	// 8325A9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA00 size=64
    let mut pc: u32 = 0x8325AA00;
    'dispatch: loop {
        match pc {
            0x8325AA00 => {
    //   block [0x8325AA00..0x8325AA40)
	// 8325AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AA0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AA14: 388BBB18  addi r4, r11, -0x44e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17640;
	// 8325AA18: 386AAAC8  addi r3, r10, -0x5538
	ctx.r[3].s64 = ctx.r[10].s64 + -21816;
	// 8325AA1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AA20: 4AFD24B1  bl 0x8222ced0
	ctx.lr = 0x8325AA24;
	sub_8222CED0(ctx, base);
	// 8325AA24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AA28: 3869BF50  addi r3, r9, -0x40b0
	ctx.r[3].s64 = ctx.r[9].s64 + -16560;
	// 8325AA2C: 4BA4F4F5  bl 0x82ca9f20
	ctx.lr = 0x8325AA30;
	sub_82CA9F20(ctx, base);
	// 8325AA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA40 size=64
    let mut pc: u32 = 0x8325AA40;
    'dispatch: loop {
        match pc {
            0x8325AA40 => {
    //   block [0x8325AA40..0x8325AA80)
	// 8325AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AA4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AA50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AA54: 388BBB48  addi r4, r11, -0x44b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17592;
	// 8325AA58: 386AAACC  addi r3, r10, -0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + -21812;
	// 8325AA5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AA60: 4AFD2471  bl 0x8222ced0
	ctx.lr = 0x8325AA64;
	sub_8222CED0(ctx, base);
	// 8325AA64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AA68: 3869BF60  addi r3, r9, -0x40a0
	ctx.r[3].s64 = ctx.r[9].s64 + -16544;
	// 8325AA6C: 4BA4F4B5  bl 0x82ca9f20
	ctx.lr = 0x8325AA70;
	sub_82CA9F20(ctx, base);
	// 8325AA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA80 size=64
    let mut pc: u32 = 0x8325AA80;
    'dispatch: loop {
        match pc {
            0x8325AA80 => {
    //   block [0x8325AA80..0x8325AAC0)
	// 8325AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AA8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AA94: 388BBB78  addi r4, r11, -0x4488
	ctx.r[4].s64 = ctx.r[11].s64 + -17544;
	// 8325AA98: 386AAAD0  addi r3, r10, -0x5530
	ctx.r[3].s64 = ctx.r[10].s64 + -21808;
	// 8325AA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AAA0: 4AFD2431  bl 0x8222ced0
	ctx.lr = 0x8325AAA4;
	sub_8222CED0(ctx, base);
	// 8325AAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AAA8: 3869BF70  addi r3, r9, -0x4090
	ctx.r[3].s64 = ctx.r[9].s64 + -16528;
	// 8325AAAC: 4BA4F475  bl 0x82ca9f20
	ctx.lr = 0x8325AAB0;
	sub_82CA9F20(ctx, base);
	// 8325AAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AAC0 size=64
    let mut pc: u32 = 0x8325AAC0;
    'dispatch: loop {
        match pc {
            0x8325AAC0 => {
    //   block [0x8325AAC0..0x8325AB00)
	// 8325AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AACC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AAD4: 388BBB9C  addi r4, r11, -0x4464
	ctx.r[4].s64 = ctx.r[11].s64 + -17508;
	// 8325AAD8: 386AAAD4  addi r3, r10, -0x552c
	ctx.r[3].s64 = ctx.r[10].s64 + -21804;
	// 8325AADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AAE0: 4AFD23F1  bl 0x8222ced0
	ctx.lr = 0x8325AAE4;
	sub_8222CED0(ctx, base);
	// 8325AAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AAE8: 3869BF80  addi r3, r9, -0x4080
	ctx.r[3].s64 = ctx.r[9].s64 + -16512;
	// 8325AAEC: 4BA4F435  bl 0x82ca9f20
	ctx.lr = 0x8325AAF0;
	sub_82CA9F20(ctx, base);
	// 8325AAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB00 size=64
    let mut pc: u32 = 0x8325AB00;
    'dispatch: loop {
        match pc {
            0x8325AB00 => {
    //   block [0x8325AB00..0x8325AB40)
	// 8325AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AB0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AB14: 388BBBC8  addi r4, r11, -0x4438
	ctx.r[4].s64 = ctx.r[11].s64 + -17464;
	// 8325AB18: 386AAAD8  addi r3, r10, -0x5528
	ctx.r[3].s64 = ctx.r[10].s64 + -21800;
	// 8325AB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AB20: 4AFD23B1  bl 0x8222ced0
	ctx.lr = 0x8325AB24;
	sub_8222CED0(ctx, base);
	// 8325AB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AB28: 3869BF90  addi r3, r9, -0x4070
	ctx.r[3].s64 = ctx.r[9].s64 + -16496;
	// 8325AB2C: 4BA4F3F5  bl 0x82ca9f20
	ctx.lr = 0x8325AB30;
	sub_82CA9F20(ctx, base);
	// 8325AB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB40 size=64
    let mut pc: u32 = 0x8325AB40;
    'dispatch: loop {
        match pc {
            0x8325AB40 => {
    //   block [0x8325AB40..0x8325AB80)
	// 8325AB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AB4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AB50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AB54: 388BBC08  addi r4, r11, -0x43f8
	ctx.r[4].s64 = ctx.r[11].s64 + -17400;
	// 8325AB58: 386AAADC  addi r3, r10, -0x5524
	ctx.r[3].s64 = ctx.r[10].s64 + -21796;
	// 8325AB5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AB60: 4AFD2371  bl 0x8222ced0
	ctx.lr = 0x8325AB64;
	sub_8222CED0(ctx, base);
	// 8325AB64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AB68: 3869BFA0  addi r3, r9, -0x4060
	ctx.r[3].s64 = ctx.r[9].s64 + -16480;
	// 8325AB6C: 4BA4F3B5  bl 0x82ca9f20
	ctx.lr = 0x8325AB70;
	sub_82CA9F20(ctx, base);
	// 8325AB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB80 size=64
    let mut pc: u32 = 0x8325AB80;
    'dispatch: loop {
        match pc {
            0x8325AB80 => {
    //   block [0x8325AB80..0x8325ABC0)
	// 8325AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AB8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AB90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AB94: 388B2D5C  addi r4, r11, 0x2d5c
	ctx.r[4].s64 = ctx.r[11].s64 + 11612;
	// 8325AB98: 386AAAE0  addi r3, r10, -0x5520
	ctx.r[3].s64 = ctx.r[10].s64 + -21792;
	// 8325AB9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ABA0: 4AFD2331  bl 0x8222ced0
	ctx.lr = 0x8325ABA4;
	sub_8222CED0(ctx, base);
	// 8325ABA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ABA8: 3869BFB0  addi r3, r9, -0x4050
	ctx.r[3].s64 = ctx.r[9].s64 + -16464;
	// 8325ABAC: 4BA4F375  bl 0x82ca9f20
	ctx.lr = 0x8325ABB0;
	sub_82CA9F20(ctx, base);
	// 8325ABB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ABB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ABB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ABBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ABC0 size=64
    let mut pc: u32 = 0x8325ABC0;
    'dispatch: loop {
        match pc {
            0x8325ABC0 => {
    //   block [0x8325ABC0..0x8325AC00)
	// 8325ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ABC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ABCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325ABD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ABD4: 388B2D68  addi r4, r11, 0x2d68
	ctx.r[4].s64 = ctx.r[11].s64 + 11624;
	// 8325ABD8: 386AAAE4  addi r3, r10, -0x551c
	ctx.r[3].s64 = ctx.r[10].s64 + -21788;
	// 8325ABDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ABE0: 4AFD22F1  bl 0x8222ced0
	ctx.lr = 0x8325ABE4;
	sub_8222CED0(ctx, base);
	// 8325ABE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ABE8: 3869BFC0  addi r3, r9, -0x4040
	ctx.r[3].s64 = ctx.r[9].s64 + -16448;
	// 8325ABEC: 4BA4F335  bl 0x82ca9f20
	ctx.lr = 0x8325ABF0;
	sub_82CA9F20(ctx, base);
	// 8325ABF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ABF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ABF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ABFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC00 size=64
    let mut pc: u32 = 0x8325AC00;
    'dispatch: loop {
        match pc {
            0x8325AC00 => {
    //   block [0x8325AC00..0x8325AC40)
	// 8325AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AC0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AC10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AC14: 388B12FC  addi r4, r11, 0x12fc
	ctx.r[4].s64 = ctx.r[11].s64 + 4860;
	// 8325AC18: 386AAAE8  addi r3, r10, -0x5518
	ctx.r[3].s64 = ctx.r[10].s64 + -21784;
	// 8325AC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AC20: 4AFD22B1  bl 0x8222ced0
	ctx.lr = 0x8325AC24;
	sub_8222CED0(ctx, base);
	// 8325AC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AC28: 3869BFD0  addi r3, r9, -0x4030
	ctx.r[3].s64 = ctx.r[9].s64 + -16432;
	// 8325AC2C: 4BA4F2F5  bl 0x82ca9f20
	ctx.lr = 0x8325AC30;
	sub_82CA9F20(ctx, base);
	// 8325AC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC40 size=64
    let mut pc: u32 = 0x8325AC40;
    'dispatch: loop {
        match pc {
            0x8325AC40 => {
    //   block [0x8325AC40..0x8325AC80)
	// 8325AC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AC4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AC50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AC54: 388B12D4  addi r4, r11, 0x12d4
	ctx.r[4].s64 = ctx.r[11].s64 + 4820;
	// 8325AC58: 386AAAEC  addi r3, r10, -0x5514
	ctx.r[3].s64 = ctx.r[10].s64 + -21780;
	// 8325AC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AC60: 4AFD2271  bl 0x8222ced0
	ctx.lr = 0x8325AC64;
	sub_8222CED0(ctx, base);
	// 8325AC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AC68: 3869BFE0  addi r3, r9, -0x4020
	ctx.r[3].s64 = ctx.r[9].s64 + -16416;
	// 8325AC6C: 4BA4F2B5  bl 0x82ca9f20
	ctx.lr = 0x8325AC70;
	sub_82CA9F20(ctx, base);
	// 8325AC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC80 size=64
    let mut pc: u32 = 0x8325AC80;
    'dispatch: loop {
        match pc {
            0x8325AC80 => {
    //   block [0x8325AC80..0x8325ACC0)
	// 8325AC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AC8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AC94: 388B12EC  addi r4, r11, 0x12ec
	ctx.r[4].s64 = ctx.r[11].s64 + 4844;
	// 8325AC98: 386AAAF0  addi r3, r10, -0x5510
	ctx.r[3].s64 = ctx.r[10].s64 + -21776;
	// 8325AC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ACA0: 4AFD2231  bl 0x8222ced0
	ctx.lr = 0x8325ACA4;
	sub_8222CED0(ctx, base);
	// 8325ACA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ACA8: 3869BFF0  addi r3, r9, -0x4010
	ctx.r[3].s64 = ctx.r[9].s64 + -16400;
	// 8325ACAC: 4BA4F275  bl 0x82ca9f20
	ctx.lr = 0x8325ACB0;
	sub_82CA9F20(ctx, base);
	// 8325ACB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ACC0 size=64
    let mut pc: u32 = 0x8325ACC0;
    'dispatch: loop {
        match pc {
            0x8325ACC0 => {
    //   block [0x8325ACC0..0x8325AD00)
	// 8325ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ACC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ACCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325ACD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ACD4: 388BBC44  addi r4, r11, -0x43bc
	ctx.r[4].s64 = ctx.r[11].s64 + -17340;
	// 8325ACD8: 386AAAF4  addi r3, r10, -0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + -21772;
	// 8325ACDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ACE0: 4AFD21F1  bl 0x8222ced0
	ctx.lr = 0x8325ACE4;
	sub_8222CED0(ctx, base);
	// 8325ACE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ACE8: 3869C000  addi r3, r9, -0x4000
	ctx.r[3].s64 = ctx.r[9].s64 + -16384;
	// 8325ACEC: 4BA4F235  bl 0x82ca9f20
	ctx.lr = 0x8325ACF0;
	sub_82CA9F20(ctx, base);
	// 8325ACF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ACF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ACF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ACFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD00 size=64
    let mut pc: u32 = 0x8325AD00;
    'dispatch: loop {
        match pc {
            0x8325AD00 => {
    //   block [0x8325AD00..0x8325AD40)
	// 8325AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AD0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AD10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AD14: 388BBC50  addi r4, r11, -0x43b0
	ctx.r[4].s64 = ctx.r[11].s64 + -17328;
	// 8325AD18: 386AAAF8  addi r3, r10, -0x5508
	ctx.r[3].s64 = ctx.r[10].s64 + -21768;
	// 8325AD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AD20: 4AFD21B1  bl 0x8222ced0
	ctx.lr = 0x8325AD24;
	sub_8222CED0(ctx, base);
	// 8325AD24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AD28: 3869C010  addi r3, r9, -0x3ff0
	ctx.r[3].s64 = ctx.r[9].s64 + -16368;
	// 8325AD2C: 4BA4F1F5  bl 0x82ca9f20
	ctx.lr = 0x8325AD30;
	sub_82CA9F20(ctx, base);
	// 8325AD30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD40 size=64
    let mut pc: u32 = 0x8325AD40;
    'dispatch: loop {
        match pc {
            0x8325AD40 => {
    //   block [0x8325AD40..0x8325AD80)
	// 8325AD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AD4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AD50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AD54: 388BBC5C  addi r4, r11, -0x43a4
	ctx.r[4].s64 = ctx.r[11].s64 + -17316;
	// 8325AD58: 386AAAFC  addi r3, r10, -0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + -21764;
	// 8325AD5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AD60: 4AFD2171  bl 0x8222ced0
	ctx.lr = 0x8325AD64;
	sub_8222CED0(ctx, base);
	// 8325AD64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AD68: 3869C020  addi r3, r9, -0x3fe0
	ctx.r[3].s64 = ctx.r[9].s64 + -16352;
	// 8325AD6C: 4BA4F1B5  bl 0x82ca9f20
	ctx.lr = 0x8325AD70;
	sub_82CA9F20(ctx, base);
	// 8325AD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD80 size=64
    let mut pc: u32 = 0x8325AD80;
    'dispatch: loop {
        match pc {
            0x8325AD80 => {
    //   block [0x8325AD80..0x8325ADC0)
	// 8325AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AD8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AD94: 388BBC68  addi r4, r11, -0x4398
	ctx.r[4].s64 = ctx.r[11].s64 + -17304;
	// 8325AD98: 386AAB00  addi r3, r10, -0x5500
	ctx.r[3].s64 = ctx.r[10].s64 + -21760;
	// 8325AD9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ADA0: 4AFD2131  bl 0x8222ced0
	ctx.lr = 0x8325ADA4;
	sub_8222CED0(ctx, base);
	// 8325ADA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ADA8: 3869C030  addi r3, r9, -0x3fd0
	ctx.r[3].s64 = ctx.r[9].s64 + -16336;
	// 8325ADAC: 4BA4F175  bl 0x82ca9f20
	ctx.lr = 0x8325ADB0;
	sub_82CA9F20(ctx, base);
	// 8325ADB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ADB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ADB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ADBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ADC0 size=64
    let mut pc: u32 = 0x8325ADC0;
    'dispatch: loop {
        match pc {
            0x8325ADC0 => {
    //   block [0x8325ADC0..0x8325AE00)
	// 8325ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ADC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ADCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325ADD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ADD4: 388BBC74  addi r4, r11, -0x438c
	ctx.r[4].s64 = ctx.r[11].s64 + -17292;
	// 8325ADD8: 386AAB04  addi r3, r10, -0x54fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21756;
	// 8325ADDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ADE0: 4AFD20F1  bl 0x8222ced0
	ctx.lr = 0x8325ADE4;
	sub_8222CED0(ctx, base);
	// 8325ADE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ADE8: 3869C040  addi r3, r9, -0x3fc0
	ctx.r[3].s64 = ctx.r[9].s64 + -16320;
	// 8325ADEC: 4BA4F135  bl 0x82ca9f20
	ctx.lr = 0x8325ADF0;
	sub_82CA9F20(ctx, base);
	// 8325ADF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ADF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ADF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ADFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AE00 size=64
    let mut pc: u32 = 0x8325AE00;
    'dispatch: loop {
        match pc {
            0x8325AE00 => {
    //   block [0x8325AE00..0x8325AE40)
	// 8325AE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AE0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AE14: 388BBC8C  addi r4, r11, -0x4374
	ctx.r[4].s64 = ctx.r[11].s64 + -17268;
	// 8325AE18: 386AAB08  addi r3, r10, -0x54f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21752;
	// 8325AE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AE20: 4AFD20B1  bl 0x8222ced0
	ctx.lr = 0x8325AE24;
	sub_8222CED0(ctx, base);
	// 8325AE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AE28: 3869C050  addi r3, r9, -0x3fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -16304;
	// 8325AE2C: 4BA4F0F5  bl 0x82ca9f20
	ctx.lr = 0x8325AE30;
	sub_82CA9F20(ctx, base);
	// 8325AE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AE40 size=108
    let mut pc: u32 = 0x8325AE40;
    'dispatch: loop {
        match pc {
            0x8325AE40 => {
    //   block [0x8325AE40..0x8325AEAC)
	// 8325AE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AE48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325AE4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AE50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AE54: 3BEBAB0C  addi r31, r11, -0x54f4
	ctx.r[31].s64 = ctx.r[11].s64 + -21748;
	// 8325AE58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325AE5C: 4B0C7AA5  bl 0x82322900
	ctx.lr = 0x8325AE60;
	sub_82322900(ctx, base);
	// 8325AE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8325AE64: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325AE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325AE6C: 99230015  stb r9, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[9].u8 ) };
	// 8325AE70: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8325AE74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE78: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325AE7C: 3868C060  addi r3, r8, -0x3fa0
	ctx.r[3].s64 = ctx.r[8].s64 + -16288;
	// 8325AE80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE84: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325AE88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE8C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325AE90: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325AE94: 4BA4F08D  bl 0x82ca9f20
	ctx.lr = 0x8325AE98;
	sub_82CA9F20(ctx, base);
	// 8325AE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AEA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325AEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AEB0 size=64
    let mut pc: u32 = 0x8325AEB0;
    'dispatch: loop {
        match pc {
            0x8325AEB0 => {
    //   block [0x8325AEB0..0x8325AEF0)
	// 8325AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AEBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325AEC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AEC4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325AEC8: 386AAB18  addi r3, r10, -0x54e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21736;
	// 8325AECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AED0: 4AFD2001  bl 0x8222ced0
	ctx.lr = 0x8325AED4;
	sub_8222CED0(ctx, base);
	// 8325AED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AED8: 3869C070  addi r3, r9, -0x3f90
	ctx.r[3].s64 = ctx.r[9].s64 + -16272;
	// 8325AEDC: 4BA4F045  bl 0x82ca9f20
	ctx.lr = 0x8325AEE0;
	sub_82CA9F20(ctx, base);
	// 8325AEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AEF0 size=64
    let mut pc: u32 = 0x8325AEF0;
    'dispatch: loop {
        match pc {
            0x8325AEF0 => {
    //   block [0x8325AEF0..0x8325AF30)
	// 8325AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AEF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AEFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325AF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AF04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325AF08: 386AAB1C  addi r3, r10, -0x54e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21732;
	// 8325AF0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AF10: 4AFD1FC1  bl 0x8222ced0
	ctx.lr = 0x8325AF14;
	sub_8222CED0(ctx, base);
	// 8325AF14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AF18: 3869C080  addi r3, r9, -0x3f80
	ctx.r[3].s64 = ctx.r[9].s64 + -16256;
	// 8325AF1C: 4BA4F005  bl 0x82ca9f20
	ctx.lr = 0x8325AF20;
	sub_82CA9F20(ctx, base);
	// 8325AF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AF30 size=64
    let mut pc: u32 = 0x8325AF30;
    'dispatch: loop {
        match pc {
            0x8325AF30 => {
    //   block [0x8325AF30..0x8325AF70)
	// 8325AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AF3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325AF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AF44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325AF48: 386AAB20  addi r3, r10, -0x54e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21728;
	// 8325AF4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AF50: 4AFD1F81  bl 0x8222ced0
	ctx.lr = 0x8325AF54;
	sub_8222CED0(ctx, base);
	// 8325AF54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AF58: 3869C090  addi r3, r9, -0x3f70
	ctx.r[3].s64 = ctx.r[9].s64 + -16240;
	// 8325AF5C: 4BA4EFC5  bl 0x82ca9f20
	ctx.lr = 0x8325AF60;
	sub_82CA9F20(ctx, base);
	// 8325AF60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AF70 size=52
    let mut pc: u32 = 0x8325AF70;
    'dispatch: loop {
        match pc {
            0x8325AF70 => {
    //   block [0x8325AF70..0x8325AFA4)
	// 8325AF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AF7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AF80: 386BAB24  addi r3, r11, -0x54dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21724;
	// 8325AF84: 4B2252B5  bl 0x82480238
	ctx.lr = 0x8325AF88;
	sub_82480238(ctx, base);
	// 8325AF88: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325AF8C: 386AC0A0  addi r3, r10, -0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + -16224;
	// 8325AF90: 4BA4EF91  bl 0x82ca9f20
	ctx.lr = 0x8325AF94;
	sub_82CA9F20(ctx, base);
	// 8325AF94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AFA8 size=236
    let mut pc: u32 = 0x8325AFA8;
    'dispatch: loop {
        match pc {
            0x8325AFA8 => {
    //   block [0x8325AFA8..0x8325B094)
	// 8325AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AFB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325AFB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AFB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AFBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325AFC0: 3BEBAB30  addi r31, r11, -0x54d0
	ctx.r[31].s64 = ctx.r[11].s64 + -21712;
	// 8325AFC4: 388AC798  addi r4, r10, -0x3868
	ctx.r[4].s64 = ctx.r[10].s64 + -14440;
	// 8325AFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325AFCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFD0: 4AFD1F01  bl 0x8222ced0
	ctx.lr = 0x8325AFD4;
	sub_8222CED0(ctx, base);
	// 8325AFD4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325AFD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFDC: 3889C77C  addi r4, r9, -0x3884
	ctx.r[4].s64 = ctx.r[9].s64 + -14468;
	// 8325AFE0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8325AFE4: 4AFD1EED  bl 0x8222ced0
	ctx.lr = 0x8325AFE8;
	sub_8222CED0(ctx, base);
	// 8325AFE8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325AFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFF0: 3888C760  addi r4, r8, -0x38a0
	ctx.r[4].s64 = ctx.r[8].s64 + -14496;
	// 8325AFF4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325AFF8: 4AFD1ED9  bl 0x8222ced0
	ctx.lr = 0x8325AFFC;
	sub_8222CED0(ctx, base);
	// 8325AFFC: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325B000: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B004: 3887C744  addi r4, r7, -0x38bc
	ctx.r[4].s64 = ctx.r[7].s64 + -14524;
	// 8325B008: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8325B00C: 4AFD1EC5  bl 0x8222ced0
	ctx.lr = 0x8325B010;
	sub_8222CED0(ctx, base);
	// 8325B010: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325B014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B018: 3886C728  addi r4, r6, -0x38d8
	ctx.r[4].s64 = ctx.r[6].s64 + -14552;
	// 8325B01C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8325B020: 4AFD1EB1  bl 0x8222ced0
	ctx.lr = 0x8325B024;
	sub_8222CED0(ctx, base);
	// 8325B024: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325B028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B02C: 3884C70C  addi r4, r4, -0x38f4
	ctx.r[4].s64 = ctx.r[4].s64 + -14580;
	// 8325B030: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8325B034: 4AFD1E9D  bl 0x8222ced0
	ctx.lr = 0x8325B038;
	sub_8222CED0(ctx, base);
	// 8325B038: 3C60820D  lis r3, -0x7df3
	ctx.r[3].s64 = -2113077248;
	// 8325B03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B040: 3883C6F0  addi r4, r3, -0x3910
	ctx.r[4].s64 = ctx.r[3].s64 + -14608;
	// 8325B044: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325B048: 4AFD1E89  bl 0x8222ced0
	ctx.lr = 0x8325B04C;
	sub_8222CED0(ctx, base);
	// 8325B04C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B054: 388BC6D4  addi r4, r11, -0x392c
	ctx.r[4].s64 = ctx.r[11].s64 + -14636;
	// 8325B058: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8325B05C: 4AFD1E75  bl 0x8222ced0
	ctx.lr = 0x8325B060;
	sub_8222CED0(ctx, base);
	// 8325B060: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325B064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B068: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 8325B06C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8325B070: 4AFD1E61  bl 0x8222ced0
	ctx.lr = 0x8325B074;
	sub_8222CED0(ctx, base);
	// 8325B074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B078: 3869C0B0  addi r3, r9, -0x3f50
	ctx.r[3].s64 = ctx.r[9].s64 + -16208;
	// 8325B07C: 4BA4EEA5  bl 0x82ca9f20
	ctx.lr = 0x8325B080;
	sub_82CA9F20(ctx, base);
	// 8325B080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B08C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325B090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B098 size=64
    let mut pc: u32 = 0x8325B098;
    'dispatch: loop {
        match pc {
            0x8325B098 => {
    //   block [0x8325B098..0x8325B0D8)
	// 8325B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B0A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B0AC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325B0B0: 386AAB54  addi r3, r10, -0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21676;
	// 8325B0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B0B8: 4AFD1E19  bl 0x8222ced0
	ctx.lr = 0x8325B0BC;
	sub_8222CED0(ctx, base);
	// 8325B0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B0C0: 3869C138  addi r3, r9, -0x3ec8
	ctx.r[3].s64 = ctx.r[9].s64 + -16072;
	// 8325B0C4: 4BA4EE5D  bl 0x82ca9f20
	ctx.lr = 0x8325B0C8;
	sub_82CA9F20(ctx, base);
	// 8325B0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B0D8 size=64
    let mut pc: u32 = 0x8325B0D8;
    'dispatch: loop {
        match pc {
            0x8325B0D8 => {
    //   block [0x8325B0D8..0x8325B118)
	// 8325B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B0E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B0E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B0EC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325B0F0: 386AAB58  addi r3, r10, -0x54a8
	ctx.r[3].s64 = ctx.r[10].s64 + -21672;
	// 8325B0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B0F8: 4AFD1DD9  bl 0x8222ced0
	ctx.lr = 0x8325B0FC;
	sub_8222CED0(ctx, base);
	// 8325B0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B100: 3869C148  addi r3, r9, -0x3eb8
	ctx.r[3].s64 = ctx.r[9].s64 + -16056;
	// 8325B104: 4BA4EE1D  bl 0x82ca9f20
	ctx.lr = 0x8325B108;
	sub_82CA9F20(ctx, base);
	// 8325B108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B118 size=64
    let mut pc: u32 = 0x8325B118;
    'dispatch: loop {
        match pc {
            0x8325B118 => {
    //   block [0x8325B118..0x8325B158)
	// 8325B118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B12C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325B130: 386AAB5C  addi r3, r10, -0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21668;
	// 8325B134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B138: 4AFD1D99  bl 0x8222ced0
	ctx.lr = 0x8325B13C;
	sub_8222CED0(ctx, base);
	// 8325B13C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B140: 3869C158  addi r3, r9, -0x3ea8
	ctx.r[3].s64 = ctx.r[9].s64 + -16040;
	// 8325B144: 4BA4EDDD  bl 0x82ca9f20
	ctx.lr = 0x8325B148;
	sub_82CA9F20(ctx, base);
	// 8325B148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B158 size=56
    let mut pc: u32 = 0x8325B158;
    'dispatch: loop {
        match pc {
            0x8325B158 => {
    //   block [0x8325B158..0x8325B190)
	// 8325B158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B16C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325B170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B174: 4AF98BE5  bl 0x821f3d58
	ctx.lr = 0x8325B178;
	sub_821F3D58(ctx, base);
	// 8325B178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B17C: 906AAB60  stw r3, -0x54a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21664 as u32), ctx.r[3].u32 ) };
	// 8325B180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B190 size=56
    let mut pc: u32 = 0x8325B190;
    'dispatch: loop {
        match pc {
            0x8325B190 => {
    //   block [0x8325B190..0x8325B1C8)
	// 8325B190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B1A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325B1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B1AC: 4AF98BAD  bl 0x821f3d58
	ctx.lr = 0x8325B1B0;
	sub_821F3D58(ctx, base);
	// 8325B1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B1B4: 906AAB64  stw r3, -0x549c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21660 as u32), ctx.r[3].u32 ) };
	// 8325B1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B1C8 size=56
    let mut pc: u32 = 0x8325B1C8;
    'dispatch: loop {
        match pc {
            0x8325B1C8 => {
    //   block [0x8325B1C8..0x8325B200)
	// 8325B1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B1DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325B1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B1E4: 4AF98B75  bl 0x821f3d58
	ctx.lr = 0x8325B1E8;
	sub_821F3D58(ctx, base);
	// 8325B1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B1EC: 906AAB68  stw r3, -0x5498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21656 as u32), ctx.r[3].u32 ) };
	// 8325B1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B200 size=56
    let mut pc: u32 = 0x8325B200;
    'dispatch: loop {
        match pc {
            0x8325B200 => {
    //   block [0x8325B200..0x8325B238)
	// 8325B200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B214: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325B218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B21C: 4AF98B3D  bl 0x821f3d58
	ctx.lr = 0x8325B220;
	sub_821F3D58(ctx, base);
	// 8325B220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B224: 906AAB6C  stw r3, -0x5494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21652 as u32), ctx.r[3].u32 ) };
	// 8325B228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B238 size=56
    let mut pc: u32 = 0x8325B238;
    'dispatch: loop {
        match pc {
            0x8325B238 => {
    //   block [0x8325B238..0x8325B270)
	// 8325B238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B24C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325B250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B254: 4AF98B05  bl 0x821f3d58
	ctx.lr = 0x8325B258;
	sub_821F3D58(ctx, base);
	// 8325B258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B25C: 906AAB70  stw r3, -0x5490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21648 as u32), ctx.r[3].u32 ) };
	// 8325B260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B270 size=56
    let mut pc: u32 = 0x8325B270;
    'dispatch: loop {
        match pc {
            0x8325B270 => {
    //   block [0x8325B270..0x8325B2A8)
	// 8325B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B284: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325B288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B28C: 4AF98ACD  bl 0x821f3d58
	ctx.lr = 0x8325B290;
	sub_821F3D58(ctx, base);
	// 8325B290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B294: 906AAB74  stw r3, -0x548c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21644 as u32), ctx.r[3].u32 ) };
	// 8325B298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B2A8 size=56
    let mut pc: u32 = 0x8325B2A8;
    'dispatch: loop {
        match pc {
            0x8325B2A8 => {
    //   block [0x8325B2A8..0x8325B2E0)
	// 8325B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B2BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325B2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B2C4: 4AF98A95  bl 0x821f3d58
	ctx.lr = 0x8325B2C8;
	sub_821F3D58(ctx, base);
	// 8325B2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B2CC: 906AAB78  stw r3, -0x5488(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21640 as u32), ctx.r[3].u32 ) };
	// 8325B2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B2E0 size=56
    let mut pc: u32 = 0x8325B2E0;
    'dispatch: loop {
        match pc {
            0x8325B2E0 => {
    //   block [0x8325B2E0..0x8325B318)
	// 8325B2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B2F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325B2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B2FC: 4AF98A5D  bl 0x821f3d58
	ctx.lr = 0x8325B300;
	sub_821F3D58(ctx, base);
	// 8325B300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B304: 906AAB7C  stw r3, -0x5484(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21636 as u32), ctx.r[3].u32 ) };
	// 8325B308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B318 size=56
    let mut pc: u32 = 0x8325B318;
    'dispatch: loop {
        match pc {
            0x8325B318 => {
    //   block [0x8325B318..0x8325B350)
	// 8325B318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B32C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325B330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B334: 4AF98A25  bl 0x821f3d58
	ctx.lr = 0x8325B338;
	sub_821F3D58(ctx, base);
	// 8325B338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B33C: 906AAB80  stw r3, -0x5480(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21632 as u32), ctx.r[3].u32 ) };
	// 8325B340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B350 size=56
    let mut pc: u32 = 0x8325B350;
    'dispatch: loop {
        match pc {
            0x8325B350 => {
    //   block [0x8325B350..0x8325B388)
	// 8325B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B364: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325B368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B36C: 4AF989ED  bl 0x821f3d58
	ctx.lr = 0x8325B370;
	sub_821F3D58(ctx, base);
	// 8325B370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B374: 906AAB84  stw r3, -0x547c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21628 as u32), ctx.r[3].u32 ) };
	// 8325B378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B388 size=56
    let mut pc: u32 = 0x8325B388;
    'dispatch: loop {
        match pc {
            0x8325B388 => {
    //   block [0x8325B388..0x8325B3C0)
	// 8325B388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B39C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325B3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B3A4: 4AF989B5  bl 0x821f3d58
	ctx.lr = 0x8325B3A8;
	sub_821F3D58(ctx, base);
	// 8325B3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B3AC: 906AAB88  stw r3, -0x5478(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21624 as u32), ctx.r[3].u32 ) };
	// 8325B3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B3C0 size=56
    let mut pc: u32 = 0x8325B3C0;
    'dispatch: loop {
        match pc {
            0x8325B3C0 => {
    //   block [0x8325B3C0..0x8325B3F8)
	// 8325B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B3D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325B3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B3DC: 4AF9897D  bl 0x821f3d58
	ctx.lr = 0x8325B3E0;
	sub_821F3D58(ctx, base);
	// 8325B3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B3E4: 906AAB8C  stw r3, -0x5474(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21620 as u32), ctx.r[3].u32 ) };
	// 8325B3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B3F8 size=56
    let mut pc: u32 = 0x8325B3F8;
    'dispatch: loop {
        match pc {
            0x8325B3F8 => {
    //   block [0x8325B3F8..0x8325B430)
	// 8325B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B40C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325B410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B414: 4AF98945  bl 0x821f3d58
	ctx.lr = 0x8325B418;
	sub_821F3D58(ctx, base);
	// 8325B418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B41C: 906AAB90  stw r3, -0x5470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21616 as u32), ctx.r[3].u32 ) };
	// 8325B420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B430 size=56
    let mut pc: u32 = 0x8325B430;
    'dispatch: loop {
        match pc {
            0x8325B430 => {
    //   block [0x8325B430..0x8325B468)
	// 8325B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B444: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325B448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B44C: 4AF9890D  bl 0x821f3d58
	ctx.lr = 0x8325B450;
	sub_821F3D58(ctx, base);
	// 8325B450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B454: 906AAB94  stw r3, -0x546c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21612 as u32), ctx.r[3].u32 ) };
	// 8325B458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B468 size=56
    let mut pc: u32 = 0x8325B468;
    'dispatch: loop {
        match pc {
            0x8325B468 => {
    //   block [0x8325B468..0x8325B4A0)
	// 8325B468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B47C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325B480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B484: 4AF988D5  bl 0x821f3d58
	ctx.lr = 0x8325B488;
	sub_821F3D58(ctx, base);
	// 8325B488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B48C: 906AAB98  stw r3, -0x5468(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21608 as u32), ctx.r[3].u32 ) };
	// 8325B490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B4A0 size=56
    let mut pc: u32 = 0x8325B4A0;
    'dispatch: loop {
        match pc {
            0x8325B4A0 => {
    //   block [0x8325B4A0..0x8325B4D8)
	// 8325B4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B4B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325B4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B4BC: 4AF9889D  bl 0x821f3d58
	ctx.lr = 0x8325B4C0;
	sub_821F3D58(ctx, base);
	// 8325B4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B4C4: 906AAB9C  stw r3, -0x5464(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21604 as u32), ctx.r[3].u32 ) };
	// 8325B4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B4D8 size=56
    let mut pc: u32 = 0x8325B4D8;
    'dispatch: loop {
        match pc {
            0x8325B4D8 => {
    //   block [0x8325B4D8..0x8325B510)
	// 8325B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B4EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325B4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B4F4: 4AF98865  bl 0x821f3d58
	ctx.lr = 0x8325B4F8;
	sub_821F3D58(ctx, base);
	// 8325B4F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B4FC: 906AABA0  stw r3, -0x5460(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21600 as u32), ctx.r[3].u32 ) };
	// 8325B500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B510 size=56
    let mut pc: u32 = 0x8325B510;
    'dispatch: loop {
        match pc {
            0x8325B510 => {
    //   block [0x8325B510..0x8325B548)
	// 8325B510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B524: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325B528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B52C: 4AF9882D  bl 0x821f3d58
	ctx.lr = 0x8325B530;
	sub_821F3D58(ctx, base);
	// 8325B530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B534: 906AABA4  stw r3, -0x545c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21596 as u32), ctx.r[3].u32 ) };
	// 8325B538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B548 size=56
    let mut pc: u32 = 0x8325B548;
    'dispatch: loop {
        match pc {
            0x8325B548 => {
    //   block [0x8325B548..0x8325B580)
	// 8325B548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B55C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325B560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B564: 4AF987F5  bl 0x821f3d58
	ctx.lr = 0x8325B568;
	sub_821F3D58(ctx, base);
	// 8325B568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B56C: 906AABA8  stw r3, -0x5458(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21592 as u32), ctx.r[3].u32 ) };
	// 8325B570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B580 size=56
    let mut pc: u32 = 0x8325B580;
    'dispatch: loop {
        match pc {
            0x8325B580 => {
    //   block [0x8325B580..0x8325B5B8)
	// 8325B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B58C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B594: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325B598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B59C: 4AF987BD  bl 0x821f3d58
	ctx.lr = 0x8325B5A0;
	sub_821F3D58(ctx, base);
	// 8325B5A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B5A4: 906AABAC  stw r3, -0x5454(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21588 as u32), ctx.r[3].u32 ) };
	// 8325B5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B5B8 size=56
    let mut pc: u32 = 0x8325B5B8;
    'dispatch: loop {
        match pc {
            0x8325B5B8 => {
    //   block [0x8325B5B8..0x8325B5F0)
	// 8325B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B5C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B5C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B5CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325B5D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B5D4: 4AF98785  bl 0x821f3d58
	ctx.lr = 0x8325B5D8;
	sub_821F3D58(ctx, base);
	// 8325B5D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B5DC: 906AABB0  stw r3, -0x5450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21584 as u32), ctx.r[3].u32 ) };
	// 8325B5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B5F0 size=64
    let mut pc: u32 = 0x8325B5F0;
    'dispatch: loop {
        match pc {
            0x8325B5F0 => {
    //   block [0x8325B5F0..0x8325B630)
	// 8325B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B5FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B604: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325B608: 386AABB4  addi r3, r10, -0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + -21580;
	// 8325B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B610: 4AFD18C1  bl 0x8222ced0
	ctx.lr = 0x8325B614;
	sub_8222CED0(ctx, base);
	// 8325B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B618: 3869C168  addi r3, r9, -0x3e98
	ctx.r[3].s64 = ctx.r[9].s64 + -16024;
	// 8325B61C: 4BA4E905  bl 0x82ca9f20
	ctx.lr = 0x8325B620;
	sub_82CA9F20(ctx, base);
	// 8325B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B630 size=64
    let mut pc: u32 = 0x8325B630;
    'dispatch: loop {
        match pc {
            0x8325B630 => {
    //   block [0x8325B630..0x8325B670)
	// 8325B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B63C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B644: 388BCE84  addi r4, r11, -0x317c
	ctx.r[4].s64 = ctx.r[11].s64 + -12668;
	// 8325B648: 386AABB8  addi r3, r10, -0x5448
	ctx.r[3].s64 = ctx.r[10].s64 + -21576;
	// 8325B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B650: 4AFD1881  bl 0x8222ced0
	ctx.lr = 0x8325B654;
	sub_8222CED0(ctx, base);
	// 8325B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B658: 3869C178  addi r3, r9, -0x3e88
	ctx.r[3].s64 = ctx.r[9].s64 + -16008;
	// 8325B65C: 4BA4E8C5  bl 0x82ca9f20
	ctx.lr = 0x8325B660;
	sub_82CA9F20(ctx, base);
	// 8325B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B670 size=64
    let mut pc: u32 = 0x8325B670;
    'dispatch: loop {
        match pc {
            0x8325B670 => {
    //   block [0x8325B670..0x8325B6B0)
	// 8325B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B684: 388B4A34  addi r4, r11, 0x4a34
	ctx.r[4].s64 = ctx.r[11].s64 + 18996;
	// 8325B688: 386AABBC  addi r3, r10, -0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + -21572;
	// 8325B68C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B690: 4AFD1841  bl 0x8222ced0
	ctx.lr = 0x8325B694;
	sub_8222CED0(ctx, base);
	// 8325B694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B698: 3869C188  addi r3, r9, -0x3e78
	ctx.r[3].s64 = ctx.r[9].s64 + -15992;
	// 8325B69C: 4BA4E885  bl 0x82ca9f20
	ctx.lr = 0x8325B6A0;
	sub_82CA9F20(ctx, base);
	// 8325B6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B6B0 size=64
    let mut pc: u32 = 0x8325B6B0;
    'dispatch: loop {
        match pc {
            0x8325B6B0 => {
    //   block [0x8325B6B0..0x8325B6F0)
	// 8325B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B6BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B6C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B6C4: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325B6C8: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 8325B6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B6D0: 4AFD1801  bl 0x8222ced0
	ctx.lr = 0x8325B6D4;
	sub_8222CED0(ctx, base);
	// 8325B6D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B6D8: 3869C198  addi r3, r9, -0x3e68
	ctx.r[3].s64 = ctx.r[9].s64 + -15976;
	// 8325B6DC: 4BA4E845  bl 0x82ca9f20
	ctx.lr = 0x8325B6E0;
	sub_82CA9F20(ctx, base);
	// 8325B6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B6F0 size=64
    let mut pc: u32 = 0x8325B6F0;
    'dispatch: loop {
        match pc {
            0x8325B6F0 => {
    //   block [0x8325B6F0..0x8325B730)
	// 8325B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B6FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B704: 388BCEC4  addi r4, r11, -0x313c
	ctx.r[4].s64 = ctx.r[11].s64 + -12604;
	// 8325B708: 386AABC4  addi r3, r10, -0x543c
	ctx.r[3].s64 = ctx.r[10].s64 + -21564;
	// 8325B70C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B710: 4AFD17C1  bl 0x8222ced0
	ctx.lr = 0x8325B714;
	sub_8222CED0(ctx, base);
	// 8325B714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B718: 3869C1A8  addi r3, r9, -0x3e58
	ctx.r[3].s64 = ctx.r[9].s64 + -15960;
	// 8325B71C: 4BA4E805  bl 0x82ca9f20
	ctx.lr = 0x8325B720;
	sub_82CA9F20(ctx, base);
	// 8325B720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B730 size=64
    let mut pc: u32 = 0x8325B730;
    'dispatch: loop {
        match pc {
            0x8325B730 => {
    //   block [0x8325B730..0x8325B770)
	// 8325B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B73C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B744: 388BCED8  addi r4, r11, -0x3128
	ctx.r[4].s64 = ctx.r[11].s64 + -12584;
	// 8325B748: 386AABC8  addi r3, r10, -0x5438
	ctx.r[3].s64 = ctx.r[10].s64 + -21560;
	// 8325B74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B750: 4AFD1781  bl 0x8222ced0
	ctx.lr = 0x8325B754;
	sub_8222CED0(ctx, base);
	// 8325B754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B758: 3869C1B8  addi r3, r9, -0x3e48
	ctx.r[3].s64 = ctx.r[9].s64 + -15944;
	// 8325B75C: 4BA4E7C5  bl 0x82ca9f20
	ctx.lr = 0x8325B760;
	sub_82CA9F20(ctx, base);
	// 8325B760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B770 size=64
    let mut pc: u32 = 0x8325B770;
    'dispatch: loop {
        match pc {
            0x8325B770 => {
    //   block [0x8325B770..0x8325B7B0)
	// 8325B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B77C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B784: 388BCEF8  addi r4, r11, -0x3108
	ctx.r[4].s64 = ctx.r[11].s64 + -12552;
	// 8325B788: 386AABCC  addi r3, r10, -0x5434
	ctx.r[3].s64 = ctx.r[10].s64 + -21556;
	// 8325B78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B790: 4AFD1741  bl 0x8222ced0
	ctx.lr = 0x8325B794;
	sub_8222CED0(ctx, base);
	// 8325B794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B798: 3869C1C8  addi r3, r9, -0x3e38
	ctx.r[3].s64 = ctx.r[9].s64 + -15928;
	// 8325B79C: 4BA4E785  bl 0x82ca9f20
	ctx.lr = 0x8325B7A0;
	sub_82CA9F20(ctx, base);
	// 8325B7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B7B0 size=64
    let mut pc: u32 = 0x8325B7B0;
    'dispatch: loop {
        match pc {
            0x8325B7B0 => {
    //   block [0x8325B7B0..0x8325B7F0)
	// 8325B7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B7BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B7C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B7C4: 388BCF1C  addi r4, r11, -0x30e4
	ctx.r[4].s64 = ctx.r[11].s64 + -12516;
	// 8325B7C8: 386AABD0  addi r3, r10, -0x5430
	ctx.r[3].s64 = ctx.r[10].s64 + -21552;
	// 8325B7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B7D0: 4AFD1701  bl 0x8222ced0
	ctx.lr = 0x8325B7D4;
	sub_8222CED0(ctx, base);
	// 8325B7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B7D8: 3869C1D8  addi r3, r9, -0x3e28
	ctx.r[3].s64 = ctx.r[9].s64 + -15912;
	// 8325B7DC: 4BA4E745  bl 0x82ca9f20
	ctx.lr = 0x8325B7E0;
	sub_82CA9F20(ctx, base);
	// 8325B7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B7F0 size=64
    let mut pc: u32 = 0x8325B7F0;
    'dispatch: loop {
        match pc {
            0x8325B7F0 => {
    //   block [0x8325B7F0..0x8325B830)
	// 8325B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B7FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B804: 388BCF58  addi r4, r11, -0x30a8
	ctx.r[4].s64 = ctx.r[11].s64 + -12456;
	// 8325B808: 386AABD4  addi r3, r10, -0x542c
	ctx.r[3].s64 = ctx.r[10].s64 + -21548;
	// 8325B80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B810: 4AFD16C1  bl 0x8222ced0
	ctx.lr = 0x8325B814;
	sub_8222CED0(ctx, base);
	// 8325B814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B818: 3869C1E8  addi r3, r9, -0x3e18
	ctx.r[3].s64 = ctx.r[9].s64 + -15896;
	// 8325B81C: 4BA4E705  bl 0x82ca9f20
	ctx.lr = 0x8325B820;
	sub_82CA9F20(ctx, base);
	// 8325B820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B830 size=64
    let mut pc: u32 = 0x8325B830;
    'dispatch: loop {
        match pc {
            0x8325B830 => {
    //   block [0x8325B830..0x8325B870)
	// 8325B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B83C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B844: 388BCF94  addi r4, r11, -0x306c
	ctx.r[4].s64 = ctx.r[11].s64 + -12396;
	// 8325B848: 386AABD8  addi r3, r10, -0x5428
	ctx.r[3].s64 = ctx.r[10].s64 + -21544;
	// 8325B84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B850: 4AFD1681  bl 0x8222ced0
	ctx.lr = 0x8325B854;
	sub_8222CED0(ctx, base);
	// 8325B854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B858: 3869C1F8  addi r3, r9, -0x3e08
	ctx.r[3].s64 = ctx.r[9].s64 + -15880;
	// 8325B85C: 4BA4E6C5  bl 0x82ca9f20
	ctx.lr = 0x8325B860;
	sub_82CA9F20(ctx, base);
	// 8325B860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B870 size=64
    let mut pc: u32 = 0x8325B870;
    'dispatch: loop {
        match pc {
            0x8325B870 => {
    //   block [0x8325B870..0x8325B8B0)
	// 8325B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B87C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B884: 388BCFD0  addi r4, r11, -0x3030
	ctx.r[4].s64 = ctx.r[11].s64 + -12336;
	// 8325B888: 386AABDC  addi r3, r10, -0x5424
	ctx.r[3].s64 = ctx.r[10].s64 + -21540;
	// 8325B88C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B890: 4AFD1641  bl 0x8222ced0
	ctx.lr = 0x8325B894;
	sub_8222CED0(ctx, base);
	// 8325B894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B898: 3869C208  addi r3, r9, -0x3df8
	ctx.r[3].s64 = ctx.r[9].s64 + -15864;
	// 8325B89C: 4BA4E685  bl 0x82ca9f20
	ctx.lr = 0x8325B8A0;
	sub_82CA9F20(ctx, base);
	// 8325B8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B8B0 size=64
    let mut pc: u32 = 0x8325B8B0;
    'dispatch: loop {
        match pc {
            0x8325B8B0 => {
    //   block [0x8325B8B0..0x8325B8F0)
	// 8325B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B8B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B8BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B8C4: 388BD008  addi r4, r11, -0x2ff8
	ctx.r[4].s64 = ctx.r[11].s64 + -12280;
	// 8325B8C8: 386AABE0  addi r3, r10, -0x5420
	ctx.r[3].s64 = ctx.r[10].s64 + -21536;
	// 8325B8CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B8D0: 4AFD1601  bl 0x8222ced0
	ctx.lr = 0x8325B8D4;
	sub_8222CED0(ctx, base);
	// 8325B8D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B8D8: 3869C218  addi r3, r9, -0x3de8
	ctx.r[3].s64 = ctx.r[9].s64 + -15848;
	// 8325B8DC: 4BA4E645  bl 0x82ca9f20
	ctx.lr = 0x8325B8E0;
	sub_82CA9F20(ctx, base);
	// 8325B8E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B8F0 size=64
    let mut pc: u32 = 0x8325B8F0;
    'dispatch: loop {
        match pc {
            0x8325B8F0 => {
    //   block [0x8325B8F0..0x8325B930)
	// 8325B8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B8F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B8FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B904: 388BD030  addi r4, r11, -0x2fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -12240;
	// 8325B908: 386AABE4  addi r3, r10, -0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + -21532;
	// 8325B90C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B910: 4AFD15C1  bl 0x8222ced0
	ctx.lr = 0x8325B914;
	sub_8222CED0(ctx, base);
	// 8325B914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B918: 3869C228  addi r3, r9, -0x3dd8
	ctx.r[3].s64 = ctx.r[9].s64 + -15832;
	// 8325B91C: 4BA4E605  bl 0x82ca9f20
	ctx.lr = 0x8325B920;
	sub_82CA9F20(ctx, base);
	// 8325B920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B930 size=64
    let mut pc: u32 = 0x8325B930;
    'dispatch: loop {
        match pc {
            0x8325B930 => {
    //   block [0x8325B930..0x8325B970)
	// 8325B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B93C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B944: 388BD06C  addi r4, r11, -0x2f94
	ctx.r[4].s64 = ctx.r[11].s64 + -12180;
	// 8325B948: 386AABE8  addi r3, r10, -0x5418
	ctx.r[3].s64 = ctx.r[10].s64 + -21528;
	// 8325B94C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B950: 4AFD1581  bl 0x8222ced0
	ctx.lr = 0x8325B954;
	sub_8222CED0(ctx, base);
	// 8325B954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B958: 3869C238  addi r3, r9, -0x3dc8
	ctx.r[3].s64 = ctx.r[9].s64 + -15816;
	// 8325B95C: 4BA4E5C5  bl 0x82ca9f20
	ctx.lr = 0x8325B960;
	sub_82CA9F20(ctx, base);
	// 8325B960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B970 size=64
    let mut pc: u32 = 0x8325B970;
    'dispatch: loop {
        match pc {
            0x8325B970 => {
    //   block [0x8325B970..0x8325B9B0)
	// 8325B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B97C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B984: 388BD09C  addi r4, r11, -0x2f64
	ctx.r[4].s64 = ctx.r[11].s64 + -12132;
	// 8325B988: 386AABEC  addi r3, r10, -0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + -21524;
	// 8325B98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B990: 4AFD1541  bl 0x8222ced0
	ctx.lr = 0x8325B994;
	sub_8222CED0(ctx, base);
	// 8325B994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B998: 3869C248  addi r3, r9, -0x3db8
	ctx.r[3].s64 = ctx.r[9].s64 + -15800;
	// 8325B99C: 4BA4E585  bl 0x82ca9f20
	ctx.lr = 0x8325B9A0;
	sub_82CA9F20(ctx, base);
	// 8325B9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B9B0 size=64
    let mut pc: u32 = 0x8325B9B0;
    'dispatch: loop {
        match pc {
            0x8325B9B0 => {
    //   block [0x8325B9B0..0x8325B9F0)
	// 8325B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B9BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B9C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B9C4: 388BD0C8  addi r4, r11, -0x2f38
	ctx.r[4].s64 = ctx.r[11].s64 + -12088;
	// 8325B9C8: 386AABF0  addi r3, r10, -0x5410
	ctx.r[3].s64 = ctx.r[10].s64 + -21520;
	// 8325B9CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B9D0: 4AFD1501  bl 0x8222ced0
	ctx.lr = 0x8325B9D4;
	sub_8222CED0(ctx, base);
	// 8325B9D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B9D8: 3869C258  addi r3, r9, -0x3da8
	ctx.r[3].s64 = ctx.r[9].s64 + -15784;
	// 8325B9DC: 4BA4E545  bl 0x82ca9f20
	ctx.lr = 0x8325B9E0;
	sub_82CA9F20(ctx, base);
	// 8325B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B9F0 size=64
    let mut pc: u32 = 0x8325B9F0;
    'dispatch: loop {
        match pc {
            0x8325B9F0 => {
    //   block [0x8325B9F0..0x8325BA30)
	// 8325B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B9FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BA00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BA04: 388BD0F0  addi r4, r11, -0x2f10
	ctx.r[4].s64 = ctx.r[11].s64 + -12048;
	// 8325BA08: 386AABF4  addi r3, r10, -0x540c
	ctx.r[3].s64 = ctx.r[10].s64 + -21516;
	// 8325BA0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BA10: 4AFD14C1  bl 0x8222ced0
	ctx.lr = 0x8325BA14;
	sub_8222CED0(ctx, base);
	// 8325BA14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BA18: 3869C268  addi r3, r9, -0x3d98
	ctx.r[3].s64 = ctx.r[9].s64 + -15768;
	// 8325BA1C: 4BA4E505  bl 0x82ca9f20
	ctx.lr = 0x8325BA20;
	sub_82CA9F20(ctx, base);
	// 8325BA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BA30 size=64
    let mut pc: u32 = 0x8325BA30;
    'dispatch: loop {
        match pc {
            0x8325BA30 => {
    //   block [0x8325BA30..0x8325BA70)
	// 8325BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BA3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BA40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BA44: 388BD114  addi r4, r11, -0x2eec
	ctx.r[4].s64 = ctx.r[11].s64 + -12012;
	// 8325BA48: 386AABF8  addi r3, r10, -0x5408
	ctx.r[3].s64 = ctx.r[10].s64 + -21512;
	// 8325BA4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BA50: 4AFD1481  bl 0x8222ced0
	ctx.lr = 0x8325BA54;
	sub_8222CED0(ctx, base);
	// 8325BA54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BA58: 3869C278  addi r3, r9, -0x3d88
	ctx.r[3].s64 = ctx.r[9].s64 + -15752;
	// 8325BA5C: 4BA4E4C5  bl 0x82ca9f20
	ctx.lr = 0x8325BA60;
	sub_82CA9F20(ctx, base);
	// 8325BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BA70 size=64
    let mut pc: u32 = 0x8325BA70;
    'dispatch: loop {
        match pc {
            0x8325BA70 => {
    //   block [0x8325BA70..0x8325BAB0)
	// 8325BA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BA7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BA84: 388BD134  addi r4, r11, -0x2ecc
	ctx.r[4].s64 = ctx.r[11].s64 + -11980;
	// 8325BA88: 386AABFC  addi r3, r10, -0x5404
	ctx.r[3].s64 = ctx.r[10].s64 + -21508;
	// 8325BA8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BA90: 4AFD1441  bl 0x8222ced0
	ctx.lr = 0x8325BA94;
	sub_8222CED0(ctx, base);
	// 8325BA94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BA98: 3869C288  addi r3, r9, -0x3d78
	ctx.r[3].s64 = ctx.r[9].s64 + -15736;
	// 8325BA9C: 4BA4E485  bl 0x82ca9f20
	ctx.lr = 0x8325BAA0;
	sub_82CA9F20(ctx, base);
	// 8325BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BAB0 size=64
    let mut pc: u32 = 0x8325BAB0;
    'dispatch: loop {
        match pc {
            0x8325BAB0 => {
    //   block [0x8325BAB0..0x8325BAF0)
	// 8325BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BABC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BAC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BAC4: 388BD158  addi r4, r11, -0x2ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -11944;
	// 8325BAC8: 386AAC00  addi r3, r10, -0x5400
	ctx.r[3].s64 = ctx.r[10].s64 + -21504;
	// 8325BACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BAD0: 4AFD1401  bl 0x8222ced0
	ctx.lr = 0x8325BAD4;
	sub_8222CED0(ctx, base);
	// 8325BAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BAD8: 3869C298  addi r3, r9, -0x3d68
	ctx.r[3].s64 = ctx.r[9].s64 + -15720;
	// 8325BADC: 4BA4E445  bl 0x82ca9f20
	ctx.lr = 0x8325BAE0;
	sub_82CA9F20(ctx, base);
	// 8325BAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BAF0 size=64
    let mut pc: u32 = 0x8325BAF0;
    'dispatch: loop {
        match pc {
            0x8325BAF0 => {
    //   block [0x8325BAF0..0x8325BB30)
	// 8325BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BAFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BB00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BB04: 388BD180  addi r4, r11, -0x2e80
	ctx.r[4].s64 = ctx.r[11].s64 + -11904;
	// 8325BB08: 386AAC04  addi r3, r10, -0x53fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21500;
	// 8325BB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BB10: 4AFD13C1  bl 0x8222ced0
	ctx.lr = 0x8325BB14;
	sub_8222CED0(ctx, base);
	// 8325BB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BB18: 3869C2A8  addi r3, r9, -0x3d58
	ctx.r[3].s64 = ctx.r[9].s64 + -15704;
	// 8325BB1C: 4BA4E405  bl 0x82ca9f20
	ctx.lr = 0x8325BB20;
	sub_82CA9F20(ctx, base);
	// 8325BB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BB30 size=64
    let mut pc: u32 = 0x8325BB30;
    'dispatch: loop {
        match pc {
            0x8325BB30 => {
    //   block [0x8325BB30..0x8325BB70)
	// 8325BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BB3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BB40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BB44: 388BD1B0  addi r4, r11, -0x2e50
	ctx.r[4].s64 = ctx.r[11].s64 + -11856;
	// 8325BB48: 386AAC08  addi r3, r10, -0x53f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21496;
	// 8325BB4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BB50: 4AFD1381  bl 0x8222ced0
	ctx.lr = 0x8325BB54;
	sub_8222CED0(ctx, base);
	// 8325BB54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BB58: 3869C2B8  addi r3, r9, -0x3d48
	ctx.r[3].s64 = ctx.r[9].s64 + -15688;
	// 8325BB5C: 4BA4E3C5  bl 0x82ca9f20
	ctx.lr = 0x8325BB60;
	sub_82CA9F20(ctx, base);
	// 8325BB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BB70 size=64
    let mut pc: u32 = 0x8325BB70;
    'dispatch: loop {
        match pc {
            0x8325BB70 => {
    //   block [0x8325BB70..0x8325BBB0)
	// 8325BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BB78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BB7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BB84: 388BD1D4  addi r4, r11, -0x2e2c
	ctx.r[4].s64 = ctx.r[11].s64 + -11820;
	// 8325BB88: 386AAC0C  addi r3, r10, -0x53f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21492;
	// 8325BB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BB90: 4AFD1341  bl 0x8222ced0
	ctx.lr = 0x8325BB94;
	sub_8222CED0(ctx, base);
	// 8325BB94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BB98: 3869C2C8  addi r3, r9, -0x3d38
	ctx.r[3].s64 = ctx.r[9].s64 + -15672;
	// 8325BB9C: 4BA4E385  bl 0x82ca9f20
	ctx.lr = 0x8325BBA0;
	sub_82CA9F20(ctx, base);
	// 8325BBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BBB0 size=64
    let mut pc: u32 = 0x8325BBB0;
    'dispatch: loop {
        match pc {
            0x8325BBB0 => {
    //   block [0x8325BBB0..0x8325BBF0)
	// 8325BBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BBBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BBC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BBC4: 388BD20C  addi r4, r11, -0x2df4
	ctx.r[4].s64 = ctx.r[11].s64 + -11764;
	// 8325BBC8: 386AAC10  addi r3, r10, -0x53f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21488;
	// 8325BBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BBD0: 4AFD1301  bl 0x8222ced0
	ctx.lr = 0x8325BBD4;
	sub_8222CED0(ctx, base);
	// 8325BBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BBD8: 3869C2D8  addi r3, r9, -0x3d28
	ctx.r[3].s64 = ctx.r[9].s64 + -15656;
	// 8325BBDC: 4BA4E345  bl 0x82ca9f20
	ctx.lr = 0x8325BBE0;
	sub_82CA9F20(ctx, base);
	// 8325BBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BBF0 size=192
    let mut pc: u32 = 0x8325BBF0;
    'dispatch: loop {
        match pc {
            0x8325BBF0 => {
    //   block [0x8325BBF0..0x8325BC48)
	// 8325BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BBF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BBFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BC00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BC08: 388BD244  addi r4, r11, -0x2dbc
	ctx.r[4].s64 = ctx.r[11].s64 + -11708;
	// 8325BC0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BC10: 4AFD12C1  bl 0x8222ced0
	ctx.lr = 0x8325BC14;
	sub_8222CED0(ctx, base);
	// 8325BC14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BC18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BC1C: 4AF92F1D  bl 0x821eeb38
	ctx.lr = 0x8325BC20;
	sub_821EEB38(ctx, base);
	// 8325BC20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BC24: 4B9A7BCD  bl 0x82c037f0
	ctx.lr = 0x8325BC28;
	sub_82C037F0(ctx, base);
	// 8325BC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BC2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BC30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BC34: 916AAC14  stw r11, -0x53ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21484 as u32), ctx.r[11].u32 ) };
	// 8325BC38: 4AF6AB31  bl 0x821c6768
	ctx.lr = 0x8325BC3C;
	sub_821C6768(ctx, base);
	// 8325BC3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BC40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BC44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BC48; continue 'dispatch;
            }
            0x8325BC48 => {
    //   block [0x8325BC48..0x8325BC74)
	// 8325BC48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BC4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BC54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BC58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BC5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC60: 4082FFE8  bne 0x8325bc48
	if !ctx.cr[0].eq {
	pc = 0x8325BC48; continue 'dispatch;
	}
	// 8325BC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BC68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BC6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BC70: 4AF6AAF9  bl 0x821c6768
	ctx.lr = 0x8325BC74;
	sub_821C6768(ctx, base);
	pc = 0x8325BC74; continue 'dispatch;
            }
            0x8325BC74 => {
    //   block [0x8325BC74..0x8325BCB0)
	// 8325BC74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BC78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BC80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BC84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BC88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC8C: 4082FFE8  bne 0x8325bc74
	if !ctx.cr[0].eq {
	pc = 0x8325BC74; continue 'dispatch;
	}
	// 8325BC90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BC94: 386BC2E8  addi r3, r11, -0x3d18
	ctx.r[3].s64 = ctx.r[11].s64 + -15640;
	// 8325BC98: 4BA4E289  bl 0x82ca9f20
	ctx.lr = 0x8325BC9C;
	sub_82CA9F20(ctx, base);
	// 8325BC9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BCA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BCB0 size=192
    let mut pc: u32 = 0x8325BCB0;
    'dispatch: loop {
        match pc {
            0x8325BCB0 => {
    //   block [0x8325BCB0..0x8325BD08)
	// 8325BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BCB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BCBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BCC0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BCC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BCC8: 388BD270  addi r4, r11, -0x2d90
	ctx.r[4].s64 = ctx.r[11].s64 + -11664;
	// 8325BCCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BCD0: 4AFD1201  bl 0x8222ced0
	ctx.lr = 0x8325BCD4;
	sub_8222CED0(ctx, base);
	// 8325BCD4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BCD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BCDC: 4AF92E5D  bl 0x821eeb38
	ctx.lr = 0x8325BCE0;
	sub_821EEB38(ctx, base);
	// 8325BCE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BCE4: 4B9A7B0D  bl 0x82c037f0
	ctx.lr = 0x8325BCE8;
	sub_82C037F0(ctx, base);
	// 8325BCE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BCEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BCF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BCF4: 916AAC18  stw r11, -0x53e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 8325BCF8: 4AF6AA71  bl 0x821c6768
	ctx.lr = 0x8325BCFC;
	sub_821C6768(ctx, base);
	// 8325BCFC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BD00: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BD04: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BD08; continue 'dispatch;
            }
            0x8325BD08 => {
    //   block [0x8325BD08..0x8325BD34)
	// 8325BD08: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BD0C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD10: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BD14: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BD18: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BD1C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD20: 4082FFE8  bne 0x8325bd08
	if !ctx.cr[0].eq {
	pc = 0x8325BD08; continue 'dispatch;
	}
	// 8325BD24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BD28: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BD2C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BD30: 4AF6AA39  bl 0x821c6768
	ctx.lr = 0x8325BD34;
	sub_821C6768(ctx, base);
	pc = 0x8325BD34; continue 'dispatch;
            }
            0x8325BD34 => {
    //   block [0x8325BD34..0x8325BD70)
	// 8325BD34: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BD38: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD3C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BD40: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BD44: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BD48: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD4C: 4082FFE8  bne 0x8325bd34
	if !ctx.cr[0].eq {
	pc = 0x8325BD34; continue 'dispatch;
	}
	// 8325BD50: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BD54: 386BC2F0  addi r3, r11, -0x3d10
	ctx.r[3].s64 = ctx.r[11].s64 + -15632;
	// 8325BD58: 4BA4E1C9  bl 0x82ca9f20
	ctx.lr = 0x8325BD5C;
	sub_82CA9F20(ctx, base);
	// 8325BD5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BD70 size=192
    let mut pc: u32 = 0x8325BD70;
    'dispatch: loop {
        match pc {
            0x8325BD70 => {
    //   block [0x8325BD70..0x8325BDC8)
	// 8325BD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BD78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BD7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BD80: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BD84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BD88: 388BD29C  addi r4, r11, -0x2d64
	ctx.r[4].s64 = ctx.r[11].s64 + -11620;
	// 8325BD8C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BD90: 4AFD1141  bl 0x8222ced0
	ctx.lr = 0x8325BD94;
	sub_8222CED0(ctx, base);
	// 8325BD94: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BD98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BD9C: 4AF92D9D  bl 0x821eeb38
	ctx.lr = 0x8325BDA0;
	sub_821EEB38(ctx, base);
	// 8325BDA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BDA4: 4B9A7A4D  bl 0x82c037f0
	ctx.lr = 0x8325BDA8;
	sub_82C037F0(ctx, base);
	// 8325BDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BDAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BDB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BDB4: 916AAC1C  stw r11, -0x53e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21476 as u32), ctx.r[11].u32 ) };
	// 8325BDB8: 4AF6A9B1  bl 0x821c6768
	ctx.lr = 0x8325BDBC;
	sub_821C6768(ctx, base);
	// 8325BDBC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BDC0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BDC4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BDC8; continue 'dispatch;
            }
            0x8325BDC8 => {
    //   block [0x8325BDC8..0x8325BDF4)
	// 8325BDC8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BDCC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BDD0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BDD4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BDD8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BDDC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BDE0: 4082FFE8  bne 0x8325bdc8
	if !ctx.cr[0].eq {
	pc = 0x8325BDC8; continue 'dispatch;
	}
	// 8325BDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BDE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BDEC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BDF0: 4AF6A979  bl 0x821c6768
	ctx.lr = 0x8325BDF4;
	sub_821C6768(ctx, base);
	pc = 0x8325BDF4; continue 'dispatch;
            }
            0x8325BDF4 => {
    //   block [0x8325BDF4..0x8325BE30)
	// 8325BDF4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BDF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BDFC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BE00: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BE04: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BE08: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BE0C: 4082FFE8  bne 0x8325bdf4
	if !ctx.cr[0].eq {
	pc = 0x8325BDF4; continue 'dispatch;
	}
	// 8325BE10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BE14: 386BC2F8  addi r3, r11, -0x3d08
	ctx.r[3].s64 = ctx.r[11].s64 + -15624;
	// 8325BE18: 4BA4E109  bl 0x82ca9f20
	ctx.lr = 0x8325BE1C;
	sub_82CA9F20(ctx, base);
	// 8325BE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BE28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BE30 size=192
    let mut pc: u32 = 0x8325BE30;
    'dispatch: loop {
        match pc {
            0x8325BE30 => {
    //   block [0x8325BE30..0x8325BE88)
	// 8325BE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BE38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BE3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BE40: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BE44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BE48: 388BD2C4  addi r4, r11, -0x2d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -11580;
	// 8325BE4C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BE50: 4AFD1081  bl 0x8222ced0
	ctx.lr = 0x8325BE54;
	sub_8222CED0(ctx, base);
	// 8325BE54: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BE58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BE5C: 4AF92CDD  bl 0x821eeb38
	ctx.lr = 0x8325BE60;
	sub_821EEB38(ctx, base);
	// 8325BE60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BE64: 4B9A798D  bl 0x82c037f0
	ctx.lr = 0x8325BE68;
	sub_82C037F0(ctx, base);
	// 8325BE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BE6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BE70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BE74: 916AAC20  stw r11, -0x53e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21472 as u32), ctx.r[11].u32 ) };
	// 8325BE78: 4AF6A8F1  bl 0x821c6768
	ctx.lr = 0x8325BE7C;
	sub_821C6768(ctx, base);
	// 8325BE7C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BE80: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BE84: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BE88; continue 'dispatch;
            }
            0x8325BE88 => {
    //   block [0x8325BE88..0x8325BEB4)
	// 8325BE88: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BE8C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BE90: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BE94: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BE98: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BE9C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BEA0: 4082FFE8  bne 0x8325be88
	if !ctx.cr[0].eq {
	pc = 0x8325BE88; continue 'dispatch;
	}
	// 8325BEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BEA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BEAC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BEB0: 4AF6A8B9  bl 0x821c6768
	ctx.lr = 0x8325BEB4;
	sub_821C6768(ctx, base);
	pc = 0x8325BEB4; continue 'dispatch;
            }
            0x8325BEB4 => {
    //   block [0x8325BEB4..0x8325BEF0)
	// 8325BEB4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BEB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BEBC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BEC0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BEC4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BEC8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BECC: 4082FFE8  bne 0x8325beb4
	if !ctx.cr[0].eq {
	pc = 0x8325BEB4; continue 'dispatch;
	}
	// 8325BED0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BED4: 386BC300  addi r3, r11, -0x3d00
	ctx.r[3].s64 = ctx.r[11].s64 + -15616;
	// 8325BED8: 4BA4E049  bl 0x82ca9f20
	ctx.lr = 0x8325BEDC;
	sub_82CA9F20(ctx, base);
	// 8325BEDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BEE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BEF0 size=192
    let mut pc: u32 = 0x8325BEF0;
    'dispatch: loop {
        match pc {
            0x8325BEF0 => {
    //   block [0x8325BEF0..0x8325BF48)
	// 8325BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BEFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BF00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BF04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BF08: 388BD2F0  addi r4, r11, -0x2d10
	ctx.r[4].s64 = ctx.r[11].s64 + -11536;
	// 8325BF0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BF10: 4AFD0FC1  bl 0x8222ced0
	ctx.lr = 0x8325BF14;
	sub_8222CED0(ctx, base);
	// 8325BF14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BF1C: 4AF92C1D  bl 0x821eeb38
	ctx.lr = 0x8325BF20;
	sub_821EEB38(ctx, base);
	// 8325BF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BF24: 4B9A78CD  bl 0x82c037f0
	ctx.lr = 0x8325BF28;
	sub_82C037F0(ctx, base);
	// 8325BF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BF2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BF34: 916AAC24  stw r11, -0x53dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21468 as u32), ctx.r[11].u32 ) };
	// 8325BF38: 4AF6A831  bl 0x821c6768
	ctx.lr = 0x8325BF3C;
	sub_821C6768(ctx, base);
	// 8325BF3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BF40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BF44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BF48; continue 'dispatch;
            }
            0x8325BF48 => {
    //   block [0x8325BF48..0x8325BF74)
	// 8325BF48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BF4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BF54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BF58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BF5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF60: 4082FFE8  bne 0x8325bf48
	if !ctx.cr[0].eq {
	pc = 0x8325BF48; continue 'dispatch;
	}
	// 8325BF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BF68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BF6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BF70: 4AF6A7F9  bl 0x821c6768
	ctx.lr = 0x8325BF74;
	sub_821C6768(ctx, base);
	pc = 0x8325BF74; continue 'dispatch;
            }
            0x8325BF74 => {
    //   block [0x8325BF74..0x8325BFB0)
	// 8325BF74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BF78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BF80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BF84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BF88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF8C: 4082FFE8  bne 0x8325bf74
	if !ctx.cr[0].eq {
	pc = 0x8325BF74; continue 'dispatch;
	}
	// 8325BF90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BF94: 386BC308  addi r3, r11, -0x3cf8
	ctx.r[3].s64 = ctx.r[11].s64 + -15608;
	// 8325BF98: 4BA4DF89  bl 0x82ca9f20
	ctx.lr = 0x8325BF9C;
	sub_82CA9F20(ctx, base);
	// 8325BF9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BFB0 size=192
    let mut pc: u32 = 0x8325BFB0;
    'dispatch: loop {
        match pc {
            0x8325BFB0 => {
    //   block [0x8325BFB0..0x8325C008)
	// 8325BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BFBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BFC0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BFC8: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 8325BFCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BFD0: 4AFD0F01  bl 0x8222ced0
	ctx.lr = 0x8325BFD4;
	sub_8222CED0(ctx, base);
	// 8325BFD4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BFD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BFDC: 4AF92B5D  bl 0x821eeb38
	ctx.lr = 0x8325BFE0;
	sub_821EEB38(ctx, base);
	// 8325BFE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BFE4: 4B9A780D  bl 0x82c037f0
	ctx.lr = 0x8325BFE8;
	sub_82C037F0(ctx, base);
	// 8325BFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BFEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BFF4: 916AAC28  stw r11, -0x53d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21464 as u32), ctx.r[11].u32 ) };
	// 8325BFF8: 4AF6A771  bl 0x821c6768
	ctx.lr = 0x8325BFFC;
	sub_821C6768(ctx, base);
	// 8325BFFC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C000: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C004: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C008; continue 'dispatch;
            }
            0x8325C008 => {
    //   block [0x8325C008..0x8325C034)
	// 8325C008: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C00C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C010: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C014: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C018: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C01C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C020: 4082FFE8  bne 0x8325c008
	if !ctx.cr[0].eq {
	pc = 0x8325C008; continue 'dispatch;
	}
	// 8325C024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C028: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C02C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C030: 4AF6A739  bl 0x821c6768
	ctx.lr = 0x8325C034;
	sub_821C6768(ctx, base);
	pc = 0x8325C034; continue 'dispatch;
            }
            0x8325C034 => {
    //   block [0x8325C034..0x8325C070)
	// 8325C034: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C038: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C03C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C040: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C044: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C048: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C04C: 4082FFE8  bne 0x8325c034
	if !ctx.cr[0].eq {
	pc = 0x8325C034; continue 'dispatch;
	}
	// 8325C050: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C054: 386BC310  addi r3, r11, -0x3cf0
	ctx.r[3].s64 = ctx.r[11].s64 + -15600;
	// 8325C058: 4BA4DEC9  bl 0x82ca9f20
	ctx.lr = 0x8325C05C;
	sub_82CA9F20(ctx, base);
	// 8325C05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C070 size=192
    let mut pc: u32 = 0x8325C070;
    'dispatch: loop {
        match pc {
            0x8325C070 => {
    //   block [0x8325C070..0x8325C0C8)
	// 8325C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C07C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C080: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C088: 388BD344  addi r4, r11, -0x2cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -11452;
	// 8325C08C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C090: 4AFD0E41  bl 0x8222ced0
	ctx.lr = 0x8325C094;
	sub_8222CED0(ctx, base);
	// 8325C094: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C09C: 4AF92A9D  bl 0x821eeb38
	ctx.lr = 0x8325C0A0;
	sub_821EEB38(ctx, base);
	// 8325C0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C0A4: 4B9A774D  bl 0x82c037f0
	ctx.lr = 0x8325C0A8;
	sub_82C037F0(ctx, base);
	// 8325C0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C0AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C0B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C0B4: 916AAC2C  stw r11, -0x53d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21460 as u32), ctx.r[11].u32 ) };
	// 8325C0B8: 4AF6A6B1  bl 0x821c6768
	ctx.lr = 0x8325C0BC;
	sub_821C6768(ctx, base);
	// 8325C0BC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C0C0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C0C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C0C8; continue 'dispatch;
            }
            0x8325C0C8 => {
    //   block [0x8325C0C8..0x8325C0F4)
	// 8325C0C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C0CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C0D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C0D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C0D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C0DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C0E0: 4082FFE8  bne 0x8325c0c8
	if !ctx.cr[0].eq {
	pc = 0x8325C0C8; continue 'dispatch;
	}
	// 8325C0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C0E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C0EC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C0F0: 4AF6A679  bl 0x821c6768
	ctx.lr = 0x8325C0F4;
	sub_821C6768(ctx, base);
	pc = 0x8325C0F4; continue 'dispatch;
            }
            0x8325C0F4 => {
    //   block [0x8325C0F4..0x8325C130)
	// 8325C0F4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C0F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C0FC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C100: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C104: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C108: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C10C: 4082FFE8  bne 0x8325c0f4
	if !ctx.cr[0].eq {
	pc = 0x8325C0F4; continue 'dispatch;
	}
	// 8325C110: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C114: 386BC318  addi r3, r11, -0x3ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -15592;
	// 8325C118: 4BA4DE09  bl 0x82ca9f20
	ctx.lr = 0x8325C11C;
	sub_82CA9F20(ctx, base);
	// 8325C11C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C130 size=192
    let mut pc: u32 = 0x8325C130;
    'dispatch: loop {
        match pc {
            0x8325C130 => {
    //   block [0x8325C130..0x8325C188)
	// 8325C130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C13C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C140: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C148: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 8325C14C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C150: 4AFD0D81  bl 0x8222ced0
	ctx.lr = 0x8325C154;
	sub_8222CED0(ctx, base);
	// 8325C154: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C15C: 4AF929DD  bl 0x821eeb38
	ctx.lr = 0x8325C160;
	sub_821EEB38(ctx, base);
	// 8325C160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C164: 4B9A768D  bl 0x82c037f0
	ctx.lr = 0x8325C168;
	sub_82C037F0(ctx, base);
	// 8325C168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C16C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C174: 916AAC30  stw r11, -0x53d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21456 as u32), ctx.r[11].u32 ) };
	// 8325C178: 4AF6A5F1  bl 0x821c6768
	ctx.lr = 0x8325C17C;
	sub_821C6768(ctx, base);
	// 8325C17C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C180: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C184: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C188; continue 'dispatch;
            }
            0x8325C188 => {
    //   block [0x8325C188..0x8325C1B4)
	// 8325C188: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C18C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C190: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C194: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C198: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C19C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C1A0: 4082FFE8  bne 0x8325c188
	if !ctx.cr[0].eq {
	pc = 0x8325C188; continue 'dispatch;
	}
	// 8325C1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C1A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C1AC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C1B0: 4AF6A5B9  bl 0x821c6768
	ctx.lr = 0x8325C1B4;
	sub_821C6768(ctx, base);
	pc = 0x8325C1B4; continue 'dispatch;
            }
            0x8325C1B4 => {
    //   block [0x8325C1B4..0x8325C1F0)
	// 8325C1B4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C1B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C1BC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C1C0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C1C4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C1C8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C1CC: 4082FFE8  bne 0x8325c1b4
	if !ctx.cr[0].eq {
	pc = 0x8325C1B4; continue 'dispatch;
	}
	// 8325C1D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C1D4: 386BC320  addi r3, r11, -0x3ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -15584;
	// 8325C1D8: 4BA4DD49  bl 0x82ca9f20
	ctx.lr = 0x8325C1DC;
	sub_82CA9F20(ctx, base);
	// 8325C1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C1E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C1F0 size=192
    let mut pc: u32 = 0x8325C1F0;
    'dispatch: loop {
        match pc {
            0x8325C1F0 => {
    //   block [0x8325C1F0..0x8325C248)
	// 8325C1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C1FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C200: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C208: 388BD370  addi r4, r11, -0x2c90
	ctx.r[4].s64 = ctx.r[11].s64 + -11408;
	// 8325C20C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C210: 4AFD0CC1  bl 0x8222ced0
	ctx.lr = 0x8325C214;
	sub_8222CED0(ctx, base);
	// 8325C214: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C21C: 4AF9291D  bl 0x821eeb38
	ctx.lr = 0x8325C220;
	sub_821EEB38(ctx, base);
	// 8325C220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C224: 4B9A75CD  bl 0x82c037f0
	ctx.lr = 0x8325C228;
	sub_82C037F0(ctx, base);
	// 8325C228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C22C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C234: 916AAC34  stw r11, -0x53cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21452 as u32), ctx.r[11].u32 ) };
	// 8325C238: 4AF6A531  bl 0x821c6768
	ctx.lr = 0x8325C23C;
	sub_821C6768(ctx, base);
	// 8325C23C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C240: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C244: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C248; continue 'dispatch;
            }
            0x8325C248 => {
    //   block [0x8325C248..0x8325C274)
	// 8325C248: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C24C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C250: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C254: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C258: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C25C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C260: 4082FFE8  bne 0x8325c248
	if !ctx.cr[0].eq {
	pc = 0x8325C248; continue 'dispatch;
	}
	// 8325C264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C268: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C26C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C270: 4AF6A4F9  bl 0x821c6768
	ctx.lr = 0x8325C274;
	sub_821C6768(ctx, base);
	pc = 0x8325C274; continue 'dispatch;
            }
            0x8325C274 => {
    //   block [0x8325C274..0x8325C2B0)
	// 8325C274: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C278: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C27C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C280: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C284: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C288: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C28C: 4082FFE8  bne 0x8325c274
	if !ctx.cr[0].eq {
	pc = 0x8325C274; continue 'dispatch;
	}
	// 8325C290: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C294: 386BC328  addi r3, r11, -0x3cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -15576;
	// 8325C298: 4BA4DC89  bl 0x82ca9f20
	ctx.lr = 0x8325C29C;
	sub_82CA9F20(ctx, base);
	// 8325C29C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C2B0 size=192
    let mut pc: u32 = 0x8325C2B0;
    'dispatch: loop {
        match pc {
            0x8325C2B0 => {
    //   block [0x8325C2B0..0x8325C308)
	// 8325C2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C2B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C2BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C2C0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C2C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C2C8: 388BD39C  addi r4, r11, -0x2c64
	ctx.r[4].s64 = ctx.r[11].s64 + -11364;
	// 8325C2CC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C2D0: 4AFD0C01  bl 0x8222ced0
	ctx.lr = 0x8325C2D4;
	sub_8222CED0(ctx, base);
	// 8325C2D4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C2DC: 4AF9285D  bl 0x821eeb38
	ctx.lr = 0x8325C2E0;
	sub_821EEB38(ctx, base);
	// 8325C2E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C2E4: 4B9A750D  bl 0x82c037f0
	ctx.lr = 0x8325C2E8;
	sub_82C037F0(ctx, base);
	// 8325C2E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C2EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C2F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C2F4: 916AAC38  stw r11, -0x53c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21448 as u32), ctx.r[11].u32 ) };
	// 8325C2F8: 4AF6A471  bl 0x821c6768
	ctx.lr = 0x8325C2FC;
	sub_821C6768(ctx, base);
	// 8325C2FC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C300: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C304: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C308; continue 'dispatch;
            }
            0x8325C308 => {
    //   block [0x8325C308..0x8325C334)
	// 8325C308: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C30C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C310: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C314: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C318: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C31C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C320: 4082FFE8  bne 0x8325c308
	if !ctx.cr[0].eq {
	pc = 0x8325C308; continue 'dispatch;
	}
	// 8325C324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C328: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C32C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C330: 4AF6A439  bl 0x821c6768
	ctx.lr = 0x8325C334;
	sub_821C6768(ctx, base);
	pc = 0x8325C334; continue 'dispatch;
            }
            0x8325C334 => {
    //   block [0x8325C334..0x8325C370)
	// 8325C334: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C338: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C33C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C340: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C344: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C348: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C34C: 4082FFE8  bne 0x8325c334
	if !ctx.cr[0].eq {
	pc = 0x8325C334; continue 'dispatch;
	}
	// 8325C350: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C354: 386BC330  addi r3, r11, -0x3cd0
	ctx.r[3].s64 = ctx.r[11].s64 + -15568;
	// 8325C358: 4BA4DBC9  bl 0x82ca9f20
	ctx.lr = 0x8325C35C;
	sub_82CA9F20(ctx, base);
	// 8325C35C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C370 size=192
    let mut pc: u32 = 0x8325C370;
    'dispatch: loop {
        match pc {
            0x8325C370 => {
    //   block [0x8325C370..0x8325C3C8)
	// 8325C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C37C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C380: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C388: 388BD3C8  addi r4, r11, -0x2c38
	ctx.r[4].s64 = ctx.r[11].s64 + -11320;
	// 8325C38C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C390: 4AFD0B41  bl 0x8222ced0
	ctx.lr = 0x8325C394;
	sub_8222CED0(ctx, base);
	// 8325C394: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C39C: 4AF9279D  bl 0x821eeb38
	ctx.lr = 0x8325C3A0;
	sub_821EEB38(ctx, base);
	// 8325C3A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C3A4: 4B9A744D  bl 0x82c037f0
	ctx.lr = 0x8325C3A8;
	sub_82C037F0(ctx, base);
	// 8325C3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C3AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C3B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C3B4: 916AAC3C  stw r11, -0x53c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21444 as u32), ctx.r[11].u32 ) };
	// 8325C3B8: 4AF6A3B1  bl 0x821c6768
	ctx.lr = 0x8325C3BC;
	sub_821C6768(ctx, base);
	// 8325C3BC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C3C0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C3C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C3C8; continue 'dispatch;
            }
            0x8325C3C8 => {
    //   block [0x8325C3C8..0x8325C3F4)
	// 8325C3C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C3CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C3D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C3D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C3D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C3DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C3E0: 4082FFE8  bne 0x8325c3c8
	if !ctx.cr[0].eq {
	pc = 0x8325C3C8; continue 'dispatch;
	}
	// 8325C3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C3E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C3EC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C3F0: 4AF6A379  bl 0x821c6768
	ctx.lr = 0x8325C3F4;
	sub_821C6768(ctx, base);
	pc = 0x8325C3F4; continue 'dispatch;
            }
            0x8325C3F4 => {
    //   block [0x8325C3F4..0x8325C430)
	// 8325C3F4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C3F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C3FC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C400: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C404: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C408: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C40C: 4082FFE8  bne 0x8325c3f4
	if !ctx.cr[0].eq {
	pc = 0x8325C3F4; continue 'dispatch;
	}
	// 8325C410: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C414: 386BC338  addi r3, r11, -0x3cc8
	ctx.r[3].s64 = ctx.r[11].s64 + -15560;
	// 8325C418: 4BA4DB09  bl 0x82ca9f20
	ctx.lr = 0x8325C41C;
	sub_82CA9F20(ctx, base);
	// 8325C41C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C430 size=64
    let mut pc: u32 = 0x8325C430;
    'dispatch: loop {
        match pc {
            0x8325C430 => {
    //   block [0x8325C430..0x8325C470)
	// 8325C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C444: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325C448: 386AAC40  addi r3, r10, -0x53c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21440;
	// 8325C44C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C450: 4AFD0A81  bl 0x8222ced0
	ctx.lr = 0x8325C454;
	sub_8222CED0(ctx, base);
	// 8325C454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C458: 3869C340  addi r3, r9, -0x3cc0
	ctx.r[3].s64 = ctx.r[9].s64 + -15552;
	// 8325C45C: 4BA4DAC5  bl 0x82ca9f20
	ctx.lr = 0x8325C460;
	sub_82CA9F20(ctx, base);
	// 8325C460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C470 size=64
    let mut pc: u32 = 0x8325C470;
    'dispatch: loop {
        match pc {
            0x8325C470 => {
    //   block [0x8325C470..0x8325C4B0)
	// 8325C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C47C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C484: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325C488: 386AAC44  addi r3, r10, -0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21436;
	// 8325C48C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C490: 4AFD0A41  bl 0x8222ced0
	ctx.lr = 0x8325C494;
	sub_8222CED0(ctx, base);
	// 8325C494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C498: 3869C350  addi r3, r9, -0x3cb0
	ctx.r[3].s64 = ctx.r[9].s64 + -15536;
	// 8325C49C: 4BA4DA85  bl 0x82ca9f20
	ctx.lr = 0x8325C4A0;
	sub_82CA9F20(ctx, base);
	// 8325C4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C4B0 size=64
    let mut pc: u32 = 0x8325C4B0;
    'dispatch: loop {
        match pc {
            0x8325C4B0 => {
    //   block [0x8325C4B0..0x8325C4F0)
	// 8325C4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C4BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C4C4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325C4C8: 386AAC48  addi r3, r10, -0x53b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21432;
	// 8325C4CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C4D0: 4AFD0A01  bl 0x8222ced0
	ctx.lr = 0x8325C4D4;
	sub_8222CED0(ctx, base);
	// 8325C4D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C4D8: 3869C360  addi r3, r9, -0x3ca0
	ctx.r[3].s64 = ctx.r[9].s64 + -15520;
	// 8325C4DC: 4BA4DA45  bl 0x82ca9f20
	ctx.lr = 0x8325C4E0;
	sub_82CA9F20(ctx, base);
	// 8325C4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C4F0 size=56
    let mut pc: u32 = 0x8325C4F0;
    'dispatch: loop {
        match pc {
            0x8325C4F0 => {
    //   block [0x8325C4F0..0x8325C528)
	// 8325C4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C4FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C500: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C504: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325C508: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C50C: 4AF9784D  bl 0x821f3d58
	ctx.lr = 0x8325C510;
	sub_821F3D58(ctx, base);
	// 8325C510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C514: 906AAC4C  stw r3, -0x53b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21428 as u32), ctx.r[3].u32 ) };
	// 8325C518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C528 size=56
    let mut pc: u32 = 0x8325C528;
    'dispatch: loop {
        match pc {
            0x8325C528 => {
    //   block [0x8325C528..0x8325C560)
	// 8325C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C534: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C538: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C53C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325C540: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C544: 4AF97815  bl 0x821f3d58
	ctx.lr = 0x8325C548;
	sub_821F3D58(ctx, base);
	// 8325C548: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C54C: 906AAC50  stw r3, -0x53b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21424 as u32), ctx.r[3].u32 ) };
	// 8325C550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C560 size=56
    let mut pc: u32 = 0x8325C560;
    'dispatch: loop {
        match pc {
            0x8325C560 => {
    //   block [0x8325C560..0x8325C598)
	// 8325C560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C56C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C570: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C574: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325C578: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C57C: 4AF977DD  bl 0x821f3d58
	ctx.lr = 0x8325C580;
	sub_821F3D58(ctx, base);
	// 8325C580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C584: 906AAC54  stw r3, -0x53ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21420 as u32), ctx.r[3].u32 ) };
	// 8325C588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C598 size=56
    let mut pc: u32 = 0x8325C598;
    'dispatch: loop {
        match pc {
            0x8325C598 => {
    //   block [0x8325C598..0x8325C5D0)
	// 8325C598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C5A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C5A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C5AC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325C5B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C5B4: 4AF977A5  bl 0x821f3d58
	ctx.lr = 0x8325C5B8;
	sub_821F3D58(ctx, base);
	// 8325C5B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C5BC: 906AAC58  stw r3, -0x53a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21416 as u32), ctx.r[3].u32 ) };
	// 8325C5C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C5D0 size=56
    let mut pc: u32 = 0x8325C5D0;
    'dispatch: loop {
        match pc {
            0x8325C5D0 => {
    //   block [0x8325C5D0..0x8325C608)
	// 8325C5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C5D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C5DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C5E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C5E4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325C5E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C5EC: 4AF9776D  bl 0x821f3d58
	ctx.lr = 0x8325C5F0;
	sub_821F3D58(ctx, base);
	// 8325C5F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C5F4: 906AAC5C  stw r3, -0x53a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21412 as u32), ctx.r[3].u32 ) };
	// 8325C5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C608 size=56
    let mut pc: u32 = 0x8325C608;
    'dispatch: loop {
        match pc {
            0x8325C608 => {
    //   block [0x8325C608..0x8325C640)
	// 8325C608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C614: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C618: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C61C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325C620: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C624: 4AF97735  bl 0x821f3d58
	ctx.lr = 0x8325C628;
	sub_821F3D58(ctx, base);
	// 8325C628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C62C: 906AAC60  stw r3, -0x53a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21408 as u32), ctx.r[3].u32 ) };
	// 8325C630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C640 size=56
    let mut pc: u32 = 0x8325C640;
    'dispatch: loop {
        match pc {
            0x8325C640 => {
    //   block [0x8325C640..0x8325C678)
	// 8325C640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C64C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C650: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C654: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325C658: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C65C: 4AF976FD  bl 0x821f3d58
	ctx.lr = 0x8325C660;
	sub_821F3D58(ctx, base);
	// 8325C660: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C664: 906AAC64  stw r3, -0x539c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21404 as u32), ctx.r[3].u32 ) };
	// 8325C668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C678 size=56
    let mut pc: u32 = 0x8325C678;
    'dispatch: loop {
        match pc {
            0x8325C678 => {
    //   block [0x8325C678..0x8325C6B0)
	// 8325C678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C68C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325C690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C694: 4AF976C5  bl 0x821f3d58
	ctx.lr = 0x8325C698;
	sub_821F3D58(ctx, base);
	// 8325C698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C69C: 906AAC68  stw r3, -0x5398(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21400 as u32), ctx.r[3].u32 ) };
	// 8325C6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C6B0 size=56
    let mut pc: u32 = 0x8325C6B0;
    'dispatch: loop {
        match pc {
            0x8325C6B0 => {
    //   block [0x8325C6B0..0x8325C6E8)
	// 8325C6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C6BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C6C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C6C4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325C6C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C6CC: 4AF9768D  bl 0x821f3d58
	ctx.lr = 0x8325C6D0;
	sub_821F3D58(ctx, base);
	// 8325C6D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C6D4: 906AAC6C  stw r3, -0x5394(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21396 as u32), ctx.r[3].u32 ) };
	// 8325C6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C6E8 size=56
    let mut pc: u32 = 0x8325C6E8;
    'dispatch: loop {
        match pc {
            0x8325C6E8 => {
    //   block [0x8325C6E8..0x8325C720)
	// 8325C6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C6F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C6F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C6FC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325C700: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C704: 4AF97655  bl 0x821f3d58
	ctx.lr = 0x8325C708;
	sub_821F3D58(ctx, base);
	// 8325C708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C70C: 906AAC70  stw r3, -0x5390(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21392 as u32), ctx.r[3].u32 ) };
	// 8325C710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C720 size=56
    let mut pc: u32 = 0x8325C720;
    'dispatch: loop {
        match pc {
            0x8325C720 => {
    //   block [0x8325C720..0x8325C758)
	// 8325C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C72C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C730: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C734: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325C738: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C73C: 4AF9761D  bl 0x821f3d58
	ctx.lr = 0x8325C740;
	sub_821F3D58(ctx, base);
	// 8325C740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C744: 906AAC74  stw r3, -0x538c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21388 as u32), ctx.r[3].u32 ) };
	// 8325C748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C758 size=56
    let mut pc: u32 = 0x8325C758;
    'dispatch: loop {
        match pc {
            0x8325C758 => {
    //   block [0x8325C758..0x8325C790)
	// 8325C758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C76C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325C770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C774: 4AF975E5  bl 0x821f3d58
	ctx.lr = 0x8325C778;
	sub_821F3D58(ctx, base);
	// 8325C778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C77C: 906AAC78  stw r3, -0x5388(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21384 as u32), ctx.r[3].u32 ) };
	// 8325C780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C790 size=56
    let mut pc: u32 = 0x8325C790;
    'dispatch: loop {
        match pc {
            0x8325C790 => {
    //   block [0x8325C790..0x8325C7C8)
	// 8325C790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C79C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C7A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C7A4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325C7A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C7AC: 4AF975AD  bl 0x821f3d58
	ctx.lr = 0x8325C7B0;
	sub_821F3D58(ctx, base);
	// 8325C7B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C7B4: 906AAC7C  stw r3, -0x5384(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21380 as u32), ctx.r[3].u32 ) };
	// 8325C7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C7C8 size=56
    let mut pc: u32 = 0x8325C7C8;
    'dispatch: loop {
        match pc {
            0x8325C7C8 => {
    //   block [0x8325C7C8..0x8325C800)
	// 8325C7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C7D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C7D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C7DC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325C7E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C7E4: 4AF97575  bl 0x821f3d58
	ctx.lr = 0x8325C7E8;
	sub_821F3D58(ctx, base);
	// 8325C7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C7EC: 906AAC80  stw r3, -0x5380(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21376 as u32), ctx.r[3].u32 ) };
	// 8325C7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C800 size=56
    let mut pc: u32 = 0x8325C800;
    'dispatch: loop {
        match pc {
            0x8325C800 => {
    //   block [0x8325C800..0x8325C838)
	// 8325C800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C814: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325C818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C81C: 4AF9753D  bl 0x821f3d58
	ctx.lr = 0x8325C820;
	sub_821F3D58(ctx, base);
	// 8325C820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C824: 906AAC84  stw r3, -0x537c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21372 as u32), ctx.r[3].u32 ) };
	// 8325C828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C838 size=56
    let mut pc: u32 = 0x8325C838;
    'dispatch: loop {
        match pc {
            0x8325C838 => {
    //   block [0x8325C838..0x8325C870)
	// 8325C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C84C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325C850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C854: 4AF97505  bl 0x821f3d58
	ctx.lr = 0x8325C858;
	sub_821F3D58(ctx, base);
	// 8325C858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C85C: 906AAC88  stw r3, -0x5378(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21368 as u32), ctx.r[3].u32 ) };
	// 8325C860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C870 size=56
    let mut pc: u32 = 0x8325C870;
    'dispatch: loop {
        match pc {
            0x8325C870 => {
    //   block [0x8325C870..0x8325C8A8)
	// 8325C870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C87C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C884: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325C888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C88C: 4AF974CD  bl 0x821f3d58
	ctx.lr = 0x8325C890;
	sub_821F3D58(ctx, base);
	// 8325C890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C894: 906AAC8C  stw r3, -0x5374(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21364 as u32), ctx.r[3].u32 ) };
	// 8325C898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C8A8 size=56
    let mut pc: u32 = 0x8325C8A8;
    'dispatch: loop {
        match pc {
            0x8325C8A8 => {
    //   block [0x8325C8A8..0x8325C8E0)
	// 8325C8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C8B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C8BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325C8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C8C4: 4AF97495  bl 0x821f3d58
	ctx.lr = 0x8325C8C8;
	sub_821F3D58(ctx, base);
	// 8325C8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C8CC: 906AAC90  stw r3, -0x5370(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21360 as u32), ctx.r[3].u32 ) };
	// 8325C8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C8E0 size=56
    let mut pc: u32 = 0x8325C8E0;
    'dispatch: loop {
        match pc {
            0x8325C8E0 => {
    //   block [0x8325C8E0..0x8325C918)
	// 8325C8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C8F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325C8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C8FC: 4AF9745D  bl 0x821f3d58
	ctx.lr = 0x8325C900;
	sub_821F3D58(ctx, base);
	// 8325C900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C904: 906AAC94  stw r3, -0x536c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21356 as u32), ctx.r[3].u32 ) };
	// 8325C908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C918 size=56
    let mut pc: u32 = 0x8325C918;
    'dispatch: loop {
        match pc {
            0x8325C918 => {
    //   block [0x8325C918..0x8325C950)
	// 8325C918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C92C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325C930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C934: 4AF97425  bl 0x821f3d58
	ctx.lr = 0x8325C938;
	sub_821F3D58(ctx, base);
	// 8325C938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C93C: 906AAC98  stw r3, -0x5368(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21352 as u32), ctx.r[3].u32 ) };
	// 8325C940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C950 size=56
    let mut pc: u32 = 0x8325C950;
    'dispatch: loop {
        match pc {
            0x8325C950 => {
    //   block [0x8325C950..0x8325C988)
	// 8325C950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C95C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C964: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325C968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C96C: 4AF973ED  bl 0x821f3d58
	ctx.lr = 0x8325C970;
	sub_821F3D58(ctx, base);
	// 8325C970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C974: 906AAC9C  stw r3, -0x5364(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21348 as u32), ctx.r[3].u32 ) };
	// 8325C978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C988 size=64
    let mut pc: u32 = 0x8325C988;
    'dispatch: loop {
        match pc {
            0x8325C988 => {
    //   block [0x8325C988..0x8325C9C8)
	// 8325C988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C99C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325C9A0: 386AACA0  addi r3, r10, -0x5360
	ctx.r[3].s64 = ctx.r[10].s64 + -21344;
	// 8325C9A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C9A8: 4AFD0529  bl 0x8222ced0
	ctx.lr = 0x8325C9AC;
	sub_8222CED0(ctx, base);
	// 8325C9AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C9B0: 3869C370  addi r3, r9, -0x3c90
	ctx.r[3].s64 = ctx.r[9].s64 + -15504;
	// 8325C9B4: 4BA4D56D  bl 0x82ca9f20
	ctx.lr = 0x8325C9B8;
	sub_82CA9F20(ctx, base);
	// 8325C9B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C9C8 size=64
    let mut pc: u32 = 0x8325C9C8;
    'dispatch: loop {
        match pc {
            0x8325C9C8 => {
    //   block [0x8325C9C8..0x8325CA08)
	// 8325C9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C9D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C9D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C9DC: 388BDE44  addi r4, r11, -0x21bc
	ctx.r[4].s64 = ctx.r[11].s64 + -8636;
	// 8325C9E0: 386AACA4  addi r3, r10, -0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + -21340;
	// 8325C9E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C9E8: 4AFD04E9  bl 0x8222ced0
	ctx.lr = 0x8325C9EC;
	sub_8222CED0(ctx, base);
	// 8325C9EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C9F0: 3869C380  addi r3, r9, -0x3c80
	ctx.r[3].s64 = ctx.r[9].s64 + -15488;
	// 8325C9F4: 4BA4D52D  bl 0x82ca9f20
	ctx.lr = 0x8325C9F8;
	sub_82CA9F20(ctx, base);
	// 8325C9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CA08 size=64
    let mut pc: u32 = 0x8325CA08;
    'dispatch: loop {
        match pc {
            0x8325CA08 => {
    //   block [0x8325CA08..0x8325CA48)
	// 8325CA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CA14: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CA18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CA1C: 388BDE70  addi r4, r11, -0x2190
	ctx.r[4].s64 = ctx.r[11].s64 + -8592;
	// 8325CA20: 386AACA8  addi r3, r10, -0x5358
	ctx.r[3].s64 = ctx.r[10].s64 + -21336;
	// 8325CA24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CA28: 4AFD04A9  bl 0x8222ced0
	ctx.lr = 0x8325CA2C;
	sub_8222CED0(ctx, base);
	// 8325CA2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CA30: 3869C390  addi r3, r9, -0x3c70
	ctx.r[3].s64 = ctx.r[9].s64 + -15472;
	// 8325CA34: 4BA4D4ED  bl 0x82ca9f20
	ctx.lr = 0x8325CA38;
	sub_82CA9F20(ctx, base);
	// 8325CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CA48 size=56
    let mut pc: u32 = 0x8325CA48;
    'dispatch: loop {
        match pc {
            0x8325CA48 => {
    //   block [0x8325CA48..0x8325CA80)
	// 8325CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CA54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CA58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CA5C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325CA60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CA64: 4AF972F5  bl 0x821f3d58
	ctx.lr = 0x8325CA68;
	sub_821F3D58(ctx, base);
	// 8325CA68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CA6C: 906AACAC  stw r3, -0x5354(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21332 as u32), ctx.r[3].u32 ) };
	// 8325CA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CA80 size=56
    let mut pc: u32 = 0x8325CA80;
    'dispatch: loop {
        match pc {
            0x8325CA80 => {
    //   block [0x8325CA80..0x8325CAB8)
	// 8325CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CA90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CA94: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325CA98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CA9C: 4AF972BD  bl 0x821f3d58
	ctx.lr = 0x8325CAA0;
	sub_821F3D58(ctx, base);
	// 8325CAA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CAA4: 906AACB0  stw r3, -0x5350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21328 as u32), ctx.r[3].u32 ) };
	// 8325CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CAB8 size=56
    let mut pc: u32 = 0x8325CAB8;
    'dispatch: loop {
        match pc {
            0x8325CAB8 => {
    //   block [0x8325CAB8..0x8325CAF0)
	// 8325CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CACC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325CAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CAD4: 4AF97285  bl 0x821f3d58
	ctx.lr = 0x8325CAD8;
	sub_821F3D58(ctx, base);
	// 8325CAD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CADC: 906AACB4  stw r3, -0x534c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21324 as u32), ctx.r[3].u32 ) };
	// 8325CAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CAF0 size=56
    let mut pc: u32 = 0x8325CAF0;
    'dispatch: loop {
        match pc {
            0x8325CAF0 => {
    //   block [0x8325CAF0..0x8325CB28)
	// 8325CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CB04: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325CB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CB0C: 4AF9724D  bl 0x821f3d58
	ctx.lr = 0x8325CB10;
	sub_821F3D58(ctx, base);
	// 8325CB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CB14: 906AACB8  stw r3, -0x5348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21320 as u32), ctx.r[3].u32 ) };
	// 8325CB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CB28 size=56
    let mut pc: u32 = 0x8325CB28;
    'dispatch: loop {
        match pc {
            0x8325CB28 => {
    //   block [0x8325CB28..0x8325CB60)
	// 8325CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CB3C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325CB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CB44: 4AF97215  bl 0x821f3d58
	ctx.lr = 0x8325CB48;
	sub_821F3D58(ctx, base);
	// 8325CB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CB4C: 906AACBC  stw r3, -0x5344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21316 as u32), ctx.r[3].u32 ) };
	// 8325CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CB60 size=56
    let mut pc: u32 = 0x8325CB60;
    'dispatch: loop {
        match pc {
            0x8325CB60 => {
    //   block [0x8325CB60..0x8325CB98)
	// 8325CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CB74: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325CB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CB7C: 4AF971DD  bl 0x821f3d58
	ctx.lr = 0x8325CB80;
	sub_821F3D58(ctx, base);
	// 8325CB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CB84: 906AACC0  stw r3, -0x5340(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21312 as u32), ctx.r[3].u32 ) };
	// 8325CB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CB98 size=56
    let mut pc: u32 = 0x8325CB98;
    'dispatch: loop {
        match pc {
            0x8325CB98 => {
    //   block [0x8325CB98..0x8325CBD0)
	// 8325CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CBA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CBA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CBAC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325CBB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CBB4: 4AF971A5  bl 0x821f3d58
	ctx.lr = 0x8325CBB8;
	sub_821F3D58(ctx, base);
	// 8325CBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CBBC: 906AACC4  stw r3, -0x533c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21308 as u32), ctx.r[3].u32 ) };
	// 8325CBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CBD0 size=56
    let mut pc: u32 = 0x8325CBD0;
    'dispatch: loop {
        match pc {
            0x8325CBD0 => {
    //   block [0x8325CBD0..0x8325CC08)
	// 8325CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CBD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CBDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CBE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CBE4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325CBE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CBEC: 4AF9716D  bl 0x821f3d58
	ctx.lr = 0x8325CBF0;
	sub_821F3D58(ctx, base);
	// 8325CBF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CBF4: 906AACC8  stw r3, -0x5338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21304 as u32), ctx.r[3].u32 ) };
	// 8325CBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CC08 size=56
    let mut pc: u32 = 0x8325CC08;
    'dispatch: loop {
        match pc {
            0x8325CC08 => {
    //   block [0x8325CC08..0x8325CC40)
	// 8325CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CC14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CC18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CC1C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325CC20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CC24: 4AF97135  bl 0x821f3d58
	ctx.lr = 0x8325CC28;
	sub_821F3D58(ctx, base);
	// 8325CC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CC2C: 906AACCC  stw r3, -0x5334(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21300 as u32), ctx.r[3].u32 ) };
	// 8325CC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CC40 size=56
    let mut pc: u32 = 0x8325CC40;
    'dispatch: loop {
        match pc {
            0x8325CC40 => {
    //   block [0x8325CC40..0x8325CC78)
	// 8325CC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CC50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CC54: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325CC58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CC5C: 4AF970FD  bl 0x821f3d58
	ctx.lr = 0x8325CC60;
	sub_821F3D58(ctx, base);
	// 8325CC60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CC64: 906AACD0  stw r3, -0x5330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21296 as u32), ctx.r[3].u32 ) };
	// 8325CC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CC78 size=56
    let mut pc: u32 = 0x8325CC78;
    'dispatch: loop {
        match pc {
            0x8325CC78 => {
    //   block [0x8325CC78..0x8325CCB0)
	// 8325CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CC8C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325CC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CC94: 4AF970C5  bl 0x821f3d58
	ctx.lr = 0x8325CC98;
	sub_821F3D58(ctx, base);
	// 8325CC98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CC9C: 906AACD4  stw r3, -0x532c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21292 as u32), ctx.r[3].u32 ) };
	// 8325CCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CCB0 size=56
    let mut pc: u32 = 0x8325CCB0;
    'dispatch: loop {
        match pc {
            0x8325CCB0 => {
    //   block [0x8325CCB0..0x8325CCE8)
	// 8325CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CCC4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325CCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CCCC: 4AF9708D  bl 0x821f3d58
	ctx.lr = 0x8325CCD0;
	sub_821F3D58(ctx, base);
	// 8325CCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CCD4: 906AACD8  stw r3, -0x5328(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21288 as u32), ctx.r[3].u32 ) };
	// 8325CCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CCE8 size=56
    let mut pc: u32 = 0x8325CCE8;
    'dispatch: loop {
        match pc {
            0x8325CCE8 => {
    //   block [0x8325CCE8..0x8325CD20)
	// 8325CCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CCFC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325CD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CD04: 4AF97055  bl 0x821f3d58
	ctx.lr = 0x8325CD08;
	sub_821F3D58(ctx, base);
	// 8325CD08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CD0C: 906AACDC  stw r3, -0x5324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21284 as u32), ctx.r[3].u32 ) };
	// 8325CD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CD20 size=56
    let mut pc: u32 = 0x8325CD20;
    'dispatch: loop {
        match pc {
            0x8325CD20 => {
    //   block [0x8325CD20..0x8325CD58)
	// 8325CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CD34: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325CD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CD3C: 4AF9701D  bl 0x821f3d58
	ctx.lr = 0x8325CD40;
	sub_821F3D58(ctx, base);
	// 8325CD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CD44: 906AACE0  stw r3, -0x5320(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21280 as u32), ctx.r[3].u32 ) };
	// 8325CD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CD58 size=56
    let mut pc: u32 = 0x8325CD58;
    'dispatch: loop {
        match pc {
            0x8325CD58 => {
    //   block [0x8325CD58..0x8325CD90)
	// 8325CD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CD6C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325CD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CD74: 4AF96FE5  bl 0x821f3d58
	ctx.lr = 0x8325CD78;
	sub_821F3D58(ctx, base);
	// 8325CD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CD7C: 906AACE4  stw r3, -0x531c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21276 as u32), ctx.r[3].u32 ) };
	// 8325CD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CD90 size=56
    let mut pc: u32 = 0x8325CD90;
    'dispatch: loop {
        match pc {
            0x8325CD90 => {
    //   block [0x8325CD90..0x8325CDC8)
	// 8325CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CDA4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325CDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CDAC: 4AF96FAD  bl 0x821f3d58
	ctx.lr = 0x8325CDB0;
	sub_821F3D58(ctx, base);
	// 8325CDB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CDB4: 906AACE8  stw r3, -0x5318(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21272 as u32), ctx.r[3].u32 ) };
	// 8325CDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CDC8 size=56
    let mut pc: u32 = 0x8325CDC8;
    'dispatch: loop {
        match pc {
            0x8325CDC8 => {
    //   block [0x8325CDC8..0x8325CE00)
	// 8325CDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CDDC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325CDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CDE4: 4AF96F75  bl 0x821f3d58
	ctx.lr = 0x8325CDE8;
	sub_821F3D58(ctx, base);
	// 8325CDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CDEC: 906AACEC  stw r3, -0x5314(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21268 as u32), ctx.r[3].u32 ) };
	// 8325CDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CE00 size=56
    let mut pc: u32 = 0x8325CE00;
    'dispatch: loop {
        match pc {
            0x8325CE00 => {
    //   block [0x8325CE00..0x8325CE38)
	// 8325CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CE14: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325CE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CE1C: 4AF96F3D  bl 0x821f3d58
	ctx.lr = 0x8325CE20;
	sub_821F3D58(ctx, base);
	// 8325CE20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CE24: 906AACF0  stw r3, -0x5310(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21264 as u32), ctx.r[3].u32 ) };
	// 8325CE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CE38 size=56
    let mut pc: u32 = 0x8325CE38;
    'dispatch: loop {
        match pc {
            0x8325CE38 => {
    //   block [0x8325CE38..0x8325CE70)
	// 8325CE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CE4C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325CE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CE54: 4AF96F05  bl 0x821f3d58
	ctx.lr = 0x8325CE58;
	sub_821F3D58(ctx, base);
	// 8325CE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CE5C: 906AACF4  stw r3, -0x530c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21260 as u32), ctx.r[3].u32 ) };
	// 8325CE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CE70 size=56
    let mut pc: u32 = 0x8325CE70;
    'dispatch: loop {
        match pc {
            0x8325CE70 => {
    //   block [0x8325CE70..0x8325CEA8)
	// 8325CE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CE84: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325CE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CE8C: 4AF96ECD  bl 0x821f3d58
	ctx.lr = 0x8325CE90;
	sub_821F3D58(ctx, base);
	// 8325CE90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CE94: 906AACF8  stw r3, -0x5308(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21256 as u32), ctx.r[3].u32 ) };
	// 8325CE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CEA8 size=56
    let mut pc: u32 = 0x8325CEA8;
    'dispatch: loop {
        match pc {
            0x8325CEA8 => {
    //   block [0x8325CEA8..0x8325CEE0)
	// 8325CEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CEB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CEBC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325CEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CEC4: 4AF96E95  bl 0x821f3d58
	ctx.lr = 0x8325CEC8;
	sub_821F3D58(ctx, base);
	// 8325CEC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CECC: 906AACFC  stw r3, -0x5304(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21252 as u32), ctx.r[3].u32 ) };
	// 8325CED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CEE0 size=64
    let mut pc: u32 = 0x8325CEE0;
    'dispatch: loop {
        match pc {
            0x8325CEE0 => {
    //   block [0x8325CEE0..0x8325CF20)
	// 8325CEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CEEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CEF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CEF4: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 8325CEF8: 386AAD00  addi r3, r10, -0x5300
	ctx.r[3].s64 = ctx.r[10].s64 + -21248;
	// 8325CEFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CF00: 4AFCFFD1  bl 0x8222ced0
	ctx.lr = 0x8325CF04;
	sub_8222CED0(ctx, base);
	// 8325CF04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CF08: 3869C3A0  addi r3, r9, -0x3c60
	ctx.r[3].s64 = ctx.r[9].s64 + -15456;
	// 8325CF0C: 4BA4D015  bl 0x82ca9f20
	ctx.lr = 0x8325CF10;
	sub_82CA9F20(ctx, base);
	// 8325CF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CF20 size=64
    let mut pc: u32 = 0x8325CF20;
    'dispatch: loop {
        match pc {
            0x8325CF20 => {
    //   block [0x8325CF20..0x8325CF60)
	// 8325CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CF2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CF30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CF34: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325CF38: 386AAD04  addi r3, r10, -0x52fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21244;
	// 8325CF3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CF40: 4AFCFF91  bl 0x8222ced0
	ctx.lr = 0x8325CF44;
	sub_8222CED0(ctx, base);
	// 8325CF44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CF48: 3869C3B0  addi r3, r9, -0x3c50
	ctx.r[3].s64 = ctx.r[9].s64 + -15440;
	// 8325CF4C: 4BA4CFD5  bl 0x82ca9f20
	ctx.lr = 0x8325CF50;
	sub_82CA9F20(ctx, base);
	// 8325CF50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CF60 size=64
    let mut pc: u32 = 0x8325CF60;
    'dispatch: loop {
        match pc {
            0x8325CF60 => {
    //   block [0x8325CF60..0x8325CFA0)
	// 8325CF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CF68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CF6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CF70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CF74: 388BDEE0  addi r4, r11, -0x2120
	ctx.r[4].s64 = ctx.r[11].s64 + -8480;
	// 8325CF78: 386AAD08  addi r3, r10, -0x52f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21240;
	// 8325CF7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CF80: 4AFCFF51  bl 0x8222ced0
	ctx.lr = 0x8325CF84;
	sub_8222CED0(ctx, base);
	// 8325CF84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CF88: 3869C3C0  addi r3, r9, -0x3c40
	ctx.r[3].s64 = ctx.r[9].s64 + -15424;
	// 8325CF8C: 4BA4CF95  bl 0x82ca9f20
	ctx.lr = 0x8325CF90;
	sub_82CA9F20(ctx, base);
	// 8325CF90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CFA0 size=64
    let mut pc: u32 = 0x8325CFA0;
    'dispatch: loop {
        match pc {
            0x8325CFA0 => {
    //   block [0x8325CFA0..0x8325CFE0)
	// 8325CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CFA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CFAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CFB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CFB4: 388BDF1C  addi r4, r11, -0x20e4
	ctx.r[4].s64 = ctx.r[11].s64 + -8420;
	// 8325CFB8: 386AAD0C  addi r3, r10, -0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21236;
	// 8325CFBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CFC0: 4AFCFF11  bl 0x8222ced0
	ctx.lr = 0x8325CFC4;
	sub_8222CED0(ctx, base);
	// 8325CFC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CFC8: 3869C3D0  addi r3, r9, -0x3c30
	ctx.r[3].s64 = ctx.r[9].s64 + -15408;
	// 8325CFCC: 4BA4CF55  bl 0x82ca9f20
	ctx.lr = 0x8325CFD0;
	sub_82CA9F20(ctx, base);
	// 8325CFD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CFE0 size=64
    let mut pc: u32 = 0x8325CFE0;
    'dispatch: loop {
        match pc {
            0x8325CFE0 => {
    //   block [0x8325CFE0..0x8325D020)
	// 8325CFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CFE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CFEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CFF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CFF4: 388BDF50  addi r4, r11, -0x20b0
	ctx.r[4].s64 = ctx.r[11].s64 + -8368;
	// 8325CFF8: 386AAD10  addi r3, r10, -0x52f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21232;
	// 8325CFFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D000: 4AFCFED1  bl 0x8222ced0
	ctx.lr = 0x8325D004;
	sub_8222CED0(ctx, base);
	// 8325D004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D008: 3869C3E0  addi r3, r9, -0x3c20
	ctx.r[3].s64 = ctx.r[9].s64 + -15392;
	// 8325D00C: 4BA4CF15  bl 0x82ca9f20
	ctx.lr = 0x8325D010;
	sub_82CA9F20(ctx, base);
	// 8325D010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D020 size=64
    let mut pc: u32 = 0x8325D020;
    'dispatch: loop {
        match pc {
            0x8325D020 => {
    //   block [0x8325D020..0x8325D060)
	// 8325D020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D02C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D034: 388BDF8C  addi r4, r11, -0x2074
	ctx.r[4].s64 = ctx.r[11].s64 + -8308;
	// 8325D038: 386AAD14  addi r3, r10, -0x52ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21228;
	// 8325D03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D040: 4AFCFE91  bl 0x8222ced0
	ctx.lr = 0x8325D044;
	sub_8222CED0(ctx, base);
	// 8325D044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D048: 3869C3F0  addi r3, r9, -0x3c10
	ctx.r[3].s64 = ctx.r[9].s64 + -15376;
	// 8325D04C: 4BA4CED5  bl 0x82ca9f20
	ctx.lr = 0x8325D050;
	sub_82CA9F20(ctx, base);
	// 8325D050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D060 size=64
    let mut pc: u32 = 0x8325D060;
    'dispatch: loop {
        match pc {
            0x8325D060 => {
    //   block [0x8325D060..0x8325D0A0)
	// 8325D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D06C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D074: 388BDFC0  addi r4, r11, -0x2040
	ctx.r[4].s64 = ctx.r[11].s64 + -8256;
	// 8325D078: 386AAD18  addi r3, r10, -0x52e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21224;
	// 8325D07C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D080: 4AFCFE51  bl 0x8222ced0
	ctx.lr = 0x8325D084;
	sub_8222CED0(ctx, base);
	// 8325D084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D088: 3869C400  addi r3, r9, -0x3c00
	ctx.r[3].s64 = ctx.r[9].s64 + -15360;
	// 8325D08C: 4BA4CE95  bl 0x82ca9f20
	ctx.lr = 0x8325D090;
	sub_82CA9F20(ctx, base);
	// 8325D090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D0A0 size=64
    let mut pc: u32 = 0x8325D0A0;
    'dispatch: loop {
        match pc {
            0x8325D0A0 => {
    //   block [0x8325D0A0..0x8325D0E0)
	// 8325D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D0AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D0B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D0B4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325D0B8: 386AAD1C  addi r3, r10, -0x52e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21220;
	// 8325D0BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D0C0: 4AFCFE11  bl 0x8222ced0
	ctx.lr = 0x8325D0C4;
	sub_8222CED0(ctx, base);
	// 8325D0C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D0C8: 3869C410  addi r3, r9, -0x3bf0
	ctx.r[3].s64 = ctx.r[9].s64 + -15344;
	// 8325D0CC: 4BA4CE55  bl 0x82ca9f20
	ctx.lr = 0x8325D0D0;
	sub_82CA9F20(ctx, base);
	// 8325D0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D0E0 size=64
    let mut pc: u32 = 0x8325D0E0;
    'dispatch: loop {
        match pc {
            0x8325D0E0 => {
    //   block [0x8325D0E0..0x8325D120)
	// 8325D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D0EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325D0F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D0F4: 388BE9D4  addi r4, r11, -0x162c
	ctx.r[4].s64 = ctx.r[11].s64 + -5676;
	// 8325D0F8: 386AAD20  addi r3, r10, -0x52e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21216;
	// 8325D0FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D100: 4AFCFDD1  bl 0x8222ced0
	ctx.lr = 0x8325D104;
	sub_8222CED0(ctx, base);
	// 8325D104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D108: 3869C420  addi r3, r9, -0x3be0
	ctx.r[3].s64 = ctx.r[9].s64 + -15328;
	// 8325D10C: 4BA4CE15  bl 0x82ca9f20
	ctx.lr = 0x8325D110;
	sub_82CA9F20(ctx, base);
	// 8325D110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D120 size=64
    let mut pc: u32 = 0x8325D120;
    'dispatch: loop {
        match pc {
            0x8325D120 => {
    //   block [0x8325D120..0x8325D160)
	// 8325D120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D12C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D134: 388BE094  addi r4, r11, -0x1f6c
	ctx.r[4].s64 = ctx.r[11].s64 + -8044;
	// 8325D138: 386AAD24  addi r3, r10, -0x52dc
	ctx.r[3].s64 = ctx.r[10].s64 + -21212;
	// 8325D13C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D140: 4AFCFD91  bl 0x8222ced0
	ctx.lr = 0x8325D144;
	sub_8222CED0(ctx, base);
	// 8325D144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D148: 3869C430  addi r3, r9, -0x3bd0
	ctx.r[3].s64 = ctx.r[9].s64 + -15312;
	// 8325D14C: 4BA4CDD5  bl 0x82ca9f20
	ctx.lr = 0x8325D150;
	sub_82CA9F20(ctx, base);
	// 8325D150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


