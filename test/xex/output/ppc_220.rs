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
    //   block [0x83278A78..0x83278AD0)
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
	pc = 0x83278AD0; continue 'dispatch;
            }
            0x83278AD0 => {
    //   block [0x83278AD0..0x83278AFC)
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
	pc = 0x83278AFC; continue 'dispatch;
            }
            0x83278AFC => {
    //   block [0x83278AFC..0x83278B38)
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
    //   block [0x83278B38..0x83278B90)
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
	pc = 0x83278B90; continue 'dispatch;
            }
            0x83278B90 => {
    //   block [0x83278B90..0x83278BBC)
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
	pc = 0x83278BBC; continue 'dispatch;
            }
            0x83278BBC => {
    //   block [0x83278BBC..0x83278BF8)
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
    //   block [0x83278BF8..0x83278C50)
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
	pc = 0x83278C50; continue 'dispatch;
            }
            0x83278C50 => {
    //   block [0x83278C50..0x83278C7C)
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
	pc = 0x83278C7C; continue 'dispatch;
            }
            0x83278C7C => {
    //   block [0x83278C7C..0x83278CB8)
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
    //   block [0x83278CB8..0x83278D10)
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
	pc = 0x83278D10; continue 'dispatch;
            }
            0x83278D10 => {
    //   block [0x83278D10..0x83278D3C)
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
	pc = 0x83278D3C; continue 'dispatch;
            }
            0x83278D3C => {
    //   block [0x83278D3C..0x83278D78)
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
    //   block [0x83278D78..0x83278DD0)
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
	pc = 0x83278DD0; continue 'dispatch;
            }
            0x83278DD0 => {
    //   block [0x83278DD0..0x83278DFC)
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
	pc = 0x83278DFC; continue 'dispatch;
            }
            0x83278DFC => {
    //   block [0x83278DFC..0x83278E38)
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
    //   block [0x83278E38..0x83278E90)
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
	pc = 0x83278E90; continue 'dispatch;
            }
            0x83278E90 => {
    //   block [0x83278E90..0x83278EBC)
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
	pc = 0x83278EBC; continue 'dispatch;
            }
            0x83278EBC => {
    //   block [0x83278EBC..0x83278EF8)
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
    //   block [0x83278EF8..0x83278F50)
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
	pc = 0x83278F50; continue 'dispatch;
            }
            0x83278F50 => {
    //   block [0x83278F50..0x83278F7C)
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
	pc = 0x83278F7C; continue 'dispatch;
            }
            0x83278F7C => {
    //   block [0x83278F7C..0x83278FB8)
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
    //   block [0x83278FB8..0x83279010)
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
	pc = 0x83279010; continue 'dispatch;
            }
            0x83279010 => {
    //   block [0x83279010..0x8327903C)
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
	pc = 0x8327903C; continue 'dispatch;
            }
            0x8327903C => {
    //   block [0x8327903C..0x83279078)
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
    //   block [0x83279078..0x832790D0)
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
	pc = 0x832790D0; continue 'dispatch;
            }
            0x832790D0 => {
    //   block [0x832790D0..0x832790FC)
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
	pc = 0x832790FC; continue 'dispatch;
            }
            0x832790FC => {
    //   block [0x832790FC..0x83279138)
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
    //   block [0x83279138..0x83279190)
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
	pc = 0x83279190; continue 'dispatch;
            }
            0x83279190 => {
    //   block [0x83279190..0x832791BC)
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
	pc = 0x832791BC; continue 'dispatch;
            }
            0x832791BC => {
    //   block [0x832791BC..0x832791F8)
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
    //   block [0x832791F8..0x83279250)
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
	pc = 0x83279250; continue 'dispatch;
            }
            0x83279250 => {
    //   block [0x83279250..0x8327927C)
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
	pc = 0x8327927C; continue 'dispatch;
            }
            0x8327927C => {
    //   block [0x8327927C..0x832792B8)
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


