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


pub fn sub_8327B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B3A0 size=56
    let mut pc: u32 = 0x8327B3A0;
    'dispatch: loop {
        match pc {
            0x8327B3A0 => {
    //   block [0x8327B3A0..0x8327B3D8)
	// 8327B3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B3A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B3AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B3B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B3B4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327B3B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B3BC: 4AF7899D  bl 0x821f3d58
	ctx.lr = 0x8327B3C0;
	sub_821F3D58(ctx, base);
	// 8327B3C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B3C4: 906AD984  stw r3, -0x267c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9852 as u32), ctx.r[3].u32 ) };
	// 8327B3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B3D8 size=64
    let mut pc: u32 = 0x8327B3D8;
    'dispatch: loop {
        match pc {
            0x8327B3D8 => {
    //   block [0x8327B3D8..0x8327B418)
	// 8327B3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B3E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B3E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B3E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B3EC: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 8327B3F0: 386AD988  addi r3, r10, -0x2678
	ctx.r[3].s64 = ctx.r[10].s64 + -9848;
	// 8327B3F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B3F8: 4AFB1AD9  bl 0x8222ced0
	ctx.lr = 0x8327B3FC;
	sub_8222CED0(ctx, base);
	// 8327B3FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B400: 38690248  addi r3, r9, 0x248
	ctx.r[3].s64 = ctx.r[9].s64 + 584;
	// 8327B404: 4BA2EB1D  bl 0x82ca9f20
	ctx.lr = 0x8327B408;
	sub_82CA9F20(ctx, base);
	// 8327B408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B418 size=64
    let mut pc: u32 = 0x8327B418;
    'dispatch: loop {
        match pc {
            0x8327B418 => {
    //   block [0x8327B418..0x8327B458)
	// 8327B418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B42C: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 8327B430: 386AD98C  addi r3, r10, -0x2674
	ctx.r[3].s64 = ctx.r[10].s64 + -9844;
	// 8327B434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B438: 4AFB1A99  bl 0x8222ced0
	ctx.lr = 0x8327B43C;
	sub_8222CED0(ctx, base);
	// 8327B43C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B440: 38690258  addi r3, r9, 0x258
	ctx.r[3].s64 = ctx.r[9].s64 + 600;
	// 8327B444: 4BA2EADD  bl 0x82ca9f20
	ctx.lr = 0x8327B448;
	sub_82CA9F20(ctx, base);
	// 8327B448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B458 size=64
    let mut pc: u32 = 0x8327B458;
    'dispatch: loop {
        match pc {
            0x8327B458 => {
    //   block [0x8327B458..0x8327B498)
	// 8327B458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B46C: 388BE0E0  addi r4, r11, -0x1f20
	ctx.r[4].s64 = ctx.r[11].s64 + -7968;
	// 8327B470: 386AD990  addi r3, r10, -0x2670
	ctx.r[3].s64 = ctx.r[10].s64 + -9840;
	// 8327B474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B478: 4AFB1A59  bl 0x8222ced0
	ctx.lr = 0x8327B47C;
	sub_8222CED0(ctx, base);
	// 8327B47C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B480: 38690268  addi r3, r9, 0x268
	ctx.r[3].s64 = ctx.r[9].s64 + 616;
	// 8327B484: 4BA2EA9D  bl 0x82ca9f20
	ctx.lr = 0x8327B488;
	sub_82CA9F20(ctx, base);
	// 8327B488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B498 size=64
    let mut pc: u32 = 0x8327B498;
    'dispatch: loop {
        match pc {
            0x8327B498 => {
    //   block [0x8327B498..0x8327B4D8)
	// 8327B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B4A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B4AC: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 8327B4B0: 386AD994  addi r3, r10, -0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + -9836;
	// 8327B4B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B4B8: 4AFB1A19  bl 0x8222ced0
	ctx.lr = 0x8327B4BC;
	sub_8222CED0(ctx, base);
	// 8327B4BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B4C0: 38690278  addi r3, r9, 0x278
	ctx.r[3].s64 = ctx.r[9].s64 + 632;
	// 8327B4C4: 4BA2EA5D  bl 0x82ca9f20
	ctx.lr = 0x8327B4C8;
	sub_82CA9F20(ctx, base);
	// 8327B4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B4D8 size=64
    let mut pc: u32 = 0x8327B4D8;
    'dispatch: loop {
        match pc {
            0x8327B4D8 => {
    //   block [0x8327B4D8..0x8327B518)
	// 8327B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B4E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B4E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B4EC: 388BA8EC  addi r4, r11, -0x5714
	ctx.r[4].s64 = ctx.r[11].s64 + -22292;
	// 8327B4F0: 386AD998  addi r3, r10, -0x2668
	ctx.r[3].s64 = ctx.r[10].s64 + -9832;
	// 8327B4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B4F8: 4AFB19D9  bl 0x8222ced0
	ctx.lr = 0x8327B4FC;
	sub_8222CED0(ctx, base);
	// 8327B4FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B500: 38690288  addi r3, r9, 0x288
	ctx.r[3].s64 = ctx.r[9].s64 + 648;
	// 8327B504: 4BA2EA1D  bl 0x82ca9f20
	ctx.lr = 0x8327B508;
	sub_82CA9F20(ctx, base);
	// 8327B508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B518 size=64
    let mut pc: u32 = 0x8327B518;
    'dispatch: loop {
        match pc {
            0x8327B518 => {
    //   block [0x8327B518..0x8327B558)
	// 8327B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B524: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B52C: 388BE10C  addi r4, r11, -0x1ef4
	ctx.r[4].s64 = ctx.r[11].s64 + -7924;
	// 8327B530: 386AD99C  addi r3, r10, -0x2664
	ctx.r[3].s64 = ctx.r[10].s64 + -9828;
	// 8327B534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B538: 4AFB1999  bl 0x8222ced0
	ctx.lr = 0x8327B53C;
	sub_8222CED0(ctx, base);
	// 8327B53C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B540: 38690298  addi r3, r9, 0x298
	ctx.r[3].s64 = ctx.r[9].s64 + 664;
	// 8327B544: 4BA2E9DD  bl 0x82ca9f20
	ctx.lr = 0x8327B548;
	sub_82CA9F20(ctx, base);
	// 8327B548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B558 size=64
    let mut pc: u32 = 0x8327B558;
    'dispatch: loop {
        match pc {
            0x8327B558 => {
    //   block [0x8327B558..0x8327B598)
	// 8327B558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B564: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327B568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B56C: 388BFD7C  addi r4, r11, -0x284
	ctx.r[4].s64 = ctx.r[11].s64 + -644;
	// 8327B570: 386AD9A0  addi r3, r10, -0x2660
	ctx.r[3].s64 = ctx.r[10].s64 + -9824;
	// 8327B574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B578: 4AFB1959  bl 0x8222ced0
	ctx.lr = 0x8327B57C;
	sub_8222CED0(ctx, base);
	// 8327B57C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B580: 386902A8  addi r3, r9, 0x2a8
	ctx.r[3].s64 = ctx.r[9].s64 + 680;
	// 8327B584: 4BA2E99D  bl 0x82ca9f20
	ctx.lr = 0x8327B588;
	sub_82CA9F20(ctx, base);
	// 8327B588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B598 size=64
    let mut pc: u32 = 0x8327B598;
    'dispatch: loop {
        match pc {
            0x8327B598 => {
    //   block [0x8327B598..0x8327B5D8)
	// 8327B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B5A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327B5A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B5AC: 388BFD38  addi r4, r11, -0x2c8
	ctx.r[4].s64 = ctx.r[11].s64 + -712;
	// 8327B5B0: 386AD9A4  addi r3, r10, -0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + -9820;
	// 8327B5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B5B8: 4AFB1919  bl 0x8222ced0
	ctx.lr = 0x8327B5BC;
	sub_8222CED0(ctx, base);
	// 8327B5BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B5C0: 386902B8  addi r3, r9, 0x2b8
	ctx.r[3].s64 = ctx.r[9].s64 + 696;
	// 8327B5C4: 4BA2E95D  bl 0x82ca9f20
	ctx.lr = 0x8327B5C8;
	sub_82CA9F20(ctx, base);
	// 8327B5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B5D8 size=64
    let mut pc: u32 = 0x8327B5D8;
    'dispatch: loop {
        match pc {
            0x8327B5D8 => {
    //   block [0x8327B5D8..0x8327B618)
	// 8327B5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B5E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B5EC: 388BE144  addi r4, r11, -0x1ebc
	ctx.r[4].s64 = ctx.r[11].s64 + -7868;
	// 8327B5F0: 386AD9A8  addi r3, r10, -0x2658
	ctx.r[3].s64 = ctx.r[10].s64 + -9816;
	// 8327B5F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B5F8: 4AFB18D9  bl 0x8222ced0
	ctx.lr = 0x8327B5FC;
	sub_8222CED0(ctx, base);
	// 8327B5FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B600: 386902C8  addi r3, r9, 0x2c8
	ctx.r[3].s64 = ctx.r[9].s64 + 712;
	// 8327B604: 4BA2E91D  bl 0x82ca9f20
	ctx.lr = 0x8327B608;
	sub_82CA9F20(ctx, base);
	// 8327B608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B618 size=64
    let mut pc: u32 = 0x8327B618;
    'dispatch: loop {
        match pc {
            0x8327B618 => {
    //   block [0x8327B618..0x8327B658)
	// 8327B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B624: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B62C: 388BE180  addi r4, r11, -0x1e80
	ctx.r[4].s64 = ctx.r[11].s64 + -7808;
	// 8327B630: 386AD9AC  addi r3, r10, -0x2654
	ctx.r[3].s64 = ctx.r[10].s64 + -9812;
	// 8327B634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B638: 4AFB1899  bl 0x8222ced0
	ctx.lr = 0x8327B63C;
	sub_8222CED0(ctx, base);
	// 8327B63C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B640: 386902D8  addi r3, r9, 0x2d8
	ctx.r[3].s64 = ctx.r[9].s64 + 728;
	// 8327B644: 4BA2E8DD  bl 0x82ca9f20
	ctx.lr = 0x8327B648;
	sub_82CA9F20(ctx, base);
	// 8327B648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B658 size=64
    let mut pc: u32 = 0x8327B658;
    'dispatch: loop {
        match pc {
            0x8327B658 => {
    //   block [0x8327B658..0x8327B698)
	// 8327B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B664: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B66C: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 8327B670: 386AD9B0  addi r3, r10, -0x2650
	ctx.r[3].s64 = ctx.r[10].s64 + -9808;
	// 8327B674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B678: 4AFB1859  bl 0x8222ced0
	ctx.lr = 0x8327B67C;
	sub_8222CED0(ctx, base);
	// 8327B67C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B680: 386902E8  addi r3, r9, 0x2e8
	ctx.r[3].s64 = ctx.r[9].s64 + 744;
	// 8327B684: 4BA2E89D  bl 0x82ca9f20
	ctx.lr = 0x8327B688;
	sub_82CA9F20(ctx, base);
	// 8327B688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B698 size=64
    let mut pc: u32 = 0x8327B698;
    'dispatch: loop {
        match pc {
            0x8327B698 => {
    //   block [0x8327B698..0x8327B6D8)
	// 8327B698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B6A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B6A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B6A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B6AC: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 8327B6B0: 386AD9B4  addi r3, r10, -0x264c
	ctx.r[3].s64 = ctx.r[10].s64 + -9804;
	// 8327B6B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B6B8: 4AFB1819  bl 0x8222ced0
	ctx.lr = 0x8327B6BC;
	sub_8222CED0(ctx, base);
	// 8327B6BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B6C0: 386902F8  addi r3, r9, 0x2f8
	ctx.r[3].s64 = ctx.r[9].s64 + 760;
	// 8327B6C4: 4BA2E85D  bl 0x82ca9f20
	ctx.lr = 0x8327B6C8;
	sub_82CA9F20(ctx, base);
	// 8327B6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B6D8 size=64
    let mut pc: u32 = 0x8327B6D8;
    'dispatch: loop {
        match pc {
            0x8327B6D8 => {
    //   block [0x8327B6D8..0x8327B718)
	// 8327B6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B6E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B6EC: 388BE268  addi r4, r11, -0x1d98
	ctx.r[4].s64 = ctx.r[11].s64 + -7576;
	// 8327B6F0: 386AD9B8  addi r3, r10, -0x2648
	ctx.r[3].s64 = ctx.r[10].s64 + -9800;
	// 8327B6F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B6F8: 4AFB17D9  bl 0x8222ced0
	ctx.lr = 0x8327B6FC;
	sub_8222CED0(ctx, base);
	// 8327B6FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B700: 38690308  addi r3, r9, 0x308
	ctx.r[3].s64 = ctx.r[9].s64 + 776;
	// 8327B704: 4BA2E81D  bl 0x82ca9f20
	ctx.lr = 0x8327B708;
	sub_82CA9F20(ctx, base);
	// 8327B708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B718 size=64
    let mut pc: u32 = 0x8327B718;
    'dispatch: loop {
        match pc {
            0x8327B718 => {
    //   block [0x8327B718..0x8327B758)
	// 8327B718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B72C: 388BE29C  addi r4, r11, -0x1d64
	ctx.r[4].s64 = ctx.r[11].s64 + -7524;
	// 8327B730: 386AD9BC  addi r3, r10, -0x2644
	ctx.r[3].s64 = ctx.r[10].s64 + -9796;
	// 8327B734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B738: 4AFB1799  bl 0x8222ced0
	ctx.lr = 0x8327B73C;
	sub_8222CED0(ctx, base);
	// 8327B73C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B740: 38690318  addi r3, r9, 0x318
	ctx.r[3].s64 = ctx.r[9].s64 + 792;
	// 8327B744: 4BA2E7DD  bl 0x82ca9f20
	ctx.lr = 0x8327B748;
	sub_82CA9F20(ctx, base);
	// 8327B748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B758 size=64
    let mut pc: u32 = 0x8327B758;
    'dispatch: loop {
        match pc {
            0x8327B758 => {
    //   block [0x8327B758..0x8327B798)
	// 8327B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B764: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B76C: 388BE2D8  addi r4, r11, -0x1d28
	ctx.r[4].s64 = ctx.r[11].s64 + -7464;
	// 8327B770: 386AD9C0  addi r3, r10, -0x2640
	ctx.r[3].s64 = ctx.r[10].s64 + -9792;
	// 8327B774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B778: 4AFB1759  bl 0x8222ced0
	ctx.lr = 0x8327B77C;
	sub_8222CED0(ctx, base);
	// 8327B77C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B780: 38690328  addi r3, r9, 0x328
	ctx.r[3].s64 = ctx.r[9].s64 + 808;
	// 8327B784: 4BA2E79D  bl 0x82ca9f20
	ctx.lr = 0x8327B788;
	sub_82CA9F20(ctx, base);
	// 8327B788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B798 size=64
    let mut pc: u32 = 0x8327B798;
    'dispatch: loop {
        match pc {
            0x8327B798 => {
    //   block [0x8327B798..0x8327B7D8)
	// 8327B798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B7A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B7A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327B7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B7AC: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 8327B7B0: 386AD9C4  addi r3, r10, -0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + -9788;
	// 8327B7B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B7B8: 4AFB1719  bl 0x8222ced0
	ctx.lr = 0x8327B7BC;
	sub_8222CED0(ctx, base);
	// 8327B7BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B7C0: 38690338  addi r3, r9, 0x338
	ctx.r[3].s64 = ctx.r[9].s64 + 824;
	// 8327B7C4: 4BA2E75D  bl 0x82ca9f20
	ctx.lr = 0x8327B7C8;
	sub_82CA9F20(ctx, base);
	// 8327B7C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B7D8 size=64
    let mut pc: u32 = 0x8327B7D8;
    'dispatch: loop {
        match pc {
            0x8327B7D8 => {
    //   block [0x8327B7D8..0x8327B818)
	// 8327B7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B7E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B7EC: 388BE144  addi r4, r11, -0x1ebc
	ctx.r[4].s64 = ctx.r[11].s64 + -7868;
	// 8327B7F0: 386AD9C8  addi r3, r10, -0x2638
	ctx.r[3].s64 = ctx.r[10].s64 + -9784;
	// 8327B7F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B7F8: 4AFB16D9  bl 0x8222ced0
	ctx.lr = 0x8327B7FC;
	sub_8222CED0(ctx, base);
	// 8327B7FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B800: 38690348  addi r3, r9, 0x348
	ctx.r[3].s64 = ctx.r[9].s64 + 840;
	// 8327B804: 4BA2E71D  bl 0x82ca9f20
	ctx.lr = 0x8327B808;
	sub_82CA9F20(ctx, base);
	// 8327B808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B818 size=64
    let mut pc: u32 = 0x8327B818;
    'dispatch: loop {
        match pc {
            0x8327B818 => {
    //   block [0x8327B818..0x8327B858)
	// 8327B818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B824: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B82C: 388BE180  addi r4, r11, -0x1e80
	ctx.r[4].s64 = ctx.r[11].s64 + -7808;
	// 8327B830: 386AD9CC  addi r3, r10, -0x2634
	ctx.r[3].s64 = ctx.r[10].s64 + -9780;
	// 8327B834: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B838: 4AFB1699  bl 0x8222ced0
	ctx.lr = 0x8327B83C;
	sub_8222CED0(ctx, base);
	// 8327B83C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B840: 38690358  addi r3, r9, 0x358
	ctx.r[3].s64 = ctx.r[9].s64 + 856;
	// 8327B844: 4BA2E6DD  bl 0x82ca9f20
	ctx.lr = 0x8327B848;
	sub_82CA9F20(ctx, base);
	// 8327B848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B858 size=64
    let mut pc: u32 = 0x8327B858;
    'dispatch: loop {
        match pc {
            0x8327B858 => {
    //   block [0x8327B858..0x8327B898)
	// 8327B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B864: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B868: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B86C: 388BE310  addi r4, r11, -0x1cf0
	ctx.r[4].s64 = ctx.r[11].s64 + -7408;
	// 8327B870: 386AD9D0  addi r3, r10, -0x2630
	ctx.r[3].s64 = ctx.r[10].s64 + -9776;
	// 8327B874: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B878: 4AFB1659  bl 0x8222ced0
	ctx.lr = 0x8327B87C;
	sub_8222CED0(ctx, base);
	// 8327B87C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B880: 38690368  addi r3, r9, 0x368
	ctx.r[3].s64 = ctx.r[9].s64 + 872;
	// 8327B884: 4BA2E69D  bl 0x82ca9f20
	ctx.lr = 0x8327B888;
	sub_82CA9F20(ctx, base);
	// 8327B888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B898 size=64
    let mut pc: u32 = 0x8327B898;
    'dispatch: loop {
        match pc {
            0x8327B898 => {
    //   block [0x8327B898..0x8327B8D8)
	// 8327B898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B8A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B8A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B8A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B8AC: 388BE348  addi r4, r11, -0x1cb8
	ctx.r[4].s64 = ctx.r[11].s64 + -7352;
	// 8327B8B0: 386AD9D4  addi r3, r10, -0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + -9772;
	// 8327B8B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B8B8: 4AFB1619  bl 0x8222ced0
	ctx.lr = 0x8327B8BC;
	sub_8222CED0(ctx, base);
	// 8327B8BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B8C0: 38690378  addi r3, r9, 0x378
	ctx.r[3].s64 = ctx.r[9].s64 + 888;
	// 8327B8C4: 4BA2E65D  bl 0x82ca9f20
	ctx.lr = 0x8327B8C8;
	sub_82CA9F20(ctx, base);
	// 8327B8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B8D8 size=64
    let mut pc: u32 = 0x8327B8D8;
    'dispatch: loop {
        match pc {
            0x8327B8D8 => {
    //   block [0x8327B8D8..0x8327B918)
	// 8327B8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B8E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B8E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B8EC: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 8327B8F0: 386AD9D8  addi r3, r10, -0x2628
	ctx.r[3].s64 = ctx.r[10].s64 + -9768;
	// 8327B8F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B8F8: 4AFB15D9  bl 0x8222ced0
	ctx.lr = 0x8327B8FC;
	sub_8222CED0(ctx, base);
	// 8327B8FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B900: 38690388  addi r3, r9, 0x388
	ctx.r[3].s64 = ctx.r[9].s64 + 904;
	// 8327B904: 4BA2E61D  bl 0x82ca9f20
	ctx.lr = 0x8327B908;
	sub_82CA9F20(ctx, base);
	// 8327B908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B918 size=64
    let mut pc: u32 = 0x8327B918;
    'dispatch: loop {
        match pc {
            0x8327B918 => {
    //   block [0x8327B918..0x8327B958)
	// 8327B918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B924: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B92C: 388BA8EC  addi r4, r11, -0x5714
	ctx.r[4].s64 = ctx.r[11].s64 + -22292;
	// 8327B930: 386AD9DC  addi r3, r10, -0x2624
	ctx.r[3].s64 = ctx.r[10].s64 + -9764;
	// 8327B934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B938: 4AFB1599  bl 0x8222ced0
	ctx.lr = 0x8327B93C;
	sub_8222CED0(ctx, base);
	// 8327B93C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B940: 38690398  addi r3, r9, 0x398
	ctx.r[3].s64 = ctx.r[9].s64 + 920;
	// 8327B944: 4BA2E5DD  bl 0x82ca9f20
	ctx.lr = 0x8327B948;
	sub_82CA9F20(ctx, base);
	// 8327B948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B958 size=64
    let mut pc: u32 = 0x8327B958;
    'dispatch: loop {
        match pc {
            0x8327B958 => {
    //   block [0x8327B958..0x8327B998)
	// 8327B958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B964: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327B968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B96C: 388BE38C  addi r4, r11, -0x1c74
	ctx.r[4].s64 = ctx.r[11].s64 + -7284;
	// 8327B970: 386AD9E0  addi r3, r10, -0x2620
	ctx.r[3].s64 = ctx.r[10].s64 + -9760;
	// 8327B974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327B978: 4AFB1559  bl 0x8222ced0
	ctx.lr = 0x8327B97C;
	sub_8222CED0(ctx, base);
	// 8327B97C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327B980: 386903A8  addi r3, r9, 0x3a8
	ctx.r[3].s64 = ctx.r[9].s64 + 936;
	// 8327B984: 4BA2E59D  bl 0x82ca9f20
	ctx.lr = 0x8327B988;
	sub_82CA9F20(ctx, base);
	// 8327B988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B998 size=56
    let mut pc: u32 = 0x8327B998;
    'dispatch: loop {
        match pc {
            0x8327B998 => {
    //   block [0x8327B998..0x8327B9D0)
	// 8327B998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B9A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B9A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B9A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B9AC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327B9B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B9B4: 4AF783A5  bl 0x821f3d58
	ctx.lr = 0x8327B9B8;
	sub_821F3D58(ctx, base);
	// 8327B9B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B9BC: 906AD9E4  stw r3, -0x261c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9756 as u32), ctx.r[3].u32 ) };
	// 8327B9C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327B9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327B9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327B9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327B9D0 size=56
    let mut pc: u32 = 0x8327B9D0;
    'dispatch: loop {
        match pc {
            0x8327B9D0 => {
    //   block [0x8327B9D0..0x8327BA08)
	// 8327B9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327B9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327B9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327B9DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327B9E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327B9E4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327B9E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327B9EC: 4AF7836D  bl 0x821f3d58
	ctx.lr = 0x8327B9F0;
	sub_821F3D58(ctx, base);
	// 8327B9F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327B9F4: 906AD9E8  stw r3, -0x2618(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9752 as u32), ctx.r[3].u32 ) };
	// 8327B9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327B9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BA08 size=56
    let mut pc: u32 = 0x8327BA08;
    'dispatch: loop {
        match pc {
            0x8327BA08 => {
    //   block [0x8327BA08..0x8327BA40)
	// 8327BA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BA14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BA18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BA1C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327BA20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BA24: 4AF78335  bl 0x821f3d58
	ctx.lr = 0x8327BA28;
	sub_821F3D58(ctx, base);
	// 8327BA28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BA2C: 906AD9EC  stw r3, -0x2614(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9748 as u32), ctx.r[3].u32 ) };
	// 8327BA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BA40 size=56
    let mut pc: u32 = 0x8327BA40;
    'dispatch: loop {
        match pc {
            0x8327BA40 => {
    //   block [0x8327BA40..0x8327BA78)
	// 8327BA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BA4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BA50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BA54: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327BA58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BA5C: 4AF782FD  bl 0x821f3d58
	ctx.lr = 0x8327BA60;
	sub_821F3D58(ctx, base);
	// 8327BA60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BA64: 906AD9F0  stw r3, -0x2610(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9744 as u32), ctx.r[3].u32 ) };
	// 8327BA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BA78 size=56
    let mut pc: u32 = 0x8327BA78;
    'dispatch: loop {
        match pc {
            0x8327BA78 => {
    //   block [0x8327BA78..0x8327BAB0)
	// 8327BA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BA84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BA88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BA8C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327BA90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BA94: 4AF782C5  bl 0x821f3d58
	ctx.lr = 0x8327BA98;
	sub_821F3D58(ctx, base);
	// 8327BA98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BA9C: 906AD9F4  stw r3, -0x260c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9740 as u32), ctx.r[3].u32 ) };
	// 8327BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BAB0 size=56
    let mut pc: u32 = 0x8327BAB0;
    'dispatch: loop {
        match pc {
            0x8327BAB0 => {
    //   block [0x8327BAB0..0x8327BAE8)
	// 8327BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BAC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BAC4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327BAC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BACC: 4AF7828D  bl 0x821f3d58
	ctx.lr = 0x8327BAD0;
	sub_821F3D58(ctx, base);
	// 8327BAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BAD4: 906AD9F8  stw r3, -0x2608(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9736 as u32), ctx.r[3].u32 ) };
	// 8327BAD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BAE8 size=56
    let mut pc: u32 = 0x8327BAE8;
    'dispatch: loop {
        match pc {
            0x8327BAE8 => {
    //   block [0x8327BAE8..0x8327BB20)
	// 8327BAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BAF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BAF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BAFC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327BB00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BB04: 4AF78255  bl 0x821f3d58
	ctx.lr = 0x8327BB08;
	sub_821F3D58(ctx, base);
	// 8327BB08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BB0C: 906AD9FC  stw r3, -0x2604(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9732 as u32), ctx.r[3].u32 ) };
	// 8327BB10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BB20 size=56
    let mut pc: u32 = 0x8327BB20;
    'dispatch: loop {
        match pc {
            0x8327BB20 => {
    //   block [0x8327BB20..0x8327BB58)
	// 8327BB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BB28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BB2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BB30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BB34: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327BB38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BB3C: 4AF7821D  bl 0x821f3d58
	ctx.lr = 0x8327BB40;
	sub_821F3D58(ctx, base);
	// 8327BB40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BB44: 906ADA00  stw r3, -0x2600(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9728 as u32), ctx.r[3].u32 ) };
	// 8327BB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BB58 size=56
    let mut pc: u32 = 0x8327BB58;
    'dispatch: loop {
        match pc {
            0x8327BB58 => {
    //   block [0x8327BB58..0x8327BB90)
	// 8327BB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BB64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BB68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BB6C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327BB70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BB74: 4AF781E5  bl 0x821f3d58
	ctx.lr = 0x8327BB78;
	sub_821F3D58(ctx, base);
	// 8327BB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BB7C: 906ADA04  stw r3, -0x25fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9724 as u32), ctx.r[3].u32 ) };
	// 8327BB80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BB90 size=56
    let mut pc: u32 = 0x8327BB90;
    'dispatch: loop {
        match pc {
            0x8327BB90 => {
    //   block [0x8327BB90..0x8327BBC8)
	// 8327BB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BB98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BB9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BBA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BBA4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327BBA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BBAC: 4AF781AD  bl 0x821f3d58
	ctx.lr = 0x8327BBB0;
	sub_821F3D58(ctx, base);
	// 8327BBB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BBB4: 906ADA08  stw r3, -0x25f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9720 as u32), ctx.r[3].u32 ) };
	// 8327BBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BBC8 size=56
    let mut pc: u32 = 0x8327BBC8;
    'dispatch: loop {
        match pc {
            0x8327BBC8 => {
    //   block [0x8327BBC8..0x8327BC00)
	// 8327BBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BBD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BBD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BBDC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327BBE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BBE4: 4AF78175  bl 0x821f3d58
	ctx.lr = 0x8327BBE8;
	sub_821F3D58(ctx, base);
	// 8327BBE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BBEC: 906ADA0C  stw r3, -0x25f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9716 as u32), ctx.r[3].u32 ) };
	// 8327BBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BC00 size=56
    let mut pc: u32 = 0x8327BC00;
    'dispatch: loop {
        match pc {
            0x8327BC00 => {
    //   block [0x8327BC00..0x8327BC38)
	// 8327BC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BC0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BC10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BC14: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327BC18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BC1C: 4AF7813D  bl 0x821f3d58
	ctx.lr = 0x8327BC20;
	sub_821F3D58(ctx, base);
	// 8327BC20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BC24: 906ADA10  stw r3, -0x25f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9712 as u32), ctx.r[3].u32 ) };
	// 8327BC28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BC38 size=56
    let mut pc: u32 = 0x8327BC38;
    'dispatch: loop {
        match pc {
            0x8327BC38 => {
    //   block [0x8327BC38..0x8327BC70)
	// 8327BC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BC40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BC44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BC48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BC4C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327BC50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BC54: 4AF78105  bl 0x821f3d58
	ctx.lr = 0x8327BC58;
	sub_821F3D58(ctx, base);
	// 8327BC58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BC5C: 906ADA14  stw r3, -0x25ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9708 as u32), ctx.r[3].u32 ) };
	// 8327BC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BC70 size=56
    let mut pc: u32 = 0x8327BC70;
    'dispatch: loop {
        match pc {
            0x8327BC70 => {
    //   block [0x8327BC70..0x8327BCA8)
	// 8327BC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BC7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BC80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BC84: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327BC88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BC8C: 4AF780CD  bl 0x821f3d58
	ctx.lr = 0x8327BC90;
	sub_821F3D58(ctx, base);
	// 8327BC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BC94: 906ADA18  stw r3, -0x25e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9704 as u32), ctx.r[3].u32 ) };
	// 8327BC98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BCA8 size=56
    let mut pc: u32 = 0x8327BCA8;
    'dispatch: loop {
        match pc {
            0x8327BCA8 => {
    //   block [0x8327BCA8..0x8327BCE0)
	// 8327BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BCB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BCB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BCBC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327BCC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BCC4: 4AF78095  bl 0x821f3d58
	ctx.lr = 0x8327BCC8;
	sub_821F3D58(ctx, base);
	// 8327BCC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BCCC: 906ADA1C  stw r3, -0x25e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9700 as u32), ctx.r[3].u32 ) };
	// 8327BCD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BCD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BCD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BCDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BCE0 size=56
    let mut pc: u32 = 0x8327BCE0;
    'dispatch: loop {
        match pc {
            0x8327BCE0 => {
    //   block [0x8327BCE0..0x8327BD18)
	// 8327BCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BCE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BCEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BCF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BCF4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327BCF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BCFC: 4AF7805D  bl 0x821f3d58
	ctx.lr = 0x8327BD00;
	sub_821F3D58(ctx, base);
	// 8327BD00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BD04: 906ADA20  stw r3, -0x25e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9696 as u32), ctx.r[3].u32 ) };
	// 8327BD08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BD18 size=56
    let mut pc: u32 = 0x8327BD18;
    'dispatch: loop {
        match pc {
            0x8327BD18 => {
    //   block [0x8327BD18..0x8327BD50)
	// 8327BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BD24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BD28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BD2C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327BD30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BD34: 4AF78025  bl 0x821f3d58
	ctx.lr = 0x8327BD38;
	sub_821F3D58(ctx, base);
	// 8327BD38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BD3C: 906ADA24  stw r3, -0x25dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9692 as u32), ctx.r[3].u32 ) };
	// 8327BD40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BD50 size=56
    let mut pc: u32 = 0x8327BD50;
    'dispatch: loop {
        match pc {
            0x8327BD50 => {
    //   block [0x8327BD50..0x8327BD88)
	// 8327BD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BD58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BD5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BD60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BD64: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327BD68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BD6C: 4AF77FED  bl 0x821f3d58
	ctx.lr = 0x8327BD70;
	sub_821F3D58(ctx, base);
	// 8327BD70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BD74: 906ADA28  stw r3, -0x25d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9688 as u32), ctx.r[3].u32 ) };
	// 8327BD78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BD88 size=56
    let mut pc: u32 = 0x8327BD88;
    'dispatch: loop {
        match pc {
            0x8327BD88 => {
    //   block [0x8327BD88..0x8327BDC0)
	// 8327BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BD90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BD94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BD98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BD9C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327BDA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BDA4: 4AF77FB5  bl 0x821f3d58
	ctx.lr = 0x8327BDA8;
	sub_821F3D58(ctx, base);
	// 8327BDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BDAC: 906ADA2C  stw r3, -0x25d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9684 as u32), ctx.r[3].u32 ) };
	// 8327BDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BDC0 size=56
    let mut pc: u32 = 0x8327BDC0;
    'dispatch: loop {
        match pc {
            0x8327BDC0 => {
    //   block [0x8327BDC0..0x8327BDF8)
	// 8327BDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BDCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BDD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BDD4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327BDD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BDDC: 4AF77F7D  bl 0x821f3d58
	ctx.lr = 0x8327BDE0;
	sub_821F3D58(ctx, base);
	// 8327BDE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BDE4: 906ADA30  stw r3, -0x25d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9680 as u32), ctx.r[3].u32 ) };
	// 8327BDE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BDF8 size=56
    let mut pc: u32 = 0x8327BDF8;
    'dispatch: loop {
        match pc {
            0x8327BDF8 => {
    //   block [0x8327BDF8..0x8327BE30)
	// 8327BDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BE00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BE04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327BE08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327BE0C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327BE10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327BE14: 4AF77F45  bl 0x821f3d58
	ctx.lr = 0x8327BE18;
	sub_821F3D58(ctx, base);
	// 8327BE18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BE1C: 906ADA34  stw r3, -0x25cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9676 as u32), ctx.r[3].u32 ) };
	// 8327BE20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BE30 size=64
    let mut pc: u32 = 0x8327BE30;
    'dispatch: loop {
        match pc {
            0x8327BE30 => {
    //   block [0x8327BE30..0x8327BE70)
	// 8327BE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BE3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BE40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BE44: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 8327BE48: 386ADA38  addi r3, r10, -0x25c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9672;
	// 8327BE4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BE50: 4AFB1081  bl 0x8222ced0
	ctx.lr = 0x8327BE54;
	sub_8222CED0(ctx, base);
	// 8327BE54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BE58: 386903B8  addi r3, r9, 0x3b8
	ctx.r[3].s64 = ctx.r[9].s64 + 952;
	// 8327BE5C: 4BA2E0C5  bl 0x82ca9f20
	ctx.lr = 0x8327BE60;
	sub_82CA9F20(ctx, base);
	// 8327BE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BE70 size=64
    let mut pc: u32 = 0x8327BE70;
    'dispatch: loop {
        match pc {
            0x8327BE70 => {
    //   block [0x8327BE70..0x8327BEB0)
	// 8327BE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BE7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BE80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BE84: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 8327BE88: 386ADA3C  addi r3, r10, -0x25c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9668;
	// 8327BE8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BE90: 4AFB1041  bl 0x8222ced0
	ctx.lr = 0x8327BE94;
	sub_8222CED0(ctx, base);
	// 8327BE94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BE98: 386903C8  addi r3, r9, 0x3c8
	ctx.r[3].s64 = ctx.r[9].s64 + 968;
	// 8327BE9C: 4BA2E085  bl 0x82ca9f20
	ctx.lr = 0x8327BEA0;
	sub_82CA9F20(ctx, base);
	// 8327BEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BEB0 size=64
    let mut pc: u32 = 0x8327BEB0;
    'dispatch: loop {
        match pc {
            0x8327BEB0 => {
    //   block [0x8327BEB0..0x8327BEF0)
	// 8327BEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BEBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BEC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BEC4: 388BBDF4  addi r4, r11, -0x420c
	ctx.r[4].s64 = ctx.r[11].s64 + -16908;
	// 8327BEC8: 386ADA40  addi r3, r10, -0x25c0
	ctx.r[3].s64 = ctx.r[10].s64 + -9664;
	// 8327BECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BED0: 4AFB1001  bl 0x8222ced0
	ctx.lr = 0x8327BED4;
	sub_8222CED0(ctx, base);
	// 8327BED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BED8: 386903D8  addi r3, r9, 0x3d8
	ctx.r[3].s64 = ctx.r[9].s64 + 984;
	// 8327BEDC: 4BA2E045  bl 0x82ca9f20
	ctx.lr = 0x8327BEE0;
	sub_82CA9F20(ctx, base);
	// 8327BEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BEF0 size=64
    let mut pc: u32 = 0x8327BEF0;
    'dispatch: loop {
        match pc {
            0x8327BEF0 => {
    //   block [0x8327BEF0..0x8327BF30)
	// 8327BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BEF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BEFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BF04: 388BBDC4  addi r4, r11, -0x423c
	ctx.r[4].s64 = ctx.r[11].s64 + -16956;
	// 8327BF08: 386ADA44  addi r3, r10, -0x25bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9660;
	// 8327BF0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BF10: 4AFB0FC1  bl 0x8222ced0
	ctx.lr = 0x8327BF14;
	sub_8222CED0(ctx, base);
	// 8327BF14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BF18: 386903E8  addi r3, r9, 0x3e8
	ctx.r[3].s64 = ctx.r[9].s64 + 1000;
	// 8327BF1C: 4BA2E005  bl 0x82ca9f20
	ctx.lr = 0x8327BF20;
	sub_82CA9F20(ctx, base);
	// 8327BF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BF30 size=64
    let mut pc: u32 = 0x8327BF30;
    'dispatch: loop {
        match pc {
            0x8327BF30 => {
    //   block [0x8327BF30..0x8327BF70)
	// 8327BF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BF3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BF44: 388BBBB8  addi r4, r11, -0x4448
	ctx.r[4].s64 = ctx.r[11].s64 + -17480;
	// 8327BF48: 386ADA48  addi r3, r10, -0x25b8
	ctx.r[3].s64 = ctx.r[10].s64 + -9656;
	// 8327BF4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BF50: 4AFB0F81  bl 0x8222ced0
	ctx.lr = 0x8327BF54;
	sub_8222CED0(ctx, base);
	// 8327BF54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BF58: 386903F8  addi r3, r9, 0x3f8
	ctx.r[3].s64 = ctx.r[9].s64 + 1016;
	// 8327BF5C: 4BA2DFC5  bl 0x82ca9f20
	ctx.lr = 0x8327BF60;
	sub_82CA9F20(ctx, base);
	// 8327BF60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BF70 size=64
    let mut pc: u32 = 0x8327BF70;
    'dispatch: loop {
        match pc {
            0x8327BF70 => {
    //   block [0x8327BF70..0x8327BFB0)
	// 8327BF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BF7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BF80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BF84: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 8327BF88: 386ADA4C  addi r3, r10, -0x25b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9652;
	// 8327BF8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BF90: 4AFB0F41  bl 0x8222ced0
	ctx.lr = 0x8327BF94;
	sub_8222CED0(ctx, base);
	// 8327BF94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BF98: 38690408  addi r3, r9, 0x408
	ctx.r[3].s64 = ctx.r[9].s64 + 1032;
	// 8327BF9C: 4BA2DF85  bl 0x82ca9f20
	ctx.lr = 0x8327BFA0;
	sub_82CA9F20(ctx, base);
	// 8327BFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BFB0 size=64
    let mut pc: u32 = 0x8327BFB0;
    'dispatch: loop {
        match pc {
            0x8327BFB0 => {
    //   block [0x8327BFB0..0x8327BFF0)
	// 8327BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BFB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BFBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327BFC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327BFC4: 388BAD48  addi r4, r11, -0x52b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21176;
	// 8327BFC8: 386ADA50  addi r3, r10, -0x25b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9648;
	// 8327BFCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327BFD0: 4AFB0F01  bl 0x8222ced0
	ctx.lr = 0x8327BFD4;
	sub_8222CED0(ctx, base);
	// 8327BFD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327BFD8: 38690418  addi r3, r9, 0x418
	ctx.r[3].s64 = ctx.r[9].s64 + 1048;
	// 8327BFDC: 4BA2DF45  bl 0x82ca9f20
	ctx.lr = 0x8327BFE0;
	sub_82CA9F20(ctx, base);
	// 8327BFE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327BFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327BFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327BFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327BFF0 size=64
    let mut pc: u32 = 0x8327BFF0;
    'dispatch: loop {
        match pc {
            0x8327BFF0 => {
    //   block [0x8327BFF0..0x8327C030)
	// 8327BFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327BFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327BFF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327BFFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C004: 388BBC28  addi r4, r11, -0x43d8
	ctx.r[4].s64 = ctx.r[11].s64 + -17368;
	// 8327C008: 386ADA54  addi r3, r10, -0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9644;
	// 8327C00C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C010: 4AFB0EC1  bl 0x8222ced0
	ctx.lr = 0x8327C014;
	sub_8222CED0(ctx, base);
	// 8327C014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C018: 38690428  addi r3, r9, 0x428
	ctx.r[3].s64 = ctx.r[9].s64 + 1064;
	// 8327C01C: 4BA2DF05  bl 0x82ca9f20
	ctx.lr = 0x8327C020;
	sub_82CA9F20(ctx, base);
	// 8327C020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C030 size=64
    let mut pc: u32 = 0x8327C030;
    'dispatch: loop {
        match pc {
            0x8327C030 => {
    //   block [0x8327C030..0x8327C070)
	// 8327C030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C03C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C044: 388BAD48  addi r4, r11, -0x52b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21176;
	// 8327C048: 386ADA58  addi r3, r10, -0x25a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9640;
	// 8327C04C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C050: 4AFB0E81  bl 0x8222ced0
	ctx.lr = 0x8327C054;
	sub_8222CED0(ctx, base);
	// 8327C054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C058: 38690438  addi r3, r9, 0x438
	ctx.r[3].s64 = ctx.r[9].s64 + 1080;
	// 8327C05C: 4BA2DEC5  bl 0x82ca9f20
	ctx.lr = 0x8327C060;
	sub_82CA9F20(ctx, base);
	// 8327C060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C070 size=64
    let mut pc: u32 = 0x8327C070;
    'dispatch: loop {
        match pc {
            0x8327C070 => {
    //   block [0x8327C070..0x8327C0B0)
	// 8327C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C07C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C084: 388BBC28  addi r4, r11, -0x43d8
	ctx.r[4].s64 = ctx.r[11].s64 + -17368;
	// 8327C088: 386ADA5C  addi r3, r10, -0x25a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9636;
	// 8327C08C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C090: 4AFB0E41  bl 0x8222ced0
	ctx.lr = 0x8327C094;
	sub_8222CED0(ctx, base);
	// 8327C094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C098: 38690448  addi r3, r9, 0x448
	ctx.r[3].s64 = ctx.r[9].s64 + 1096;
	// 8327C09C: 4BA2DE85  bl 0x82ca9f20
	ctx.lr = 0x8327C0A0;
	sub_82CA9F20(ctx, base);
	// 8327C0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C0B0 size=56
    let mut pc: u32 = 0x8327C0B0;
    'dispatch: loop {
        match pc {
            0x8327C0B0 => {
    //   block [0x8327C0B0..0x8327C0E8)
	// 8327C0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C0BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C0C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C0C4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327C0C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C0CC: 4AF77C8D  bl 0x821f3d58
	ctx.lr = 0x8327C0D0;
	sub_821F3D58(ctx, base);
	// 8327C0D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C0D4: 906ADA60  stw r3, -0x25a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9632 as u32), ctx.r[3].u32 ) };
	// 8327C0D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C0E8 size=56
    let mut pc: u32 = 0x8327C0E8;
    'dispatch: loop {
        match pc {
            0x8327C0E8 => {
    //   block [0x8327C0E8..0x8327C120)
	// 8327C0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C0F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C0F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C0F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C0FC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327C100: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C104: 4AF77C55  bl 0x821f3d58
	ctx.lr = 0x8327C108;
	sub_821F3D58(ctx, base);
	// 8327C108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C10C: 906ADA64  stw r3, -0x259c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9628 as u32), ctx.r[3].u32 ) };
	// 8327C110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C120 size=56
    let mut pc: u32 = 0x8327C120;
    'dispatch: loop {
        match pc {
            0x8327C120 => {
    //   block [0x8327C120..0x8327C158)
	// 8327C120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C12C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C130: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C134: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327C138: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C13C: 4AF77C1D  bl 0x821f3d58
	ctx.lr = 0x8327C140;
	sub_821F3D58(ctx, base);
	// 8327C140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C144: 906ADA68  stw r3, -0x2598(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9624 as u32), ctx.r[3].u32 ) };
	// 8327C148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C158 size=56
    let mut pc: u32 = 0x8327C158;
    'dispatch: loop {
        match pc {
            0x8327C158 => {
    //   block [0x8327C158..0x8327C190)
	// 8327C158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C16C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327C170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C174: 4AF77BE5  bl 0x821f3d58
	ctx.lr = 0x8327C178;
	sub_821F3D58(ctx, base);
	// 8327C178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C17C: 906ADA6C  stw r3, -0x2594(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9620 as u32), ctx.r[3].u32 ) };
	// 8327C180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C190 size=56
    let mut pc: u32 = 0x8327C190;
    'dispatch: loop {
        match pc {
            0x8327C190 => {
    //   block [0x8327C190..0x8327C1C8)
	// 8327C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C1A4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327C1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C1AC: 4AF77BAD  bl 0x821f3d58
	ctx.lr = 0x8327C1B0;
	sub_821F3D58(ctx, base);
	// 8327C1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C1B4: 906ADA70  stw r3, -0x2590(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9616 as u32), ctx.r[3].u32 ) };
	// 8327C1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C1C8 size=56
    let mut pc: u32 = 0x8327C1C8;
    'dispatch: loop {
        match pc {
            0x8327C1C8 => {
    //   block [0x8327C1C8..0x8327C200)
	// 8327C1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C1DC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327C1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C1E4: 4AF77B75  bl 0x821f3d58
	ctx.lr = 0x8327C1E8;
	sub_821F3D58(ctx, base);
	// 8327C1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C1EC: 906ADA74  stw r3, -0x258c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9612 as u32), ctx.r[3].u32 ) };
	// 8327C1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C200 size=56
    let mut pc: u32 = 0x8327C200;
    'dispatch: loop {
        match pc {
            0x8327C200 => {
    //   block [0x8327C200..0x8327C238)
	// 8327C200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C214: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327C218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C21C: 4AF77B3D  bl 0x821f3d58
	ctx.lr = 0x8327C220;
	sub_821F3D58(ctx, base);
	// 8327C220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C224: 906ADA78  stw r3, -0x2588(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9608 as u32), ctx.r[3].u32 ) };
	// 8327C228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C238 size=56
    let mut pc: u32 = 0x8327C238;
    'dispatch: loop {
        match pc {
            0x8327C238 => {
    //   block [0x8327C238..0x8327C270)
	// 8327C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C24C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327C250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C254: 4AF77B05  bl 0x821f3d58
	ctx.lr = 0x8327C258;
	sub_821F3D58(ctx, base);
	// 8327C258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C25C: 906ADA7C  stw r3, -0x2584(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9604 as u32), ctx.r[3].u32 ) };
	// 8327C260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C270 size=56
    let mut pc: u32 = 0x8327C270;
    'dispatch: loop {
        match pc {
            0x8327C270 => {
    //   block [0x8327C270..0x8327C2A8)
	// 8327C270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C284: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327C288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C28C: 4AF77ACD  bl 0x821f3d58
	ctx.lr = 0x8327C290;
	sub_821F3D58(ctx, base);
	// 8327C290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C294: 906ADA80  stw r3, -0x2580(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9600 as u32), ctx.r[3].u32 ) };
	// 8327C298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C2A8 size=56
    let mut pc: u32 = 0x8327C2A8;
    'dispatch: loop {
        match pc {
            0x8327C2A8 => {
    //   block [0x8327C2A8..0x8327C2E0)
	// 8327C2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C2BC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327C2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C2C4: 4AF77A95  bl 0x821f3d58
	ctx.lr = 0x8327C2C8;
	sub_821F3D58(ctx, base);
	// 8327C2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C2CC: 906ADA84  stw r3, -0x257c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9596 as u32), ctx.r[3].u32 ) };
	// 8327C2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C2E0 size=56
    let mut pc: u32 = 0x8327C2E0;
    'dispatch: loop {
        match pc {
            0x8327C2E0 => {
    //   block [0x8327C2E0..0x8327C318)
	// 8327C2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C2F4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327C2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C2FC: 4AF77A5D  bl 0x821f3d58
	ctx.lr = 0x8327C300;
	sub_821F3D58(ctx, base);
	// 8327C300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C304: 906ADA88  stw r3, -0x2578(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9592 as u32), ctx.r[3].u32 ) };
	// 8327C308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C318 size=56
    let mut pc: u32 = 0x8327C318;
    'dispatch: loop {
        match pc {
            0x8327C318 => {
    //   block [0x8327C318..0x8327C350)
	// 8327C318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C32C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327C330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C334: 4AF77A25  bl 0x821f3d58
	ctx.lr = 0x8327C338;
	sub_821F3D58(ctx, base);
	// 8327C338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C33C: 906ADA8C  stw r3, -0x2574(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9588 as u32), ctx.r[3].u32 ) };
	// 8327C340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C350 size=56
    let mut pc: u32 = 0x8327C350;
    'dispatch: loop {
        match pc {
            0x8327C350 => {
    //   block [0x8327C350..0x8327C388)
	// 8327C350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C364: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327C368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C36C: 4AF779ED  bl 0x821f3d58
	ctx.lr = 0x8327C370;
	sub_821F3D58(ctx, base);
	// 8327C370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C374: 906ADA90  stw r3, -0x2570(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9584 as u32), ctx.r[3].u32 ) };
	// 8327C378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C388 size=56
    let mut pc: u32 = 0x8327C388;
    'dispatch: loop {
        match pc {
            0x8327C388 => {
    //   block [0x8327C388..0x8327C3C0)
	// 8327C388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C39C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327C3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C3A4: 4AF779B5  bl 0x821f3d58
	ctx.lr = 0x8327C3A8;
	sub_821F3D58(ctx, base);
	// 8327C3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C3AC: 906ADA94  stw r3, -0x256c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9580 as u32), ctx.r[3].u32 ) };
	// 8327C3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C3C0 size=56
    let mut pc: u32 = 0x8327C3C0;
    'dispatch: loop {
        match pc {
            0x8327C3C0 => {
    //   block [0x8327C3C0..0x8327C3F8)
	// 8327C3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C3D4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327C3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C3DC: 4AF7797D  bl 0x821f3d58
	ctx.lr = 0x8327C3E0;
	sub_821F3D58(ctx, base);
	// 8327C3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C3E4: 906ADA98  stw r3, -0x2568(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9576 as u32), ctx.r[3].u32 ) };
	// 8327C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C3F8 size=56
    let mut pc: u32 = 0x8327C3F8;
    'dispatch: loop {
        match pc {
            0x8327C3F8 => {
    //   block [0x8327C3F8..0x8327C430)
	// 8327C3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C40C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327C410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C414: 4AF77945  bl 0x821f3d58
	ctx.lr = 0x8327C418;
	sub_821F3D58(ctx, base);
	// 8327C418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C41C: 906ADA9C  stw r3, -0x2564(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9572 as u32), ctx.r[3].u32 ) };
	// 8327C420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C430 size=56
    let mut pc: u32 = 0x8327C430;
    'dispatch: loop {
        match pc {
            0x8327C430 => {
    //   block [0x8327C430..0x8327C468)
	// 8327C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C444: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327C448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C44C: 4AF7790D  bl 0x821f3d58
	ctx.lr = 0x8327C450;
	sub_821F3D58(ctx, base);
	// 8327C450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C454: 906ADAA0  stw r3, -0x2560(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9568 as u32), ctx.r[3].u32 ) };
	// 8327C458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C468 size=56
    let mut pc: u32 = 0x8327C468;
    'dispatch: loop {
        match pc {
            0x8327C468 => {
    //   block [0x8327C468..0x8327C4A0)
	// 8327C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C47C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327C480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C484: 4AF778D5  bl 0x821f3d58
	ctx.lr = 0x8327C488;
	sub_821F3D58(ctx, base);
	// 8327C488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C48C: 906ADAA4  stw r3, -0x255c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9564 as u32), ctx.r[3].u32 ) };
	// 8327C490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C4A0 size=56
    let mut pc: u32 = 0x8327C4A0;
    'dispatch: loop {
        match pc {
            0x8327C4A0 => {
    //   block [0x8327C4A0..0x8327C4D8)
	// 8327C4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C4B4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327C4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C4BC: 4AF7789D  bl 0x821f3d58
	ctx.lr = 0x8327C4C0;
	sub_821F3D58(ctx, base);
	// 8327C4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C4C4: 906ADAA8  stw r3, -0x2558(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9560 as u32), ctx.r[3].u32 ) };
	// 8327C4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C4D8 size=56
    let mut pc: u32 = 0x8327C4D8;
    'dispatch: loop {
        match pc {
            0x8327C4D8 => {
    //   block [0x8327C4D8..0x8327C510)
	// 8327C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C4EC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327C4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C4F4: 4AF77865  bl 0x821f3d58
	ctx.lr = 0x8327C4F8;
	sub_821F3D58(ctx, base);
	// 8327C4F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C4FC: 906ADAAC  stw r3, -0x2554(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9556 as u32), ctx.r[3].u32 ) };
	// 8327C500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C510 size=56
    let mut pc: u32 = 0x8327C510;
    'dispatch: loop {
        match pc {
            0x8327C510 => {
    //   block [0x8327C510..0x8327C548)
	// 8327C510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327C520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327C524: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327C528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327C52C: 4AF7782D  bl 0x821f3d58
	ctx.lr = 0x8327C530;
	sub_821F3D58(ctx, base);
	// 8327C530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C534: 906ADAB0  stw r3, -0x2550(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9552 as u32), ctx.r[3].u32 ) };
	// 8327C538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C548 size=376
    let mut pc: u32 = 0x8327C548;
    'dispatch: loop {
        match pc {
            0x8327C548 => {
    //   block [0x8327C548..0x8327C6C0)
	// 8327C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8327C554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8327C55C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C560: 3BEBDAB8  addi r31, r11, -0x2548
	ctx.r[31].s64 = ctx.r[11].s64 + -9544;
	// 8327C564: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 8327C568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8327C56C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C570: 4AFB0961  bl 0x8222ced0
	ctx.lr = 0x8327C574;
	sub_8222CED0(ctx, base);
	// 8327C574: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C578: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C57C: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 8327C580: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8327C584: 4AFB094D  bl 0x8222ced0
	ctx.lr = 0x8327C588;
	sub_8222CED0(ctx, base);
	// 8327C588: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C58C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C590: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 8327C594: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8327C598: 4AFB0939  bl 0x8222ced0
	ctx.lr = 0x8327C59C;
	sub_8222CED0(ctx, base);
	// 8327C59C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C5A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5A4: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 8327C5A8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8327C5AC: 4AFB0925  bl 0x8222ced0
	ctx.lr = 0x8327C5B0;
	sub_8222CED0(ctx, base);
	// 8327C5B0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5B8: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8327C5BC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8327C5C0: 4AFB0911  bl 0x8222ced0
	ctx.lr = 0x8327C5C4;
	sub_8222CED0(ctx, base);
	// 8327C5C4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C5C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5CC: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 8327C5D0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8327C5D4: 4AFB08FD  bl 0x8222ced0
	ctx.lr = 0x8327C5D8;
	sub_8222CED0(ctx, base);
	// 8327C5D8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5E0: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 8327C5E4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8327C5E8: 4AFB08E9  bl 0x8222ced0
	ctx.lr = 0x8327C5EC;
	sub_8222CED0(ctx, base);
	// 8327C5EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C5F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C5F4: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 8327C5F8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8327C5FC: 4AFB08D5  bl 0x8222ced0
	ctx.lr = 0x8327C600;
	sub_8222CED0(ctx, base);
	// 8327C600: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C604: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C608: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8327C60C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8327C610: 4AFB08C1  bl 0x8222ced0
	ctx.lr = 0x8327C614;
	sub_8222CED0(ctx, base);
	// 8327C614: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C618: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C61C: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 8327C620: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8327C624: 4AFB08AD  bl 0x8222ced0
	ctx.lr = 0x8327C628;
	sub_8222CED0(ctx, base);
	// 8327C628: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C62C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C630: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 8327C634: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8327C638: 4AFB0899  bl 0x8222ced0
	ctx.lr = 0x8327C63C;
	sub_8222CED0(ctx, base);
	// 8327C63C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C640: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C644: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 8327C648: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8327C64C: 4AFB0885  bl 0x8222ced0
	ctx.lr = 0x8327C650;
	sub_8222CED0(ctx, base);
	// 8327C650: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C658: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 8327C65C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8327C660: 4AFB0871  bl 0x8222ced0
	ctx.lr = 0x8327C664;
	sub_8222CED0(ctx, base);
	// 8327C664: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C668: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C66C: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 8327C670: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8327C674: 4AFB085D  bl 0x8222ced0
	ctx.lr = 0x8327C678;
	sub_8222CED0(ctx, base);
	// 8327C678: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C67C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C680: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 8327C684: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8327C688: 4AFB0849  bl 0x8222ced0
	ctx.lr = 0x8327C68C;
	sub_8222CED0(ctx, base);
	// 8327C68C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C690: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C694: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 8327C698: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8327C69C: 4AFB0835  bl 0x8222ced0
	ctx.lr = 0x8327C6A0;
	sub_8222CED0(ctx, base);
	// 8327C6A0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8327C6A4: 386A0458  addi r3, r10, 0x458
	ctx.r[3].s64 = ctx.r[10].s64 + 1112;
	// 8327C6A8: 4BA2D879  bl 0x82ca9f20
	ctx.lr = 0x8327C6AC;
	sub_82CA9F20(ctx, base);
	// 8327C6AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C6B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8327C6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C6C0 size=376
    let mut pc: u32 = 0x8327C6C0;
    'dispatch: loop {
        match pc {
            0x8327C6C0 => {
    //   block [0x8327C6C0..0x8327C838)
	// 8327C6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C6C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8327C6CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C6D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8327C6D4: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C6D8: 3BEBDAF8  addi r31, r11, -0x2508
	ctx.r[31].s64 = ctx.r[11].s64 + -9480;
	// 8327C6DC: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 8327C6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8327C6E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C6E8: 4AFB07E9  bl 0x8222ced0
	ctx.lr = 0x8327C6EC;
	sub_8222CED0(ctx, base);
	// 8327C6EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C6F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C6F4: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 8327C6F8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8327C6FC: 4AFB07D5  bl 0x8222ced0
	ctx.lr = 0x8327C700;
	sub_8222CED0(ctx, base);
	// 8327C700: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C708: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8327C70C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8327C710: 4AFB07C1  bl 0x8222ced0
	ctx.lr = 0x8327C714;
	sub_8222CED0(ctx, base);
	// 8327C714: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C718: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C71C: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 8327C720: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8327C724: 4AFB07AD  bl 0x8222ced0
	ctx.lr = 0x8327C728;
	sub_8222CED0(ctx, base);
	// 8327C728: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C72C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C730: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 8327C734: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8327C738: 4AFB0799  bl 0x8222ced0
	ctx.lr = 0x8327C73C;
	sub_8222CED0(ctx, base);
	// 8327C73C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C740: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C744: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 8327C748: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8327C74C: 4AFB0785  bl 0x8222ced0
	ctx.lr = 0x8327C750;
	sub_8222CED0(ctx, base);
	// 8327C750: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C758: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 8327C75C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8327C760: 4AFB0771  bl 0x8222ced0
	ctx.lr = 0x8327C764;
	sub_8222CED0(ctx, base);
	// 8327C764: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C768: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C76C: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 8327C770: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8327C774: 4AFB075D  bl 0x8222ced0
	ctx.lr = 0x8327C778;
	sub_8222CED0(ctx, base);
	// 8327C778: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8327C77C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C780: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 8327C784: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8327C788: 4AFB0749  bl 0x8222ced0
	ctx.lr = 0x8327C78C;
	sub_8222CED0(ctx, base);
	// 8327C78C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8327C790: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C794: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 8327C798: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8327C79C: 4AFB0735  bl 0x8222ced0
	ctx.lr = 0x8327C7A0;
	sub_8222CED0(ctx, base);
	// 8327C7A0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8327C7A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7A8: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 8327C7AC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8327C7B0: 4AFB0721  bl 0x8222ced0
	ctx.lr = 0x8327C7B4;
	sub_8222CED0(ctx, base);
	// 8327C7B4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8327C7B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7BC: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 8327C7C0: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8327C7C4: 4AFB070D  bl 0x8222ced0
	ctx.lr = 0x8327C7C8;
	sub_8222CED0(ctx, base);
	// 8327C7C8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8327C7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7D0: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 8327C7D4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8327C7D8: 4AFB06F9  bl 0x8222ced0
	ctx.lr = 0x8327C7DC;
	sub_8222CED0(ctx, base);
	// 8327C7DC: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8327C7E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7E4: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 8327C7E8: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8327C7EC: 4AFB06E5  bl 0x8222ced0
	ctx.lr = 0x8327C7F0;
	sub_8222CED0(ctx, base);
	// 8327C7F0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8327C7F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C7F8: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 8327C7FC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8327C800: 4AFB06D1  bl 0x8222ced0
	ctx.lr = 0x8327C804;
	sub_8222CED0(ctx, base);
	// 8327C804: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8327C808: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C80C: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 8327C810: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8327C814: 4AFB06BD  bl 0x8222ced0
	ctx.lr = 0x8327C818;
	sub_8222CED0(ctx, base);
	// 8327C818: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8327C81C: 386A04C0  addi r3, r10, 0x4c0
	ctx.r[3].s64 = ctx.r[10].s64 + 1216;
	// 8327C820: 4BA2D701  bl 0x82ca9f20
	ctx.lr = 0x8327C824;
	sub_82CA9F20(ctx, base);
	// 8327C824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8327C834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C838 size=64
    let mut pc: u32 = 0x8327C838;
    'dispatch: loop {
        match pc {
            0x8327C838 => {
    //   block [0x8327C838..0x8327C878)
	// 8327C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C844: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8327C848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C84C: 388B0F58  addi r4, r11, 0xf58
	ctx.r[4].s64 = ctx.r[11].s64 + 3928;
	// 8327C850: 386ADAB4  addi r3, r10, -0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + -9548;
	// 8327C854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C858: 4AFB0679  bl 0x8222ced0
	ctx.lr = 0x8327C85C;
	sub_8222CED0(ctx, base);
	// 8327C85C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C860: 38690528  addi r3, r9, 0x528
	ctx.r[3].s64 = ctx.r[9].s64 + 1320;
	// 8327C864: 4BA2D6BD  bl 0x82ca9f20
	ctx.lr = 0x8327C868;
	sub_82CA9F20(ctx, base);
	// 8327C868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C878 size=64
    let mut pc: u32 = 0x8327C878;
    'dispatch: loop {
        match pc {
            0x8327C878 => {
    //   block [0x8327C878..0x8327C8B8)
	// 8327C878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C884: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327C888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C88C: 388BE58C  addi r4, r11, -0x1a74
	ctx.r[4].s64 = ctx.r[11].s64 + -6772;
	// 8327C890: 386ADB38  addi r3, r10, -0x24c8
	ctx.r[3].s64 = ctx.r[10].s64 + -9416;
	// 8327C894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C898: 4AFB0639  bl 0x8222ced0
	ctx.lr = 0x8327C89C;
	sub_8222CED0(ctx, base);
	// 8327C89C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C8A0: 38690538  addi r3, r9, 0x538
	ctx.r[3].s64 = ctx.r[9].s64 + 1336;
	// 8327C8A4: 4BA2D67D  bl 0x82ca9f20
	ctx.lr = 0x8327C8A8;
	sub_82CA9F20(ctx, base);
	// 8327C8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C8B8 size=64
    let mut pc: u32 = 0x8327C8B8;
    'dispatch: loop {
        match pc {
            0x8327C8B8 => {
    //   block [0x8327C8B8..0x8327C8F8)
	// 8327C8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C8C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C8C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C8CC: 388BF970  addi r4, r11, -0x690
	ctx.r[4].s64 = ctx.r[11].s64 + -1680;
	// 8327C8D0: 386ADB3C  addi r3, r10, -0x24c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9412;
	// 8327C8D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C8D8: 4AFB05F9  bl 0x8222ced0
	ctx.lr = 0x8327C8DC;
	sub_8222CED0(ctx, base);
	// 8327C8DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C8E0: 38690548  addi r3, r9, 0x548
	ctx.r[3].s64 = ctx.r[9].s64 + 1352;
	// 8327C8E4: 4BA2D63D  bl 0x82ca9f20
	ctx.lr = 0x8327C8E8;
	sub_82CA9F20(ctx, base);
	// 8327C8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C8F8 size=64
    let mut pc: u32 = 0x8327C8F8;
    'dispatch: loop {
        match pc {
            0x8327C8F8 => {
    //   block [0x8327C8F8..0x8327C938)
	// 8327C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C904: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C90C: 388BE32C  addi r4, r11, -0x1cd4
	ctx.r[4].s64 = ctx.r[11].s64 + -7380;
	// 8327C910: 386ADB40  addi r3, r10, -0x24c0
	ctx.r[3].s64 = ctx.r[10].s64 + -9408;
	// 8327C914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C918: 4AFB05B9  bl 0x8222ced0
	ctx.lr = 0x8327C91C;
	sub_8222CED0(ctx, base);
	// 8327C91C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C920: 38690558  addi r3, r9, 0x558
	ctx.r[3].s64 = ctx.r[9].s64 + 1368;
	// 8327C924: 4BA2D5FD  bl 0x82ca9f20
	ctx.lr = 0x8327C928;
	sub_82CA9F20(ctx, base);
	// 8327C928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C938 size=64
    let mut pc: u32 = 0x8327C938;
    'dispatch: loop {
        match pc {
            0x8327C938 => {
    //   block [0x8327C938..0x8327C978)
	// 8327C938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C944: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C94C: 388BE2FC  addi r4, r11, -0x1d04
	ctx.r[4].s64 = ctx.r[11].s64 + -7428;
	// 8327C950: 386ADB44  addi r3, r10, -0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9404;
	// 8327C954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C958: 4AFB0579  bl 0x8222ced0
	ctx.lr = 0x8327C95C;
	sub_8222CED0(ctx, base);
	// 8327C95C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C960: 38690568  addi r3, r9, 0x568
	ctx.r[3].s64 = ctx.r[9].s64 + 1384;
	// 8327C964: 4BA2D5BD  bl 0x82ca9f20
	ctx.lr = 0x8327C968;
	sub_82CA9F20(ctx, base);
	// 8327C968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C978 size=64
    let mut pc: u32 = 0x8327C978;
    'dispatch: loop {
        match pc {
            0x8327C978 => {
    //   block [0x8327C978..0x8327C9B8)
	// 8327C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C984: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C98C: 388BE2A0  addi r4, r11, -0x1d60
	ctx.r[4].s64 = ctx.r[11].s64 + -7520;
	// 8327C990: 386ADB48  addi r3, r10, -0x24b8
	ctx.r[3].s64 = ctx.r[10].s64 + -9400;
	// 8327C994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C998: 4AFB0539  bl 0x8222ced0
	ctx.lr = 0x8327C99C;
	sub_8222CED0(ctx, base);
	// 8327C99C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C9A0: 38690578  addi r3, r9, 0x578
	ctx.r[3].s64 = ctx.r[9].s64 + 1400;
	// 8327C9A4: 4BA2D57D  bl 0x82ca9f20
	ctx.lr = 0x8327C9A8;
	sub_82CA9F20(ctx, base);
	// 8327C9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C9B8 size=64
    let mut pc: u32 = 0x8327C9B8;
    'dispatch: loop {
        match pc {
            0x8327C9B8 => {
    //   block [0x8327C9B8..0x8327C9F8)
	// 8327C9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327C9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327C9C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327C9C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327C9CC: 388BE3A8  addi r4, r11, -0x1c58
	ctx.r[4].s64 = ctx.r[11].s64 + -7256;
	// 8327C9D0: 386ADB4C  addi r3, r10, -0x24b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9396;
	// 8327C9D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327C9D8: 4AFB04F9  bl 0x8222ced0
	ctx.lr = 0x8327C9DC;
	sub_8222CED0(ctx, base);
	// 8327C9DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327C9E0: 38690588  addi r3, r9, 0x588
	ctx.r[3].s64 = ctx.r[9].s64 + 1416;
	// 8327C9E4: 4BA2D53D  bl 0x82ca9f20
	ctx.lr = 0x8327C9E8;
	sub_82CA9F20(ctx, base);
	// 8327C9E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327C9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327C9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327C9F8 size=64
    let mut pc: u32 = 0x8327C9F8;
    'dispatch: loop {
        match pc {
            0x8327C9F8 => {
    //   block [0x8327C9F8..0x8327CA38)
	// 8327C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CA04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327CA08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CA0C: 388BE36C  addi r4, r11, -0x1c94
	ctx.r[4].s64 = ctx.r[11].s64 + -7316;
	// 8327CA10: 386ADB50  addi r3, r10, -0x24b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9392;
	// 8327CA14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CA18: 4AFB04B9  bl 0x8222ced0
	ctx.lr = 0x8327CA1C;
	sub_8222CED0(ctx, base);
	// 8327CA1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CA20: 38690598  addi r3, r9, 0x598
	ctx.r[3].s64 = ctx.r[9].s64 + 1432;
	// 8327CA24: 4BA2D4FD  bl 0x82ca9f20
	ctx.lr = 0x8327CA28;
	sub_82CA9F20(ctx, base);
	// 8327CA28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CA38 size=64
    let mut pc: u32 = 0x8327CA38;
    'dispatch: loop {
        match pc {
            0x8327CA38 => {
    //   block [0x8327CA38..0x8327CA78)
	// 8327CA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CA44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CA4C: 388BE598  addi r4, r11, -0x1a68
	ctx.r[4].s64 = ctx.r[11].s64 + -6760;
	// 8327CA50: 386ADB54  addi r3, r10, -0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9388;
	// 8327CA54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CA58: 4AFB0479  bl 0x8222ced0
	ctx.lr = 0x8327CA5C;
	sub_8222CED0(ctx, base);
	// 8327CA5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CA60: 386905A8  addi r3, r9, 0x5a8
	ctx.r[3].s64 = ctx.r[9].s64 + 1448;
	// 8327CA64: 4BA2D4BD  bl 0x82ca9f20
	ctx.lr = 0x8327CA68;
	sub_82CA9F20(ctx, base);
	// 8327CA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CA78 size=64
    let mut pc: u32 = 0x8327CA78;
    'dispatch: loop {
        match pc {
            0x8327CA78 => {
    //   block [0x8327CA78..0x8327CAB8)
	// 8327CA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CA84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8327CA88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CA8C: 388BF970  addi r4, r11, -0x690
	ctx.r[4].s64 = ctx.r[11].s64 + -1680;
	// 8327CA90: 386ADB58  addi r3, r10, -0x24a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9384;
	// 8327CA94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CA98: 4AFB0439  bl 0x8222ced0
	ctx.lr = 0x8327CA9C;
	sub_8222CED0(ctx, base);
	// 8327CA9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CAA0: 386905B8  addi r3, r9, 0x5b8
	ctx.r[3].s64 = ctx.r[9].s64 + 1464;
	// 8327CAA4: 4BA2D47D  bl 0x82ca9f20
	ctx.lr = 0x8327CAA8;
	sub_82CA9F20(ctx, base);
	// 8327CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CAB8 size=64
    let mut pc: u32 = 0x8327CAB8;
    'dispatch: loop {
        match pc {
            0x8327CAB8 => {
    //   block [0x8327CAB8..0x8327CAF8)
	// 8327CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CAC4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CAC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CACC: 388BE5A8  addi r4, r11, -0x1a58
	ctx.r[4].s64 = ctx.r[11].s64 + -6744;
	// 8327CAD0: 386ADB5C  addi r3, r10, -0x24a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9380;
	// 8327CAD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CAD8: 4AFB03F9  bl 0x8222ced0
	ctx.lr = 0x8327CADC;
	sub_8222CED0(ctx, base);
	// 8327CADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CAE0: 386905C8  addi r3, r9, 0x5c8
	ctx.r[3].s64 = ctx.r[9].s64 + 1480;
	// 8327CAE4: 4BA2D43D  bl 0x82ca9f20
	ctx.lr = 0x8327CAE8;
	sub_82CA9F20(ctx, base);
	// 8327CAE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CAF8 size=64
    let mut pc: u32 = 0x8327CAF8;
    'dispatch: loop {
        match pc {
            0x8327CAF8 => {
    //   block [0x8327CAF8..0x8327CB38)
	// 8327CAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CB00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CB04: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CB08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CB0C: 388BE5B4  addi r4, r11, -0x1a4c
	ctx.r[4].s64 = ctx.r[11].s64 + -6732;
	// 8327CB10: 386ADB60  addi r3, r10, -0x24a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9376;
	// 8327CB14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CB18: 4AFB03B9  bl 0x8222ced0
	ctx.lr = 0x8327CB1C;
	sub_8222CED0(ctx, base);
	// 8327CB1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CB20: 386905D8  addi r3, r9, 0x5d8
	ctx.r[3].s64 = ctx.r[9].s64 + 1496;
	// 8327CB24: 4BA2D3FD  bl 0x82ca9f20
	ctx.lr = 0x8327CB28;
	sub_82CA9F20(ctx, base);
	// 8327CB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CB38 size=64
    let mut pc: u32 = 0x8327CB38;
    'dispatch: loop {
        match pc {
            0x8327CB38 => {
    //   block [0x8327CB38..0x8327CB78)
	// 8327CB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CB44: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327CB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CB4C: 388BE5D4  addi r4, r11, -0x1a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -6700;
	// 8327CB50: 386ADB64  addi r3, r10, -0x249c
	ctx.r[3].s64 = ctx.r[10].s64 + -9372;
	// 8327CB54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327CB58: 4AFB0379  bl 0x8222ced0
	ctx.lr = 0x8327CB5C;
	sub_8222CED0(ctx, base);
	// 8327CB5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327CB60: 386905E8  addi r3, r9, 0x5e8
	ctx.r[3].s64 = ctx.r[9].s64 + 1512;
	// 8327CB64: 4BA2D3BD  bl 0x82ca9f20
	ctx.lr = 0x8327CB68;
	sub_82CA9F20(ctx, base);
	// 8327CB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CB78 size=56
    let mut pc: u32 = 0x8327CB78;
    'dispatch: loop {
        match pc {
            0x8327CB78 => {
    //   block [0x8327CB78..0x8327CBB0)
	// 8327CB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CB84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CB88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CB8C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327CB90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CB94: 4AF771C5  bl 0x821f3d58
	ctx.lr = 0x8327CB98;
	sub_821F3D58(ctx, base);
	// 8327CB98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CB9C: 906ADB68  stw r3, -0x2498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9368 as u32), ctx.r[3].u32 ) };
	// 8327CBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CBB0 size=56
    let mut pc: u32 = 0x8327CBB0;
    'dispatch: loop {
        match pc {
            0x8327CBB0 => {
    //   block [0x8327CBB0..0x8327CBE8)
	// 8327CBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CBBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CBC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CBC4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327CBC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CBCC: 4AF7718D  bl 0x821f3d58
	ctx.lr = 0x8327CBD0;
	sub_821F3D58(ctx, base);
	// 8327CBD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CBD4: 906ADB6C  stw r3, -0x2494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9364 as u32), ctx.r[3].u32 ) };
	// 8327CBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CBE8 size=56
    let mut pc: u32 = 0x8327CBE8;
    'dispatch: loop {
        match pc {
            0x8327CBE8 => {
    //   block [0x8327CBE8..0x8327CC20)
	// 8327CBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CBF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CBF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CBFC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327CC00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CC04: 4AF77155  bl 0x821f3d58
	ctx.lr = 0x8327CC08;
	sub_821F3D58(ctx, base);
	// 8327CC08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CC0C: 906ADB70  stw r3, -0x2490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9360 as u32), ctx.r[3].u32 ) };
	// 8327CC10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CC20 size=56
    let mut pc: u32 = 0x8327CC20;
    'dispatch: loop {
        match pc {
            0x8327CC20 => {
    //   block [0x8327CC20..0x8327CC58)
	// 8327CC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CC28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CC2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CC30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CC34: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327CC38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CC3C: 4AF7711D  bl 0x821f3d58
	ctx.lr = 0x8327CC40;
	sub_821F3D58(ctx, base);
	// 8327CC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CC44: 906ADB74  stw r3, -0x248c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9356 as u32), ctx.r[3].u32 ) };
	// 8327CC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CC58 size=56
    let mut pc: u32 = 0x8327CC58;
    'dispatch: loop {
        match pc {
            0x8327CC58 => {
    //   block [0x8327CC58..0x8327CC90)
	// 8327CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CC64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CC68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CC6C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327CC70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CC74: 4AF770E5  bl 0x821f3d58
	ctx.lr = 0x8327CC78;
	sub_821F3D58(ctx, base);
	// 8327CC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CC7C: 906ADB78  stw r3, -0x2488(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9352 as u32), ctx.r[3].u32 ) };
	// 8327CC80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CC90 size=56
    let mut pc: u32 = 0x8327CC90;
    'dispatch: loop {
        match pc {
            0x8327CC90 => {
    //   block [0x8327CC90..0x8327CCC8)
	// 8327CC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CC98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CC9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CCA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CCA4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327CCA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CCAC: 4AF770AD  bl 0x821f3d58
	ctx.lr = 0x8327CCB0;
	sub_821F3D58(ctx, base);
	// 8327CCB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CCB4: 906ADB7C  stw r3, -0x2484(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9348 as u32), ctx.r[3].u32 ) };
	// 8327CCB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CCC8 size=56
    let mut pc: u32 = 0x8327CCC8;
    'dispatch: loop {
        match pc {
            0x8327CCC8 => {
    //   block [0x8327CCC8..0x8327CD00)
	// 8327CCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CCD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CCD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CCDC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327CCE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CCE4: 4AF77075  bl 0x821f3d58
	ctx.lr = 0x8327CCE8;
	sub_821F3D58(ctx, base);
	// 8327CCE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CCEC: 906ADB80  stw r3, -0x2480(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9344 as u32), ctx.r[3].u32 ) };
	// 8327CCF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CD00 size=56
    let mut pc: u32 = 0x8327CD00;
    'dispatch: loop {
        match pc {
            0x8327CD00 => {
    //   block [0x8327CD00..0x8327CD38)
	// 8327CD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CD0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CD10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CD14: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327CD18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CD1C: 4AF7703D  bl 0x821f3d58
	ctx.lr = 0x8327CD20;
	sub_821F3D58(ctx, base);
	// 8327CD20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CD24: 906ADB84  stw r3, -0x247c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9340 as u32), ctx.r[3].u32 ) };
	// 8327CD28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CD2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CD30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CD38 size=56
    let mut pc: u32 = 0x8327CD38;
    'dispatch: loop {
        match pc {
            0x8327CD38 => {
    //   block [0x8327CD38..0x8327CD70)
	// 8327CD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CD40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CD44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CD48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CD4C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327CD50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CD54: 4AF77005  bl 0x821f3d58
	ctx.lr = 0x8327CD58;
	sub_821F3D58(ctx, base);
	// 8327CD58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CD5C: 906ADB88  stw r3, -0x2478(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9336 as u32), ctx.r[3].u32 ) };
	// 8327CD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CD70 size=56
    let mut pc: u32 = 0x8327CD70;
    'dispatch: loop {
        match pc {
            0x8327CD70 => {
    //   block [0x8327CD70..0x8327CDA8)
	// 8327CD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CD78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CD7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CD80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CD84: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327CD88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CD8C: 4AF76FCD  bl 0x821f3d58
	ctx.lr = 0x8327CD90;
	sub_821F3D58(ctx, base);
	// 8327CD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CD94: 906ADB8C  stw r3, -0x2474(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9332 as u32), ctx.r[3].u32 ) };
	// 8327CD98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CDA8 size=56
    let mut pc: u32 = 0x8327CDA8;
    'dispatch: loop {
        match pc {
            0x8327CDA8 => {
    //   block [0x8327CDA8..0x8327CDE0)
	// 8327CDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CDB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CDB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CDB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CDBC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327CDC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CDC4: 4AF76F95  bl 0x821f3d58
	ctx.lr = 0x8327CDC8;
	sub_821F3D58(ctx, base);
	// 8327CDC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CDCC: 906ADB90  stw r3, -0x2470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9328 as u32), ctx.r[3].u32 ) };
	// 8327CDD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CDE0 size=56
    let mut pc: u32 = 0x8327CDE0;
    'dispatch: loop {
        match pc {
            0x8327CDE0 => {
    //   block [0x8327CDE0..0x8327CE18)
	// 8327CDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CDE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CDEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CDF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CDF4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327CDF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CDFC: 4AF76F5D  bl 0x821f3d58
	ctx.lr = 0x8327CE00;
	sub_821F3D58(ctx, base);
	// 8327CE00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CE04: 906ADB94  stw r3, -0x246c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9324 as u32), ctx.r[3].u32 ) };
	// 8327CE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CE18 size=56
    let mut pc: u32 = 0x8327CE18;
    'dispatch: loop {
        match pc {
            0x8327CE18 => {
    //   block [0x8327CE18..0x8327CE50)
	// 8327CE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CE24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CE28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CE2C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327CE30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CE34: 4AF76F25  bl 0x821f3d58
	ctx.lr = 0x8327CE38;
	sub_821F3D58(ctx, base);
	// 8327CE38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CE3C: 906ADB98  stw r3, -0x2468(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9320 as u32), ctx.r[3].u32 ) };
	// 8327CE40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CE50 size=56
    let mut pc: u32 = 0x8327CE50;
    'dispatch: loop {
        match pc {
            0x8327CE50 => {
    //   block [0x8327CE50..0x8327CE88)
	// 8327CE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CE58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CE5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CE60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CE64: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327CE68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CE6C: 4AF76EED  bl 0x821f3d58
	ctx.lr = 0x8327CE70;
	sub_821F3D58(ctx, base);
	// 8327CE70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CE74: 906ADB9C  stw r3, -0x2464(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9316 as u32), ctx.r[3].u32 ) };
	// 8327CE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CE88 size=56
    let mut pc: u32 = 0x8327CE88;
    'dispatch: loop {
        match pc {
            0x8327CE88 => {
    //   block [0x8327CE88..0x8327CEC0)
	// 8327CE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CE94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CE98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CE9C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327CEA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CEA4: 4AF76EB5  bl 0x821f3d58
	ctx.lr = 0x8327CEA8;
	sub_821F3D58(ctx, base);
	// 8327CEA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CEAC: 906ADBA0  stw r3, -0x2460(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9312 as u32), ctx.r[3].u32 ) };
	// 8327CEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CEC0 size=56
    let mut pc: u32 = 0x8327CEC0;
    'dispatch: loop {
        match pc {
            0x8327CEC0 => {
    //   block [0x8327CEC0..0x8327CEF8)
	// 8327CEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CEC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CECC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CED0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CED4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327CED8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CEDC: 4AF76E7D  bl 0x821f3d58
	ctx.lr = 0x8327CEE0;
	sub_821F3D58(ctx, base);
	// 8327CEE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CEE4: 906ADBA4  stw r3, -0x245c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9308 as u32), ctx.r[3].u32 ) };
	// 8327CEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CEF8 size=56
    let mut pc: u32 = 0x8327CEF8;
    'dispatch: loop {
        match pc {
            0x8327CEF8 => {
    //   block [0x8327CEF8..0x8327CF30)
	// 8327CEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CF04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CF08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CF0C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327CF10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CF14: 4AF76E45  bl 0x821f3d58
	ctx.lr = 0x8327CF18;
	sub_821F3D58(ctx, base);
	// 8327CF18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CF1C: 906ADBA8  stw r3, -0x2458(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9304 as u32), ctx.r[3].u32 ) };
	// 8327CF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CF30 size=56
    let mut pc: u32 = 0x8327CF30;
    'dispatch: loop {
        match pc {
            0x8327CF30 => {
    //   block [0x8327CF30..0x8327CF68)
	// 8327CF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CF3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CF40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CF44: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327CF48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CF4C: 4AF76E0D  bl 0x821f3d58
	ctx.lr = 0x8327CF50;
	sub_821F3D58(ctx, base);
	// 8327CF50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CF54: 906ADBAC  stw r3, -0x2454(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9300 as u32), ctx.r[3].u32 ) };
	// 8327CF58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CF68 size=56
    let mut pc: u32 = 0x8327CF68;
    'dispatch: loop {
        match pc {
            0x8327CF68 => {
    //   block [0x8327CF68..0x8327CFA0)
	// 8327CF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CF70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CF74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CF78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CF7C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327CF80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CF84: 4AF76DD5  bl 0x821f3d58
	ctx.lr = 0x8327CF88;
	sub_821F3D58(ctx, base);
	// 8327CF88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CF8C: 906ADBB0  stw r3, -0x2450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9296 as u32), ctx.r[3].u32 ) };
	// 8327CF90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CFA0 size=56
    let mut pc: u32 = 0x8327CFA0;
    'dispatch: loop {
        match pc {
            0x8327CFA0 => {
    //   block [0x8327CFA0..0x8327CFD8)
	// 8327CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CFA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CFAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CFB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CFB4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327CFB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CFBC: 4AF76D9D  bl 0x821f3d58
	ctx.lr = 0x8327CFC0;
	sub_821F3D58(ctx, base);
	// 8327CFC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CFC4: 906ADBB4  stw r3, -0x244c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9292 as u32), ctx.r[3].u32 ) };
	// 8327CFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327CFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327CFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327CFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327CFD8 size=56
    let mut pc: u32 = 0x8327CFD8;
    'dispatch: loop {
        match pc {
            0x8327CFD8 => {
    //   block [0x8327CFD8..0x8327D010)
	// 8327CFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327CFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327CFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327CFE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327CFE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327CFEC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327CFF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327CFF4: 4AF76D65  bl 0x821f3d58
	ctx.lr = 0x8327CFF8;
	sub_821F3D58(ctx, base);
	// 8327CFF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327CFFC: 906ADBB8  stw r3, -0x2448(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9288 as u32), ctx.r[3].u32 ) };
	// 8327D000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D010 size=64
    let mut pc: u32 = 0x8327D010;
    'dispatch: loop {
        match pc {
            0x8327D010 => {
    //   block [0x8327D010..0x8327D050)
	// 8327D010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D01C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D024: 388BE5F8  addi r4, r11, -0x1a08
	ctx.r[4].s64 = ctx.r[11].s64 + -6664;
	// 8327D028: 386ADBBC  addi r3, r10, -0x2444
	ctx.r[3].s64 = ctx.r[10].s64 + -9284;
	// 8327D02C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D030: 4AFAFEA1  bl 0x8222ced0
	ctx.lr = 0x8327D034;
	sub_8222CED0(ctx, base);
	// 8327D034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D038: 386905F8  addi r3, r9, 0x5f8
	ctx.r[3].s64 = ctx.r[9].s64 + 1528;
	// 8327D03C: 4BA2CEE5  bl 0x82ca9f20
	ctx.lr = 0x8327D040;
	sub_82CA9F20(ctx, base);
	// 8327D040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D050 size=64
    let mut pc: u32 = 0x8327D050;
    'dispatch: loop {
        match pc {
            0x8327D050 => {
    //   block [0x8327D050..0x8327D090)
	// 8327D050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D05C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D064: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 8327D068: 386ADBC0  addi r3, r10, -0x2440
	ctx.r[3].s64 = ctx.r[10].s64 + -9280;
	// 8327D06C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D070: 4AFAFE61  bl 0x8222ced0
	ctx.lr = 0x8327D074;
	sub_8222CED0(ctx, base);
	// 8327D074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D078: 38690608  addi r3, r9, 0x608
	ctx.r[3].s64 = ctx.r[9].s64 + 1544;
	// 8327D07C: 4BA2CEA5  bl 0x82ca9f20
	ctx.lr = 0x8327D080;
	sub_82CA9F20(ctx, base);
	// 8327D080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D090 size=64
    let mut pc: u32 = 0x8327D090;
    'dispatch: loop {
        match pc {
            0x8327D090 => {
    //   block [0x8327D090..0x8327D0D0)
	// 8327D090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D09C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D0A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D0A4: 388BE648  addi r4, r11, -0x19b8
	ctx.r[4].s64 = ctx.r[11].s64 + -6584;
	// 8327D0A8: 386ADBC4  addi r3, r10, -0x243c
	ctx.r[3].s64 = ctx.r[10].s64 + -9276;
	// 8327D0AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D0B0: 4AFAFE21  bl 0x8222ced0
	ctx.lr = 0x8327D0B4;
	sub_8222CED0(ctx, base);
	// 8327D0B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D0B8: 38690618  addi r3, r9, 0x618
	ctx.r[3].s64 = ctx.r[9].s64 + 1560;
	// 8327D0BC: 4BA2CE65  bl 0x82ca9f20
	ctx.lr = 0x8327D0C0;
	sub_82CA9F20(ctx, base);
	// 8327D0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D0D0 size=64
    let mut pc: u32 = 0x8327D0D0;
    'dispatch: loop {
        match pc {
            0x8327D0D0 => {
    //   block [0x8327D0D0..0x8327D110)
	// 8327D0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D0DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D0E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D0E4: 388BE690  addi r4, r11, -0x1970
	ctx.r[4].s64 = ctx.r[11].s64 + -6512;
	// 8327D0E8: 386ADBC8  addi r3, r10, -0x2438
	ctx.r[3].s64 = ctx.r[10].s64 + -9272;
	// 8327D0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D0F0: 4AFAFDE1  bl 0x8222ced0
	ctx.lr = 0x8327D0F4;
	sub_8222CED0(ctx, base);
	// 8327D0F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D0F8: 38690628  addi r3, r9, 0x628
	ctx.r[3].s64 = ctx.r[9].s64 + 1576;
	// 8327D0FC: 4BA2CE25  bl 0x82ca9f20
	ctx.lr = 0x8327D100;
	sub_82CA9F20(ctx, base);
	// 8327D100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D110 size=64
    let mut pc: u32 = 0x8327D110;
    'dispatch: loop {
        match pc {
            0x8327D110 => {
    //   block [0x8327D110..0x8327D150)
	// 8327D110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D11C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D124: 388BE6D8  addi r4, r11, -0x1928
	ctx.r[4].s64 = ctx.r[11].s64 + -6440;
	// 8327D128: 386ADBCC  addi r3, r10, -0x2434
	ctx.r[3].s64 = ctx.r[10].s64 + -9268;
	// 8327D12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D130: 4AFAFDA1  bl 0x8222ced0
	ctx.lr = 0x8327D134;
	sub_8222CED0(ctx, base);
	// 8327D134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D138: 38690638  addi r3, r9, 0x638
	ctx.r[3].s64 = ctx.r[9].s64 + 1592;
	// 8327D13C: 4BA2CDE5  bl 0x82ca9f20
	ctx.lr = 0x8327D140;
	sub_82CA9F20(ctx, base);
	// 8327D140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D150 size=64
    let mut pc: u32 = 0x8327D150;
    'dispatch: loop {
        match pc {
            0x8327D150 => {
    //   block [0x8327D150..0x8327D190)
	// 8327D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D15C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D164: 388BE728  addi r4, r11, -0x18d8
	ctx.r[4].s64 = ctx.r[11].s64 + -6360;
	// 8327D168: 386ADBD0  addi r3, r10, -0x2430
	ctx.r[3].s64 = ctx.r[10].s64 + -9264;
	// 8327D16C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D170: 4AFAFD61  bl 0x8222ced0
	ctx.lr = 0x8327D174;
	sub_8222CED0(ctx, base);
	// 8327D174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D178: 38690648  addi r3, r9, 0x648
	ctx.r[3].s64 = ctx.r[9].s64 + 1608;
	// 8327D17C: 4BA2CDA5  bl 0x82ca9f20
	ctx.lr = 0x8327D180;
	sub_82CA9F20(ctx, base);
	// 8327D180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D190 size=64
    let mut pc: u32 = 0x8327D190;
    'dispatch: loop {
        match pc {
            0x8327D190 => {
    //   block [0x8327D190..0x8327D1D0)
	// 8327D190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D19C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D1A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D1A4: 388BE770  addi r4, r11, -0x1890
	ctx.r[4].s64 = ctx.r[11].s64 + -6288;
	// 8327D1A8: 386ADBD4  addi r3, r10, -0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + -9260;
	// 8327D1AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D1B0: 4AFAFD21  bl 0x8222ced0
	ctx.lr = 0x8327D1B4;
	sub_8222CED0(ctx, base);
	// 8327D1B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D1B8: 38690658  addi r3, r9, 0x658
	ctx.r[3].s64 = ctx.r[9].s64 + 1624;
	// 8327D1BC: 4BA2CD65  bl 0x82ca9f20
	ctx.lr = 0x8327D1C0;
	sub_82CA9F20(ctx, base);
	// 8327D1C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D1D0 size=64
    let mut pc: u32 = 0x8327D1D0;
    'dispatch: loop {
        match pc {
            0x8327D1D0 => {
    //   block [0x8327D1D0..0x8327D210)
	// 8327D1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D1D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D1DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D1E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D1E4: 388BE7C0  addi r4, r11, -0x1840
	ctx.r[4].s64 = ctx.r[11].s64 + -6208;
	// 8327D1E8: 386ADBD8  addi r3, r10, -0x2428
	ctx.r[3].s64 = ctx.r[10].s64 + -9256;
	// 8327D1EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D1F0: 4AFAFCE1  bl 0x8222ced0
	ctx.lr = 0x8327D1F4;
	sub_8222CED0(ctx, base);
	// 8327D1F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D1F8: 38690668  addi r3, r9, 0x668
	ctx.r[3].s64 = ctx.r[9].s64 + 1640;
	// 8327D1FC: 4BA2CD25  bl 0x82ca9f20
	ctx.lr = 0x8327D200;
	sub_82CA9F20(ctx, base);
	// 8327D200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D210 size=64
    let mut pc: u32 = 0x8327D210;
    'dispatch: loop {
        match pc {
            0x8327D210 => {
    //   block [0x8327D210..0x8327D250)
	// 8327D210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D21C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D224: 388BE810  addi r4, r11, -0x17f0
	ctx.r[4].s64 = ctx.r[11].s64 + -6128;
	// 8327D228: 386ADBDC  addi r3, r10, -0x2424
	ctx.r[3].s64 = ctx.r[10].s64 + -9252;
	// 8327D22C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D230: 4AFAFCA1  bl 0x8222ced0
	ctx.lr = 0x8327D234;
	sub_8222CED0(ctx, base);
	// 8327D234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D238: 38690678  addi r3, r9, 0x678
	ctx.r[3].s64 = ctx.r[9].s64 + 1656;
	// 8327D23C: 4BA2CCE5  bl 0x82ca9f20
	ctx.lr = 0x8327D240;
	sub_82CA9F20(ctx, base);
	// 8327D240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D250 size=64
    let mut pc: u32 = 0x8327D250;
    'dispatch: loop {
        match pc {
            0x8327D250 => {
    //   block [0x8327D250..0x8327D290)
	// 8327D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D25C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D264: 388BE850  addi r4, r11, -0x17b0
	ctx.r[4].s64 = ctx.r[11].s64 + -6064;
	// 8327D268: 386ADBE0  addi r3, r10, -0x2420
	ctx.r[3].s64 = ctx.r[10].s64 + -9248;
	// 8327D26C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D270: 4AFAFC61  bl 0x8222ced0
	ctx.lr = 0x8327D274;
	sub_8222CED0(ctx, base);
	// 8327D274: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D278: 38690688  addi r3, r9, 0x688
	ctx.r[3].s64 = ctx.r[9].s64 + 1672;
	// 8327D27C: 4BA2CCA5  bl 0x82ca9f20
	ctx.lr = 0x8327D280;
	sub_82CA9F20(ctx, base);
	// 8327D280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D290 size=64
    let mut pc: u32 = 0x8327D290;
    'dispatch: loop {
        match pc {
            0x8327D290 => {
    //   block [0x8327D290..0x8327D2D0)
	// 8327D290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D29C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D2A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D2A4: 388BE898  addi r4, r11, -0x1768
	ctx.r[4].s64 = ctx.r[11].s64 + -5992;
	// 8327D2A8: 386ADBE4  addi r3, r10, -0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + -9244;
	// 8327D2AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D2B0: 4AFAFC21  bl 0x8222ced0
	ctx.lr = 0x8327D2B4;
	sub_8222CED0(ctx, base);
	// 8327D2B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D2B8: 38690698  addi r3, r9, 0x698
	ctx.r[3].s64 = ctx.r[9].s64 + 1688;
	// 8327D2BC: 4BA2CC65  bl 0x82ca9f20
	ctx.lr = 0x8327D2C0;
	sub_82CA9F20(ctx, base);
	// 8327D2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D2D0 size=64
    let mut pc: u32 = 0x8327D2D0;
    'dispatch: loop {
        match pc {
            0x8327D2D0 => {
    //   block [0x8327D2D0..0x8327D310)
	// 8327D2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D2DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D2E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D2E4: 388BE8E0  addi r4, r11, -0x1720
	ctx.r[4].s64 = ctx.r[11].s64 + -5920;
	// 8327D2E8: 386ADBE8  addi r3, r10, -0x2418
	ctx.r[3].s64 = ctx.r[10].s64 + -9240;
	// 8327D2EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D2F0: 4AFAFBE1  bl 0x8222ced0
	ctx.lr = 0x8327D2F4;
	sub_8222CED0(ctx, base);
	// 8327D2F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D2F8: 386906A8  addi r3, r9, 0x6a8
	ctx.r[3].s64 = ctx.r[9].s64 + 1704;
	// 8327D2FC: 4BA2CC25  bl 0x82ca9f20
	ctx.lr = 0x8327D300;
	sub_82CA9F20(ctx, base);
	// 8327D300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D310 size=64
    let mut pc: u32 = 0x8327D310;
    'dispatch: loop {
        match pc {
            0x8327D310 => {
    //   block [0x8327D310..0x8327D350)
	// 8327D310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D31C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D324: 388BE928  addi r4, r11, -0x16d8
	ctx.r[4].s64 = ctx.r[11].s64 + -5848;
	// 8327D328: 386ADBEC  addi r3, r10, -0x2414
	ctx.r[3].s64 = ctx.r[10].s64 + -9236;
	// 8327D32C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D330: 4AFAFBA1  bl 0x8222ced0
	ctx.lr = 0x8327D334;
	sub_8222CED0(ctx, base);
	// 8327D334: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D338: 386906B8  addi r3, r9, 0x6b8
	ctx.r[3].s64 = ctx.r[9].s64 + 1720;
	// 8327D33C: 4BA2CBE5  bl 0x82ca9f20
	ctx.lr = 0x8327D340;
	sub_82CA9F20(ctx, base);
	// 8327D340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D350 size=64
    let mut pc: u32 = 0x8327D350;
    'dispatch: loop {
        match pc {
            0x8327D350 => {
    //   block [0x8327D350..0x8327D390)
	// 8327D350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D35C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D364: 388BE970  addi r4, r11, -0x1690
	ctx.r[4].s64 = ctx.r[11].s64 + -5776;
	// 8327D368: 386ADBF0  addi r3, r10, -0x2410
	ctx.r[3].s64 = ctx.r[10].s64 + -9232;
	// 8327D36C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D370: 4AFAFB61  bl 0x8222ced0
	ctx.lr = 0x8327D374;
	sub_8222CED0(ctx, base);
	// 8327D374: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D378: 386906C8  addi r3, r9, 0x6c8
	ctx.r[3].s64 = ctx.r[9].s64 + 1736;
	// 8327D37C: 4BA2CBA5  bl 0x82ca9f20
	ctx.lr = 0x8327D380;
	sub_82CA9F20(ctx, base);
	// 8327D380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D390 size=64
    let mut pc: u32 = 0x8327D390;
    'dispatch: loop {
        match pc {
            0x8327D390 => {
    //   block [0x8327D390..0x8327D3D0)
	// 8327D390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D39C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D3A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D3A4: 388BE9B8  addi r4, r11, -0x1648
	ctx.r[4].s64 = ctx.r[11].s64 + -5704;
	// 8327D3A8: 386ADBF4  addi r3, r10, -0x240c
	ctx.r[3].s64 = ctx.r[10].s64 + -9228;
	// 8327D3AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D3B0: 4AFAFB21  bl 0x8222ced0
	ctx.lr = 0x8327D3B4;
	sub_8222CED0(ctx, base);
	// 8327D3B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D3B8: 386906D8  addi r3, r9, 0x6d8
	ctx.r[3].s64 = ctx.r[9].s64 + 1752;
	// 8327D3BC: 4BA2CB65  bl 0x82ca9f20
	ctx.lr = 0x8327D3C0;
	sub_82CA9F20(ctx, base);
	// 8327D3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D3D0 size=56
    let mut pc: u32 = 0x8327D3D0;
    'dispatch: loop {
        match pc {
            0x8327D3D0 => {
    //   block [0x8327D3D0..0x8327D408)
	// 8327D3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D3DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D3E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D3E4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327D3E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D3EC: 4AF7696D  bl 0x821f3d58
	ctx.lr = 0x8327D3F0;
	sub_821F3D58(ctx, base);
	// 8327D3F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D3F4: 906ADBF8  stw r3, -0x2408(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9224 as u32), ctx.r[3].u32 ) };
	// 8327D3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D408 size=56
    let mut pc: u32 = 0x8327D408;
    'dispatch: loop {
        match pc {
            0x8327D408 => {
    //   block [0x8327D408..0x8327D440)
	// 8327D408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D414: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D41C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327D420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D424: 4AF76935  bl 0x821f3d58
	ctx.lr = 0x8327D428;
	sub_821F3D58(ctx, base);
	// 8327D428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D42C: 906ADBFC  stw r3, -0x2404(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9220 as u32), ctx.r[3].u32 ) };
	// 8327D430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D440 size=56
    let mut pc: u32 = 0x8327D440;
    'dispatch: loop {
        match pc {
            0x8327D440 => {
    //   block [0x8327D440..0x8327D478)
	// 8327D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D44C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D454: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327D458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D45C: 4AF768FD  bl 0x821f3d58
	ctx.lr = 0x8327D460;
	sub_821F3D58(ctx, base);
	// 8327D460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D464: 906ADC00  stw r3, -0x2400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9216 as u32), ctx.r[3].u32 ) };
	// 8327D468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D478 size=56
    let mut pc: u32 = 0x8327D478;
    'dispatch: loop {
        match pc {
            0x8327D478 => {
    //   block [0x8327D478..0x8327D4B0)
	// 8327D478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D484: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D48C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327D490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D494: 4AF768C5  bl 0x821f3d58
	ctx.lr = 0x8327D498;
	sub_821F3D58(ctx, base);
	// 8327D498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D49C: 906ADC04  stw r3, -0x23fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9212 as u32), ctx.r[3].u32 ) };
	// 8327D4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D4B0 size=56
    let mut pc: u32 = 0x8327D4B0;
    'dispatch: loop {
        match pc {
            0x8327D4B0 => {
    //   block [0x8327D4B0..0x8327D4E8)
	// 8327D4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D4BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D4C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D4C4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327D4C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D4CC: 4AF7688D  bl 0x821f3d58
	ctx.lr = 0x8327D4D0;
	sub_821F3D58(ctx, base);
	// 8327D4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D4D4: 906ADC08  stw r3, -0x23f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9208 as u32), ctx.r[3].u32 ) };
	// 8327D4D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D4E8 size=56
    let mut pc: u32 = 0x8327D4E8;
    'dispatch: loop {
        match pc {
            0x8327D4E8 => {
    //   block [0x8327D4E8..0x8327D520)
	// 8327D4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D4F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D4FC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327D500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D504: 4AF76855  bl 0x821f3d58
	ctx.lr = 0x8327D508;
	sub_821F3D58(ctx, base);
	// 8327D508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D50C: 906ADC0C  stw r3, -0x23f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9204 as u32), ctx.r[3].u32 ) };
	// 8327D510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D520 size=56
    let mut pc: u32 = 0x8327D520;
    'dispatch: loop {
        match pc {
            0x8327D520 => {
    //   block [0x8327D520..0x8327D558)
	// 8327D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D52C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D534: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327D538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D53C: 4AF7681D  bl 0x821f3d58
	ctx.lr = 0x8327D540;
	sub_821F3D58(ctx, base);
	// 8327D540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D544: 906ADC10  stw r3, -0x23f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9200 as u32), ctx.r[3].u32 ) };
	// 8327D548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D558 size=56
    let mut pc: u32 = 0x8327D558;
    'dispatch: loop {
        match pc {
            0x8327D558 => {
    //   block [0x8327D558..0x8327D590)
	// 8327D558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D56C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327D570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D574: 4AF767E5  bl 0x821f3d58
	ctx.lr = 0x8327D578;
	sub_821F3D58(ctx, base);
	// 8327D578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D57C: 906ADC14  stw r3, -0x23ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9196 as u32), ctx.r[3].u32 ) };
	// 8327D580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D590 size=56
    let mut pc: u32 = 0x8327D590;
    'dispatch: loop {
        match pc {
            0x8327D590 => {
    //   block [0x8327D590..0x8327D5C8)
	// 8327D590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D59C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D5A4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327D5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D5AC: 4AF767AD  bl 0x821f3d58
	ctx.lr = 0x8327D5B0;
	sub_821F3D58(ctx, base);
	// 8327D5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D5B4: 906ADC18  stw r3, -0x23e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9192 as u32), ctx.r[3].u32 ) };
	// 8327D5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D5C8 size=56
    let mut pc: u32 = 0x8327D5C8;
    'dispatch: loop {
        match pc {
            0x8327D5C8 => {
    //   block [0x8327D5C8..0x8327D600)
	// 8327D5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D5D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D5DC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327D5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D5E4: 4AF76775  bl 0x821f3d58
	ctx.lr = 0x8327D5E8;
	sub_821F3D58(ctx, base);
	// 8327D5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D5EC: 906ADC1C  stw r3, -0x23e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9188 as u32), ctx.r[3].u32 ) };
	// 8327D5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D600 size=56
    let mut pc: u32 = 0x8327D600;
    'dispatch: loop {
        match pc {
            0x8327D600 => {
    //   block [0x8327D600..0x8327D638)
	// 8327D600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D60C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D614: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327D618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D61C: 4AF7673D  bl 0x821f3d58
	ctx.lr = 0x8327D620;
	sub_821F3D58(ctx, base);
	// 8327D620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D624: 906ADC20  stw r3, -0x23e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9184 as u32), ctx.r[3].u32 ) };
	// 8327D628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D638 size=56
    let mut pc: u32 = 0x8327D638;
    'dispatch: loop {
        match pc {
            0x8327D638 => {
    //   block [0x8327D638..0x8327D670)
	// 8327D638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D64C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327D650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D654: 4AF76705  bl 0x821f3d58
	ctx.lr = 0x8327D658;
	sub_821F3D58(ctx, base);
	// 8327D658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D65C: 906ADC24  stw r3, -0x23dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9180 as u32), ctx.r[3].u32 ) };
	// 8327D660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D670 size=56
    let mut pc: u32 = 0x8327D670;
    'dispatch: loop {
        match pc {
            0x8327D670 => {
    //   block [0x8327D670..0x8327D6A8)
	// 8327D670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D684: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327D688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D68C: 4AF766CD  bl 0x821f3d58
	ctx.lr = 0x8327D690;
	sub_821F3D58(ctx, base);
	// 8327D690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D694: 906ADC28  stw r3, -0x23d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9176 as u32), ctx.r[3].u32 ) };
	// 8327D698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D6A8 size=56
    let mut pc: u32 = 0x8327D6A8;
    'dispatch: loop {
        match pc {
            0x8327D6A8 => {
    //   block [0x8327D6A8..0x8327D6E0)
	// 8327D6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D6B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D6BC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327D6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D6C4: 4AF76695  bl 0x821f3d58
	ctx.lr = 0x8327D6C8;
	sub_821F3D58(ctx, base);
	// 8327D6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D6CC: 906ADC2C  stw r3, -0x23d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9172 as u32), ctx.r[3].u32 ) };
	// 8327D6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D6E0 size=56
    let mut pc: u32 = 0x8327D6E0;
    'dispatch: loop {
        match pc {
            0x8327D6E0 => {
    //   block [0x8327D6E0..0x8327D718)
	// 8327D6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D6EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D6F4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327D6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D6FC: 4AF7665D  bl 0x821f3d58
	ctx.lr = 0x8327D700;
	sub_821F3D58(ctx, base);
	// 8327D700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D704: 906ADC30  stw r3, -0x23d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9168 as u32), ctx.r[3].u32 ) };
	// 8327D708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D718 size=56
    let mut pc: u32 = 0x8327D718;
    'dispatch: loop {
        match pc {
            0x8327D718 => {
    //   block [0x8327D718..0x8327D750)
	// 8327D718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D72C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327D730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D734: 4AF76625  bl 0x821f3d58
	ctx.lr = 0x8327D738;
	sub_821F3D58(ctx, base);
	// 8327D738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D73C: 906ADC34  stw r3, -0x23cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9164 as u32), ctx.r[3].u32 ) };
	// 8327D740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D750 size=56
    let mut pc: u32 = 0x8327D750;
    'dispatch: loop {
        match pc {
            0x8327D750 => {
    //   block [0x8327D750..0x8327D788)
	// 8327D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D764: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327D768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D76C: 4AF765ED  bl 0x821f3d58
	ctx.lr = 0x8327D770;
	sub_821F3D58(ctx, base);
	// 8327D770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D774: 906ADC38  stw r3, -0x23c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9160 as u32), ctx.r[3].u32 ) };
	// 8327D778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D788 size=56
    let mut pc: u32 = 0x8327D788;
    'dispatch: loop {
        match pc {
            0x8327D788 => {
    //   block [0x8327D788..0x8327D7C0)
	// 8327D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D79C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327D7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D7A4: 4AF765B5  bl 0x821f3d58
	ctx.lr = 0x8327D7A8;
	sub_821F3D58(ctx, base);
	// 8327D7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D7AC: 906ADC3C  stw r3, -0x23c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9156 as u32), ctx.r[3].u32 ) };
	// 8327D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D7C0 size=56
    let mut pc: u32 = 0x8327D7C0;
    'dispatch: loop {
        match pc {
            0x8327D7C0 => {
    //   block [0x8327D7C0..0x8327D7F8)
	// 8327D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D7D4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327D7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D7DC: 4AF7657D  bl 0x821f3d58
	ctx.lr = 0x8327D7E0;
	sub_821F3D58(ctx, base);
	// 8327D7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D7E4: 906ADC40  stw r3, -0x23c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9152 as u32), ctx.r[3].u32 ) };
	// 8327D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D7F8 size=56
    let mut pc: u32 = 0x8327D7F8;
    'dispatch: loop {
        match pc {
            0x8327D7F8 => {
    //   block [0x8327D7F8..0x8327D830)
	// 8327D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D80C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327D810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D814: 4AF76545  bl 0x821f3d58
	ctx.lr = 0x8327D818;
	sub_821F3D58(ctx, base);
	// 8327D818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D81C: 906ADC44  stw r3, -0x23bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9148 as u32), ctx.r[3].u32 ) };
	// 8327D820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


