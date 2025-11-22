pub fn sub_83273AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273AC0 size=56
    let mut pc: u32 = 0x83273AC0;
    'dispatch: loop {
        match pc {
            0x83273AC0 => {
    //   block [0x83273AC0..0x83273AF8)
	// 83273AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273AD4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83273AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273ADC: 4AF8027D  bl 0x821f3d58
	ctx.lr = 0x83273AE0;
	sub_821F3D58(ctx, base);
	// 83273AE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273AE4: 906AD1B4  stw r3, -0x2e4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11852 as u32), ctx.r[3].u32 ) };
	// 83273AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273AF8 size=56
    let mut pc: u32 = 0x83273AF8;
    'dispatch: loop {
        match pc {
            0x83273AF8 => {
    //   block [0x83273AF8..0x83273B30)
	// 83273AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273B0C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83273B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273B14: 4AF80245  bl 0x821f3d58
	ctx.lr = 0x83273B18;
	sub_821F3D58(ctx, base);
	// 83273B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273B1C: 906AD1B8  stw r3, -0x2e48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11848 as u32), ctx.r[3].u32 ) };
	// 83273B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273B30 size=56
    let mut pc: u32 = 0x83273B30;
    'dispatch: loop {
        match pc {
            0x83273B30 => {
    //   block [0x83273B30..0x83273B68)
	// 83273B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273B44: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83273B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273B4C: 4AF8020D  bl 0x821f3d58
	ctx.lr = 0x83273B50;
	sub_821F3D58(ctx, base);
	// 83273B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273B54: 906AD1BC  stw r3, -0x2e44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11844 as u32), ctx.r[3].u32 ) };
	// 83273B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273B68 size=56
    let mut pc: u32 = 0x83273B68;
    'dispatch: loop {
        match pc {
            0x83273B68 => {
    //   block [0x83273B68..0x83273BA0)
	// 83273B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273B7C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83273B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273B84: 4AF801D5  bl 0x821f3d58
	ctx.lr = 0x83273B88;
	sub_821F3D58(ctx, base);
	// 83273B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273B8C: 906AD1C0  stw r3, -0x2e40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11840 as u32), ctx.r[3].u32 ) };
	// 83273B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273BA0 size=56
    let mut pc: u32 = 0x83273BA0;
    'dispatch: loop {
        match pc {
            0x83273BA0 => {
    //   block [0x83273BA0..0x83273BD8)
	// 83273BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273BB4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83273BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273BBC: 4AF8019D  bl 0x821f3d58
	ctx.lr = 0x83273BC0;
	sub_821F3D58(ctx, base);
	// 83273BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273BC4: 906AD1C4  stw r3, -0x2e3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11836 as u32), ctx.r[3].u32 ) };
	// 83273BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273BD8 size=56
    let mut pc: u32 = 0x83273BD8;
    'dispatch: loop {
        match pc {
            0x83273BD8 => {
    //   block [0x83273BD8..0x83273C10)
	// 83273BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273BEC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83273BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273BF4: 4AF80165  bl 0x821f3d58
	ctx.lr = 0x83273BF8;
	sub_821F3D58(ctx, base);
	// 83273BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273BFC: 906AD1C8  stw r3, -0x2e38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11832 as u32), ctx.r[3].u32 ) };
	// 83273C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273C10 size=56
    let mut pc: u32 = 0x83273C10;
    'dispatch: loop {
        match pc {
            0x83273C10 => {
    //   block [0x83273C10..0x83273C48)
	// 83273C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273C24: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83273C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273C2C: 4AF8012D  bl 0x821f3d58
	ctx.lr = 0x83273C30;
	sub_821F3D58(ctx, base);
	// 83273C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273C34: 906AD1CC  stw r3, -0x2e34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11828 as u32), ctx.r[3].u32 ) };
	// 83273C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273C48 size=56
    let mut pc: u32 = 0x83273C48;
    'dispatch: loop {
        match pc {
            0x83273C48 => {
    //   block [0x83273C48..0x83273C80)
	// 83273C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273C5C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83273C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273C64: 4AF800F5  bl 0x821f3d58
	ctx.lr = 0x83273C68;
	sub_821F3D58(ctx, base);
	// 83273C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273C6C: 906AD1D0  stw r3, -0x2e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11824 as u32), ctx.r[3].u32 ) };
	// 83273C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273C80 size=56
    let mut pc: u32 = 0x83273C80;
    'dispatch: loop {
        match pc {
            0x83273C80 => {
    //   block [0x83273C80..0x83273CB8)
	// 83273C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273C94: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83273C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273C9C: 4AF800BD  bl 0x821f3d58
	ctx.lr = 0x83273CA0;
	sub_821F3D58(ctx, base);
	// 83273CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273CA4: 906AD1D4  stw r3, -0x2e2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11820 as u32), ctx.r[3].u32 ) };
	// 83273CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273CB8 size=56
    let mut pc: u32 = 0x83273CB8;
    'dispatch: loop {
        match pc {
            0x83273CB8 => {
    //   block [0x83273CB8..0x83273CF0)
	// 83273CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273CCC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83273CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273CD4: 4AF80085  bl 0x821f3d58
	ctx.lr = 0x83273CD8;
	sub_821F3D58(ctx, base);
	// 83273CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273CDC: 906AD1D8  stw r3, -0x2e28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11816 as u32), ctx.r[3].u32 ) };
	// 83273CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273CF0 size=56
    let mut pc: u32 = 0x83273CF0;
    'dispatch: loop {
        match pc {
            0x83273CF0 => {
    //   block [0x83273CF0..0x83273D28)
	// 83273CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273D04: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83273D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273D0C: 4AF8004D  bl 0x821f3d58
	ctx.lr = 0x83273D10;
	sub_821F3D58(ctx, base);
	// 83273D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273D14: 906AD1DC  stw r3, -0x2e24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11812 as u32), ctx.r[3].u32 ) };
	// 83273D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273D28 size=56
    let mut pc: u32 = 0x83273D28;
    'dispatch: loop {
        match pc {
            0x83273D28 => {
    //   block [0x83273D28..0x83273D60)
	// 83273D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273D3C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83273D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273D44: 4AF80015  bl 0x821f3d58
	ctx.lr = 0x83273D48;
	sub_821F3D58(ctx, base);
	// 83273D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273D4C: 906AD1E0  stw r3, -0x2e20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11808 as u32), ctx.r[3].u32 ) };
	// 83273D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273D60 size=56
    let mut pc: u32 = 0x83273D60;
    'dispatch: loop {
        match pc {
            0x83273D60 => {
    //   block [0x83273D60..0x83273D98)
	// 83273D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273D74: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83273D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273D7C: 4AF7FFDD  bl 0x821f3d58
	ctx.lr = 0x83273D80;
	sub_821F3D58(ctx, base);
	// 83273D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273D84: 906AD1E4  stw r3, -0x2e1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11804 as u32), ctx.r[3].u32 ) };
	// 83273D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273D98 size=56
    let mut pc: u32 = 0x83273D98;
    'dispatch: loop {
        match pc {
            0x83273D98 => {
    //   block [0x83273D98..0x83273DD0)
	// 83273D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273DAC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83273DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273DB4: 4AF7FFA5  bl 0x821f3d58
	ctx.lr = 0x83273DB8;
	sub_821F3D58(ctx, base);
	// 83273DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273DBC: 906AD1E8  stw r3, -0x2e18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11800 as u32), ctx.r[3].u32 ) };
	// 83273DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273DD0 size=64
    let mut pc: u32 = 0x83273DD0;
    'dispatch: loop {
        match pc {
            0x83273DD0 => {
    //   block [0x83273DD0..0x83273E10)
	// 83273DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273DDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273DE4: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 83273DE8: 386AD1EC  addi r3, r10, -0x2e14
	ctx.r[3].s64 = ctx.r[10].s64 + -11796;
	// 83273DEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273DF0: 4AFB90E1  bl 0x8222ced0
	ctx.lr = 0x83273DF4;
	sub_8222CED0(ctx, base);
	// 83273DF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273DF8: 3869F6A0  addi r3, r9, -0x960
	ctx.r[3].s64 = ctx.r[9].s64 + -2400;
	// 83273DFC: 4BA36125  bl 0x82ca9f20
	ctx.lr = 0x83273E00;
	sub_82CA9F20(ctx, base);
	// 83273E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273E10 size=64
    let mut pc: u32 = 0x83273E10;
    'dispatch: loop {
        match pc {
            0x83273E10 => {
    //   block [0x83273E10..0x83273E50)
	// 83273E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273E1C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273E20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273E24: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 83273E28: 386AD1F0  addi r3, r10, -0x2e10
	ctx.r[3].s64 = ctx.r[10].s64 + -11792;
	// 83273E2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273E30: 4AFB90A1  bl 0x8222ced0
	ctx.lr = 0x83273E34;
	sub_8222CED0(ctx, base);
	// 83273E34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273E38: 3869F6B0  addi r3, r9, -0x950
	ctx.r[3].s64 = ctx.r[9].s64 + -2384;
	// 83273E3C: 4BA360E5  bl 0x82ca9f20
	ctx.lr = 0x83273E40;
	sub_82CA9F20(ctx, base);
	// 83273E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273E50 size=64
    let mut pc: u32 = 0x83273E50;
    'dispatch: loop {
        match pc {
            0x83273E50 => {
    //   block [0x83273E50..0x83273E90)
	// 83273E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273E5C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273E64: 388BB96C  addi r4, r11, -0x4694
	ctx.r[4].s64 = ctx.r[11].s64 + -18068;
	// 83273E68: 386AD1F4  addi r3, r10, -0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11788;
	// 83273E6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273E70: 4AFB9061  bl 0x8222ced0
	ctx.lr = 0x83273E74;
	sub_8222CED0(ctx, base);
	// 83273E74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273E78: 3869F6C0  addi r3, r9, -0x940
	ctx.r[3].s64 = ctx.r[9].s64 + -2368;
	// 83273E7C: 4BA360A5  bl 0x82ca9f20
	ctx.lr = 0x83273E80;
	sub_82CA9F20(ctx, base);
	// 83273E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273E90 size=64
    let mut pc: u32 = 0x83273E90;
    'dispatch: loop {
        match pc {
            0x83273E90 => {
    //   block [0x83273E90..0x83273ED0)
	// 83273E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273E9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273EA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273EA4: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 83273EA8: 386AD1F8  addi r3, r10, -0x2e08
	ctx.r[3].s64 = ctx.r[10].s64 + -11784;
	// 83273EAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273EB0: 4AFB9021  bl 0x8222ced0
	ctx.lr = 0x83273EB4;
	sub_8222CED0(ctx, base);
	// 83273EB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273EB8: 3869F6D0  addi r3, r9, -0x930
	ctx.r[3].s64 = ctx.r[9].s64 + -2352;
	// 83273EBC: 4BA36065  bl 0x82ca9f20
	ctx.lr = 0x83273EC0;
	sub_82CA9F20(ctx, base);
	// 83273EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273ED0 size=64
    let mut pc: u32 = 0x83273ED0;
    'dispatch: loop {
        match pc {
            0x83273ED0 => {
    //   block [0x83273ED0..0x83273F10)
	// 83273ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273EDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83273EE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273EE4: 388BDEE0  addi r4, r11, -0x2120
	ctx.r[4].s64 = ctx.r[11].s64 + -8480;
	// 83273EE8: 386AD1FC  addi r3, r10, -0x2e04
	ctx.r[3].s64 = ctx.r[10].s64 + -11780;
	// 83273EEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273EF0: 4AFB8FE1  bl 0x8222ced0
	ctx.lr = 0x83273EF4;
	sub_8222CED0(ctx, base);
	// 83273EF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273EF8: 3869F6E0  addi r3, r9, -0x920
	ctx.r[3].s64 = ctx.r[9].s64 + -2336;
	// 83273EFC: 4BA36025  bl 0x82ca9f20
	ctx.lr = 0x83273F00;
	sub_82CA9F20(ctx, base);
	// 83273F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273F10 size=64
    let mut pc: u32 = 0x83273F10;
    'dispatch: loop {
        match pc {
            0x83273F10 => {
    //   block [0x83273F10..0x83273F50)
	// 83273F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273F1C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273F24: 388BB9A4  addi r4, r11, -0x465c
	ctx.r[4].s64 = ctx.r[11].s64 + -18012;
	// 83273F28: 386AD200  addi r3, r10, -0x2e00
	ctx.r[3].s64 = ctx.r[10].s64 + -11776;
	// 83273F2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273F30: 4AFB8FA1  bl 0x8222ced0
	ctx.lr = 0x83273F34;
	sub_8222CED0(ctx, base);
	// 83273F34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273F38: 3869F6F0  addi r3, r9, -0x910
	ctx.r[3].s64 = ctx.r[9].s64 + -2320;
	// 83273F3C: 4BA35FE5  bl 0x82ca9f20
	ctx.lr = 0x83273F40;
	sub_82CA9F20(ctx, base);
	// 83273F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273F50 size=64
    let mut pc: u32 = 0x83273F50;
    'dispatch: loop {
        match pc {
            0x83273F50 => {
    //   block [0x83273F50..0x83273F90)
	// 83273F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273F5C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273F60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273F64: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 83273F68: 386AD204  addi r3, r10, -0x2dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11772;
	// 83273F6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273F70: 4AFB8F61  bl 0x8222ced0
	ctx.lr = 0x83273F74;
	sub_8222CED0(ctx, base);
	// 83273F74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273F78: 3869F700  addi r3, r9, -0x900
	ctx.r[3].s64 = ctx.r[9].s64 + -2304;
	// 83273F7C: 4BA35FA5  bl 0x82ca9f20
	ctx.lr = 0x83273F80;
	sub_82CA9F20(ctx, base);
	// 83273F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273F90 size=64
    let mut pc: u32 = 0x83273F90;
    'dispatch: loop {
        match pc {
            0x83273F90 => {
    //   block [0x83273F90..0x83273FD0)
	// 83273F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273F9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273FA4: 388BBA20  addi r4, r11, -0x45e0
	ctx.r[4].s64 = ctx.r[11].s64 + -17888;
	// 83273FA8: 386AD208  addi r3, r10, -0x2df8
	ctx.r[3].s64 = ctx.r[10].s64 + -11768;
	// 83273FAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273FB0: 4AFB8F21  bl 0x8222ced0
	ctx.lr = 0x83273FB4;
	sub_8222CED0(ctx, base);
	// 83273FB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273FB8: 3869F710  addi r3, r9, -0x8f0
	ctx.r[3].s64 = ctx.r[9].s64 + -2288;
	// 83273FBC: 4BA35F65  bl 0x82ca9f20
	ctx.lr = 0x83273FC0;
	sub_82CA9F20(ctx, base);
	// 83273FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273FD0 size=64
    let mut pc: u32 = 0x83273FD0;
    'dispatch: loop {
        match pc {
            0x83273FD0 => {
    //   block [0x83273FD0..0x83274010)
	// 83273FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273FDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273FE4: 388BBA54  addi r4, r11, -0x45ac
	ctx.r[4].s64 = ctx.r[11].s64 + -17836;
	// 83273FE8: 386AD20C  addi r3, r10, -0x2df4
	ctx.r[3].s64 = ctx.r[10].s64 + -11764;
	// 83273FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273FF0: 4AFB8EE1  bl 0x8222ced0
	ctx.lr = 0x83273FF4;
	sub_8222CED0(ctx, base);
	// 83273FF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273FF8: 3869F720  addi r3, r9, -0x8e0
	ctx.r[3].s64 = ctx.r[9].s64 + -2272;
	// 83273FFC: 4BA35F25  bl 0x82ca9f20
	ctx.lr = 0x83274000;
	sub_82CA9F20(ctx, base);
	// 83274000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274010 size=64
    let mut pc: u32 = 0x83274010;
    'dispatch: loop {
        match pc {
            0x83274010 => {
    //   block [0x83274010..0x83274050)
	// 83274010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327401C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274024: 388BBA88  addi r4, r11, -0x4578
	ctx.r[4].s64 = ctx.r[11].s64 + -17784;
	// 83274028: 386AD210  addi r3, r10, -0x2df0
	ctx.r[3].s64 = ctx.r[10].s64 + -11760;
	// 8327402C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274030: 4AFB8EA1  bl 0x8222ced0
	ctx.lr = 0x83274034;
	sub_8222CED0(ctx, base);
	// 83274034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274038: 3869F730  addi r3, r9, -0x8d0
	ctx.r[3].s64 = ctx.r[9].s64 + -2256;
	// 8327403C: 4BA35EE5  bl 0x82ca9f20
	ctx.lr = 0x83274040;
	sub_82CA9F20(ctx, base);
	// 83274040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327404C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274050 size=64
    let mut pc: u32 = 0x83274050;
    'dispatch: loop {
        match pc {
            0x83274050 => {
    //   block [0x83274050..0x83274090)
	// 83274050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327405C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274064: 388BBABC  addi r4, r11, -0x4544
	ctx.r[4].s64 = ctx.r[11].s64 + -17732;
	// 83274068: 386AD214  addi r3, r10, -0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + -11756;
	// 8327406C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274070: 4AFB8E61  bl 0x8222ced0
	ctx.lr = 0x83274074;
	sub_8222CED0(ctx, base);
	// 83274074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274078: 3869F740  addi r3, r9, -0x8c0
	ctx.r[3].s64 = ctx.r[9].s64 + -2240;
	// 8327407C: 4BA35EA5  bl 0x82ca9f20
	ctx.lr = 0x83274080;
	sub_82CA9F20(ctx, base);
	// 83274080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327408C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274090 size=64
    let mut pc: u32 = 0x83274090;
    'dispatch: loop {
        match pc {
            0x83274090 => {
    //   block [0x83274090..0x832740D0)
	// 83274090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327409C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832740A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832740A4: 388BBAF8  addi r4, r11, -0x4508
	ctx.r[4].s64 = ctx.r[11].s64 + -17672;
	// 832740A8: 386AD218  addi r3, r10, -0x2de8
	ctx.r[3].s64 = ctx.r[10].s64 + -11752;
	// 832740AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832740B0: 4AFB8E21  bl 0x8222ced0
	ctx.lr = 0x832740B4;
	sub_8222CED0(ctx, base);
	// 832740B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832740B8: 3869F750  addi r3, r9, -0x8b0
	ctx.r[3].s64 = ctx.r[9].s64 + -2224;
	// 832740BC: 4BA35E65  bl 0x82ca9f20
	ctx.lr = 0x832740C0;
	sub_82CA9F20(ctx, base);
	// 832740C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832740C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832740C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832740CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832740D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832740D0 size=64
    let mut pc: u32 = 0x832740D0;
    'dispatch: loop {
        match pc {
            0x832740D0 => {
    //   block [0x832740D0..0x83274110)
	// 832740D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832740D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832740D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832740DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832740E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832740E4: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 832740E8: 386AD21C  addi r3, r10, -0x2de4
	ctx.r[3].s64 = ctx.r[10].s64 + -11748;
	// 832740EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832740F0: 4AFB8DE1  bl 0x8222ced0
	ctx.lr = 0x832740F4;
	sub_8222CED0(ctx, base);
	// 832740F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832740F8: 3869F760  addi r3, r9, -0x8a0
	ctx.r[3].s64 = ctx.r[9].s64 + -2208;
	// 832740FC: 4BA35E25  bl 0x82ca9f20
	ctx.lr = 0x83274100;
	sub_82CA9F20(ctx, base);
	// 83274100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274110 size=56
    let mut pc: u32 = 0x83274110;
    'dispatch: loop {
        match pc {
            0x83274110 => {
    //   block [0x83274110..0x83274148)
	// 83274110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327411C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274120: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274124: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83274128: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327412C: 4AF7FC2D  bl 0x821f3d58
	ctx.lr = 0x83274130;
	sub_821F3D58(ctx, base);
	// 83274130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274134: 906AD220  stw r3, -0x2de0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11744 as u32), ctx.r[3].u32 ) };
	// 83274138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327413C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274148 size=56
    let mut pc: u32 = 0x83274148;
    'dispatch: loop {
        match pc {
            0x83274148 => {
    //   block [0x83274148..0x83274180)
	// 83274148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327414C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274154: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274158: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327415C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83274160: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274164: 4AF7FBF5  bl 0x821f3d58
	ctx.lr = 0x83274168;
	sub_821F3D58(ctx, base);
	// 83274168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327416C: 906AD224  stw r3, -0x2ddc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11740 as u32), ctx.r[3].u32 ) };
	// 83274170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327417C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274180 size=56
    let mut pc: u32 = 0x83274180;
    'dispatch: loop {
        match pc {
            0x83274180 => {
    //   block [0x83274180..0x832741B8)
	// 83274180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327418C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274190: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274194: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83274198: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327419C: 4AF7FBBD  bl 0x821f3d58
	ctx.lr = 0x832741A0;
	sub_821F3D58(ctx, base);
	// 832741A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832741A4: 906AD228  stw r3, -0x2dd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11736 as u32), ctx.r[3].u32 ) };
	// 832741A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832741AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832741B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832741B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832741B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832741B8 size=56
    let mut pc: u32 = 0x832741B8;
    'dispatch: loop {
        match pc {
            0x832741B8 => {
    //   block [0x832741B8..0x832741F0)
	// 832741B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832741BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832741C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832741C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832741C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832741CC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832741D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832741D4: 4AF7FB85  bl 0x821f3d58
	ctx.lr = 0x832741D8;
	sub_821F3D58(ctx, base);
	// 832741D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832741DC: 906AD22C  stw r3, -0x2dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11732 as u32), ctx.r[3].u32 ) };
	// 832741E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832741E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832741E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832741EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832741F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832741F0 size=56
    let mut pc: u32 = 0x832741F0;
    'dispatch: loop {
        match pc {
            0x832741F0 => {
    //   block [0x832741F0..0x83274228)
	// 832741F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832741F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832741F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832741FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274200: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274204: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83274208: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327420C: 4AF7FB4D  bl 0x821f3d58
	ctx.lr = 0x83274210;
	sub_821F3D58(ctx, base);
	// 83274210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274214: 906AD230  stw r3, -0x2dd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11728 as u32), ctx.r[3].u32 ) };
	// 83274218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327421C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274228 size=56
    let mut pc: u32 = 0x83274228;
    'dispatch: loop {
        match pc {
            0x83274228 => {
    //   block [0x83274228..0x83274260)
	// 83274228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327422C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274234: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274238: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327423C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83274240: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274244: 4AF7FB15  bl 0x821f3d58
	ctx.lr = 0x83274248;
	sub_821F3D58(ctx, base);
	// 83274248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327424C: 906AD234  stw r3, -0x2dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11724 as u32), ctx.r[3].u32 ) };
	// 83274250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327425C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274260 size=56
    let mut pc: u32 = 0x83274260;
    'dispatch: loop {
        match pc {
            0x83274260 => {
    //   block [0x83274260..0x83274298)
	// 83274260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327426C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274274: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83274278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327427C: 4AF7FADD  bl 0x821f3d58
	ctx.lr = 0x83274280;
	sub_821F3D58(ctx, base);
	// 83274280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274284: 906AD238  stw r3, -0x2dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11720 as u32), ctx.r[3].u32 ) };
	// 83274288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327428C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274298 size=56
    let mut pc: u32 = 0x83274298;
    'dispatch: loop {
        match pc {
            0x83274298 => {
    //   block [0x83274298..0x832742D0)
	// 83274298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832742A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832742A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832742A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832742AC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832742B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832742B4: 4AF7FAA5  bl 0x821f3d58
	ctx.lr = 0x832742B8;
	sub_821F3D58(ctx, base);
	// 832742B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832742BC: 906AD23C  stw r3, -0x2dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11716 as u32), ctx.r[3].u32 ) };
	// 832742C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832742C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832742C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832742CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832742D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832742D0 size=56
    let mut pc: u32 = 0x832742D0;
    'dispatch: loop {
        match pc {
            0x832742D0 => {
    //   block [0x832742D0..0x83274308)
	// 832742D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832742D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832742D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832742DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832742E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832742E4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832742E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832742EC: 4AF7FA6D  bl 0x821f3d58
	ctx.lr = 0x832742F0;
	sub_821F3D58(ctx, base);
	// 832742F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832742F4: 906AD240  stw r3, -0x2dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11712 as u32), ctx.r[3].u32 ) };
	// 832742F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832742FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274308 size=56
    let mut pc: u32 = 0x83274308;
    'dispatch: loop {
        match pc {
            0x83274308 => {
    //   block [0x83274308..0x83274340)
	// 83274308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327430C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327431C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83274320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274324: 4AF7FA35  bl 0x821f3d58
	ctx.lr = 0x83274328;
	sub_821F3D58(ctx, base);
	// 83274328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327432C: 906AD244  stw r3, -0x2dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11708 as u32), ctx.r[3].u32 ) };
	// 83274330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327433C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274340 size=56
    let mut pc: u32 = 0x83274340;
    'dispatch: loop {
        match pc {
            0x83274340 => {
    //   block [0x83274340..0x83274378)
	// 83274340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327434C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274354: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83274358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327435C: 4AF7F9FD  bl 0x821f3d58
	ctx.lr = 0x83274360;
	sub_821F3D58(ctx, base);
	// 83274360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274364: 906AD248  stw r3, -0x2db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11704 as u32), ctx.r[3].u32 ) };
	// 83274368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327436C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274378 size=56
    let mut pc: u32 = 0x83274378;
    'dispatch: loop {
        match pc {
            0x83274378 => {
    //   block [0x83274378..0x832743B0)
	// 83274378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274384: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327438C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83274390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274394: 4AF7F9C5  bl 0x821f3d58
	ctx.lr = 0x83274398;
	sub_821F3D58(ctx, base);
	// 83274398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327439C: 906AD24C  stw r3, -0x2db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11700 as u32), ctx.r[3].u32 ) };
	// 832743A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832743A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832743A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832743AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832743B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832743B0 size=56
    let mut pc: u32 = 0x832743B0;
    'dispatch: loop {
        match pc {
            0x832743B0 => {
    //   block [0x832743B0..0x832743E8)
	// 832743B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832743B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832743B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832743BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832743C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832743C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832743C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832743CC: 4AF7F98D  bl 0x821f3d58
	ctx.lr = 0x832743D0;
	sub_821F3D58(ctx, base);
	// 832743D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832743D4: 906AD250  stw r3, -0x2db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11696 as u32), ctx.r[3].u32 ) };
	// 832743D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832743DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832743E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832743E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832743E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832743E8 size=56
    let mut pc: u32 = 0x832743E8;
    'dispatch: loop {
        match pc {
            0x832743E8 => {
    //   block [0x832743E8..0x83274420)
	// 832743E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832743EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832743F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832743F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832743F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832743FC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83274400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274404: 4AF7F955  bl 0x821f3d58
	ctx.lr = 0x83274408;
	sub_821F3D58(ctx, base);
	// 83274408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327440C: 906AD254  stw r3, -0x2dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11692 as u32), ctx.r[3].u32 ) };
	// 83274410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327441C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274420 size=56
    let mut pc: u32 = 0x83274420;
    'dispatch: loop {
        match pc {
            0x83274420 => {
    //   block [0x83274420..0x83274458)
	// 83274420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327442C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274434: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83274438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327443C: 4AF7F91D  bl 0x821f3d58
	ctx.lr = 0x83274440;
	sub_821F3D58(ctx, base);
	// 83274440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274444: 906AD258  stw r3, -0x2da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11688 as u32), ctx.r[3].u32 ) };
	// 83274448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327444C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274458 size=56
    let mut pc: u32 = 0x83274458;
    'dispatch: loop {
        match pc {
            0x83274458 => {
    //   block [0x83274458..0x83274490)
	// 83274458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274464: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274468: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327446C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83274470: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274474: 4AF7F8E5  bl 0x821f3d58
	ctx.lr = 0x83274478;
	sub_821F3D58(ctx, base);
	// 83274478: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327447C: 906AD25C  stw r3, -0x2da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11684 as u32), ctx.r[3].u32 ) };
	// 83274480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327448C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274490 size=56
    let mut pc: u32 = 0x83274490;
    'dispatch: loop {
        match pc {
            0x83274490 => {
    //   block [0x83274490..0x832744C8)
	// 83274490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327449C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832744A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832744A4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832744A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832744AC: 4AF7F8AD  bl 0x821f3d58
	ctx.lr = 0x832744B0;
	sub_821F3D58(ctx, base);
	// 832744B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832744B4: 906AD260  stw r3, -0x2da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11680 as u32), ctx.r[3].u32 ) };
	// 832744B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832744BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832744C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832744C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832744C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832744C8 size=56
    let mut pc: u32 = 0x832744C8;
    'dispatch: loop {
        match pc {
            0x832744C8 => {
    //   block [0x832744C8..0x83274500)
	// 832744C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832744CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832744D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832744D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832744D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832744DC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832744E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832744E4: 4AF7F875  bl 0x821f3d58
	ctx.lr = 0x832744E8;
	sub_821F3D58(ctx, base);
	// 832744E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832744EC: 906AD264  stw r3, -0x2d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11676 as u32), ctx.r[3].u32 ) };
	// 832744F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832744F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832744F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832744FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274500 size=56
    let mut pc: u32 = 0x83274500;
    'dispatch: loop {
        match pc {
            0x83274500 => {
    //   block [0x83274500..0x83274538)
	// 83274500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327450C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274510: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274514: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83274518: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327451C: 4AF7F83D  bl 0x821f3d58
	ctx.lr = 0x83274520;
	sub_821F3D58(ctx, base);
	// 83274520: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274524: 906AD268  stw r3, -0x2d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11672 as u32), ctx.r[3].u32 ) };
	// 83274528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327452C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274538 size=56
    let mut pc: u32 = 0x83274538;
    'dispatch: loop {
        match pc {
            0x83274538 => {
    //   block [0x83274538..0x83274570)
	// 83274538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327453C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274548: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327454C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83274550: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274554: 4AF7F805  bl 0x821f3d58
	ctx.lr = 0x83274558;
	sub_821F3D58(ctx, base);
	// 83274558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327455C: 906AD26C  stw r3, -0x2d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11668 as u32), ctx.r[3].u32 ) };
	// 83274560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327456C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274570 size=56
    let mut pc: u32 = 0x83274570;
    'dispatch: loop {
        match pc {
            0x83274570 => {
    //   block [0x83274570..0x832745A8)
	// 83274570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327457C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274580: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274584: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83274588: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327458C: 4AF7F7CD  bl 0x821f3d58
	ctx.lr = 0x83274590;
	sub_821F3D58(ctx, base);
	// 83274590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274594: 906AD270  stw r3, -0x2d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11664 as u32), ctx.r[3].u32 ) };
	// 83274598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327459C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832745A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832745A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832745A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832745A8 size=64
    let mut pc: u32 = 0x832745A8;
    'dispatch: loop {
        match pc {
            0x832745A8 => {
    //   block [0x832745A8..0x832745E8)
	// 832745A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832745AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832745B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832745B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832745B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832745BC: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 832745C0: 386AD274  addi r3, r10, -0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11660;
	// 832745C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832745C8: 4AFB8909  bl 0x8222ced0
	ctx.lr = 0x832745CC;
	sub_8222CED0(ctx, base);
	// 832745CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832745D0: 3869F770  addi r3, r9, -0x890
	ctx.r[3].s64 = ctx.r[9].s64 + -2192;
	// 832745D4: 4BA3594D  bl 0x82ca9f20
	ctx.lr = 0x832745D8;
	sub_82CA9F20(ctx, base);
	// 832745D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832745DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832745E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832745E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832745E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832745E8 size=64
    let mut pc: u32 = 0x832745E8;
    'dispatch: loop {
        match pc {
            0x832745E8 => {
    //   block [0x832745E8..0x83274628)
	// 832745E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832745EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832745F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832745F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832745F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832745FC: 388BBBB8  addi r4, r11, -0x4448
	ctx.r[4].s64 = ctx.r[11].s64 + -17480;
	// 83274600: 386AD278  addi r3, r10, -0x2d88
	ctx.r[3].s64 = ctx.r[10].s64 + -11656;
	// 83274604: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274608: 4AFB88C9  bl 0x8222ced0
	ctx.lr = 0x8327460C;
	sub_8222CED0(ctx, base);
	// 8327460C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274610: 3869F780  addi r3, r9, -0x880
	ctx.r[3].s64 = ctx.r[9].s64 + -2176;
	// 83274614: 4BA3590D  bl 0x82ca9f20
	ctx.lr = 0x83274618;
	sub_82CA9F20(ctx, base);
	// 83274618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327461C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274628 size=64
    let mut pc: u32 = 0x83274628;
    'dispatch: loop {
        match pc {
            0x83274628 => {
    //   block [0x83274628..0x83274668)
	// 83274628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327462C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274634: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274638: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327463C: 388BBBF0  addi r4, r11, -0x4410
	ctx.r[4].s64 = ctx.r[11].s64 + -17424;
	// 83274640: 386AD27C  addi r3, r10, -0x2d84
	ctx.r[3].s64 = ctx.r[10].s64 + -11652;
	// 83274644: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274648: 4AFB8889  bl 0x8222ced0
	ctx.lr = 0x8327464C;
	sub_8222CED0(ctx, base);
	// 8327464C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274650: 3869F790  addi r3, r9, -0x870
	ctx.r[3].s64 = ctx.r[9].s64 + -2160;
	// 83274654: 4BA358CD  bl 0x82ca9f20
	ctx.lr = 0x83274658;
	sub_82CA9F20(ctx, base);
	// 83274658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327465C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274668 size=64
    let mut pc: u32 = 0x83274668;
    'dispatch: loop {
        match pc {
            0x83274668 => {
    //   block [0x83274668..0x832746A8)
	// 83274668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274674: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327467C: 388BBB54  addi r4, r11, -0x44ac
	ctx.r[4].s64 = ctx.r[11].s64 + -17580;
	// 83274680: 386AD280  addi r3, r10, -0x2d80
	ctx.r[3].s64 = ctx.r[10].s64 + -11648;
	// 83274684: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274688: 4AFB8849  bl 0x8222ced0
	ctx.lr = 0x8327468C;
	sub_8222CED0(ctx, base);
	// 8327468C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274690: 3869F7A0  addi r3, r9, -0x860
	ctx.r[3].s64 = ctx.r[9].s64 + -2144;
	// 83274694: 4BA3588D  bl 0x82ca9f20
	ctx.lr = 0x83274698;
	sub_82CA9F20(ctx, base);
	// 83274698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327469C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832746A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832746A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832746A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832746A8 size=64
    let mut pc: u32 = 0x832746A8;
    'dispatch: loop {
        match pc {
            0x832746A8 => {
    //   block [0x832746A8..0x832746E8)
	// 832746A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832746AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832746B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832746B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832746B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832746BC: 388BBB34  addi r4, r11, -0x44cc
	ctx.r[4].s64 = ctx.r[11].s64 + -17612;
	// 832746C0: 386AD284  addi r3, r10, -0x2d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11644;
	// 832746C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832746C8: 4AFB8809  bl 0x8222ced0
	ctx.lr = 0x832746CC;
	sub_8222CED0(ctx, base);
	// 832746CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832746D0: 3869F7B0  addi r3, r9, -0x850
	ctx.r[3].s64 = ctx.r[9].s64 + -2128;
	// 832746D4: 4BA3584D  bl 0x82ca9f20
	ctx.lr = 0x832746D8;
	sub_82CA9F20(ctx, base);
	// 832746D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832746DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832746E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832746E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832746E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832746E8 size=64
    let mut pc: u32 = 0x832746E8;
    'dispatch: loop {
        match pc {
            0x832746E8 => {
    //   block [0x832746E8..0x83274728)
	// 832746E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832746EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832746F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832746F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832746F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832746FC: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83274700: 386AD288  addi r3, r10, -0x2d78
	ctx.r[3].s64 = ctx.r[10].s64 + -11640;
	// 83274704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274708: 4AFB87C9  bl 0x8222ced0
	ctx.lr = 0x8327470C;
	sub_8222CED0(ctx, base);
	// 8327470C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274710: 3869F7C0  addi r3, r9, -0x840
	ctx.r[3].s64 = ctx.r[9].s64 + -2112;
	// 83274714: 4BA3580D  bl 0x82ca9f20
	ctx.lr = 0x83274718;
	sub_82CA9F20(ctx, base);
	// 83274718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327471C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274728 size=64
    let mut pc: u32 = 0x83274728;
    'dispatch: loop {
        match pc {
            0x83274728 => {
    //   block [0x83274728..0x83274768)
	// 83274728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274734: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327473C: 388BBC28  addi r4, r11, -0x43d8
	ctx.r[4].s64 = ctx.r[11].s64 + -17368;
	// 83274740: 386AD28C  addi r3, r10, -0x2d74
	ctx.r[3].s64 = ctx.r[10].s64 + -11636;
	// 83274744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274748: 4AFB8789  bl 0x8222ced0
	ctx.lr = 0x8327474C;
	sub_8222CED0(ctx, base);
	// 8327474C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274750: 3869F7D0  addi r3, r9, -0x830
	ctx.r[3].s64 = ctx.r[9].s64 + -2096;
	// 83274754: 4BA357CD  bl 0x82ca9f20
	ctx.lr = 0x83274758;
	sub_82CA9F20(ctx, base);
	// 83274758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327475C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274768 size=64
    let mut pc: u32 = 0x83274768;
    'dispatch: loop {
        match pc {
            0x83274768 => {
    //   block [0x83274768..0x832747A8)
	// 83274768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274774: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327477C: 388BBC64  addi r4, r11, -0x439c
	ctx.r[4].s64 = ctx.r[11].s64 + -17308;
	// 83274780: 386AD290  addi r3, r10, -0x2d70
	ctx.r[3].s64 = ctx.r[10].s64 + -11632;
	// 83274784: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274788: 4AFB8749  bl 0x8222ced0
	ctx.lr = 0x8327478C;
	sub_8222CED0(ctx, base);
	// 8327478C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274790: 3869F7E0  addi r3, r9, -0x820
	ctx.r[3].s64 = ctx.r[9].s64 + -2080;
	// 83274794: 4BA3578D  bl 0x82ca9f20
	ctx.lr = 0x83274798;
	sub_82CA9F20(ctx, base);
	// 83274798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832747A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832747A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832747A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832747A8 size=64
    let mut pc: u32 = 0x832747A8;
    'dispatch: loop {
        match pc {
            0x832747A8 => {
    //   block [0x832747A8..0x832747E8)
	// 832747A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832747AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832747B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832747B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832747B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832747BC: 388BBCB0  addi r4, r11, -0x4350
	ctx.r[4].s64 = ctx.r[11].s64 + -17232;
	// 832747C0: 386AD294  addi r3, r10, -0x2d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11628;
	// 832747C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832747C8: 4AFB8709  bl 0x8222ced0
	ctx.lr = 0x832747CC;
	sub_8222CED0(ctx, base);
	// 832747CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832747D0: 3869F7F0  addi r3, r9, -0x810
	ctx.r[3].s64 = ctx.r[9].s64 + -2064;
	// 832747D4: 4BA3574D  bl 0x82ca9f20
	ctx.lr = 0x832747D8;
	sub_82CA9F20(ctx, base);
	// 832747D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832747DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832747E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832747E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832747E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832747E8 size=64
    let mut pc: u32 = 0x832747E8;
    'dispatch: loop {
        match pc {
            0x832747E8 => {
    //   block [0x832747E8..0x83274828)
	// 832747E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832747EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832747F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832747F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832747F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832747FC: 388BBCE4  addi r4, r11, -0x431c
	ctx.r[4].s64 = ctx.r[11].s64 + -17180;
	// 83274800: 386AD298  addi r3, r10, -0x2d68
	ctx.r[3].s64 = ctx.r[10].s64 + -11624;
	// 83274804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274808: 4AFB86C9  bl 0x8222ced0
	ctx.lr = 0x8327480C;
	sub_8222CED0(ctx, base);
	// 8327480C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274810: 3869F800  addi r3, r9, -0x800
	ctx.r[3].s64 = ctx.r[9].s64 + -2048;
	// 83274814: 4BA3570D  bl 0x82ca9f20
	ctx.lr = 0x83274818;
	sub_82CA9F20(ctx, base);
	// 83274818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327481C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274828 size=64
    let mut pc: u32 = 0x83274828;
    'dispatch: loop {
        match pc {
            0x83274828 => {
    //   block [0x83274828..0x83274868)
	// 83274828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274834: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274838: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327483C: 388BBD18  addi r4, r11, -0x42e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17128;
	// 83274840: 386AD29C  addi r3, r10, -0x2d64
	ctx.r[3].s64 = ctx.r[10].s64 + -11620;
	// 83274844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274848: 4AFB8689  bl 0x8222ced0
	ctx.lr = 0x8327484C;
	sub_8222CED0(ctx, base);
	// 8327484C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274850: 3869F810  addi r3, r9, -0x7f0
	ctx.r[3].s64 = ctx.r[9].s64 + -2032;
	// 83274854: 4BA356CD  bl 0x82ca9f20
	ctx.lr = 0x83274858;
	sub_82CA9F20(ctx, base);
	// 83274858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327485C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274868 size=64
    let mut pc: u32 = 0x83274868;
    'dispatch: loop {
        match pc {
            0x83274868 => {
    //   block [0x83274868..0x832748A8)
	// 83274868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327487C: 388BBD4C  addi r4, r11, -0x42b4
	ctx.r[4].s64 = ctx.r[11].s64 + -17076;
	// 83274880: 386AD2A0  addi r3, r10, -0x2d60
	ctx.r[3].s64 = ctx.r[10].s64 + -11616;
	// 83274884: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274888: 4AFB8649  bl 0x8222ced0
	ctx.lr = 0x8327488C;
	sub_8222CED0(ctx, base);
	// 8327488C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274890: 3869F820  addi r3, r9, -0x7e0
	ctx.r[3].s64 = ctx.r[9].s64 + -2016;
	// 83274894: 4BA3568D  bl 0x82ca9f20
	ctx.lr = 0x83274898;
	sub_82CA9F20(ctx, base);
	// 83274898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327489C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832748A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832748A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832748A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832748A8 size=64
    let mut pc: u32 = 0x832748A8;
    'dispatch: loop {
        match pc {
            0x832748A8 => {
    //   block [0x832748A8..0x832748E8)
	// 832748A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832748AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832748B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832748B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832748B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832748BC: 388BBD88  addi r4, r11, -0x4278
	ctx.r[4].s64 = ctx.r[11].s64 + -17016;
	// 832748C0: 386AD2A4  addi r3, r10, -0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11612;
	// 832748C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832748C8: 4AFB8609  bl 0x8222ced0
	ctx.lr = 0x832748CC;
	sub_8222CED0(ctx, base);
	// 832748CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832748D0: 3869F830  addi r3, r9, -0x7d0
	ctx.r[3].s64 = ctx.r[9].s64 + -2000;
	// 832748D4: 4BA3564D  bl 0x82ca9f20
	ctx.lr = 0x832748D8;
	sub_82CA9F20(ctx, base);
	// 832748D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832748DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832748E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832748E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832748E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832748E8 size=64
    let mut pc: u32 = 0x832748E8;
    'dispatch: loop {
        match pc {
            0x832748E8 => {
    //   block [0x832748E8..0x83274928)
	// 832748E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832748EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832748F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832748F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832748F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832748FC: 388BBDC4  addi r4, r11, -0x423c
	ctx.r[4].s64 = ctx.r[11].s64 + -16956;
	// 83274900: 386AD2A8  addi r3, r10, -0x2d58
	ctx.r[3].s64 = ctx.r[10].s64 + -11608;
	// 83274904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274908: 4AFB85C9  bl 0x8222ced0
	ctx.lr = 0x8327490C;
	sub_8222CED0(ctx, base);
	// 8327490C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274910: 3869F840  addi r3, r9, -0x7c0
	ctx.r[3].s64 = ctx.r[9].s64 + -1984;
	// 83274914: 4BA3560D  bl 0x82ca9f20
	ctx.lr = 0x83274918;
	sub_82CA9F20(ctx, base);
	// 83274918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327491C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274928 size=64
    let mut pc: u32 = 0x83274928;
    'dispatch: loop {
        match pc {
            0x83274928 => {
    //   block [0x83274928..0x83274968)
	// 83274928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327492C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274934: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327493C: 388BBDF4  addi r4, r11, -0x420c
	ctx.r[4].s64 = ctx.r[11].s64 + -16908;
	// 83274940: 386AD2AC  addi r3, r10, -0x2d54
	ctx.r[3].s64 = ctx.r[10].s64 + -11604;
	// 83274944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274948: 4AFB8589  bl 0x8222ced0
	ctx.lr = 0x8327494C;
	sub_8222CED0(ctx, base);
	// 8327494C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274950: 3869F850  addi r3, r9, -0x7b0
	ctx.r[3].s64 = ctx.r[9].s64 + -1968;
	// 83274954: 4BA355CD  bl 0x82ca9f20
	ctx.lr = 0x83274958;
	sub_82CA9F20(ctx, base);
	// 83274958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327495C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274968 size=64
    let mut pc: u32 = 0x83274968;
    'dispatch: loop {
        match pc {
            0x83274968 => {
    //   block [0x83274968..0x832749A8)
	// 83274968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327496C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274974: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327497C: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 83274980: 386AD2B0  addi r3, r10, -0x2d50
	ctx.r[3].s64 = ctx.r[10].s64 + -11600;
	// 83274984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274988: 4AFB8549  bl 0x8222ced0
	ctx.lr = 0x8327498C;
	sub_8222CED0(ctx, base);
	// 8327498C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274990: 3869F860  addi r3, r9, -0x7a0
	ctx.r[3].s64 = ctx.r[9].s64 + -1952;
	// 83274994: 4BA3558D  bl 0x82ca9f20
	ctx.lr = 0x83274998;
	sub_82CA9F20(ctx, base);
	// 83274998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832749A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832749A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832749A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832749A8 size=64
    let mut pc: u32 = 0x832749A8;
    'dispatch: loop {
        match pc {
            0x832749A8 => {
    //   block [0x832749A8..0x832749E8)
	// 832749A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832749AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832749B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832749B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832749B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832749BC: 388B29F4  addi r4, r11, 0x29f4
	ctx.r[4].s64 = ctx.r[11].s64 + 10740;
	// 832749C0: 386AD2B4  addi r3, r10, -0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11596;
	// 832749C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832749C8: 4AFB8509  bl 0x8222ced0
	ctx.lr = 0x832749CC;
	sub_8222CED0(ctx, base);
	// 832749CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832749D0: 3869F870  addi r3, r9, -0x790
	ctx.r[3].s64 = ctx.r[9].s64 + -1936;
	// 832749D4: 4BA3554D  bl 0x82ca9f20
	ctx.lr = 0x832749D8;
	sub_82CA9F20(ctx, base);
	// 832749D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832749DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832749E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832749E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832749E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832749E8 size=64
    let mut pc: u32 = 0x832749E8;
    'dispatch: loop {
        match pc {
            0x832749E8 => {
    //   block [0x832749E8..0x83274A28)
	// 832749E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832749EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832749F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832749F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832749F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832749FC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83274A00: 386AD2B8  addi r3, r10, -0x2d48
	ctx.r[3].s64 = ctx.r[10].s64 + -11592;
	// 83274A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274A08: 4AFB84C9  bl 0x8222ced0
	ctx.lr = 0x83274A0C;
	sub_8222CED0(ctx, base);
	// 83274A0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274A10: 3869F880  addi r3, r9, -0x780
	ctx.r[3].s64 = ctx.r[9].s64 + -1920;
	// 83274A14: 4BA3550D  bl 0x82ca9f20
	ctx.lr = 0x83274A18;
	sub_82CA9F20(ctx, base);
	// 83274A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274A28 size=64
    let mut pc: u32 = 0x83274A28;
    'dispatch: loop {
        match pc {
            0x83274A28 => {
    //   block [0x83274A28..0x83274A68)
	// 83274A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274A34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274A3C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83274A40: 386AD2BC  addi r3, r10, -0x2d44
	ctx.r[3].s64 = ctx.r[10].s64 + -11588;
	// 83274A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274A48: 4AFB8489  bl 0x8222ced0
	ctx.lr = 0x83274A4C;
	sub_8222CED0(ctx, base);
	// 83274A4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274A50: 3869F890  addi r3, r9, -0x770
	ctx.r[3].s64 = ctx.r[9].s64 + -1904;
	// 83274A54: 4BA354CD  bl 0x82ca9f20
	ctx.lr = 0x83274A58;
	sub_82CA9F20(ctx, base);
	// 83274A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274A68 size=64
    let mut pc: u32 = 0x83274A68;
    'dispatch: loop {
        match pc {
            0x83274A68 => {
    //   block [0x83274A68..0x83274AA8)
	// 83274A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274A74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274A7C: 388BBE94  addi r4, r11, -0x416c
	ctx.r[4].s64 = ctx.r[11].s64 + -16748;
	// 83274A80: 386AD2C0  addi r3, r10, -0x2d40
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	// 83274A84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274A88: 4AFB8449  bl 0x8222ced0
	ctx.lr = 0x83274A8C;
	sub_8222CED0(ctx, base);
	// 83274A8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274A90: 3869F8A0  addi r3, r9, -0x760
	ctx.r[3].s64 = ctx.r[9].s64 + -1888;
	// 83274A94: 4BA3548D  bl 0x82ca9f20
	ctx.lr = 0x83274A98;
	sub_82CA9F20(ctx, base);
	// 83274A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274AA8 size=64
    let mut pc: u32 = 0x83274AA8;
    'dispatch: loop {
        match pc {
            0x83274AA8 => {
    //   block [0x83274AA8..0x83274AE8)
	// 83274AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274AB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274ABC: 388BBEA0  addi r4, r11, -0x4160
	ctx.r[4].s64 = ctx.r[11].s64 + -16736;
	// 83274AC0: 386AD2C4  addi r3, r10, -0x2d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11580;
	// 83274AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274AC8: 4AFB8409  bl 0x8222ced0
	ctx.lr = 0x83274ACC;
	sub_8222CED0(ctx, base);
	// 83274ACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274AD0: 3869F8B0  addi r3, r9, -0x750
	ctx.r[3].s64 = ctx.r[9].s64 + -1872;
	// 83274AD4: 4BA3544D  bl 0x82ca9f20
	ctx.lr = 0x83274AD8;
	sub_82CA9F20(ctx, base);
	// 83274AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274AE8 size=64
    let mut pc: u32 = 0x83274AE8;
    'dispatch: loop {
        match pc {
            0x83274AE8 => {
    //   block [0x83274AE8..0x83274B28)
	// 83274AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274AF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274AFC: 388BBEB0  addi r4, r11, -0x4150
	ctx.r[4].s64 = ctx.r[11].s64 + -16720;
	// 83274B00: 386AD2C8  addi r3, r10, -0x2d38
	ctx.r[3].s64 = ctx.r[10].s64 + -11576;
	// 83274B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274B08: 4AFB83C9  bl 0x8222ced0
	ctx.lr = 0x83274B0C;
	sub_8222CED0(ctx, base);
	// 83274B0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274B10: 3869F8C0  addi r3, r9, -0x740
	ctx.r[3].s64 = ctx.r[9].s64 + -1856;
	// 83274B14: 4BA3540D  bl 0x82ca9f20
	ctx.lr = 0x83274B18;
	sub_82CA9F20(ctx, base);
	// 83274B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274B28 size=64
    let mut pc: u32 = 0x83274B28;
    'dispatch: loop {
        match pc {
            0x83274B28 => {
    //   block [0x83274B28..0x83274B68)
	// 83274B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274B34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274B38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274B3C: 388BBEBC  addi r4, r11, -0x4144
	ctx.r[4].s64 = ctx.r[11].s64 + -16708;
	// 83274B40: 386AD2CC  addi r3, r10, -0x2d34
	ctx.r[3].s64 = ctx.r[10].s64 + -11572;
	// 83274B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274B48: 4AFB8389  bl 0x8222ced0
	ctx.lr = 0x83274B4C;
	sub_8222CED0(ctx, base);
	// 83274B4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274B50: 3869F8D0  addi r3, r9, -0x730
	ctx.r[3].s64 = ctx.r[9].s64 + -1840;
	// 83274B54: 4BA353CD  bl 0x82ca9f20
	ctx.lr = 0x83274B58;
	sub_82CA9F20(ctx, base);
	// 83274B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274B68 size=64
    let mut pc: u32 = 0x83274B68;
    'dispatch: loop {
        match pc {
            0x83274B68 => {
    //   block [0x83274B68..0x83274BA8)
	// 83274B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274B74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274B78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274B7C: 388BBEC8  addi r4, r11, -0x4138
	ctx.r[4].s64 = ctx.r[11].s64 + -16696;
	// 83274B80: 386AD2D0  addi r3, r10, -0x2d30
	ctx.r[3].s64 = ctx.r[10].s64 + -11568;
	// 83274B84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274B88: 4AFB8349  bl 0x8222ced0
	ctx.lr = 0x83274B8C;
	sub_8222CED0(ctx, base);
	// 83274B8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274B90: 3869F8E0  addi r3, r9, -0x720
	ctx.r[3].s64 = ctx.r[9].s64 + -1824;
	// 83274B94: 4BA3538D  bl 0x82ca9f20
	ctx.lr = 0x83274B98;
	sub_82CA9F20(ctx, base);
	// 83274B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274BA8 size=64
    let mut pc: u32 = 0x83274BA8;
    'dispatch: loop {
        match pc {
            0x83274BA8 => {
    //   block [0x83274BA8..0x83274BE8)
	// 83274BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274BB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274BBC: 388BBEDC  addi r4, r11, -0x4124
	ctx.r[4].s64 = ctx.r[11].s64 + -16676;
	// 83274BC0: 386AD2D4  addi r3, r10, -0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11564;
	// 83274BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274BC8: 4AFB8309  bl 0x8222ced0
	ctx.lr = 0x83274BCC;
	sub_8222CED0(ctx, base);
	// 83274BCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274BD0: 3869F8F0  addi r3, r9, -0x710
	ctx.r[3].s64 = ctx.r[9].s64 + -1808;
	// 83274BD4: 4BA3534D  bl 0x82ca9f20
	ctx.lr = 0x83274BD8;
	sub_82CA9F20(ctx, base);
	// 83274BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274BE8 size=64
    let mut pc: u32 = 0x83274BE8;
    'dispatch: loop {
        match pc {
            0x83274BE8 => {
    //   block [0x83274BE8..0x83274C28)
	// 83274BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274BFC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83274C00: 386AD2D8  addi r3, r10, -0x2d28
	ctx.r[3].s64 = ctx.r[10].s64 + -11560;
	// 83274C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274C08: 4AFB82C9  bl 0x8222ced0
	ctx.lr = 0x83274C0C;
	sub_8222CED0(ctx, base);
	// 83274C0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274C10: 3869F900  addi r3, r9, -0x700
	ctx.r[3].s64 = ctx.r[9].s64 + -1792;
	// 83274C14: 4BA3530D  bl 0x82ca9f20
	ctx.lr = 0x83274C18;
	sub_82CA9F20(ctx, base);
	// 83274C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274C28 size=64
    let mut pc: u32 = 0x83274C28;
    'dispatch: loop {
        match pc {
            0x83274C28 => {
    //   block [0x83274C28..0x83274C68)
	// 83274C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274C3C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83274C40: 386AD2DC  addi r3, r10, -0x2d24
	ctx.r[3].s64 = ctx.r[10].s64 + -11556;
	// 83274C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274C48: 4AFB8289  bl 0x8222ced0
	ctx.lr = 0x83274C4C;
	sub_8222CED0(ctx, base);
	// 83274C4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274C50: 3869F910  addi r3, r9, -0x6f0
	ctx.r[3].s64 = ctx.r[9].s64 + -1776;
	// 83274C54: 4BA352CD  bl 0x82ca9f20
	ctx.lr = 0x83274C58;
	sub_82CA9F20(ctx, base);
	// 83274C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274C68 size=64
    let mut pc: u32 = 0x83274C68;
    'dispatch: loop {
        match pc {
            0x83274C68 => {
    //   block [0x83274C68..0x83274CA8)
	// 83274C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274C74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274C7C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83274C80: 386AD2E0  addi r3, r10, -0x2d20
	ctx.r[3].s64 = ctx.r[10].s64 + -11552;
	// 83274C84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274C88: 4AFB8249  bl 0x8222ced0
	ctx.lr = 0x83274C8C;
	sub_8222CED0(ctx, base);
	// 83274C8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274C90: 3869F920  addi r3, r9, -0x6e0
	ctx.r[3].s64 = ctx.r[9].s64 + -1760;
	// 83274C94: 4BA3528D  bl 0x82ca9f20
	ctx.lr = 0x83274C98;
	sub_82CA9F20(ctx, base);
	// 83274C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274CA8 size=64
    let mut pc: u32 = 0x83274CA8;
    'dispatch: loop {
        match pc {
            0x83274CA8 => {
    //   block [0x83274CA8..0x83274CE8)
	// 83274CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274CB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274CBC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83274CC0: 386AD2E4  addi r3, r10, -0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11548;
	// 83274CC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274CC8: 4AFB8209  bl 0x8222ced0
	ctx.lr = 0x83274CCC;
	sub_8222CED0(ctx, base);
	// 83274CCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274CD0: 3869F930  addi r3, r9, -0x6d0
	ctx.r[3].s64 = ctx.r[9].s64 + -1744;
	// 83274CD4: 4BA3524D  bl 0x82ca9f20
	ctx.lr = 0x83274CD8;
	sub_82CA9F20(ctx, base);
	// 83274CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274CE8 size=56
    let mut pc: u32 = 0x83274CE8;
    'dispatch: loop {
        match pc {
            0x83274CE8 => {
    //   block [0x83274CE8..0x83274D20)
	// 83274CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274CF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83274CF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274CFC: 386BD418  addi r3, r11, -0x2be8
	ctx.r[3].s64 = ctx.r[11].s64 + -11240;
	// 83274D00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274D04: 4AF7F055  bl 0x821f3d58
	ctx.lr = 0x83274D08;
	sub_821F3D58(ctx, base);
	// 83274D08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274D0C: 906AD2E8  stw r3, -0x2d18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11544 as u32), ctx.r[3].u32 ) };
	// 83274D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274D20 size=56
    let mut pc: u32 = 0x83274D20;
    'dispatch: loop {
        match pc {
            0x83274D20 => {
    //   block [0x83274D20..0x83274D58)
	// 83274D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274D2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274D30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274D34: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83274D38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274D3C: 4AF7F01D  bl 0x821f3d58
	ctx.lr = 0x83274D40;
	sub_821F3D58(ctx, base);
	// 83274D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274D44: 906AD2EC  stw r3, -0x2d14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11540 as u32), ctx.r[3].u32 ) };
	// 83274D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274D58 size=56
    let mut pc: u32 = 0x83274D58;
    'dispatch: loop {
        match pc {
            0x83274D58 => {
    //   block [0x83274D58..0x83274D90)
	// 83274D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274D64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274D68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274D6C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83274D70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274D74: 4AF7EFE5  bl 0x821f3d58
	ctx.lr = 0x83274D78;
	sub_821F3D58(ctx, base);
	// 83274D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274D7C: 906AD2F0  stw r3, -0x2d10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11536 as u32), ctx.r[3].u32 ) };
	// 83274D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274D90 size=56
    let mut pc: u32 = 0x83274D90;
    'dispatch: loop {
        match pc {
            0x83274D90 => {
    //   block [0x83274D90..0x83274DC8)
	// 83274D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274D98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274D9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274DA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274DA4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83274DA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274DAC: 4AF7EFAD  bl 0x821f3d58
	ctx.lr = 0x83274DB0;
	sub_821F3D58(ctx, base);
	// 83274DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274DB4: 906AD2F4  stw r3, -0x2d0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11532 as u32), ctx.r[3].u32 ) };
	// 83274DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274DC8 size=56
    let mut pc: u32 = 0x83274DC8;
    'dispatch: loop {
        match pc {
            0x83274DC8 => {
    //   block [0x83274DC8..0x83274E00)
	// 83274DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274DD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274DD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274DDC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83274DE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274DE4: 4AF7EF75  bl 0x821f3d58
	ctx.lr = 0x83274DE8;
	sub_821F3D58(ctx, base);
	// 83274DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274DEC: 906AD2F8  stw r3, -0x2d08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11528 as u32), ctx.r[3].u32 ) };
	// 83274DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274E00 size=56
    let mut pc: u32 = 0x83274E00;
    'dispatch: loop {
        match pc {
            0x83274E00 => {
    //   block [0x83274E00..0x83274E38)
	// 83274E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274E0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274E10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274E14: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83274E18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274E1C: 4AF7EF3D  bl 0x821f3d58
	ctx.lr = 0x83274E20;
	sub_821F3D58(ctx, base);
	// 83274E20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274E24: 906AD2FC  stw r3, -0x2d04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11524 as u32), ctx.r[3].u32 ) };
	// 83274E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274E38 size=56
    let mut pc: u32 = 0x83274E38;
    'dispatch: loop {
        match pc {
            0x83274E38 => {
    //   block [0x83274E38..0x83274E70)
	// 83274E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274E48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274E4C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83274E50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274E54: 4AF7EF05  bl 0x821f3d58
	ctx.lr = 0x83274E58;
	sub_821F3D58(ctx, base);
	// 83274E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274E5C: 906AD300  stw r3, -0x2d00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11520 as u32), ctx.r[3].u32 ) };
	// 83274E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274E70 size=56
    let mut pc: u32 = 0x83274E70;
    'dispatch: loop {
        match pc {
            0x83274E70 => {
    //   block [0x83274E70..0x83274EA8)
	// 83274E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274E80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274E84: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83274E88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274E8C: 4AF7EECD  bl 0x821f3d58
	ctx.lr = 0x83274E90;
	sub_821F3D58(ctx, base);
	// 83274E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274E94: 906AD304  stw r3, -0x2cfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11516 as u32), ctx.r[3].u32 ) };
	// 83274E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274EA8 size=56
    let mut pc: u32 = 0x83274EA8;
    'dispatch: loop {
        match pc {
            0x83274EA8 => {
    //   block [0x83274EA8..0x83274EE0)
	// 83274EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274EB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274EB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274EBC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83274EC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274EC4: 4AF7EE95  bl 0x821f3d58
	ctx.lr = 0x83274EC8;
	sub_821F3D58(ctx, base);
	// 83274EC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274ECC: 906AD308  stw r3, -0x2cf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11512 as u32), ctx.r[3].u32 ) };
	// 83274ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274EE0 size=56
    let mut pc: u32 = 0x83274EE0;
    'dispatch: loop {
        match pc {
            0x83274EE0 => {
    //   block [0x83274EE0..0x83274F18)
	// 83274EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274EEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274EF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274EF4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83274EF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274EFC: 4AF7EE5D  bl 0x821f3d58
	ctx.lr = 0x83274F00;
	sub_821F3D58(ctx, base);
	// 83274F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274F04: 906AD30C  stw r3, -0x2cf4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11508 as u32), ctx.r[3].u32 ) };
	// 83274F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274F18 size=56
    let mut pc: u32 = 0x83274F18;
    'dispatch: loop {
        match pc {
            0x83274F18 => {
    //   block [0x83274F18..0x83274F50)
	// 83274F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274F24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274F28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274F2C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83274F30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274F34: 4AF7EE25  bl 0x821f3d58
	ctx.lr = 0x83274F38;
	sub_821F3D58(ctx, base);
	// 83274F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274F3C: 906AD310  stw r3, -0x2cf0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11504 as u32), ctx.r[3].u32 ) };
	// 83274F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274F50 size=56
    let mut pc: u32 = 0x83274F50;
    'dispatch: loop {
        match pc {
            0x83274F50 => {
    //   block [0x83274F50..0x83274F88)
	// 83274F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274F5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274F60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274F64: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83274F68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274F6C: 4AF7EDED  bl 0x821f3d58
	ctx.lr = 0x83274F70;
	sub_821F3D58(ctx, base);
	// 83274F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274F74: 906AD314  stw r3, -0x2cec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11500 as u32), ctx.r[3].u32 ) };
	// 83274F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274F88 size=56
    let mut pc: u32 = 0x83274F88;
    'dispatch: loop {
        match pc {
            0x83274F88 => {
    //   block [0x83274F88..0x83274FC0)
	// 83274F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274F94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274F98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274F9C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83274FA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274FA4: 4AF7EDB5  bl 0x821f3d58
	ctx.lr = 0x83274FA8;
	sub_821F3D58(ctx, base);
	// 83274FA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274FAC: 906AD318  stw r3, -0x2ce8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11496 as u32), ctx.r[3].u32 ) };
	// 83274FB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274FC0 size=56
    let mut pc: u32 = 0x83274FC0;
    'dispatch: loop {
        match pc {
            0x83274FC0 => {
    //   block [0x83274FC0..0x83274FF8)
	// 83274FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274FCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274FD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274FD4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83274FD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274FDC: 4AF7ED7D  bl 0x821f3d58
	ctx.lr = 0x83274FE0;
	sub_821F3D58(ctx, base);
	// 83274FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274FE4: 906AD31C  stw r3, -0x2ce4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11492 as u32), ctx.r[3].u32 ) };
	// 83274FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274FF8 size=56
    let mut pc: u32 = 0x83274FF8;
    'dispatch: loop {
        match pc {
            0x83274FF8 => {
    //   block [0x83274FF8..0x83275030)
	// 83274FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275004: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275008: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327500C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83275010: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275014: 4AF7ED45  bl 0x821f3d58
	ctx.lr = 0x83275018;
	sub_821F3D58(ctx, base);
	// 83275018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327501C: 906AD320  stw r3, -0x2ce0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11488 as u32), ctx.r[3].u32 ) };
	// 83275020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327502C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275030 size=56
    let mut pc: u32 = 0x83275030;
    'dispatch: loop {
        match pc {
            0x83275030 => {
    //   block [0x83275030..0x83275068)
	// 83275030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327503C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275040: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275044: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83275048: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327504C: 4AF7ED0D  bl 0x821f3d58
	ctx.lr = 0x83275050;
	sub_821F3D58(ctx, base);
	// 83275050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275054: 906AD324  stw r3, -0x2cdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11484 as u32), ctx.r[3].u32 ) };
	// 83275058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327505C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275068 size=56
    let mut pc: u32 = 0x83275068;
    'dispatch: loop {
        match pc {
            0x83275068 => {
    //   block [0x83275068..0x832750A0)
	// 83275068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327506C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275074: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275078: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327507C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83275080: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275084: 4AF7ECD5  bl 0x821f3d58
	ctx.lr = 0x83275088;
	sub_821F3D58(ctx, base);
	// 83275088: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327508C: 906AD328  stw r3, -0x2cd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11480 as u32), ctx.r[3].u32 ) };
	// 83275090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327509C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832750A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832750A0 size=56
    let mut pc: u32 = 0x832750A0;
    'dispatch: loop {
        match pc {
            0x832750A0 => {
    //   block [0x832750A0..0x832750D8)
	// 832750A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832750A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832750A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832750AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832750B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832750B4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832750B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832750BC: 4AF7EC9D  bl 0x821f3d58
	ctx.lr = 0x832750C0;
	sub_821F3D58(ctx, base);
	// 832750C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832750C4: 906AD32C  stw r3, -0x2cd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11476 as u32), ctx.r[3].u32 ) };
	// 832750C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832750CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832750D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832750D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832750D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832750D8 size=56
    let mut pc: u32 = 0x832750D8;
    'dispatch: loop {
        match pc {
            0x832750D8 => {
    //   block [0x832750D8..0x83275110)
	// 832750D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832750DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832750E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832750E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832750E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832750EC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832750F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832750F4: 4AF7EC65  bl 0x821f3d58
	ctx.lr = 0x832750F8;
	sub_821F3D58(ctx, base);
	// 832750F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832750FC: 906AD330  stw r3, -0x2cd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11472 as u32), ctx.r[3].u32 ) };
	// 83275100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275110 size=56
    let mut pc: u32 = 0x83275110;
    'dispatch: loop {
        match pc {
            0x83275110 => {
    //   block [0x83275110..0x83275148)
	// 83275110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327511C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275120: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275124: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83275128: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327512C: 4AF7EC2D  bl 0x821f3d58
	ctx.lr = 0x83275130;
	sub_821F3D58(ctx, base);
	// 83275130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275134: 906AD334  stw r3, -0x2ccc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11468 as u32), ctx.r[3].u32 ) };
	// 83275138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327513C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275148 size=56
    let mut pc: u32 = 0x83275148;
    'dispatch: loop {
        match pc {
            0x83275148 => {
    //   block [0x83275148..0x83275180)
	// 83275148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327514C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275154: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275158: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327515C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83275160: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275164: 4AF7EBF5  bl 0x821f3d58
	ctx.lr = 0x83275168;
	sub_821F3D58(ctx, base);
	// 83275168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327516C: 906AD338  stw r3, -0x2cc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11464 as u32), ctx.r[3].u32 ) };
	// 83275170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327517C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275180 size=56
    let mut pc: u32 = 0x83275180;
    'dispatch: loop {
        match pc {
            0x83275180 => {
    //   block [0x83275180..0x832751B8)
	// 83275180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327518C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275190: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275194: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83275198: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327519C: 4AF7EBBD  bl 0x821f3d58
	ctx.lr = 0x832751A0;
	sub_821F3D58(ctx, base);
	// 832751A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832751A4: 906AD33C  stw r3, -0x2cc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11460 as u32), ctx.r[3].u32 ) };
	// 832751A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832751AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832751B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832751B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832751B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832751B8 size=64
    let mut pc: u32 = 0x832751B8;
    'dispatch: loop {
        match pc {
            0x832751B8 => {
    //   block [0x832751B8..0x832751F8)
	// 832751B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832751BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832751C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832751C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832751C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832751CC: 388B7230  addi r4, r11, 0x7230
	ctx.r[4].s64 = ctx.r[11].s64 + 29232;
	// 832751D0: 386AD340  addi r3, r10, -0x2cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -11456;
	// 832751D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832751D8: 4AFB7CF9  bl 0x8222ced0
	ctx.lr = 0x832751DC;
	sub_8222CED0(ctx, base);
	// 832751DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832751E0: 3869F940  addi r3, r9, -0x6c0
	ctx.r[3].s64 = ctx.r[9].s64 + -1728;
	// 832751E4: 4BA34D3D  bl 0x82ca9f20
	ctx.lr = 0x832751E8;
	sub_82CA9F20(ctx, base);
	// 832751E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832751EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832751F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832751F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832751F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832751F8 size=64
    let mut pc: u32 = 0x832751F8;
    'dispatch: loop {
        match pc {
            0x832751F8 => {
    //   block [0x832751F8..0x83275238)
	// 832751F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832751FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275204: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327520C: 388BC06C  addi r4, r11, -0x3f94
	ctx.r[4].s64 = ctx.r[11].s64 + -16276;
	// 83275210: 386AD344  addi r3, r10, -0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11452;
	// 83275214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275218: 4AFB7CB9  bl 0x8222ced0
	ctx.lr = 0x8327521C;
	sub_8222CED0(ctx, base);
	// 8327521C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275220: 3869F950  addi r3, r9, -0x6b0
	ctx.r[3].s64 = ctx.r[9].s64 + -1712;
	// 83275224: 4BA34CFD  bl 0x82ca9f20
	ctx.lr = 0x83275228;
	sub_82CA9F20(ctx, base);
	// 83275228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327522C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275238 size=64
    let mut pc: u32 = 0x83275238;
    'dispatch: loop {
        match pc {
            0x83275238 => {
    //   block [0x83275238..0x83275278)
	// 83275238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275244: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327524C: 388BC074  addi r4, r11, -0x3f8c
	ctx.r[4].s64 = ctx.r[11].s64 + -16268;
	// 83275250: 386AD348  addi r3, r10, -0x2cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -11448;
	// 83275254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275258: 4AFB7C79  bl 0x8222ced0
	ctx.lr = 0x8327525C;
	sub_8222CED0(ctx, base);
	// 8327525C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275260: 3869F960  addi r3, r9, -0x6a0
	ctx.r[3].s64 = ctx.r[9].s64 + -1696;
	// 83275264: 4BA34CBD  bl 0x82ca9f20
	ctx.lr = 0x83275268;
	sub_82CA9F20(ctx, base);
	// 83275268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327526C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275278 size=64
    let mut pc: u32 = 0x83275278;
    'dispatch: loop {
        match pc {
            0x83275278 => {
    //   block [0x83275278..0x832752B8)
	// 83275278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327527C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275284: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327528C: 388BC07C  addi r4, r11, -0x3f84
	ctx.r[4].s64 = ctx.r[11].s64 + -16260;
	// 83275290: 386AD34C  addi r3, r10, -0x2cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -11444;
	// 83275294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275298: 4AFB7C39  bl 0x8222ced0
	ctx.lr = 0x8327529C;
	sub_8222CED0(ctx, base);
	// 8327529C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832752A0: 3869F970  addi r3, r9, -0x690
	ctx.r[3].s64 = ctx.r[9].s64 + -1680;
	// 832752A4: 4BA34C7D  bl 0x82ca9f20
	ctx.lr = 0x832752A8;
	sub_82CA9F20(ctx, base);
	// 832752A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832752AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832752B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832752B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832752B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832752B8 size=64
    let mut pc: u32 = 0x832752B8;
    'dispatch: loop {
        match pc {
            0x832752B8 => {
    //   block [0x832752B8..0x832752F8)
	// 832752B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832752BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832752C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832752C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832752C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832752CC: 388B0420  addi r4, r11, 0x420
	ctx.r[4].s64 = ctx.r[11].s64 + 1056;
	// 832752D0: 386AD350  addi r3, r10, -0x2cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11440;
	// 832752D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832752D8: 4AFB7BF9  bl 0x8222ced0
	ctx.lr = 0x832752DC;
	sub_8222CED0(ctx, base);
	// 832752DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832752E0: 3869F980  addi r3, r9, -0x680
	ctx.r[3].s64 = ctx.r[9].s64 + -1664;
	// 832752E4: 4BA34C3D  bl 0x82ca9f20
	ctx.lr = 0x832752E8;
	sub_82CA9F20(ctx, base);
	// 832752E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832752EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832752F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832752F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832752F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832752F8 size=64
    let mut pc: u32 = 0x832752F8;
    'dispatch: loop {
        match pc {
            0x832752F8 => {
    //   block [0x832752F8..0x83275338)
	// 832752F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832752FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275304: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275308: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327530C: 388B9C5C  addi r4, r11, -0x63a4
	ctx.r[4].s64 = ctx.r[11].s64 + -25508;
	// 83275310: 386AD354  addi r3, r10, -0x2cac
	ctx.r[3].s64 = ctx.r[10].s64 + -11436;
	// 83275314: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275318: 4AFB7BB9  bl 0x8222ced0
	ctx.lr = 0x8327531C;
	sub_8222CED0(ctx, base);
	// 8327531C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275320: 3869F990  addi r3, r9, -0x670
	ctx.r[3].s64 = ctx.r[9].s64 + -1648;
	// 83275324: 4BA34BFD  bl 0x82ca9f20
	ctx.lr = 0x83275328;
	sub_82CA9F20(ctx, base);
	// 83275328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327532C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275338 size=56
    let mut pc: u32 = 0x83275338;
    'dispatch: loop {
        match pc {
            0x83275338 => {
    //   block [0x83275338..0x83275370)
	// 83275338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327533C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327534C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83275350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275354: 4AF7EA05  bl 0x821f3d58
	ctx.lr = 0x83275358;
	sub_821F3D58(ctx, base);
	// 83275358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327535C: 906AD358  stw r3, -0x2ca8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11432 as u32), ctx.r[3].u32 ) };
	// 83275360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327536C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275370 size=56
    let mut pc: u32 = 0x83275370;
    'dispatch: loop {
        match pc {
            0x83275370 => {
    //   block [0x83275370..0x832753A8)
	// 83275370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327537C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275384: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83275388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327538C: 4AF7E9CD  bl 0x821f3d58
	ctx.lr = 0x83275390;
	sub_821F3D58(ctx, base);
	// 83275390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275394: 906AD35C  stw r3, -0x2ca4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11428 as u32), ctx.r[3].u32 ) };
	// 83275398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327539C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832753A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832753A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832753A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832753A8 size=56
    let mut pc: u32 = 0x832753A8;
    'dispatch: loop {
        match pc {
            0x832753A8 => {
    //   block [0x832753A8..0x832753E0)
	// 832753A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832753AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832753B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832753B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832753B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832753BC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832753C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832753C4: 4AF7E995  bl 0x821f3d58
	ctx.lr = 0x832753C8;
	sub_821F3D58(ctx, base);
	// 832753C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832753CC: 906AD360  stw r3, -0x2ca0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11424 as u32), ctx.r[3].u32 ) };
	// 832753D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832753D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832753D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832753DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832753E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832753E0 size=56
    let mut pc: u32 = 0x832753E0;
    'dispatch: loop {
        match pc {
            0x832753E0 => {
    //   block [0x832753E0..0x83275418)
	// 832753E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832753E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832753E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832753EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832753F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832753F4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832753F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832753FC: 4AF7E95D  bl 0x821f3d58
	ctx.lr = 0x83275400;
	sub_821F3D58(ctx, base);
	// 83275400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275404: 906AD364  stw r3, -0x2c9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11420 as u32), ctx.r[3].u32 ) };
	// 83275408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275418 size=56
    let mut pc: u32 = 0x83275418;
    'dispatch: loop {
        match pc {
            0x83275418 => {
    //   block [0x83275418..0x83275450)
	// 83275418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275424: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275428: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327542C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83275430: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275434: 4AF7E925  bl 0x821f3d58
	ctx.lr = 0x83275438;
	sub_821F3D58(ctx, base);
	// 83275438: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327543C: 906AD368  stw r3, -0x2c98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11416 as u32), ctx.r[3].u32 ) };
	// 83275440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327544C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275450 size=56
    let mut pc: u32 = 0x83275450;
    'dispatch: loop {
        match pc {
            0x83275450 => {
    //   block [0x83275450..0x83275488)
	// 83275450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327545C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275460: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275464: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83275468: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327546C: 4AF7E8ED  bl 0x821f3d58
	ctx.lr = 0x83275470;
	sub_821F3D58(ctx, base);
	// 83275470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275474: 906AD36C  stw r3, -0x2c94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11412 as u32), ctx.r[3].u32 ) };
	// 83275478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327547C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275488 size=56
    let mut pc: u32 = 0x83275488;
    'dispatch: loop {
        match pc {
            0x83275488 => {
    //   block [0x83275488..0x832754C0)
	// 83275488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327548C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275498: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327549C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832754A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832754A4: 4AF7E8B5  bl 0x821f3d58
	ctx.lr = 0x832754A8;
	sub_821F3D58(ctx, base);
	// 832754A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832754AC: 906AD370  stw r3, -0x2c90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11408 as u32), ctx.r[3].u32 ) };
	// 832754B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832754B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832754B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832754BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832754C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832754C0 size=56
    let mut pc: u32 = 0x832754C0;
    'dispatch: loop {
        match pc {
            0x832754C0 => {
    //   block [0x832754C0..0x832754F8)
	// 832754C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832754C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832754C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832754CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832754D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832754D4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832754D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832754DC: 4AF7E87D  bl 0x821f3d58
	ctx.lr = 0x832754E0;
	sub_821F3D58(ctx, base);
	// 832754E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832754E4: 906AD374  stw r3, -0x2c8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11404 as u32), ctx.r[3].u32 ) };
	// 832754E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832754EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832754F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832754F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832754F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832754F8 size=56
    let mut pc: u32 = 0x832754F8;
    'dispatch: loop {
        match pc {
            0x832754F8 => {
    //   block [0x832754F8..0x83275530)
	// 832754F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832754FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275504: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275508: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327550C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83275510: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275514: 4AF7E845  bl 0x821f3d58
	ctx.lr = 0x83275518;
	sub_821F3D58(ctx, base);
	// 83275518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327551C: 906AD378  stw r3, -0x2c88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11400 as u32), ctx.r[3].u32 ) };
	// 83275520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327552C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275530 size=56
    let mut pc: u32 = 0x83275530;
    'dispatch: loop {
        match pc {
            0x83275530 => {
    //   block [0x83275530..0x83275568)
	// 83275530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327553C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275540: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275544: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83275548: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327554C: 4AF7E80D  bl 0x821f3d58
	ctx.lr = 0x83275550;
	sub_821F3D58(ctx, base);
	// 83275550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275554: 906AD37C  stw r3, -0x2c84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11396 as u32), ctx.r[3].u32 ) };
	// 83275558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275568 size=56
    let mut pc: u32 = 0x83275568;
    'dispatch: loop {
        match pc {
            0x83275568 => {
    //   block [0x83275568..0x832755A0)
	// 83275568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275574: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275578: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327557C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83275580: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275584: 4AF7E7D5  bl 0x821f3d58
	ctx.lr = 0x83275588;
	sub_821F3D58(ctx, base);
	// 83275588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327558C: 906AD380  stw r3, -0x2c80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11392 as u32), ctx.r[3].u32 ) };
	// 83275590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327559C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832755A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832755A0 size=56
    let mut pc: u32 = 0x832755A0;
    'dispatch: loop {
        match pc {
            0x832755A0 => {
    //   block [0x832755A0..0x832755D8)
	// 832755A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832755A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832755A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832755AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832755B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832755B4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832755B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832755BC: 4AF7E79D  bl 0x821f3d58
	ctx.lr = 0x832755C0;
	sub_821F3D58(ctx, base);
	// 832755C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832755C4: 906AD384  stw r3, -0x2c7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11388 as u32), ctx.r[3].u32 ) };
	// 832755C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832755CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832755D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832755D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832755D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832755D8 size=56
    let mut pc: u32 = 0x832755D8;
    'dispatch: loop {
        match pc {
            0x832755D8 => {
    //   block [0x832755D8..0x83275610)
	// 832755D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832755DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832755E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832755E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832755E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832755EC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832755F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832755F4: 4AF7E765  bl 0x821f3d58
	ctx.lr = 0x832755F8;
	sub_821F3D58(ctx, base);
	// 832755F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832755FC: 906AD388  stw r3, -0x2c78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11384 as u32), ctx.r[3].u32 ) };
	// 83275600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327560C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275610 size=56
    let mut pc: u32 = 0x83275610;
    'dispatch: loop {
        match pc {
            0x83275610 => {
    //   block [0x83275610..0x83275648)
	// 83275610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327561C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275624: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83275628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327562C: 4AF7E72D  bl 0x821f3d58
	ctx.lr = 0x83275630;
	sub_821F3D58(ctx, base);
	// 83275630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275634: 906AD38C  stw r3, -0x2c74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11380 as u32), ctx.r[3].u32 ) };
	// 83275638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327563C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275648 size=56
    let mut pc: u32 = 0x83275648;
    'dispatch: loop {
        match pc {
            0x83275648 => {
    //   block [0x83275648..0x83275680)
	// 83275648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327564C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275654: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275658: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327565C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83275660: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275664: 4AF7E6F5  bl 0x821f3d58
	ctx.lr = 0x83275668;
	sub_821F3D58(ctx, base);
	// 83275668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327566C: 906AD390  stw r3, -0x2c70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11376 as u32), ctx.r[3].u32 ) };
	// 83275670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275680 size=56
    let mut pc: u32 = 0x83275680;
    'dispatch: loop {
        match pc {
            0x83275680 => {
    //   block [0x83275680..0x832756B8)
	// 83275680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327568C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275690: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275694: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83275698: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327569C: 4AF7E6BD  bl 0x821f3d58
	ctx.lr = 0x832756A0;
	sub_821F3D58(ctx, base);
	// 832756A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832756A4: 906AD394  stw r3, -0x2c6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11372 as u32), ctx.r[3].u32 ) };
	// 832756A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832756AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832756B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832756B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832756B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832756B8 size=56
    let mut pc: u32 = 0x832756B8;
    'dispatch: loop {
        match pc {
            0x832756B8 => {
    //   block [0x832756B8..0x832756F0)
	// 832756B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832756BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832756C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832756C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832756C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832756CC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832756D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832756D4: 4AF7E685  bl 0x821f3d58
	ctx.lr = 0x832756D8;
	sub_821F3D58(ctx, base);
	// 832756D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832756DC: 906AD398  stw r3, -0x2c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11368 as u32), ctx.r[3].u32 ) };
	// 832756E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832756E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832756E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832756EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832756F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832756F0 size=56
    let mut pc: u32 = 0x832756F0;
    'dispatch: loop {
        match pc {
            0x832756F0 => {
    //   block [0x832756F0..0x83275728)
	// 832756F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832756F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832756F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832756FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275700: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275704: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83275708: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327570C: 4AF7E64D  bl 0x821f3d58
	ctx.lr = 0x83275710;
	sub_821F3D58(ctx, base);
	// 83275710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275714: 906AD39C  stw r3, -0x2c64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11364 as u32), ctx.r[3].u32 ) };
	// 83275718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275728 size=56
    let mut pc: u32 = 0x83275728;
    'dispatch: loop {
        match pc {
            0x83275728 => {
    //   block [0x83275728..0x83275760)
	// 83275728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275734: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275738: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327573C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83275740: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275744: 4AF7E615  bl 0x821f3d58
	ctx.lr = 0x83275748;
	sub_821F3D58(ctx, base);
	// 83275748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327574C: 906AD3A0  stw r3, -0x2c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11360 as u32), ctx.r[3].u32 ) };
	// 83275750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275760 size=56
    let mut pc: u32 = 0x83275760;
    'dispatch: loop {
        match pc {
            0x83275760 => {
    //   block [0x83275760..0x83275798)
	// 83275760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327576C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275770: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275774: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83275778: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327577C: 4AF7E5DD  bl 0x821f3d58
	ctx.lr = 0x83275780;
	sub_821F3D58(ctx, base);
	// 83275780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275784: 906AD3A4  stw r3, -0x2c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11356 as u32), ctx.r[3].u32 ) };
	// 83275788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327578C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275798 size=56
    let mut pc: u32 = 0x83275798;
    'dispatch: loop {
        match pc {
            0x83275798 => {
    //   block [0x83275798..0x832757D0)
	// 83275798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832757A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832757A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832757A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832757AC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832757B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832757B4: 4AF7E5A5  bl 0x821f3d58
	ctx.lr = 0x832757B8;
	sub_821F3D58(ctx, base);
	// 832757B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832757BC: 906AD3A8  stw r3, -0x2c58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11352 as u32), ctx.r[3].u32 ) };
	// 832757C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832757C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832757C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832757CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832757D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832757D0 size=64
    let mut pc: u32 = 0x832757D0;
    'dispatch: loop {
        match pc {
            0x832757D0 => {
    //   block [0x832757D0..0x83275810)
	// 832757D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832757D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832757D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832757DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832757E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832757E4: 388B07C0  addi r4, r11, 0x7c0
	ctx.r[4].s64 = ctx.r[11].s64 + 1984;
	// 832757E8: 386AD3AC  addi r3, r10, -0x2c54
	ctx.r[3].s64 = ctx.r[10].s64 + -11348;
	// 832757EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832757F0: 4AFB76E1  bl 0x8222ced0
	ctx.lr = 0x832757F4;
	sub_8222CED0(ctx, base);
	// 832757F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832757F8: 3869F9A0  addi r3, r9, -0x660
	ctx.r[3].s64 = ctx.r[9].s64 + -1632;
	// 832757FC: 4BA34725  bl 0x82ca9f20
	ctx.lr = 0x83275800;
	sub_82CA9F20(ctx, base);
	// 83275800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327580C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275810 size=64
    let mut pc: u32 = 0x83275810;
    'dispatch: loop {
        match pc {
            0x83275810 => {
    //   block [0x83275810..0x83275850)
	// 83275810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327581C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275824: 388BDCDC  addi r4, r11, -0x2324
	ctx.r[4].s64 = ctx.r[11].s64 + -8996;
	// 83275828: 386AD3B0  addi r3, r10, -0x2c50
	ctx.r[3].s64 = ctx.r[10].s64 + -11344;
	// 8327582C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275830: 4AFB76A1  bl 0x8222ced0
	ctx.lr = 0x83275834;
	sub_8222CED0(ctx, base);
	// 83275834: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275838: 3869F9B0  addi r3, r9, -0x650
	ctx.r[3].s64 = ctx.r[9].s64 + -1616;
	// 8327583C: 4BA346E5  bl 0x82ca9f20
	ctx.lr = 0x83275840;
	sub_82CA9F20(ctx, base);
	// 83275840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275850 size=64
    let mut pc: u32 = 0x83275850;
    'dispatch: loop {
        match pc {
            0x83275850 => {
    //   block [0x83275850..0x83275890)
	// 83275850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327585C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275864: 388BDCF4  addi r4, r11, -0x230c
	ctx.r[4].s64 = ctx.r[11].s64 + -8972;
	// 83275868: 386AD3B4  addi r3, r10, -0x2c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11340;
	// 8327586C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275870: 4AFB7661  bl 0x8222ced0
	ctx.lr = 0x83275874;
	sub_8222CED0(ctx, base);
	// 83275874: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275878: 3869F9C0  addi r3, r9, -0x640
	ctx.r[3].s64 = ctx.r[9].s64 + -1600;
	// 8327587C: 4BA346A5  bl 0x82ca9f20
	ctx.lr = 0x83275880;
	sub_82CA9F20(ctx, base);
	// 83275880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327588C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275890 size=64
    let mut pc: u32 = 0x83275890;
    'dispatch: loop {
        match pc {
            0x83275890 => {
    //   block [0x83275890..0x832758D0)
	// 83275890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327589C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832758A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832758A4: 388BDD08  addi r4, r11, -0x22f8
	ctx.r[4].s64 = ctx.r[11].s64 + -8952;
	// 832758A8: 386AD3B8  addi r3, r10, -0x2c48
	ctx.r[3].s64 = ctx.r[10].s64 + -11336;
	// 832758AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832758B0: 4AFB7621  bl 0x8222ced0
	ctx.lr = 0x832758B4;
	sub_8222CED0(ctx, base);
	// 832758B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832758B8: 3869F9D0  addi r3, r9, -0x630
	ctx.r[3].s64 = ctx.r[9].s64 + -1584;
	// 832758BC: 4BA34665  bl 0x82ca9f20
	ctx.lr = 0x832758C0;
	sub_82CA9F20(ctx, base);
	// 832758C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832758C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832758C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832758CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832758D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832758D0 size=64
    let mut pc: u32 = 0x832758D0;
    'dispatch: loop {
        match pc {
            0x832758D0 => {
    //   block [0x832758D0..0x83275910)
	// 832758D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832758D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832758D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832758DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832758E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832758E4: 388BDD1C  addi r4, r11, -0x22e4
	ctx.r[4].s64 = ctx.r[11].s64 + -8932;
	// 832758E8: 386AD3BC  addi r3, r10, -0x2c44
	ctx.r[3].s64 = ctx.r[10].s64 + -11332;
	// 832758EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832758F0: 4AFB75E1  bl 0x8222ced0
	ctx.lr = 0x832758F4;
	sub_8222CED0(ctx, base);
	// 832758F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832758F8: 3869F9E0  addi r3, r9, -0x620
	ctx.r[3].s64 = ctx.r[9].s64 + -1568;
	// 832758FC: 4BA34625  bl 0x82ca9f20
	ctx.lr = 0x83275900;
	sub_82CA9F20(ctx, base);
	// 83275900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327590C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275910 size=64
    let mut pc: u32 = 0x83275910;
    'dispatch: loop {
        match pc {
            0x83275910 => {
    //   block [0x83275910..0x83275950)
	// 83275910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327591C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275924: 388BDD30  addi r4, r11, -0x22d0
	ctx.r[4].s64 = ctx.r[11].s64 + -8912;
	// 83275928: 386AD3C0  addi r3, r10, -0x2c40
	ctx.r[3].s64 = ctx.r[10].s64 + -11328;
	// 8327592C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275930: 4AFB75A1  bl 0x8222ced0
	ctx.lr = 0x83275934;
	sub_8222CED0(ctx, base);
	// 83275934: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275938: 3869F9F0  addi r3, r9, -0x610
	ctx.r[3].s64 = ctx.r[9].s64 + -1552;
	// 8327593C: 4BA345E5  bl 0x82ca9f20
	ctx.lr = 0x83275940;
	sub_82CA9F20(ctx, base);
	// 83275940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327594C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275950 size=64
    let mut pc: u32 = 0x83275950;
    'dispatch: loop {
        match pc {
            0x83275950 => {
    //   block [0x83275950..0x83275990)
	// 83275950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327595C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275964: 388BDD44  addi r4, r11, -0x22bc
	ctx.r[4].s64 = ctx.r[11].s64 + -8892;
	// 83275968: 386AD3C4  addi r3, r10, -0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11324;
	// 8327596C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275970: 4AFB7561  bl 0x8222ced0
	ctx.lr = 0x83275974;
	sub_8222CED0(ctx, base);
	// 83275974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275978: 3869FA00  addi r3, r9, -0x600
	ctx.r[3].s64 = ctx.r[9].s64 + -1536;
	// 8327597C: 4BA345A5  bl 0x82ca9f20
	ctx.lr = 0x83275980;
	sub_82CA9F20(ctx, base);
	// 83275980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327598C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275990 size=64
    let mut pc: u32 = 0x83275990;
    'dispatch: loop {
        match pc {
            0x83275990 => {
    //   block [0x83275990..0x832759D0)
	// 83275990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327599C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832759A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832759A4: 388BDD58  addi r4, r11, -0x22a8
	ctx.r[4].s64 = ctx.r[11].s64 + -8872;
	// 832759A8: 386AD3C8  addi r3, r10, -0x2c38
	ctx.r[3].s64 = ctx.r[10].s64 + -11320;
	// 832759AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832759B0: 4AFB7521  bl 0x8222ced0
	ctx.lr = 0x832759B4;
	sub_8222CED0(ctx, base);
	// 832759B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832759B8: 3869FA10  addi r3, r9, -0x5f0
	ctx.r[3].s64 = ctx.r[9].s64 + -1520;
	// 832759BC: 4BA34565  bl 0x82ca9f20
	ctx.lr = 0x832759C0;
	sub_82CA9F20(ctx, base);
	// 832759C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832759C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832759C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832759CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832759D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832759D0 size=64
    let mut pc: u32 = 0x832759D0;
    'dispatch: loop {
        match pc {
            0x832759D0 => {
    //   block [0x832759D0..0x83275A10)
	// 832759D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832759D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832759D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832759DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832759E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832759E4: 388BDD70  addi r4, r11, -0x2290
	ctx.r[4].s64 = ctx.r[11].s64 + -8848;
	// 832759E8: 386AD3CC  addi r3, r10, -0x2c34
	ctx.r[3].s64 = ctx.r[10].s64 + -11316;
	// 832759EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832759F0: 4AFB74E1  bl 0x8222ced0
	ctx.lr = 0x832759F4;
	sub_8222CED0(ctx, base);
	// 832759F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832759F8: 3869FA20  addi r3, r9, -0x5e0
	ctx.r[3].s64 = ctx.r[9].s64 + -1504;
	// 832759FC: 4BA34525  bl 0x82ca9f20
	ctx.lr = 0x83275A00;
	sub_82CA9F20(ctx, base);
	// 83275A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275A10 size=56
    let mut pc: u32 = 0x83275A10;
    'dispatch: loop {
        match pc {
            0x83275A10 => {
    //   block [0x83275A10..0x83275A48)
	// 83275A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275A18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275A1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275A20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275A24: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83275A28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275A2C: 4AF7E32D  bl 0x821f3d58
	ctx.lr = 0x83275A30;
	sub_821F3D58(ctx, base);
	// 83275A30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275A34: 906AD3D0  stw r3, -0x2c30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11312 as u32), ctx.r[3].u32 ) };
	// 83275A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275A48 size=56
    let mut pc: u32 = 0x83275A48;
    'dispatch: loop {
        match pc {
            0x83275A48 => {
    //   block [0x83275A48..0x83275A80)
	// 83275A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275A54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275A58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275A5C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83275A60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275A64: 4AF7E2F5  bl 0x821f3d58
	ctx.lr = 0x83275A68;
	sub_821F3D58(ctx, base);
	// 83275A68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275A6C: 906AD3D4  stw r3, -0x2c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11308 as u32), ctx.r[3].u32 ) };
	// 83275A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275A80 size=56
    let mut pc: u32 = 0x83275A80;
    'dispatch: loop {
        match pc {
            0x83275A80 => {
    //   block [0x83275A80..0x83275AB8)
	// 83275A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275A8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275A90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275A94: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83275A98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275A9C: 4AF7E2BD  bl 0x821f3d58
	ctx.lr = 0x83275AA0;
	sub_821F3D58(ctx, base);
	// 83275AA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275AA4: 906AD3D8  stw r3, -0x2c28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11304 as u32), ctx.r[3].u32 ) };
	// 83275AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275AB8 size=56
    let mut pc: u32 = 0x83275AB8;
    'dispatch: loop {
        match pc {
            0x83275AB8 => {
    //   block [0x83275AB8..0x83275AF0)
	// 83275AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275AC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275ACC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83275AD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275AD4: 4AF7E285  bl 0x821f3d58
	ctx.lr = 0x83275AD8;
	sub_821F3D58(ctx, base);
	// 83275AD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275ADC: 906AD3DC  stw r3, -0x2c24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11300 as u32), ctx.r[3].u32 ) };
	// 83275AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275AF0 size=56
    let mut pc: u32 = 0x83275AF0;
    'dispatch: loop {
        match pc {
            0x83275AF0 => {
    //   block [0x83275AF0..0x83275B28)
	// 83275AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275AFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275B00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275B04: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83275B08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275B0C: 4AF7E24D  bl 0x821f3d58
	ctx.lr = 0x83275B10;
	sub_821F3D58(ctx, base);
	// 83275B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275B14: 906AD3E0  stw r3, -0x2c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11296 as u32), ctx.r[3].u32 ) };
	// 83275B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275B28 size=56
    let mut pc: u32 = 0x83275B28;
    'dispatch: loop {
        match pc {
            0x83275B28 => {
    //   block [0x83275B28..0x83275B60)
	// 83275B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275B34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275B38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275B3C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83275B40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275B44: 4AF7E215  bl 0x821f3d58
	ctx.lr = 0x83275B48;
	sub_821F3D58(ctx, base);
	// 83275B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275B4C: 906AD3E4  stw r3, -0x2c1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11292 as u32), ctx.r[3].u32 ) };
	// 83275B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275B60 size=56
    let mut pc: u32 = 0x83275B60;
    'dispatch: loop {
        match pc {
            0x83275B60 => {
    //   block [0x83275B60..0x83275B98)
	// 83275B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275B6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275B70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275B74: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83275B78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275B7C: 4AF7E1DD  bl 0x821f3d58
	ctx.lr = 0x83275B80;
	sub_821F3D58(ctx, base);
	// 83275B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275B84: 906AD3E8  stw r3, -0x2c18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11288 as u32), ctx.r[3].u32 ) };
	// 83275B88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275B98 size=56
    let mut pc: u32 = 0x83275B98;
    'dispatch: loop {
        match pc {
            0x83275B98 => {
    //   block [0x83275B98..0x83275BD0)
	// 83275B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275BA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275BA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275BAC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83275BB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275BB4: 4AF7E1A5  bl 0x821f3d58
	ctx.lr = 0x83275BB8;
	sub_821F3D58(ctx, base);
	// 83275BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275BBC: 906AD3EC  stw r3, -0x2c14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11284 as u32), ctx.r[3].u32 ) };
	// 83275BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275BD0 size=56
    let mut pc: u32 = 0x83275BD0;
    'dispatch: loop {
        match pc {
            0x83275BD0 => {
    //   block [0x83275BD0..0x83275C08)
	// 83275BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275BDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275BE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275BE4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83275BE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275BEC: 4AF7E16D  bl 0x821f3d58
	ctx.lr = 0x83275BF0;
	sub_821F3D58(ctx, base);
	// 83275BF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275BF4: 906AD3F0  stw r3, -0x2c10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11280 as u32), ctx.r[3].u32 ) };
	// 83275BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275C08 size=56
    let mut pc: u32 = 0x83275C08;
    'dispatch: loop {
        match pc {
            0x83275C08 => {
    //   block [0x83275C08..0x83275C40)
	// 83275C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275C14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275C18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275C1C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83275C20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275C24: 4AF7E135  bl 0x821f3d58
	ctx.lr = 0x83275C28;
	sub_821F3D58(ctx, base);
	// 83275C28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275C2C: 906AD3F4  stw r3, -0x2c0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11276 as u32), ctx.r[3].u32 ) };
	// 83275C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275C40 size=56
    let mut pc: u32 = 0x83275C40;
    'dispatch: loop {
        match pc {
            0x83275C40 => {
    //   block [0x83275C40..0x83275C78)
	// 83275C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275C4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275C50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275C54: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83275C58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275C5C: 4AF7E0FD  bl 0x821f3d58
	ctx.lr = 0x83275C60;
	sub_821F3D58(ctx, base);
	// 83275C60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275C64: 906AD3F8  stw r3, -0x2c08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11272 as u32), ctx.r[3].u32 ) };
	// 83275C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275C78 size=56
    let mut pc: u32 = 0x83275C78;
    'dispatch: loop {
        match pc {
            0x83275C78 => {
    //   block [0x83275C78..0x83275CB0)
	// 83275C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275C80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275C84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275C88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275C8C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83275C90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275C94: 4AF7E0C5  bl 0x821f3d58
	ctx.lr = 0x83275C98;
	sub_821F3D58(ctx, base);
	// 83275C98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275C9C: 906AD3FC  stw r3, -0x2c04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11268 as u32), ctx.r[3].u32 ) };
	// 83275CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275CB0 size=56
    let mut pc: u32 = 0x83275CB0;
    'dispatch: loop {
        match pc {
            0x83275CB0 => {
    //   block [0x83275CB0..0x83275CE8)
	// 83275CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275CBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275CC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275CC4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83275CC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275CCC: 4AF7E08D  bl 0x821f3d58
	ctx.lr = 0x83275CD0;
	sub_821F3D58(ctx, base);
	// 83275CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275CD4: 906AD400  stw r3, -0x2c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11264 as u32), ctx.r[3].u32 ) };
	// 83275CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275CE8 size=56
    let mut pc: u32 = 0x83275CE8;
    'dispatch: loop {
        match pc {
            0x83275CE8 => {
    //   block [0x83275CE8..0x83275D20)
	// 83275CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275CF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275CF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275CFC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83275D00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275D04: 4AF7E055  bl 0x821f3d58
	ctx.lr = 0x83275D08;
	sub_821F3D58(ctx, base);
	// 83275D08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275D0C: 906AD404  stw r3, -0x2bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11260 as u32), ctx.r[3].u32 ) };
	// 83275D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275D20 size=56
    let mut pc: u32 = 0x83275D20;
    'dispatch: loop {
        match pc {
            0x83275D20 => {
    //   block [0x83275D20..0x83275D58)
	// 83275D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275D2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275D30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275D34: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83275D38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275D3C: 4AF7E01D  bl 0x821f3d58
	ctx.lr = 0x83275D40;
	sub_821F3D58(ctx, base);
	// 83275D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275D44: 906AD408  stw r3, -0x2bf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11256 as u32), ctx.r[3].u32 ) };
	// 83275D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275D58 size=56
    let mut pc: u32 = 0x83275D58;
    'dispatch: loop {
        match pc {
            0x83275D58 => {
    //   block [0x83275D58..0x83275D90)
	// 83275D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275D64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275D68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275D6C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83275D70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275D74: 4AF7DFE5  bl 0x821f3d58
	ctx.lr = 0x83275D78;
	sub_821F3D58(ctx, base);
	// 83275D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275D7C: 906AD40C  stw r3, -0x2bf4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11252 as u32), ctx.r[3].u32 ) };
	// 83275D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275D90 size=56
    let mut pc: u32 = 0x83275D90;
    'dispatch: loop {
        match pc {
            0x83275D90 => {
    //   block [0x83275D90..0x83275DC8)
	// 83275D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275D98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275D9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275DA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275DA4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83275DA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275DAC: 4AF7DFAD  bl 0x821f3d58
	ctx.lr = 0x83275DB0;
	sub_821F3D58(ctx, base);
	// 83275DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275DB4: 906AD410  stw r3, -0x2bf0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11248 as u32), ctx.r[3].u32 ) };
	// 83275DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275DC8 size=56
    let mut pc: u32 = 0x83275DC8;
    'dispatch: loop {
        match pc {
            0x83275DC8 => {
    //   block [0x83275DC8..0x83275E00)
	// 83275DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275DD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275DD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275DDC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83275DE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275DE4: 4AF7DF75  bl 0x821f3d58
	ctx.lr = 0x83275DE8;
	sub_821F3D58(ctx, base);
	// 83275DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275DEC: 906AD414  stw r3, -0x2bec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11244 as u32), ctx.r[3].u32 ) };
	// 83275DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275E00 size=56
    let mut pc: u32 = 0x83275E00;
    'dispatch: loop {
        match pc {
            0x83275E00 => {
    //   block [0x83275E00..0x83275E38)
	// 83275E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275E0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275E10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275E14: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83275E18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275E1C: 4AF7DF3D  bl 0x821f3d58
	ctx.lr = 0x83275E20;
	sub_821F3D58(ctx, base);
	// 83275E20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275E24: 906AD418  stw r3, -0x2be8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11240 as u32), ctx.r[3].u32 ) };
	// 83275E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275E38 size=56
    let mut pc: u32 = 0x83275E38;
    'dispatch: loop {
        match pc {
            0x83275E38 => {
    //   block [0x83275E38..0x83275E70)
	// 83275E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275E48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275E4C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83275E50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275E54: 4AF7DF05  bl 0x821f3d58
	ctx.lr = 0x83275E58;
	sub_821F3D58(ctx, base);
	// 83275E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275E5C: 906AD41C  stw r3, -0x2be4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11236 as u32), ctx.r[3].u32 ) };
	// 83275E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275E70 size=56
    let mut pc: u32 = 0x83275E70;
    'dispatch: loop {
        match pc {
            0x83275E70 => {
    //   block [0x83275E70..0x83275EA8)
	// 83275E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275E80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275E84: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83275E88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275E8C: 4AF7DECD  bl 0x821f3d58
	ctx.lr = 0x83275E90;
	sub_821F3D58(ctx, base);
	// 83275E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275E94: 906AD420  stw r3, -0x2be0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11232 as u32), ctx.r[3].u32 ) };
	// 83275E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275EA8 size=64
    let mut pc: u32 = 0x83275EA8;
    'dispatch: loop {
        match pc {
            0x83275EA8 => {
    //   block [0x83275EA8..0x83275EE8)
	// 83275EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275EB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275EB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275EBC: 388BC244  addi r4, r11, -0x3dbc
	ctx.r[4].s64 = ctx.r[11].s64 + -15804;
	// 83275EC0: 386AD424  addi r3, r10, -0x2bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -11228;
	// 83275EC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275EC8: 4AFB7009  bl 0x8222ced0
	ctx.lr = 0x83275ECC;
	sub_8222CED0(ctx, base);
	// 83275ECC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275ED0: 3869FA30  addi r3, r9, -0x5d0
	ctx.r[3].s64 = ctx.r[9].s64 + -1488;
	// 83275ED4: 4BA3404D  bl 0x82ca9f20
	ctx.lr = 0x83275ED8;
	sub_82CA9F20(ctx, base);
	// 83275ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275EE8 size=64
    let mut pc: u32 = 0x83275EE8;
    'dispatch: loop {
        match pc {
            0x83275EE8 => {
    //   block [0x83275EE8..0x83275F28)
	// 83275EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275EF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275EFC: 388BC278  addi r4, r11, -0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + -15752;
	// 83275F00: 386AD428  addi r3, r10, -0x2bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -11224;
	// 83275F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275F08: 4AFB6FC9  bl 0x8222ced0
	ctx.lr = 0x83275F0C;
	sub_8222CED0(ctx, base);
	// 83275F0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275F10: 3869FA40  addi r3, r9, -0x5c0
	ctx.r[3].s64 = ctx.r[9].s64 + -1472;
	// 83275F14: 4BA3400D  bl 0x82ca9f20
	ctx.lr = 0x83275F18;
	sub_82CA9F20(ctx, base);
	// 83275F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275F28 size=64
    let mut pc: u32 = 0x83275F28;
    'dispatch: loop {
        match pc {
            0x83275F28 => {
    //   block [0x83275F28..0x83275F68)
	// 83275F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275F34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275F3C: 388BC2A8  addi r4, r11, -0x3d58
	ctx.r[4].s64 = ctx.r[11].s64 + -15704;
	// 83275F40: 386AD42C  addi r3, r10, -0x2bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -11220;
	// 83275F44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275F48: 4AFB6F89  bl 0x8222ced0
	ctx.lr = 0x83275F4C;
	sub_8222CED0(ctx, base);
	// 83275F4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275F50: 3869FA50  addi r3, r9, -0x5b0
	ctx.r[3].s64 = ctx.r[9].s64 + -1456;
	// 83275F54: 4BA33FCD  bl 0x82ca9f20
	ctx.lr = 0x83275F58;
	sub_82CA9F20(ctx, base);
	// 83275F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275F68 size=64
    let mut pc: u32 = 0x83275F68;
    'dispatch: loop {
        match pc {
            0x83275F68 => {
    //   block [0x83275F68..0x83275FA8)
	// 83275F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275F74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275F7C: 388BC2E4  addi r4, r11, -0x3d1c
	ctx.r[4].s64 = ctx.r[11].s64 + -15644;
	// 83275F80: 386AD430  addi r3, r10, -0x2bd0
	ctx.r[3].s64 = ctx.r[10].s64 + -11216;
	// 83275F84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275F88: 4AFB6F49  bl 0x8222ced0
	ctx.lr = 0x83275F8C;
	sub_8222CED0(ctx, base);
	// 83275F8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275F90: 3869FA60  addi r3, r9, -0x5a0
	ctx.r[3].s64 = ctx.r[9].s64 + -1440;
	// 83275F94: 4BA33F8D  bl 0x82ca9f20
	ctx.lr = 0x83275F98;
	sub_82CA9F20(ctx, base);
	// 83275F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275FA8 size=64
    let mut pc: u32 = 0x83275FA8;
    'dispatch: loop {
        match pc {
            0x83275FA8 => {
    //   block [0x83275FA8..0x83275FE8)
	// 83275FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275FB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275FBC: 388BC2F0  addi r4, r11, -0x3d10
	ctx.r[4].s64 = ctx.r[11].s64 + -15632;
	// 83275FC0: 386AD434  addi r3, r10, -0x2bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -11212;
	// 83275FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275FC8: 4AFB6F09  bl 0x8222ced0
	ctx.lr = 0x83275FCC;
	sub_8222CED0(ctx, base);
	// 83275FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275FD0: 3869FA70  addi r3, r9, -0x590
	ctx.r[3].s64 = ctx.r[9].s64 + -1424;
	// 83275FD4: 4BA33F4D  bl 0x82ca9f20
	ctx.lr = 0x83275FD8;
	sub_82CA9F20(ctx, base);
	// 83275FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275FE8 size=64
    let mut pc: u32 = 0x83275FE8;
    'dispatch: loop {
        match pc {
            0x83275FE8 => {
    //   block [0x83275FE8..0x83276028)
	// 83275FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275FF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275FFC: 388BC31C  addi r4, r11, -0x3ce4
	ctx.r[4].s64 = ctx.r[11].s64 + -15588;
	// 83276000: 386AD438  addi r3, r10, -0x2bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -11208;
	// 83276004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276008: 4AFB6EC9  bl 0x8222ced0
	ctx.lr = 0x8327600C;
	sub_8222CED0(ctx, base);
	// 8327600C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276010: 3869FA80  addi r3, r9, -0x580
	ctx.r[3].s64 = ctx.r[9].s64 + -1408;
	// 83276014: 4BA33F0D  bl 0x82ca9f20
	ctx.lr = 0x83276018;
	sub_82CA9F20(ctx, base);
	// 83276018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327601C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276028 size=64
    let mut pc: u32 = 0x83276028;
    'dispatch: loop {
        match pc {
            0x83276028 => {
    //   block [0x83276028..0x83276068)
	// 83276028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327602C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276034: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327603C: 388BC350  addi r4, r11, -0x3cb0
	ctx.r[4].s64 = ctx.r[11].s64 + -15536;
	// 83276040: 386AD43C  addi r3, r10, -0x2bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -11204;
	// 83276044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276048: 4AFB6E89  bl 0x8222ced0
	ctx.lr = 0x8327604C;
	sub_8222CED0(ctx, base);
	// 8327604C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276050: 3869FA90  addi r3, r9, -0x570
	ctx.r[3].s64 = ctx.r[9].s64 + -1392;
	// 83276054: 4BA33ECD  bl 0x82ca9f20
	ctx.lr = 0x83276058;
	sub_82CA9F20(ctx, base);
	// 83276058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327605C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276068 size=64
    let mut pc: u32 = 0x83276068;
    'dispatch: loop {
        match pc {
            0x83276068 => {
    //   block [0x83276068..0x832760A8)
	// 83276068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327606C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276074: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327607C: 388BC37C  addi r4, r11, -0x3c84
	ctx.r[4].s64 = ctx.r[11].s64 + -15492;
	// 83276080: 386AD440  addi r3, r10, -0x2bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -11200;
	// 83276084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276088: 4AFB6E49  bl 0x8222ced0
	ctx.lr = 0x8327608C;
	sub_8222CED0(ctx, base);
	// 8327608C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276090: 3869FAA0  addi r3, r9, -0x560
	ctx.r[3].s64 = ctx.r[9].s64 + -1376;
	// 83276094: 4BA33E8D  bl 0x82ca9f20
	ctx.lr = 0x83276098;
	sub_82CA9F20(ctx, base);
	// 83276098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327609C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832760A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832760A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832760A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832760A8 size=56
    let mut pc: u32 = 0x832760A8;
    'dispatch: loop {
        match pc {
            0x832760A8 => {
    //   block [0x832760A8..0x832760E0)
	// 832760A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832760AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832760B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832760B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832760B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832760BC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832760C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832760C4: 4AF7DC95  bl 0x821f3d58
	ctx.lr = 0x832760C8;
	sub_821F3D58(ctx, base);
	// 832760C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832760CC: 906AD444  stw r3, -0x2bbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11196 as u32), ctx.r[3].u32 ) };
	// 832760D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832760D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832760D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832760DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832760E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832760E0 size=56
    let mut pc: u32 = 0x832760E0;
    'dispatch: loop {
        match pc {
            0x832760E0 => {
    //   block [0x832760E0..0x83276118)
	// 832760E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832760E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832760E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832760EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832760F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832760F4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832760F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832760FC: 4AF7DC5D  bl 0x821f3d58
	ctx.lr = 0x83276100;
	sub_821F3D58(ctx, base);
	// 83276100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276104: 906AD448  stw r3, -0x2bb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11192 as u32), ctx.r[3].u32 ) };
	// 83276108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327610C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276118 size=56
    let mut pc: u32 = 0x83276118;
    'dispatch: loop {
        match pc {
            0x83276118 => {
    //   block [0x83276118..0x83276150)
	// 83276118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327611C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327612C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83276130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276134: 4AF7DC25  bl 0x821f3d58
	ctx.lr = 0x83276138;
	sub_821F3D58(ctx, base);
	// 83276138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327613C: 906AD44C  stw r3, -0x2bb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11188 as u32), ctx.r[3].u32 ) };
	// 83276140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327614C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276150 size=56
    let mut pc: u32 = 0x83276150;
    'dispatch: loop {
        match pc {
            0x83276150 => {
    //   block [0x83276150..0x83276188)
	// 83276150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327615C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276164: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83276168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327616C: 4AF7DBED  bl 0x821f3d58
	ctx.lr = 0x83276170;
	sub_821F3D58(ctx, base);
	// 83276170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276174: 906AD450  stw r3, -0x2bb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11184 as u32), ctx.r[3].u32 ) };
	// 83276178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327617C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276188 size=56
    let mut pc: u32 = 0x83276188;
    'dispatch: loop {
        match pc {
            0x83276188 => {
    //   block [0x83276188..0x832761C0)
	// 83276188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327618C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276194: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276198: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327619C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832761A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832761A4: 4AF7DBB5  bl 0x821f3d58
	ctx.lr = 0x832761A8;
	sub_821F3D58(ctx, base);
	// 832761A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832761AC: 906AD454  stw r3, -0x2bac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11180 as u32), ctx.r[3].u32 ) };
	// 832761B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832761B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832761B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832761BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832761C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832761C0 size=56
    let mut pc: u32 = 0x832761C0;
    'dispatch: loop {
        match pc {
            0x832761C0 => {
    //   block [0x832761C0..0x832761F8)
	// 832761C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832761C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832761C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832761CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832761D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832761D4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832761D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832761DC: 4AF7DB7D  bl 0x821f3d58
	ctx.lr = 0x832761E0;
	sub_821F3D58(ctx, base);
	// 832761E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832761E4: 906AD458  stw r3, -0x2ba8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11176 as u32), ctx.r[3].u32 ) };
	// 832761E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832761EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832761F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832761F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832761F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832761F8 size=56
    let mut pc: u32 = 0x832761F8;
    'dispatch: loop {
        match pc {
            0x832761F8 => {
    //   block [0x832761F8..0x83276230)
	// 832761F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832761FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276208: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327620C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83276210: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276214: 4AF7DB45  bl 0x821f3d58
	ctx.lr = 0x83276218;
	sub_821F3D58(ctx, base);
	// 83276218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327621C: 906AD45C  stw r3, -0x2ba4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11172 as u32), ctx.r[3].u32 ) };
	// 83276220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327622C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276230 size=56
    let mut pc: u32 = 0x83276230;
    'dispatch: loop {
        match pc {
            0x83276230 => {
    //   block [0x83276230..0x83276268)
	// 83276230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327623C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276240: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276244: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83276248: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327624C: 4AF7DB0D  bl 0x821f3d58
	ctx.lr = 0x83276250;
	sub_821F3D58(ctx, base);
	// 83276250: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276254: 906AD460  stw r3, -0x2ba0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11168 as u32), ctx.r[3].u32 ) };
	// 83276258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327625C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276268 size=56
    let mut pc: u32 = 0x83276268;
    'dispatch: loop {
        match pc {
            0x83276268 => {
    //   block [0x83276268..0x832762A0)
	// 83276268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276274: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276278: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327627C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83276280: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276284: 4AF7DAD5  bl 0x821f3d58
	ctx.lr = 0x83276288;
	sub_821F3D58(ctx, base);
	// 83276288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327628C: 906AD464  stw r3, -0x2b9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11164 as u32), ctx.r[3].u32 ) };
	// 83276290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327629C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832762A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832762A0 size=56
    let mut pc: u32 = 0x832762A0;
    'dispatch: loop {
        match pc {
            0x832762A0 => {
    //   block [0x832762A0..0x832762D8)
	// 832762A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832762A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832762A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832762AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832762B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832762B4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832762B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832762BC: 4AF7DA9D  bl 0x821f3d58
	ctx.lr = 0x832762C0;
	sub_821F3D58(ctx, base);
	// 832762C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832762C4: 906AD468  stw r3, -0x2b98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11160 as u32), ctx.r[3].u32 ) };
	// 832762C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832762CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832762D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832762D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832762D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832762D8 size=56
    let mut pc: u32 = 0x832762D8;
    'dispatch: loop {
        match pc {
            0x832762D8 => {
    //   block [0x832762D8..0x83276310)
	// 832762D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832762DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832762E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832762E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832762E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832762EC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832762F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832762F4: 4AF7DA65  bl 0x821f3d58
	ctx.lr = 0x832762F8;
	sub_821F3D58(ctx, base);
	// 832762F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832762FC: 906AD46C  stw r3, -0x2b94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11156 as u32), ctx.r[3].u32 ) };
	// 83276300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327630C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276310 size=56
    let mut pc: u32 = 0x83276310;
    'dispatch: loop {
        match pc {
            0x83276310 => {
    //   block [0x83276310..0x83276348)
	// 83276310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327631C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276320: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276324: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83276328: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327632C: 4AF7DA2D  bl 0x821f3d58
	ctx.lr = 0x83276330;
	sub_821F3D58(ctx, base);
	// 83276330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276334: 906AD470  stw r3, -0x2b90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11152 as u32), ctx.r[3].u32 ) };
	// 83276338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327633C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276348 size=56
    let mut pc: u32 = 0x83276348;
    'dispatch: loop {
        match pc {
            0x83276348 => {
    //   block [0x83276348..0x83276380)
	// 83276348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276354: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276358: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327635C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83276360: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276364: 4AF7D9F5  bl 0x821f3d58
	ctx.lr = 0x83276368;
	sub_821F3D58(ctx, base);
	// 83276368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327636C: 906AD474  stw r3, -0x2b8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11148 as u32), ctx.r[3].u32 ) };
	// 83276370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327637C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276380 size=56
    let mut pc: u32 = 0x83276380;
    'dispatch: loop {
        match pc {
            0x83276380 => {
    //   block [0x83276380..0x832763B8)
	// 83276380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327638C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276390: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276394: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83276398: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327639C: 4AF7D9BD  bl 0x821f3d58
	ctx.lr = 0x832763A0;
	sub_821F3D58(ctx, base);
	// 832763A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832763A4: 906AD478  stw r3, -0x2b88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11144 as u32), ctx.r[3].u32 ) };
	// 832763A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832763AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832763B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832763B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832763B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832763B8 size=56
    let mut pc: u32 = 0x832763B8;
    'dispatch: loop {
        match pc {
            0x832763B8 => {
    //   block [0x832763B8..0x832763F0)
	// 832763B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832763BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832763C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832763C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832763C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832763CC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832763D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832763D4: 4AF7D985  bl 0x821f3d58
	ctx.lr = 0x832763D8;
	sub_821F3D58(ctx, base);
	// 832763D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832763DC: 906AD47C  stw r3, -0x2b84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11140 as u32), ctx.r[3].u32 ) };
	// 832763E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832763E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832763E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832763EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832763F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832763F0 size=56
    let mut pc: u32 = 0x832763F0;
    'dispatch: loop {
        match pc {
            0x832763F0 => {
    //   block [0x832763F0..0x83276428)
	// 832763F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832763F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832763F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832763FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276404: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83276408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327640C: 4AF7D94D  bl 0x821f3d58
	ctx.lr = 0x83276410;
	sub_821F3D58(ctx, base);
	// 83276410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276414: 906AD480  stw r3, -0x2b80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11136 as u32), ctx.r[3].u32 ) };
	// 83276418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276428 size=56
    let mut pc: u32 = 0x83276428;
    'dispatch: loop {
        match pc {
            0x83276428 => {
    //   block [0x83276428..0x83276460)
	// 83276428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276434: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327643C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83276440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276444: 4AF7D915  bl 0x821f3d58
	ctx.lr = 0x83276448;
	sub_821F3D58(ctx, base);
	// 83276448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327644C: 906AD484  stw r3, -0x2b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11132 as u32), ctx.r[3].u32 ) };
	// 83276450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327645C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276460 size=56
    let mut pc: u32 = 0x83276460;
    'dispatch: loop {
        match pc {
            0x83276460 => {
    //   block [0x83276460..0x83276498)
	// 83276460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327646C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276474: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83276478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327647C: 4AF7D8DD  bl 0x821f3d58
	ctx.lr = 0x83276480;
	sub_821F3D58(ctx, base);
	// 83276480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276484: 906AD488  stw r3, -0x2b78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11128 as u32), ctx.r[3].u32 ) };
	// 83276488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327648C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276498 size=56
    let mut pc: u32 = 0x83276498;
    'dispatch: loop {
        match pc {
            0x83276498 => {
    //   block [0x83276498..0x832764D0)
	// 83276498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832764A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832764A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832764A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832764AC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832764B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832764B4: 4AF7D8A5  bl 0x821f3d58
	ctx.lr = 0x832764B8;
	sub_821F3D58(ctx, base);
	// 832764B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832764BC: 906AD48C  stw r3, -0x2b74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11124 as u32), ctx.r[3].u32 ) };
	// 832764C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832764C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832764C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832764CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832764D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832764D0 size=56
    let mut pc: u32 = 0x832764D0;
    'dispatch: loop {
        match pc {
            0x832764D0 => {
    //   block [0x832764D0..0x83276508)
	// 832764D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832764D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832764D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832764DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832764E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832764E4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832764E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832764EC: 4AF7D86D  bl 0x821f3d58
	ctx.lr = 0x832764F0;
	sub_821F3D58(ctx, base);
	// 832764F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832764F4: 906AD490  stw r3, -0x2b70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11120 as u32), ctx.r[3].u32 ) };
	// 832764F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832764FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276508 size=56
    let mut pc: u32 = 0x83276508;
    'dispatch: loop {
        match pc {
            0x83276508 => {
    //   block [0x83276508..0x83276540)
	// 83276508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327651C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83276520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276524: 4AF7D835  bl 0x821f3d58
	ctx.lr = 0x83276528;
	sub_821F3D58(ctx, base);
	// 83276528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327652C: 906AD494  stw r3, -0x2b6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11116 as u32), ctx.r[3].u32 ) };
	// 83276530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327653C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276540 size=64
    let mut pc: u32 = 0x83276540;
    'dispatch: loop {
        match pc {
            0x83276540 => {
    //   block [0x83276540..0x83276580)
	// 83276540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327654C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276554: 388BC700  addi r4, r11, -0x3900
	ctx.r[4].s64 = ctx.r[11].s64 + -14592;
	// 83276558: 386AD498  addi r3, r10, -0x2b68
	ctx.r[3].s64 = ctx.r[10].s64 + -11112;
	// 8327655C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276560: 4AFB6971  bl 0x8222ced0
	ctx.lr = 0x83276564;
	sub_8222CED0(ctx, base);
	// 83276564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276568: 3869FAB0  addi r3, r9, -0x550
	ctx.r[3].s64 = ctx.r[9].s64 + -1360;
	// 8327656C: 4BA339B5  bl 0x82ca9f20
	ctx.lr = 0x83276570;
	sub_82CA9F20(ctx, base);
	// 83276570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327657C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276580 size=64
    let mut pc: u32 = 0x83276580;
    'dispatch: loop {
        match pc {
            0x83276580 => {
    //   block [0x83276580..0x832765C0)
	// 83276580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327658C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276594: 388BC734  addi r4, r11, -0x38cc
	ctx.r[4].s64 = ctx.r[11].s64 + -14540;
	// 83276598: 386AD49C  addi r3, r10, -0x2b64
	ctx.r[3].s64 = ctx.r[10].s64 + -11108;
	// 8327659C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832765A0: 4AFB6931  bl 0x8222ced0
	ctx.lr = 0x832765A4;
	sub_8222CED0(ctx, base);
	// 832765A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832765A8: 3869FAC0  addi r3, r9, -0x540
	ctx.r[3].s64 = ctx.r[9].s64 + -1344;
	// 832765AC: 4BA33975  bl 0x82ca9f20
	ctx.lr = 0x832765B0;
	sub_82CA9F20(ctx, base);
	// 832765B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832765B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832765B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832765BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832765C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832765C0 size=64
    let mut pc: u32 = 0x832765C0;
    'dispatch: loop {
        match pc {
            0x832765C0 => {
    //   block [0x832765C0..0x83276600)
	// 832765C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832765C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832765C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832765CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832765D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832765D4: 388BC76C  addi r4, r11, -0x3894
	ctx.r[4].s64 = ctx.r[11].s64 + -14484;
	// 832765D8: 386AD4A0  addi r3, r10, -0x2b60
	ctx.r[3].s64 = ctx.r[10].s64 + -11104;
	// 832765DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832765E0: 4AFB68F1  bl 0x8222ced0
	ctx.lr = 0x832765E4;
	sub_8222CED0(ctx, base);
	// 832765E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832765E8: 3869FAD0  addi r3, r9, -0x530
	ctx.r[3].s64 = ctx.r[9].s64 + -1328;
	// 832765EC: 4BA33935  bl 0x82ca9f20
	ctx.lr = 0x832765F0;
	sub_82CA9F20(ctx, base);
	// 832765F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832765F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832765F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832765FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276600 size=64
    let mut pc: u32 = 0x83276600;
    'dispatch: loop {
        match pc {
            0x83276600 => {
    //   block [0x83276600..0x83276640)
	// 83276600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327660C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276614: 388BC79C  addi r4, r11, -0x3864
	ctx.r[4].s64 = ctx.r[11].s64 + -14436;
	// 83276618: 386AD4A4  addi r3, r10, -0x2b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11100;
	// 8327661C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276620: 4AFB68B1  bl 0x8222ced0
	ctx.lr = 0x83276624;
	sub_8222CED0(ctx, base);
	// 83276624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276628: 3869FAE0  addi r3, r9, -0x520
	ctx.r[3].s64 = ctx.r[9].s64 + -1312;
	// 8327662C: 4BA338F5  bl 0x82ca9f20
	ctx.lr = 0x83276630;
	sub_82CA9F20(ctx, base);
	// 83276630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327663C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276640 size=64
    let mut pc: u32 = 0x83276640;
    'dispatch: loop {
        match pc {
            0x83276640 => {
    //   block [0x83276640..0x83276680)
	// 83276640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327664C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276654: 388BC7D8  addi r4, r11, -0x3828
	ctx.r[4].s64 = ctx.r[11].s64 + -14376;
	// 83276658: 386AD4A8  addi r3, r10, -0x2b58
	ctx.r[3].s64 = ctx.r[10].s64 + -11096;
	// 8327665C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276660: 4AFB6871  bl 0x8222ced0
	ctx.lr = 0x83276664;
	sub_8222CED0(ctx, base);
	// 83276664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276668: 3869FAF0  addi r3, r9, -0x510
	ctx.r[3].s64 = ctx.r[9].s64 + -1296;
	// 8327666C: 4BA338B5  bl 0x82ca9f20
	ctx.lr = 0x83276670;
	sub_82CA9F20(ctx, base);
	// 83276670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327667C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276680 size=64
    let mut pc: u32 = 0x83276680;
    'dispatch: loop {
        match pc {
            0x83276680 => {
    //   block [0x83276680..0x832766C0)
	// 83276680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327668C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276694: 388BC818  addi r4, r11, -0x37e8
	ctx.r[4].s64 = ctx.r[11].s64 + -14312;
	// 83276698: 386AD4AC  addi r3, r10, -0x2b54
	ctx.r[3].s64 = ctx.r[10].s64 + -11092;
	// 8327669C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832766A0: 4AFB6831  bl 0x8222ced0
	ctx.lr = 0x832766A4;
	sub_8222CED0(ctx, base);
	// 832766A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832766A8: 3869FB00  addi r3, r9, -0x500
	ctx.r[3].s64 = ctx.r[9].s64 + -1280;
	// 832766AC: 4BA33875  bl 0x82ca9f20
	ctx.lr = 0x832766B0;
	sub_82CA9F20(ctx, base);
	// 832766B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832766B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832766B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832766BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832766C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832766C0 size=64
    let mut pc: u32 = 0x832766C0;
    'dispatch: loop {
        match pc {
            0x832766C0 => {
    //   block [0x832766C0..0x83276700)
	// 832766C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832766C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832766C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832766CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832766D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832766D4: 388BC860  addi r4, r11, -0x37a0
	ctx.r[4].s64 = ctx.r[11].s64 + -14240;
	// 832766D8: 386AD4B0  addi r3, r10, -0x2b50
	ctx.r[3].s64 = ctx.r[10].s64 + -11088;
	// 832766DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832766E0: 4AFB67F1  bl 0x8222ced0
	ctx.lr = 0x832766E4;
	sub_8222CED0(ctx, base);
	// 832766E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832766E8: 3869FB10  addi r3, r9, -0x4f0
	ctx.r[3].s64 = ctx.r[9].s64 + -1264;
	// 832766EC: 4BA33835  bl 0x82ca9f20
	ctx.lr = 0x832766F0;
	sub_82CA9F20(ctx, base);
	// 832766F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832766F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832766F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832766FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276700 size=64
    let mut pc: u32 = 0x83276700;
    'dispatch: loop {
        match pc {
            0x83276700 => {
    //   block [0x83276700..0x83276740)
	// 83276700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327670C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276714: 388BC8A8  addi r4, r11, -0x3758
	ctx.r[4].s64 = ctx.r[11].s64 + -14168;
	// 83276718: 386AD4B4  addi r3, r10, -0x2b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11084;
	// 8327671C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276720: 4AFB67B1  bl 0x8222ced0
	ctx.lr = 0x83276724;
	sub_8222CED0(ctx, base);
	// 83276724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276728: 3869FB20  addi r3, r9, -0x4e0
	ctx.r[3].s64 = ctx.r[9].s64 + -1248;
	// 8327672C: 4BA337F5  bl 0x82ca9f20
	ctx.lr = 0x83276730;
	sub_82CA9F20(ctx, base);
	// 83276730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327673C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276740 size=64
    let mut pc: u32 = 0x83276740;
    'dispatch: loop {
        match pc {
            0x83276740 => {
    //   block [0x83276740..0x83276780)
	// 83276740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327674C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276750: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276754: 388BC8E8  addi r4, r11, -0x3718
	ctx.r[4].s64 = ctx.r[11].s64 + -14104;
	// 83276758: 386AD4B8  addi r3, r10, -0x2b48
	ctx.r[3].s64 = ctx.r[10].s64 + -11080;
	// 8327675C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276760: 4AFB6771  bl 0x8222ced0
	ctx.lr = 0x83276764;
	sub_8222CED0(ctx, base);
	// 83276764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276768: 3869FB30  addi r3, r9, -0x4d0
	ctx.r[3].s64 = ctx.r[9].s64 + -1232;
	// 8327676C: 4BA337B5  bl 0x82ca9f20
	ctx.lr = 0x83276770;
	sub_82CA9F20(ctx, base);
	// 83276770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327677C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276780 size=64
    let mut pc: u32 = 0x83276780;
    'dispatch: loop {
        match pc {
            0x83276780 => {
    //   block [0x83276780..0x832767C0)
	// 83276780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327678C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276790: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276794: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83276798: 386AD4BC  addi r3, r10, -0x2b44
	ctx.r[3].s64 = ctx.r[10].s64 + -11076;
	// 8327679C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832767A0: 4AFB6731  bl 0x8222ced0
	ctx.lr = 0x832767A4;
	sub_8222CED0(ctx, base);
	// 832767A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832767A8: 3869FB40  addi r3, r9, -0x4c0
	ctx.r[3].s64 = ctx.r[9].s64 + -1216;
	// 832767AC: 4BA33775  bl 0x82ca9f20
	ctx.lr = 0x832767B0;
	sub_82CA9F20(ctx, base);
	// 832767B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832767B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832767B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832767BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832767C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832767C0 size=64
    let mut pc: u32 = 0x832767C0;
    'dispatch: loop {
        match pc {
            0x832767C0 => {
    //   block [0x832767C0..0x83276800)
	// 832767C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832767C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832767C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832767CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832767D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832767D4: 388BCB94  addi r4, r11, -0x346c
	ctx.r[4].s64 = ctx.r[11].s64 + -13420;
	// 832767D8: 386AD4C0  addi r3, r10, -0x2b40
	ctx.r[3].s64 = ctx.r[10].s64 + -11072;
	// 832767DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832767E0: 4AFB66F1  bl 0x8222ced0
	ctx.lr = 0x832767E4;
	sub_8222CED0(ctx, base);
	// 832767E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832767E8: 3869FB50  addi r3, r9, -0x4b0
	ctx.r[3].s64 = ctx.r[9].s64 + -1200;
	// 832767EC: 4BA33735  bl 0x82ca9f20
	ctx.lr = 0x832767F0;
	sub_82CA9F20(ctx, base);
	// 832767F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832767F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832767F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832767FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276800 size=56
    let mut pc: u32 = 0x83276800;
    'dispatch: loop {
        match pc {
            0x83276800 => {
    //   block [0x83276800..0x83276838)
	// 83276800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327680C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276814: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83276818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327681C: 4AF7D53D  bl 0x821f3d58
	ctx.lr = 0x83276820;
	sub_821F3D58(ctx, base);
	// 83276820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276824: 906AD4C4  stw r3, -0x2b3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11068 as u32), ctx.r[3].u32 ) };
	// 83276828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327682C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276838 size=56
    let mut pc: u32 = 0x83276838;
    'dispatch: loop {
        match pc {
            0x83276838 => {
    //   block [0x83276838..0x83276870)
	// 83276838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327683C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327684C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83276850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276854: 4AF7D505  bl 0x821f3d58
	ctx.lr = 0x83276858;
	sub_821F3D58(ctx, base);
	// 83276858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327685C: 906AD4C8  stw r3, -0x2b38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11064 as u32), ctx.r[3].u32 ) };
	// 83276860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327686C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276870 size=56
    let mut pc: u32 = 0x83276870;
    'dispatch: loop {
        match pc {
            0x83276870 => {
    //   block [0x83276870..0x832768A8)
	// 83276870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327687C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276884: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83276888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327688C: 4AF7D4CD  bl 0x821f3d58
	ctx.lr = 0x83276890;
	sub_821F3D58(ctx, base);
	// 83276890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276894: 906AD4CC  stw r3, -0x2b34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11060 as u32), ctx.r[3].u32 ) };
	// 83276898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327689C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832768A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832768A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832768A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832768A8 size=56
    let mut pc: u32 = 0x832768A8;
    'dispatch: loop {
        match pc {
            0x832768A8 => {
    //   block [0x832768A8..0x832768E0)
	// 832768A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832768AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832768B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832768B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832768B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832768BC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832768C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832768C4: 4AF7D495  bl 0x821f3d58
	ctx.lr = 0x832768C8;
	sub_821F3D58(ctx, base);
	// 832768C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832768CC: 906AD4D0  stw r3, -0x2b30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11056 as u32), ctx.r[3].u32 ) };
	// 832768D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832768D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832768D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832768DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832768E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832768E0 size=56
    let mut pc: u32 = 0x832768E0;
    'dispatch: loop {
        match pc {
            0x832768E0 => {
    //   block [0x832768E0..0x83276918)
	// 832768E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832768E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832768E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832768EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832768F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832768F4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832768F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832768FC: 4AF7D45D  bl 0x821f3d58
	ctx.lr = 0x83276900;
	sub_821F3D58(ctx, base);
	// 83276900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276904: 906AD4D4  stw r3, -0x2b2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11052 as u32), ctx.r[3].u32 ) };
	// 83276908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327690C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276918 size=56
    let mut pc: u32 = 0x83276918;
    'dispatch: loop {
        match pc {
            0x83276918 => {
    //   block [0x83276918..0x83276950)
	// 83276918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327691C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327692C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83276930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276934: 4AF7D425  bl 0x821f3d58
	ctx.lr = 0x83276938;
	sub_821F3D58(ctx, base);
	// 83276938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327693C: 906AD4D8  stw r3, -0x2b28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11048 as u32), ctx.r[3].u32 ) };
	// 83276940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327694C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276950 size=56
    let mut pc: u32 = 0x83276950;
    'dispatch: loop {
        match pc {
            0x83276950 => {
    //   block [0x83276950..0x83276988)
	// 83276950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327695C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276964: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83276968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327696C: 4AF7D3ED  bl 0x821f3d58
	ctx.lr = 0x83276970;
	sub_821F3D58(ctx, base);
	// 83276970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276974: 906AD4DC  stw r3, -0x2b24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11044 as u32), ctx.r[3].u32 ) };
	// 83276978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327697C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276988 size=56
    let mut pc: u32 = 0x83276988;
    'dispatch: loop {
        match pc {
            0x83276988 => {
    //   block [0x83276988..0x832769C0)
	// 83276988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327698C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327699C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832769A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832769A4: 4AF7D3B5  bl 0x821f3d58
	ctx.lr = 0x832769A8;
	sub_821F3D58(ctx, base);
	// 832769A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832769AC: 906AD4E0  stw r3, -0x2b20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11040 as u32), ctx.r[3].u32 ) };
	// 832769B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832769B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832769B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832769BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832769C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832769C0 size=56
    let mut pc: u32 = 0x832769C0;
    'dispatch: loop {
        match pc {
            0x832769C0 => {
    //   block [0x832769C0..0x832769F8)
	// 832769C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832769C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832769C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832769CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832769D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832769D4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832769D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832769DC: 4AF7D37D  bl 0x821f3d58
	ctx.lr = 0x832769E0;
	sub_821F3D58(ctx, base);
	// 832769E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832769E4: 906AD4E4  stw r3, -0x2b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11036 as u32), ctx.r[3].u32 ) };
	// 832769E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832769EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832769F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832769F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832769F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832769F8 size=56
    let mut pc: u32 = 0x832769F8;
    'dispatch: loop {
        match pc {
            0x832769F8 => {
    //   block [0x832769F8..0x83276A30)
	// 832769F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832769FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276A08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276A0C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83276A10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276A14: 4AF7D345  bl 0x821f3d58
	ctx.lr = 0x83276A18;
	sub_821F3D58(ctx, base);
	// 83276A18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276A1C: 906AD4E8  stw r3, -0x2b18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11032 as u32), ctx.r[3].u32 ) };
	// 83276A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276A30 size=56
    let mut pc: u32 = 0x83276A30;
    'dispatch: loop {
        match pc {
            0x83276A30 => {
    //   block [0x83276A30..0x83276A68)
	// 83276A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276A40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276A44: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83276A48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276A4C: 4AF7D30D  bl 0x821f3d58
	ctx.lr = 0x83276A50;
	sub_821F3D58(ctx, base);
	// 83276A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276A54: 906AD4EC  stw r3, -0x2b14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11028 as u32), ctx.r[3].u32 ) };
	// 83276A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276A68 size=56
    let mut pc: u32 = 0x83276A68;
    'dispatch: loop {
        match pc {
            0x83276A68 => {
    //   block [0x83276A68..0x83276AA0)
	// 83276A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276A74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276A78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276A7C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83276A80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276A84: 4AF7D2D5  bl 0x821f3d58
	ctx.lr = 0x83276A88;
	sub_821F3D58(ctx, base);
	// 83276A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276A8C: 906AD4F0  stw r3, -0x2b10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11024 as u32), ctx.r[3].u32 ) };
	// 83276A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276AA0 size=56
    let mut pc: u32 = 0x83276AA0;
    'dispatch: loop {
        match pc {
            0x83276AA0 => {
    //   block [0x83276AA0..0x83276AD8)
	// 83276AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276AAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276AB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276AB4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83276AB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276ABC: 4AF7D29D  bl 0x821f3d58
	ctx.lr = 0x83276AC0;
	sub_821F3D58(ctx, base);
	// 83276AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276AC4: 906AD4F4  stw r3, -0x2b0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11020 as u32), ctx.r[3].u32 ) };
	// 83276AC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276AD8 size=56
    let mut pc: u32 = 0x83276AD8;
    'dispatch: loop {
        match pc {
            0x83276AD8 => {
    //   block [0x83276AD8..0x83276B10)
	// 83276AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276AE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276AE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276AEC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83276AF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276AF4: 4AF7D265  bl 0x821f3d58
	ctx.lr = 0x83276AF8;
	sub_821F3D58(ctx, base);
	// 83276AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276AFC: 906AD4F8  stw r3, -0x2b08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11016 as u32), ctx.r[3].u32 ) };
	// 83276B00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276B10 size=56
    let mut pc: u32 = 0x83276B10;
    'dispatch: loop {
        match pc {
            0x83276B10 => {
    //   block [0x83276B10..0x83276B48)
	// 83276B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276B1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276B20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276B24: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83276B28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276B2C: 4AF7D22D  bl 0x821f3d58
	ctx.lr = 0x83276B30;
	sub_821F3D58(ctx, base);
	// 83276B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276B34: 906AD4FC  stw r3, -0x2b04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11012 as u32), ctx.r[3].u32 ) };
	// 83276B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276B48 size=56
    let mut pc: u32 = 0x83276B48;
    'dispatch: loop {
        match pc {
            0x83276B48 => {
    //   block [0x83276B48..0x83276B80)
	// 83276B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276B54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276B58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276B5C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83276B60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276B64: 4AF7D1F5  bl 0x821f3d58
	ctx.lr = 0x83276B68;
	sub_821F3D58(ctx, base);
	// 83276B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276B6C: 906AD500  stw r3, -0x2b00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11008 as u32), ctx.r[3].u32 ) };
	// 83276B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276B80 size=56
    let mut pc: u32 = 0x83276B80;
    'dispatch: loop {
        match pc {
            0x83276B80 => {
    //   block [0x83276B80..0x83276BB8)
	// 83276B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276B8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276B90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276B94: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83276B98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276B9C: 4AF7D1BD  bl 0x821f3d58
	ctx.lr = 0x83276BA0;
	sub_821F3D58(ctx, base);
	// 83276BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276BA4: 906AD504  stw r3, -0x2afc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11004 as u32), ctx.r[3].u32 ) };
	// 83276BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276BB8 size=56
    let mut pc: u32 = 0x83276BB8;
    'dispatch: loop {
        match pc {
            0x83276BB8 => {
    //   block [0x83276BB8..0x83276BF0)
	// 83276BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276BCC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83276BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276BD4: 4AF7D185  bl 0x821f3d58
	ctx.lr = 0x83276BD8;
	sub_821F3D58(ctx, base);
	// 83276BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276BDC: 906AD508  stw r3, -0x2af8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11000 as u32), ctx.r[3].u32 ) };
	// 83276BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276BF0 size=56
    let mut pc: u32 = 0x83276BF0;
    'dispatch: loop {
        match pc {
            0x83276BF0 => {
    //   block [0x83276BF0..0x83276C28)
	// 83276BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276C00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276C04: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83276C08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276C0C: 4AF7D14D  bl 0x821f3d58
	ctx.lr = 0x83276C10;
	sub_821F3D58(ctx, base);
	// 83276C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276C14: 906AD50C  stw r3, -0x2af4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10996 as u32), ctx.r[3].u32 ) };
	// 83276C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276C28 size=56
    let mut pc: u32 = 0x83276C28;
    'dispatch: loop {
        match pc {
            0x83276C28 => {
    //   block [0x83276C28..0x83276C60)
	// 83276C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276C38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276C3C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83276C40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276C44: 4AF7D115  bl 0x821f3d58
	ctx.lr = 0x83276C48;
	sub_821F3D58(ctx, base);
	// 83276C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276C4C: 906AD510  stw r3, -0x2af0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10992 as u32), ctx.r[3].u32 ) };
	// 83276C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276C60 size=56
    let mut pc: u32 = 0x83276C60;
    'dispatch: loop {
        match pc {
            0x83276C60 => {
    //   block [0x83276C60..0x83276C98)
	// 83276C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276C6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276C70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276C74: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83276C78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276C7C: 4AF7D0DD  bl 0x821f3d58
	ctx.lr = 0x83276C80;
	sub_821F3D58(ctx, base);
	// 83276C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276C84: 906AD514  stw r3, -0x2aec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10988 as u32), ctx.r[3].u32 ) };
	// 83276C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276C98 size=64
    let mut pc: u32 = 0x83276C98;
    'dispatch: loop {
        match pc {
            0x83276C98 => {
    //   block [0x83276C98..0x83276CD8)
	// 83276C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276CA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276CA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276CAC: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 83276CB0: 386AD518  addi r3, r10, -0x2ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -10984;
	// 83276CB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276CB8: 4AFB6219  bl 0x8222ced0
	ctx.lr = 0x83276CBC;
	sub_8222CED0(ctx, base);
	// 83276CBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276CC0: 3869FB60  addi r3, r9, -0x4a0
	ctx.r[3].s64 = ctx.r[9].s64 + -1184;
	// 83276CC4: 4BA3325D  bl 0x82ca9f20
	ctx.lr = 0x83276CC8;
	sub_82CA9F20(ctx, base);
	// 83276CC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276CD8 size=64
    let mut pc: u32 = 0x83276CD8;
    'dispatch: loop {
        match pc {
            0x83276CD8 => {
    //   block [0x83276CD8..0x83276D18)
	// 83276CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276CE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276CE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276CEC: 388BBDC4  addi r4, r11, -0x423c
	ctx.r[4].s64 = ctx.r[11].s64 + -16956;
	// 83276CF0: 386AD51C  addi r3, r10, -0x2ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -10980;
	// 83276CF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276CF8: 4AFB61D9  bl 0x8222ced0
	ctx.lr = 0x83276CFC;
	sub_8222CED0(ctx, base);
	// 83276CFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276D00: 3869FB70  addi r3, r9, -0x490
	ctx.r[3].s64 = ctx.r[9].s64 + -1168;
	// 83276D04: 4BA3321D  bl 0x82ca9f20
	ctx.lr = 0x83276D08;
	sub_82CA9F20(ctx, base);
	// 83276D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276D18 size=64
    let mut pc: u32 = 0x83276D18;
    'dispatch: loop {
        match pc {
            0x83276D18 => {
    //   block [0x83276D18..0x83276D58)
	// 83276D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276D24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276D28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276D2C: 388BBDF4  addi r4, r11, -0x420c
	ctx.r[4].s64 = ctx.r[11].s64 + -16908;
	// 83276D30: 386AD520  addi r3, r10, -0x2ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -10976;
	// 83276D34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276D38: 4AFB6199  bl 0x8222ced0
	ctx.lr = 0x83276D3C;
	sub_8222CED0(ctx, base);
	// 83276D3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276D40: 3869FB80  addi r3, r9, -0x480
	ctx.r[3].s64 = ctx.r[9].s64 + -1152;
	// 83276D44: 4BA331DD  bl 0x82ca9f20
	ctx.lr = 0x83276D48;
	sub_82CA9F20(ctx, base);
	// 83276D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276D58 size=64
    let mut pc: u32 = 0x83276D58;
    'dispatch: loop {
        match pc {
            0x83276D58 => {
    //   block [0x83276D58..0x83276D98)
	// 83276D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276D64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276D68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276D6C: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 83276D70: 386AD524  addi r3, r10, -0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + -10972;
	// 83276D74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276D78: 4AFB6159  bl 0x8222ced0
	ctx.lr = 0x83276D7C;
	sub_8222CED0(ctx, base);
	// 83276D7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276D80: 3869FB90  addi r3, r9, -0x470
	ctx.r[3].s64 = ctx.r[9].s64 + -1136;
	// 83276D84: 4BA3319D  bl 0x82ca9f20
	ctx.lr = 0x83276D88;
	sub_82CA9F20(ctx, base);
	// 83276D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276D98 size=64
    let mut pc: u32 = 0x83276D98;
    'dispatch: loop {
        match pc {
            0x83276D98 => {
    //   block [0x83276D98..0x83276DD8)
	// 83276D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276DA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276DAC: 388BBBB8  addi r4, r11, -0x4448
	ctx.r[4].s64 = ctx.r[11].s64 + -17480;
	// 83276DB0: 386AD528  addi r3, r10, -0x2ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -10968;
	// 83276DB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276DB8: 4AFB6119  bl 0x8222ced0
	ctx.lr = 0x83276DBC;
	sub_8222CED0(ctx, base);
	// 83276DBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276DC0: 3869FBA0  addi r3, r9, -0x460
	ctx.r[3].s64 = ctx.r[9].s64 + -1120;
	// 83276DC4: 4BA3315D  bl 0x82ca9f20
	ctx.lr = 0x83276DC8;
	sub_82CA9F20(ctx, base);
	// 83276DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276DD8 size=64
    let mut pc: u32 = 0x83276DD8;
    'dispatch: loop {
        match pc {
            0x83276DD8 => {
    //   block [0x83276DD8..0x83276E18)
	// 83276DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276DE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83276DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276DEC: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83276DF0: 386AD52C  addi r3, r10, -0x2ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -10964;
	// 83276DF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83276DF8: 4AFB60D9  bl 0x8222ced0
	ctx.lr = 0x83276DFC;
	sub_8222CED0(ctx, base);
	// 83276DFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83276E00: 3869FBB0  addi r3, r9, -0x450
	ctx.r[3].s64 = ctx.r[9].s64 + -1104;
	// 83276E04: 4BA3311D  bl 0x82ca9f20
	ctx.lr = 0x83276E08;
	sub_82CA9F20(ctx, base);
	// 83276E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276E18 size=56
    let mut pc: u32 = 0x83276E18;
    'dispatch: loop {
        match pc {
            0x83276E18 => {
    //   block [0x83276E18..0x83276E50)
	// 83276E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276E24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276E28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276E2C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83276E30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276E34: 4AF7CF25  bl 0x821f3d58
	ctx.lr = 0x83276E38;
	sub_821F3D58(ctx, base);
	// 83276E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276E3C: 906AD530  stw r3, -0x2ad0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10960 as u32), ctx.r[3].u32 ) };
	// 83276E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276E50 size=56
    let mut pc: u32 = 0x83276E50;
    'dispatch: loop {
        match pc {
            0x83276E50 => {
    //   block [0x83276E50..0x83276E88)
	// 83276E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276E5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276E60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276E64: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83276E68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276E6C: 4AF7CEED  bl 0x821f3d58
	ctx.lr = 0x83276E70;
	sub_821F3D58(ctx, base);
	// 83276E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276E74: 906AD534  stw r3, -0x2acc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10956 as u32), ctx.r[3].u32 ) };
	// 83276E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276E88 size=56
    let mut pc: u32 = 0x83276E88;
    'dispatch: loop {
        match pc {
            0x83276E88 => {
    //   block [0x83276E88..0x83276EC0)
	// 83276E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276E94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276E98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276E9C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83276EA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276EA4: 4AF7CEB5  bl 0x821f3d58
	ctx.lr = 0x83276EA8;
	sub_821F3D58(ctx, base);
	// 83276EA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276EAC: 906AD538  stw r3, -0x2ac8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10952 as u32), ctx.r[3].u32 ) };
	// 83276EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276EC0 size=56
    let mut pc: u32 = 0x83276EC0;
    'dispatch: loop {
        match pc {
            0x83276EC0 => {
    //   block [0x83276EC0..0x83276EF8)
	// 83276EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276ECC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276ED0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276ED4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83276ED8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276EDC: 4AF7CE7D  bl 0x821f3d58
	ctx.lr = 0x83276EE0;
	sub_821F3D58(ctx, base);
	// 83276EE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276EE4: 906AD53C  stw r3, -0x2ac4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10948 as u32), ctx.r[3].u32 ) };
	// 83276EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276EF8 size=56
    let mut pc: u32 = 0x83276EF8;
    'dispatch: loop {
        match pc {
            0x83276EF8 => {
    //   block [0x83276EF8..0x83276F30)
	// 83276EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276F04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276F08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276F0C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83276F10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276F14: 4AF7CE45  bl 0x821f3d58
	ctx.lr = 0x83276F18;
	sub_821F3D58(ctx, base);
	// 83276F18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276F1C: 906AD540  stw r3, -0x2ac0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10944 as u32), ctx.r[3].u32 ) };
	// 83276F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276F30 size=56
    let mut pc: u32 = 0x83276F30;
    'dispatch: loop {
        match pc {
            0x83276F30 => {
    //   block [0x83276F30..0x83276F68)
	// 83276F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276F3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276F40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276F44: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83276F48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276F4C: 4AF7CE0D  bl 0x821f3d58
	ctx.lr = 0x83276F50;
	sub_821F3D58(ctx, base);
	// 83276F50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276F54: 906AD544  stw r3, -0x2abc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10940 as u32), ctx.r[3].u32 ) };
	// 83276F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276F68 size=56
    let mut pc: u32 = 0x83276F68;
    'dispatch: loop {
        match pc {
            0x83276F68 => {
    //   block [0x83276F68..0x83276FA0)
	// 83276F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276F74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276F78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276F7C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83276F80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276F84: 4AF7CDD5  bl 0x821f3d58
	ctx.lr = 0x83276F88;
	sub_821F3D58(ctx, base);
	// 83276F88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276F8C: 906AD548  stw r3, -0x2ab8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10936 as u32), ctx.r[3].u32 ) };
	// 83276F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276FA0 size=56
    let mut pc: u32 = 0x83276FA0;
    'dispatch: loop {
        match pc {
            0x83276FA0 => {
    //   block [0x83276FA0..0x83276FD8)
	// 83276FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276FAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276FB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276FB4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83276FB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276FBC: 4AF7CD9D  bl 0x821f3d58
	ctx.lr = 0x83276FC0;
	sub_821F3D58(ctx, base);
	// 83276FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276FC4: 906AD54C  stw r3, -0x2ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10932 as u32), ctx.r[3].u32 ) };
	// 83276FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83276FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83276FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83276FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83276FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83276FD8 size=56
    let mut pc: u32 = 0x83276FD8;
    'dispatch: loop {
        match pc {
            0x83276FD8 => {
    //   block [0x83276FD8..0x83277010)
	// 83276FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83276FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83276FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83276FE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83276FE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83276FEC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83276FF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83276FF4: 4AF7CD65  bl 0x821f3d58
	ctx.lr = 0x83276FF8;
	sub_821F3D58(ctx, base);
	// 83276FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83276FFC: 906AD550  stw r3, -0x2ab0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10928 as u32), ctx.r[3].u32 ) };
	// 83277000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327700C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277010 size=56
    let mut pc: u32 = 0x83277010;
    'dispatch: loop {
        match pc {
            0x83277010 => {
    //   block [0x83277010..0x83277048)
	// 83277010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327701C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277020: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277024: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83277028: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327702C: 4AF7CD2D  bl 0x821f3d58
	ctx.lr = 0x83277030;
	sub_821F3D58(ctx, base);
	// 83277030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277034: 906AD554  stw r3, -0x2aac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10924 as u32), ctx.r[3].u32 ) };
	// 83277038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327703C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277048 size=56
    let mut pc: u32 = 0x83277048;
    'dispatch: loop {
        match pc {
            0x83277048 => {
    //   block [0x83277048..0x83277080)
	// 83277048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327704C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277054: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277058: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327705C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83277060: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277064: 4AF7CCF5  bl 0x821f3d58
	ctx.lr = 0x83277068;
	sub_821F3D58(ctx, base);
	// 83277068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327706C: 906AD558  stw r3, -0x2aa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10920 as u32), ctx.r[3].u32 ) };
	// 83277070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327707C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277080 size=56
    let mut pc: u32 = 0x83277080;
    'dispatch: loop {
        match pc {
            0x83277080 => {
    //   block [0x83277080..0x832770B8)
	// 83277080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327708C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277090: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277094: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83277098: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327709C: 4AF7CCBD  bl 0x821f3d58
	ctx.lr = 0x832770A0;
	sub_821F3D58(ctx, base);
	// 832770A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832770A4: 906AD55C  stw r3, -0x2aa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10916 as u32), ctx.r[3].u32 ) };
	// 832770A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832770AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832770B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832770B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832770B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832770B8 size=56
    let mut pc: u32 = 0x832770B8;
    'dispatch: loop {
        match pc {
            0x832770B8 => {
    //   block [0x832770B8..0x832770F0)
	// 832770B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832770BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832770C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832770C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832770C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832770CC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832770D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832770D4: 4AF7CC85  bl 0x821f3d58
	ctx.lr = 0x832770D8;
	sub_821F3D58(ctx, base);
	// 832770D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832770DC: 906AD560  stw r3, -0x2aa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10912 as u32), ctx.r[3].u32 ) };
	// 832770E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832770E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832770E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832770EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832770F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832770F0 size=56
    let mut pc: u32 = 0x832770F0;
    'dispatch: loop {
        match pc {
            0x832770F0 => {
    //   block [0x832770F0..0x83277128)
	// 832770F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832770F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832770F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832770FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277100: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277104: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83277108: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327710C: 4AF7CC4D  bl 0x821f3d58
	ctx.lr = 0x83277110;
	sub_821F3D58(ctx, base);
	// 83277110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277114: 906AD564  stw r3, -0x2a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10908 as u32), ctx.r[3].u32 ) };
	// 83277118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327711C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277128 size=56
    let mut pc: u32 = 0x83277128;
    'dispatch: loop {
        match pc {
            0x83277128 => {
    //   block [0x83277128..0x83277160)
	// 83277128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277134: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277138: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327713C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83277140: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277144: 4AF7CC15  bl 0x821f3d58
	ctx.lr = 0x83277148;
	sub_821F3D58(ctx, base);
	// 83277148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327714C: 906AD568  stw r3, -0x2a98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10904 as u32), ctx.r[3].u32 ) };
	// 83277150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327715C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277160 size=56
    let mut pc: u32 = 0x83277160;
    'dispatch: loop {
        match pc {
            0x83277160 => {
    //   block [0x83277160..0x83277198)
	// 83277160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327716C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277170: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277174: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83277178: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327717C: 4AF7CBDD  bl 0x821f3d58
	ctx.lr = 0x83277180;
	sub_821F3D58(ctx, base);
	// 83277180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277184: 906AD56C  stw r3, -0x2a94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10900 as u32), ctx.r[3].u32 ) };
	// 83277188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327718C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277198 size=56
    let mut pc: u32 = 0x83277198;
    'dispatch: loop {
        match pc {
            0x83277198 => {
    //   block [0x83277198..0x832771D0)
	// 83277198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327719C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832771A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832771A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832771A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832771AC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832771B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832771B4: 4AF7CBA5  bl 0x821f3d58
	ctx.lr = 0x832771B8;
	sub_821F3D58(ctx, base);
	// 832771B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832771BC: 906AD570  stw r3, -0x2a90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10896 as u32), ctx.r[3].u32 ) };
	// 832771C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832771C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832771C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832771CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832771D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832771D0 size=56
    let mut pc: u32 = 0x832771D0;
    'dispatch: loop {
        match pc {
            0x832771D0 => {
    //   block [0x832771D0..0x83277208)
	// 832771D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832771D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832771D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832771DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832771E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832771E4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832771E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832771EC: 4AF7CB6D  bl 0x821f3d58
	ctx.lr = 0x832771F0;
	sub_821F3D58(ctx, base);
	// 832771F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832771F4: 906AD574  stw r3, -0x2a8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10892 as u32), ctx.r[3].u32 ) };
	// 832771F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832771FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277208 size=56
    let mut pc: u32 = 0x83277208;
    'dispatch: loop {
        match pc {
            0x83277208 => {
    //   block [0x83277208..0x83277240)
	// 83277208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277214: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277218: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327721C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83277220: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277224: 4AF7CB35  bl 0x821f3d58
	ctx.lr = 0x83277228;
	sub_821F3D58(ctx, base);
	// 83277228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327722C: 906AD578  stw r3, -0x2a88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10888 as u32), ctx.r[3].u32 ) };
	// 83277230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327723C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277240 size=56
    let mut pc: u32 = 0x83277240;
    'dispatch: loop {
        match pc {
            0x83277240 => {
    //   block [0x83277240..0x83277278)
	// 83277240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327724C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277250: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277254: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83277258: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327725C: 4AF7CAFD  bl 0x821f3d58
	ctx.lr = 0x83277260;
	sub_821F3D58(ctx, base);
	// 83277260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277264: 906AD57C  stw r3, -0x2a84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10884 as u32), ctx.r[3].u32 ) };
	// 83277268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327726C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277278 size=56
    let mut pc: u32 = 0x83277278;
    'dispatch: loop {
        match pc {
            0x83277278 => {
    //   block [0x83277278..0x832772B0)
	// 83277278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83277284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277288: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327728C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83277290: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83277294: 4AF7CAC5  bl 0x821f3d58
	ctx.lr = 0x83277298;
	sub_821F3D58(ctx, base);
	// 83277298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327729C: 906AD580  stw r3, -0x2a80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10880 as u32), ctx.r[3].u32 ) };
	// 832772A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832772A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832772A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832772AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832772B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832772B0 size=64
    let mut pc: u32 = 0x832772B0;
    'dispatch: loop {
        match pc {
            0x832772B0 => {
    //   block [0x832772B0..0x832772F0)
	// 832772B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832772B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832772B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832772BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832772C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832772C4: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 832772C8: 386AD584  addi r3, r10, -0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -10876;
	// 832772CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832772D0: 4AFB5C01  bl 0x8222ced0
	ctx.lr = 0x832772D4;
	sub_8222CED0(ctx, base);
	// 832772D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832772D8: 3869FBC0  addi r3, r9, -0x440
	ctx.r[3].s64 = ctx.r[9].s64 + -1088;
	// 832772DC: 4BA32C45  bl 0x82ca9f20
	ctx.lr = 0x832772E0;
	sub_82CA9F20(ctx, base);
	// 832772E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832772E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832772E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832772EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832772F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832772F0 size=64
    let mut pc: u32 = 0x832772F0;
    'dispatch: loop {
        match pc {
            0x832772F0 => {
    //   block [0x832772F0..0x83277330)
	// 832772F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832772F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832772F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832772FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277304: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 83277308: 386AD588  addi r3, r10, -0x2a78
	ctx.r[3].s64 = ctx.r[10].s64 + -10872;
	// 8327730C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277310: 4AFB5BC1  bl 0x8222ced0
	ctx.lr = 0x83277314;
	sub_8222CED0(ctx, base);
	// 83277314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277318: 3869FBD0  addi r3, r9, -0x430
	ctx.r[3].s64 = ctx.r[9].s64 + -1072;
	// 8327731C: 4BA32C05  bl 0x82ca9f20
	ctx.lr = 0x83277320;
	sub_82CA9F20(ctx, base);
	// 83277320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327732C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277330 size=64
    let mut pc: u32 = 0x83277330;
    'dispatch: loop {
        match pc {
            0x83277330 => {
    //   block [0x83277330..0x83277370)
	// 83277330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327733C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277344: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 83277348: 386AD58C  addi r3, r10, -0x2a74
	ctx.r[3].s64 = ctx.r[10].s64 + -10868;
	// 8327734C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277350: 4AFB5B81  bl 0x8222ced0
	ctx.lr = 0x83277354;
	sub_8222CED0(ctx, base);
	// 83277354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277358: 3869FBE0  addi r3, r9, -0x420
	ctx.r[3].s64 = ctx.r[9].s64 + -1056;
	// 8327735C: 4BA32BC5  bl 0x82ca9f20
	ctx.lr = 0x83277360;
	sub_82CA9F20(ctx, base);
	// 83277360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83277364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327736C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83277370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83277370 size=64
    let mut pc: u32 = 0x83277370;
    'dispatch: loop {
        match pc {
            0x83277370 => {
    //   block [0x83277370..0x832773B0)
	// 83277370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83277374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83277378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327737C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83277380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277384: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 83277388: 386AD590  addi r3, r10, -0x2a70
	ctx.r[3].s64 = ctx.r[10].s64 + -10864;
	// 8327738C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83277390: 4AFB5B41  bl 0x8222ced0
	ctx.lr = 0x83277394;
	sub_8222CED0(ctx, base);
	// 83277394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83277398: 3869FBF0  addi r3, r9, -0x410
	ctx.r[3].s64 = ctx.r[9].s64 + -1040;
	// 8327739C: 4BA32B85  bl 0x82ca9f20
	ctx.lr = 0x832773A0;
	sub_82CA9F20(ctx, base);
	// 832773A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832773A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832773A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832773AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832773B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832773B0 size=64
    let mut pc: u32 = 0x832773B0;
    'dispatch: loop {
        match pc {
            0x832773B0 => {
    //   block [0x832773B0..0x832773F0)
	// 832773B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832773B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832773B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832773BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832773C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832773C4: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 832773C8: 386AD594  addi r3, r10, -0x2a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -10860;
	// 832773CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832773D0: 4AFB5B01  bl 0x8222ced0
	ctx.lr = 0x832773D4;
	sub_8222CED0(ctx, base);
	// 832773D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832773D8: 3869FC00  addi r3, r9, -0x400
	ctx.r[3].s64 = ctx.r[9].s64 + -1024;
	// 832773DC: 4BA32B45  bl 0x82ca9f20
	ctx.lr = 0x832773E0;
	sub_82CA9F20(ctx, base);
	// 832773E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832773E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832773E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832773EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832773F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832773F0 size=56
    let mut pc: u32 = 0x832773F0;
    'dispatch: loop {
        match pc {
            0x832773F0 => {
    //   block [0x832773F0..0x83277428)
	// 832773F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832773F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832773F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832773FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83277400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83277404: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83277408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327740C: 4AF7C94D  bl 0x821f3d58
	ctx.lr = 0x83277410;
	sub_821F3D58(ctx, base);
	// 83277410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83277414: 906AD598  stw r3, -0x2a68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10856 as u32), ctx.r[3].u32 ) };
	// 83277418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327741C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83277420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83277424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


