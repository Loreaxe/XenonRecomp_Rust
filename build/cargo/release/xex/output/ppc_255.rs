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
    //   block [0x83283828..0x832838B8)
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
    //   block [0x83284670..0x83284700)
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


pub fn sub_832850D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832850D0 size=64
    let mut pc: u32 = 0x832850D0;
    'dispatch: loop {
        match pc {
            0x832850D0 => {
    //   block [0x832850D0..0x83285110)
	// 832850D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832850D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832850D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832850DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832850E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832850E4: 388BB0E0  addi r4, r11, -0x4f20
	ctx.r[4].s64 = ctx.r[11].s64 + -20256;
	// 832850E8: 386AE5BC  addi r3, r10, -0x1a44
	ctx.r[3].s64 = ctx.r[10].s64 + -6724;
	// 832850EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832850F0: 4AFA7DE1  bl 0x8222ced0
	ctx.lr = 0x832850F4;
	sub_8222CED0(ctx, base);
	// 832850F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832850F8: 38691940  addi r3, r9, 0x1940
	ctx.r[3].s64 = ctx.r[9].s64 + 6464;
	// 832850FC: 4BA24E25  bl 0x82ca9f20
	ctx.lr = 0x83285100;
	sub_82CA9F20(ctx, base);
	// 83285100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285110 size=64
    let mut pc: u32 = 0x83285110;
    'dispatch: loop {
        match pc {
            0x83285110 => {
    //   block [0x83285110..0x83285150)
	// 83285110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328511C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285124: 388BB104  addi r4, r11, -0x4efc
	ctx.r[4].s64 = ctx.r[11].s64 + -20220;
	// 83285128: 386AE5C0  addi r3, r10, -0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + -6720;
	// 8328512C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285130: 4AFA7DA1  bl 0x8222ced0
	ctx.lr = 0x83285134;
	sub_8222CED0(ctx, base);
	// 83285134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285138: 38691950  addi r3, r9, 0x1950
	ctx.r[3].s64 = ctx.r[9].s64 + 6480;
	// 8328513C: 4BA24DE5  bl 0x82ca9f20
	ctx.lr = 0x83285140;
	sub_82CA9F20(ctx, base);
	// 83285140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328514C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83285150 size=20
    let mut pc: u32 = 0x83285150;
    'dispatch: loop {
        match pc {
            0x83285150 => {
    //   block [0x83285150..0x83285164)
	// 83285150: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285154: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83285158: 394BE5D0  addi r10, r11, -0x1a30
	ctx.r[10].s64 = ctx.r[11].s64 + -6704;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83285168 size=20
    let mut pc: u32 = 0x83285168;
    'dispatch: loop {
        match pc {
            0x83285168 => {
    //   block [0x83285168..0x8328517C)
	// 83285168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328516C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83285170: 394BE5E0  addi r10, r11, -0x1a20
	ctx.r[10].s64 = ctx.r[11].s64 + -6688;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83285180 size=12
    let mut pc: u32 = 0x83285180;
    'dispatch: loop {
        match pc {
            0x83285180 => {
    //   block [0x83285180..0x8328518C)
	// 83285180: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285184: 386BE5F0  addi r3, r11, -0x1a10
	ctx.r[3].s64 = ctx.r[11].s64 + -6672;
	// 83285188: 4AFB6C28  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285190 size=64
    let mut pc: u32 = 0x83285190;
    'dispatch: loop {
        match pc {
            0x83285190 => {
    //   block [0x83285190..0x832851D0)
	// 83285190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328519C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832851A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832851A4: 388BB074  addi r4, r11, -0x4f8c
	ctx.r[4].s64 = ctx.r[11].s64 + -20364;
	// 832851A8: 386AE630  addi r3, r10, -0x19d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6608;
	// 832851AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832851B0: 4AFA7D21  bl 0x8222ced0
	ctx.lr = 0x832851B4;
	sub_8222CED0(ctx, base);
	// 832851B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832851B8: 38691AC0  addi r3, r9, 0x1ac0
	ctx.r[3].s64 = ctx.r[9].s64 + 6848;
	// 832851BC: 4BA24D65  bl 0x82ca9f20
	ctx.lr = 0x832851C0;
	sub_82CA9F20(ctx, base);
	// 832851C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832851C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832851C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832851CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832851D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832851D0 size=64
    let mut pc: u32 = 0x832851D0;
    'dispatch: loop {
        match pc {
            0x832851D0 => {
    //   block [0x832851D0..0x83285210)
	// 832851D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832851D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832851D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832851DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832851E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832851E4: 388BB088  addi r4, r11, -0x4f78
	ctx.r[4].s64 = ctx.r[11].s64 + -20344;
	// 832851E8: 386AE634  addi r3, r10, -0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6604;
	// 832851EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832851F0: 4AFA7CE1  bl 0x8222ced0
	ctx.lr = 0x832851F4;
	sub_8222CED0(ctx, base);
	// 832851F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832851F8: 38691AD0  addi r3, r9, 0x1ad0
	ctx.r[3].s64 = ctx.r[9].s64 + 6864;
	// 832851FC: 4BA24D25  bl 0x82ca9f20
	ctx.lr = 0x83285200;
	sub_82CA9F20(ctx, base);
	// 83285200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328520C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285210 size=64
    let mut pc: u32 = 0x83285210;
    'dispatch: loop {
        match pc {
            0x83285210 => {
    //   block [0x83285210..0x83285250)
	// 83285210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328521C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285224: 388BB098  addi r4, r11, -0x4f68
	ctx.r[4].s64 = ctx.r[11].s64 + -20328;
	// 83285228: 386AE638  addi r3, r10, -0x19c8
	ctx.r[3].s64 = ctx.r[10].s64 + -6600;
	// 8328522C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285230: 4AFA7CA1  bl 0x8222ced0
	ctx.lr = 0x83285234;
	sub_8222CED0(ctx, base);
	// 83285234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285238: 38691AE0  addi r3, r9, 0x1ae0
	ctx.r[3].s64 = ctx.r[9].s64 + 6880;
	// 8328523C: 4BA24CE5  bl 0x82ca9f20
	ctx.lr = 0x83285240;
	sub_82CA9F20(ctx, base);
	// 83285240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328524C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285250 size=52
    let mut pc: u32 = 0x83285250;
    'dispatch: loop {
        match pc {
            0x83285250 => {
    //   block [0x83285250..0x83285284)
	// 83285250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328525C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285260: 386BE63C  addi r3, r11, -0x19c4
	ctx.r[3].s64 = ctx.r[11].s64 + -6596;
	// 83285264: 4BFBBD95  bl 0x83240ff8
	ctx.lr = 0x83285268;
	sub_83240FF8(ctx, base);
	// 83285268: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328526C: 386A1AF0  addi r3, r10, 0x1af0
	ctx.r[3].s64 = ctx.r[10].s64 + 6896;
	// 83285270: 4BA24CB1  bl 0x82ca9f20
	ctx.lr = 0x83285274;
	sub_82CA9F20(ctx, base);
	// 83285274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328527C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285288 size=64
    let mut pc: u32 = 0x83285288;
    'dispatch: loop {
        match pc {
            0x83285288 => {
    //   block [0x83285288..0x832852C8)
	// 83285288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328528C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328529C: 388BB038  addi r4, r11, -0x4fc8
	ctx.r[4].s64 = ctx.r[11].s64 + -20424;
	// 832852A0: 386AE64C  addi r3, r10, -0x19b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6580;
	// 832852A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832852A8: 4AFA7C29  bl 0x8222ced0
	ctx.lr = 0x832852AC;
	sub_8222CED0(ctx, base);
	// 832852AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832852B0: 38691B38  addi r3, r9, 0x1b38
	ctx.r[3].s64 = ctx.r[9].s64 + 6968;
	// 832852B4: 4BA24C6D  bl 0x82ca9f20
	ctx.lr = 0x832852B8;
	sub_82CA9F20(ctx, base);
	// 832852B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832852BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832852C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832852C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832852C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832852C8 size=64
    let mut pc: u32 = 0x832852C8;
    'dispatch: loop {
        match pc {
            0x832852C8 => {
    //   block [0x832852C8..0x83285308)
	// 832852C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832852CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832852D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832852D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832852D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832852DC: 388BB054  addi r4, r11, -0x4fac
	ctx.r[4].s64 = ctx.r[11].s64 + -20396;
	// 832852E0: 386AE650  addi r3, r10, -0x19b0
	ctx.r[3].s64 = ctx.r[10].s64 + -6576;
	// 832852E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832852E8: 4AFA7BE9  bl 0x8222ced0
	ctx.lr = 0x832852EC;
	sub_8222CED0(ctx, base);
	// 832852EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832852F0: 38691B48  addi r3, r9, 0x1b48
	ctx.r[3].s64 = ctx.r[9].s64 + 6984;
	// 832852F4: 4BA24C2D  bl 0x82ca9f20
	ctx.lr = 0x832852F8;
	sub_82CA9F20(ctx, base);
	// 832852F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832852FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285308 size=64
    let mut pc: u32 = 0x83285308;
    'dispatch: loop {
        match pc {
            0x83285308 => {
    //   block [0x83285308..0x83285348)
	// 83285308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328530C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328531C: 388BB06C  addi r4, r11, -0x4f94
	ctx.r[4].s64 = ctx.r[11].s64 + -20372;
	// 83285320: 386AE654  addi r3, r10, -0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6572;
	// 83285324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285328: 4AFA7BA9  bl 0x8222ced0
	ctx.lr = 0x8328532C;
	sub_8222CED0(ctx, base);
	// 8328532C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285330: 38691B58  addi r3, r9, 0x1b58
	ctx.r[3].s64 = ctx.r[9].s64 + 7000;
	// 83285334: 4BA24BED  bl 0x82ca9f20
	ctx.lr = 0x83285338;
	sub_82CA9F20(ctx, base);
	// 83285338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285348 size=52
    let mut pc: u32 = 0x83285348;
    'dispatch: loop {
        match pc {
            0x83285348 => {
    //   block [0x83285348..0x8328537C)
	// 83285348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285354: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285358: 386BE658  addi r3, r11, -0x19a8
	ctx.r[3].s64 = ctx.r[11].s64 + -6568;
	// 8328535C: 4BFBBC9D  bl 0x83240ff8
	ctx.lr = 0x83285360;
	sub_83240FF8(ctx, base);
	// 83285360: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83285364: 386A1B68  addi r3, r10, 0x1b68
	ctx.r[3].s64 = ctx.r[10].s64 + 7016;
	// 83285368: 4BA24BB9  bl 0x82ca9f20
	ctx.lr = 0x8328536C;
	sub_82CA9F20(ctx, base);
	// 8328536C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285380 size=64
    let mut pc: u32 = 0x83285380;
    'dispatch: loop {
        match pc {
            0x83285380 => {
    //   block [0x83285380..0x832853C0)
	// 83285380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328538C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285394: 388BAFAC  addi r4, r11, -0x5054
	ctx.r[4].s64 = ctx.r[11].s64 + -20564;
	// 83285398: 386AE668  addi r3, r10, -0x1998
	ctx.r[3].s64 = ctx.r[10].s64 + -6552;
	// 8328539C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832853A0: 4AFA7B31  bl 0x8222ced0
	ctx.lr = 0x832853A4;
	sub_8222CED0(ctx, base);
	// 832853A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832853A8: 38691BB0  addi r3, r9, 0x1bb0
	ctx.r[3].s64 = ctx.r[9].s64 + 7088;
	// 832853AC: 4BA24B75  bl 0x82ca9f20
	ctx.lr = 0x832853B0;
	sub_82CA9F20(ctx, base);
	// 832853B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832853B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832853B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832853BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832853C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832853C0 size=64
    let mut pc: u32 = 0x832853C0;
    'dispatch: loop {
        match pc {
            0x832853C0 => {
    //   block [0x832853C0..0x83285400)
	// 832853C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832853C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832853C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832853CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832853D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832853D4: 388BAFB4  addi r4, r11, -0x504c
	ctx.r[4].s64 = ctx.r[11].s64 + -20556;
	// 832853D8: 386AE66C  addi r3, r10, -0x1994
	ctx.r[3].s64 = ctx.r[10].s64 + -6548;
	// 832853DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832853E0: 4AFA7AF1  bl 0x8222ced0
	ctx.lr = 0x832853E4;
	sub_8222CED0(ctx, base);
	// 832853E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832853E8: 38691BC0  addi r3, r9, 0x1bc0
	ctx.r[3].s64 = ctx.r[9].s64 + 7104;
	// 832853EC: 4BA24B35  bl 0x82ca9f20
	ctx.lr = 0x832853F0;
	sub_82CA9F20(ctx, base);
	// 832853F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832853F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832853F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832853FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285400 size=64
    let mut pc: u32 = 0x83285400;
    'dispatch: loop {
        match pc {
            0x83285400 => {
    //   block [0x83285400..0x83285440)
	// 83285400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328540C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285414: 388BAFC8  addi r4, r11, -0x5038
	ctx.r[4].s64 = ctx.r[11].s64 + -20536;
	// 83285418: 386AE670  addi r3, r10, -0x1990
	ctx.r[3].s64 = ctx.r[10].s64 + -6544;
	// 8328541C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285420: 4AFA7AB1  bl 0x8222ced0
	ctx.lr = 0x83285424;
	sub_8222CED0(ctx, base);
	// 83285424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285428: 38691BD0  addi r3, r9, 0x1bd0
	ctx.r[3].s64 = ctx.r[9].s64 + 7120;
	// 8328542C: 4BA24AF5  bl 0x82ca9f20
	ctx.lr = 0x83285430;
	sub_82CA9F20(ctx, base);
	// 83285430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328543C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285440 size=64
    let mut pc: u32 = 0x83285440;
    'dispatch: loop {
        match pc {
            0x83285440 => {
    //   block [0x83285440..0x83285480)
	// 83285440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328544C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285454: 388BAFDC  addi r4, r11, -0x5024
	ctx.r[4].s64 = ctx.r[11].s64 + -20516;
	// 83285458: 386AE674  addi r3, r10, -0x198c
	ctx.r[3].s64 = ctx.r[10].s64 + -6540;
	// 8328545C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285460: 4AFA7A71  bl 0x8222ced0
	ctx.lr = 0x83285464;
	sub_8222CED0(ctx, base);
	// 83285464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285468: 38691BE0  addi r3, r9, 0x1be0
	ctx.r[3].s64 = ctx.r[9].s64 + 7136;
	// 8328546C: 4BA24AB5  bl 0x82ca9f20
	ctx.lr = 0x83285470;
	sub_82CA9F20(ctx, base);
	// 83285470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328547C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285480 size=128
    let mut pc: u32 = 0x83285480;
    'dispatch: loop {
        match pc {
            0x83285480 => {
    //   block [0x83285480..0x83285500)
	// 83285480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328548C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83285494: 388BE674  addi r4, r11, -0x198c
	ctx.r[4].s64 = ctx.r[11].s64 + -6540;
	// 83285498: 4AF696A1  bl 0x821eeb38
	ctx.lr = 0x8328549C;
	sub_821EEB38(ctx, base);
	// 8328549C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832854A0: 4B97E351  bl 0x82c037f0
	ctx.lr = 0x832854A4;
	sub_82C037F0(ctx, base);
	// 832854A4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832854A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832854AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832854B0: 916AE678  stw r11, -0x1988(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6536 as u32), ctx.r[11].u32 ) };
	// 832854B4: 4AF412B5  bl 0x821c6768
	ctx.lr = 0x832854B8;
	sub_821C6768(ctx, base);
	// 832854B8: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832854BC: 38C97088  addi r6, r9, 0x7088
	ctx.r[6].s64 = ctx.r[9].s64 + 28808;
	// 832854C0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832854C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832854C8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832854CC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832854D0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832854D4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832854D8: 4082FFE8  bne 0x832854c0
	if !ctx.cr[0].eq {
	pc = 0x832854C0; continue 'dispatch;
	}
	// 832854DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832854E0: 3C80832B  lis r4, -0x7cd5
	ctx.r[4].s64 = -2094333952;
	// 832854E4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832854E8: 38641BF0  addi r3, r4, 0x1bf0
	ctx.r[3].s64 = ctx.r[4].s64 + 7152;
	// 832854EC: 4BA24A35  bl 0x82ca9f20
	ctx.lr = 0x832854F0;
	sub_82CA9F20(ctx, base);
	// 832854F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832854F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832854F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832854FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285500 size=64
    let mut pc: u32 = 0x83285500;
    'dispatch: loop {
        match pc {
            0x83285500 => {
    //   block [0x83285500..0x83285540)
	// 83285500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328550C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285514: 388BAFF4  addi r4, r11, -0x500c
	ctx.r[4].s64 = ctx.r[11].s64 + -20492;
	// 83285518: 386AE67C  addi r3, r10, -0x1984
	ctx.r[3].s64 = ctx.r[10].s64 + -6532;
	// 8328551C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285520: 4AFA79B1  bl 0x8222ced0
	ctx.lr = 0x83285524;
	sub_8222CED0(ctx, base);
	// 83285524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285528: 38691BF8  addi r3, r9, 0x1bf8
	ctx.r[3].s64 = ctx.r[9].s64 + 7160;
	// 8328552C: 4BA249F5  bl 0x82ca9f20
	ctx.lr = 0x83285530;
	sub_82CA9F20(ctx, base);
	// 83285530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328553C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285540 size=64
    let mut pc: u32 = 0x83285540;
    'dispatch: loop {
        match pc {
            0x83285540 => {
    //   block [0x83285540..0x83285580)
	// 83285540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328554C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285554: 388BB018  addi r4, r11, -0x4fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -20456;
	// 83285558: 386AE680  addi r3, r10, -0x1980
	ctx.r[3].s64 = ctx.r[10].s64 + -6528;
	// 8328555C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285560: 4AFA7971  bl 0x8222ced0
	ctx.lr = 0x83285564;
	sub_8222CED0(ctx, base);
	// 83285564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285568: 38691C08  addi r3, r9, 0x1c08
	ctx.r[3].s64 = ctx.r[9].s64 + 7176;
	// 8328556C: 4BA249B5  bl 0x82ca9f20
	ctx.lr = 0x83285570;
	sub_82CA9F20(ctx, base);
	// 83285570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328557C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285580 size=64
    let mut pc: u32 = 0x83285580;
    'dispatch: loop {
        match pc {
            0x83285580 => {
    //   block [0x83285580..0x832855C0)
	// 83285580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328558C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285594: 388BB030  addi r4, r11, -0x4fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -20432;
	// 83285598: 386AE684  addi r3, r10, -0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + -6524;
	// 8328559C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832855A0: 4AFA7931  bl 0x8222ced0
	ctx.lr = 0x832855A4;
	sub_8222CED0(ctx, base);
	// 832855A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832855A8: 38691C18  addi r3, r9, 0x1c18
	ctx.r[3].s64 = ctx.r[9].s64 + 7192;
	// 832855AC: 4BA24975  bl 0x82ca9f20
	ctx.lr = 0x832855B0;
	sub_82CA9F20(ctx, base);
	// 832855B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832855B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832855B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832855BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832855C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832855C0 size=52
    let mut pc: u32 = 0x832855C0;
    'dispatch: loop {
        match pc {
            0x832855C0 => {
    //   block [0x832855C0..0x832855F4)
	// 832855C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832855C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832855C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832855CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832855D0: 386BE688  addi r3, r11, -0x1978
	ctx.r[3].s64 = ctx.r[11].s64 + -6520;
	// 832855D4: 4BFBBA25  bl 0x83240ff8
	ctx.lr = 0x832855D8;
	sub_83240FF8(ctx, base);
	// 832855D8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832855DC: 386A1C28  addi r3, r10, 0x1c28
	ctx.r[3].s64 = ctx.r[10].s64 + 7208;
	// 832855E0: 4BA24941  bl 0x82ca9f20
	ctx.lr = 0x832855E4;
	sub_82CA9F20(ctx, base);
	// 832855E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832855E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832855EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832855F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832855F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832855F8 size=64
    let mut pc: u32 = 0x832855F8;
    'dispatch: loop {
        match pc {
            0x832855F8 => {
    //   block [0x832855F8..0x83285638)
	// 832855F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832855FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328560C: 388BAE48  addi r4, r11, -0x51b8
	ctx.r[4].s64 = ctx.r[11].s64 + -20920;
	// 83285610: 386AE698  addi r3, r10, -0x1968
	ctx.r[3].s64 = ctx.r[10].s64 + -6504;
	// 83285614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285618: 4AFA78B9  bl 0x8222ced0
	ctx.lr = 0x8328561C;
	sub_8222CED0(ctx, base);
	// 8328561C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285620: 38691C70  addi r3, r9, 0x1c70
	ctx.r[3].s64 = ctx.r[9].s64 + 7280;
	// 83285624: 4BA248FD  bl 0x82ca9f20
	ctx.lr = 0x83285628;
	sub_82CA9F20(ctx, base);
	// 83285628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328562C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285638 size=64
    let mut pc: u32 = 0x83285638;
    'dispatch: loop {
        match pc {
            0x83285638 => {
    //   block [0x83285638..0x83285678)
	// 83285638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328563C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328564C: 388BAE50  addi r4, r11, -0x51b0
	ctx.r[4].s64 = ctx.r[11].s64 + -20912;
	// 83285650: 386AE69C  addi r3, r10, -0x1964
	ctx.r[3].s64 = ctx.r[10].s64 + -6500;
	// 83285654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285658: 4AFA7879  bl 0x8222ced0
	ctx.lr = 0x8328565C;
	sub_8222CED0(ctx, base);
	// 8328565C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285660: 38691C80  addi r3, r9, 0x1c80
	ctx.r[3].s64 = ctx.r[9].s64 + 7296;
	// 83285664: 4BA248BD  bl 0x82ca9f20
	ctx.lr = 0x83285668;
	sub_82CA9F20(ctx, base);
	// 83285668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328566C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285678 size=64
    let mut pc: u32 = 0x83285678;
    'dispatch: loop {
        match pc {
            0x83285678 => {
    //   block [0x83285678..0x832856B8)
	// 83285678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328567C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328568C: 388BAE64  addi r4, r11, -0x519c
	ctx.r[4].s64 = ctx.r[11].s64 + -20892;
	// 83285690: 386AE6A0  addi r3, r10, -0x1960
	ctx.r[3].s64 = ctx.r[10].s64 + -6496;
	// 83285694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285698: 4AFA7839  bl 0x8222ced0
	ctx.lr = 0x8328569C;
	sub_8222CED0(ctx, base);
	// 8328569C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832856A0: 38691C90  addi r3, r9, 0x1c90
	ctx.r[3].s64 = ctx.r[9].s64 + 7312;
	// 832856A4: 4BA2487D  bl 0x82ca9f20
	ctx.lr = 0x832856A8;
	sub_82CA9F20(ctx, base);
	// 832856A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832856AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832856B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832856B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832856B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832856B8 size=64
    let mut pc: u32 = 0x832856B8;
    'dispatch: loop {
        match pc {
            0x832856B8 => {
    //   block [0x832856B8..0x832856F8)
	// 832856B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832856BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832856C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832856C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832856C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832856CC: 388BAE78  addi r4, r11, -0x5188
	ctx.r[4].s64 = ctx.r[11].s64 + -20872;
	// 832856D0: 386AE6A4  addi r3, r10, -0x195c
	ctx.r[3].s64 = ctx.r[10].s64 + -6492;
	// 832856D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832856D8: 4AFA77F9  bl 0x8222ced0
	ctx.lr = 0x832856DC;
	sub_8222CED0(ctx, base);
	// 832856DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832856E0: 38691CA0  addi r3, r9, 0x1ca0
	ctx.r[3].s64 = ctx.r[9].s64 + 7328;
	// 832856E4: 4BA2483D  bl 0x82ca9f20
	ctx.lr = 0x832856E8;
	sub_82CA9F20(ctx, base);
	// 832856E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832856EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832856F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832856F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832856F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832856F8 size=64
    let mut pc: u32 = 0x832856F8;
    'dispatch: loop {
        match pc {
            0x832856F8 => {
    //   block [0x832856F8..0x83285738)
	// 832856F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832856FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328570C: 388BAE9C  addi r4, r11, -0x5164
	ctx.r[4].s64 = ctx.r[11].s64 + -20836;
	// 83285710: 386AE6A8  addi r3, r10, -0x1958
	ctx.r[3].s64 = ctx.r[10].s64 + -6488;
	// 83285714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285718: 4AFA77B9  bl 0x8222ced0
	ctx.lr = 0x8328571C;
	sub_8222CED0(ctx, base);
	// 8328571C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285720: 38691CB0  addi r3, r9, 0x1cb0
	ctx.r[3].s64 = ctx.r[9].s64 + 7344;
	// 83285724: 4BA247FD  bl 0x82ca9f20
	ctx.lr = 0x83285728;
	sub_82CA9F20(ctx, base);
	// 83285728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328572C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285738 size=64
    let mut pc: u32 = 0x83285738;
    'dispatch: loop {
        match pc {
            0x83285738 => {
    //   block [0x83285738..0x83285778)
	// 83285738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328574C: 388BAEA4  addi r4, r11, -0x515c
	ctx.r[4].s64 = ctx.r[11].s64 + -20828;
	// 83285750: 386AE6AC  addi r3, r10, -0x1954
	ctx.r[3].s64 = ctx.r[10].s64 + -6484;
	// 83285754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285758: 4AFA7779  bl 0x8222ced0
	ctx.lr = 0x8328575C;
	sub_8222CED0(ctx, base);
	// 8328575C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285760: 38691CC0  addi r3, r9, 0x1cc0
	ctx.r[3].s64 = ctx.r[9].s64 + 7360;
	// 83285764: 4BA247BD  bl 0x82ca9f20
	ctx.lr = 0x83285768;
	sub_82CA9F20(ctx, base);
	// 83285768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328576C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285778 size=64
    let mut pc: u32 = 0x83285778;
    'dispatch: loop {
        match pc {
            0x83285778 => {
    //   block [0x83285778..0x832857B8)
	// 83285778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285784: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285788: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328578C: 388BAEC8  addi r4, r11, -0x5138
	ctx.r[4].s64 = ctx.r[11].s64 + -20792;
	// 83285790: 386AE6B0  addi r3, r10, -0x1950
	ctx.r[3].s64 = ctx.r[10].s64 + -6480;
	// 83285794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285798: 4AFA7739  bl 0x8222ced0
	ctx.lr = 0x8328579C;
	sub_8222CED0(ctx, base);
	// 8328579C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832857A0: 38691CD0  addi r3, r9, 0x1cd0
	ctx.r[3].s64 = ctx.r[9].s64 + 7376;
	// 832857A4: 4BA2477D  bl 0x82ca9f20
	ctx.lr = 0x832857A8;
	sub_82CA9F20(ctx, base);
	// 832857A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832857AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832857B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832857B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832857B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832857B8 size=64
    let mut pc: u32 = 0x832857B8;
    'dispatch: loop {
        match pc {
            0x832857B8 => {
    //   block [0x832857B8..0x832857F8)
	// 832857B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832857BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832857C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832857C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832857C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832857CC: 388BAEEC  addi r4, r11, -0x5114
	ctx.r[4].s64 = ctx.r[11].s64 + -20756;
	// 832857D0: 386AE6B4  addi r3, r10, -0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + -6476;
	// 832857D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832857D8: 4AFA76F9  bl 0x8222ced0
	ctx.lr = 0x832857DC;
	sub_8222CED0(ctx, base);
	// 832857DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832857E0: 38691CE0  addi r3, r9, 0x1ce0
	ctx.r[3].s64 = ctx.r[9].s64 + 7392;
	// 832857E4: 4BA2473D  bl 0x82ca9f20
	ctx.lr = 0x832857E8;
	sub_82CA9F20(ctx, base);
	// 832857E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832857EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832857F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832857F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832857F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832857F8 size=64
    let mut pc: u32 = 0x832857F8;
    'dispatch: loop {
        match pc {
            0x832857F8 => {
    //   block [0x832857F8..0x83285838)
	// 832857F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832857FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285808: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328580C: 388BAEF4  addi r4, r11, -0x510c
	ctx.r[4].s64 = ctx.r[11].s64 + -20748;
	// 83285810: 386AE6B8  addi r3, r10, -0x1948
	ctx.r[3].s64 = ctx.r[10].s64 + -6472;
	// 83285814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285818: 4AFA76B9  bl 0x8222ced0
	ctx.lr = 0x8328581C;
	sub_8222CED0(ctx, base);
	// 8328581C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285820: 38691CF0  addi r3, r9, 0x1cf0
	ctx.r[3].s64 = ctx.r[9].s64 + 7408;
	// 83285824: 4BA246FD  bl 0x82ca9f20
	ctx.lr = 0x83285828;
	sub_82CA9F20(ctx, base);
	// 83285828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328582C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285838 size=64
    let mut pc: u32 = 0x83285838;
    'dispatch: loop {
        match pc {
            0x83285838 => {
    //   block [0x83285838..0x83285878)
	// 83285838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328583C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328584C: 388BAF14  addi r4, r11, -0x50ec
	ctx.r[4].s64 = ctx.r[11].s64 + -20716;
	// 83285850: 386AE6BC  addi r3, r10, -0x1944
	ctx.r[3].s64 = ctx.r[10].s64 + -6468;
	// 83285854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285858: 4AFA7679  bl 0x8222ced0
	ctx.lr = 0x8328585C;
	sub_8222CED0(ctx, base);
	// 8328585C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285860: 38691D00  addi r3, r9, 0x1d00
	ctx.r[3].s64 = ctx.r[9].s64 + 7424;
	// 83285864: 4BA246BD  bl 0x82ca9f20
	ctx.lr = 0x83285868;
	sub_82CA9F20(ctx, base);
	// 83285868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328586C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285878 size=64
    let mut pc: u32 = 0x83285878;
    'dispatch: loop {
        match pc {
            0x83285878 => {
    //   block [0x83285878..0x832858B8)
	// 83285878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328588C: 388BAF24  addi r4, r11, -0x50dc
	ctx.r[4].s64 = ctx.r[11].s64 + -20700;
	// 83285890: 386AE6C0  addi r3, r10, -0x1940
	ctx.r[3].s64 = ctx.r[10].s64 + -6464;
	// 83285894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285898: 4AFA7639  bl 0x8222ced0
	ctx.lr = 0x8328589C;
	sub_8222CED0(ctx, base);
	// 8328589C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832858A0: 38691D10  addi r3, r9, 0x1d10
	ctx.r[3].s64 = ctx.r[9].s64 + 7440;
	// 832858A4: 4BA2467D  bl 0x82ca9f20
	ctx.lr = 0x832858A8;
	sub_82CA9F20(ctx, base);
	// 832858A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832858AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832858B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832858B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832858B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832858B8 size=64
    let mut pc: u32 = 0x832858B8;
    'dispatch: loop {
        match pc {
            0x832858B8 => {
    //   block [0x832858B8..0x832858F8)
	// 832858B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832858BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832858C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832858C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832858C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832858CC: 388BAF30  addi r4, r11, -0x50d0
	ctx.r[4].s64 = ctx.r[11].s64 + -20688;
	// 832858D0: 386AE6C4  addi r3, r10, -0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + -6460;
	// 832858D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832858D8: 4AFA75F9  bl 0x8222ced0
	ctx.lr = 0x832858DC;
	sub_8222CED0(ctx, base);
	// 832858DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832858E0: 38691D20  addi r3, r9, 0x1d20
	ctx.r[3].s64 = ctx.r[9].s64 + 7456;
	// 832858E4: 4BA2463D  bl 0x82ca9f20
	ctx.lr = 0x832858E8;
	sub_82CA9F20(ctx, base);
	// 832858E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832858EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832858F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832858F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832858F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832858F8 size=64
    let mut pc: u32 = 0x832858F8;
    'dispatch: loop {
        match pc {
            0x832858F8 => {
    //   block [0x832858F8..0x83285938)
	// 832858F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832858FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328590C: 388BAF3C  addi r4, r11, -0x50c4
	ctx.r[4].s64 = ctx.r[11].s64 + -20676;
	// 83285910: 386AE6C8  addi r3, r10, -0x1938
	ctx.r[3].s64 = ctx.r[10].s64 + -6456;
	// 83285914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285918: 4AFA75B9  bl 0x8222ced0
	ctx.lr = 0x8328591C;
	sub_8222CED0(ctx, base);
	// 8328591C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285920: 38691D30  addi r3, r9, 0x1d30
	ctx.r[3].s64 = ctx.r[9].s64 + 7472;
	// 83285924: 4BA245FD  bl 0x82ca9f20
	ctx.lr = 0x83285928;
	sub_82CA9F20(ctx, base);
	// 83285928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328592C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285938 size=64
    let mut pc: u32 = 0x83285938;
    'dispatch: loop {
        match pc {
            0x83285938 => {
    //   block [0x83285938..0x83285978)
	// 83285938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328594C: 388BAF44  addi r4, r11, -0x50bc
	ctx.r[4].s64 = ctx.r[11].s64 + -20668;
	// 83285950: 386AE6CC  addi r3, r10, -0x1934
	ctx.r[3].s64 = ctx.r[10].s64 + -6452;
	// 83285954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285958: 4AFA7579  bl 0x8222ced0
	ctx.lr = 0x8328595C;
	sub_8222CED0(ctx, base);
	// 8328595C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285960: 38691D40  addi r3, r9, 0x1d40
	ctx.r[3].s64 = ctx.r[9].s64 + 7488;
	// 83285964: 4BA245BD  bl 0x82ca9f20
	ctx.lr = 0x83285968;
	sub_82CA9F20(ctx, base);
	// 83285968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328596C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285978 size=64
    let mut pc: u32 = 0x83285978;
    'dispatch: loop {
        match pc {
            0x83285978 => {
    //   block [0x83285978..0x832859B8)
	// 83285978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328597C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328598C: 388BAF54  addi r4, r11, -0x50ac
	ctx.r[4].s64 = ctx.r[11].s64 + -20652;
	// 83285990: 386AE6D0  addi r3, r10, -0x1930
	ctx.r[3].s64 = ctx.r[10].s64 + -6448;
	// 83285994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285998: 4AFA7539  bl 0x8222ced0
	ctx.lr = 0x8328599C;
	sub_8222CED0(ctx, base);
	// 8328599C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832859A0: 38691D50  addi r3, r9, 0x1d50
	ctx.r[3].s64 = ctx.r[9].s64 + 7504;
	// 832859A4: 4BA2457D  bl 0x82ca9f20
	ctx.lr = 0x832859A8;
	sub_82CA9F20(ctx, base);
	// 832859A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832859AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832859B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832859B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832859B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832859B8 size=64
    let mut pc: u32 = 0x832859B8;
    'dispatch: loop {
        match pc {
            0x832859B8 => {
    //   block [0x832859B8..0x832859F8)
	// 832859B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832859BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832859C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832859C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832859C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832859CC: 388BAF60  addi r4, r11, -0x50a0
	ctx.r[4].s64 = ctx.r[11].s64 + -20640;
	// 832859D0: 386AE6D4  addi r3, r10, -0x192c
	ctx.r[3].s64 = ctx.r[10].s64 + -6444;
	// 832859D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832859D8: 4AFA74F9  bl 0x8222ced0
	ctx.lr = 0x832859DC;
	sub_8222CED0(ctx, base);
	// 832859DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832859E0: 38691D60  addi r3, r9, 0x1d60
	ctx.r[3].s64 = ctx.r[9].s64 + 7520;
	// 832859E4: 4BA2453D  bl 0x82ca9f20
	ctx.lr = 0x832859E8;
	sub_82CA9F20(ctx, base);
	// 832859E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832859EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832859F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832859F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832859F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832859F8 size=64
    let mut pc: u32 = 0x832859F8;
    'dispatch: loop {
        match pc {
            0x832859F8 => {
    //   block [0x832859F8..0x83285A38)
	// 832859F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832859FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285A0C: 388BAF68  addi r4, r11, -0x5098
	ctx.r[4].s64 = ctx.r[11].s64 + -20632;
	// 83285A10: 386AE6D8  addi r3, r10, -0x1928
	ctx.r[3].s64 = ctx.r[10].s64 + -6440;
	// 83285A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285A18: 4AFA74B9  bl 0x8222ced0
	ctx.lr = 0x83285A1C;
	sub_8222CED0(ctx, base);
	// 83285A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285A20: 38691D70  addi r3, r9, 0x1d70
	ctx.r[3].s64 = ctx.r[9].s64 + 7536;
	// 83285A24: 4BA244FD  bl 0x82ca9f20
	ctx.lr = 0x83285A28;
	sub_82CA9F20(ctx, base);
	// 83285A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285A38 size=64
    let mut pc: u32 = 0x83285A38;
    'dispatch: loop {
        match pc {
            0x83285A38 => {
    //   block [0x83285A38..0x83285A78)
	// 83285A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285A4C: 388BAF74  addi r4, r11, -0x508c
	ctx.r[4].s64 = ctx.r[11].s64 + -20620;
	// 83285A50: 386AE6DC  addi r3, r10, -0x1924
	ctx.r[3].s64 = ctx.r[10].s64 + -6436;
	// 83285A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285A58: 4AFA7479  bl 0x8222ced0
	ctx.lr = 0x83285A5C;
	sub_8222CED0(ctx, base);
	// 83285A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285A60: 38691D80  addi r3, r9, 0x1d80
	ctx.r[3].s64 = ctx.r[9].s64 + 7552;
	// 83285A64: 4BA244BD  bl 0x82ca9f20
	ctx.lr = 0x83285A68;
	sub_82CA9F20(ctx, base);
	// 83285A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285A78 size=64
    let mut pc: u32 = 0x83285A78;
    'dispatch: loop {
        match pc {
            0x83285A78 => {
    //   block [0x83285A78..0x83285AB8)
	// 83285A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285A8C: 388BAF80  addi r4, r11, -0x5080
	ctx.r[4].s64 = ctx.r[11].s64 + -20608;
	// 83285A90: 386AE6E0  addi r3, r10, -0x1920
	ctx.r[3].s64 = ctx.r[10].s64 + -6432;
	// 83285A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285A98: 4AFA7439  bl 0x8222ced0
	ctx.lr = 0x83285A9C;
	sub_8222CED0(ctx, base);
	// 83285A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285AA0: 38691D90  addi r3, r9, 0x1d90
	ctx.r[3].s64 = ctx.r[9].s64 + 7568;
	// 83285AA4: 4BA2447D  bl 0x82ca9f20
	ctx.lr = 0x83285AA8;
	sub_82CA9F20(ctx, base);
	// 83285AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285AB8 size=64
    let mut pc: u32 = 0x83285AB8;
    'dispatch: loop {
        match pc {
            0x83285AB8 => {
    //   block [0x83285AB8..0x83285AF8)
	// 83285AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285ACC: 388BADFC  addi r4, r11, -0x5204
	ctx.r[4].s64 = ctx.r[11].s64 + -20996;
	// 83285AD0: 386AE6E4  addi r3, r10, -0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + -6428;
	// 83285AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285AD8: 4AFA73F9  bl 0x8222ced0
	ctx.lr = 0x83285ADC;
	sub_8222CED0(ctx, base);
	// 83285ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285AE0: 38691DA0  addi r3, r9, 0x1da0
	ctx.r[3].s64 = ctx.r[9].s64 + 7584;
	// 83285AE4: 4BA2443D  bl 0x82ca9f20
	ctx.lr = 0x83285AE8;
	sub_82CA9F20(ctx, base);
	// 83285AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285AF8 size=64
    let mut pc: u32 = 0x83285AF8;
    'dispatch: loop {
        match pc {
            0x83285AF8 => {
    //   block [0x83285AF8..0x83285B38)
	// 83285AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285B0C: 388BAE04  addi r4, r11, -0x51fc
	ctx.r[4].s64 = ctx.r[11].s64 + -20988;
	// 83285B10: 386AE6E8  addi r3, r10, -0x1918
	ctx.r[3].s64 = ctx.r[10].s64 + -6424;
	// 83285B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285B18: 4AFA73B9  bl 0x8222ced0
	ctx.lr = 0x83285B1C;
	sub_8222CED0(ctx, base);
	// 83285B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285B20: 38691DB0  addi r3, r9, 0x1db0
	ctx.r[3].s64 = ctx.r[9].s64 + 7600;
	// 83285B24: 4BA243FD  bl 0x82ca9f20
	ctx.lr = 0x83285B28;
	sub_82CA9F20(ctx, base);
	// 83285B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285B38 size=64
    let mut pc: u32 = 0x83285B38;
    'dispatch: loop {
        match pc {
            0x83285B38 => {
    //   block [0x83285B38..0x83285B78)
	// 83285B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285B4C: 388BAE18  addi r4, r11, -0x51e8
	ctx.r[4].s64 = ctx.r[11].s64 + -20968;
	// 83285B50: 386AE6EC  addi r3, r10, -0x1914
	ctx.r[3].s64 = ctx.r[10].s64 + -6420;
	// 83285B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285B58: 4AFA7379  bl 0x8222ced0
	ctx.lr = 0x83285B5C;
	sub_8222CED0(ctx, base);
	// 83285B5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285B60: 38691DC0  addi r3, r9, 0x1dc0
	ctx.r[3].s64 = ctx.r[9].s64 + 7616;
	// 83285B64: 4BA243BD  bl 0x82ca9f20
	ctx.lr = 0x83285B68;
	sub_82CA9F20(ctx, base);
	// 83285B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285B78 size=64
    let mut pc: u32 = 0x83285B78;
    'dispatch: loop {
        match pc {
            0x83285B78 => {
    //   block [0x83285B78..0x83285BB8)
	// 83285B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285B8C: 388BAE2C  addi r4, r11, -0x51d4
	ctx.r[4].s64 = ctx.r[11].s64 + -20948;
	// 83285B90: 386AE6F0  addi r3, r10, -0x1910
	ctx.r[3].s64 = ctx.r[10].s64 + -6416;
	// 83285B94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285B98: 4AFA7339  bl 0x8222ced0
	ctx.lr = 0x83285B9C;
	sub_8222CED0(ctx, base);
	// 83285B9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285BA0: 38691DD0  addi r3, r9, 0x1dd0
	ctx.r[3].s64 = ctx.r[9].s64 + 7632;
	// 83285BA4: 4BA2437D  bl 0x82ca9f20
	ctx.lr = 0x83285BA8;
	sub_82CA9F20(ctx, base);
	// 83285BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285BB8 size=64
    let mut pc: u32 = 0x83285BB8;
    'dispatch: loop {
        match pc {
            0x83285BB8 => {
    //   block [0x83285BB8..0x83285BF8)
	// 83285BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285BC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285BCC: 388BADB4  addi r4, r11, -0x524c
	ctx.r[4].s64 = ctx.r[11].s64 + -21068;
	// 83285BD0: 386AE6F4  addi r3, r10, -0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + -6412;
	// 83285BD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285BD8: 4AFA72F9  bl 0x8222ced0
	ctx.lr = 0x83285BDC;
	sub_8222CED0(ctx, base);
	// 83285BDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285BE0: 38691DE0  addi r3, r9, 0x1de0
	ctx.r[3].s64 = ctx.r[9].s64 + 7648;
	// 83285BE4: 4BA2433D  bl 0x82ca9f20
	ctx.lr = 0x83285BE8;
	sub_82CA9F20(ctx, base);
	// 83285BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285BF8 size=64
    let mut pc: u32 = 0x83285BF8;
    'dispatch: loop {
        match pc {
            0x83285BF8 => {
    //   block [0x83285BF8..0x83285C38)
	// 83285BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285C08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285C0C: 388BADBC  addi r4, r11, -0x5244
	ctx.r[4].s64 = ctx.r[11].s64 + -21060;
	// 83285C10: 386AE6F8  addi r3, r10, -0x1908
	ctx.r[3].s64 = ctx.r[10].s64 + -6408;
	// 83285C14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285C18: 4AFA72B9  bl 0x8222ced0
	ctx.lr = 0x83285C1C;
	sub_8222CED0(ctx, base);
	// 83285C1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285C20: 38691DF0  addi r3, r9, 0x1df0
	ctx.r[3].s64 = ctx.r[9].s64 + 7664;
	// 83285C24: 4BA242FD  bl 0x82ca9f20
	ctx.lr = 0x83285C28;
	sub_82CA9F20(ctx, base);
	// 83285C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285C38 size=64
    let mut pc: u32 = 0x83285C38;
    'dispatch: loop {
        match pc {
            0x83285C38 => {
    //   block [0x83285C38..0x83285C78)
	// 83285C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285C44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285C4C: 388BADD0  addi r4, r11, -0x5230
	ctx.r[4].s64 = ctx.r[11].s64 + -21040;
	// 83285C50: 386AE6FC  addi r3, r10, -0x1904
	ctx.r[3].s64 = ctx.r[10].s64 + -6404;
	// 83285C54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285C58: 4AFA7279  bl 0x8222ced0
	ctx.lr = 0x83285C5C;
	sub_8222CED0(ctx, base);
	// 83285C5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285C60: 38691E00  addi r3, r9, 0x1e00
	ctx.r[3].s64 = ctx.r[9].s64 + 7680;
	// 83285C64: 4BA242BD  bl 0x82ca9f20
	ctx.lr = 0x83285C68;
	sub_82CA9F20(ctx, base);
	// 83285C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285C78 size=64
    let mut pc: u32 = 0x83285C78;
    'dispatch: loop {
        match pc {
            0x83285C78 => {
    //   block [0x83285C78..0x83285CB8)
	// 83285C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285C80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285C84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285C88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285C8C: 388BADE4  addi r4, r11, -0x521c
	ctx.r[4].s64 = ctx.r[11].s64 + -21020;
	// 83285C90: 386AE700  addi r3, r10, -0x1900
	ctx.r[3].s64 = ctx.r[10].s64 + -6400;
	// 83285C94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285C98: 4AFA7239  bl 0x8222ced0
	ctx.lr = 0x83285C9C;
	sub_8222CED0(ctx, base);
	// 83285C9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285CA0: 38691E10  addi r3, r9, 0x1e10
	ctx.r[3].s64 = ctx.r[9].s64 + 7696;
	// 83285CA4: 4BA2427D  bl 0x82ca9f20
	ctx.lr = 0x83285CA8;
	sub_82CA9F20(ctx, base);
	// 83285CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285CB8 size=64
    let mut pc: u32 = 0x83285CB8;
    'dispatch: loop {
        match pc {
            0x83285CB8 => {
    //   block [0x83285CB8..0x83285CF8)
	// 83285CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285CC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285CCC: 388BAD60  addi r4, r11, -0x52a0
	ctx.r[4].s64 = ctx.r[11].s64 + -21152;
	// 83285CD0: 386AE704  addi r3, r10, -0x18fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6396;
	// 83285CD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285CD8: 4AFA71F9  bl 0x8222ced0
	ctx.lr = 0x83285CDC;
	sub_8222CED0(ctx, base);
	// 83285CDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285CE0: 38691E20  addi r3, r9, 0x1e20
	ctx.r[3].s64 = ctx.r[9].s64 + 7712;
	// 83285CE4: 4BA2423D  bl 0x82ca9f20
	ctx.lr = 0x83285CE8;
	sub_82CA9F20(ctx, base);
	// 83285CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285CF8 size=64
    let mut pc: u32 = 0x83285CF8;
    'dispatch: loop {
        match pc {
            0x83285CF8 => {
    //   block [0x83285CF8..0x83285D38)
	// 83285CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285D04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285D08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285D0C: 388BAD68  addi r4, r11, -0x5298
	ctx.r[4].s64 = ctx.r[11].s64 + -21144;
	// 83285D10: 386AE708  addi r3, r10, -0x18f8
	ctx.r[3].s64 = ctx.r[10].s64 + -6392;
	// 83285D14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285D18: 4AFA71B9  bl 0x8222ced0
	ctx.lr = 0x83285D1C;
	sub_8222CED0(ctx, base);
	// 83285D1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285D20: 38691E30  addi r3, r9, 0x1e30
	ctx.r[3].s64 = ctx.r[9].s64 + 7728;
	// 83285D24: 4BA241FD  bl 0x82ca9f20
	ctx.lr = 0x83285D28;
	sub_82CA9F20(ctx, base);
	// 83285D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285D38 size=64
    let mut pc: u32 = 0x83285D38;
    'dispatch: loop {
        match pc {
            0x83285D38 => {
    //   block [0x83285D38..0x83285D78)
	// 83285D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285D44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285D4C: 388BAD7C  addi r4, r11, -0x5284
	ctx.r[4].s64 = ctx.r[11].s64 + -21124;
	// 83285D50: 386AE70C  addi r3, r10, -0x18f4
	ctx.r[3].s64 = ctx.r[10].s64 + -6388;
	// 83285D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285D58: 4AFA7179  bl 0x8222ced0
	ctx.lr = 0x83285D5C;
	sub_8222CED0(ctx, base);
	// 83285D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285D60: 38691E40  addi r3, r9, 0x1e40
	ctx.r[3].s64 = ctx.r[9].s64 + 7744;
	// 83285D64: 4BA241BD  bl 0x82ca9f20
	ctx.lr = 0x83285D68;
	sub_82CA9F20(ctx, base);
	// 83285D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285D78 size=64
    let mut pc: u32 = 0x83285D78;
    'dispatch: loop {
        match pc {
            0x83285D78 => {
    //   block [0x83285D78..0x83285DB8)
	// 83285D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285D8C: 388BAD90  addi r4, r11, -0x5270
	ctx.r[4].s64 = ctx.r[11].s64 + -21104;
	// 83285D90: 386AE710  addi r3, r10, -0x18f0
	ctx.r[3].s64 = ctx.r[10].s64 + -6384;
	// 83285D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285D98: 4AFA7139  bl 0x8222ced0
	ctx.lr = 0x83285D9C;
	sub_8222CED0(ctx, base);
	// 83285D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285DA0: 38691E50  addi r3, r9, 0x1e50
	ctx.r[3].s64 = ctx.r[9].s64 + 7760;
	// 83285DA4: 4BA2417D  bl 0x82ca9f20
	ctx.lr = 0x83285DA8;
	sub_82CA9F20(ctx, base);
	// 83285DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285DB8 size=64
    let mut pc: u32 = 0x83285DB8;
    'dispatch: loop {
        match pc {
            0x83285DB8 => {
    //   block [0x83285DB8..0x83285DF8)
	// 83285DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285DCC: 388BAD0C  addi r4, r11, -0x52f4
	ctx.r[4].s64 = ctx.r[11].s64 + -21236;
	// 83285DD0: 386AE714  addi r3, r10, -0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6380;
	// 83285DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285DD8: 4AFA70F9  bl 0x8222ced0
	ctx.lr = 0x83285DDC;
	sub_8222CED0(ctx, base);
	// 83285DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285DE0: 38691E60  addi r3, r9, 0x1e60
	ctx.r[3].s64 = ctx.r[9].s64 + 7776;
	// 83285DE4: 4BA2413D  bl 0x82ca9f20
	ctx.lr = 0x83285DE8;
	sub_82CA9F20(ctx, base);
	// 83285DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285DF8 size=64
    let mut pc: u32 = 0x83285DF8;
    'dispatch: loop {
        match pc {
            0x83285DF8 => {
    //   block [0x83285DF8..0x83285E38)
	// 83285DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285E0C: 388BAD14  addi r4, r11, -0x52ec
	ctx.r[4].s64 = ctx.r[11].s64 + -21228;
	// 83285E10: 386AE718  addi r3, r10, -0x18e8
	ctx.r[3].s64 = ctx.r[10].s64 + -6376;
	// 83285E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285E18: 4AFA70B9  bl 0x8222ced0
	ctx.lr = 0x83285E1C;
	sub_8222CED0(ctx, base);
	// 83285E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285E20: 38691E70  addi r3, r9, 0x1e70
	ctx.r[3].s64 = ctx.r[9].s64 + 7792;
	// 83285E24: 4BA240FD  bl 0x82ca9f20
	ctx.lr = 0x83285E28;
	sub_82CA9F20(ctx, base);
	// 83285E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285E38 size=64
    let mut pc: u32 = 0x83285E38;
    'dispatch: loop {
        match pc {
            0x83285E38 => {
    //   block [0x83285E38..0x83285E78)
	// 83285E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285E48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285E4C: 388BAD28  addi r4, r11, -0x52d8
	ctx.r[4].s64 = ctx.r[11].s64 + -21208;
	// 83285E50: 386AE71C  addi r3, r10, -0x18e4
	ctx.r[3].s64 = ctx.r[10].s64 + -6372;
	// 83285E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285E58: 4AFA7079  bl 0x8222ced0
	ctx.lr = 0x83285E5C;
	sub_8222CED0(ctx, base);
	// 83285E5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285E60: 38691E80  addi r3, r9, 0x1e80
	ctx.r[3].s64 = ctx.r[9].s64 + 7808;
	// 83285E64: 4BA240BD  bl 0x82ca9f20
	ctx.lr = 0x83285E68;
	sub_82CA9F20(ctx, base);
	// 83285E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285E78 size=64
    let mut pc: u32 = 0x83285E78;
    'dispatch: loop {
        match pc {
            0x83285E78 => {
    //   block [0x83285E78..0x83285EB8)
	// 83285E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285E88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285E8C: 388BAD3C  addi r4, r11, -0x52c4
	ctx.r[4].s64 = ctx.r[11].s64 + -21188;
	// 83285E90: 386AE720  addi r3, r10, -0x18e0
	ctx.r[3].s64 = ctx.r[10].s64 + -6368;
	// 83285E94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285E98: 4AFA7039  bl 0x8222ced0
	ctx.lr = 0x83285E9C;
	sub_8222CED0(ctx, base);
	// 83285E9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285EA0: 38691E90  addi r3, r9, 0x1e90
	ctx.r[3].s64 = ctx.r[9].s64 + 7824;
	// 83285EA4: 4BA2407D  bl 0x82ca9f20
	ctx.lr = 0x83285EA8;
	sub_82CA9F20(ctx, base);
	// 83285EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285EB8 size=64
    let mut pc: u32 = 0x83285EB8;
    'dispatch: loop {
        match pc {
            0x83285EB8 => {
    //   block [0x83285EB8..0x83285EF8)
	// 83285EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285EC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285EC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285ECC: 388BACA0  addi r4, r11, -0x5360
	ctx.r[4].s64 = ctx.r[11].s64 + -21344;
	// 83285ED0: 386AE724  addi r3, r10, -0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6364;
	// 83285ED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285ED8: 4AFA6FF9  bl 0x8222ced0
	ctx.lr = 0x83285EDC;
	sub_8222CED0(ctx, base);
	// 83285EDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285EE0: 38691EA0  addi r3, r9, 0x1ea0
	ctx.r[3].s64 = ctx.r[9].s64 + 7840;
	// 83285EE4: 4BA2403D  bl 0x82ca9f20
	ctx.lr = 0x83285EE8;
	sub_82CA9F20(ctx, base);
	// 83285EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285EF8 size=64
    let mut pc: u32 = 0x83285EF8;
    'dispatch: loop {
        match pc {
            0x83285EF8 => {
    //   block [0x83285EF8..0x83285F38)
	// 83285EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285F04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285F0C: 388BACB8  addi r4, r11, -0x5348
	ctx.r[4].s64 = ctx.r[11].s64 + -21320;
	// 83285F10: 386AE728  addi r3, r10, -0x18d8
	ctx.r[3].s64 = ctx.r[10].s64 + -6360;
	// 83285F14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285F18: 4AFA6FB9  bl 0x8222ced0
	ctx.lr = 0x83285F1C;
	sub_8222CED0(ctx, base);
	// 83285F1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285F20: 38691EB0  addi r3, r9, 0x1eb0
	ctx.r[3].s64 = ctx.r[9].s64 + 7856;
	// 83285F24: 4BA23FFD  bl 0x82ca9f20
	ctx.lr = 0x83285F28;
	sub_82CA9F20(ctx, base);
	// 83285F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285F38 size=64
    let mut pc: u32 = 0x83285F38;
    'dispatch: loop {
        match pc {
            0x83285F38 => {
    //   block [0x83285F38..0x83285F78)
	// 83285F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285F44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285F48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285F4C: 388BACC8  addi r4, r11, -0x5338
	ctx.r[4].s64 = ctx.r[11].s64 + -21304;
	// 83285F50: 386AE72C  addi r3, r10, -0x18d4
	ctx.r[3].s64 = ctx.r[10].s64 + -6356;
	// 83285F54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285F58: 4AFA6F79  bl 0x8222ced0
	ctx.lr = 0x83285F5C;
	sub_8222CED0(ctx, base);
	// 83285F5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285F60: 38691EC0  addi r3, r9, 0x1ec0
	ctx.r[3].s64 = ctx.r[9].s64 + 7872;
	// 83285F64: 4BA23FBD  bl 0x82ca9f20
	ctx.lr = 0x83285F68;
	sub_82CA9F20(ctx, base);
	// 83285F68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285F78 size=64
    let mut pc: u32 = 0x83285F78;
    'dispatch: loop {
        match pc {
            0x83285F78 => {
    //   block [0x83285F78..0x83285FB8)
	// 83285F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285F84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285F88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285F8C: 388BACCC  addi r4, r11, -0x5334
	ctx.r[4].s64 = ctx.r[11].s64 + -21300;
	// 83285F90: 386AE730  addi r3, r10, -0x18d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6352;
	// 83285F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285F98: 4AFA6F39  bl 0x8222ced0
	ctx.lr = 0x83285F9C;
	sub_8222CED0(ctx, base);
	// 83285F9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285FA0: 38691ED0  addi r3, r9, 0x1ed0
	ctx.r[3].s64 = ctx.r[9].s64 + 7888;
	// 83285FA4: 4BA23F7D  bl 0x82ca9f20
	ctx.lr = 0x83285FA8;
	sub_82CA9F20(ctx, base);
	// 83285FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285FB8 size=64
    let mut pc: u32 = 0x83285FB8;
    'dispatch: loop {
        match pc {
            0x83285FB8 => {
    //   block [0x83285FB8..0x83285FF8)
	// 83285FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285FC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285FC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285FCC: 388BACE0  addi r4, r11, -0x5320
	ctx.r[4].s64 = ctx.r[11].s64 + -21280;
	// 83285FD0: 386AE734  addi r3, r10, -0x18cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6348;
	// 83285FD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285FD8: 4AFA6EF9  bl 0x8222ced0
	ctx.lr = 0x83285FDC;
	sub_8222CED0(ctx, base);
	// 83285FDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285FE0: 38691EE0  addi r3, r9, 0x1ee0
	ctx.r[3].s64 = ctx.r[9].s64 + 7904;
	// 83285FE4: 4BA23F3D  bl 0x82ca9f20
	ctx.lr = 0x83285FE8;
	sub_82CA9F20(ctx, base);
	// 83285FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285FF8 size=64
    let mut pc: u32 = 0x83285FF8;
    'dispatch: loop {
        match pc {
            0x83285FF8 => {
    //   block [0x83285FF8..0x83286038)
	// 83285FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286004: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286008: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328600C: 388BACF0  addi r4, r11, -0x5310
	ctx.r[4].s64 = ctx.r[11].s64 + -21264;
	// 83286010: 386AE738  addi r3, r10, -0x18c8
	ctx.r[3].s64 = ctx.r[10].s64 + -6344;
	// 83286014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286018: 4AFA6EB9  bl 0x8222ced0
	ctx.lr = 0x8328601C;
	sub_8222CED0(ctx, base);
	// 8328601C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286020: 38691EF0  addi r3, r9, 0x1ef0
	ctx.r[3].s64 = ctx.r[9].s64 + 7920;
	// 83286024: 4BA23EFD  bl 0x82ca9f20
	ctx.lr = 0x83286028;
	sub_82CA9F20(ctx, base);
	// 83286028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328602C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286038 size=64
    let mut pc: u32 = 0x83286038;
    'dispatch: loop {
        match pc {
            0x83286038 => {
    //   block [0x83286038..0x83286078)
	// 83286038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328603C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286044: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328604C: 388BAAAC  addi r4, r11, -0x5554
	ctx.r[4].s64 = ctx.r[11].s64 + -21844;
	// 83286050: 386AE73C  addi r3, r10, -0x18c4
	ctx.r[3].s64 = ctx.r[10].s64 + -6340;
	// 83286054: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286058: 4AFA6E79  bl 0x8222ced0
	ctx.lr = 0x8328605C;
	sub_8222CED0(ctx, base);
	// 8328605C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286060: 38691F00  addi r3, r9, 0x1f00
	ctx.r[3].s64 = ctx.r[9].s64 + 7936;
	// 83286064: 4BA23EBD  bl 0x82ca9f20
	ctx.lr = 0x83286068;
	sub_82CA9F20(ctx, base);
	// 83286068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328606C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286078 size=64
    let mut pc: u32 = 0x83286078;
    'dispatch: loop {
        match pc {
            0x83286078 => {
    //   block [0x83286078..0x832860B8)
	// 83286078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286084: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286088: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328608C: 388BAAB4  addi r4, r11, -0x554c
	ctx.r[4].s64 = ctx.r[11].s64 + -21836;
	// 83286090: 386AE740  addi r3, r10, -0x18c0
	ctx.r[3].s64 = ctx.r[10].s64 + -6336;
	// 83286094: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286098: 4AFA6E39  bl 0x8222ced0
	ctx.lr = 0x8328609C;
	sub_8222CED0(ctx, base);
	// 8328609C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832860A0: 38691F10  addi r3, r9, 0x1f10
	ctx.r[3].s64 = ctx.r[9].s64 + 7952;
	// 832860A4: 4BA23E7D  bl 0x82ca9f20
	ctx.lr = 0x832860A8;
	sub_82CA9F20(ctx, base);
	// 832860A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832860AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832860B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832860B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832860B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832860B8 size=64
    let mut pc: u32 = 0x832860B8;
    'dispatch: loop {
        match pc {
            0x832860B8 => {
    //   block [0x832860B8..0x832860F8)
	// 832860B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832860BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832860C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832860C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832860C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832860CC: 388BAAC8  addi r4, r11, -0x5538
	ctx.r[4].s64 = ctx.r[11].s64 + -21816;
	// 832860D0: 386AE744  addi r3, r10, -0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6332;
	// 832860D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832860D8: 4AFA6DF9  bl 0x8222ced0
	ctx.lr = 0x832860DC;
	sub_8222CED0(ctx, base);
	// 832860DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832860E0: 38691F20  addi r3, r9, 0x1f20
	ctx.r[3].s64 = ctx.r[9].s64 + 7968;
	// 832860E4: 4BA23E3D  bl 0x82ca9f20
	ctx.lr = 0x832860E8;
	sub_82CA9F20(ctx, base);
	// 832860E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832860EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832860F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832860F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832860F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832860F8 size=64
    let mut pc: u32 = 0x832860F8;
    'dispatch: loop {
        match pc {
            0x832860F8 => {
    //   block [0x832860F8..0x83286138)
	// 832860F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832860FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328610C: 388BAADC  addi r4, r11, -0x5524
	ctx.r[4].s64 = ctx.r[11].s64 + -21796;
	// 83286110: 386AE748  addi r3, r10, -0x18b8
	ctx.r[3].s64 = ctx.r[10].s64 + -6328;
	// 83286114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286118: 4AFA6DB9  bl 0x8222ced0
	ctx.lr = 0x8328611C;
	sub_8222CED0(ctx, base);
	// 8328611C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286120: 38691F30  addi r3, r9, 0x1f30
	ctx.r[3].s64 = ctx.r[9].s64 + 7984;
	// 83286124: 4BA23DFD  bl 0x82ca9f20
	ctx.lr = 0x83286128;
	sub_82CA9F20(ctx, base);
	// 83286128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328612C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286138 size=64
    let mut pc: u32 = 0x83286138;
    'dispatch: loop {
        match pc {
            0x83286138 => {
    //   block [0x83286138..0x83286178)
	// 83286138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328613C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328614C: 388BAB00  addi r4, r11, -0x5500
	ctx.r[4].s64 = ctx.r[11].s64 + -21760;
	// 83286150: 386AE74C  addi r3, r10, -0x18b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6324;
	// 83286154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286158: 4AFA6D79  bl 0x8222ced0
	ctx.lr = 0x8328615C;
	sub_8222CED0(ctx, base);
	// 8328615C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286160: 38691F40  addi r3, r9, 0x1f40
	ctx.r[3].s64 = ctx.r[9].s64 + 8000;
	// 83286164: 4BA23DBD  bl 0x82ca9f20
	ctx.lr = 0x83286168;
	sub_82CA9F20(ctx, base);
	// 83286168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286178 size=64
    let mut pc: u32 = 0x83286178;
    'dispatch: loop {
        match pc {
            0x83286178 => {
    //   block [0x83286178..0x832861B8)
	// 83286178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328617C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328618C: 388BAB08  addi r4, r11, -0x54f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21752;
	// 83286190: 386AE750  addi r3, r10, -0x18b0
	ctx.r[3].s64 = ctx.r[10].s64 + -6320;
	// 83286194: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286198: 4AFA6D39  bl 0x8222ced0
	ctx.lr = 0x8328619C;
	sub_8222CED0(ctx, base);
	// 8328619C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832861A0: 38691F50  addi r3, r9, 0x1f50
	ctx.r[3].s64 = ctx.r[9].s64 + 8016;
	// 832861A4: 4BA23D7D  bl 0x82ca9f20
	ctx.lr = 0x832861A8;
	sub_82CA9F20(ctx, base);
	// 832861A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832861AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832861B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832861B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832861B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832861B8 size=64
    let mut pc: u32 = 0x832861B8;
    'dispatch: loop {
        match pc {
            0x832861B8 => {
    //   block [0x832861B8..0x832861F8)
	// 832861B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832861BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832861C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832861C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832861C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832861CC: 388BAB28  addi r4, r11, -0x54d8
	ctx.r[4].s64 = ctx.r[11].s64 + -21720;
	// 832861D0: 386AE754  addi r3, r10, -0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6316;
	// 832861D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832861D8: 4AFA6CF9  bl 0x8222ced0
	ctx.lr = 0x832861DC;
	sub_8222CED0(ctx, base);
	// 832861DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832861E0: 38691F60  addi r3, r9, 0x1f60
	ctx.r[3].s64 = ctx.r[9].s64 + 8032;
	// 832861E4: 4BA23D3D  bl 0x82ca9f20
	ctx.lr = 0x832861E8;
	sub_82CA9F20(ctx, base);
	// 832861E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832861EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832861F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832861F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832861F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832861F8 size=64
    let mut pc: u32 = 0x832861F8;
    'dispatch: loop {
        match pc {
            0x832861F8 => {
    //   block [0x832861F8..0x83286238)
	// 832861F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832861FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328620C: 388BAB48  addi r4, r11, -0x54b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21688;
	// 83286210: 386AE758  addi r3, r10, -0x18a8
	ctx.r[3].s64 = ctx.r[10].s64 + -6312;
	// 83286214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286218: 4AFA6CB9  bl 0x8222ced0
	ctx.lr = 0x8328621C;
	sub_8222CED0(ctx, base);
	// 8328621C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286220: 38691F70  addi r3, r9, 0x1f70
	ctx.r[3].s64 = ctx.r[9].s64 + 8048;
	// 83286224: 4BA23CFD  bl 0x82ca9f20
	ctx.lr = 0x83286228;
	sub_82CA9F20(ctx, base);
	// 83286228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328622C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286238 size=64
    let mut pc: u32 = 0x83286238;
    'dispatch: loop {
        match pc {
            0x83286238 => {
    //   block [0x83286238..0x83286278)
	// 83286238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328624C: 388BAB50  addi r4, r11, -0x54b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21680;
	// 83286250: 386AE75C  addi r3, r10, -0x18a4
	ctx.r[3].s64 = ctx.r[10].s64 + -6308;
	// 83286254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286258: 4AFA6C79  bl 0x8222ced0
	ctx.lr = 0x8328625C;
	sub_8222CED0(ctx, base);
	// 8328625C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286260: 38691F80  addi r3, r9, 0x1f80
	ctx.r[3].s64 = ctx.r[9].s64 + 8064;
	// 83286264: 4BA23CBD  bl 0x82ca9f20
	ctx.lr = 0x83286268;
	sub_82CA9F20(ctx, base);
	// 83286268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328626C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286278 size=64
    let mut pc: u32 = 0x83286278;
    'dispatch: loop {
        match pc {
            0x83286278 => {
    //   block [0x83286278..0x832862B8)
	// 83286278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328627C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328628C: 388BAB58  addi r4, r11, -0x54a8
	ctx.r[4].s64 = ctx.r[11].s64 + -21672;
	// 83286290: 386AE760  addi r3, r10, -0x18a0
	ctx.r[3].s64 = ctx.r[10].s64 + -6304;
	// 83286294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286298: 4AFA6C39  bl 0x8222ced0
	ctx.lr = 0x8328629C;
	sub_8222CED0(ctx, base);
	// 8328629C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832862A0: 38691F90  addi r3, r9, 0x1f90
	ctx.r[3].s64 = ctx.r[9].s64 + 8080;
	// 832862A4: 4BA23C7D  bl 0x82ca9f20
	ctx.lr = 0x832862A8;
	sub_82CA9F20(ctx, base);
	// 832862A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832862AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832862B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832862B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832862B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832862B8 size=64
    let mut pc: u32 = 0x832862B8;
    'dispatch: loop {
        match pc {
            0x832862B8 => {
    //   block [0x832862B8..0x832862F8)
	// 832862B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832862BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832862C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832862C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832862C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832862CC: 388BAB68  addi r4, r11, -0x5498
	ctx.r[4].s64 = ctx.r[11].s64 + -21656;
	// 832862D0: 386AE764  addi r3, r10, -0x189c
	ctx.r[3].s64 = ctx.r[10].s64 + -6300;
	// 832862D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832862D8: 4AFA6BF9  bl 0x8222ced0
	ctx.lr = 0x832862DC;
	sub_8222CED0(ctx, base);
	// 832862DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832862E0: 38691FA0  addi r3, r9, 0x1fa0
	ctx.r[3].s64 = ctx.r[9].s64 + 8096;
	// 832862E4: 4BA23C3D  bl 0x82ca9f20
	ctx.lr = 0x832862E8;
	sub_82CA9F20(ctx, base);
	// 832862E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832862EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832862F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832862F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832862F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832862F8 size=64
    let mut pc: u32 = 0x832862F8;
    'dispatch: loop {
        match pc {
            0x832862F8 => {
    //   block [0x832862F8..0x83286338)
	// 832862F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832862FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286308: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328630C: 388BAB84  addi r4, r11, -0x547c
	ctx.r[4].s64 = ctx.r[11].s64 + -21628;
	// 83286310: 386AE768  addi r3, r10, -0x1898
	ctx.r[3].s64 = ctx.r[10].s64 + -6296;
	// 83286314: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286318: 4AFA6BB9  bl 0x8222ced0
	ctx.lr = 0x8328631C;
	sub_8222CED0(ctx, base);
	// 8328631C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286320: 38691FB0  addi r3, r9, 0x1fb0
	ctx.r[3].s64 = ctx.r[9].s64 + 8112;
	// 83286324: 4BA23BFD  bl 0x82ca9f20
	ctx.lr = 0x83286328;
	sub_82CA9F20(ctx, base);
	// 83286328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328632C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286338 size=64
    let mut pc: u32 = 0x83286338;
    'dispatch: loop {
        match pc {
            0x83286338 => {
    //   block [0x83286338..0x83286378)
	// 83286338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328634C: 388BAB90  addi r4, r11, -0x5470
	ctx.r[4].s64 = ctx.r[11].s64 + -21616;
	// 83286350: 386AE76C  addi r3, r10, -0x1894
	ctx.r[3].s64 = ctx.r[10].s64 + -6292;
	// 83286354: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286358: 4AFA6B79  bl 0x8222ced0
	ctx.lr = 0x8328635C;
	sub_8222CED0(ctx, base);
	// 8328635C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286360: 38691FC0  addi r3, r9, 0x1fc0
	ctx.r[3].s64 = ctx.r[9].s64 + 8128;
	// 83286364: 4BA23BBD  bl 0x82ca9f20
	ctx.lr = 0x83286368;
	sub_82CA9F20(ctx, base);
	// 83286368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328636C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286378 size=64
    let mut pc: u32 = 0x83286378;
    'dispatch: loop {
        match pc {
            0x83286378 => {
    //   block [0x83286378..0x832863B8)
	// 83286378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328637C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286384: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328638C: 388BAB9C  addi r4, r11, -0x5464
	ctx.r[4].s64 = ctx.r[11].s64 + -21604;
	// 83286390: 386AE770  addi r3, r10, -0x1890
	ctx.r[3].s64 = ctx.r[10].s64 + -6288;
	// 83286394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286398: 4AFA6B39  bl 0x8222ced0
	ctx.lr = 0x8328639C;
	sub_8222CED0(ctx, base);
	// 8328639C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832863A0: 38691FD0  addi r3, r9, 0x1fd0
	ctx.r[3].s64 = ctx.r[9].s64 + 8144;
	// 832863A4: 4BA23B7D  bl 0x82ca9f20
	ctx.lr = 0x832863A8;
	sub_82CA9F20(ctx, base);
	// 832863A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832863AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832863B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832863B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832863B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832863B8 size=64
    let mut pc: u32 = 0x832863B8;
    'dispatch: loop {
        match pc {
            0x832863B8 => {
    //   block [0x832863B8..0x832863F8)
	// 832863B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832863BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832863C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832863C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832863C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832863CC: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 832863D0: 386AE774  addi r3, r10, -0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + -6284;
	// 832863D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832863D8: 4AFA6AF9  bl 0x8222ced0
	ctx.lr = 0x832863DC;
	sub_8222CED0(ctx, base);
	// 832863DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832863E0: 38691FE0  addi r3, r9, 0x1fe0
	ctx.r[3].s64 = ctx.r[9].s64 + 8160;
	// 832863E4: 4BA23B3D  bl 0x82ca9f20
	ctx.lr = 0x832863E8;
	sub_82CA9F20(ctx, base);
	// 832863E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832863EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832863F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832863F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832863F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832863F8 size=64
    let mut pc: u32 = 0x832863F8;
    'dispatch: loop {
        match pc {
            0x832863F8 => {
    //   block [0x832863F8..0x83286438)
	// 832863F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832863FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328640C: 388BABB0  addi r4, r11, -0x5450
	ctx.r[4].s64 = ctx.r[11].s64 + -21584;
	// 83286410: 386AE778  addi r3, r10, -0x1888
	ctx.r[3].s64 = ctx.r[10].s64 + -6280;
	// 83286414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286418: 4AFA6AB9  bl 0x8222ced0
	ctx.lr = 0x8328641C;
	sub_8222CED0(ctx, base);
	// 8328641C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286420: 38691FF0  addi r3, r9, 0x1ff0
	ctx.r[3].s64 = ctx.r[9].s64 + 8176;
	// 83286424: 4BA23AFD  bl 0x82ca9f20
	ctx.lr = 0x83286428;
	sub_82CA9F20(ctx, base);
	// 83286428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328642C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286438 size=64
    let mut pc: u32 = 0x83286438;
    'dispatch: loop {
        match pc {
            0x83286438 => {
    //   block [0x83286438..0x83286478)
	// 83286438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328643C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328644C: 388BABC0  addi r4, r11, -0x5440
	ctx.r[4].s64 = ctx.r[11].s64 + -21568;
	// 83286450: 386AE77C  addi r3, r10, -0x1884
	ctx.r[3].s64 = ctx.r[10].s64 + -6276;
	// 83286454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286458: 4AFA6A79  bl 0x8222ced0
	ctx.lr = 0x8328645C;
	sub_8222CED0(ctx, base);
	// 8328645C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286460: 38692000  addi r3, r9, 0x2000
	ctx.r[3].s64 = ctx.r[9].s64 + 8192;
	// 83286464: 4BA23ABD  bl 0x82ca9f20
	ctx.lr = 0x83286468;
	sub_82CA9F20(ctx, base);
	// 83286468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328646C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83286478 size=312
    let mut pc: u32 = 0x83286478;
    'dispatch: loop {
        match pc {
            0x83286478 => {
    //   block [0x83286478..0x832865B0)
	// 83286478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328647C: 4BA22F89  bl 0x82ca9404
	ctx.lr = 0x83286480;
	sub_82CA93D0(ctx, base);
	// 83286480: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286484: 38E1FFA8  addi r7, r1, -0x58
	ctx.r[7].s64 = ctx.r[1].s64 + -88;
	// 83286488: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8328648C: 38C1FFAC  addi r6, r1, -0x54
	ctx.r[6].s64 = ctx.r[1].s64 + -84;
	// 83286490: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 83286494: 3901FFA4  addi r8, r1, -0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + -92;
	// 83286498: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8328649C: 3BC1FFC0  addi r30, r1, -0x40
	ctx.r[30].s64 = ctx.r[1].s64 + -64;
	// 832864A0: D1A1FFA0  stfs f13, -0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
	// 832864A4: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832864A8: 3BA1FFC4  addi r29, r1, -0x3c
	ctx.r[29].s64 = ctx.r[1].s64 + -60;
	// 832864AC: D001FFA8  stfs f0, -0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 832864B0: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
	// 832864B4: D001FFAC  stfs f0, -0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-84 as u32), tmp.u32 ) };
	// 832864B8: 3921FFB8  addi r9, r1, -0x48
	ctx.r[9].s64 = ctx.r[1].s64 + -72;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832865B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832865B0 size=64
    let mut pc: u32 = 0x832865B0;
    'dispatch: loop {
        match pc {
            0x832865B0 => {
    //   block [0x832865B0..0x832865F0)
	// 832865B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832865B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832865B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832865BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832865C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832865C4: 388BAA44  addi r4, r11, -0x55bc
	ctx.r[4].s64 = ctx.r[11].s64 + -21948;
	// 832865C8: 386AE7C0  addi r3, r10, -0x1840
	ctx.r[3].s64 = ctx.r[10].s64 + -6208;
	// 832865CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832865D0: 4AFA6901  bl 0x8222ced0
	ctx.lr = 0x832865D4;
	sub_8222CED0(ctx, base);
	// 832865D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832865D8: 38692118  addi r3, r9, 0x2118
	ctx.r[3].s64 = ctx.r[9].s64 + 8472;
	// 832865DC: 4BA23945  bl 0x82ca9f20
	ctx.lr = 0x832865E0;
	sub_82CA9F20(ctx, base);
	// 832865E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832865E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832865E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832865EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832865F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832865F0 size=64
    let mut pc: u32 = 0x832865F0;
    'dispatch: loop {
        match pc {
            0x832865F0 => {
    //   block [0x832865F0..0x83286630)
	// 832865F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832865F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832865F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832865FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286604: 388BAA4C  addi r4, r11, -0x55b4
	ctx.r[4].s64 = ctx.r[11].s64 + -21940;
	// 83286608: 386AE7C4  addi r3, r10, -0x183c
	ctx.r[3].s64 = ctx.r[10].s64 + -6204;
	// 8328660C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286610: 4AFA68C1  bl 0x8222ced0
	ctx.lr = 0x83286614;
	sub_8222CED0(ctx, base);
	// 83286614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286618: 38692128  addi r3, r9, 0x2128
	ctx.r[3].s64 = ctx.r[9].s64 + 8488;
	// 8328661C: 4BA23905  bl 0x82ca9f20
	ctx.lr = 0x83286620;
	sub_82CA9F20(ctx, base);
	// 83286620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328662C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286630 size=64
    let mut pc: u32 = 0x83286630;
    'dispatch: loop {
        match pc {
            0x83286630 => {
    //   block [0x83286630..0x83286670)
	// 83286630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328663C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286644: 388BAA60  addi r4, r11, -0x55a0
	ctx.r[4].s64 = ctx.r[11].s64 + -21920;
	// 83286648: 386AE7C8  addi r3, r10, -0x1838
	ctx.r[3].s64 = ctx.r[10].s64 + -6200;
	// 8328664C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286650: 4AFA6881  bl 0x8222ced0
	ctx.lr = 0x83286654;
	sub_8222CED0(ctx, base);
	// 83286654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286658: 38692138  addi r3, r9, 0x2138
	ctx.r[3].s64 = ctx.r[9].s64 + 8504;
	// 8328665C: 4BA238C5  bl 0x82ca9f20
	ctx.lr = 0x83286660;
	sub_82CA9F20(ctx, base);
	// 83286660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328666C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286670 size=64
    let mut pc: u32 = 0x83286670;
    'dispatch: loop {
        match pc {
            0x83286670 => {
    //   block [0x83286670..0x832866B0)
	// 83286670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328667C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286684: 388BAA74  addi r4, r11, -0x558c
	ctx.r[4].s64 = ctx.r[11].s64 + -21900;
	// 83286688: 386AE7CC  addi r3, r10, -0x1834
	ctx.r[3].s64 = ctx.r[10].s64 + -6196;
	// 8328668C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286690: 4AFA6841  bl 0x8222ced0
	ctx.lr = 0x83286694;
	sub_8222CED0(ctx, base);
	// 83286694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286698: 38692148  addi r3, r9, 0x2148
	ctx.r[3].s64 = ctx.r[9].s64 + 8520;
	// 8328669C: 4BA23885  bl 0x82ca9f20
	ctx.lr = 0x832866A0;
	sub_82CA9F20(ctx, base);
	// 832866A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832866A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832866A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832866AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832866B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832866B0 size=64
    let mut pc: u32 = 0x832866B0;
    'dispatch: loop {
        match pc {
            0x832866B0 => {
    //   block [0x832866B0..0x832866F0)
	// 832866B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832866B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832866B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832866BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832866C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832866C4: 388BAA88  addi r4, r11, -0x5578
	ctx.r[4].s64 = ctx.r[11].s64 + -21880;
	// 832866C8: 386AE7D0  addi r3, r10, -0x1830
	ctx.r[3].s64 = ctx.r[10].s64 + -6192;
	// 832866CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832866D0: 4AFA6801  bl 0x8222ced0
	ctx.lr = 0x832866D4;
	sub_8222CED0(ctx, base);
	// 832866D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832866D8: 38692158  addi r3, r9, 0x2158
	ctx.r[3].s64 = ctx.r[9].s64 + 8536;
	// 832866DC: 4BA23845  bl 0x82ca9f20
	ctx.lr = 0x832866E0;
	sub_82CA9F20(ctx, base);
	// 832866E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832866E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832866E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832866EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832866F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832866F0 size=64
    let mut pc: u32 = 0x832866F0;
    'dispatch: loop {
        match pc {
            0x832866F0 => {
    //   block [0x832866F0..0x83286730)
	// 832866F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832866F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832866F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832866FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286704: 388BAA98  addi r4, r11, -0x5568
	ctx.r[4].s64 = ctx.r[11].s64 + -21864;
	// 83286708: 386AE7D4  addi r3, r10, -0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + -6188;
	// 8328670C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286710: 4AFA67C1  bl 0x8222ced0
	ctx.lr = 0x83286714;
	sub_8222CED0(ctx, base);
	// 83286714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286718: 38692168  addi r3, r9, 0x2168
	ctx.r[3].s64 = ctx.r[9].s64 + 8552;
	// 8328671C: 4BA23805  bl 0x82ca9f20
	ctx.lr = 0x83286720;
	sub_82CA9F20(ctx, base);
	// 83286720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328672C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286730 size=64
    let mut pc: u32 = 0x83286730;
    'dispatch: loop {
        match pc {
            0x83286730 => {
    //   block [0x83286730..0x83286770)
	// 83286730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328673C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286744: 388BA9E0  addi r4, r11, -0x5620
	ctx.r[4].s64 = ctx.r[11].s64 + -22048;
	// 83286748: 386AE7D8  addi r3, r10, -0x1828
	ctx.r[3].s64 = ctx.r[10].s64 + -6184;
	// 8328674C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286750: 4AFA6781  bl 0x8222ced0
	ctx.lr = 0x83286754;
	sub_8222CED0(ctx, base);
	// 83286754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286758: 38692178  addi r3, r9, 0x2178
	ctx.r[3].s64 = ctx.r[9].s64 + 8568;
	// 8328675C: 4BA237C5  bl 0x82ca9f20
	ctx.lr = 0x83286760;
	sub_82CA9F20(ctx, base);
	// 83286760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328676C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286770 size=64
    let mut pc: u32 = 0x83286770;
    'dispatch: loop {
        match pc {
            0x83286770 => {
    //   block [0x83286770..0x832867B0)
	// 83286770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328677C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286784: 388BA9E8  addi r4, r11, -0x5618
	ctx.r[4].s64 = ctx.r[11].s64 + -22040;
	// 83286788: 386AE7DC  addi r3, r10, -0x1824
	ctx.r[3].s64 = ctx.r[10].s64 + -6180;
	// 8328678C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286790: 4AFA6741  bl 0x8222ced0
	ctx.lr = 0x83286794;
	sub_8222CED0(ctx, base);
	// 83286794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286798: 38692188  addi r3, r9, 0x2188
	ctx.r[3].s64 = ctx.r[9].s64 + 8584;
	// 8328679C: 4BA23785  bl 0x82ca9f20
	ctx.lr = 0x832867A0;
	sub_82CA9F20(ctx, base);
	// 832867A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832867A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832867A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832867AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832867B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832867B0 size=64
    let mut pc: u32 = 0x832867B0;
    'dispatch: loop {
        match pc {
            0x832867B0 => {
    //   block [0x832867B0..0x832867F0)
	// 832867B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832867B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832867B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832867BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832867C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832867C4: 388BA9FC  addi r4, r11, -0x5604
	ctx.r[4].s64 = ctx.r[11].s64 + -22020;
	// 832867C8: 386AE7E0  addi r3, r10, -0x1820
	ctx.r[3].s64 = ctx.r[10].s64 + -6176;
	// 832867CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832867D0: 4AFA6701  bl 0x8222ced0
	ctx.lr = 0x832867D4;
	sub_8222CED0(ctx, base);
	// 832867D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832867D8: 38692198  addi r3, r9, 0x2198
	ctx.r[3].s64 = ctx.r[9].s64 + 8600;
	// 832867DC: 4BA23745  bl 0x82ca9f20
	ctx.lr = 0x832867E0;
	sub_82CA9F20(ctx, base);
	// 832867E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832867E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832867E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832867EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832867F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832867F0 size=64
    let mut pc: u32 = 0x832867F0;
    'dispatch: loop {
        match pc {
            0x832867F0 => {
    //   block [0x832867F0..0x83286830)
	// 832867F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832867F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832867F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832867FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286804: 388BAA10  addi r4, r11, -0x55f0
	ctx.r[4].s64 = ctx.r[11].s64 + -22000;
	// 83286808: 386AE7E4  addi r3, r10, -0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + -6172;
	// 8328680C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286810: 4AFA66C1  bl 0x8222ced0
	ctx.lr = 0x83286814;
	sub_8222CED0(ctx, base);
	// 83286814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286818: 386921A8  addi r3, r9, 0x21a8
	ctx.r[3].s64 = ctx.r[9].s64 + 8616;
	// 8328681C: 4BA23705  bl 0x82ca9f20
	ctx.lr = 0x83286820;
	sub_82CA9F20(ctx, base);
	// 83286820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328682C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286830 size=64
    let mut pc: u32 = 0x83286830;
    'dispatch: loop {
        match pc {
            0x83286830 => {
    //   block [0x83286830..0x83286870)
	// 83286830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328683C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286844: 388BAA24  addi r4, r11, -0x55dc
	ctx.r[4].s64 = ctx.r[11].s64 + -21980;
	// 83286848: 386AE7E8  addi r3, r10, -0x1818
	ctx.r[3].s64 = ctx.r[10].s64 + -6168;
	// 8328684C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286850: 4AFA6681  bl 0x8222ced0
	ctx.lr = 0x83286854;
	sub_8222CED0(ctx, base);
	// 83286854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286858: 386921B8  addi r3, r9, 0x21b8
	ctx.r[3].s64 = ctx.r[9].s64 + 8632;
	// 8328685C: 4BA236C5  bl 0x82ca9f20
	ctx.lr = 0x83286860;
	sub_82CA9F20(ctx, base);
	// 83286860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328686C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286870 size=64
    let mut pc: u32 = 0x83286870;
    'dispatch: loop {
        match pc {
            0x83286870 => {
    //   block [0x83286870..0x832868B0)
	// 83286870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328687C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286884: 388BAA30  addi r4, r11, -0x55d0
	ctx.r[4].s64 = ctx.r[11].s64 + -21968;
	// 83286888: 386AE7EC  addi r3, r10, -0x1814
	ctx.r[3].s64 = ctx.r[10].s64 + -6164;
	// 8328688C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286890: 4AFA6641  bl 0x8222ced0
	ctx.lr = 0x83286894;
	sub_8222CED0(ctx, base);
	// 83286894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286898: 386921C8  addi r3, r9, 0x21c8
	ctx.r[3].s64 = ctx.r[9].s64 + 8648;
	// 8328689C: 4BA23685  bl 0x82ca9f20
	ctx.lr = 0x832868A0;
	sub_82CA9F20(ctx, base);
	// 832868A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832868A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832868A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832868AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832868B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832868B0 size=64
    let mut pc: u32 = 0x832868B0;
    'dispatch: loop {
        match pc {
            0x832868B0 => {
    //   block [0x832868B0..0x832868F0)
	// 832868B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832868B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832868B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832868BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832868C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832868C4: 388BA96C  addi r4, r11, -0x5694
	ctx.r[4].s64 = ctx.r[11].s64 + -22164;
	// 832868C8: 386AE7F0  addi r3, r10, -0x1810
	ctx.r[3].s64 = ctx.r[10].s64 + -6160;
	// 832868CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832868D0: 4AFA6601  bl 0x8222ced0
	ctx.lr = 0x832868D4;
	sub_8222CED0(ctx, base);
	// 832868D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832868D8: 386921D8  addi r3, r9, 0x21d8
	ctx.r[3].s64 = ctx.r[9].s64 + 8664;
	// 832868DC: 4BA23645  bl 0x82ca9f20
	ctx.lr = 0x832868E0;
	sub_82CA9F20(ctx, base);
	// 832868E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832868E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832868E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832868EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832868F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832868F0 size=64
    let mut pc: u32 = 0x832868F0;
    'dispatch: loop {
        match pc {
            0x832868F0 => {
    //   block [0x832868F0..0x83286930)
	// 832868F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832868F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832868F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832868FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286904: 388BA974  addi r4, r11, -0x568c
	ctx.r[4].s64 = ctx.r[11].s64 + -22156;
	// 83286908: 386AE7F4  addi r3, r10, -0x180c
	ctx.r[3].s64 = ctx.r[10].s64 + -6156;
	// 8328690C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286910: 4AFA65C1  bl 0x8222ced0
	ctx.lr = 0x83286914;
	sub_8222CED0(ctx, base);
	// 83286914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286918: 386921E8  addi r3, r9, 0x21e8
	ctx.r[3].s64 = ctx.r[9].s64 + 8680;
	// 8328691C: 4BA23605  bl 0x82ca9f20
	ctx.lr = 0x83286920;
	sub_82CA9F20(ctx, base);
	// 83286920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328692C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286930 size=64
    let mut pc: u32 = 0x83286930;
    'dispatch: loop {
        match pc {
            0x83286930 => {
    //   block [0x83286930..0x83286970)
	// 83286930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328693C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286944: 388BA988  addi r4, r11, -0x5678
	ctx.r[4].s64 = ctx.r[11].s64 + -22136;
	// 83286948: 386AE7F8  addi r3, r10, -0x1808
	ctx.r[3].s64 = ctx.r[10].s64 + -6152;
	// 8328694C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286950: 4AFA6581  bl 0x8222ced0
	ctx.lr = 0x83286954;
	sub_8222CED0(ctx, base);
	// 83286954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286958: 386921F8  addi r3, r9, 0x21f8
	ctx.r[3].s64 = ctx.r[9].s64 + 8696;
	// 8328695C: 4BA235C5  bl 0x82ca9f20
	ctx.lr = 0x83286960;
	sub_82CA9F20(ctx, base);
	// 83286960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328696C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286970 size=64
    let mut pc: u32 = 0x83286970;
    'dispatch: loop {
        match pc {
            0x83286970 => {
    //   block [0x83286970..0x832869B0)
	// 83286970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328697C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286984: 388BA99C  addi r4, r11, -0x5664
	ctx.r[4].s64 = ctx.r[11].s64 + -22116;
	// 83286988: 386AE7FC  addi r3, r10, -0x1804
	ctx.r[3].s64 = ctx.r[10].s64 + -6148;
	// 8328698C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286990: 4AFA6541  bl 0x8222ced0
	ctx.lr = 0x83286994;
	sub_8222CED0(ctx, base);
	// 83286994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286998: 38692208  addi r3, r9, 0x2208
	ctx.r[3].s64 = ctx.r[9].s64 + 8712;
	// 8328699C: 4BA23585  bl 0x82ca9f20
	ctx.lr = 0x832869A0;
	sub_82CA9F20(ctx, base);
	// 832869A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832869A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832869A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832869AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832869B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832869B0 size=64
    let mut pc: u32 = 0x832869B0;
    'dispatch: loop {
        match pc {
            0x832869B0 => {
    //   block [0x832869B0..0x832869F0)
	// 832869B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832869B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832869B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832869BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832869C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832869C4: 388BA9B0  addi r4, r11, -0x5650
	ctx.r[4].s64 = ctx.r[11].s64 + -22096;
	// 832869C8: 386AE800  addi r3, r10, -0x1800
	ctx.r[3].s64 = ctx.r[10].s64 + -6144;
	// 832869CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832869D0: 4AFA6501  bl 0x8222ced0
	ctx.lr = 0x832869D4;
	sub_8222CED0(ctx, base);
	// 832869D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832869D8: 38692218  addi r3, r9, 0x2218
	ctx.r[3].s64 = ctx.r[9].s64 + 8728;
	// 832869DC: 4BA23545  bl 0x82ca9f20
	ctx.lr = 0x832869E0;
	sub_82CA9F20(ctx, base);
	// 832869E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832869E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832869E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832869EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832869F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832869F0 size=64
    let mut pc: u32 = 0x832869F0;
    'dispatch: loop {
        match pc {
            0x832869F0 => {
    //   block [0x832869F0..0x83286A30)
	// 832869F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832869F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832869F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832869FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286A04: 388BA9BC  addi r4, r11, -0x5644
	ctx.r[4].s64 = ctx.r[11].s64 + -22084;
	// 83286A08: 386AE804  addi r3, r10, -0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6140;
	// 83286A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286A10: 4AFA64C1  bl 0x8222ced0
	ctx.lr = 0x83286A14;
	sub_8222CED0(ctx, base);
	// 83286A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286A18: 38692228  addi r3, r9, 0x2228
	ctx.r[3].s64 = ctx.r[9].s64 + 8744;
	// 83286A1C: 4BA23505  bl 0x82ca9f20
	ctx.lr = 0x83286A20;
	sub_82CA9F20(ctx, base);
	// 83286A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286A30 size=64
    let mut pc: u32 = 0x83286A30;
    'dispatch: loop {
        match pc {
            0x83286A30 => {
    //   block [0x83286A30..0x83286A70)
	// 83286A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286A44: 388BA9CC  addi r4, r11, -0x5634
	ctx.r[4].s64 = ctx.r[11].s64 + -22068;
	// 83286A48: 386AE808  addi r3, r10, -0x17f8
	ctx.r[3].s64 = ctx.r[10].s64 + -6136;
	// 83286A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286A50: 4AFA6481  bl 0x8222ced0
	ctx.lr = 0x83286A54;
	sub_8222CED0(ctx, base);
	// 83286A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286A58: 38692238  addi r3, r9, 0x2238
	ctx.r[3].s64 = ctx.r[9].s64 + 8760;
	// 83286A5C: 4BA234C5  bl 0x82ca9f20
	ctx.lr = 0x83286A60;
	sub_82CA9F20(ctx, base);
	// 83286A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286A70 size=64
    let mut pc: u32 = 0x83286A70;
    'dispatch: loop {
        match pc {
            0x83286A70 => {
    //   block [0x83286A70..0x83286AB0)
	// 83286A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286A7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286A84: 388BA93C  addi r4, r11, -0x56c4
	ctx.r[4].s64 = ctx.r[11].s64 + -22212;
	// 83286A88: 386AE80C  addi r3, r10, -0x17f4
	ctx.r[3].s64 = ctx.r[10].s64 + -6132;
	// 83286A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286A90: 4AFA6441  bl 0x8222ced0
	ctx.lr = 0x83286A94;
	sub_8222CED0(ctx, base);
	// 83286A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286A98: 38692248  addi r3, r9, 0x2248
	ctx.r[3].s64 = ctx.r[9].s64 + 8776;
	// 83286A9C: 4BA23485  bl 0x82ca9f20
	ctx.lr = 0x83286AA0;
	sub_82CA9F20(ctx, base);
	// 83286AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286AB0 size=64
    let mut pc: u32 = 0x83286AB0;
    'dispatch: loop {
        match pc {
            0x83286AB0 => {
    //   block [0x83286AB0..0x83286AF0)
	// 83286AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286ABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286AC4: 388BA944  addi r4, r11, -0x56bc
	ctx.r[4].s64 = ctx.r[11].s64 + -22204;
	// 83286AC8: 386AE810  addi r3, r10, -0x17f0
	ctx.r[3].s64 = ctx.r[10].s64 + -6128;
	// 83286ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286AD0: 4AFA6401  bl 0x8222ced0
	ctx.lr = 0x83286AD4;
	sub_8222CED0(ctx, base);
	// 83286AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286AD8: 38692258  addi r3, r9, 0x2258
	ctx.r[3].s64 = ctx.r[9].s64 + 8792;
	// 83286ADC: 4BA23445  bl 0x82ca9f20
	ctx.lr = 0x83286AE0;
	sub_82CA9F20(ctx, base);
	// 83286AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286AF0 size=64
    let mut pc: u32 = 0x83286AF0;
    'dispatch: loop {
        match pc {
            0x83286AF0 => {
    //   block [0x83286AF0..0x83286B30)
	// 83286AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286AFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286B04: 388BA958  addi r4, r11, -0x56a8
	ctx.r[4].s64 = ctx.r[11].s64 + -22184;
	// 83286B08: 386AE814  addi r3, r10, -0x17ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6124;
	// 83286B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286B10: 4AFA63C1  bl 0x8222ced0
	ctx.lr = 0x83286B14;
	sub_8222CED0(ctx, base);
	// 83286B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286B18: 38692268  addi r3, r9, 0x2268
	ctx.r[3].s64 = ctx.r[9].s64 + 8808;
	// 83286B1C: 4BA23405  bl 0x82ca9f20
	ctx.lr = 0x83286B20;
	sub_82CA9F20(ctx, base);
	// 83286B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


