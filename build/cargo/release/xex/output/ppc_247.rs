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
    //   block [0x83264488..0x83264548)
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
    //   block [0x83264548..0x83264608)
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
    //   block [0x83264AF0..0x83264BB0)
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
    //   block [0x83264EE0..0x83264FA0)
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
    //   block [0x83264FA0..0x83265060)
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
    //   block [0x83265398..0x83265428)
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
    //   block [0x83265428..0x832654B8)
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
    //   block [0x83266180..0x8326620C)
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


pub fn sub_832668B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832668B8 size=64
    let mut pc: u32 = 0x832668B8;
    'dispatch: loop {
        match pc {
            0x832668B8 => {
    //   block [0x832668B8..0x832668F8)
	// 832668B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832668BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832668C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832668C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832668C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832668CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832668D0: 386AB840  addi r3, r10, -0x47c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18368;
	// 832668D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832668D8: 4AFC65F9  bl 0x8222ced0
	ctx.lr = 0x832668DC;
	sub_8222CED0(ctx, base);
	// 832668DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832668E0: 3869D740  addi r3, r9, -0x28c0
	ctx.r[3].s64 = ctx.r[9].s64 + -10432;
	// 832668E4: 4BA4363D  bl 0x82ca9f20
	ctx.lr = 0x832668E8;
	sub_82CA9F20(ctx, base);
	// 832668E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832668EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832668F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832668F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832668F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832668F8 size=64
    let mut pc: u32 = 0x832668F8;
    'dispatch: loop {
        match pc {
            0x832668F8 => {
    //   block [0x832668F8..0x83266938)
	// 832668F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832668FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326690C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266910: 386AB844  addi r3, r10, -0x47bc
	ctx.r[3].s64 = ctx.r[10].s64 + -18364;
	// 83266914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266918: 4AFC65B9  bl 0x8222ced0
	ctx.lr = 0x8326691C;
	sub_8222CED0(ctx, base);
	// 8326691C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266920: 3869D750  addi r3, r9, -0x28b0
	ctx.r[3].s64 = ctx.r[9].s64 + -10416;
	// 83266924: 4BA435FD  bl 0x82ca9f20
	ctx.lr = 0x83266928;
	sub_82CA9F20(ctx, base);
	// 83266928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326692C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266938 size=64
    let mut pc: u32 = 0x83266938;
    'dispatch: loop {
        match pc {
            0x83266938 => {
    //   block [0x83266938..0x83266978)
	// 83266938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326694C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266950: 386AB848  addi r3, r10, -0x47b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18360;
	// 83266954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266958: 4AFC6579  bl 0x8222ced0
	ctx.lr = 0x8326695C;
	sub_8222CED0(ctx, base);
	// 8326695C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266960: 3869D760  addi r3, r9, -0x28a0
	ctx.r[3].s64 = ctx.r[9].s64 + -10400;
	// 83266964: 4BA435BD  bl 0x82ca9f20
	ctx.lr = 0x83266968;
	sub_82CA9F20(ctx, base);
	// 83266968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326696C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266978 size=64
    let mut pc: u32 = 0x83266978;
    'dispatch: loop {
        match pc {
            0x83266978 => {
    //   block [0x83266978..0x832669B8)
	// 83266978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326698C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266990: 386AB84C  addi r3, r10, -0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18356;
	// 83266994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266998: 4AFC6539  bl 0x8222ced0
	ctx.lr = 0x8326699C;
	sub_8222CED0(ctx, base);
	// 8326699C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832669A0: 3869D770  addi r3, r9, -0x2890
	ctx.r[3].s64 = ctx.r[9].s64 + -10384;
	// 832669A4: 4BA4357D  bl 0x82ca9f20
	ctx.lr = 0x832669A8;
	sub_82CA9F20(ctx, base);
	// 832669A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832669AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832669B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832669B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832669B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832669B8 size=64
    let mut pc: u32 = 0x832669B8;
    'dispatch: loop {
        match pc {
            0x832669B8 => {
    //   block [0x832669B8..0x832669F8)
	// 832669B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832669BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832669C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832669C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832669C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832669CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832669D0: 386AB850  addi r3, r10, -0x47b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18352;
	// 832669D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832669D8: 4AFC64F9  bl 0x8222ced0
	ctx.lr = 0x832669DC;
	sub_8222CED0(ctx, base);
	// 832669DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832669E0: 3869D780  addi r3, r9, -0x2880
	ctx.r[3].s64 = ctx.r[9].s64 + -10368;
	// 832669E4: 4BA4353D  bl 0x82ca9f20
	ctx.lr = 0x832669E8;
	sub_82CA9F20(ctx, base);
	// 832669E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832669EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832669F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832669F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832669F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832669F8 size=64
    let mut pc: u32 = 0x832669F8;
    'dispatch: loop {
        match pc {
            0x832669F8 => {
    //   block [0x832669F8..0x83266A38)
	// 832669F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832669FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266A0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266A10: 386AB854  addi r3, r10, -0x47ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18348;
	// 83266A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266A18: 4AFC64B9  bl 0x8222ced0
	ctx.lr = 0x83266A1C;
	sub_8222CED0(ctx, base);
	// 83266A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266A20: 3869D790  addi r3, r9, -0x2870
	ctx.r[3].s64 = ctx.r[9].s64 + -10352;
	// 83266A24: 4BA434FD  bl 0x82ca9f20
	ctx.lr = 0x83266A28;
	sub_82CA9F20(ctx, base);
	// 83266A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266A38 size=64
    let mut pc: u32 = 0x83266A38;
    'dispatch: loop {
        match pc {
            0x83266A38 => {
    //   block [0x83266A38..0x83266A78)
	// 83266A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266A4C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266A50: 386AB858  addi r3, r10, -0x47a8
	ctx.r[3].s64 = ctx.r[10].s64 + -18344;
	// 83266A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266A58: 4AFC6479  bl 0x8222ced0
	ctx.lr = 0x83266A5C;
	sub_8222CED0(ctx, base);
	// 83266A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266A60: 3869D7A0  addi r3, r9, -0x2860
	ctx.r[3].s64 = ctx.r[9].s64 + -10336;
	// 83266A64: 4BA434BD  bl 0x82ca9f20
	ctx.lr = 0x83266A68;
	sub_82CA9F20(ctx, base);
	// 83266A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266A78 size=64
    let mut pc: u32 = 0x83266A78;
    'dispatch: loop {
        match pc {
            0x83266A78 => {
    //   block [0x83266A78..0x83266AB8)
	// 83266A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266A8C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266A90: 386AB85C  addi r3, r10, -0x47a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18340;
	// 83266A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266A98: 4AFC6439  bl 0x8222ced0
	ctx.lr = 0x83266A9C;
	sub_8222CED0(ctx, base);
	// 83266A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266AA0: 3869D7B0  addi r3, r9, -0x2850
	ctx.r[3].s64 = ctx.r[9].s64 + -10320;
	// 83266AA4: 4BA4347D  bl 0x82ca9f20
	ctx.lr = 0x83266AA8;
	sub_82CA9F20(ctx, base);
	// 83266AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266AB8 size=64
    let mut pc: u32 = 0x83266AB8;
    'dispatch: loop {
        match pc {
            0x83266AB8 => {
    //   block [0x83266AB8..0x83266AF8)
	// 83266AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266ACC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266AD0: 386AB860  addi r3, r10, -0x47a0
	ctx.r[3].s64 = ctx.r[10].s64 + -18336;
	// 83266AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266AD8: 4AFC63F9  bl 0x8222ced0
	ctx.lr = 0x83266ADC;
	sub_8222CED0(ctx, base);
	// 83266ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266AE0: 3869D7C0  addi r3, r9, -0x2840
	ctx.r[3].s64 = ctx.r[9].s64 + -10304;
	// 83266AE4: 4BA4343D  bl 0x82ca9f20
	ctx.lr = 0x83266AE8;
	sub_82CA9F20(ctx, base);
	// 83266AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266AF8 size=64
    let mut pc: u32 = 0x83266AF8;
    'dispatch: loop {
        match pc {
            0x83266AF8 => {
    //   block [0x83266AF8..0x83266B38)
	// 83266AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266B0C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266B10: 386AB864  addi r3, r10, -0x479c
	ctx.r[3].s64 = ctx.r[10].s64 + -18332;
	// 83266B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266B18: 4AFC63B9  bl 0x8222ced0
	ctx.lr = 0x83266B1C;
	sub_8222CED0(ctx, base);
	// 83266B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266B20: 3869D7D0  addi r3, r9, -0x2830
	ctx.r[3].s64 = ctx.r[9].s64 + -10288;
	// 83266B24: 4BA433FD  bl 0x82ca9f20
	ctx.lr = 0x83266B28;
	sub_82CA9F20(ctx, base);
	// 83266B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266B38 size=64
    let mut pc: u32 = 0x83266B38;
    'dispatch: loop {
        match pc {
            0x83266B38 => {
    //   block [0x83266B38..0x83266B78)
	// 83266B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266B4C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266B50: 386AB868  addi r3, r10, -0x4798
	ctx.r[3].s64 = ctx.r[10].s64 + -18328;
	// 83266B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266B58: 4AFC6379  bl 0x8222ced0
	ctx.lr = 0x83266B5C;
	sub_8222CED0(ctx, base);
	// 83266B5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266B60: 3869D7E0  addi r3, r9, -0x2820
	ctx.r[3].s64 = ctx.r[9].s64 + -10272;
	// 83266B64: 4BA433BD  bl 0x82ca9f20
	ctx.lr = 0x83266B68;
	sub_82CA9F20(ctx, base);
	// 83266B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266B78 size=64
    let mut pc: u32 = 0x83266B78;
    'dispatch: loop {
        match pc {
            0x83266B78 => {
    //   block [0x83266B78..0x83266BB8)
	// 83266B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266B8C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266B90: 386AB86C  addi r3, r10, -0x4794
	ctx.r[3].s64 = ctx.r[10].s64 + -18324;
	// 83266B94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266B98: 4AFC6339  bl 0x8222ced0
	ctx.lr = 0x83266B9C;
	sub_8222CED0(ctx, base);
	// 83266B9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266BA0: 3869D7F0  addi r3, r9, -0x2810
	ctx.r[3].s64 = ctx.r[9].s64 + -10256;
	// 83266BA4: 4BA4337D  bl 0x82ca9f20
	ctx.lr = 0x83266BA8;
	sub_82CA9F20(ctx, base);
	// 83266BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266BB8 size=56
    let mut pc: u32 = 0x83266BB8;
    'dispatch: loop {
        match pc {
            0x83266BB8 => {
    //   block [0x83266BB8..0x83266BF0)
	// 83266BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266BCC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83266BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266BD4: 4AF8D185  bl 0x821f3d58
	ctx.lr = 0x83266BD8;
	sub_821F3D58(ctx, base);
	// 83266BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266BDC: 906AB870  stw r3, -0x4790(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18320 as u32), ctx.r[3].u32 ) };
	// 83266BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266BF0 size=56
    let mut pc: u32 = 0x83266BF0;
    'dispatch: loop {
        match pc {
            0x83266BF0 => {
    //   block [0x83266BF0..0x83266C28)
	// 83266BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266C00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266C04: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83266C08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266C0C: 4AF8D14D  bl 0x821f3d58
	ctx.lr = 0x83266C10;
	sub_821F3D58(ctx, base);
	// 83266C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266C14: 906AB874  stw r3, -0x478c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18316 as u32), ctx.r[3].u32 ) };
	// 83266C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266C28 size=56
    let mut pc: u32 = 0x83266C28;
    'dispatch: loop {
        match pc {
            0x83266C28 => {
    //   block [0x83266C28..0x83266C60)
	// 83266C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266C34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266C38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266C3C: 386B3508  addi r3, r11, 0x3508
	ctx.r[3].s64 = ctx.r[11].s64 + 13576;
	// 83266C40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266C44: 4AF8D115  bl 0x821f3d58
	ctx.lr = 0x83266C48;
	sub_821F3D58(ctx, base);
	// 83266C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266C4C: 906AB878  stw r3, -0x4788(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18312 as u32), ctx.r[3].u32 ) };
	// 83266C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266C60 size=56
    let mut pc: u32 = 0x83266C60;
    'dispatch: loop {
        match pc {
            0x83266C60 => {
    //   block [0x83266C60..0x83266C98)
	// 83266C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266C6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266C70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266C74: 386B3520  addi r3, r11, 0x3520
	ctx.r[3].s64 = ctx.r[11].s64 + 13600;
	// 83266C78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266C7C: 4AF8D0DD  bl 0x821f3d58
	ctx.lr = 0x83266C80;
	sub_821F3D58(ctx, base);
	// 83266C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266C84: 906AB87C  stw r3, -0x4784(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18308 as u32), ctx.r[3].u32 ) };
	// 83266C88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266C98 size=56
    let mut pc: u32 = 0x83266C98;
    'dispatch: loop {
        match pc {
            0x83266C98 => {
    //   block [0x83266C98..0x83266CD0)
	// 83266C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266CA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266CA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266CAC: 386B3530  addi r3, r11, 0x3530
	ctx.r[3].s64 = ctx.r[11].s64 + 13616;
	// 83266CB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266CB4: 4AF8D0A5  bl 0x821f3d58
	ctx.lr = 0x83266CB8;
	sub_821F3D58(ctx, base);
	// 83266CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266CBC: 906AB880  stw r3, -0x4780(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18304 as u32), ctx.r[3].u32 ) };
	// 83266CC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266CD0 size=56
    let mut pc: u32 = 0x83266CD0;
    'dispatch: loop {
        match pc {
            0x83266CD0 => {
    //   block [0x83266CD0..0x83266D08)
	// 83266CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266CDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266CE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266CE4: 386B3544  addi r3, r11, 0x3544
	ctx.r[3].s64 = ctx.r[11].s64 + 13636;
	// 83266CE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266CEC: 4AF8D06D  bl 0x821f3d58
	ctx.lr = 0x83266CF0;
	sub_821F3D58(ctx, base);
	// 83266CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266CF4: 906AB884  stw r3, -0x477c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18300 as u32), ctx.r[3].u32 ) };
	// 83266CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266D08 size=56
    let mut pc: u32 = 0x83266D08;
    'dispatch: loop {
        match pc {
            0x83266D08 => {
    //   block [0x83266D08..0x83266D40)
	// 83266D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266D14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266D18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266D1C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83266D20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266D24: 4AF8D035  bl 0x821f3d58
	ctx.lr = 0x83266D28;
	sub_821F3D58(ctx, base);
	// 83266D28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266D2C: 906AB888  stw r3, -0x4778(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18296 as u32), ctx.r[3].u32 ) };
	// 83266D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266D40 size=56
    let mut pc: u32 = 0x83266D40;
    'dispatch: loop {
        match pc {
            0x83266D40 => {
    //   block [0x83266D40..0x83266D78)
	// 83266D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266D4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266D50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266D54: 386B355C  addi r3, r11, 0x355c
	ctx.r[3].s64 = ctx.r[11].s64 + 13660;
	// 83266D58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266D5C: 4AF8CFFD  bl 0x821f3d58
	ctx.lr = 0x83266D60;
	sub_821F3D58(ctx, base);
	// 83266D60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266D64: 906AB88C  stw r3, -0x4774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18292 as u32), ctx.r[3].u32 ) };
	// 83266D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266D78 size=64
    let mut pc: u32 = 0x83266D78;
    'dispatch: loop {
        match pc {
            0x83266D78 => {
    //   block [0x83266D78..0x83266DB8)
	// 83266D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266D8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266D90: 386AB890  addi r3, r10, -0x4770
	ctx.r[3].s64 = ctx.r[10].s64 + -18288;
	// 83266D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266D98: 4AFC6139  bl 0x8222ced0
	ctx.lr = 0x83266D9C;
	sub_8222CED0(ctx, base);
	// 83266D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266DA0: 3869D800  addi r3, r9, -0x2800
	ctx.r[3].s64 = ctx.r[9].s64 + -10240;
	// 83266DA4: 4BA4317D  bl 0x82ca9f20
	ctx.lr = 0x83266DA8;
	sub_82CA9F20(ctx, base);
	// 83266DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266DB8 size=64
    let mut pc: u32 = 0x83266DB8;
    'dispatch: loop {
        match pc {
            0x83266DB8 => {
    //   block [0x83266DB8..0x83266DF8)
	// 83266DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266DCC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266DD0: 386AB894  addi r3, r10, -0x476c
	ctx.r[3].s64 = ctx.r[10].s64 + -18284;
	// 83266DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266DD8: 4AFC60F9  bl 0x8222ced0
	ctx.lr = 0x83266DDC;
	sub_8222CED0(ctx, base);
	// 83266DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266DE0: 3869D810  addi r3, r9, -0x27f0
	ctx.r[3].s64 = ctx.r[9].s64 + -10224;
	// 83266DE4: 4BA4313D  bl 0x82ca9f20
	ctx.lr = 0x83266DE8;
	sub_82CA9F20(ctx, base);
	// 83266DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266DF8 size=64
    let mut pc: u32 = 0x83266DF8;
    'dispatch: loop {
        match pc {
            0x83266DF8 => {
    //   block [0x83266DF8..0x83266E38)
	// 83266DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266E0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83266E10: 386AB898  addi r3, r10, -0x4768
	ctx.r[3].s64 = ctx.r[10].s64 + -18280;
	// 83266E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266E18: 4AFC60B9  bl 0x8222ced0
	ctx.lr = 0x83266E1C;
	sub_8222CED0(ctx, base);
	// 83266E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266E20: 3869D820  addi r3, r9, -0x27e0
	ctx.r[3].s64 = ctx.r[9].s64 + -10208;
	// 83266E24: 4BA430FD  bl 0x82ca9f20
	ctx.lr = 0x83266E28;
	sub_82CA9F20(ctx, base);
	// 83266E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266E38 size=56
    let mut pc: u32 = 0x83266E38;
    'dispatch: loop {
        match pc {
            0x83266E38 => {
    //   block [0x83266E38..0x83266E70)
	// 83266E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266E44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266E48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266E4C: 386B35C0  addi r3, r11, 0x35c0
	ctx.r[3].s64 = ctx.r[11].s64 + 13760;
	// 83266E50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266E54: 4AF8CF05  bl 0x821f3d58
	ctx.lr = 0x83266E58;
	sub_821F3D58(ctx, base);
	// 83266E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266E5C: 906AB89C  stw r3, -0x4764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18276 as u32), ctx.r[3].u32 ) };
	// 83266E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266E70 size=56
    let mut pc: u32 = 0x83266E70;
    'dispatch: loop {
        match pc {
            0x83266E70 => {
    //   block [0x83266E70..0x83266EA8)
	// 83266E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266E80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266E84: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83266E88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266E8C: 4AF8CECD  bl 0x821f3d58
	ctx.lr = 0x83266E90;
	sub_821F3D58(ctx, base);
	// 83266E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266E94: 906AB8A0  stw r3, -0x4760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18272 as u32), ctx.r[3].u32 ) };
	// 83266E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266EA8 size=56
    let mut pc: u32 = 0x83266EA8;
    'dispatch: loop {
        match pc {
            0x83266EA8 => {
    //   block [0x83266EA8..0x83266EE0)
	// 83266EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266EB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83266EB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266EBC: 386B35D4  addi r3, r11, 0x35d4
	ctx.r[3].s64 = ctx.r[11].s64 + 13780;
	// 83266EC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266EC4: 4AF8CE95  bl 0x821f3d58
	ctx.lr = 0x83266EC8;
	sub_821F3D58(ctx, base);
	// 83266EC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266ECC: 906AB8A4  stw r3, -0x475c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18268 as u32), ctx.r[3].u32 ) };
	// 83266ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266EE0 size=56
    let mut pc: u32 = 0x83266EE0;
    'dispatch: loop {
        match pc {
            0x83266EE0 => {
    //   block [0x83266EE0..0x83266F18)
	// 83266EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266EEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266EF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266EF4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83266EF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266EFC: 4AF8CE5D  bl 0x821f3d58
	ctx.lr = 0x83266F00;
	sub_821F3D58(ctx, base);
	// 83266F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F04: 906AB8A8  stw r3, -0x4758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18264 as u32), ctx.r[3].u32 ) };
	// 83266F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266F18 size=56
    let mut pc: u32 = 0x83266F18;
    'dispatch: loop {
        match pc {
            0x83266F18 => {
    //   block [0x83266F18..0x83266F50)
	// 83266F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266F24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266F28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266F2C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83266F30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266F34: 4AF8CE25  bl 0x821f3d58
	ctx.lr = 0x83266F38;
	sub_821F3D58(ctx, base);
	// 83266F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F3C: 906AB8AC  stw r3, -0x4754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18260 as u32), ctx.r[3].u32 ) };
	// 83266F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266F50 size=56
    let mut pc: u32 = 0x83266F50;
    'dispatch: loop {
        match pc {
            0x83266F50 => {
    //   block [0x83266F50..0x83266F88)
	// 83266F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266F5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83266F60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83266F64: 386B0CA0  addi r3, r11, 0xca0
	ctx.r[3].s64 = ctx.r[11].s64 + 3232;
	// 83266F68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83266F6C: 4AF8CDED  bl 0x821f3d58
	ctx.lr = 0x83266F70;
	sub_821F3D58(ctx, base);
	// 83266F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F74: 906AB8B0  stw r3, -0x4750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18256 as u32), ctx.r[3].u32 ) };
	// 83266F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266F88 size=64
    let mut pc: u32 = 0x83266F88;
    'dispatch: loop {
        match pc {
            0x83266F88 => {
    //   block [0x83266F88..0x83266FC8)
	// 83266F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266F94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266F98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266F9C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83266FA0: 386AB8B4  addi r3, r10, -0x474c
	ctx.r[3].s64 = ctx.r[10].s64 + -18252;
	// 83266FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266FA8: 4AFC5F29  bl 0x8222ced0
	ctx.lr = 0x83266FAC;
	sub_8222CED0(ctx, base);
	// 83266FAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266FB0: 3869D830  addi r3, r9, -0x27d0
	ctx.r[3].s64 = ctx.r[9].s64 + -10192;
	// 83266FB4: 4BA42F6D  bl 0x82ca9f20
	ctx.lr = 0x83266FB8;
	sub_82CA9F20(ctx, base);
	// 83266FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83266FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83266FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83266FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83266FC8 size=64
    let mut pc: u32 = 0x83266FC8;
    'dispatch: loop {
        match pc {
            0x83266FC8 => {
    //   block [0x83266FC8..0x83267008)
	// 83266FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83266FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83266FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83266FD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83266FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83266FDC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83266FE0: 386AB8B8  addi r3, r10, -0x4748
	ctx.r[3].s64 = ctx.r[10].s64 + -18248;
	// 83266FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83266FE8: 4AFC5EE9  bl 0x8222ced0
	ctx.lr = 0x83266FEC;
	sub_8222CED0(ctx, base);
	// 83266FEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83266FF0: 3869D840  addi r3, r9, -0x27c0
	ctx.r[3].s64 = ctx.r[9].s64 + -10176;
	// 83266FF4: 4BA42F2D  bl 0x82ca9f20
	ctx.lr = 0x83266FF8;
	sub_82CA9F20(ctx, base);
	// 83266FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83266FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267008 size=64
    let mut pc: u32 = 0x83267008;
    'dispatch: loop {
        match pc {
            0x83267008 => {
    //   block [0x83267008..0x83267048)
	// 83267008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267014: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326701C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267020: 386AB8BC  addi r3, r10, -0x4744
	ctx.r[3].s64 = ctx.r[10].s64 + -18244;
	// 83267024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267028: 4AFC5EA9  bl 0x8222ced0
	ctx.lr = 0x8326702C;
	sub_8222CED0(ctx, base);
	// 8326702C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267030: 3869D850  addi r3, r9, -0x27b0
	ctx.r[3].s64 = ctx.r[9].s64 + -10160;
	// 83267034: 4BA42EED  bl 0x82ca9f20
	ctx.lr = 0x83267038;
	sub_82CA9F20(ctx, base);
	// 83267038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326703C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267048 size=56
    let mut pc: u32 = 0x83267048;
    'dispatch: loop {
        match pc {
            0x83267048 => {
    //   block [0x83267048..0x83267080)
	// 83267048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326704C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267054: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267058: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326705C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83267060: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267064: 4AF8CCF5  bl 0x821f3d58
	ctx.lr = 0x83267068;
	sub_821F3D58(ctx, base);
	// 83267068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326706C: 906AB8C0  stw r3, -0x4740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18240 as u32), ctx.r[3].u32 ) };
	// 83267070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326707C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267080 size=56
    let mut pc: u32 = 0x83267080;
    'dispatch: loop {
        match pc {
            0x83267080 => {
    //   block [0x83267080..0x832670B8)
	// 83267080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326708C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267090: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267094: 386B365C  addi r3, r11, 0x365c
	ctx.r[3].s64 = ctx.r[11].s64 + 13916;
	// 83267098: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326709C: 4AF8CCBD  bl 0x821f3d58
	ctx.lr = 0x832670A0;
	sub_821F3D58(ctx, base);
	// 832670A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832670A4: 906AB8C4  stw r3, -0x473c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18236 as u32), ctx.r[3].u32 ) };
	// 832670A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832670AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832670B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832670B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832670B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832670B8 size=64
    let mut pc: u32 = 0x832670B8;
    'dispatch: loop {
        match pc {
            0x832670B8 => {
    //   block [0x832670B8..0x832670F8)
	// 832670B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832670BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832670C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832670C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832670C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832670CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832670D0: 386AB8C8  addi r3, r10, -0x4738
	ctx.r[3].s64 = ctx.r[10].s64 + -18232;
	// 832670D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832670D8: 4AFC5DF9  bl 0x8222ced0
	ctx.lr = 0x832670DC;
	sub_8222CED0(ctx, base);
	// 832670DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832670E0: 3869D860  addi r3, r9, -0x27a0
	ctx.r[3].s64 = ctx.r[9].s64 + -10144;
	// 832670E4: 4BA42E3D  bl 0x82ca9f20
	ctx.lr = 0x832670E8;
	sub_82CA9F20(ctx, base);
	// 832670E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832670EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832670F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832670F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832670F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832670F8 size=64
    let mut pc: u32 = 0x832670F8;
    'dispatch: loop {
        match pc {
            0x832670F8 => {
    //   block [0x832670F8..0x83267138)
	// 832670F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832670FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326710C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83267110: 386AB8CC  addi r3, r10, -0x4734
	ctx.r[3].s64 = ctx.r[10].s64 + -18228;
	// 83267114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267118: 4AFC5DB9  bl 0x8222ced0
	ctx.lr = 0x8326711C;
	sub_8222CED0(ctx, base);
	// 8326711C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267120: 3869D870  addi r3, r9, -0x2790
	ctx.r[3].s64 = ctx.r[9].s64 + -10128;
	// 83267124: 4BA42DFD  bl 0x82ca9f20
	ctx.lr = 0x83267128;
	sub_82CA9F20(ctx, base);
	// 83267128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326712C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267138 size=64
    let mut pc: u32 = 0x83267138;
    'dispatch: loop {
        match pc {
            0x83267138 => {
    //   block [0x83267138..0x83267178)
	// 83267138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326714C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267150: 386AB8D0  addi r3, r10, -0x4730
	ctx.r[3].s64 = ctx.r[10].s64 + -18224;
	// 83267154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267158: 4AFC5D79  bl 0x8222ced0
	ctx.lr = 0x8326715C;
	sub_8222CED0(ctx, base);
	// 8326715C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267160: 3869D880  addi r3, r9, -0x2780
	ctx.r[3].s64 = ctx.r[9].s64 + -10112;
	// 83267164: 4BA42DBD  bl 0x82ca9f20
	ctx.lr = 0x83267168;
	sub_82CA9F20(ctx, base);
	// 83267168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326716C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267178 size=56
    let mut pc: u32 = 0x83267178;
    'dispatch: loop {
        match pc {
            0x83267178 => {
    //   block [0x83267178..0x832671B0)
	// 83267178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326717C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326718C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83267190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267194: 4AF8CBC5  bl 0x821f3d58
	ctx.lr = 0x83267198;
	sub_821F3D58(ctx, base);
	// 83267198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326719C: 906AB8D4  stw r3, -0x472c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18220 as u32), ctx.r[3].u32 ) };
	// 832671A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832671A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832671A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832671AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832671B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832671B0 size=56
    let mut pc: u32 = 0x832671B0;
    'dispatch: loop {
        match pc {
            0x832671B0 => {
    //   block [0x832671B0..0x832671E8)
	// 832671B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832671B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832671B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832671BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832671C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832671C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832671C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832671CC: 4AF8CB8D  bl 0x821f3d58
	ctx.lr = 0x832671D0;
	sub_821F3D58(ctx, base);
	// 832671D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832671D4: 906AB8D8  stw r3, -0x4728(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18216 as u32), ctx.r[3].u32 ) };
	// 832671D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832671DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832671E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832671E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832671E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832671E8 size=56
    let mut pc: u32 = 0x832671E8;
    'dispatch: loop {
        match pc {
            0x832671E8 => {
    //   block [0x832671E8..0x83267220)
	// 832671E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832671EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832671F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832671F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832671F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832671FC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83267200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267204: 4AF8CB55  bl 0x821f3d58
	ctx.lr = 0x83267208;
	sub_821F3D58(ctx, base);
	// 83267208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326720C: 906AB8DC  stw r3, -0x4724(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18212 as u32), ctx.r[3].u32 ) };
	// 83267210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326721C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267220 size=56
    let mut pc: u32 = 0x83267220;
    'dispatch: loop {
        match pc {
            0x83267220 => {
    //   block [0x83267220..0x83267258)
	// 83267220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326722C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267234: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83267238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326723C: 4AF8CB1D  bl 0x821f3d58
	ctx.lr = 0x83267240;
	sub_821F3D58(ctx, base);
	// 83267240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267244: 906AB8E0  stw r3, -0x4720(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18208 as u32), ctx.r[3].u32 ) };
	// 83267248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326724C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267258 size=56
    let mut pc: u32 = 0x83267258;
    'dispatch: loop {
        match pc {
            0x83267258 => {
    //   block [0x83267258..0x83267290)
	// 83267258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326725C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267264: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326726C: 386B3C80  addi r3, r11, 0x3c80
	ctx.r[3].s64 = ctx.r[11].s64 + 15488;
	// 83267270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267274: 4AF8CAE5  bl 0x821f3d58
	ctx.lr = 0x83267278;
	sub_821F3D58(ctx, base);
	// 83267278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326727C: 906AB8E4  stw r3, -0x471c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18204 as u32), ctx.r[3].u32 ) };
	// 83267280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326728C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267290 size=64
    let mut pc: u32 = 0x83267290;
    'dispatch: loop {
        match pc {
            0x83267290 => {
    //   block [0x83267290..0x832672D0)
	// 83267290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326729C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832672A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832672A4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832672A8: 386AB8E8  addi r3, r10, -0x4718
	ctx.r[3].s64 = ctx.r[10].s64 + -18200;
	// 832672AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832672B0: 4AFC5C21  bl 0x8222ced0
	ctx.lr = 0x832672B4;
	sub_8222CED0(ctx, base);
	// 832672B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832672B8: 3869D890  addi r3, r9, -0x2770
	ctx.r[3].s64 = ctx.r[9].s64 + -10096;
	// 832672BC: 4BA42C65  bl 0x82ca9f20
	ctx.lr = 0x832672C0;
	sub_82CA9F20(ctx, base);
	// 832672C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832672C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832672C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832672CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832672D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832672D0 size=64
    let mut pc: u32 = 0x832672D0;
    'dispatch: loop {
        match pc {
            0x832672D0 => {
    //   block [0x832672D0..0x83267310)
	// 832672D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832672D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832672D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832672DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832672E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832672E4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832672E8: 386AB8EC  addi r3, r10, -0x4714
	ctx.r[3].s64 = ctx.r[10].s64 + -18196;
	// 832672EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832672F0: 4AFC5BE1  bl 0x8222ced0
	ctx.lr = 0x832672F4;
	sub_8222CED0(ctx, base);
	// 832672F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832672F8: 3869D8A0  addi r3, r9, -0x2760
	ctx.r[3].s64 = ctx.r[9].s64 + -10080;
	// 832672FC: 4BA42C25  bl 0x82ca9f20
	ctx.lr = 0x83267300;
	sub_82CA9F20(ctx, base);
	// 83267300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326730C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267310 size=64
    let mut pc: u32 = 0x83267310;
    'dispatch: loop {
        match pc {
            0x83267310 => {
    //   block [0x83267310..0x83267350)
	// 83267310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326731C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267324: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267328: 386AB8F0  addi r3, r10, -0x4710
	ctx.r[3].s64 = ctx.r[10].s64 + -18192;
	// 8326732C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267330: 4AFC5BA1  bl 0x8222ced0
	ctx.lr = 0x83267334;
	sub_8222CED0(ctx, base);
	// 83267334: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267338: 3869D8B0  addi r3, r9, -0x2750
	ctx.r[3].s64 = ctx.r[9].s64 + -10064;
	// 8326733C: 4BA42BE5  bl 0x82ca9f20
	ctx.lr = 0x83267340;
	sub_82CA9F20(ctx, base);
	// 83267340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326734C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267350 size=64
    let mut pc: u32 = 0x83267350;
    'dispatch: loop {
        match pc {
            0x83267350 => {
    //   block [0x83267350..0x83267390)
	// 83267350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326735C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267364: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267368: 386AB8F4  addi r3, r10, -0x470c
	ctx.r[3].s64 = ctx.r[10].s64 + -18188;
	// 8326736C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267370: 4AFC5B61  bl 0x8222ced0
	ctx.lr = 0x83267374;
	sub_8222CED0(ctx, base);
	// 83267374: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267378: 3869D8C0  addi r3, r9, -0x2740
	ctx.r[3].s64 = ctx.r[9].s64 + -10048;
	// 8326737C: 4BA42BA5  bl 0x82ca9f20
	ctx.lr = 0x83267380;
	sub_82CA9F20(ctx, base);
	// 83267380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326738C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267390 size=64
    let mut pc: u32 = 0x83267390;
    'dispatch: loop {
        match pc {
            0x83267390 => {
    //   block [0x83267390..0x832673D0)
	// 83267390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326739C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832673A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832673A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832673A8: 386AB8F8  addi r3, r10, -0x4708
	ctx.r[3].s64 = ctx.r[10].s64 + -18184;
	// 832673AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832673B0: 4AFC5B21  bl 0x8222ced0
	ctx.lr = 0x832673B4;
	sub_8222CED0(ctx, base);
	// 832673B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832673B8: 3869D8D0  addi r3, r9, -0x2730
	ctx.r[3].s64 = ctx.r[9].s64 + -10032;
	// 832673BC: 4BA42B65  bl 0x82ca9f20
	ctx.lr = 0x832673C0;
	sub_82CA9F20(ctx, base);
	// 832673C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832673C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832673C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832673CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832673D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832673D0 size=64
    let mut pc: u32 = 0x832673D0;
    'dispatch: loop {
        match pc {
            0x832673D0 => {
    //   block [0x832673D0..0x83267410)
	// 832673D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832673D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832673D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832673DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832673E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832673E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832673E8: 386AB8FC  addi r3, r10, -0x4704
	ctx.r[3].s64 = ctx.r[10].s64 + -18180;
	// 832673EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832673F0: 4AFC5AE1  bl 0x8222ced0
	ctx.lr = 0x832673F4;
	sub_8222CED0(ctx, base);
	// 832673F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832673F8: 3869D8E0  addi r3, r9, -0x2720
	ctx.r[3].s64 = ctx.r[9].s64 + -10016;
	// 832673FC: 4BA42B25  bl 0x82ca9f20
	ctx.lr = 0x83267400;
	sub_82CA9F20(ctx, base);
	// 83267400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326740C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267410 size=64
    let mut pc: u32 = 0x83267410;
    'dispatch: loop {
        match pc {
            0x83267410 => {
    //   block [0x83267410..0x83267450)
	// 83267410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326741C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267424: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267428: 386AB900  addi r3, r10, -0x4700
	ctx.r[3].s64 = ctx.r[10].s64 + -18176;
	// 8326742C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267430: 4AFC5AA1  bl 0x8222ced0
	ctx.lr = 0x83267434;
	sub_8222CED0(ctx, base);
	// 83267434: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267438: 3869D8F0  addi r3, r9, -0x2710
	ctx.r[3].s64 = ctx.r[9].s64 + -10000;
	// 8326743C: 4BA42AE5  bl 0x82ca9f20
	ctx.lr = 0x83267440;
	sub_82CA9F20(ctx, base);
	// 83267440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326744C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267450 size=64
    let mut pc: u32 = 0x83267450;
    'dispatch: loop {
        match pc {
            0x83267450 => {
    //   block [0x83267450..0x83267490)
	// 83267450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326745C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267464: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83267468: 386AB904  addi r3, r10, -0x46fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18172;
	// 8326746C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267470: 4AFC5A61  bl 0x8222ced0
	ctx.lr = 0x83267474;
	sub_8222CED0(ctx, base);
	// 83267474: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267478: 3869D900  addi r3, r9, -0x2700
	ctx.r[3].s64 = ctx.r[9].s64 + -9984;
	// 8326747C: 4BA42AA5  bl 0x82ca9f20
	ctx.lr = 0x83267480;
	sub_82CA9F20(ctx, base);
	// 83267480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326748C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267490 size=64
    let mut pc: u32 = 0x83267490;
    'dispatch: loop {
        match pc {
            0x83267490 => {
    //   block [0x83267490..0x832674D0)
	// 83267490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326749C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832674A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832674A4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832674A8: 386AB908  addi r3, r10, -0x46f8
	ctx.r[3].s64 = ctx.r[10].s64 + -18168;
	// 832674AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832674B0: 4AFC5A21  bl 0x8222ced0
	ctx.lr = 0x832674B4;
	sub_8222CED0(ctx, base);
	// 832674B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832674B8: 3869D910  addi r3, r9, -0x26f0
	ctx.r[3].s64 = ctx.r[9].s64 + -9968;
	// 832674BC: 4BA42A65  bl 0x82ca9f20
	ctx.lr = 0x832674C0;
	sub_82CA9F20(ctx, base);
	// 832674C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832674C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832674C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832674CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832674D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832674D0 size=56
    let mut pc: u32 = 0x832674D0;
    'dispatch: loop {
        match pc {
            0x832674D0 => {
    //   block [0x832674D0..0x83267508)
	// 832674D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832674D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832674D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832674DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832674E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832674E4: 386B430C  addi r3, r11, 0x430c
	ctx.r[3].s64 = ctx.r[11].s64 + 17164;
	// 832674E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832674EC: 4AF8C86D  bl 0x821f3d58
	ctx.lr = 0x832674F0;
	sub_821F3D58(ctx, base);
	// 832674F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832674F4: 906AB90C  stw r3, -0x46f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18164 as u32), ctx.r[3].u32 ) };
	// 832674F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832674FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267508 size=56
    let mut pc: u32 = 0x83267508;
    'dispatch: loop {
        match pc {
            0x83267508 => {
    //   block [0x83267508..0x83267540)
	// 83267508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326750C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267514: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326751C: 386B4320  addi r3, r11, 0x4320
	ctx.r[3].s64 = ctx.r[11].s64 + 17184;
	// 83267520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267524: 4AF8C835  bl 0x821f3d58
	ctx.lr = 0x83267528;
	sub_821F3D58(ctx, base);
	// 83267528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326752C: 906AB910  stw r3, -0x46f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18160 as u32), ctx.r[3].u32 ) };
	// 83267530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326753C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267540 size=64
    let mut pc: u32 = 0x83267540;
    'dispatch: loop {
        match pc {
            0x83267540 => {
    //   block [0x83267540..0x83267580)
	// 83267540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326754C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267554: 388BA3AC  addi r4, r11, -0x5c54
	ctx.r[4].s64 = ctx.r[11].s64 + -23636;
	// 83267558: 386AB914  addi r3, r10, -0x46ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18156;
	// 8326755C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267560: 4AFC5971  bl 0x8222ced0
	ctx.lr = 0x83267564;
	sub_8222CED0(ctx, base);
	// 83267564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267568: 3869D920  addi r3, r9, -0x26e0
	ctx.r[3].s64 = ctx.r[9].s64 + -9952;
	// 8326756C: 4BA429B5  bl 0x82ca9f20
	ctx.lr = 0x83267570;
	sub_82CA9F20(ctx, base);
	// 83267570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326757C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267580 size=64
    let mut pc: u32 = 0x83267580;
    'dispatch: loop {
        match pc {
            0x83267580 => {
    //   block [0x83267580..0x832675C0)
	// 83267580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326758C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267594: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267598: 386AB918  addi r3, r10, -0x46e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18152;
	// 8326759C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832675A0: 4AFC5931  bl 0x8222ced0
	ctx.lr = 0x832675A4;
	sub_8222CED0(ctx, base);
	// 832675A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832675A8: 3869D930  addi r3, r9, -0x26d0
	ctx.r[3].s64 = ctx.r[9].s64 + -9936;
	// 832675AC: 4BA42975  bl 0x82ca9f20
	ctx.lr = 0x832675B0;
	sub_82CA9F20(ctx, base);
	// 832675B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832675B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832675B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832675BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832675C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832675C0 size=64
    let mut pc: u32 = 0x832675C0;
    'dispatch: loop {
        match pc {
            0x832675C0 => {
    //   block [0x832675C0..0x83267600)
	// 832675C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832675C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832675C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832675CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832675D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832675D4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832675D8: 386AB91C  addi r3, r10, -0x46e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18148;
	// 832675DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832675E0: 4AFC58F1  bl 0x8222ced0
	ctx.lr = 0x832675E4;
	sub_8222CED0(ctx, base);
	// 832675E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832675E8: 3869D940  addi r3, r9, -0x26c0
	ctx.r[3].s64 = ctx.r[9].s64 + -9920;
	// 832675EC: 4BA42935  bl 0x82ca9f20
	ctx.lr = 0x832675F0;
	sub_82CA9F20(ctx, base);
	// 832675F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832675F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832675F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832675FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267600 size=64
    let mut pc: u32 = 0x83267600;
    'dispatch: loop {
        match pc {
            0x83267600 => {
    //   block [0x83267600..0x83267640)
	// 83267600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326760C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267614: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83267618: 386AB920  addi r3, r10, -0x46e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18144;
	// 8326761C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267620: 4AFC58B1  bl 0x8222ced0
	ctx.lr = 0x83267624;
	sub_8222CED0(ctx, base);
	// 83267624: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267628: 3869D950  addi r3, r9, -0x26b0
	ctx.r[3].s64 = ctx.r[9].s64 + -9904;
	// 8326762C: 4BA428F5  bl 0x82ca9f20
	ctx.lr = 0x83267630;
	sub_82CA9F20(ctx, base);
	// 83267630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326763C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267640 size=64
    let mut pc: u32 = 0x83267640;
    'dispatch: loop {
        match pc {
            0x83267640 => {
    //   block [0x83267640..0x83267680)
	// 83267640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326764C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267654: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83267658: 386AB924  addi r3, r10, -0x46dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18140;
	// 8326765C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267660: 4AFC5871  bl 0x8222ced0
	ctx.lr = 0x83267664;
	sub_8222CED0(ctx, base);
	// 83267664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83267668: 3869D970  addi r3, r9, -0x2690
	ctx.r[3].s64 = ctx.r[9].s64 + -9872;
	// 8326766C: 4BA428B5  bl 0x82ca9f20
	ctx.lr = 0x83267670;
	sub_82CA9F20(ctx, base);
	// 83267670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326767C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267680 size=64
    let mut pc: u32 = 0x83267680;
    'dispatch: loop {
        match pc {
            0x83267680 => {
    //   block [0x83267680..0x832676C0)
	// 83267680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326768C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83267690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267694: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83267698: 386AB928  addi r3, r10, -0x46d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18136;
	// 8326769C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832676A0: 4AFC5831  bl 0x8222ced0
	ctx.lr = 0x832676A4;
	sub_8222CED0(ctx, base);
	// 832676A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832676A8: 3869D980  addi r3, r9, -0x2680
	ctx.r[3].s64 = ctx.r[9].s64 + -9856;
	// 832676AC: 4BA42875  bl 0x82ca9f20
	ctx.lr = 0x832676B0;
	sub_82CA9F20(ctx, base);
	// 832676B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832676B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832676B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832676BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832676C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832676C0 size=64
    let mut pc: u32 = 0x832676C0;
    'dispatch: loop {
        match pc {
            0x832676C0 => {
    //   block [0x832676C0..0x83267700)
	// 832676C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832676C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832676C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832676CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832676D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832676D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832676D8: 386AB92C  addi r3, r10, -0x46d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18132;
	// 832676DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832676E0: 4AFC57F1  bl 0x8222ced0
	ctx.lr = 0x832676E4;
	sub_8222CED0(ctx, base);
	// 832676E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832676E8: 3869D990  addi r3, r9, -0x2670
	ctx.r[3].s64 = ctx.r[9].s64 + -9840;
	// 832676EC: 4BA42835  bl 0x82ca9f20
	ctx.lr = 0x832676F0;
	sub_82CA9F20(ctx, base);
	// 832676F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832676F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832676F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832676FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267700 size=156
    let mut pc: u32 = 0x83267700;
    'dispatch: loop {
        match pc {
            0x83267700 => {
    //   block [0x83267700..0x8326779C)
	// 83267700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326770C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267710: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83267714: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 83267718: 3BEBB930  addi r31, r11, -0x46d0
	ctx.r[31].s64 = ctx.r[11].s64 + -18128;
	// 8326771C: 388A8A00  addi r4, r10, -0x7600
	ctx.r[4].s64 = ctx.r[10].s64 + -30208;
	// 83267720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83267724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267728: 4AFC57A9  bl 0x8222ced0
	ctx.lr = 0x8326772C;
	sub_8222CED0(ctx, base);
	// 8326772C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83267730: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267734: 38898A18  addi r4, r9, -0x75e8
	ctx.r[4].s64 = ctx.r[9].s64 + -30184;
	// 83267738: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8326773C: 4AFC5795  bl 0x8222ced0
	ctx.lr = 0x83267740;
	sub_8222CED0(ctx, base);
	// 83267740: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 83267744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267748: 38888A24  addi r4, r8, -0x75dc
	ctx.r[4].s64 = ctx.r[8].s64 + -30172;
	// 8326774C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83267750: 4AFC5781  bl 0x8222ced0
	ctx.lr = 0x83267754;
	sub_8222CED0(ctx, base);
	// 83267754: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 83267758: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326775C: 38878A38  addi r4, r7, -0x75c8
	ctx.r[4].s64 = ctx.r[7].s64 + -30152;
	// 83267760: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83267764: 4AFC576D  bl 0x8222ced0
	ctx.lr = 0x83267768;
	sub_8222CED0(ctx, base);
	// 83267768: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8326776C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83267770: 38868A48  addi r4, r6, -0x75b8
	ctx.r[4].s64 = ctx.r[6].s64 + -30136;
	// 83267774: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83267778: 4AFC5759  bl 0x8222ced0
	ctx.lr = 0x8326777C;
	sub_8222CED0(ctx, base);
	// 8326777C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83267780: 3865D9A0  addi r3, r5, -0x2660
	ctx.r[3].s64 = ctx.r[5].s64 + -9824;
	// 83267784: 4BA4279D  bl 0x82ca9f20
	ctx.lr = 0x83267788;
	sub_82CA9F20(ctx, base);
	// 83267788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326778C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83267798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832677A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832677A0 size=32
    let mut pc: u32 = 0x832677A0;
    'dispatch: loop {
        match pc {
            0x832677A0 => {
    //   block [0x832677A0..0x832677C0)
	// 832677A0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832677A4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832677A8: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 832677AC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 832677B0: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 832677B4: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832677B8: 916AB944  stw r11, -0x46bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18108 as u32), ctx.r[11].u32 ) };
	// 832677BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832677C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832677C0 size=44
    let mut pc: u32 = 0x832677C0;
    'dispatch: loop {
        match pc {
            0x832677C0 => {
    //   block [0x832677C0..0x832677EC)
	// 832677C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832677C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832677C8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832677CC: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 832677D0: C9AA0DF0  lfd f13, 0xdf0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3568 as u32) ) };
	// 832677D4: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 832677D8: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 832677DC: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 832677E0: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 832677E4: 9169B948  stw r11, -0x46b8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18104 as u32), ctx.r[11].u32 ) };
	// 832677E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832677F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832677F0 size=44
    let mut pc: u32 = 0x832677F0;
    'dispatch: loop {
        match pc {
            0x832677F0 => {
    //   block [0x832677F0..0x8326781C)
	// 832677F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832677F4: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 832677F8: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832677FC: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83267800: C9AA9630  lfd f13, -0x69d0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-27088 as u32) ) };
	// 83267804: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83267808: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326780C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83267810: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83267814: 9169B94C  stw r11, -0x46b4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18100 as u32), ctx.r[11].u32 ) };
	// 83267818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83267820 size=44
    let mut pc: u32 = 0x83267820;
    'dispatch: loop {
        match pc {
            0x83267820 => {
    //   block [0x83267820..0x8326784C)
	// 83267820: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83267824: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83267828: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326782C: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83267830: C9AA11E8  lfd f13, 0x11e8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4584 as u32) ) };
	// 83267834: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83267838: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326783C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83267840: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83267844: 9169B950  stw r11, -0x46b0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18096 as u32), ctx.r[11].u32 ) };
	// 83267848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83267850 size=44
    let mut pc: u32 = 0x83267850;
    'dispatch: loop {
        match pc {
            0x83267850 => {
    //   block [0x83267850..0x8326787C)
	// 83267850: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83267854: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83267858: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326785C: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83267860: C9AA9610  lfd f13, -0x69f0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-27120 as u32) ) };
	// 83267864: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83267868: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326786C: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83267870: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83267874: 9169B954  stw r11, -0x46ac(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18092 as u32), ctx.r[11].u32 ) };
	// 83267878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267880 size=56
    let mut pc: u32 = 0x83267880;
    'dispatch: loop {
        match pc {
            0x83267880 => {
    //   block [0x83267880..0x832678B8)
	// 83267880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326788C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83267890: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267894: 386B0F58  addi r3, r11, 0xf58
	ctx.r[3].s64 = ctx.r[11].s64 + 3928;
	// 83267898: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326789C: 4AF8C4BD  bl 0x821f3d58
	ctx.lr = 0x832678A0;
	sub_821F3D58(ctx, base);
	// 832678A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832678A4: 906AB958  stw r3, -0x46a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18088 as u32), ctx.r[3].u32 ) };
	// 832678A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832678AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832678B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832678B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832678B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832678B8 size=56
    let mut pc: u32 = 0x832678B8;
    'dispatch: loop {
        match pc {
            0x832678B8 => {
    //   block [0x832678B8..0x832678F0)
	// 832678B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832678BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832678C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832678C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832678C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832678CC: 386B5BB4  addi r3, r11, 0x5bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 23476;
	// 832678D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832678D4: 4AF8C485  bl 0x821f3d58
	ctx.lr = 0x832678D8;
	sub_821F3D58(ctx, base);
	// 832678D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832678DC: 906AB95C  stw r3, -0x46a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18084 as u32), ctx.r[3].u32 ) };
	// 832678E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832678E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832678E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832678EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832678F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832678F0 size=56
    let mut pc: u32 = 0x832678F0;
    'dispatch: loop {
        match pc {
            0x832678F0 => {
    //   block [0x832678F0..0x83267928)
	// 832678F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832678F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832678F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832678FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267900: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267904: 386B0484  addi r3, r11, 0x484
	ctx.r[3].s64 = ctx.r[11].s64 + 1156;
	// 83267908: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326790C: 4AF8C44D  bl 0x821f3d58
	ctx.lr = 0x83267910;
	sub_821F3D58(ctx, base);
	// 83267910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267914: 906AB960  stw r3, -0x46a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18080 as u32), ctx.r[3].u32 ) };
	// 83267918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326791C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267928 size=56
    let mut pc: u32 = 0x83267928;
    'dispatch: loop {
        match pc {
            0x83267928 => {
    //   block [0x83267928..0x83267960)
	// 83267928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326792C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267934: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267938: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326793C: 386B5BC8  addi r3, r11, 0x5bc8
	ctx.r[3].s64 = ctx.r[11].s64 + 23496;
	// 83267940: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267944: 4AF8C415  bl 0x821f3d58
	ctx.lr = 0x83267948;
	sub_821F3D58(ctx, base);
	// 83267948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326794C: 906AB964  stw r3, -0x469c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18076 as u32), ctx.r[3].u32 ) };
	// 83267950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326795C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267960 size=56
    let mut pc: u32 = 0x83267960;
    'dispatch: loop {
        match pc {
            0x83267960 => {
    //   block [0x83267960..0x83267998)
	// 83267960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326796C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267970: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267974: 386B5BDC  addi r3, r11, 0x5bdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23516;
	// 83267978: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326797C: 4AF8C3DD  bl 0x821f3d58
	ctx.lr = 0x83267980;
	sub_821F3D58(ctx, base);
	// 83267980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267984: 906AB968  stw r3, -0x4698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18072 as u32), ctx.r[3].u32 ) };
	// 83267988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326798C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267998 size=56
    let mut pc: u32 = 0x83267998;
    'dispatch: loop {
        match pc {
            0x83267998 => {
    //   block [0x83267998..0x832679D0)
	// 83267998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326799C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832679A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832679A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832679A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832679AC: 386B5BE8  addi r3, r11, 0x5be8
	ctx.r[3].s64 = ctx.r[11].s64 + 23528;
	// 832679B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832679B4: 4AF8C3A5  bl 0x821f3d58
	ctx.lr = 0x832679B8;
	sub_821F3D58(ctx, base);
	// 832679B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832679BC: 906AB96C  stw r3, -0x4694(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18068 as u32), ctx.r[3].u32 ) };
	// 832679C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832679C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832679C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832679CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832679D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832679D0 size=56
    let mut pc: u32 = 0x832679D0;
    'dispatch: loop {
        match pc {
            0x832679D0 => {
    //   block [0x832679D0..0x83267A08)
	// 832679D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832679D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832679D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832679DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832679E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832679E4: 386B5C04  addi r3, r11, 0x5c04
	ctx.r[3].s64 = ctx.r[11].s64 + 23556;
	// 832679E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832679EC: 4AF8C36D  bl 0x821f3d58
	ctx.lr = 0x832679F0;
	sub_821F3D58(ctx, base);
	// 832679F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832679F4: 906AB970  stw r3, -0x4690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18064 as u32), ctx.r[3].u32 ) };
	// 832679F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832679FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267A08 size=56
    let mut pc: u32 = 0x83267A08;
    'dispatch: loop {
        match pc {
            0x83267A08 => {
    //   block [0x83267A08..0x83267A40)
	// 83267A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267A10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267A14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267A18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267A1C: 386B29C8  addi r3, r11, 0x29c8
	ctx.r[3].s64 = ctx.r[11].s64 + 10696;
	// 83267A20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267A24: 4AF8C335  bl 0x821f3d58
	ctx.lr = 0x83267A28;
	sub_821F3D58(ctx, base);
	// 83267A28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267A2C: 906AB974  stw r3, -0x468c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18060 as u32), ctx.r[3].u32 ) };
	// 83267A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267A40 size=56
    let mut pc: u32 = 0x83267A40;
    'dispatch: loop {
        match pc {
            0x83267A40 => {
    //   block [0x83267A40..0x83267A78)
	// 83267A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267A4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267A50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267A54: 386BF11C  addi r3, r11, -0xee4
	ctx.r[3].s64 = ctx.r[11].s64 + -3812;
	// 83267A58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267A5C: 4AF8C2FD  bl 0x821f3d58
	ctx.lr = 0x83267A60;
	sub_821F3D58(ctx, base);
	// 83267A60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267A64: 906AB978  stw r3, -0x4688(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18056 as u32), ctx.r[3].u32 ) };
	// 83267A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267A78 size=56
    let mut pc: u32 = 0x83267A78;
    'dispatch: loop {
        match pc {
            0x83267A78 => {
    //   block [0x83267A78..0x83267AB0)
	// 83267A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267A84: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267A88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267A8C: 386B8C24  addi r3, r11, -0x73dc
	ctx.r[3].s64 = ctx.r[11].s64 + -29660;
	// 83267A90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267A94: 4AF8C2C5  bl 0x821f3d58
	ctx.lr = 0x83267A98;
	sub_821F3D58(ctx, base);
	// 83267A98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267A9C: 906AB97C  stw r3, -0x4684(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18052 as u32), ctx.r[3].u32 ) };
	// 83267AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267AB0 size=56
    let mut pc: u32 = 0x83267AB0;
    'dispatch: loop {
        match pc {
            0x83267AB0 => {
    //   block [0x83267AB0..0x83267AE8)
	// 83267AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267ABC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267AC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267AC4: 386B5C1C  addi r3, r11, 0x5c1c
	ctx.r[3].s64 = ctx.r[11].s64 + 23580;
	// 83267AC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267ACC: 4AF8C28D  bl 0x821f3d58
	ctx.lr = 0x83267AD0;
	sub_821F3D58(ctx, base);
	// 83267AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267AD4: 906AB980  stw r3, -0x4680(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18048 as u32), ctx.r[3].u32 ) };
	// 83267AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267AE8 size=56
    let mut pc: u32 = 0x83267AE8;
    'dispatch: loop {
        match pc {
            0x83267AE8 => {
    //   block [0x83267AE8..0x83267B20)
	// 83267AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267AF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267AF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267AFC: 386B5C30  addi r3, r11, 0x5c30
	ctx.r[3].s64 = ctx.r[11].s64 + 23600;
	// 83267B00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267B04: 4AF8C255  bl 0x821f3d58
	ctx.lr = 0x83267B08;
	sub_821F3D58(ctx, base);
	// 83267B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267B0C: 906AB984  stw r3, -0x467c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18044 as u32), ctx.r[3].u32 ) };
	// 83267B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267B20 size=56
    let mut pc: u32 = 0x83267B20;
    'dispatch: loop {
        match pc {
            0x83267B20 => {
    //   block [0x83267B20..0x83267B58)
	// 83267B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267B2C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267B30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267B34: 386BF0FC  addi r3, r11, -0xf04
	ctx.r[3].s64 = ctx.r[11].s64 + -3844;
	// 83267B38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267B3C: 4AF8C21D  bl 0x821f3d58
	ctx.lr = 0x83267B40;
	sub_821F3D58(ctx, base);
	// 83267B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267B44: 906AB988  stw r3, -0x4678(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18040 as u32), ctx.r[3].u32 ) };
	// 83267B48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267B58 size=56
    let mut pc: u32 = 0x83267B58;
    'dispatch: loop {
        match pc {
            0x83267B58 => {
    //   block [0x83267B58..0x83267B90)
	// 83267B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267B64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83267B68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267B6C: 386BE9C4  addi r3, r11, -0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + -5692;
	// 83267B70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267B74: 4AF8C1E5  bl 0x821f3d58
	ctx.lr = 0x83267B78;
	sub_821F3D58(ctx, base);
	// 83267B78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267B7C: 906AB98C  stw r3, -0x4674(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18036 as u32), ctx.r[3].u32 ) };
	// 83267B80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267B90 size=56
    let mut pc: u32 = 0x83267B90;
    'dispatch: loop {
        match pc {
            0x83267B90 => {
    //   block [0x83267B90..0x83267BC8)
	// 83267B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267B9C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267BA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267BA4: 386B8EB4  addi r3, r11, -0x714c
	ctx.r[3].s64 = ctx.r[11].s64 + -29004;
	// 83267BA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267BAC: 4AF8C1AD  bl 0x821f3d58
	ctx.lr = 0x83267BB0;
	sub_821F3D58(ctx, base);
	// 83267BB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267BB4: 906AB990  stw r3, -0x4670(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18032 as u32), ctx.r[3].u32 ) };
	// 83267BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267BC8 size=56
    let mut pc: u32 = 0x83267BC8;
    'dispatch: loop {
        match pc {
            0x83267BC8 => {
    //   block [0x83267BC8..0x83267C00)
	// 83267BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267BD4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267BD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267BDC: 386B5C48  addi r3, r11, 0x5c48
	ctx.r[3].s64 = ctx.r[11].s64 + 23624;
	// 83267BE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267BE4: 4AF8C175  bl 0x821f3d58
	ctx.lr = 0x83267BE8;
	sub_821F3D58(ctx, base);
	// 83267BE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267BEC: 906AB994  stw r3, -0x466c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18028 as u32), ctx.r[3].u32 ) };
	// 83267BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267C00 size=56
    let mut pc: u32 = 0x83267C00;
    'dispatch: loop {
        match pc {
            0x83267C00 => {
    //   block [0x83267C00..0x83267C38)
	// 83267C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267C0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267C10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267C14: 386B5C60  addi r3, r11, 0x5c60
	ctx.r[3].s64 = ctx.r[11].s64 + 23648;
	// 83267C18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267C1C: 4AF8C13D  bl 0x821f3d58
	ctx.lr = 0x83267C20;
	sub_821F3D58(ctx, base);
	// 83267C20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267C24: 906AB998  stw r3, -0x4668(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18024 as u32), ctx.r[3].u32 ) };
	// 83267C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267C38 size=56
    let mut pc: u32 = 0x83267C38;
    'dispatch: loop {
        match pc {
            0x83267C38 => {
    //   block [0x83267C38..0x83267C70)
	// 83267C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267C44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267C48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267C4C: 386B5C78  addi r3, r11, 0x5c78
	ctx.r[3].s64 = ctx.r[11].s64 + 23672;
	// 83267C50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267C54: 4AF8C105  bl 0x821f3d58
	ctx.lr = 0x83267C58;
	sub_821F3D58(ctx, base);
	// 83267C58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267C5C: 906AB99C  stw r3, -0x4664(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18020 as u32), ctx.r[3].u32 ) };
	// 83267C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267C70 size=56
    let mut pc: u32 = 0x83267C70;
    'dispatch: loop {
        match pc {
            0x83267C70 => {
    //   block [0x83267C70..0x83267CA8)
	// 83267C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267C7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267C80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267C84: 386B5C94  addi r3, r11, 0x5c94
	ctx.r[3].s64 = ctx.r[11].s64 + 23700;
	// 83267C88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267C8C: 4AF8C0CD  bl 0x821f3d58
	ctx.lr = 0x83267C90;
	sub_821F3D58(ctx, base);
	// 83267C90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267C94: 906AB9A0  stw r3, -0x4660(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18016 as u32), ctx.r[3].u32 ) };
	// 83267C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267CA8 size=56
    let mut pc: u32 = 0x83267CA8;
    'dispatch: loop {
        match pc {
            0x83267CA8 => {
    //   block [0x83267CA8..0x83267CE0)
	// 83267CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267CB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267CB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267CBC: 386B5CAC  addi r3, r11, 0x5cac
	ctx.r[3].s64 = ctx.r[11].s64 + 23724;
	// 83267CC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267CC4: 4AF8C095  bl 0x821f3d58
	ctx.lr = 0x83267CC8;
	sub_821F3D58(ctx, base);
	// 83267CC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267CCC: 906AB9A4  stw r3, -0x465c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18012 as u32), ctx.r[3].u32 ) };
	// 83267CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267CE0 size=56
    let mut pc: u32 = 0x83267CE0;
    'dispatch: loop {
        match pc {
            0x83267CE0 => {
    //   block [0x83267CE0..0x83267D18)
	// 83267CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267CEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267CF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267CF4: 386B8E7C  addi r3, r11, -0x7184
	ctx.r[3].s64 = ctx.r[11].s64 + -29060;
	// 83267CF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267CFC: 4AF8C05D  bl 0x821f3d58
	ctx.lr = 0x83267D00;
	sub_821F3D58(ctx, base);
	// 83267D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267D04: 906AB9A8  stw r3, -0x4658(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18008 as u32), ctx.r[3].u32 ) };
	// 83267D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D18 size=56
    let mut pc: u32 = 0x83267D18;
    'dispatch: loop {
        match pc {
            0x83267D18 => {
    //   block [0x83267D18..0x83267D50)
	// 83267D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267D24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267D28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267D2C: 386B5CB4  addi r3, r11, 0x5cb4
	ctx.r[3].s64 = ctx.r[11].s64 + 23732;
	// 83267D30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267D34: 4AF8C025  bl 0x821f3d58
	ctx.lr = 0x83267D38;
	sub_821F3D58(ctx, base);
	// 83267D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267D3C: 906AB9AC  stw r3, -0x4654(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18004 as u32), ctx.r[3].u32 ) };
	// 83267D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D50 size=56
    let mut pc: u32 = 0x83267D50;
    'dispatch: loop {
        match pc {
            0x83267D50 => {
    //   block [0x83267D50..0x83267D88)
	// 83267D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267D5C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267D60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267D64: 386B9C20  addi r3, r11, -0x63e0
	ctx.r[3].s64 = ctx.r[11].s64 + -25568;
	// 83267D68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267D6C: 4AF8BFED  bl 0x821f3d58
	ctx.lr = 0x83267D70;
	sub_821F3D58(ctx, base);
	// 83267D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267D74: 906AB9B0  stw r3, -0x4650(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18000 as u32), ctx.r[3].u32 ) };
	// 83267D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267D88 size=56
    let mut pc: u32 = 0x83267D88;
    'dispatch: loop {
        match pc {
            0x83267D88 => {
    //   block [0x83267D88..0x83267DC0)
	// 83267D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267D94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267D98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267D9C: 386B5CC4  addi r3, r11, 0x5cc4
	ctx.r[3].s64 = ctx.r[11].s64 + 23748;
	// 83267DA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267DA4: 4AF8BFB5  bl 0x821f3d58
	ctx.lr = 0x83267DA8;
	sub_821F3D58(ctx, base);
	// 83267DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267DAC: 906AB9B4  stw r3, -0x464c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17996 as u32), ctx.r[3].u32 ) };
	// 83267DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267DC0 size=56
    let mut pc: u32 = 0x83267DC0;
    'dispatch: loop {
        match pc {
            0x83267DC0 => {
    //   block [0x83267DC0..0x83267DF8)
	// 83267DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267DCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267DD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267DD4: 386B5CDC  addi r3, r11, 0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23772;
	// 83267DD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267DDC: 4AF8BF7D  bl 0x821f3d58
	ctx.lr = 0x83267DE0;
	sub_821F3D58(ctx, base);
	// 83267DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267DE4: 906AB9B8  stw r3, -0x4648(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17992 as u32), ctx.r[3].u32 ) };
	// 83267DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267DF8 size=56
    let mut pc: u32 = 0x83267DF8;
    'dispatch: loop {
        match pc {
            0x83267DF8 => {
    //   block [0x83267DF8..0x83267E30)
	// 83267DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267E04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267E08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267E0C: 386BA50C  addi r3, r11, -0x5af4
	ctx.r[3].s64 = ctx.r[11].s64 + -23284;
	// 83267E10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267E14: 4AF8BF45  bl 0x821f3d58
	ctx.lr = 0x83267E18;
	sub_821F3D58(ctx, base);
	// 83267E18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267E1C: 906AB9BC  stw r3, -0x4644(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17988 as u32), ctx.r[3].u32 ) };
	// 83267E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267E30 size=56
    let mut pc: u32 = 0x83267E30;
    'dispatch: loop {
        match pc {
            0x83267E30 => {
    //   block [0x83267E30..0x83267E68)
	// 83267E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267E3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267E40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267E44: 386B5CF4  addi r3, r11, 0x5cf4
	ctx.r[3].s64 = ctx.r[11].s64 + 23796;
	// 83267E48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267E4C: 4AF8BF0D  bl 0x821f3d58
	ctx.lr = 0x83267E50;
	sub_821F3D58(ctx, base);
	// 83267E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267E54: 906AB9C0  stw r3, -0x4640(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17984 as u32), ctx.r[3].u32 ) };
	// 83267E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267E68 size=56
    let mut pc: u32 = 0x83267E68;
    'dispatch: loop {
        match pc {
            0x83267E68 => {
    //   block [0x83267E68..0x83267EA0)
	// 83267E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267E74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267E78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267E7C: 386B5D08  addi r3, r11, 0x5d08
	ctx.r[3].s64 = ctx.r[11].s64 + 23816;
	// 83267E80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267E84: 4AF8BED5  bl 0x821f3d58
	ctx.lr = 0x83267E88;
	sub_821F3D58(ctx, base);
	// 83267E88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267E8C: 906AB9C4  stw r3, -0x463c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17980 as u32), ctx.r[3].u32 ) };
	// 83267E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267EA0 size=56
    let mut pc: u32 = 0x83267EA0;
    'dispatch: loop {
        match pc {
            0x83267EA0 => {
    //   block [0x83267EA0..0x83267ED8)
	// 83267EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267EAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267EB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267EB4: 386B5D18  addi r3, r11, 0x5d18
	ctx.r[3].s64 = ctx.r[11].s64 + 23832;
	// 83267EB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267EBC: 4AF8BE9D  bl 0x821f3d58
	ctx.lr = 0x83267EC0;
	sub_821F3D58(ctx, base);
	// 83267EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267EC4: 906AB9C8  stw r3, -0x4638(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17976 as u32), ctx.r[3].u32 ) };
	// 83267EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267ED8 size=56
    let mut pc: u32 = 0x83267ED8;
    'dispatch: loop {
        match pc {
            0x83267ED8 => {
    //   block [0x83267ED8..0x83267F10)
	// 83267ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267EE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267EE4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267EE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267EEC: 386B9CE0  addi r3, r11, -0x6320
	ctx.r[3].s64 = ctx.r[11].s64 + -25376;
	// 83267EF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267EF4: 4AF8BE65  bl 0x821f3d58
	ctx.lr = 0x83267EF8;
	sub_821F3D58(ctx, base);
	// 83267EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267EFC: 906AB9CC  stw r3, -0x4634(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17972 as u32), ctx.r[3].u32 ) };
	// 83267F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F10 size=56
    let mut pc: u32 = 0x83267F10;
    'dispatch: loop {
        match pc {
            0x83267F10 => {
    //   block [0x83267F10..0x83267F48)
	// 83267F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267F1C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267F20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267F24: 386B9CF8  addi r3, r11, -0x6308
	ctx.r[3].s64 = ctx.r[11].s64 + -25352;
	// 83267F28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267F2C: 4AF8BE2D  bl 0x821f3d58
	ctx.lr = 0x83267F30;
	sub_821F3D58(ctx, base);
	// 83267F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267F34: 906AB9D0  stw r3, -0x4630(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17968 as u32), ctx.r[3].u32 ) };
	// 83267F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F48 size=56
    let mut pc: u32 = 0x83267F48;
    'dispatch: loop {
        match pc {
            0x83267F48 => {
    //   block [0x83267F48..0x83267F80)
	// 83267F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267F54: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83267F58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267F5C: 386B9CEC  addi r3, r11, -0x6314
	ctx.r[3].s64 = ctx.r[11].s64 + -25364;
	// 83267F60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267F64: 4AF8BDF5  bl 0x821f3d58
	ctx.lr = 0x83267F68;
	sub_821F3D58(ctx, base);
	// 83267F68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267F6C: 906AB9D4  stw r3, -0x462c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17964 as u32), ctx.r[3].u32 ) };
	// 83267F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267F80 size=56
    let mut pc: u32 = 0x83267F80;
    'dispatch: loop {
        match pc {
            0x83267F80 => {
    //   block [0x83267F80..0x83267FB8)
	// 83267F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267F8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267F90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267F94: 386B5D28  addi r3, r11, 0x5d28
	ctx.r[3].s64 = ctx.r[11].s64 + 23848;
	// 83267F98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267F9C: 4AF8BDBD  bl 0x821f3d58
	ctx.lr = 0x83267FA0;
	sub_821F3D58(ctx, base);
	// 83267FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267FA4: 906AB9D8  stw r3, -0x4628(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17960 as u32), ctx.r[3].u32 ) };
	// 83267FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267FB8 size=56
    let mut pc: u32 = 0x83267FB8;
    'dispatch: loop {
        match pc {
            0x83267FB8 => {
    //   block [0x83267FB8..0x83267FF0)
	// 83267FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267FC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83267FC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83267FCC: 386B5D38  addi r3, r11, 0x5d38
	ctx.r[3].s64 = ctx.r[11].s64 + 23864;
	// 83267FD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83267FD4: 4AF8BD85  bl 0x821f3d58
	ctx.lr = 0x83267FD8;
	sub_821F3D58(ctx, base);
	// 83267FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83267FDC: 906AB9DC  stw r3, -0x4624(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17956 as u32), ctx.r[3].u32 ) };
	// 83267FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83267FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83267FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83267FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83267FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83267FF0 size=56
    let mut pc: u32 = 0x83267FF0;
    'dispatch: loop {
        match pc {
            0x83267FF0 => {
    //   block [0x83267FF0..0x83268028)
	// 83267FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83267FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83267FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83267FFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268000: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268004: 386B5D48  addi r3, r11, 0x5d48
	ctx.r[3].s64 = ctx.r[11].s64 + 23880;
	// 83268008: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326800C: 4AF8BD4D  bl 0x821f3d58
	ctx.lr = 0x83268010;
	sub_821F3D58(ctx, base);
	// 83268010: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268014: 906AB9E0  stw r3, -0x4620(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17952 as u32), ctx.r[3].u32 ) };
	// 83268018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268028 size=56
    let mut pc: u32 = 0x83268028;
    'dispatch: loop {
        match pc {
            0x83268028 => {
    //   block [0x83268028..0x83268060)
	// 83268028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83268034: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268038: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326803C: 386B5D58  addi r3, r11, 0x5d58
	ctx.r[3].s64 = ctx.r[11].s64 + 23896;
	// 83268040: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83268044: 4AF8BD15  bl 0x821f3d58
	ctx.lr = 0x83268048;
	sub_821F3D58(ctx, base);
	// 83268048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326804C: 906AB9E4  stw r3, -0x461c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17948 as u32), ctx.r[3].u32 ) };
	// 83268050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83268054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326805C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83268060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83268060 size=56
    let mut pc: u32 = 0x83268060;
    'dispatch: loop {
        match pc {
            0x83268060 => {
    //   block [0x83268060..0x83268098)
	// 83268060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83268064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83268068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326806C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83268070: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83268074: 386B5D68  addi r3, r11, 0x5d68
	ctx.r[3].s64 = ctx.r[11].s64 + 23912;
	// 83268078: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326807C: 4AF8BCDD  bl 0x821f3d58
	ctx.lr = 0x83268080;
	sub_821F3D58(ctx, base);
	// 83268080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83268084: 906AB9E8  stw r3, -0x4618(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17944 as u32), ctx.r[3].u32 ) };
	// 83268088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326808C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83268090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83268094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


