pub fn sub_8324D978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D978 size=56
    let mut pc: u32 = 0x8324D978;
    'dispatch: loop {
        match pc {
            0x8324D978 => {
    //   block [0x8324D978..0x8324D9B0)
	// 8324D978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D984: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D98C: 386BE6E8  addi r3, r11, -0x1918
	ctx.r[3].s64 = ctx.r[11].s64 + -6424;
	// 8324D990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D994: 4AFA63C5  bl 0x821f3d58
	ctx.lr = 0x8324D998;
	sub_821F3D58(ctx, base);
	// 8324D998: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D99C: 906A7C20  stw r3, 0x7c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31776 as u32), ctx.r[3].u32 ) };
	// 8324D9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D9B0 size=56
    let mut pc: u32 = 0x8324D9B0;
    'dispatch: loop {
        match pc {
            0x8324D9B0 => {
    //   block [0x8324D9B0..0x8324D9E8)
	// 8324D9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D9BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D9C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D9C4: 386BE6F8  addi r3, r11, -0x1908
	ctx.r[3].s64 = ctx.r[11].s64 + -6408;
	// 8324D9C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D9CC: 4AFA638D  bl 0x821f3d58
	ctx.lr = 0x8324D9D0;
	sub_821F3D58(ctx, base);
	// 8324D9D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D9D4: 906A7C24  stw r3, 0x7c24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31780 as u32), ctx.r[3].u32 ) };
	// 8324D9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D9E8 size=56
    let mut pc: u32 = 0x8324D9E8;
    'dispatch: loop {
        match pc {
            0x8324D9E8 => {
    //   block [0x8324D9E8..0x8324DA20)
	// 8324D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D9F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D9F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D9F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D9FC: 386BE710  addi r3, r11, -0x18f0
	ctx.r[3].s64 = ctx.r[11].s64 + -6384;
	// 8324DA00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DA04: 4AFA6355  bl 0x821f3d58
	ctx.lr = 0x8324DA08;
	sub_821F3D58(ctx, base);
	// 8324DA08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DA0C: 906A7C28  stw r3, 0x7c28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31784 as u32), ctx.r[3].u32 ) };
	// 8324DA10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DA20 size=56
    let mut pc: u32 = 0x8324DA20;
    'dispatch: loop {
        match pc {
            0x8324DA20 => {
    //   block [0x8324DA20..0x8324DA58)
	// 8324DA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DA28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DA2C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DA30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DA34: 386BE728  addi r3, r11, -0x18d8
	ctx.r[3].s64 = ctx.r[11].s64 + -6360;
	// 8324DA38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DA3C: 4AFA631D  bl 0x821f3d58
	ctx.lr = 0x8324DA40;
	sub_821F3D58(ctx, base);
	// 8324DA40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DA44: 906A7C2C  stw r3, 0x7c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31788 as u32), ctx.r[3].u32 ) };
	// 8324DA48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DA58 size=56
    let mut pc: u32 = 0x8324DA58;
    'dispatch: loop {
        match pc {
            0x8324DA58 => {
    //   block [0x8324DA58..0x8324DA90)
	// 8324DA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DA60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DA64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DA68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DA6C: 386BE740  addi r3, r11, -0x18c0
	ctx.r[3].s64 = ctx.r[11].s64 + -6336;
	// 8324DA70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DA74: 4AFA62E5  bl 0x821f3d58
	ctx.lr = 0x8324DA78;
	sub_821F3D58(ctx, base);
	// 8324DA78: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DA7C: 906A7C30  stw r3, 0x7c30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31792 as u32), ctx.r[3].u32 ) };
	// 8324DA80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DA90 size=56
    let mut pc: u32 = 0x8324DA90;
    'dispatch: loop {
        match pc {
            0x8324DA90 => {
    //   block [0x8324DA90..0x8324DAC8)
	// 8324DA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DA98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DA9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DAA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DAA4: 386BE758  addi r3, r11, -0x18a8
	ctx.r[3].s64 = ctx.r[11].s64 + -6312;
	// 8324DAA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DAAC: 4AFA62AD  bl 0x821f3d58
	ctx.lr = 0x8324DAB0;
	sub_821F3D58(ctx, base);
	// 8324DAB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DAB4: 906A7C34  stw r3, 0x7c34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31796 as u32), ctx.r[3].u32 ) };
	// 8324DAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DAC8 size=56
    let mut pc: u32 = 0x8324DAC8;
    'dispatch: loop {
        match pc {
            0x8324DAC8 => {
    //   block [0x8324DAC8..0x8324DB00)
	// 8324DAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DAD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DAD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DAD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DADC: 386BE770  addi r3, r11, -0x1890
	ctx.r[3].s64 = ctx.r[11].s64 + -6288;
	// 8324DAE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DAE4: 4AFA6275  bl 0x821f3d58
	ctx.lr = 0x8324DAE8;
	sub_821F3D58(ctx, base);
	// 8324DAE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DAEC: 906A7C38  stw r3, 0x7c38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31800 as u32), ctx.r[3].u32 ) };
	// 8324DAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DB00 size=56
    let mut pc: u32 = 0x8324DB00;
    'dispatch: loop {
        match pc {
            0x8324DB00 => {
    //   block [0x8324DB00..0x8324DB38)
	// 8324DB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DB0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DB10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DB14: 386BE780  addi r3, r11, -0x1880
	ctx.r[3].s64 = ctx.r[11].s64 + -6272;
	// 8324DB18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DB1C: 4AFA623D  bl 0x821f3d58
	ctx.lr = 0x8324DB20;
	sub_821F3D58(ctx, base);
	// 8324DB20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DB24: 906A7C3C  stw r3, 0x7c3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31804 as u32), ctx.r[3].u32 ) };
	// 8324DB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DB38 size=64
    let mut pc: u32 = 0x8324DB38;
    'dispatch: loop {
        match pc {
            0x8324DB38 => {
    //   block [0x8324DB38..0x8324DB78)
	// 8324DB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DB44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DB48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DB4C: 388BE790  addi r4, r11, -0x1870
	ctx.r[4].s64 = ctx.r[11].s64 + -6256;
	// 8324DB50: 386A7C40  addi r3, r10, 0x7c40
	ctx.r[3].s64 = ctx.r[10].s64 + 31808;
	// 8324DB54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DB58: 4AFDF379  bl 0x8222ced0
	ctx.lr = 0x8324DB5C;
	sub_8222CED0(ctx, base);
	// 8324DB5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DB60: 38698A68  addi r3, r9, -0x7598
	ctx.r[3].s64 = ctx.r[9].s64 + -30104;
	// 8324DB64: 4BA5C3BD  bl 0x82ca9f20
	ctx.lr = 0x8324DB68;
	sub_82CA9F20(ctx, base);
	// 8324DB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DB78 size=64
    let mut pc: u32 = 0x8324DB78;
    'dispatch: loop {
        match pc {
            0x8324DB78 => {
    //   block [0x8324DB78..0x8324DBB8)
	// 8324DB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DB84: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DB88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DB8C: 388BE79C  addi r4, r11, -0x1864
	ctx.r[4].s64 = ctx.r[11].s64 + -6244;
	// 8324DB90: 386A7C44  addi r3, r10, 0x7c44
	ctx.r[3].s64 = ctx.r[10].s64 + 31812;
	// 8324DB94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DB98: 4AFDF339  bl 0x8222ced0
	ctx.lr = 0x8324DB9C;
	sub_8222CED0(ctx, base);
	// 8324DB9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DBA0: 38698A78  addi r3, r9, -0x7588
	ctx.r[3].s64 = ctx.r[9].s64 + -30088;
	// 8324DBA4: 4BA5C37D  bl 0x82ca9f20
	ctx.lr = 0x8324DBA8;
	sub_82CA9F20(ctx, base);
	// 8324DBA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DBB8 size=64
    let mut pc: u32 = 0x8324DBB8;
    'dispatch: loop {
        match pc {
            0x8324DBB8 => {
    //   block [0x8324DBB8..0x8324DBF8)
	// 8324DBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DBC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DBC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DBC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DBCC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324DBD0: 386A7C48  addi r3, r10, 0x7c48
	ctx.r[3].s64 = ctx.r[10].s64 + 31816;
	// 8324DBD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DBD8: 4AFDF2F9  bl 0x8222ced0
	ctx.lr = 0x8324DBDC;
	sub_8222CED0(ctx, base);
	// 8324DBDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DBE0: 38698A88  addi r3, r9, -0x7578
	ctx.r[3].s64 = ctx.r[9].s64 + -30072;
	// 8324DBE4: 4BA5C33D  bl 0x82ca9f20
	ctx.lr = 0x8324DBE8;
	sub_82CA9F20(ctx, base);
	// 8324DBE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DBF8 size=64
    let mut pc: u32 = 0x8324DBF8;
    'dispatch: loop {
        match pc {
            0x8324DBF8 => {
    //   block [0x8324DBF8..0x8324DC38)
	// 8324DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DC00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DC04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DC08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DC0C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324DC10: 386A7C4C  addi r3, r10, 0x7c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 31820;
	// 8324DC14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DC18: 4AFDF2B9  bl 0x8222ced0
	ctx.lr = 0x8324DC1C;
	sub_8222CED0(ctx, base);
	// 8324DC1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DC20: 38698A98  addi r3, r9, -0x7568
	ctx.r[3].s64 = ctx.r[9].s64 + -30056;
	// 8324DC24: 4BA5C2FD  bl 0x82ca9f20
	ctx.lr = 0x8324DC28;
	sub_82CA9F20(ctx, base);
	// 8324DC28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DC38 size=64
    let mut pc: u32 = 0x8324DC38;
    'dispatch: loop {
        match pc {
            0x8324DC38 => {
    //   block [0x8324DC38..0x8324DC78)
	// 8324DC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DC40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DC44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DC48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DC4C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324DC50: 386A7C50  addi r3, r10, 0x7c50
	ctx.r[3].s64 = ctx.r[10].s64 + 31824;
	// 8324DC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DC58: 4AFDF279  bl 0x8222ced0
	ctx.lr = 0x8324DC5C;
	sub_8222CED0(ctx, base);
	// 8324DC5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DC60: 38698AA8  addi r3, r9, -0x7558
	ctx.r[3].s64 = ctx.r[9].s64 + -30040;
	// 8324DC64: 4BA5C2BD  bl 0x82ca9f20
	ctx.lr = 0x8324DC68;
	sub_82CA9F20(ctx, base);
	// 8324DC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DC78 size=64
    let mut pc: u32 = 0x8324DC78;
    'dispatch: loop {
        match pc {
            0x8324DC78 => {
    //   block [0x8324DC78..0x8324DCB8)
	// 8324DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DC88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DC8C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324DC90: 386A7C54  addi r3, r10, 0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + 31828;
	// 8324DC94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DC98: 4AFDF239  bl 0x8222ced0
	ctx.lr = 0x8324DC9C;
	sub_8222CED0(ctx, base);
	// 8324DC9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DCA0: 38698AB8  addi r3, r9, -0x7548
	ctx.r[3].s64 = ctx.r[9].s64 + -30024;
	// 8324DCA4: 4BA5C27D  bl 0x82ca9f20
	ctx.lr = 0x8324DCA8;
	sub_82CA9F20(ctx, base);
	// 8324DCA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DCB8 size=56
    let mut pc: u32 = 0x8324DCB8;
    'dispatch: loop {
        match pc {
            0x8324DCB8 => {
    //   block [0x8324DCB8..0x8324DCF0)
	// 8324DCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DCC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DCC4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DCC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DCCC: 386BE7F8  addi r3, r11, -0x1808
	ctx.r[3].s64 = ctx.r[11].s64 + -6152;
	// 8324DCD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DCD4: 4AFA6085  bl 0x821f3d58
	ctx.lr = 0x8324DCD8;
	sub_821F3D58(ctx, base);
	// 8324DCD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DCDC: 906A7C58  stw r3, 0x7c58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31832 as u32), ctx.r[3].u32 ) };
	// 8324DCE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DCF0 size=56
    let mut pc: u32 = 0x8324DCF0;
    'dispatch: loop {
        match pc {
            0x8324DCF0 => {
    //   block [0x8324DCF0..0x8324DD28)
	// 8324DCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DCF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DCFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DD00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DD04: 386BE800  addi r3, r11, -0x1800
	ctx.r[3].s64 = ctx.r[11].s64 + -6144;
	// 8324DD08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DD0C: 4AFA604D  bl 0x821f3d58
	ctx.lr = 0x8324DD10;
	sub_821F3D58(ctx, base);
	// 8324DD10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DD14: 906A7C5C  stw r3, 0x7c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31836 as u32), ctx.r[3].u32 ) };
	// 8324DD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DD28 size=56
    let mut pc: u32 = 0x8324DD28;
    'dispatch: loop {
        match pc {
            0x8324DD28 => {
    //   block [0x8324DD28..0x8324DD60)
	// 8324DD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DD34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DD38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DD3C: 386BE80C  addi r3, r11, -0x17f4
	ctx.r[3].s64 = ctx.r[11].s64 + -6132;
	// 8324DD40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DD44: 4AFA6015  bl 0x821f3d58
	ctx.lr = 0x8324DD48;
	sub_821F3D58(ctx, base);
	// 8324DD48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DD4C: 906A7C60  stw r3, 0x7c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31840 as u32), ctx.r[3].u32 ) };
	// 8324DD50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DD60 size=56
    let mut pc: u32 = 0x8324DD60;
    'dispatch: loop {
        match pc {
            0x8324DD60 => {
    //   block [0x8324DD60..0x8324DD98)
	// 8324DD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DD68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DD6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DD70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DD74: 386BE814  addi r3, r11, -0x17ec
	ctx.r[3].s64 = ctx.r[11].s64 + -6124;
	// 8324DD78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DD7C: 4AFA5FDD  bl 0x821f3d58
	ctx.lr = 0x8324DD80;
	sub_821F3D58(ctx, base);
	// 8324DD80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DD84: 906A7C64  stw r3, 0x7c64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31844 as u32), ctx.r[3].u32 ) };
	// 8324DD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DD98 size=56
    let mut pc: u32 = 0x8324DD98;
    'dispatch: loop {
        match pc {
            0x8324DD98 => {
    //   block [0x8324DD98..0x8324DDD0)
	// 8324DD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DDA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DDA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DDAC: 386BE824  addi r3, r11, -0x17dc
	ctx.r[3].s64 = ctx.r[11].s64 + -6108;
	// 8324DDB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DDB4: 4AFA5FA5  bl 0x821f3d58
	ctx.lr = 0x8324DDB8;
	sub_821F3D58(ctx, base);
	// 8324DDB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DDBC: 906A7C68  stw r3, 0x7c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31848 as u32), ctx.r[3].u32 ) };
	// 8324DDC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DDD0 size=56
    let mut pc: u32 = 0x8324DDD0;
    'dispatch: loop {
        match pc {
            0x8324DDD0 => {
    //   block [0x8324DDD0..0x8324DE08)
	// 8324DDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DDD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DDDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DDE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DDE4: 386BE834  addi r3, r11, -0x17cc
	ctx.r[3].s64 = ctx.r[11].s64 + -6092;
	// 8324DDE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DDEC: 4AFA5F6D  bl 0x821f3d58
	ctx.lr = 0x8324DDF0;
	sub_821F3D58(ctx, base);
	// 8324DDF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DDF4: 906A7C6C  stw r3, 0x7c6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31852 as u32), ctx.r[3].u32 ) };
	// 8324DDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DE08 size=56
    let mut pc: u32 = 0x8324DE08;
    'dispatch: loop {
        match pc {
            0x8324DE08 => {
    //   block [0x8324DE08..0x8324DE40)
	// 8324DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DE14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DE18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DE1C: 386BE848  addi r3, r11, -0x17b8
	ctx.r[3].s64 = ctx.r[11].s64 + -6072;
	// 8324DE20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DE24: 4AFA5F35  bl 0x821f3d58
	ctx.lr = 0x8324DE28;
	sub_821F3D58(ctx, base);
	// 8324DE28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DE2C: 906A7C70  stw r3, 0x7c70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31856 as u32), ctx.r[3].u32 ) };
	// 8324DE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DE40 size=56
    let mut pc: u32 = 0x8324DE40;
    'dispatch: loop {
        match pc {
            0x8324DE40 => {
    //   block [0x8324DE40..0x8324DE78)
	// 8324DE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DE4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324DE54: 386BE85C  addi r3, r11, -0x17a4
	ctx.r[3].s64 = ctx.r[11].s64 + -6052;
	// 8324DE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324DE5C: 4AFA5EFD  bl 0x821f3d58
	ctx.lr = 0x8324DE60;
	sub_821F3D58(ctx, base);
	// 8324DE60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DE64: 906A7C74  stw r3, 0x7c74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31860 as u32), ctx.r[3].u32 ) };
	// 8324DE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DE78 size=64
    let mut pc: u32 = 0x8324DE78;
    'dispatch: loop {
        match pc {
            0x8324DE78 => {
    //   block [0x8324DE78..0x8324DEB8)
	// 8324DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DE84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DE88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DE8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324DE90: 386A7C78  addi r3, r10, 0x7c78
	ctx.r[3].s64 = ctx.r[10].s64 + 31864;
	// 8324DE94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DE98: 4AFDF039  bl 0x8222ced0
	ctx.lr = 0x8324DE9C;
	sub_8222CED0(ctx, base);
	// 8324DE9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DEA0: 38698AC8  addi r3, r9, -0x7538
	ctx.r[3].s64 = ctx.r[9].s64 + -30008;
	// 8324DEA4: 4BA5C07D  bl 0x82ca9f20
	ctx.lr = 0x8324DEA8;
	sub_82CA9F20(ctx, base);
	// 8324DEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DEB8 size=64
    let mut pc: u32 = 0x8324DEB8;
    'dispatch: loop {
        match pc {
            0x8324DEB8 => {
    //   block [0x8324DEB8..0x8324DEF8)
	// 8324DEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DEC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DEC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DEC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DECC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324DED0: 386A7C7C  addi r3, r10, 0x7c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 31868;
	// 8324DED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DED8: 4AFDEFF9  bl 0x8222ced0
	ctx.lr = 0x8324DEDC;
	sub_8222CED0(ctx, base);
	// 8324DEDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DEE0: 38698AD8  addi r3, r9, -0x7528
	ctx.r[3].s64 = ctx.r[9].s64 + -29992;
	// 8324DEE4: 4BA5C03D  bl 0x82ca9f20
	ctx.lr = 0x8324DEE8;
	sub_82CA9F20(ctx, base);
	// 8324DEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DEF8 size=64
    let mut pc: u32 = 0x8324DEF8;
    'dispatch: loop {
        match pc {
            0x8324DEF8 => {
    //   block [0x8324DEF8..0x8324DF38)
	// 8324DEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DF04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324DF08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DF0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324DF10: 386A7C80  addi r3, r10, 0x7c80
	ctx.r[3].s64 = ctx.r[10].s64 + 31872;
	// 8324DF14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DF18: 4AFDEFB9  bl 0x8222ced0
	ctx.lr = 0x8324DF1C;
	sub_8222CED0(ctx, base);
	// 8324DF1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DF20: 38698AE8  addi r3, r9, -0x7518
	ctx.r[3].s64 = ctx.r[9].s64 + -29976;
	// 8324DF24: 4BA5BFFD  bl 0x82ca9f20
	ctx.lr = 0x8324DF28;
	sub_82CA9F20(ctx, base);
	// 8324DF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DF38 size=64
    let mut pc: u32 = 0x8324DF38;
    'dispatch: loop {
        match pc {
            0x8324DF38 => {
    //   block [0x8324DF38..0x8324DF78)
	// 8324DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DF40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DF44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DF48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DF4C: 388BE874  addi r4, r11, -0x178c
	ctx.r[4].s64 = ctx.r[11].s64 + -6028;
	// 8324DF50: 386A7C84  addi r3, r10, 0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + 31876;
	// 8324DF54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DF58: 4AFDEF79  bl 0x8222ced0
	ctx.lr = 0x8324DF5C;
	sub_8222CED0(ctx, base);
	// 8324DF5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DF60: 38698AF8  addi r3, r9, -0x7508
	ctx.r[3].s64 = ctx.r[9].s64 + -29960;
	// 8324DF64: 4BA5BFBD  bl 0x82ca9f20
	ctx.lr = 0x8324DF68;
	sub_82CA9F20(ctx, base);
	// 8324DF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DF78 size=64
    let mut pc: u32 = 0x8324DF78;
    'dispatch: loop {
        match pc {
            0x8324DF78 => {
    //   block [0x8324DF78..0x8324DFB8)
	// 8324DF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DF84: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DF88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DF8C: 388BE894  addi r4, r11, -0x176c
	ctx.r[4].s64 = ctx.r[11].s64 + -5996;
	// 8324DF90: 386A7C88  addi r3, r10, 0x7c88
	ctx.r[3].s64 = ctx.r[10].s64 + 31880;
	// 8324DF94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DF98: 4AFDEF39  bl 0x8222ced0
	ctx.lr = 0x8324DF9C;
	sub_8222CED0(ctx, base);
	// 8324DF9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DFA0: 38698B08  addi r3, r9, -0x74f8
	ctx.r[3].s64 = ctx.r[9].s64 + -29944;
	// 8324DFA4: 4BA5BF7D  bl 0x82ca9f20
	ctx.lr = 0x8324DFA8;
	sub_82CA9F20(ctx, base);
	// 8324DFA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DFB8 size=64
    let mut pc: u32 = 0x8324DFB8;
    'dispatch: loop {
        match pc {
            0x8324DFB8 => {
    //   block [0x8324DFB8..0x8324DFF8)
	// 8324DFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324DFC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324DFC4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324DFC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324DFCC: 388BE8B8  addi r4, r11, -0x1748
	ctx.r[4].s64 = ctx.r[11].s64 + -5960;
	// 8324DFD0: 386A7C8C  addi r3, r10, 0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 31884;
	// 8324DFD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324DFD8: 4AFDEEF9  bl 0x8222ced0
	ctx.lr = 0x8324DFDC;
	sub_8222CED0(ctx, base);
	// 8324DFDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324DFE0: 38698B18  addi r3, r9, -0x74e8
	ctx.r[3].s64 = ctx.r[9].s64 + -29928;
	// 8324DFE4: 4BA5BF3D  bl 0x82ca9f20
	ctx.lr = 0x8324DFE8;
	sub_82CA9F20(ctx, base);
	// 8324DFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324DFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324DFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324DFF8 size=64
    let mut pc: u32 = 0x8324DFF8;
    'dispatch: loop {
        match pc {
            0x8324DFF8 => {
    //   block [0x8324DFF8..0x8324E038)
	// 8324DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324DFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E004: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E008: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E00C: 388BE8DC  addi r4, r11, -0x1724
	ctx.r[4].s64 = ctx.r[11].s64 + -5924;
	// 8324E010: 386A7C90  addi r3, r10, 0x7c90
	ctx.r[3].s64 = ctx.r[10].s64 + 31888;
	// 8324E014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E018: 4AFDEEB9  bl 0x8222ced0
	ctx.lr = 0x8324E01C;
	sub_8222CED0(ctx, base);
	// 8324E01C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E020: 38698B28  addi r3, r9, -0x74d8
	ctx.r[3].s64 = ctx.r[9].s64 + -29912;
	// 8324E024: 4BA5BEFD  bl 0x82ca9f20
	ctx.lr = 0x8324E028;
	sub_82CA9F20(ctx, base);
	// 8324E028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E038 size=64
    let mut pc: u32 = 0x8324E038;
    'dispatch: loop {
        match pc {
            0x8324E038 => {
    //   block [0x8324E038..0x8324E078)
	// 8324E038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E044: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E048: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E04C: 388BE904  addi r4, r11, -0x16fc
	ctx.r[4].s64 = ctx.r[11].s64 + -5884;
	// 8324E050: 386A7C94  addi r3, r10, 0x7c94
	ctx.r[3].s64 = ctx.r[10].s64 + 31892;
	// 8324E054: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E058: 4AFDEE79  bl 0x8222ced0
	ctx.lr = 0x8324E05C;
	sub_8222CED0(ctx, base);
	// 8324E05C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E060: 38698B38  addi r3, r9, -0x74c8
	ctx.r[3].s64 = ctx.r[9].s64 + -29896;
	// 8324E064: 4BA5BEBD  bl 0x82ca9f20
	ctx.lr = 0x8324E068;
	sub_82CA9F20(ctx, base);
	// 8324E068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E078 size=64
    let mut pc: u32 = 0x8324E078;
    'dispatch: loop {
        match pc {
            0x8324E078 => {
    //   block [0x8324E078..0x8324E0B8)
	// 8324E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E084: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E088: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E08C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E090: 386A7C98  addi r3, r10, 0x7c98
	ctx.r[3].s64 = ctx.r[10].s64 + 31896;
	// 8324E094: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E098: 4AFDEE39  bl 0x8222ced0
	ctx.lr = 0x8324E09C;
	sub_8222CED0(ctx, base);
	// 8324E09C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E0A0: 38698B48  addi r3, r9, -0x74b8
	ctx.r[3].s64 = ctx.r[9].s64 + -29880;
	// 8324E0A4: 4BA5BE7D  bl 0x82ca9f20
	ctx.lr = 0x8324E0A8;
	sub_82CA9F20(ctx, base);
	// 8324E0A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E0B8 size=64
    let mut pc: u32 = 0x8324E0B8;
    'dispatch: loop {
        match pc {
            0x8324E0B8 => {
    //   block [0x8324E0B8..0x8324E0F8)
	// 8324E0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E0C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E0C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E0C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E0CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324E0D0: 386A7C9C  addi r3, r10, 0x7c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 31900;
	// 8324E0D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E0D8: 4AFDEDF9  bl 0x8222ced0
	ctx.lr = 0x8324E0DC;
	sub_8222CED0(ctx, base);
	// 8324E0DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E0E0: 38698B58  addi r3, r9, -0x74a8
	ctx.r[3].s64 = ctx.r[9].s64 + -29864;
	// 8324E0E4: 4BA5BE3D  bl 0x82ca9f20
	ctx.lr = 0x8324E0E8;
	sub_82CA9F20(ctx, base);
	// 8324E0E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E0F8 size=64
    let mut pc: u32 = 0x8324E0F8;
    'dispatch: loop {
        match pc {
            0x8324E0F8 => {
    //   block [0x8324E0F8..0x8324E138)
	// 8324E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E108: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E10C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324E110: 386A7CA0  addi r3, r10, 0x7ca0
	ctx.r[3].s64 = ctx.r[10].s64 + 31904;
	// 8324E114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E118: 4AFDEDB9  bl 0x8222ced0
	ctx.lr = 0x8324E11C;
	sub_8222CED0(ctx, base);
	// 8324E11C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E120: 38698B68  addi r3, r9, -0x7498
	ctx.r[3].s64 = ctx.r[9].s64 + -29848;
	// 8324E124: 4BA5BDFD  bl 0x82ca9f20
	ctx.lr = 0x8324E128;
	sub_82CA9F20(ctx, base);
	// 8324E128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E138 size=64
    let mut pc: u32 = 0x8324E138;
    'dispatch: loop {
        match pc {
            0x8324E138 => {
    //   block [0x8324E138..0x8324E178)
	// 8324E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E148: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E14C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E150: 386A7CA4  addi r3, r10, 0x7ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 31908;
	// 8324E154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E158: 4AFDED79  bl 0x8222ced0
	ctx.lr = 0x8324E15C;
	sub_8222CED0(ctx, base);
	// 8324E15C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E160: 38698B78  addi r3, r9, -0x7488
	ctx.r[3].s64 = ctx.r[9].s64 + -29832;
	// 8324E164: 4BA5BDBD  bl 0x82ca9f20
	ctx.lr = 0x8324E168;
	sub_82CA9F20(ctx, base);
	// 8324E168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E178 size=56
    let mut pc: u32 = 0x8324E178;
    'dispatch: loop {
        match pc {
            0x8324E178 => {
    //   block [0x8324E178..0x8324E1B0)
	// 8324E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E184: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E18C: 386BEA10  addi r3, r11, -0x15f0
	ctx.r[3].s64 = ctx.r[11].s64 + -5616;
	// 8324E190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E194: 4AFA5BC5  bl 0x821f3d58
	ctx.lr = 0x8324E198;
	sub_821F3D58(ctx, base);
	// 8324E198: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E19C: 906A7CA8  stw r3, 0x7ca8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31912 as u32), ctx.r[3].u32 ) };
	// 8324E1A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E1B0 size=56
    let mut pc: u32 = 0x8324E1B0;
    'dispatch: loop {
        match pc {
            0x8324E1B0 => {
    //   block [0x8324E1B0..0x8324E1E8)
	// 8324E1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E1BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E1C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E1C4: 386BEA20  addi r3, r11, -0x15e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5600;
	// 8324E1C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E1CC: 4AFA5B8D  bl 0x821f3d58
	ctx.lr = 0x8324E1D0;
	sub_821F3D58(ctx, base);
	// 8324E1D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E1D4: 906A7CAC  stw r3, 0x7cac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31916 as u32), ctx.r[3].u32 ) };
	// 8324E1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E1E8 size=56
    let mut pc: u32 = 0x8324E1E8;
    'dispatch: loop {
        match pc {
            0x8324E1E8 => {
    //   block [0x8324E1E8..0x8324E220)
	// 8324E1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E1F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E1F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E1F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E1FC: 386BEA34  addi r3, r11, -0x15cc
	ctx.r[3].s64 = ctx.r[11].s64 + -5580;
	// 8324E200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E204: 4AFA5B55  bl 0x821f3d58
	ctx.lr = 0x8324E208;
	sub_821F3D58(ctx, base);
	// 8324E208: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E20C: 906A7CB0  stw r3, 0x7cb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31920 as u32), ctx.r[3].u32 ) };
	// 8324E210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E220 size=56
    let mut pc: u32 = 0x8324E220;
    'dispatch: loop {
        match pc {
            0x8324E220 => {
    //   block [0x8324E220..0x8324E258)
	// 8324E220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E22C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E234: 386BEA44  addi r3, r11, -0x15bc
	ctx.r[3].s64 = ctx.r[11].s64 + -5564;
	// 8324E238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E23C: 4AFA5B1D  bl 0x821f3d58
	ctx.lr = 0x8324E240;
	sub_821F3D58(ctx, base);
	// 8324E240: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E244: 906A7CB4  stw r3, 0x7cb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31924 as u32), ctx.r[3].u32 ) };
	// 8324E248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E258 size=56
    let mut pc: u32 = 0x8324E258;
    'dispatch: loop {
        match pc {
            0x8324E258 => {
    //   block [0x8324E258..0x8324E290)
	// 8324E258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E264: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E26C: 386BEA54  addi r3, r11, -0x15ac
	ctx.r[3].s64 = ctx.r[11].s64 + -5548;
	// 8324E270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E274: 4AFA5AE5  bl 0x821f3d58
	ctx.lr = 0x8324E278;
	sub_821F3D58(ctx, base);
	// 8324E278: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E27C: 906A7CB8  stw r3, 0x7cb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31928 as u32), ctx.r[3].u32 ) };
	// 8324E280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E290 size=56
    let mut pc: u32 = 0x8324E290;
    'dispatch: loop {
        match pc {
            0x8324E290 => {
    //   block [0x8324E290..0x8324E2C8)
	// 8324E290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E29C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E2A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E2A4: 386BEA68  addi r3, r11, -0x1598
	ctx.r[3].s64 = ctx.r[11].s64 + -5528;
	// 8324E2A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E2AC: 4AFA5AAD  bl 0x821f3d58
	ctx.lr = 0x8324E2B0;
	sub_821F3D58(ctx, base);
	// 8324E2B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E2B4: 906A7CBC  stw r3, 0x7cbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31932 as u32), ctx.r[3].u32 ) };
	// 8324E2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E2C8 size=56
    let mut pc: u32 = 0x8324E2C8;
    'dispatch: loop {
        match pc {
            0x8324E2C8 => {
    //   block [0x8324E2C8..0x8324E300)
	// 8324E2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E2D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E2D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E2DC: 386BEA78  addi r3, r11, -0x1588
	ctx.r[3].s64 = ctx.r[11].s64 + -5512;
	// 8324E2E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E2E4: 4AFA5A75  bl 0x821f3d58
	ctx.lr = 0x8324E2E8;
	sub_821F3D58(ctx, base);
	// 8324E2E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E2EC: 906A7CC0  stw r3, 0x7cc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31936 as u32), ctx.r[3].u32 ) };
	// 8324E2F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E300 size=56
    let mut pc: u32 = 0x8324E300;
    'dispatch: loop {
        match pc {
            0x8324E300 => {
    //   block [0x8324E300..0x8324E338)
	// 8324E300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E30C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E314: 386BEA88  addi r3, r11, -0x1578
	ctx.r[3].s64 = ctx.r[11].s64 + -5496;
	// 8324E318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E31C: 4AFA5A3D  bl 0x821f3d58
	ctx.lr = 0x8324E320;
	sub_821F3D58(ctx, base);
	// 8324E320: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E324: 906A7CC4  stw r3, 0x7cc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31940 as u32), ctx.r[3].u32 ) };
	// 8324E328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E338 size=56
    let mut pc: u32 = 0x8324E338;
    'dispatch: loop {
        match pc {
            0x8324E338 => {
    //   block [0x8324E338..0x8324E370)
	// 8324E338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E344: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E34C: 386BEA98  addi r3, r11, -0x1568
	ctx.r[3].s64 = ctx.r[11].s64 + -5480;
	// 8324E350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E354: 4AFA5A05  bl 0x821f3d58
	ctx.lr = 0x8324E358;
	sub_821F3D58(ctx, base);
	// 8324E358: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E35C: 906A7CC8  stw r3, 0x7cc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31944 as u32), ctx.r[3].u32 ) };
	// 8324E360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E370 size=64
    let mut pc: u32 = 0x8324E370;
    'dispatch: loop {
        match pc {
            0x8324E370 => {
    //   block [0x8324E370..0x8324E3B0)
	// 8324E370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E37C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E380: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E384: 388BEAA8  addi r4, r11, -0x1558
	ctx.r[4].s64 = ctx.r[11].s64 + -5464;
	// 8324E388: 386A7CCC  addi r3, r10, 0x7ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 31948;
	// 8324E38C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E390: 4AFDEB41  bl 0x8222ced0
	ctx.lr = 0x8324E394;
	sub_8222CED0(ctx, base);
	// 8324E394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E398: 38698B88  addi r3, r9, -0x7478
	ctx.r[3].s64 = ctx.r[9].s64 + -29816;
	// 8324E39C: 4BA5BB85  bl 0x82ca9f20
	ctx.lr = 0x8324E3A0;
	sub_82CA9F20(ctx, base);
	// 8324E3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E3B0 size=64
    let mut pc: u32 = 0x8324E3B0;
    'dispatch: loop {
        match pc {
            0x8324E3B0 => {
    //   block [0x8324E3B0..0x8324E3F0)
	// 8324E3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E3BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E3C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E3C4: 388BEAC4  addi r4, r11, -0x153c
	ctx.r[4].s64 = ctx.r[11].s64 + -5436;
	// 8324E3C8: 386A7CD0  addi r3, r10, 0x7cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 31952;
	// 8324E3CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E3D0: 4AFDEB01  bl 0x8222ced0
	ctx.lr = 0x8324E3D4;
	sub_8222CED0(ctx, base);
	// 8324E3D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E3D8: 38698B98  addi r3, r9, -0x7468
	ctx.r[3].s64 = ctx.r[9].s64 + -29800;
	// 8324E3DC: 4BA5BB45  bl 0x82ca9f20
	ctx.lr = 0x8324E3E0;
	sub_82CA9F20(ctx, base);
	// 8324E3E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E3F0 size=64
    let mut pc: u32 = 0x8324E3F0;
    'dispatch: loop {
        match pc {
            0x8324E3F0 => {
    //   block [0x8324E3F0..0x8324E430)
	// 8324E3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E3F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E3FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E400: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E404: 388BEAE0  addi r4, r11, -0x1520
	ctx.r[4].s64 = ctx.r[11].s64 + -5408;
	// 8324E408: 386A7CD4  addi r3, r10, 0x7cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31956;
	// 8324E40C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E410: 4AFDEAC1  bl 0x8222ced0
	ctx.lr = 0x8324E414;
	sub_8222CED0(ctx, base);
	// 8324E414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E418: 38698BA8  addi r3, r9, -0x7458
	ctx.r[3].s64 = ctx.r[9].s64 + -29784;
	// 8324E41C: 4BA5BB05  bl 0x82ca9f20
	ctx.lr = 0x8324E420;
	sub_82CA9F20(ctx, base);
	// 8324E420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E430 size=64
    let mut pc: u32 = 0x8324E430;
    'dispatch: loop {
        match pc {
            0x8324E430 => {
    //   block [0x8324E430..0x8324E470)
	// 8324E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E43C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E440: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E444: 388BEAFC  addi r4, r11, -0x1504
	ctx.r[4].s64 = ctx.r[11].s64 + -5380;
	// 8324E448: 386A7CD8  addi r3, r10, 0x7cd8
	ctx.r[3].s64 = ctx.r[10].s64 + 31960;
	// 8324E44C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E450: 4AFDEA81  bl 0x8222ced0
	ctx.lr = 0x8324E454;
	sub_8222CED0(ctx, base);
	// 8324E454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E458: 38698BB8  addi r3, r9, -0x7448
	ctx.r[3].s64 = ctx.r[9].s64 + -29768;
	// 8324E45C: 4BA5BAC5  bl 0x82ca9f20
	ctx.lr = 0x8324E460;
	sub_82CA9F20(ctx, base);
	// 8324E460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E470 size=64
    let mut pc: u32 = 0x8324E470;
    'dispatch: loop {
        match pc {
            0x8324E470 => {
    //   block [0x8324E470..0x8324E4B0)
	// 8324E470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E47C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E480: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E484: 388BE790  addi r4, r11, -0x1870
	ctx.r[4].s64 = ctx.r[11].s64 + -6256;
	// 8324E488: 386A7CDC  addi r3, r10, 0x7cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 31964;
	// 8324E48C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E490: 4AFDEA41  bl 0x8222ced0
	ctx.lr = 0x8324E494;
	sub_8222CED0(ctx, base);
	// 8324E494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E498: 38698BC8  addi r3, r9, -0x7438
	ctx.r[3].s64 = ctx.r[9].s64 + -29752;
	// 8324E49C: 4BA5BA85  bl 0x82ca9f20
	ctx.lr = 0x8324E4A0;
	sub_82CA9F20(ctx, base);
	// 8324E4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E4B0 size=64
    let mut pc: u32 = 0x8324E4B0;
    'dispatch: loop {
        match pc {
            0x8324E4B0 => {
    //   block [0x8324E4B0..0x8324E4F0)
	// 8324E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E4BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E4C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E4C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E4C8: 386A7CE0  addi r3, r10, 0x7ce0
	ctx.r[3].s64 = ctx.r[10].s64 + 31968;
	// 8324E4CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E4D0: 4AFDEA01  bl 0x8222ced0
	ctx.lr = 0x8324E4D4;
	sub_8222CED0(ctx, base);
	// 8324E4D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E4D8: 38698BD8  addi r3, r9, -0x7428
	ctx.r[3].s64 = ctx.r[9].s64 + -29736;
	// 8324E4DC: 4BA5BA45  bl 0x82ca9f20
	ctx.lr = 0x8324E4E0;
	sub_82CA9F20(ctx, base);
	// 8324E4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E4F0 size=64
    let mut pc: u32 = 0x8324E4F0;
    'dispatch: loop {
        match pc {
            0x8324E4F0 => {
    //   block [0x8324E4F0..0x8324E530)
	// 8324E4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E4FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E500: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E504: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324E508: 386A7CE4  addi r3, r10, 0x7ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 31972;
	// 8324E50C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E510: 4AFDE9C1  bl 0x8222ced0
	ctx.lr = 0x8324E514;
	sub_8222CED0(ctx, base);
	// 8324E514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E518: 38698BE8  addi r3, r9, -0x7418
	ctx.r[3].s64 = ctx.r[9].s64 + -29720;
	// 8324E51C: 4BA5BA05  bl 0x82ca9f20
	ctx.lr = 0x8324E520;
	sub_82CA9F20(ctx, base);
	// 8324E520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E530 size=64
    let mut pc: u32 = 0x8324E530;
    'dispatch: loop {
        match pc {
            0x8324E530 => {
    //   block [0x8324E530..0x8324E570)
	// 8324E530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E53C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E540: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E544: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324E548: 386A7CE8  addi r3, r10, 0x7ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 31976;
	// 8324E54C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E550: 4AFDE981  bl 0x8222ced0
	ctx.lr = 0x8324E554;
	sub_8222CED0(ctx, base);
	// 8324E554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E558: 38698BF8  addi r3, r9, -0x7408
	ctx.r[3].s64 = ctx.r[9].s64 + -29704;
	// 8324E55C: 4BA5B9C5  bl 0x82ca9f20
	ctx.lr = 0x8324E560;
	sub_82CA9F20(ctx, base);
	// 8324E560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E570 size=64
    let mut pc: u32 = 0x8324E570;
    'dispatch: loop {
        match pc {
            0x8324E570 => {
    //   block [0x8324E570..0x8324E5B0)
	// 8324E570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E57C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E580: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E584: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E588: 386A7CEC  addi r3, r10, 0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + 31980;
	// 8324E58C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E590: 4AFDE941  bl 0x8222ced0
	ctx.lr = 0x8324E594;
	sub_8222CED0(ctx, base);
	// 8324E594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E598: 38698C08  addi r3, r9, -0x73f8
	ctx.r[3].s64 = ctx.r[9].s64 + -29688;
	// 8324E59C: 4BA5B985  bl 0x82ca9f20
	ctx.lr = 0x8324E5A0;
	sub_82CA9F20(ctx, base);
	// 8324E5A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E5B0 size=64
    let mut pc: u32 = 0x8324E5B0;
    'dispatch: loop {
        match pc {
            0x8324E5B0 => {
    //   block [0x8324E5B0..0x8324E5F0)
	// 8324E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E5B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E5BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E5C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E5C4: 388BEDC8  addi r4, r11, -0x1238
	ctx.r[4].s64 = ctx.r[11].s64 + -4664;
	// 8324E5C8: 386A7CF0  addi r3, r10, 0x7cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 31984;
	// 8324E5CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E5D0: 4AFDE901  bl 0x8222ced0
	ctx.lr = 0x8324E5D4;
	sub_8222CED0(ctx, base);
	// 8324E5D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E5D8: 38698C18  addi r3, r9, -0x73e8
	ctx.r[3].s64 = ctx.r[9].s64 + -29672;
	// 8324E5DC: 4BA5B945  bl 0x82ca9f20
	ctx.lr = 0x8324E5E0;
	sub_82CA9F20(ctx, base);
	// 8324E5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E5F0 size=64
    let mut pc: u32 = 0x8324E5F0;
    'dispatch: loop {
        match pc {
            0x8324E5F0 => {
    //   block [0x8324E5F0..0x8324E630)
	// 8324E5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E5FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E600: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E604: 388BEDD4  addi r4, r11, -0x122c
	ctx.r[4].s64 = ctx.r[11].s64 + -4652;
	// 8324E608: 386A7CF4  addi r3, r10, 0x7cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31988;
	// 8324E60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E610: 4AFDE8C1  bl 0x8222ced0
	ctx.lr = 0x8324E614;
	sub_8222CED0(ctx, base);
	// 8324E614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E618: 38698C28  addi r3, r9, -0x73d8
	ctx.r[3].s64 = ctx.r[9].s64 + -29656;
	// 8324E61C: 4BA5B905  bl 0x82ca9f20
	ctx.lr = 0x8324E620;
	sub_82CA9F20(ctx, base);
	// 8324E620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E630 size=64
    let mut pc: u32 = 0x8324E630;
    'dispatch: loop {
        match pc {
            0x8324E630 => {
    //   block [0x8324E630..0x8324E670)
	// 8324E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E63C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E640: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E644: 388BEDF0  addi r4, r11, -0x1210
	ctx.r[4].s64 = ctx.r[11].s64 + -4624;
	// 8324E648: 386A7CF8  addi r3, r10, 0x7cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31992;
	// 8324E64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E650: 4AFDE881  bl 0x8222ced0
	ctx.lr = 0x8324E654;
	sub_8222CED0(ctx, base);
	// 8324E654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E658: 38698C38  addi r3, r9, -0x73c8
	ctx.r[3].s64 = ctx.r[9].s64 + -29640;
	// 8324E65C: 4BA5B8C5  bl 0x82ca9f20
	ctx.lr = 0x8324E660;
	sub_82CA9F20(ctx, base);
	// 8324E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E670 size=64
    let mut pc: u32 = 0x8324E670;
    'dispatch: loop {
        match pc {
            0x8324E670 => {
    //   block [0x8324E670..0x8324E6B0)
	// 8324E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E67C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E680: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E684: 388BEE14  addi r4, r11, -0x11ec
	ctx.r[4].s64 = ctx.r[11].s64 + -4588;
	// 8324E688: 386A7CFC  addi r3, r10, 0x7cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 31996;
	// 8324E68C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E690: 4AFDE841  bl 0x8222ced0
	ctx.lr = 0x8324E694;
	sub_8222CED0(ctx, base);
	// 8324E694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E698: 38698C48  addi r3, r9, -0x73b8
	ctx.r[3].s64 = ctx.r[9].s64 + -29624;
	// 8324E69C: 4BA5B885  bl 0x82ca9f20
	ctx.lr = 0x8324E6A0;
	sub_82CA9F20(ctx, base);
	// 8324E6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E6B0 size=64
    let mut pc: u32 = 0x8324E6B0;
    'dispatch: loop {
        match pc {
            0x8324E6B0 => {
    //   block [0x8324E6B0..0x8324E6F0)
	// 8324E6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E6BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E6C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E6C4: 388BEE2C  addi r4, r11, -0x11d4
	ctx.r[4].s64 = ctx.r[11].s64 + -4564;
	// 8324E6C8: 386A7D00  addi r3, r10, 0x7d00
	ctx.r[3].s64 = ctx.r[10].s64 + 32000;
	// 8324E6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E6D0: 4AFDE801  bl 0x8222ced0
	ctx.lr = 0x8324E6D4;
	sub_8222CED0(ctx, base);
	// 8324E6D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E6D8: 38698C58  addi r3, r9, -0x73a8
	ctx.r[3].s64 = ctx.r[9].s64 + -29608;
	// 8324E6DC: 4BA5B845  bl 0x82ca9f20
	ctx.lr = 0x8324E6E0;
	sub_82CA9F20(ctx, base);
	// 8324E6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E6F0 size=64
    let mut pc: u32 = 0x8324E6F0;
    'dispatch: loop {
        match pc {
            0x8324E6F0 => {
    //   block [0x8324E6F0..0x8324E730)
	// 8324E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E6FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E700: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E704: 388BEE3C  addi r4, r11, -0x11c4
	ctx.r[4].s64 = ctx.r[11].s64 + -4548;
	// 8324E708: 386A7D04  addi r3, r10, 0x7d04
	ctx.r[3].s64 = ctx.r[10].s64 + 32004;
	// 8324E70C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E710: 4AFDE7C1  bl 0x8222ced0
	ctx.lr = 0x8324E714;
	sub_8222CED0(ctx, base);
	// 8324E714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E718: 38698C68  addi r3, r9, -0x7398
	ctx.r[3].s64 = ctx.r[9].s64 + -29592;
	// 8324E71C: 4BA5B805  bl 0x82ca9f20
	ctx.lr = 0x8324E720;
	sub_82CA9F20(ctx, base);
	// 8324E720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E730 size=64
    let mut pc: u32 = 0x8324E730;
    'dispatch: loop {
        match pc {
            0x8324E730 => {
    //   block [0x8324E730..0x8324E770)
	// 8324E730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E73C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E740: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E744: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E748: 386A7D08  addi r3, r10, 0x7d08
	ctx.r[3].s64 = ctx.r[10].s64 + 32008;
	// 8324E74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E750: 4AFDE781  bl 0x8222ced0
	ctx.lr = 0x8324E754;
	sub_8222CED0(ctx, base);
	// 8324E754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E758: 38698C78  addi r3, r9, -0x7388
	ctx.r[3].s64 = ctx.r[9].s64 + -29576;
	// 8324E75C: 4BA5B7C5  bl 0x82ca9f20
	ctx.lr = 0x8324E760;
	sub_82CA9F20(ctx, base);
	// 8324E760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E770 size=64
    let mut pc: u32 = 0x8324E770;
    'dispatch: loop {
        match pc {
            0x8324E770 => {
    //   block [0x8324E770..0x8324E7B0)
	// 8324E770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E77C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E780: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E784: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E788: 386A7D0C  addi r3, r10, 0x7d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32012;
	// 8324E78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E790: 4AFDE741  bl 0x8222ced0
	ctx.lr = 0x8324E794;
	sub_8222CED0(ctx, base);
	// 8324E794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E798: 38698C88  addi r3, r9, -0x7378
	ctx.r[3].s64 = ctx.r[9].s64 + -29560;
	// 8324E79C: 4BA5B785  bl 0x82ca9f20
	ctx.lr = 0x8324E7A0;
	sub_82CA9F20(ctx, base);
	// 8324E7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E7B0 size=64
    let mut pc: u32 = 0x8324E7B0;
    'dispatch: loop {
        match pc {
            0x8324E7B0 => {
    //   block [0x8324E7B0..0x8324E7F0)
	// 8324E7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E7BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E7C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E7C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324E7C8: 386A7D10  addi r3, r10, 0x7d10
	ctx.r[3].s64 = ctx.r[10].s64 + 32016;
	// 8324E7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E7D0: 4AFDE701  bl 0x8222ced0
	ctx.lr = 0x8324E7D4;
	sub_8222CED0(ctx, base);
	// 8324E7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E7D8: 38698C98  addi r3, r9, -0x7368
	ctx.r[3].s64 = ctx.r[9].s64 + -29544;
	// 8324E7DC: 4BA5B745  bl 0x82ca9f20
	ctx.lr = 0x8324E7E0;
	sub_82CA9F20(ctx, base);
	// 8324E7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E7F0 size=64
    let mut pc: u32 = 0x8324E7F0;
    'dispatch: loop {
        match pc {
            0x8324E7F0 => {
    //   block [0x8324E7F0..0x8324E830)
	// 8324E7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E7FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E800: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E804: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324E808: 386A7D14  addi r3, r10, 0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + 32020;
	// 8324E80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E810: 4AFDE6C1  bl 0x8222ced0
	ctx.lr = 0x8324E814;
	sub_8222CED0(ctx, base);
	// 8324E814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E818: 38698CA8  addi r3, r9, -0x7358
	ctx.r[3].s64 = ctx.r[9].s64 + -29528;
	// 8324E81C: 4BA5B705  bl 0x82ca9f20
	ctx.lr = 0x8324E820;
	sub_82CA9F20(ctx, base);
	// 8324E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E830 size=64
    let mut pc: u32 = 0x8324E830;
    'dispatch: loop {
        match pc {
            0x8324E830 => {
    //   block [0x8324E830..0x8324E870)
	// 8324E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E840: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E844: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E848: 386A7D18  addi r3, r10, 0x7d18
	ctx.r[3].s64 = ctx.r[10].s64 + 32024;
	// 8324E84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E850: 4AFDE681  bl 0x8222ced0
	ctx.lr = 0x8324E854;
	sub_8222CED0(ctx, base);
	// 8324E854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E858: 38698CB8  addi r3, r9, -0x7348
	ctx.r[3].s64 = ctx.r[9].s64 + -29512;
	// 8324E85C: 4BA5B6C5  bl 0x82ca9f20
	ctx.lr = 0x8324E860;
	sub_82CA9F20(ctx, base);
	// 8324E860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E870 size=56
    let mut pc: u32 = 0x8324E870;
    'dispatch: loop {
        match pc {
            0x8324E870 => {
    //   block [0x8324E870..0x8324E8A8)
	// 8324E870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E87C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E884: 386BEEF0  addi r3, r11, -0x1110
	ctx.r[3].s64 = ctx.r[11].s64 + -4368;
	// 8324E888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E88C: 4AFA54CD  bl 0x821f3d58
	ctx.lr = 0x8324E890;
	sub_821F3D58(ctx, base);
	// 8324E890: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E894: 906A7D1C  stw r3, 0x7d1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32028 as u32), ctx.r[3].u32 ) };
	// 8324E898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E8A8 size=56
    let mut pc: u32 = 0x8324E8A8;
    'dispatch: loop {
        match pc {
            0x8324E8A8 => {
    //   block [0x8324E8A8..0x8324E8E0)
	// 8324E8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E8B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E8BC: 386BEF00  addi r3, r11, -0x1100
	ctx.r[3].s64 = ctx.r[11].s64 + -4352;
	// 8324E8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E8C4: 4AFA5495  bl 0x821f3d58
	ctx.lr = 0x8324E8C8;
	sub_821F3D58(ctx, base);
	// 8324E8C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E8CC: 906A7D20  stw r3, 0x7d20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32032 as u32), ctx.r[3].u32 ) };
	// 8324E8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E8E0 size=56
    let mut pc: u32 = 0x8324E8E0;
    'dispatch: loop {
        match pc {
            0x8324E8E0 => {
    //   block [0x8324E8E0..0x8324E918)
	// 8324E8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E8EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E8F4: 386BEF14  addi r3, r11, -0x10ec
	ctx.r[3].s64 = ctx.r[11].s64 + -4332;
	// 8324E8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E8FC: 4AFA545D  bl 0x821f3d58
	ctx.lr = 0x8324E900;
	sub_821F3D58(ctx, base);
	// 8324E900: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E904: 906A7D24  stw r3, 0x7d24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32036 as u32), ctx.r[3].u32 ) };
	// 8324E908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E918 size=56
    let mut pc: u32 = 0x8324E918;
    'dispatch: loop {
        match pc {
            0x8324E918 => {
    //   block [0x8324E918..0x8324E950)
	// 8324E918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E924: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324E928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324E92C: 386BEF28  addi r3, r11, -0x10d8
	ctx.r[3].s64 = ctx.r[11].s64 + -4312;
	// 8324E930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324E934: 4AFA5425  bl 0x821f3d58
	ctx.lr = 0x8324E938;
	sub_821F3D58(ctx, base);
	// 8324E938: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E93C: 906A7D28  stw r3, 0x7d28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32040 as u32), ctx.r[3].u32 ) };
	// 8324E940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324E950 size=12
    let mut pc: u32 = 0x8324E950;
    'dispatch: loop {
        match pc {
            0x8324E950 => {
    //   block [0x8324E950..0x8324E95C)
	// 8324E950: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324E954: 386B8CC8  addi r3, r11, -0x7338
	ctx.r[3].s64 = ctx.r[11].s64 + -29496;
	// 8324E958: 4BA5B5C8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324E960 size=12
    let mut pc: u32 = 0x8324E960;
    'dispatch: loop {
        match pc {
            0x8324E960 => {
    //   block [0x8324E960..0x8324E96C)
	// 8324E960: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324E964: 386B8D30  addi r3, r11, -0x72d0
	ctx.r[3].s64 = ctx.r[11].s64 + -29392;
	// 8324E968: 4BA5B5B8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E970 size=64
    let mut pc: u32 = 0x8324E970;
    'dispatch: loop {
        match pc {
            0x8324E970 => {
    //   block [0x8324E970..0x8324E9B0)
	// 8324E970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E97C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E980: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E984: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324E988: 386A7D4C  addi r3, r10, 0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32076;
	// 8324E98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E990: 4AFDE541  bl 0x8222ced0
	ctx.lr = 0x8324E994;
	sub_8222CED0(ctx, base);
	// 8324E994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E998: 38698D98  addi r3, r9, -0x7268
	ctx.r[3].s64 = ctx.r[9].s64 + -29288;
	// 8324E99C: 4BA5B585  bl 0x82ca9f20
	ctx.lr = 0x8324E9A0;
	sub_82CA9F20(ctx, base);
	// 8324E9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E9B0 size=64
    let mut pc: u32 = 0x8324E9B0;
    'dispatch: loop {
        match pc {
            0x8324E9B0 => {
    //   block [0x8324E9B0..0x8324E9F0)
	// 8324E9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E9BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324E9C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324E9C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324E9C8: 386A7D50  addi r3, r10, 0x7d50
	ctx.r[3].s64 = ctx.r[10].s64 + 32080;
	// 8324E9CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324E9D0: 4AFDE501  bl 0x8222ced0
	ctx.lr = 0x8324E9D4;
	sub_8222CED0(ctx, base);
	// 8324E9D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324E9D8: 38698DA8  addi r3, r9, -0x7258
	ctx.r[3].s64 = ctx.r[9].s64 + -29272;
	// 8324E9DC: 4BA5B545  bl 0x82ca9f20
	ctx.lr = 0x8324E9E0;
	sub_82CA9F20(ctx, base);
	// 8324E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324E9F0 size=64
    let mut pc: u32 = 0x8324E9F0;
    'dispatch: loop {
        match pc {
            0x8324E9F0 => {
    //   block [0x8324E9F0..0x8324EA30)
	// 8324E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324E9FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EA00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EA04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324EA08: 386A7D54  addi r3, r10, 0x7d54
	ctx.r[3].s64 = ctx.r[10].s64 + 32084;
	// 8324EA0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EA10: 4AFDE4C1  bl 0x8222ced0
	ctx.lr = 0x8324EA14;
	sub_8222CED0(ctx, base);
	// 8324EA14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EA18: 38698DB8  addi r3, r9, -0x7248
	ctx.r[3].s64 = ctx.r[9].s64 + -29256;
	// 8324EA1C: 4BA5B505  bl 0x82ca9f20
	ctx.lr = 0x8324EA20;
	sub_82CA9F20(ctx, base);
	// 8324EA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EA30 size=64
    let mut pc: u32 = 0x8324EA30;
    'dispatch: loop {
        match pc {
            0x8324EA30 => {
    //   block [0x8324EA30..0x8324EA70)
	// 8324EA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EA3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EA40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EA44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324EA48: 386A7D58  addi r3, r10, 0x7d58
	ctx.r[3].s64 = ctx.r[10].s64 + 32088;
	// 8324EA4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EA50: 4AFDE481  bl 0x8222ced0
	ctx.lr = 0x8324EA54;
	sub_8222CED0(ctx, base);
	// 8324EA54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EA58: 38698DC8  addi r3, r9, -0x7238
	ctx.r[3].s64 = ctx.r[9].s64 + -29240;
	// 8324EA5C: 4BA5B4C5  bl 0x82ca9f20
	ctx.lr = 0x8324EA60;
	sub_82CA9F20(ctx, base);
	// 8324EA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EA70 size=64
    let mut pc: u32 = 0x8324EA70;
    'dispatch: loop {
        match pc {
            0x8324EA70 => {
    //   block [0x8324EA70..0x8324EAB0)
	// 8324EA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EA7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EA80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EA84: 388BF070  addi r4, r11, -0xf90
	ctx.r[4].s64 = ctx.r[11].s64 + -3984;
	// 8324EA88: 386A7D5C  addi r3, r10, 0x7d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32092;
	// 8324EA8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EA90: 4AFDE441  bl 0x8222ced0
	ctx.lr = 0x8324EA94;
	sub_8222CED0(ctx, base);
	// 8324EA94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EA98: 38698DD8  addi r3, r9, -0x7228
	ctx.r[3].s64 = ctx.r[9].s64 + -29224;
	// 8324EA9C: 4BA5B485  bl 0x82ca9f20
	ctx.lr = 0x8324EAA0;
	sub_82CA9F20(ctx, base);
	// 8324EAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EAB0 size=64
    let mut pc: u32 = 0x8324EAB0;
    'dispatch: loop {
        match pc {
            0x8324EAB0 => {
    //   block [0x8324EAB0..0x8324EAF0)
	// 8324EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EABC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EAC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EAC4: 388BF088  addi r4, r11, -0xf78
	ctx.r[4].s64 = ctx.r[11].s64 + -3960;
	// 8324EAC8: 386A7D60  addi r3, r10, 0x7d60
	ctx.r[3].s64 = ctx.r[10].s64 + 32096;
	// 8324EACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EAD0: 4AFDE401  bl 0x8222ced0
	ctx.lr = 0x8324EAD4;
	sub_8222CED0(ctx, base);
	// 8324EAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EAD8: 38698DE8  addi r3, r9, -0x7218
	ctx.r[3].s64 = ctx.r[9].s64 + -29208;
	// 8324EADC: 4BA5B445  bl 0x82ca9f20
	ctx.lr = 0x8324EAE0;
	sub_82CA9F20(ctx, base);
	// 8324EAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EAF0 size=64
    let mut pc: u32 = 0x8324EAF0;
    'dispatch: loop {
        match pc {
            0x8324EAF0 => {
    //   block [0x8324EAF0..0x8324EB30)
	// 8324EAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EAFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EB00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EB04: 388BF0A8  addi r4, r11, -0xf58
	ctx.r[4].s64 = ctx.r[11].s64 + -3928;
	// 8324EB08: 386A7D64  addi r3, r10, 0x7d64
	ctx.r[3].s64 = ctx.r[10].s64 + 32100;
	// 8324EB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EB10: 4AFDE3C1  bl 0x8222ced0
	ctx.lr = 0x8324EB14;
	sub_8222CED0(ctx, base);
	// 8324EB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EB18: 38698DF8  addi r3, r9, -0x7208
	ctx.r[3].s64 = ctx.r[9].s64 + -29192;
	// 8324EB1C: 4BA5B405  bl 0x82ca9f20
	ctx.lr = 0x8324EB20;
	sub_82CA9F20(ctx, base);
	// 8324EB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EB30 size=144
    let mut pc: u32 = 0x8324EB30;
    'dispatch: loop {
        match pc {
            0x8324EB30 => {
    //   block [0x8324EB30..0x8324EB54)
	// 8324EB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EB3C: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 8324EB40: 4AFD0719  bl 0x8221f258
	ctx.lr = 0x8324EB44;
	sub_8221F258(ctx, base);
	// 8324EB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324EB48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8324EB4C: 419A0008  beq cr6, 0x8324eb54
	if ctx.cr[6].eq {
	pc = 0x8324EB54; continue 'dispatch;
	}
	// 8324EB50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324EB54; continue 'dispatch;
            }
            0x8324EB54 => {
    //   block [0x8324EB54..0x8324EB60)
	// 8324EB54: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324EB58: 41820008  beq 0x8324eb60
	if ctx.cr[0].eq {
	pc = 0x8324EB60; continue 'dispatch;
	}
	// 8324EB5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324EB60; continue 'dispatch;
            }
            0x8324EB60 => {
    //   block [0x8324EB60..0x8324EB6C)
	// 8324EB60: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324EB64: 41820008  beq 0x8324eb6c
	if ctx.cr[0].eq {
	pc = 0x8324EB6C; continue 'dispatch;
	}
	// 8324EB68: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324EB6C; continue 'dispatch;
            }
            0x8324EB6C => {
    //   block [0x8324EB6C..0x8324EBC0)
	// 8324EB6C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324EB70: 99430031  stb r10, 0x31(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(49 as u32), ctx.r[10].u8 ) };
	// 8324EB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8324EB78: 39097D68  addi r8, r9, 0x7d68
	ctx.r[8].s64 = ctx.r[9].s64 + 32104;
	// 8324EB7C: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 8324EB80: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324EB84: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8324EB88: 99630031  stb r11, 0x31(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(49 as u32), ctx.r[11].u8 ) };
	// 8324EB8C: 38678E08  addi r3, r7, -0x71f8
	ctx.r[3].s64 = ctx.r[7].s64 + -29176;
	// 8324EB90: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324EB94: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324EB98: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324EB9C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324EBA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324EBA4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324EBA8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324EBAC: 4BA5B375  bl 0x82ca9f20
	ctx.lr = 0x8324EBB0;
	sub_82CA9F20(ctx, base);
	// 8324EBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EBC0 size=64
    let mut pc: u32 = 0x8324EBC0;
    'dispatch: loop {
        match pc {
            0x8324EBC0 => {
    //   block [0x8324EBC0..0x8324EC00)
	// 8324EBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EBCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EBD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EBD4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324EBD8: 386A7D74  addi r3, r10, 0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + 32116;
	// 8324EBDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EBE0: 4AFDE2F1  bl 0x8222ced0
	ctx.lr = 0x8324EBE4;
	sub_8222CED0(ctx, base);
	// 8324EBE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EBE8: 38698E18  addi r3, r9, -0x71e8
	ctx.r[3].s64 = ctx.r[9].s64 + -29160;
	// 8324EBEC: 4BA5B335  bl 0x82ca9f20
	ctx.lr = 0x8324EBF0;
	sub_82CA9F20(ctx, base);
	// 8324EBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EC00 size=64
    let mut pc: u32 = 0x8324EC00;
    'dispatch: loop {
        match pc {
            0x8324EC00 => {
    //   block [0x8324EC00..0x8324EC40)
	// 8324EC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EC0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EC10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EC14: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324EC18: 386A7D78  addi r3, r10, 0x7d78
	ctx.r[3].s64 = ctx.r[10].s64 + 32120;
	// 8324EC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EC20: 4AFDE2B1  bl 0x8222ced0
	ctx.lr = 0x8324EC24;
	sub_8222CED0(ctx, base);
	// 8324EC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EC28: 38698E28  addi r3, r9, -0x71d8
	ctx.r[3].s64 = ctx.r[9].s64 + -29144;
	// 8324EC2C: 4BA5B2F5  bl 0x82ca9f20
	ctx.lr = 0x8324EC30;
	sub_82CA9F20(ctx, base);
	// 8324EC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EC40 size=64
    let mut pc: u32 = 0x8324EC40;
    'dispatch: loop {
        match pc {
            0x8324EC40 => {
    //   block [0x8324EC40..0x8324EC80)
	// 8324EC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EC50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EC54: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324EC58: 386A7D7C  addi r3, r10, 0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 32124;
	// 8324EC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EC60: 4AFDE271  bl 0x8222ced0
	ctx.lr = 0x8324EC64;
	sub_8222CED0(ctx, base);
	// 8324EC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EC68: 38698E38  addi r3, r9, -0x71c8
	ctx.r[3].s64 = ctx.r[9].s64 + -29128;
	// 8324EC6C: 4BA5B2B5  bl 0x82ca9f20
	ctx.lr = 0x8324EC70;
	sub_82CA9F20(ctx, base);
	// 8324EC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EC80 size=64
    let mut pc: u32 = 0x8324EC80;
    'dispatch: loop {
        match pc {
            0x8324EC80 => {
    //   block [0x8324EC80..0x8324ECC0)
	// 8324EC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EC8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EC90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EC94: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324EC98: 386A7D80  addi r3, r10, 0x7d80
	ctx.r[3].s64 = ctx.r[10].s64 + 32128;
	// 8324EC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ECA0: 4AFDE231  bl 0x8222ced0
	ctx.lr = 0x8324ECA4;
	sub_8222CED0(ctx, base);
	// 8324ECA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ECA8: 38698E48  addi r3, r9, -0x71b8
	ctx.r[3].s64 = ctx.r[9].s64 + -29112;
	// 8324ECAC: 4BA5B275  bl 0x82ca9f20
	ctx.lr = 0x8324ECB0;
	sub_82CA9F20(ctx, base);
	// 8324ECB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ECB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ECB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ECC0 size=64
    let mut pc: u32 = 0x8324ECC0;
    'dispatch: loop {
        match pc {
            0x8324ECC0 => {
    //   block [0x8324ECC0..0x8324ED00)
	// 8324ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ECC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ECCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ECD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ECD4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324ECD8: 386A7D84  addi r3, r10, 0x7d84
	ctx.r[3].s64 = ctx.r[10].s64 + 32132;
	// 8324ECDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ECE0: 4AFDE1F1  bl 0x8222ced0
	ctx.lr = 0x8324ECE4;
	sub_8222CED0(ctx, base);
	// 8324ECE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ECE8: 38698E58  addi r3, r9, -0x71a8
	ctx.r[3].s64 = ctx.r[9].s64 + -29096;
	// 8324ECEC: 4BA5B235  bl 0x82ca9f20
	ctx.lr = 0x8324ECF0;
	sub_82CA9F20(ctx, base);
	// 8324ECF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ECF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ECF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ECFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ED00 size=64
    let mut pc: u32 = 0x8324ED00;
    'dispatch: loop {
        match pc {
            0x8324ED00 => {
    //   block [0x8324ED00..0x8324ED40)
	// 8324ED00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ED04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ED08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ED0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ED10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ED14: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324ED18: 386A7D88  addi r3, r10, 0x7d88
	ctx.r[3].s64 = ctx.r[10].s64 + 32136;
	// 8324ED1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ED20: 4AFDE1B1  bl 0x8222ced0
	ctx.lr = 0x8324ED24;
	sub_8222CED0(ctx, base);
	// 8324ED24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ED28: 38698E68  addi r3, r9, -0x7198
	ctx.r[3].s64 = ctx.r[9].s64 + -29080;
	// 8324ED2C: 4BA5B1F5  bl 0x82ca9f20
	ctx.lr = 0x8324ED30;
	sub_82CA9F20(ctx, base);
	// 8324ED30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ED3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ED40 size=64
    let mut pc: u32 = 0x8324ED40;
    'dispatch: loop {
        match pc {
            0x8324ED40 => {
    //   block [0x8324ED40..0x8324ED80)
	// 8324ED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ED48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ED4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ED50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ED54: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324ED58: 386A7D8C  addi r3, r10, 0x7d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 32140;
	// 8324ED5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ED60: 4AFDE171  bl 0x8222ced0
	ctx.lr = 0x8324ED64;
	sub_8222CED0(ctx, base);
	// 8324ED64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ED68: 38698E78  addi r3, r9, -0x7188
	ctx.r[3].s64 = ctx.r[9].s64 + -29064;
	// 8324ED6C: 4BA5B1B5  bl 0x82ca9f20
	ctx.lr = 0x8324ED70;
	sub_82CA9F20(ctx, base);
	// 8324ED70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ED74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ED78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ED7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ED80 size=64
    let mut pc: u32 = 0x8324ED80;
    'dispatch: loop {
        match pc {
            0x8324ED80 => {
    //   block [0x8324ED80..0x8324EDC0)
	// 8324ED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ED84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ED88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ED8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324ED90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ED94: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324ED98: 386A7D90  addi r3, r10, 0x7d90
	ctx.r[3].s64 = ctx.r[10].s64 + 32144;
	// 8324ED9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EDA0: 4AFDE131  bl 0x8222ced0
	ctx.lr = 0x8324EDA4;
	sub_8222CED0(ctx, base);
	// 8324EDA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EDA8: 38698E88  addi r3, r9, -0x7178
	ctx.r[3].s64 = ctx.r[9].s64 + -29048;
	// 8324EDAC: 4BA5B175  bl 0x82ca9f20
	ctx.lr = 0x8324EDB0;
	sub_82CA9F20(ctx, base);
	// 8324EDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EDC0 size=64
    let mut pc: u32 = 0x8324EDC0;
    'dispatch: loop {
        match pc {
            0x8324EDC0 => {
    //   block [0x8324EDC0..0x8324EE00)
	// 8324EDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EDCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EDD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EDD4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324EDD8: 386A7D94  addi r3, r10, 0x7d94
	ctx.r[3].s64 = ctx.r[10].s64 + 32148;
	// 8324EDDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EDE0: 4AFDE0F1  bl 0x8222ced0
	ctx.lr = 0x8324EDE4;
	sub_8222CED0(ctx, base);
	// 8324EDE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EDE8: 38698E98  addi r3, r9, -0x7168
	ctx.r[3].s64 = ctx.r[9].s64 + -29032;
	// 8324EDEC: 4BA5B135  bl 0x82ca9f20
	ctx.lr = 0x8324EDF0;
	sub_82CA9F20(ctx, base);
	// 8324EDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EE00 size=56
    let mut pc: u32 = 0x8324EE00;
    'dispatch: loop {
        match pc {
            0x8324EE00 => {
    //   block [0x8324EE00..0x8324EE38)
	// 8324EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EE0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EE14: 386B0638  addi r3, r11, 0x638
	ctx.r[3].s64 = ctx.r[11].s64 + 1592;
	// 8324EE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EE1C: 4AFA4F3D  bl 0x821f3d58
	ctx.lr = 0x8324EE20;
	sub_821F3D58(ctx, base);
	// 8324EE20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EE24: 906A7D98  stw r3, 0x7d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32152 as u32), ctx.r[3].u32 ) };
	// 8324EE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EE38 size=56
    let mut pc: u32 = 0x8324EE38;
    'dispatch: loop {
        match pc {
            0x8324EE38 => {
    //   block [0x8324EE38..0x8324EE70)
	// 8324EE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EE44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EE4C: 386B0648  addi r3, r11, 0x648
	ctx.r[3].s64 = ctx.r[11].s64 + 1608;
	// 8324EE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EE54: 4AFA4F05  bl 0x821f3d58
	ctx.lr = 0x8324EE58;
	sub_821F3D58(ctx, base);
	// 8324EE58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EE5C: 906A7D9C  stw r3, 0x7d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32156 as u32), ctx.r[3].u32 ) };
	// 8324EE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EE70 size=56
    let mut pc: u32 = 0x8324EE70;
    'dispatch: loop {
        match pc {
            0x8324EE70 => {
    //   block [0x8324EE70..0x8324EEA8)
	// 8324EE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EE7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EE84: 386B065C  addi r3, r11, 0x65c
	ctx.r[3].s64 = ctx.r[11].s64 + 1628;
	// 8324EE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EE8C: 4AFA4ECD  bl 0x821f3d58
	ctx.lr = 0x8324EE90;
	sub_821F3D58(ctx, base);
	// 8324EE90: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EE94: 906A7DA0  stw r3, 0x7da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32160 as u32), ctx.r[3].u32 ) };
	// 8324EE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EEA8 size=56
    let mut pc: u32 = 0x8324EEA8;
    'dispatch: loop {
        match pc {
            0x8324EEA8 => {
    //   block [0x8324EEA8..0x8324EEE0)
	// 8324EEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EEB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EEBC: 386B0670  addi r3, r11, 0x670
	ctx.r[3].s64 = ctx.r[11].s64 + 1648;
	// 8324EEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EEC4: 4AFA4E95  bl 0x821f3d58
	ctx.lr = 0x8324EEC8;
	sub_821F3D58(ctx, base);
	// 8324EEC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EECC: 906A7DA4  stw r3, 0x7da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32164 as u32), ctx.r[3].u32 ) };
	// 8324EED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EEE0 size=56
    let mut pc: u32 = 0x8324EEE0;
    'dispatch: loop {
        match pc {
            0x8324EEE0 => {
    //   block [0x8324EEE0..0x8324EF18)
	// 8324EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EEEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EEF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EEF4: 386B0688  addi r3, r11, 0x688
	ctx.r[3].s64 = ctx.r[11].s64 + 1672;
	// 8324EEF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EEFC: 4AFA4E5D  bl 0x821f3d58
	ctx.lr = 0x8324EF00;
	sub_821F3D58(ctx, base);
	// 8324EF00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF04: 906A7DA8  stw r3, 0x7da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32168 as u32), ctx.r[3].u32 ) };
	// 8324EF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EF18 size=56
    let mut pc: u32 = 0x8324EF18;
    'dispatch: loop {
        match pc {
            0x8324EF18 => {
    //   block [0x8324EF18..0x8324EF50)
	// 8324EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EF24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EF2C: 386B0698  addi r3, r11, 0x698
	ctx.r[3].s64 = ctx.r[11].s64 + 1688;
	// 8324EF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EF34: 4AFA4E25  bl 0x821f3d58
	ctx.lr = 0x8324EF38;
	sub_821F3D58(ctx, base);
	// 8324EF38: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF3C: 906A7DAC  stw r3, 0x7dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32172 as u32), ctx.r[3].u32 ) };
	// 8324EF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EF50 size=56
    let mut pc: u32 = 0x8324EF50;
    'dispatch: loop {
        match pc {
            0x8324EF50 => {
    //   block [0x8324EF50..0x8324EF88)
	// 8324EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EF5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324EF60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324EF64: 386B06A8  addi r3, r11, 0x6a8
	ctx.r[3].s64 = ctx.r[11].s64 + 1704;
	// 8324EF68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324EF6C: 4AFA4DED  bl 0x821f3d58
	ctx.lr = 0x8324EF70;
	sub_821F3D58(ctx, base);
	// 8324EF70: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF74: 906A7DB0  stw r3, 0x7db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32176 as u32), ctx.r[3].u32 ) };
	// 8324EF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EF88 size=64
    let mut pc: u32 = 0x8324EF88;
    'dispatch: loop {
        match pc {
            0x8324EF88 => {
    //   block [0x8324EF88..0x8324EFC8)
	// 8324EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EF94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EF98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EF9C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324EFA0: 386A7DB4  addi r3, r10, 0x7db4
	ctx.r[3].s64 = ctx.r[10].s64 + 32180;
	// 8324EFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EFA8: 4AFDDF29  bl 0x8222ced0
	ctx.lr = 0x8324EFAC;
	sub_8222CED0(ctx, base);
	// 8324EFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EFB0: 38698EA8  addi r3, r9, -0x7158
	ctx.r[3].s64 = ctx.r[9].s64 + -29016;
	// 8324EFB4: 4BA5AF6D  bl 0x82ca9f20
	ctx.lr = 0x8324EFB8;
	sub_82CA9F20(ctx, base);
	// 8324EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324EFC8 size=64
    let mut pc: u32 = 0x8324EFC8;
    'dispatch: loop {
        match pc {
            0x8324EFC8 => {
    //   block [0x8324EFC8..0x8324F008)
	// 8324EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324EFD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324EFD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324EFDC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324EFE0: 386A7DB8  addi r3, r10, 0x7db8
	ctx.r[3].s64 = ctx.r[10].s64 + 32184;
	// 8324EFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324EFE8: 4AFDDEE9  bl 0x8222ced0
	ctx.lr = 0x8324EFEC;
	sub_8222CED0(ctx, base);
	// 8324EFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324EFF0: 38698EB8  addi r3, r9, -0x7148
	ctx.r[3].s64 = ctx.r[9].s64 + -29000;
	// 8324EFF4: 4BA5AF2D  bl 0x82ca9f20
	ctx.lr = 0x8324EFF8;
	sub_82CA9F20(ctx, base);
	// 8324EFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324EFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F008 size=64
    let mut pc: u32 = 0x8324F008;
    'dispatch: loop {
        match pc {
            0x8324F008 => {
    //   block [0x8324F008..0x8324F048)
	// 8324F008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F014: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324F018: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F01C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324F020: 386A7DBC  addi r3, r10, 0x7dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 32188;
	// 8324F024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F028: 4AFDDEA9  bl 0x8222ced0
	ctx.lr = 0x8324F02C;
	sub_8222CED0(ctx, base);
	// 8324F02C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F030: 38698EC8  addi r3, r9, -0x7138
	ctx.r[3].s64 = ctx.r[9].s64 + -28984;
	// 8324F034: 4BA5AEED  bl 0x82ca9f20
	ctx.lr = 0x8324F038;
	sub_82CA9F20(ctx, base);
	// 8324F038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F048 size=64
    let mut pc: u32 = 0x8324F048;
    'dispatch: loop {
        match pc {
            0x8324F048 => {
    //   block [0x8324F048..0x8324F088)
	// 8324F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F054: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F058: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F05C: 388B06D4  addi r4, r11, 0x6d4
	ctx.r[4].s64 = ctx.r[11].s64 + 1748;
	// 8324F060: 386A7DC0  addi r3, r10, 0x7dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 32192;
	// 8324F064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F068: 4AFDDE69  bl 0x8222ced0
	ctx.lr = 0x8324F06C;
	sub_8222CED0(ctx, base);
	// 8324F06C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F070: 38698ED8  addi r3, r9, -0x7128
	ctx.r[3].s64 = ctx.r[9].s64 + -28968;
	// 8324F074: 4BA5AEAD  bl 0x82ca9f20
	ctx.lr = 0x8324F078;
	sub_82CA9F20(ctx, base);
	// 8324F078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F088 size=64
    let mut pc: u32 = 0x8324F088;
    'dispatch: loop {
        match pc {
            0x8324F088 => {
    //   block [0x8324F088..0x8324F0C8)
	// 8324F088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F094: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F098: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F09C: 388B06D8  addi r4, r11, 0x6d8
	ctx.r[4].s64 = ctx.r[11].s64 + 1752;
	// 8324F0A0: 386A7DC4  addi r3, r10, 0x7dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32196;
	// 8324F0A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F0A8: 4AFDDE29  bl 0x8222ced0
	ctx.lr = 0x8324F0AC;
	sub_8222CED0(ctx, base);
	// 8324F0AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F0B0: 38698EE8  addi r3, r9, -0x7118
	ctx.r[3].s64 = ctx.r[9].s64 + -28952;
	// 8324F0B4: 4BA5AE6D  bl 0x82ca9f20
	ctx.lr = 0x8324F0B8;
	sub_82CA9F20(ctx, base);
	// 8324F0B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F0C8 size=64
    let mut pc: u32 = 0x8324F0C8;
    'dispatch: loop {
        match pc {
            0x8324F0C8 => {
    //   block [0x8324F0C8..0x8324F108)
	// 8324F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F0D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F0D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F0D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F0DC: 388B06DC  addi r4, r11, 0x6dc
	ctx.r[4].s64 = ctx.r[11].s64 + 1756;
	// 8324F0E0: 386A7DD0  addi r3, r10, 0x7dd0
	ctx.r[3].s64 = ctx.r[10].s64 + 32208;
	// 8324F0E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F0E8: 4AFDDDE9  bl 0x8222ced0
	ctx.lr = 0x8324F0EC;
	sub_8222CED0(ctx, base);
	// 8324F0EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F0F0: 38698EF8  addi r3, r9, -0x7108
	ctx.r[3].s64 = ctx.r[9].s64 + -28936;
	// 8324F0F4: 4BA5AE2D  bl 0x82ca9f20
	ctx.lr = 0x8324F0F8;
	sub_82CA9F20(ctx, base);
	// 8324F0F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F108 size=64
    let mut pc: u32 = 0x8324F108;
    'dispatch: loop {
        match pc {
            0x8324F108 => {
    //   block [0x8324F108..0x8324F148)
	// 8324F108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F114: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F118: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F11C: 388B06F4  addi r4, r11, 0x6f4
	ctx.r[4].s64 = ctx.r[11].s64 + 1780;
	// 8324F120: 386A7DD4  addi r3, r10, 0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32212;
	// 8324F124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F128: 4AFDDDA9  bl 0x8222ced0
	ctx.lr = 0x8324F12C;
	sub_8222CED0(ctx, base);
	// 8324F12C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F130: 38698F08  addi r3, r9, -0x70f8
	ctx.r[3].s64 = ctx.r[9].s64 + -28920;
	// 8324F134: 4BA5ADED  bl 0x82ca9f20
	ctx.lr = 0x8324F138;
	sub_82CA9F20(ctx, base);
	// 8324F138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F148 size=64
    let mut pc: u32 = 0x8324F148;
    'dispatch: loop {
        match pc {
            0x8324F148 => {
    //   block [0x8324F148..0x8324F188)
	// 8324F148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F154: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F158: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F15C: 388B070C  addi r4, r11, 0x70c
	ctx.r[4].s64 = ctx.r[11].s64 + 1804;
	// 8324F160: 386A7DD8  addi r3, r10, 0x7dd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32216;
	// 8324F164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F168: 4AFDDD69  bl 0x8222ced0
	ctx.lr = 0x8324F16C;
	sub_8222CED0(ctx, base);
	// 8324F16C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F170: 38698F18  addi r3, r9, -0x70e8
	ctx.r[3].s64 = ctx.r[9].s64 + -28904;
	// 8324F174: 4BA5ADAD  bl 0x82ca9f20
	ctx.lr = 0x8324F178;
	sub_82CA9F20(ctx, base);
	// 8324F178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F188 size=64
    let mut pc: u32 = 0x8324F188;
    'dispatch: loop {
        match pc {
            0x8324F188 => {
    //   block [0x8324F188..0x8324F1C8)
	// 8324F188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F194: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F198: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F19C: 388B0724  addi r4, r11, 0x724
	ctx.r[4].s64 = ctx.r[11].s64 + 1828;
	// 8324F1A0: 386A7DDC  addi r3, r10, 0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 32220;
	// 8324F1A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F1A8: 4AFDDD29  bl 0x8222ced0
	ctx.lr = 0x8324F1AC;
	sub_8222CED0(ctx, base);
	// 8324F1AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F1B0: 38698F28  addi r3, r9, -0x70d8
	ctx.r[3].s64 = ctx.r[9].s64 + -28888;
	// 8324F1B4: 4BA5AD6D  bl 0x82ca9f20
	ctx.lr = 0x8324F1B8;
	sub_82CA9F20(ctx, base);
	// 8324F1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F1C8 size=64
    let mut pc: u32 = 0x8324F1C8;
    'dispatch: loop {
        match pc {
            0x8324F1C8 => {
    //   block [0x8324F1C8..0x8324F208)
	// 8324F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F1D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F1D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F1DC: 388B073C  addi r4, r11, 0x73c
	ctx.r[4].s64 = ctx.r[11].s64 + 1852;
	// 8324F1E0: 386A7DE0  addi r3, r10, 0x7de0
	ctx.r[3].s64 = ctx.r[10].s64 + 32224;
	// 8324F1E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F1E8: 4AFDDCE9  bl 0x8222ced0
	ctx.lr = 0x8324F1EC;
	sub_8222CED0(ctx, base);
	// 8324F1EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F1F0: 38698F38  addi r3, r9, -0x70c8
	ctx.r[3].s64 = ctx.r[9].s64 + -28872;
	// 8324F1F4: 4BA5AD2D  bl 0x82ca9f20
	ctx.lr = 0x8324F1F8;
	sub_82CA9F20(ctx, base);
	// 8324F1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F208 size=64
    let mut pc: u32 = 0x8324F208;
    'dispatch: loop {
        match pc {
            0x8324F208 => {
    //   block [0x8324F208..0x8324F248)
	// 8324F208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F214: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F218: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F21C: 388B0750  addi r4, r11, 0x750
	ctx.r[4].s64 = ctx.r[11].s64 + 1872;
	// 8324F220: 386A7DE4  addi r3, r10, 0x7de4
	ctx.r[3].s64 = ctx.r[10].s64 + 32228;
	// 8324F224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F228: 4AFDDCA9  bl 0x8222ced0
	ctx.lr = 0x8324F22C;
	sub_8222CED0(ctx, base);
	// 8324F22C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F230: 38698F48  addi r3, r9, -0x70b8
	ctx.r[3].s64 = ctx.r[9].s64 + -28856;
	// 8324F234: 4BA5ACED  bl 0x82ca9f20
	ctx.lr = 0x8324F238;
	sub_82CA9F20(ctx, base);
	// 8324F238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F248 size=64
    let mut pc: u32 = 0x8324F248;
    'dispatch: loop {
        match pc {
            0x8324F248 => {
    //   block [0x8324F248..0x8324F288)
	// 8324F248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F254: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F258: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F25C: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324F260: 386A7DE8  addi r3, r10, 0x7de8
	ctx.r[3].s64 = ctx.r[10].s64 + 32232;
	// 8324F264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F268: 4AFDDC69  bl 0x8222ced0
	ctx.lr = 0x8324F26C;
	sub_8222CED0(ctx, base);
	// 8324F26C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F270: 38698F58  addi r3, r9, -0x70a8
	ctx.r[3].s64 = ctx.r[9].s64 + -28840;
	// 8324F274: 4BA5ACAD  bl 0x82ca9f20
	ctx.lr = 0x8324F278;
	sub_82CA9F20(ctx, base);
	// 8324F278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F288 size=64
    let mut pc: u32 = 0x8324F288;
    'dispatch: loop {
        match pc {
            0x8324F288 => {
    //   block [0x8324F288..0x8324F2C8)
	// 8324F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F294: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F298: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F29C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324F2A0: 386A7DEC  addi r3, r10, 0x7dec
	ctx.r[3].s64 = ctx.r[10].s64 + 32236;
	// 8324F2A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F2A8: 4AFDDC29  bl 0x8222ced0
	ctx.lr = 0x8324F2AC;
	sub_8222CED0(ctx, base);
	// 8324F2AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F2B0: 38698F68  addi r3, r9, -0x7098
	ctx.r[3].s64 = ctx.r[9].s64 + -28824;
	// 8324F2B4: 4BA5AC6D  bl 0x82ca9f20
	ctx.lr = 0x8324F2B8;
	sub_82CA9F20(ctx, base);
	// 8324F2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F2C8 size=64
    let mut pc: u32 = 0x8324F2C8;
    'dispatch: loop {
        match pc {
            0x8324F2C8 => {
    //   block [0x8324F2C8..0x8324F308)
	// 8324F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F2D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F2D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F2DC: 388B078C  addi r4, r11, 0x78c
	ctx.r[4].s64 = ctx.r[11].s64 + 1932;
	// 8324F2E0: 386A7DF0  addi r3, r10, 0x7df0
	ctx.r[3].s64 = ctx.r[10].s64 + 32240;
	// 8324F2E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F2E8: 4AFDDBE9  bl 0x8222ced0
	ctx.lr = 0x8324F2EC;
	sub_8222CED0(ctx, base);
	// 8324F2EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F2F0: 38698F78  addi r3, r9, -0x7088
	ctx.r[3].s64 = ctx.r[9].s64 + -28808;
	// 8324F2F4: 4BA5AC2D  bl 0x82ca9f20
	ctx.lr = 0x8324F2F8;
	sub_82CA9F20(ctx, base);
	// 8324F2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F308 size=64
    let mut pc: u32 = 0x8324F308;
    'dispatch: loop {
        match pc {
            0x8324F308 => {
    //   block [0x8324F308..0x8324F348)
	// 8324F308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F314: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F318: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F31C: 388B07A4  addi r4, r11, 0x7a4
	ctx.r[4].s64 = ctx.r[11].s64 + 1956;
	// 8324F320: 386A7DF4  addi r3, r10, 0x7df4
	ctx.r[3].s64 = ctx.r[10].s64 + 32244;
	// 8324F324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F328: 4AFDDBA9  bl 0x8222ced0
	ctx.lr = 0x8324F32C;
	sub_8222CED0(ctx, base);
	// 8324F32C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F330: 38698F88  addi r3, r9, -0x7078
	ctx.r[3].s64 = ctx.r[9].s64 + -28792;
	// 8324F334: 4BA5ABED  bl 0x82ca9f20
	ctx.lr = 0x8324F338;
	sub_82CA9F20(ctx, base);
	// 8324F338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F348 size=64
    let mut pc: u32 = 0x8324F348;
    'dispatch: loop {
        match pc {
            0x8324F348 => {
    //   block [0x8324F348..0x8324F388)
	// 8324F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F354: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F358: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F35C: 388B07BC  addi r4, r11, 0x7bc
	ctx.r[4].s64 = ctx.r[11].s64 + 1980;
	// 8324F360: 386A7DF8  addi r3, r10, 0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + 32248;
	// 8324F364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F368: 4AFDDB69  bl 0x8222ced0
	ctx.lr = 0x8324F36C;
	sub_8222CED0(ctx, base);
	// 8324F36C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F370: 38698F98  addi r3, r9, -0x7068
	ctx.r[3].s64 = ctx.r[9].s64 + -28776;
	// 8324F374: 4BA5ABAD  bl 0x82ca9f20
	ctx.lr = 0x8324F378;
	sub_82CA9F20(ctx, base);
	// 8324F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F388 size=64
    let mut pc: u32 = 0x8324F388;
    'dispatch: loop {
        match pc {
            0x8324F388 => {
    //   block [0x8324F388..0x8324F3C8)
	// 8324F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F394: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F398: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F39C: 388B07D0  addi r4, r11, 0x7d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2000;
	// 8324F3A0: 386A7DFC  addi r3, r10, 0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 32252;
	// 8324F3A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F3A8: 4AFDDB29  bl 0x8222ced0
	ctx.lr = 0x8324F3AC;
	sub_8222CED0(ctx, base);
	// 8324F3AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F3B0: 38698FA8  addi r3, r9, -0x7058
	ctx.r[3].s64 = ctx.r[9].s64 + -28760;
	// 8324F3B4: 4BA5AB6D  bl 0x82ca9f20
	ctx.lr = 0x8324F3B8;
	sub_82CA9F20(ctx, base);
	// 8324F3B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F3C8 size=64
    let mut pc: u32 = 0x8324F3C8;
    'dispatch: loop {
        match pc {
            0x8324F3C8 => {
    //   block [0x8324F3C8..0x8324F408)
	// 8324F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F3D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F3D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F3D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F3DC: 388B07EC  addi r4, r11, 0x7ec
	ctx.r[4].s64 = ctx.r[11].s64 + 2028;
	// 8324F3E0: 386A7E00  addi r3, r10, 0x7e00
	ctx.r[3].s64 = ctx.r[10].s64 + 32256;
	// 8324F3E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F3E8: 4AFDDAE9  bl 0x8222ced0
	ctx.lr = 0x8324F3EC;
	sub_8222CED0(ctx, base);
	// 8324F3EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F3F0: 38698FB8  addi r3, r9, -0x7048
	ctx.r[3].s64 = ctx.r[9].s64 + -28744;
	// 8324F3F4: 4BA5AB2D  bl 0x82ca9f20
	ctx.lr = 0x8324F3F8;
	sub_82CA9F20(ctx, base);
	// 8324F3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F408 size=64
    let mut pc: u32 = 0x8324F408;
    'dispatch: loop {
        match pc {
            0x8324F408 => {
    //   block [0x8324F408..0x8324F448)
	// 8324F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F414: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F41C: 388B0804  addi r4, r11, 0x804
	ctx.r[4].s64 = ctx.r[11].s64 + 2052;
	// 8324F420: 386A7E04  addi r3, r10, 0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + 32260;
	// 8324F424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F428: 4AFDDAA9  bl 0x8222ced0
	ctx.lr = 0x8324F42C;
	sub_8222CED0(ctx, base);
	// 8324F42C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F430: 38698FC8  addi r3, r9, -0x7038
	ctx.r[3].s64 = ctx.r[9].s64 + -28728;
	// 8324F434: 4BA5AAED  bl 0x82ca9f20
	ctx.lr = 0x8324F438;
	sub_82CA9F20(ctx, base);
	// 8324F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F448 size=64
    let mut pc: u32 = 0x8324F448;
    'dispatch: loop {
        match pc {
            0x8324F448 => {
    //   block [0x8324F448..0x8324F488)
	// 8324F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F454: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F45C: 388B0818  addi r4, r11, 0x818
	ctx.r[4].s64 = ctx.r[11].s64 + 2072;
	// 8324F460: 386A7E08  addi r3, r10, 0x7e08
	ctx.r[3].s64 = ctx.r[10].s64 + 32264;
	// 8324F464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F468: 4AFDDA69  bl 0x8222ced0
	ctx.lr = 0x8324F46C;
	sub_8222CED0(ctx, base);
	// 8324F46C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F470: 38698FD8  addi r3, r9, -0x7028
	ctx.r[3].s64 = ctx.r[9].s64 + -28712;
	// 8324F474: 4BA5AAAD  bl 0x82ca9f20
	ctx.lr = 0x8324F478;
	sub_82CA9F20(ctx, base);
	// 8324F478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F488 size=64
    let mut pc: u32 = 0x8324F488;
    'dispatch: loop {
        match pc {
            0x8324F488 => {
    //   block [0x8324F488..0x8324F4C8)
	// 8324F488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F494: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F49C: 388B082C  addi r4, r11, 0x82c
	ctx.r[4].s64 = ctx.r[11].s64 + 2092;
	// 8324F4A0: 386A7E0C  addi r3, r10, 0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32268;
	// 8324F4A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F4A8: 4AFDDA29  bl 0x8222ced0
	ctx.lr = 0x8324F4AC;
	sub_8222CED0(ctx, base);
	// 8324F4AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F4B0: 38698FE8  addi r3, r9, -0x7018
	ctx.r[3].s64 = ctx.r[9].s64 + -28696;
	// 8324F4B4: 4BA5AA6D  bl 0x82ca9f20
	ctx.lr = 0x8324F4B8;
	sub_82CA9F20(ctx, base);
	// 8324F4B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F4C8 size=64
    let mut pc: u32 = 0x8324F4C8;
    'dispatch: loop {
        match pc {
            0x8324F4C8 => {
    //   block [0x8324F4C8..0x8324F508)
	// 8324F4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F4D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F4D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F4D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F4DC: 388B0840  addi r4, r11, 0x840
	ctx.r[4].s64 = ctx.r[11].s64 + 2112;
	// 8324F4E0: 386A7E10  addi r3, r10, 0x7e10
	ctx.r[3].s64 = ctx.r[10].s64 + 32272;
	// 8324F4E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F4E8: 4AFDD9E9  bl 0x8222ced0
	ctx.lr = 0x8324F4EC;
	sub_8222CED0(ctx, base);
	// 8324F4EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F4F0: 38698FF8  addi r3, r9, -0x7008
	ctx.r[3].s64 = ctx.r[9].s64 + -28680;
	// 8324F4F4: 4BA5AA2D  bl 0x82ca9f20
	ctx.lr = 0x8324F4F8;
	sub_82CA9F20(ctx, base);
	// 8324F4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F508 size=64
    let mut pc: u32 = 0x8324F508;
    'dispatch: loop {
        match pc {
            0x8324F508 => {
    //   block [0x8324F508..0x8324F548)
	// 8324F508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F514: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F51C: 388B0854  addi r4, r11, 0x854
	ctx.r[4].s64 = ctx.r[11].s64 + 2132;
	// 8324F520: 386A7E14  addi r3, r10, 0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + 32276;
	// 8324F524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F528: 4AFDD9A9  bl 0x8222ced0
	ctx.lr = 0x8324F52C;
	sub_8222CED0(ctx, base);
	// 8324F52C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F530: 38699008  addi r3, r9, -0x6ff8
	ctx.r[3].s64 = ctx.r[9].s64 + -28664;
	// 8324F534: 4BA5A9ED  bl 0x82ca9f20
	ctx.lr = 0x8324F538;
	sub_82CA9F20(ctx, base);
	// 8324F538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F548 size=64
    let mut pc: u32 = 0x8324F548;
    'dispatch: loop {
        match pc {
            0x8324F548 => {
    //   block [0x8324F548..0x8324F588)
	// 8324F548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F554: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F558: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F55C: 388B0864  addi r4, r11, 0x864
	ctx.r[4].s64 = ctx.r[11].s64 + 2148;
	// 8324F560: 386A7E18  addi r3, r10, 0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + 32280;
	// 8324F564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F568: 4AFDD969  bl 0x8222ced0
	ctx.lr = 0x8324F56C;
	sub_8222CED0(ctx, base);
	// 8324F56C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F570: 38699018  addi r3, r9, -0x6fe8
	ctx.r[3].s64 = ctx.r[9].s64 + -28648;
	// 8324F574: 4BA5A9AD  bl 0x82ca9f20
	ctx.lr = 0x8324F578;
	sub_82CA9F20(ctx, base);
	// 8324F578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F57C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F588 size=64
    let mut pc: u32 = 0x8324F588;
    'dispatch: loop {
        match pc {
            0x8324F588 => {
    //   block [0x8324F588..0x8324F5C8)
	// 8324F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F594: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F598: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F59C: 388B0874  addi r4, r11, 0x874
	ctx.r[4].s64 = ctx.r[11].s64 + 2164;
	// 8324F5A0: 386A7E1C  addi r3, r10, 0x7e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 32284;
	// 8324F5A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F5A8: 4AFDD929  bl 0x8222ced0
	ctx.lr = 0x8324F5AC;
	sub_8222CED0(ctx, base);
	// 8324F5AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F5B0: 38699028  addi r3, r9, -0x6fd8
	ctx.r[3].s64 = ctx.r[9].s64 + -28632;
	// 8324F5B4: 4BA5A96D  bl 0x82ca9f20
	ctx.lr = 0x8324F5B8;
	sub_82CA9F20(ctx, base);
	// 8324F5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F5C8 size=64
    let mut pc: u32 = 0x8324F5C8;
    'dispatch: loop {
        match pc {
            0x8324F5C8 => {
    //   block [0x8324F5C8..0x8324F608)
	// 8324F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F5D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F5D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F5DC: 388B0888  addi r4, r11, 0x888
	ctx.r[4].s64 = ctx.r[11].s64 + 2184;
	// 8324F5E0: 386A7E20  addi r3, r10, 0x7e20
	ctx.r[3].s64 = ctx.r[10].s64 + 32288;
	// 8324F5E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F5E8: 4AFDD8E9  bl 0x8222ced0
	ctx.lr = 0x8324F5EC;
	sub_8222CED0(ctx, base);
	// 8324F5EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F5F0: 38699038  addi r3, r9, -0x6fc8
	ctx.r[3].s64 = ctx.r[9].s64 + -28616;
	// 8324F5F4: 4BA5A92D  bl 0x82ca9f20
	ctx.lr = 0x8324F5F8;
	sub_82CA9F20(ctx, base);
	// 8324F5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F608 size=64
    let mut pc: u32 = 0x8324F608;
    'dispatch: loop {
        match pc {
            0x8324F608 => {
    //   block [0x8324F608..0x8324F648)
	// 8324F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F614: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F618: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F61C: 388B08A8  addi r4, r11, 0x8a8
	ctx.r[4].s64 = ctx.r[11].s64 + 2216;
	// 8324F620: 386A7E24  addi r3, r10, 0x7e24
	ctx.r[3].s64 = ctx.r[10].s64 + 32292;
	// 8324F624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F628: 4AFDD8A9  bl 0x8222ced0
	ctx.lr = 0x8324F62C;
	sub_8222CED0(ctx, base);
	// 8324F62C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F630: 38699048  addi r3, r9, -0x6fb8
	ctx.r[3].s64 = ctx.r[9].s64 + -28600;
	// 8324F634: 4BA5A8ED  bl 0x82ca9f20
	ctx.lr = 0x8324F638;
	sub_82CA9F20(ctx, base);
	// 8324F638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F648 size=64
    let mut pc: u32 = 0x8324F648;
    'dispatch: loop {
        match pc {
            0x8324F648 => {
    //   block [0x8324F648..0x8324F688)
	// 8324F648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F654: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F658: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F65C: 388B08BC  addi r4, r11, 0x8bc
	ctx.r[4].s64 = ctx.r[11].s64 + 2236;
	// 8324F660: 386A7E28  addi r3, r10, 0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + 32296;
	// 8324F664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F668: 4AFDD869  bl 0x8222ced0
	ctx.lr = 0x8324F66C;
	sub_8222CED0(ctx, base);
	// 8324F66C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F670: 38699058  addi r3, r9, -0x6fa8
	ctx.r[3].s64 = ctx.r[9].s64 + -28584;
	// 8324F674: 4BA5A8AD  bl 0x82ca9f20
	ctx.lr = 0x8324F678;
	sub_82CA9F20(ctx, base);
	// 8324F678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F688 size=64
    let mut pc: u32 = 0x8324F688;
    'dispatch: loop {
        match pc {
            0x8324F688 => {
    //   block [0x8324F688..0x8324F6C8)
	// 8324F688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F694: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F698: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F69C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 8324F6A0: 386A7E2C  addi r3, r10, 0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 32300;
	// 8324F6A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F6A8: 4AFDD829  bl 0x8222ced0
	ctx.lr = 0x8324F6AC;
	sub_8222CED0(ctx, base);
	// 8324F6AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F6B0: 38699068  addi r3, r9, -0x6f98
	ctx.r[3].s64 = ctx.r[9].s64 + -28568;
	// 8324F6B4: 4BA5A86D  bl 0x82ca9f20
	ctx.lr = 0x8324F6B8;
	sub_82CA9F20(ctx, base);
	// 8324F6B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F6C8 size=64
    let mut pc: u32 = 0x8324F6C8;
    'dispatch: loop {
        match pc {
            0x8324F6C8 => {
    //   block [0x8324F6C8..0x8324F708)
	// 8324F6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F6D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F6D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F6DC: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 8324F6E0: 386A7E30  addi r3, r10, 0x7e30
	ctx.r[3].s64 = ctx.r[10].s64 + 32304;
	// 8324F6E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F6E8: 4AFDD7E9  bl 0x8222ced0
	ctx.lr = 0x8324F6EC;
	sub_8222CED0(ctx, base);
	// 8324F6EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F6F0: 38699078  addi r3, r9, -0x6f88
	ctx.r[3].s64 = ctx.r[9].s64 + -28552;
	// 8324F6F4: 4BA5A82D  bl 0x82ca9f20
	ctx.lr = 0x8324F6F8;
	sub_82CA9F20(ctx, base);
	// 8324F6F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F708 size=64
    let mut pc: u32 = 0x8324F708;
    'dispatch: loop {
        match pc {
            0x8324F708 => {
    //   block [0x8324F708..0x8324F748)
	// 8324F708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F714: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F718: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F71C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 8324F720: 386A7E34  addi r3, r10, 0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + 32308;
	// 8324F724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F728: 4AFDD7A9  bl 0x8222ced0
	ctx.lr = 0x8324F72C;
	sub_8222CED0(ctx, base);
	// 8324F72C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F730: 38699088  addi r3, r9, -0x6f78
	ctx.r[3].s64 = ctx.r[9].s64 + -28536;
	// 8324F734: 4BA5A7ED  bl 0x82ca9f20
	ctx.lr = 0x8324F738;
	sub_82CA9F20(ctx, base);
	// 8324F738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F748 size=64
    let mut pc: u32 = 0x8324F748;
    'dispatch: loop {
        match pc {
            0x8324F748 => {
    //   block [0x8324F748..0x8324F788)
	// 8324F748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F754: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F758: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F75C: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 8324F760: 386A7E38  addi r3, r10, 0x7e38
	ctx.r[3].s64 = ctx.r[10].s64 + 32312;
	// 8324F764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F768: 4AFDD769  bl 0x8222ced0
	ctx.lr = 0x8324F76C;
	sub_8222CED0(ctx, base);
	// 8324F76C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F770: 38699098  addi r3, r9, -0x6f68
	ctx.r[3].s64 = ctx.r[9].s64 + -28520;
	// 8324F774: 4BA5A7AD  bl 0x82ca9f20
	ctx.lr = 0x8324F778;
	sub_82CA9F20(ctx, base);
	// 8324F778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F788 size=64
    let mut pc: u32 = 0x8324F788;
    'dispatch: loop {
        match pc {
            0x8324F788 => {
    //   block [0x8324F788..0x8324F7C8)
	// 8324F788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F794: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F798: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F79C: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324F7A0: 386A7E3C  addi r3, r10, 0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 32316;
	// 8324F7A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F7A8: 4AFDD729  bl 0x8222ced0
	ctx.lr = 0x8324F7AC;
	sub_8222CED0(ctx, base);
	// 8324F7AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F7B0: 386990A8  addi r3, r9, -0x6f58
	ctx.r[3].s64 = ctx.r[9].s64 + -28504;
	// 8324F7B4: 4BA5A76D  bl 0x82ca9f20
	ctx.lr = 0x8324F7B8;
	sub_82CA9F20(ctx, base);
	// 8324F7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F7C8 size=64
    let mut pc: u32 = 0x8324F7C8;
    'dispatch: loop {
        match pc {
            0x8324F7C8 => {
    //   block [0x8324F7C8..0x8324F808)
	// 8324F7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F7D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F7D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F7DC: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324F7E0: 386A7E40  addi r3, r10, 0x7e40
	ctx.r[3].s64 = ctx.r[10].s64 + 32320;
	// 8324F7E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F7E8: 4AFDD6E9  bl 0x8222ced0
	ctx.lr = 0x8324F7EC;
	sub_8222CED0(ctx, base);
	// 8324F7EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F7F0: 386990B8  addi r3, r9, -0x6f48
	ctx.r[3].s64 = ctx.r[9].s64 + -28488;
	// 8324F7F4: 4BA5A72D  bl 0x82ca9f20
	ctx.lr = 0x8324F7F8;
	sub_82CA9F20(ctx, base);
	// 8324F7F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F808 size=64
    let mut pc: u32 = 0x8324F808;
    'dispatch: loop {
        match pc {
            0x8324F808 => {
    //   block [0x8324F808..0x8324F848)
	// 8324F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F814: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F818: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F81C: 388B0900  addi r4, r11, 0x900
	ctx.r[4].s64 = ctx.r[11].s64 + 2304;
	// 8324F820: 386A7E44  addi r3, r10, 0x7e44
	ctx.r[3].s64 = ctx.r[10].s64 + 32324;
	// 8324F824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F828: 4AFDD6A9  bl 0x8222ced0
	ctx.lr = 0x8324F82C;
	sub_8222CED0(ctx, base);
	// 8324F82C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F830: 386990C8  addi r3, r9, -0x6f38
	ctx.r[3].s64 = ctx.r[9].s64 + -28472;
	// 8324F834: 4BA5A6ED  bl 0x82ca9f20
	ctx.lr = 0x8324F838;
	sub_82CA9F20(ctx, base);
	// 8324F838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F848 size=64
    let mut pc: u32 = 0x8324F848;
    'dispatch: loop {
        match pc {
            0x8324F848 => {
    //   block [0x8324F848..0x8324F888)
	// 8324F848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F854: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F858: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F85C: 388B0914  addi r4, r11, 0x914
	ctx.r[4].s64 = ctx.r[11].s64 + 2324;
	// 8324F860: 386A7E48  addi r3, r10, 0x7e48
	ctx.r[3].s64 = ctx.r[10].s64 + 32328;
	// 8324F864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F868: 4AFDD669  bl 0x8222ced0
	ctx.lr = 0x8324F86C;
	sub_8222CED0(ctx, base);
	// 8324F86C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F870: 386990D8  addi r3, r9, -0x6f28
	ctx.r[3].s64 = ctx.r[9].s64 + -28456;
	// 8324F874: 4BA5A6AD  bl 0x82ca9f20
	ctx.lr = 0x8324F878;
	sub_82CA9F20(ctx, base);
	// 8324F878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F888 size=64
    let mut pc: u32 = 0x8324F888;
    'dispatch: loop {
        match pc {
            0x8324F888 => {
    //   block [0x8324F888..0x8324F8C8)
	// 8324F888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F894: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F898: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F89C: 388B092C  addi r4, r11, 0x92c
	ctx.r[4].s64 = ctx.r[11].s64 + 2348;
	// 8324F8A0: 386A7E4C  addi r3, r10, 0x7e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32332;
	// 8324F8A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F8A8: 4AFDD629  bl 0x8222ced0
	ctx.lr = 0x8324F8AC;
	sub_8222CED0(ctx, base);
	// 8324F8AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F8B0: 386990E8  addi r3, r9, -0x6f18
	ctx.r[3].s64 = ctx.r[9].s64 + -28440;
	// 8324F8B4: 4BA5A66D  bl 0x82ca9f20
	ctx.lr = 0x8324F8B8;
	sub_82CA9F20(ctx, base);
	// 8324F8B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F8C8 size=64
    let mut pc: u32 = 0x8324F8C8;
    'dispatch: loop {
        match pc {
            0x8324F8C8 => {
    //   block [0x8324F8C8..0x8324F908)
	// 8324F8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F8D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F8D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F8D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F8DC: 388B0944  addi r4, r11, 0x944
	ctx.r[4].s64 = ctx.r[11].s64 + 2372;
	// 8324F8E0: 386A7E50  addi r3, r10, 0x7e50
	ctx.r[3].s64 = ctx.r[10].s64 + 32336;
	// 8324F8E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F8E8: 4AFDD5E9  bl 0x8222ced0
	ctx.lr = 0x8324F8EC;
	sub_8222CED0(ctx, base);
	// 8324F8EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F8F0: 386990F8  addi r3, r9, -0x6f08
	ctx.r[3].s64 = ctx.r[9].s64 + -28424;
	// 8324F8F4: 4BA5A62D  bl 0x82ca9f20
	ctx.lr = 0x8324F8F8;
	sub_82CA9F20(ctx, base);
	// 8324F8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F908 size=64
    let mut pc: u32 = 0x8324F908;
    'dispatch: loop {
        match pc {
            0x8324F908 => {
    //   block [0x8324F908..0x8324F948)
	// 8324F908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F914: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F918: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F91C: 388B095C  addi r4, r11, 0x95c
	ctx.r[4].s64 = ctx.r[11].s64 + 2396;
	// 8324F920: 386A7E54  addi r3, r10, 0x7e54
	ctx.r[3].s64 = ctx.r[10].s64 + 32340;
	// 8324F924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F928: 4AFDD5A9  bl 0x8222ced0
	ctx.lr = 0x8324F92C;
	sub_8222CED0(ctx, base);
	// 8324F92C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F930: 38699108  addi r3, r9, -0x6ef8
	ctx.r[3].s64 = ctx.r[9].s64 + -28408;
	// 8324F934: 4BA5A5ED  bl 0x82ca9f20
	ctx.lr = 0x8324F938;
	sub_82CA9F20(ctx, base);
	// 8324F938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F948 size=64
    let mut pc: u32 = 0x8324F948;
    'dispatch: loop {
        match pc {
            0x8324F948 => {
    //   block [0x8324F948..0x8324F988)
	// 8324F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F954: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F958: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F95C: 388B096C  addi r4, r11, 0x96c
	ctx.r[4].s64 = ctx.r[11].s64 + 2412;
	// 8324F960: 386A7E58  addi r3, r10, 0x7e58
	ctx.r[3].s64 = ctx.r[10].s64 + 32344;
	// 8324F964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F968: 4AFDD569  bl 0x8222ced0
	ctx.lr = 0x8324F96C;
	sub_8222CED0(ctx, base);
	// 8324F96C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F970: 38699118  addi r3, r9, -0x6ee8
	ctx.r[3].s64 = ctx.r[9].s64 + -28392;
	// 8324F974: 4BA5A5AD  bl 0x82ca9f20
	ctx.lr = 0x8324F978;
	sub_82CA9F20(ctx, base);
	// 8324F978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F988 size=64
    let mut pc: u32 = 0x8324F988;
    'dispatch: loop {
        match pc {
            0x8324F988 => {
    //   block [0x8324F988..0x8324F9C8)
	// 8324F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F994: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F998: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F99C: 388B0988  addi r4, r11, 0x988
	ctx.r[4].s64 = ctx.r[11].s64 + 2440;
	// 8324F9A0: 386A7E5C  addi r3, r10, 0x7e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32348;
	// 8324F9A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F9A8: 4AFDD529  bl 0x8222ced0
	ctx.lr = 0x8324F9AC;
	sub_8222CED0(ctx, base);
	// 8324F9AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F9B0: 38699128  addi r3, r9, -0x6ed8
	ctx.r[3].s64 = ctx.r[9].s64 + -28376;
	// 8324F9B4: 4BA5A56D  bl 0x82ca9f20
	ctx.lr = 0x8324F9B8;
	sub_82CA9F20(ctx, base);
	// 8324F9B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F9C8 size=64
    let mut pc: u32 = 0x8324F9C8;
    'dispatch: loop {
        match pc {
            0x8324F9C8 => {
    //   block [0x8324F9C8..0x8324FA08)
	// 8324F9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F9D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F9D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F9D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F9DC: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324F9E0: 386A7E60  addi r3, r10, 0x7e60
	ctx.r[3].s64 = ctx.r[10].s64 + 32352;
	// 8324F9E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F9E8: 4AFDD4E9  bl 0x8222ced0
	ctx.lr = 0x8324F9EC;
	sub_8222CED0(ctx, base);
	// 8324F9EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F9F0: 38699138  addi r3, r9, -0x6ec8
	ctx.r[3].s64 = ctx.r[9].s64 + -28360;
	// 8324F9F4: 4BA5A52D  bl 0x82ca9f20
	ctx.lr = 0x8324F9F8;
	sub_82CA9F20(ctx, base);
	// 8324F9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FA08 size=64
    let mut pc: u32 = 0x8324FA08;
    'dispatch: loop {
        match pc {
            0x8324FA08 => {
    //   block [0x8324FA08..0x8324FA48)
	// 8324FA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FA14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FA18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FA1C: 388B09A4  addi r4, r11, 0x9a4
	ctx.r[4].s64 = ctx.r[11].s64 + 2468;
	// 8324FA20: 386A7E64  addi r3, r10, 0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + 32356;
	// 8324FA24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FA28: 4AFDD4A9  bl 0x8222ced0
	ctx.lr = 0x8324FA2C;
	sub_8222CED0(ctx, base);
	// 8324FA2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FA30: 38699148  addi r3, r9, -0x6eb8
	ctx.r[3].s64 = ctx.r[9].s64 + -28344;
	// 8324FA34: 4BA5A4ED  bl 0x82ca9f20
	ctx.lr = 0x8324FA38;
	sub_82CA9F20(ctx, base);
	// 8324FA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FA48 size=64
    let mut pc: u32 = 0x8324FA48;
    'dispatch: loop {
        match pc {
            0x8324FA48 => {
    //   block [0x8324FA48..0x8324FA88)
	// 8324FA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FA54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FA58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FA5C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324FA60: 386A7E68  addi r3, r10, 0x7e68
	ctx.r[3].s64 = ctx.r[10].s64 + 32360;
	// 8324FA64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FA68: 4AFDD469  bl 0x8222ced0
	ctx.lr = 0x8324FA6C;
	sub_8222CED0(ctx, base);
	// 8324FA6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FA70: 38699158  addi r3, r9, -0x6ea8
	ctx.r[3].s64 = ctx.r[9].s64 + -28328;
	// 8324FA74: 4BA5A4AD  bl 0x82ca9f20
	ctx.lr = 0x8324FA78;
	sub_82CA9F20(ctx, base);
	// 8324FA78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FA88 size=64
    let mut pc: u32 = 0x8324FA88;
    'dispatch: loop {
        match pc {
            0x8324FA88 => {
    //   block [0x8324FA88..0x8324FAC8)
	// 8324FA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FA90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FA94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FA98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FA9C: 388B09C0  addi r4, r11, 0x9c0
	ctx.r[4].s64 = ctx.r[11].s64 + 2496;
	// 8324FAA0: 386A7E6C  addi r3, r10, 0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 32364;
	// 8324FAA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FAA8: 4AFDD429  bl 0x8222ced0
	ctx.lr = 0x8324FAAC;
	sub_8222CED0(ctx, base);
	// 8324FAAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FAB0: 38699168  addi r3, r9, -0x6e98
	ctx.r[3].s64 = ctx.r[9].s64 + -28312;
	// 8324FAB4: 4BA5A46D  bl 0x82ca9f20
	ctx.lr = 0x8324FAB8;
	sub_82CA9F20(ctx, base);
	// 8324FAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FAC8 size=64
    let mut pc: u32 = 0x8324FAC8;
    'dispatch: loop {
        match pc {
            0x8324FAC8 => {
    //   block [0x8324FAC8..0x8324FB08)
	// 8324FAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FAD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FAD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FAD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FADC: 388B09E4  addi r4, r11, 0x9e4
	ctx.r[4].s64 = ctx.r[11].s64 + 2532;
	// 8324FAE0: 386A7E70  addi r3, r10, 0x7e70
	ctx.r[3].s64 = ctx.r[10].s64 + 32368;
	// 8324FAE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FAE8: 4AFDD3E9  bl 0x8222ced0
	ctx.lr = 0x8324FAEC;
	sub_8222CED0(ctx, base);
	// 8324FAEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FAF0: 38699178  addi r3, r9, -0x6e88
	ctx.r[3].s64 = ctx.r[9].s64 + -28296;
	// 8324FAF4: 4BA5A42D  bl 0x82ca9f20
	ctx.lr = 0x8324FAF8;
	sub_82CA9F20(ctx, base);
	// 8324FAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FB08 size=64
    let mut pc: u32 = 0x8324FB08;
    'dispatch: loop {
        match pc {
            0x8324FB08 => {
    //   block [0x8324FB08..0x8324FB48)
	// 8324FB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FB14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FB18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FB1C: 388B09FC  addi r4, r11, 0x9fc
	ctx.r[4].s64 = ctx.r[11].s64 + 2556;
	// 8324FB20: 386A7E74  addi r3, r10, 0x7e74
	ctx.r[3].s64 = ctx.r[10].s64 + 32372;
	// 8324FB24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FB28: 4AFDD3A9  bl 0x8222ced0
	ctx.lr = 0x8324FB2C;
	sub_8222CED0(ctx, base);
	// 8324FB2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FB30: 38699188  addi r3, r9, -0x6e78
	ctx.r[3].s64 = ctx.r[9].s64 + -28280;
	// 8324FB34: 4BA5A3ED  bl 0x82ca9f20
	ctx.lr = 0x8324FB38;
	sub_82CA9F20(ctx, base);
	// 8324FB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FB48 size=64
    let mut pc: u32 = 0x8324FB48;
    'dispatch: loop {
        match pc {
            0x8324FB48 => {
    //   block [0x8324FB48..0x8324FB88)
	// 8324FB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FB50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FB54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FB58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FB5C: 388B0A18  addi r4, r11, 0xa18
	ctx.r[4].s64 = ctx.r[11].s64 + 2584;
	// 8324FB60: 386A7E78  addi r3, r10, 0x7e78
	ctx.r[3].s64 = ctx.r[10].s64 + 32376;
	// 8324FB64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FB68: 4AFDD369  bl 0x8222ced0
	ctx.lr = 0x8324FB6C;
	sub_8222CED0(ctx, base);
	// 8324FB6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FB70: 38699198  addi r3, r9, -0x6e68
	ctx.r[3].s64 = ctx.r[9].s64 + -28264;
	// 8324FB74: 4BA5A3AD  bl 0x82ca9f20
	ctx.lr = 0x8324FB78;
	sub_82CA9F20(ctx, base);
	// 8324FB78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FB88 size=64
    let mut pc: u32 = 0x8324FB88;
    'dispatch: loop {
        match pc {
            0x8324FB88 => {
    //   block [0x8324FB88..0x8324FBC8)
	// 8324FB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FB90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FB94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FB98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FB9C: 388B0A38  addi r4, r11, 0xa38
	ctx.r[4].s64 = ctx.r[11].s64 + 2616;
	// 8324FBA0: 386A7E7C  addi r3, r10, 0x7e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 32380;
	// 8324FBA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FBA8: 4AFDD329  bl 0x8222ced0
	ctx.lr = 0x8324FBAC;
	sub_8222CED0(ctx, base);
	// 8324FBAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FBB0: 386991A8  addi r3, r9, -0x6e58
	ctx.r[3].s64 = ctx.r[9].s64 + -28248;
	// 8324FBB4: 4BA5A36D  bl 0x82ca9f20
	ctx.lr = 0x8324FBB8;
	sub_82CA9F20(ctx, base);
	// 8324FBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FBC8 size=64
    let mut pc: u32 = 0x8324FBC8;
    'dispatch: loop {
        match pc {
            0x8324FBC8 => {
    //   block [0x8324FBC8..0x8324FC08)
	// 8324FBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FBD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FBD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FBDC: 388B0A50  addi r4, r11, 0xa50
	ctx.r[4].s64 = ctx.r[11].s64 + 2640;
	// 8324FBE0: 386A7E80  addi r3, r10, 0x7e80
	ctx.r[3].s64 = ctx.r[10].s64 + 32384;
	// 8324FBE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FBE8: 4AFDD2E9  bl 0x8222ced0
	ctx.lr = 0x8324FBEC;
	sub_8222CED0(ctx, base);
	// 8324FBEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FBF0: 386991B8  addi r3, r9, -0x6e48
	ctx.r[3].s64 = ctx.r[9].s64 + -28232;
	// 8324FBF4: 4BA5A32D  bl 0x82ca9f20
	ctx.lr = 0x8324FBF8;
	sub_82CA9F20(ctx, base);
	// 8324FBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FC08 size=64
    let mut pc: u32 = 0x8324FC08;
    'dispatch: loop {
        match pc {
            0x8324FC08 => {
    //   block [0x8324FC08..0x8324FC48)
	// 8324FC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FC14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FC18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FC1C: 388B0A6C  addi r4, r11, 0xa6c
	ctx.r[4].s64 = ctx.r[11].s64 + 2668;
	// 8324FC20: 386A7E84  addi r3, r10, 0x7e84
	ctx.r[3].s64 = ctx.r[10].s64 + 32388;
	// 8324FC24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FC28: 4AFDD2A9  bl 0x8222ced0
	ctx.lr = 0x8324FC2C;
	sub_8222CED0(ctx, base);
	// 8324FC2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FC30: 386991C8  addi r3, r9, -0x6e38
	ctx.r[3].s64 = ctx.r[9].s64 + -28216;
	// 8324FC34: 4BA5A2ED  bl 0x82ca9f20
	ctx.lr = 0x8324FC38;
	sub_82CA9F20(ctx, base);
	// 8324FC38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FC3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FC40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FC48 size=64
    let mut pc: u32 = 0x8324FC48;
    'dispatch: loop {
        match pc {
            0x8324FC48 => {
    //   block [0x8324FC48..0x8324FC88)
	// 8324FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FC50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FC54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FC58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FC5C: 388B0A88  addi r4, r11, 0xa88
	ctx.r[4].s64 = ctx.r[11].s64 + 2696;
	// 8324FC60: 386A7E88  addi r3, r10, 0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + 32392;
	// 8324FC64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FC68: 4AFDD269  bl 0x8222ced0
	ctx.lr = 0x8324FC6C;
	sub_8222CED0(ctx, base);
	// 8324FC6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FC70: 386991D8  addi r3, r9, -0x6e28
	ctx.r[3].s64 = ctx.r[9].s64 + -28200;
	// 8324FC74: 4BA5A2AD  bl 0x82ca9f20
	ctx.lr = 0x8324FC78;
	sub_82CA9F20(ctx, base);
	// 8324FC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FC84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FC88 size=64
    let mut pc: u32 = 0x8324FC88;
    'dispatch: loop {
        match pc {
            0x8324FC88 => {
    //   block [0x8324FC88..0x8324FCC8)
	// 8324FC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FC90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FC94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FC98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FC9C: 388B0AA8  addi r4, r11, 0xaa8
	ctx.r[4].s64 = ctx.r[11].s64 + 2728;
	// 8324FCA0: 386A7E8C  addi r3, r10, 0x7e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 32396;
	// 8324FCA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FCA8: 4AFDD229  bl 0x8222ced0
	ctx.lr = 0x8324FCAC;
	sub_8222CED0(ctx, base);
	// 8324FCAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FCB0: 386991E8  addi r3, r9, -0x6e18
	ctx.r[3].s64 = ctx.r[9].s64 + -28184;
	// 8324FCB4: 4BA5A26D  bl 0x82ca9f20
	ctx.lr = 0x8324FCB8;
	sub_82CA9F20(ctx, base);
	// 8324FCB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FCC8 size=64
    let mut pc: u32 = 0x8324FCC8;
    'dispatch: loop {
        match pc {
            0x8324FCC8 => {
    //   block [0x8324FCC8..0x8324FD08)
	// 8324FCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FCD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FCD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FCDC: 388B0AC4  addi r4, r11, 0xac4
	ctx.r[4].s64 = ctx.r[11].s64 + 2756;
	// 8324FCE0: 386A7E90  addi r3, r10, 0x7e90
	ctx.r[3].s64 = ctx.r[10].s64 + 32400;
	// 8324FCE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FCE8: 4AFDD1E9  bl 0x8222ced0
	ctx.lr = 0x8324FCEC;
	sub_8222CED0(ctx, base);
	// 8324FCEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FCF0: 386991F8  addi r3, r9, -0x6e08
	ctx.r[3].s64 = ctx.r[9].s64 + -28168;
	// 8324FCF4: 4BA5A22D  bl 0x82ca9f20
	ctx.lr = 0x8324FCF8;
	sub_82CA9F20(ctx, base);
	// 8324FCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FD08 size=64
    let mut pc: u32 = 0x8324FD08;
    'dispatch: loop {
        match pc {
            0x8324FD08 => {
    //   block [0x8324FD08..0x8324FD48)
	// 8324FD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FD10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FD14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FD18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FD1C: 388B0ADC  addi r4, r11, 0xadc
	ctx.r[4].s64 = ctx.r[11].s64 + 2780;
	// 8324FD20: 386A7E94  addi r3, r10, 0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + 32404;
	// 8324FD24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FD28: 4AFDD1A9  bl 0x8222ced0
	ctx.lr = 0x8324FD2C;
	sub_8222CED0(ctx, base);
	// 8324FD2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FD30: 38699208  addi r3, r9, -0x6df8
	ctx.r[3].s64 = ctx.r[9].s64 + -28152;
	// 8324FD34: 4BA5A1ED  bl 0x82ca9f20
	ctx.lr = 0x8324FD38;
	sub_82CA9F20(ctx, base);
	// 8324FD38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FD48 size=64
    let mut pc: u32 = 0x8324FD48;
    'dispatch: loop {
        match pc {
            0x8324FD48 => {
    //   block [0x8324FD48..0x8324FD88)
	// 8324FD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FD50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FD54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FD58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FD5C: 388B0AF4  addi r4, r11, 0xaf4
	ctx.r[4].s64 = ctx.r[11].s64 + 2804;
	// 8324FD60: 386A7E98  addi r3, r10, 0x7e98
	ctx.r[3].s64 = ctx.r[10].s64 + 32408;
	// 8324FD64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FD68: 4AFDD169  bl 0x8222ced0
	ctx.lr = 0x8324FD6C;
	sub_8222CED0(ctx, base);
	// 8324FD6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FD70: 38699218  addi r3, r9, -0x6de8
	ctx.r[3].s64 = ctx.r[9].s64 + -28136;
	// 8324FD74: 4BA5A1AD  bl 0x82ca9f20
	ctx.lr = 0x8324FD78;
	sub_82CA9F20(ctx, base);
	// 8324FD78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FD88 size=64
    let mut pc: u32 = 0x8324FD88;
    'dispatch: loop {
        match pc {
            0x8324FD88 => {
    //   block [0x8324FD88..0x8324FDC8)
	// 8324FD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FD90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FD94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FD98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FD9C: 388B0B0C  addi r4, r11, 0xb0c
	ctx.r[4].s64 = ctx.r[11].s64 + 2828;
	// 8324FDA0: 386A7E9C  addi r3, r10, 0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 32412;
	// 8324FDA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FDA8: 4AFDD129  bl 0x8222ced0
	ctx.lr = 0x8324FDAC;
	sub_8222CED0(ctx, base);
	// 8324FDAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FDB0: 38699228  addi r3, r9, -0x6dd8
	ctx.r[3].s64 = ctx.r[9].s64 + -28120;
	// 8324FDB4: 4BA5A16D  bl 0x82ca9f20
	ctx.lr = 0x8324FDB8;
	sub_82CA9F20(ctx, base);
	// 8324FDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FDC8 size=64
    let mut pc: u32 = 0x8324FDC8;
    'dispatch: loop {
        match pc {
            0x8324FDC8 => {
    //   block [0x8324FDC8..0x8324FE08)
	// 8324FDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FDD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FDD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FDDC: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324FDE0: 386A7EA0  addi r3, r10, 0x7ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 32416;
	// 8324FDE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FDE8: 4AFDD0E9  bl 0x8222ced0
	ctx.lr = 0x8324FDEC;
	sub_8222CED0(ctx, base);
	// 8324FDEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FDF0: 38699238  addi r3, r9, -0x6dc8
	ctx.r[3].s64 = ctx.r[9].s64 + -28104;
	// 8324FDF4: 4BA5A12D  bl 0x82ca9f20
	ctx.lr = 0x8324FDF8;
	sub_82CA9F20(ctx, base);
	// 8324FDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FE08 size=64
    let mut pc: u32 = 0x8324FE08;
    'dispatch: loop {
        match pc {
            0x8324FE08 => {
    //   block [0x8324FE08..0x8324FE48)
	// 8324FE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FE14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FE18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FE1C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324FE20: 386A7EA4  addi r3, r10, 0x7ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 32420;
	// 8324FE24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FE28: 4AFDD0A9  bl 0x8222ced0
	ctx.lr = 0x8324FE2C;
	sub_8222CED0(ctx, base);
	// 8324FE2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FE30: 38699248  addi r3, r9, -0x6db8
	ctx.r[3].s64 = ctx.r[9].s64 + -28088;
	// 8324FE34: 4BA5A0ED  bl 0x82ca9f20
	ctx.lr = 0x8324FE38;
	sub_82CA9F20(ctx, base);
	// 8324FE38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FE48 size=64
    let mut pc: u32 = 0x8324FE48;
    'dispatch: loop {
        match pc {
            0x8324FE48 => {
    //   block [0x8324FE48..0x8324FE88)
	// 8324FE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FE50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FE54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FE58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FE5C: 388B0B30  addi r4, r11, 0xb30
	ctx.r[4].s64 = ctx.r[11].s64 + 2864;
	// 8324FE60: 386A7EA8  addi r3, r10, 0x7ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 32424;
	// 8324FE64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FE68: 4AFDD069  bl 0x8222ced0
	ctx.lr = 0x8324FE6C;
	sub_8222CED0(ctx, base);
	// 8324FE6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FE70: 38699258  addi r3, r9, -0x6da8
	ctx.r[3].s64 = ctx.r[9].s64 + -28072;
	// 8324FE74: 4BA5A0AD  bl 0x82ca9f20
	ctx.lr = 0x8324FE78;
	sub_82CA9F20(ctx, base);
	// 8324FE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FE88 size=64
    let mut pc: u32 = 0x8324FE88;
    'dispatch: loop {
        match pc {
            0x8324FE88 => {
    //   block [0x8324FE88..0x8324FEC8)
	// 8324FE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FE94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FE98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FE9C: 388B0B50  addi r4, r11, 0xb50
	ctx.r[4].s64 = ctx.r[11].s64 + 2896;
	// 8324FEA0: 386A7EAC  addi r3, r10, 0x7eac
	ctx.r[3].s64 = ctx.r[10].s64 + 32428;
	// 8324FEA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FEA8: 4AFDD029  bl 0x8222ced0
	ctx.lr = 0x8324FEAC;
	sub_8222CED0(ctx, base);
	// 8324FEAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FEB0: 38699268  addi r3, r9, -0x6d98
	ctx.r[3].s64 = ctx.r[9].s64 + -28056;
	// 8324FEB4: 4BA5A06D  bl 0x82ca9f20
	ctx.lr = 0x8324FEB8;
	sub_82CA9F20(ctx, base);
	// 8324FEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FEC8 size=64
    let mut pc: u32 = 0x8324FEC8;
    'dispatch: loop {
        match pc {
            0x8324FEC8 => {
    //   block [0x8324FEC8..0x8324FF08)
	// 8324FEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FED4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FED8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FEDC: 388B0B70  addi r4, r11, 0xb70
	ctx.r[4].s64 = ctx.r[11].s64 + 2928;
	// 8324FEE0: 386A7EB0  addi r3, r10, 0x7eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 32432;
	// 8324FEE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FEE8: 4AFDCFE9  bl 0x8222ced0
	ctx.lr = 0x8324FEEC;
	sub_8222CED0(ctx, base);
	// 8324FEEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FEF0: 38699278  addi r3, r9, -0x6d88
	ctx.r[3].s64 = ctx.r[9].s64 + -28040;
	// 8324FEF4: 4BA5A02D  bl 0x82ca9f20
	ctx.lr = 0x8324FEF8;
	sub_82CA9F20(ctx, base);
	// 8324FEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FF08 size=64
    let mut pc: u32 = 0x8324FF08;
    'dispatch: loop {
        match pc {
            0x8324FF08 => {
    //   block [0x8324FF08..0x8324FF48)
	// 8324FF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FF14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FF18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FF1C: 388B0B8C  addi r4, r11, 0xb8c
	ctx.r[4].s64 = ctx.r[11].s64 + 2956;
	// 8324FF20: 386A7EB4  addi r3, r10, 0x7eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32436;
	// 8324FF24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FF28: 4AFDCFA9  bl 0x8222ced0
	ctx.lr = 0x8324FF2C;
	sub_8222CED0(ctx, base);
	// 8324FF2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FF30: 38699288  addi r3, r9, -0x6d78
	ctx.r[3].s64 = ctx.r[9].s64 + -28024;
	// 8324FF34: 4BA59FED  bl 0x82ca9f20
	ctx.lr = 0x8324FF38;
	sub_82CA9F20(ctx, base);
	// 8324FF38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FF48 size=64
    let mut pc: u32 = 0x8324FF48;
    'dispatch: loop {
        match pc {
            0x8324FF48 => {
    //   block [0x8324FF48..0x8324FF88)
	// 8324FF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FF54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FF58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FF5C: 388B0BA8  addi r4, r11, 0xba8
	ctx.r[4].s64 = ctx.r[11].s64 + 2984;
	// 8324FF60: 386A7EB8  addi r3, r10, 0x7eb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32440;
	// 8324FF64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FF68: 4AFDCF69  bl 0x8222ced0
	ctx.lr = 0x8324FF6C;
	sub_8222CED0(ctx, base);
	// 8324FF6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FF70: 38699298  addi r3, r9, -0x6d68
	ctx.r[3].s64 = ctx.r[9].s64 + -28008;
	// 8324FF74: 4BA59FAD  bl 0x82ca9f20
	ctx.lr = 0x8324FF78;
	sub_82CA9F20(ctx, base);
	// 8324FF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FF88 size=64
    let mut pc: u32 = 0x8324FF88;
    'dispatch: loop {
        match pc {
            0x8324FF88 => {
    //   block [0x8324FF88..0x8324FFC8)
	// 8324FF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FF94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FF98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FF9C: 388B0BC8  addi r4, r11, 0xbc8
	ctx.r[4].s64 = ctx.r[11].s64 + 3016;
	// 8324FFA0: 386A7EBC  addi r3, r10, 0x7ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 32444;
	// 8324FFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FFA8: 4AFDCF29  bl 0x8222ced0
	ctx.lr = 0x8324FFAC;
	sub_8222CED0(ctx, base);
	// 8324FFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FFB0: 386992A8  addi r3, r9, -0x6d58
	ctx.r[3].s64 = ctx.r[9].s64 + -27992;
	// 8324FFB4: 4BA59F6D  bl 0x82ca9f20
	ctx.lr = 0x8324FFB8;
	sub_82CA9F20(ctx, base);
	// 8324FFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FFC8 size=64
    let mut pc: u32 = 0x8324FFC8;
    'dispatch: loop {
        match pc {
            0x8324FFC8 => {
    //   block [0x8324FFC8..0x83250008)
	// 8324FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FFD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FFD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FFDC: 388B0BDC  addi r4, r11, 0xbdc
	ctx.r[4].s64 = ctx.r[11].s64 + 3036;
	// 8324FFE0: 386A7EC0  addi r3, r10, 0x7ec0
	ctx.r[3].s64 = ctx.r[10].s64 + 32448;
	// 8324FFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FFE8: 4AFDCEE9  bl 0x8222ced0
	ctx.lr = 0x8324FFEC;
	sub_8222CED0(ctx, base);
	// 8324FFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FFF0: 386992B8  addi r3, r9, -0x6d48
	ctx.r[3].s64 = ctx.r[9].s64 + -27976;
	// 8324FFF4: 4BA59F2D  bl 0x82ca9f20
	ctx.lr = 0x8324FFF8;
	sub_82CA9F20(ctx, base);
	// 8324FFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250008 size=64
    let mut pc: u32 = 0x83250008;
    'dispatch: loop {
        match pc {
            0x83250008 => {
    //   block [0x83250008..0x83250048)
	// 83250008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250014: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250018: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325001C: 388B0BF0  addi r4, r11, 0xbf0
	ctx.r[4].s64 = ctx.r[11].s64 + 3056;
	// 83250020: 386A7EC4  addi r3, r10, 0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 32452;
	// 83250024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250028: 4AFDCEA9  bl 0x8222ced0
	ctx.lr = 0x8325002C;
	sub_8222CED0(ctx, base);
	// 8325002C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250030: 386992C8  addi r3, r9, -0x6d38
	ctx.r[3].s64 = ctx.r[9].s64 + -27960;
	// 83250034: 4BA59EED  bl 0x82ca9f20
	ctx.lr = 0x83250038;
	sub_82CA9F20(ctx, base);
	// 83250038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325003C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250048 size=64
    let mut pc: u32 = 0x83250048;
    'dispatch: loop {
        match pc {
            0x83250048 => {
    //   block [0x83250048..0x83250088)
	// 83250048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325004C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250054: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250058: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325005C: 388B0C10  addi r4, r11, 0xc10
	ctx.r[4].s64 = ctx.r[11].s64 + 3088;
	// 83250060: 386A7EC8  addi r3, r10, 0x7ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 32456;
	// 83250064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250068: 4AFDCE69  bl 0x8222ced0
	ctx.lr = 0x8325006C;
	sub_8222CED0(ctx, base);
	// 8325006C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250070: 386992D8  addi r3, r9, -0x6d28
	ctx.r[3].s64 = ctx.r[9].s64 + -27944;
	// 83250074: 4BA59EAD  bl 0x82ca9f20
	ctx.lr = 0x83250078;
	sub_82CA9F20(ctx, base);
	// 83250078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325007C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250088 size=64
    let mut pc: u32 = 0x83250088;
    'dispatch: loop {
        match pc {
            0x83250088 => {
    //   block [0x83250088..0x832500C8)
	// 83250088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325008C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250094: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250098: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325009C: 388B0C28  addi r4, r11, 0xc28
	ctx.r[4].s64 = ctx.r[11].s64 + 3112;
	// 832500A0: 386A7ECC  addi r3, r10, 0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 32460;
	// 832500A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832500A8: 4AFDCE29  bl 0x8222ced0
	ctx.lr = 0x832500AC;
	sub_8222CED0(ctx, base);
	// 832500AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832500B0: 386992E8  addi r3, r9, -0x6d18
	ctx.r[3].s64 = ctx.r[9].s64 + -27928;
	// 832500B4: 4BA59E6D  bl 0x82ca9f20
	ctx.lr = 0x832500B8;
	sub_82CA9F20(ctx, base);
	// 832500B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832500BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832500C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832500C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832500C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832500C8 size=64
    let mut pc: u32 = 0x832500C8;
    'dispatch: loop {
        match pc {
            0x832500C8 => {
    //   block [0x832500C8..0x83250108)
	// 832500C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832500CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832500D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832500D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832500D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832500DC: 388B0C40  addi r4, r11, 0xc40
	ctx.r[4].s64 = ctx.r[11].s64 + 3136;
	// 832500E0: 386A7ED0  addi r3, r10, 0x7ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 32464;
	// 832500E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832500E8: 4AFDCDE9  bl 0x8222ced0
	ctx.lr = 0x832500EC;
	sub_8222CED0(ctx, base);
	// 832500EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832500F0: 386992F8  addi r3, r9, -0x6d08
	ctx.r[3].s64 = ctx.r[9].s64 + -27912;
	// 832500F4: 4BA59E2D  bl 0x82ca9f20
	ctx.lr = 0x832500F8;
	sub_82CA9F20(ctx, base);
	// 832500F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832500FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250108 size=64
    let mut pc: u32 = 0x83250108;
    'dispatch: loop {
        match pc {
            0x83250108 => {
    //   block [0x83250108..0x83250148)
	// 83250108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250114: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250118: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325011C: 388B0C5C  addi r4, r11, 0xc5c
	ctx.r[4].s64 = ctx.r[11].s64 + 3164;
	// 83250120: 386A7ED4  addi r3, r10, 0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 32468;
	// 83250124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250128: 4AFDCDA9  bl 0x8222ced0
	ctx.lr = 0x8325012C;
	sub_8222CED0(ctx, base);
	// 8325012C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250130: 38699308  addi r3, r9, -0x6cf8
	ctx.r[3].s64 = ctx.r[9].s64 + -27896;
	// 83250134: 4BA59DED  bl 0x82ca9f20
	ctx.lr = 0x83250138;
	sub_82CA9F20(ctx, base);
	// 83250138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325013C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250148 size=64
    let mut pc: u32 = 0x83250148;
    'dispatch: loop {
        match pc {
            0x83250148 => {
    //   block [0x83250148..0x83250188)
	// 83250148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325014C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250154: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250158: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325015C: 388B0C7C  addi r4, r11, 0xc7c
	ctx.r[4].s64 = ctx.r[11].s64 + 3196;
	// 83250160: 386A7ED8  addi r3, r10, 0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 32472;
	// 83250164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250168: 4AFDCD69  bl 0x8222ced0
	ctx.lr = 0x8325016C;
	sub_8222CED0(ctx, base);
	// 8325016C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250170: 38699318  addi r3, r9, -0x6ce8
	ctx.r[3].s64 = ctx.r[9].s64 + -27880;
	// 83250174: 4BA59DAD  bl 0x82ca9f20
	ctx.lr = 0x83250178;
	sub_82CA9F20(ctx, base);
	// 83250178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250188 size=64
    let mut pc: u32 = 0x83250188;
    'dispatch: loop {
        match pc {
            0x83250188 => {
    //   block [0x83250188..0x832501C8)
	// 83250188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250194: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250198: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325019C: 388B0C9C  addi r4, r11, 0xc9c
	ctx.r[4].s64 = ctx.r[11].s64 + 3228;
	// 832501A0: 386A7EDC  addi r3, r10, 0x7edc
	ctx.r[3].s64 = ctx.r[10].s64 + 32476;
	// 832501A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832501A8: 4AFDCD29  bl 0x8222ced0
	ctx.lr = 0x832501AC;
	sub_8222CED0(ctx, base);
	// 832501AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832501B0: 38699328  addi r3, r9, -0x6cd8
	ctx.r[3].s64 = ctx.r[9].s64 + -27864;
	// 832501B4: 4BA59D6D  bl 0x82ca9f20
	ctx.lr = 0x832501B8;
	sub_82CA9F20(ctx, base);
	// 832501B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832501BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832501C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832501C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832501C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832501C8 size=64
    let mut pc: u32 = 0x832501C8;
    'dispatch: loop {
        match pc {
            0x832501C8 => {
    //   block [0x832501C8..0x83250208)
	// 832501C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832501CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832501D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832501D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832501D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832501DC: 388B0CB8  addi r4, r11, 0xcb8
	ctx.r[4].s64 = ctx.r[11].s64 + 3256;
	// 832501E0: 386A7EE0  addi r3, r10, 0x7ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 32480;
	// 832501E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832501E8: 4AFDCCE9  bl 0x8222ced0
	ctx.lr = 0x832501EC;
	sub_8222CED0(ctx, base);
	// 832501EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832501F0: 38699338  addi r3, r9, -0x6cc8
	ctx.r[3].s64 = ctx.r[9].s64 + -27848;
	// 832501F4: 4BA59D2D  bl 0x82ca9f20
	ctx.lr = 0x832501F8;
	sub_82CA9F20(ctx, base);
	// 832501F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832501FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250208 size=64
    let mut pc: u32 = 0x83250208;
    'dispatch: loop {
        match pc {
            0x83250208 => {
    //   block [0x83250208..0x83250248)
	// 83250208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250214: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250218: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325021C: 388B0CD0  addi r4, r11, 0xcd0
	ctx.r[4].s64 = ctx.r[11].s64 + 3280;
	// 83250220: 386A7EE4  addi r3, r10, 0x7ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 32484;
	// 83250224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250228: 4AFDCCA9  bl 0x8222ced0
	ctx.lr = 0x8325022C;
	sub_8222CED0(ctx, base);
	// 8325022C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250230: 38699348  addi r3, r9, -0x6cb8
	ctx.r[3].s64 = ctx.r[9].s64 + -27832;
	// 83250234: 4BA59CED  bl 0x82ca9f20
	ctx.lr = 0x83250238;
	sub_82CA9F20(ctx, base);
	// 83250238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325023C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250248 size=64
    let mut pc: u32 = 0x83250248;
    'dispatch: loop {
        match pc {
            0x83250248 => {
    //   block [0x83250248..0x83250288)
	// 83250248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325024C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250254: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250258: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325025C: 388B0CE0  addi r4, r11, 0xce0
	ctx.r[4].s64 = ctx.r[11].s64 + 3296;
	// 83250260: 386A7EE8  addi r3, r10, 0x7ee8
	ctx.r[3].s64 = ctx.r[10].s64 + 32488;
	// 83250264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250268: 4AFDCC69  bl 0x8222ced0
	ctx.lr = 0x8325026C;
	sub_8222CED0(ctx, base);
	// 8325026C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250270: 38699358  addi r3, r9, -0x6ca8
	ctx.r[3].s64 = ctx.r[9].s64 + -27816;
	// 83250274: 4BA59CAD  bl 0x82ca9f20
	ctx.lr = 0x83250278;
	sub_82CA9F20(ctx, base);
	// 83250278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325027C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250288 size=64
    let mut pc: u32 = 0x83250288;
    'dispatch: loop {
        match pc {
            0x83250288 => {
    //   block [0x83250288..0x832502C8)
	// 83250288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325028C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250294: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250298: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325029C: 388B0D00  addi r4, r11, 0xd00
	ctx.r[4].s64 = ctx.r[11].s64 + 3328;
	// 832502A0: 386A7EEC  addi r3, r10, 0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + 32492;
	// 832502A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832502A8: 4AFDCC29  bl 0x8222ced0
	ctx.lr = 0x832502AC;
	sub_8222CED0(ctx, base);
	// 832502AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832502B0: 38699368  addi r3, r9, -0x6c98
	ctx.r[3].s64 = ctx.r[9].s64 + -27800;
	// 832502B4: 4BA59C6D  bl 0x82ca9f20
	ctx.lr = 0x832502B8;
	sub_82CA9F20(ctx, base);
	// 832502B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832502BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832502C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832502C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832502C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832502C8 size=64
    let mut pc: u32 = 0x832502C8;
    'dispatch: loop {
        match pc {
            0x832502C8 => {
    //   block [0x832502C8..0x83250308)
	// 832502C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832502CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832502D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832502D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832502D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832502DC: 388B0D1C  addi r4, r11, 0xd1c
	ctx.r[4].s64 = ctx.r[11].s64 + 3356;
	// 832502E0: 386A7EF0  addi r3, r10, 0x7ef0
	ctx.r[3].s64 = ctx.r[10].s64 + 32496;
	// 832502E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832502E8: 4AFDCBE9  bl 0x8222ced0
	ctx.lr = 0x832502EC;
	sub_8222CED0(ctx, base);
	// 832502EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832502F0: 38699378  addi r3, r9, -0x6c88
	ctx.r[3].s64 = ctx.r[9].s64 + -27784;
	// 832502F4: 4BA59C2D  bl 0x82ca9f20
	ctx.lr = 0x832502F8;
	sub_82CA9F20(ctx, base);
	// 832502F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832502FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250308 size=64
    let mut pc: u32 = 0x83250308;
    'dispatch: loop {
        match pc {
            0x83250308 => {
    //   block [0x83250308..0x83250348)
	// 83250308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250314: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250318: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325031C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 83250320: 386A7EF4  addi r3, r10, 0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 32500;
	// 83250324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250328: 4AFDCBA9  bl 0x8222ced0
	ctx.lr = 0x8325032C;
	sub_8222CED0(ctx, base);
	// 8325032C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250330: 38699388  addi r3, r9, -0x6c78
	ctx.r[3].s64 = ctx.r[9].s64 + -27768;
	// 83250334: 4BA59BED  bl 0x82ca9f20
	ctx.lr = 0x83250338;
	sub_82CA9F20(ctx, base);
	// 83250338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325033C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250348 size=64
    let mut pc: u32 = 0x83250348;
    'dispatch: loop {
        match pc {
            0x83250348 => {
    //   block [0x83250348..0x83250388)
	// 83250348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250354: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250358: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325035C: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 83250360: 386A7EF8  addi r3, r10, 0x7ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 32504;
	// 83250364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250368: 4AFDCB69  bl 0x8222ced0
	ctx.lr = 0x8325036C;
	sub_8222CED0(ctx, base);
	// 8325036C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250370: 38699398  addi r3, r9, -0x6c68
	ctx.r[3].s64 = ctx.r[9].s64 + -27752;
	// 83250374: 4BA59BAD  bl 0x82ca9f20
	ctx.lr = 0x83250378;
	sub_82CA9F20(ctx, base);
	// 83250378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325037C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250388 size=64
    let mut pc: u32 = 0x83250388;
    'dispatch: loop {
        match pc {
            0x83250388 => {
    //   block [0x83250388..0x832503C8)
	// 83250388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250394: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250398: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325039C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 832503A0: 386A7EFC  addi r3, r10, 0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + 32508;
	// 832503A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832503A8: 4AFDCB29  bl 0x8222ced0
	ctx.lr = 0x832503AC;
	sub_8222CED0(ctx, base);
	// 832503AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832503B0: 386993A8  addi r3, r9, -0x6c58
	ctx.r[3].s64 = ctx.r[9].s64 + -27736;
	// 832503B4: 4BA59B6D  bl 0x82ca9f20
	ctx.lr = 0x832503B8;
	sub_82CA9F20(ctx, base);
	// 832503B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832503BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832503C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832503C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832503C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832503C8 size=64
    let mut pc: u32 = 0x832503C8;
    'dispatch: loop {
        match pc {
            0x832503C8 => {
    //   block [0x832503C8..0x83250408)
	// 832503C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832503CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832503D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832503D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832503D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832503DC: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 832503E0: 386A7F00  addi r3, r10, 0x7f00
	ctx.r[3].s64 = ctx.r[10].s64 + 32512;
	// 832503E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832503E8: 4AFDCAE9  bl 0x8222ced0
	ctx.lr = 0x832503EC;
	sub_8222CED0(ctx, base);
	// 832503EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832503F0: 386993B8  addi r3, r9, -0x6c48
	ctx.r[3].s64 = ctx.r[9].s64 + -27720;
	// 832503F4: 4BA59B2D  bl 0x82ca9f20
	ctx.lr = 0x832503F8;
	sub_82CA9F20(ctx, base);
	// 832503F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832503FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250408 size=64
    let mut pc: u32 = 0x83250408;
    'dispatch: loop {
        match pc {
            0x83250408 => {
    //   block [0x83250408..0x83250448)
	// 83250408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325040C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250414: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325041C: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 83250420: 386A7F04  addi r3, r10, 0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + 32516;
	// 83250424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250428: 4AFDCAA9  bl 0x8222ced0
	ctx.lr = 0x8325042C;
	sub_8222CED0(ctx, base);
	// 8325042C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250430: 386993C8  addi r3, r9, -0x6c38
	ctx.r[3].s64 = ctx.r[9].s64 + -27704;
	// 83250434: 4BA59AED  bl 0x82ca9f20
	ctx.lr = 0x83250438;
	sub_82CA9F20(ctx, base);
	// 83250438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325043C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250448 size=64
    let mut pc: u32 = 0x83250448;
    'dispatch: loop {
        match pc {
            0x83250448 => {
    //   block [0x83250448..0x83250488)
	// 83250448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325044C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250454: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325045C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 83250460: 386A7F08  addi r3, r10, 0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + 32520;
	// 83250464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250468: 4AFDCA69  bl 0x8222ced0
	ctx.lr = 0x8325046C;
	sub_8222CED0(ctx, base);
	// 8325046C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250470: 386993D8  addi r3, r9, -0x6c28
	ctx.r[3].s64 = ctx.r[9].s64 + -27688;
	// 83250474: 4BA59AAD  bl 0x82ca9f20
	ctx.lr = 0x83250478;
	sub_82CA9F20(ctx, base);
	// 83250478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250488 size=64
    let mut pc: u32 = 0x83250488;
    'dispatch: loop {
        match pc {
            0x83250488 => {
    //   block [0x83250488..0x832504C8)
	// 83250488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250494: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325049C: 388B0D38  addi r4, r11, 0xd38
	ctx.r[4].s64 = ctx.r[11].s64 + 3384;
	// 832504A0: 386A7F0C  addi r3, r10, 0x7f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32524;
	// 832504A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832504A8: 4AFDCA29  bl 0x8222ced0
	ctx.lr = 0x832504AC;
	sub_8222CED0(ctx, base);
	// 832504AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832504B0: 386993E8  addi r3, r9, -0x6c18
	ctx.r[3].s64 = ctx.r[9].s64 + -27672;
	// 832504B4: 4BA59A6D  bl 0x82ca9f20
	ctx.lr = 0x832504B8;
	sub_82CA9F20(ctx, base);
	// 832504B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832504BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832504C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832504C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832504C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832504C8 size=64
    let mut pc: u32 = 0x832504C8;
    'dispatch: loop {
        match pc {
            0x832504C8 => {
    //   block [0x832504C8..0x83250508)
	// 832504C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832504CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832504D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832504D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832504D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832504DC: 388B0D38  addi r4, r11, 0xd38
	ctx.r[4].s64 = ctx.r[11].s64 + 3384;
	// 832504E0: 386A7F10  addi r3, r10, 0x7f10
	ctx.r[3].s64 = ctx.r[10].s64 + 32528;
	// 832504E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832504E8: 4AFDC9E9  bl 0x8222ced0
	ctx.lr = 0x832504EC;
	sub_8222CED0(ctx, base);
	// 832504EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832504F0: 386993F8  addi r3, r9, -0x6c08
	ctx.r[3].s64 = ctx.r[9].s64 + -27656;
	// 832504F4: 4BA59A2D  bl 0x82ca9f20
	ctx.lr = 0x832504F8;
	sub_82CA9F20(ctx, base);
	// 832504F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832504FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250508 size=64
    let mut pc: u32 = 0x83250508;
    'dispatch: loop {
        match pc {
            0x83250508 => {
    //   block [0x83250508..0x83250548)
	// 83250508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250514: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325051C: 388B0D48  addi r4, r11, 0xd48
	ctx.r[4].s64 = ctx.r[11].s64 + 3400;
	// 83250520: 386A7F14  addi r3, r10, 0x7f14
	ctx.r[3].s64 = ctx.r[10].s64 + 32532;
	// 83250524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250528: 4AFDC9A9  bl 0x8222ced0
	ctx.lr = 0x8325052C;
	sub_8222CED0(ctx, base);
	// 8325052C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250530: 38699408  addi r3, r9, -0x6bf8
	ctx.r[3].s64 = ctx.r[9].s64 + -27640;
	// 83250534: 4BA599ED  bl 0x82ca9f20
	ctx.lr = 0x83250538;
	sub_82CA9F20(ctx, base);
	// 83250538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325053C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250548 size=64
    let mut pc: u32 = 0x83250548;
    'dispatch: loop {
        match pc {
            0x83250548 => {
    //   block [0x83250548..0x83250588)
	// 83250548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325054C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250554: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250558: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325055C: 388B0D5C  addi r4, r11, 0xd5c
	ctx.r[4].s64 = ctx.r[11].s64 + 3420;
	// 83250560: 386A7F18  addi r3, r10, 0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + 32536;
	// 83250564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250568: 4AFDC969  bl 0x8222ced0
	ctx.lr = 0x8325056C;
	sub_8222CED0(ctx, base);
	// 8325056C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250570: 38699418  addi r3, r9, -0x6be8
	ctx.r[3].s64 = ctx.r[9].s64 + -27624;
	// 83250574: 4BA599AD  bl 0x82ca9f20
	ctx.lr = 0x83250578;
	sub_82CA9F20(ctx, base);
	// 83250578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325057C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250588 size=64
    let mut pc: u32 = 0x83250588;
    'dispatch: loop {
        match pc {
            0x83250588 => {
    //   block [0x83250588..0x832505C8)
	// 83250588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325058C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250594: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250598: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325059C: 388B0D6C  addi r4, r11, 0xd6c
	ctx.r[4].s64 = ctx.r[11].s64 + 3436;
	// 832505A0: 386A7F1C  addi r3, r10, 0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 32540;
	// 832505A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832505A8: 4AFDC929  bl 0x8222ced0
	ctx.lr = 0x832505AC;
	sub_8222CED0(ctx, base);
	// 832505AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832505B0: 38699428  addi r3, r9, -0x6bd8
	ctx.r[3].s64 = ctx.r[9].s64 + -27608;
	// 832505B4: 4BA5996D  bl 0x82ca9f20
	ctx.lr = 0x832505B8;
	sub_82CA9F20(ctx, base);
	// 832505B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832505BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832505C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832505C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832505C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832505C8 size=64
    let mut pc: u32 = 0x832505C8;
    'dispatch: loop {
        match pc {
            0x832505C8 => {
    //   block [0x832505C8..0x83250608)
	// 832505C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832505CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832505D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832505D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832505D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832505DC: 388B0D80  addi r4, r11, 0xd80
	ctx.r[4].s64 = ctx.r[11].s64 + 3456;
	// 832505E0: 386A7F20  addi r3, r10, 0x7f20
	ctx.r[3].s64 = ctx.r[10].s64 + 32544;
	// 832505E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832505E8: 4AFDC8E9  bl 0x8222ced0
	ctx.lr = 0x832505EC;
	sub_8222CED0(ctx, base);
	// 832505EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832505F0: 38699438  addi r3, r9, -0x6bc8
	ctx.r[3].s64 = ctx.r[9].s64 + -27592;
	// 832505F4: 4BA5992D  bl 0x82ca9f20
	ctx.lr = 0x832505F8;
	sub_82CA9F20(ctx, base);
	// 832505F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832505FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250608 size=64
    let mut pc: u32 = 0x83250608;
    'dispatch: loop {
        match pc {
            0x83250608 => {
    //   block [0x83250608..0x83250648)
	// 83250608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325060C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250614: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250618: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325061C: 388B0D94  addi r4, r11, 0xd94
	ctx.r[4].s64 = ctx.r[11].s64 + 3476;
	// 83250620: 386A7F24  addi r3, r10, 0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + 32548;
	// 83250624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250628: 4AFDC8A9  bl 0x8222ced0
	ctx.lr = 0x8325062C;
	sub_8222CED0(ctx, base);
	// 8325062C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250630: 38699448  addi r3, r9, -0x6bb8
	ctx.r[3].s64 = ctx.r[9].s64 + -27576;
	// 83250634: 4BA598ED  bl 0x82ca9f20
	ctx.lr = 0x83250638;
	sub_82CA9F20(ctx, base);
	// 83250638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250648 size=64
    let mut pc: u32 = 0x83250648;
    'dispatch: loop {
        match pc {
            0x83250648 => {
    //   block [0x83250648..0x83250688)
	// 83250648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250654: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250658: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325065C: 388B0DA8  addi r4, r11, 0xda8
	ctx.r[4].s64 = ctx.r[11].s64 + 3496;
	// 83250660: 386A7F28  addi r3, r10, 0x7f28
	ctx.r[3].s64 = ctx.r[10].s64 + 32552;
	// 83250664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250668: 4AFDC869  bl 0x8222ced0
	ctx.lr = 0x8325066C;
	sub_8222CED0(ctx, base);
	// 8325066C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250670: 38699458  addi r3, r9, -0x6ba8
	ctx.r[3].s64 = ctx.r[9].s64 + -27560;
	// 83250674: 4BA598AD  bl 0x82ca9f20
	ctx.lr = 0x83250678;
	sub_82CA9F20(ctx, base);
	// 83250678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325067C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250688 size=64
    let mut pc: u32 = 0x83250688;
    'dispatch: loop {
        match pc {
            0x83250688 => {
    //   block [0x83250688..0x832506C8)
	// 83250688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325068C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250694: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250698: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325069C: 388B0DC0  addi r4, r11, 0xdc0
	ctx.r[4].s64 = ctx.r[11].s64 + 3520;
	// 832506A0: 386A7F2C  addi r3, r10, 0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 32556;
	// 832506A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832506A8: 4AFDC829  bl 0x8222ced0
	ctx.lr = 0x832506AC;
	sub_8222CED0(ctx, base);
	// 832506AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832506B0: 38699468  addi r3, r9, -0x6b98
	ctx.r[3].s64 = ctx.r[9].s64 + -27544;
	// 832506B4: 4BA5986D  bl 0x82ca9f20
	ctx.lr = 0x832506B8;
	sub_82CA9F20(ctx, base);
	// 832506B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832506BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832506C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832506C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832506C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832506C8 size=64
    let mut pc: u32 = 0x832506C8;
    'dispatch: loop {
        match pc {
            0x832506C8 => {
    //   block [0x832506C8..0x83250708)
	// 832506C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832506CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832506D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832506D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832506D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832506DC: 388B0DDC  addi r4, r11, 0xddc
	ctx.r[4].s64 = ctx.r[11].s64 + 3548;
	// 832506E0: 386A7F30  addi r3, r10, 0x7f30
	ctx.r[3].s64 = ctx.r[10].s64 + 32560;
	// 832506E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832506E8: 4AFDC7E9  bl 0x8222ced0
	ctx.lr = 0x832506EC;
	sub_8222CED0(ctx, base);
	// 832506EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832506F0: 38699478  addi r3, r9, -0x6b88
	ctx.r[3].s64 = ctx.r[9].s64 + -27528;
	// 832506F4: 4BA5982D  bl 0x82ca9f20
	ctx.lr = 0x832506F8;
	sub_82CA9F20(ctx, base);
	// 832506F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832506FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250708 size=64
    let mut pc: u32 = 0x83250708;
    'dispatch: loop {
        match pc {
            0x83250708 => {
    //   block [0x83250708..0x83250748)
	// 83250708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250714: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250718: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325071C: 388B0DF8  addi r4, r11, 0xdf8
	ctx.r[4].s64 = ctx.r[11].s64 + 3576;
	// 83250720: 386A7F34  addi r3, r10, 0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + 32564;
	// 83250724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250728: 4AFDC7A9  bl 0x8222ced0
	ctx.lr = 0x8325072C;
	sub_8222CED0(ctx, base);
	// 8325072C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250730: 38699488  addi r3, r9, -0x6b78
	ctx.r[3].s64 = ctx.r[9].s64 + -27512;
	// 83250734: 4BA597ED  bl 0x82ca9f20
	ctx.lr = 0x83250738;
	sub_82CA9F20(ctx, base);
	// 83250738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325073C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250748 size=64
    let mut pc: u32 = 0x83250748;
    'dispatch: loop {
        match pc {
            0x83250748 => {
    //   block [0x83250748..0x83250788)
	// 83250748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250754: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250758: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325075C: 388B0E14  addi r4, r11, 0xe14
	ctx.r[4].s64 = ctx.r[11].s64 + 3604;
	// 83250760: 386A7F38  addi r3, r10, 0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + 32568;
	// 83250764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250768: 4AFDC769  bl 0x8222ced0
	ctx.lr = 0x8325076C;
	sub_8222CED0(ctx, base);
	// 8325076C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250770: 38699498  addi r3, r9, -0x6b68
	ctx.r[3].s64 = ctx.r[9].s64 + -27496;
	// 83250774: 4BA597AD  bl 0x82ca9f20
	ctx.lr = 0x83250778;
	sub_82CA9F20(ctx, base);
	// 83250778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325077C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250788 size=64
    let mut pc: u32 = 0x83250788;
    'dispatch: loop {
        match pc {
            0x83250788 => {
    //   block [0x83250788..0x832507C8)
	// 83250788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325078C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250794: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250798: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325079C: 388BE190  addi r4, r11, -0x1e70
	ctx.r[4].s64 = ctx.r[11].s64 + -7792;
	// 832507A0: 386A7F3C  addi r3, r10, 0x7f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 32572;
	// 832507A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832507A8: 4AFDC729  bl 0x8222ced0
	ctx.lr = 0x832507AC;
	sub_8222CED0(ctx, base);
	// 832507AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832507B0: 386994A8  addi r3, r9, -0x6b58
	ctx.r[3].s64 = ctx.r[9].s64 + -27480;
	// 832507B4: 4BA5976D  bl 0x82ca9f20
	ctx.lr = 0x832507B8;
	sub_82CA9F20(ctx, base);
	// 832507B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832507BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832507C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832507C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832507C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832507C8 size=64
    let mut pc: u32 = 0x832507C8;
    'dispatch: loop {
        match pc {
            0x832507C8 => {
    //   block [0x832507C8..0x83250808)
	// 832507C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832507CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832507D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832507D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832507D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832507DC: 388B0E28  addi r4, r11, 0xe28
	ctx.r[4].s64 = ctx.r[11].s64 + 3624;
	// 832507E0: 386A7F40  addi r3, r10, 0x7f40
	ctx.r[3].s64 = ctx.r[10].s64 + 32576;
	// 832507E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832507E8: 4AFDC6E9  bl 0x8222ced0
	ctx.lr = 0x832507EC;
	sub_8222CED0(ctx, base);
	// 832507EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832507F0: 386994B8  addi r3, r9, -0x6b48
	ctx.r[3].s64 = ctx.r[9].s64 + -27464;
	// 832507F4: 4BA5972D  bl 0x82ca9f20
	ctx.lr = 0x832507F8;
	sub_82CA9F20(ctx, base);
	// 832507F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832507FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250808 size=64
    let mut pc: u32 = 0x83250808;
    'dispatch: loop {
        match pc {
            0x83250808 => {
    //   block [0x83250808..0x83250848)
	// 83250808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250814: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250818: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325081C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83250820: 386A7F44  addi r3, r10, 0x7f44
	ctx.r[3].s64 = ctx.r[10].s64 + 32580;
	// 83250824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250828: 4AFDC6A9  bl 0x8222ced0
	ctx.lr = 0x8325082C;
	sub_8222CED0(ctx, base);
	// 8325082C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250830: 386994C8  addi r3, r9, -0x6b38
	ctx.r[3].s64 = ctx.r[9].s64 + -27448;
	// 83250834: 4BA596ED  bl 0x82ca9f20
	ctx.lr = 0x83250838;
	sub_82CA9F20(ctx, base);
	// 83250838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325083C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250848 size=64
    let mut pc: u32 = 0x83250848;
    'dispatch: loop {
        match pc {
            0x83250848 => {
    //   block [0x83250848..0x83250888)
	// 83250848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250854: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250858: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325085C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83250860: 386A7F48  addi r3, r10, 0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + 32584;
	// 83250864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250868: 4AFDC669  bl 0x8222ced0
	ctx.lr = 0x8325086C;
	sub_8222CED0(ctx, base);
	// 8325086C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250870: 386994F8  addi r3, r9, -0x6b08
	ctx.r[3].s64 = ctx.r[9].s64 + -27400;
	// 83250874: 4BA596AD  bl 0x82ca9f20
	ctx.lr = 0x83250878;
	sub_82CA9F20(ctx, base);
	// 83250878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325087C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250888 size=64
    let mut pc: u32 = 0x83250888;
    'dispatch: loop {
        match pc {
            0x83250888 => {
    //   block [0x83250888..0x832508C8)
	// 83250888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250894: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250898: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325089C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832508A0: 386A7F4C  addi r3, r10, 0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32588;
	// 832508A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832508A8: 4AFDC629  bl 0x8222ced0
	ctx.lr = 0x832508AC;
	sub_8222CED0(ctx, base);
	// 832508AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832508B0: 38699508  addi r3, r9, -0x6af8
	ctx.r[3].s64 = ctx.r[9].s64 + -27384;
	// 832508B4: 4BA5966D  bl 0x82ca9f20
	ctx.lr = 0x832508B8;
	sub_82CA9F20(ctx, base);
	// 832508B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832508BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832508C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832508C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832508C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832508C8 size=64
    let mut pc: u32 = 0x832508C8;
    'dispatch: loop {
        match pc {
            0x832508C8 => {
    //   block [0x832508C8..0x83250908)
	// 832508C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832508CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832508D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832508D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832508D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832508DC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832508E0: 386A7F50  addi r3, r10, 0x7f50
	ctx.r[3].s64 = ctx.r[10].s64 + 32592;
	// 832508E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832508E8: 4AFDC5E9  bl 0x8222ced0
	ctx.lr = 0x832508EC;
	sub_8222CED0(ctx, base);
	// 832508EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832508F0: 38699518  addi r3, r9, -0x6ae8
	ctx.r[3].s64 = ctx.r[9].s64 + -27368;
	// 832508F4: 4BA5962D  bl 0x82ca9f20
	ctx.lr = 0x832508F8;
	sub_82CA9F20(ctx, base);
	// 832508F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832508FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250908 size=64
    let mut pc: u32 = 0x83250908;
    'dispatch: loop {
        match pc {
            0x83250908 => {
    //   block [0x83250908..0x83250948)
	// 83250908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250918: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325091C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83250920: 386A7F54  addi r3, r10, 0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + 32596;
	// 83250924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250928: 4AFDC5A9  bl 0x8222ced0
	ctx.lr = 0x8325092C;
	sub_8222CED0(ctx, base);
	// 8325092C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250930: 38699528  addi r3, r9, -0x6ad8
	ctx.r[3].s64 = ctx.r[9].s64 + -27352;
	// 83250934: 4BA595ED  bl 0x82ca9f20
	ctx.lr = 0x83250938;
	sub_82CA9F20(ctx, base);
	// 83250938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325093C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250948 size=64
    let mut pc: u32 = 0x83250948;
    'dispatch: loop {
        match pc {
            0x83250948 => {
    //   block [0x83250948..0x83250988)
	// 83250948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325094C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250958: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325095C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83250960: 386A7F58  addi r3, r10, 0x7f58
	ctx.r[3].s64 = ctx.r[10].s64 + 32600;
	// 83250964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250968: 4AFDC569  bl 0x8222ced0
	ctx.lr = 0x8325096C;
	sub_8222CED0(ctx, base);
	// 8325096C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250970: 38699538  addi r3, r9, -0x6ac8
	ctx.r[3].s64 = ctx.r[9].s64 + -27336;
	// 83250974: 4BA595AD  bl 0x82ca9f20
	ctx.lr = 0x83250978;
	sub_82CA9F20(ctx, base);
	// 83250978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325097C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250988 size=64
    let mut pc: u32 = 0x83250988;
    'dispatch: loop {
        match pc {
            0x83250988 => {
    //   block [0x83250988..0x832509C8)
	// 83250988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250998: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325099C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832509A0: 386A7F5C  addi r3, r10, 0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32604;
	// 832509A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832509A8: 4AFDC529  bl 0x8222ced0
	ctx.lr = 0x832509AC;
	sub_8222CED0(ctx, base);
	// 832509AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832509B0: 38699548  addi r3, r9, -0x6ab8
	ctx.r[3].s64 = ctx.r[9].s64 + -27320;
	// 832509B4: 4BA5956D  bl 0x82ca9f20
	ctx.lr = 0x832509B8;
	sub_82CA9F20(ctx, base);
	// 832509B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832509BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832509C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832509C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832509C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832509C8 size=52
    let mut pc: u32 = 0x832509C8;
    'dispatch: loop {
        match pc {
            0x832509C8 => {
    //   block [0x832509C8..0x832509FC)
	// 832509C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832509CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832509D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832509D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832509D8: C82B1A08  lfd f1, 0x1a08(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(6664 as u32) ) };
	// 832509DC: 4AFE94D5  bl 0x82239eb0
	ctx.lr = 0x832509E0;
	sub_82239EB0(ctx, base);
	// 832509E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832509E4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832509E8: D00A7F60  stfs f0, 0x7f60(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32608 as u32), tmp.u32 ) };
	// 832509EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832509F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832509F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832509F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250A00 size=64
    let mut pc: u32 = 0x83250A00;
    'dispatch: loop {
        match pc {
            0x83250A00 => {
    //   block [0x83250A00..0x83250A40)
	// 83250A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250A0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250A10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250A14: 388B2090  addi r4, r11, 0x2090
	ctx.r[4].s64 = ctx.r[11].s64 + 8336;
	// 83250A18: 386A7F64  addi r3, r10, 0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + 32612;
	// 83250A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250A20: 4AFDC4B1  bl 0x8222ced0
	ctx.lr = 0x83250A24;
	sub_8222CED0(ctx, base);
	// 83250A24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250A28: 38699558  addi r3, r9, -0x6aa8
	ctx.r[3].s64 = ctx.r[9].s64 + -27304;
	// 83250A2C: 4BA594F5  bl 0x82ca9f20
	ctx.lr = 0x83250A30;
	sub_82CA9F20(ctx, base);
	// 83250A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250A40 size=144
    let mut pc: u32 = 0x83250A40;
    'dispatch: loop {
        match pc {
            0x83250A40 => {
    //   block [0x83250A40..0x83250A64)
	// 83250A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250A4C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83250A50: 4AFCE809  bl 0x8221f258
	ctx.lr = 0x83250A54;
	sub_8221F258(ctx, base);
	// 83250A54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83250A58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83250A5C: 419A0008  beq cr6, 0x83250a64
	if ctx.cr[6].eq {
	pc = 0x83250A64; continue 'dispatch;
	}
	// 83250A60: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83250A64; continue 'dispatch;
            }
            0x83250A64 => {
    //   block [0x83250A64..0x83250A70)
	// 83250A64: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83250A68: 41820008  beq 0x83250a70
	if ctx.cr[0].eq {
	pc = 0x83250A70; continue 'dispatch;
	}
	// 83250A6C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83250A70; continue 'dispatch;
            }
            0x83250A70 => {
    //   block [0x83250A70..0x83250A7C)
	// 83250A70: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83250A74: 41820008  beq 0x83250a7c
	if ctx.cr[0].eq {
	pc = 0x83250A7C; continue 'dispatch;
	}
	// 83250A78: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83250A7C; continue 'dispatch;
            }
            0x83250A7C => {
    //   block [0x83250A7C..0x83250AD0)
	// 83250A7C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83250A80: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83250A84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83250A88: 39097F68  addi r8, r9, 0x7f68
	ctx.r[8].s64 = ctx.r[9].s64 + 32616;
	// 83250A8C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83250A90: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83250A94: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83250A98: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83250A9C: 38679568  addi r3, r7, -0x6a98
	ctx.r[3].s64 = ctx.r[7].s64 + -27288;
	// 83250AA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83250AA4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83250AA8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83250AAC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83250AB0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83250AB4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83250AB8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83250ABC: 4BA59465  bl 0x82ca9f20
	ctx.lr = 0x83250AC0;
	sub_82CA9F20(ctx, base);
	// 83250AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83250AD0 size=96
    let mut pc: u32 = 0x83250AD0;
    'dispatch: loop {
        match pc {
            0x83250AD0 => {
    //   block [0x83250AD0..0x83250B30)
	// 83250AD0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250AD4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83250AD8: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 83250ADC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83250AE0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83250AE4: C00B92D4  lfs f0, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83250AE8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83250AEC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83250AF0: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250B30 size=64
    let mut pc: u32 = 0x83250B30;
    'dispatch: loop {
        match pc {
            0x83250B30 => {
    //   block [0x83250B30..0x83250B70)
	// 83250B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250B3C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250B40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250B44: 388B239C  addi r4, r11, 0x239c
	ctx.r[4].s64 = ctx.r[11].s64 + 9116;
	// 83250B48: 386A7F74  addi r3, r10, 0x7f74
	ctx.r[3].s64 = ctx.r[10].s64 + 32628;
	// 83250B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250B50: 4AFDC381  bl 0x8222ced0
	ctx.lr = 0x83250B54;
	sub_8222CED0(ctx, base);
	// 83250B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250B58: 38699578  addi r3, r9, -0x6a88
	ctx.r[3].s64 = ctx.r[9].s64 + -27272;
	// 83250B5C: 4BA593C5  bl 0x82ca9f20
	ctx.lr = 0x83250B60;
	sub_82CA9F20(ctx, base);
	// 83250B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250B70 size=64
    let mut pc: u32 = 0x83250B70;
    'dispatch: loop {
        match pc {
            0x83250B70 => {
    //   block [0x83250B70..0x83250BB0)
	// 83250B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250B7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250B80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250B84: 388B23B4  addi r4, r11, 0x23b4
	ctx.r[4].s64 = ctx.r[11].s64 + 9140;
	// 83250B88: 386A7F78  addi r3, r10, 0x7f78
	ctx.r[3].s64 = ctx.r[10].s64 + 32632;
	// 83250B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250B90: 4AFDC341  bl 0x8222ced0
	ctx.lr = 0x83250B94;
	sub_8222CED0(ctx, base);
	// 83250B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250B98: 38699588  addi r3, r9, -0x6a78
	ctx.r[3].s64 = ctx.r[9].s64 + -27256;
	// 83250B9C: 4BA59385  bl 0x82ca9f20
	ctx.lr = 0x83250BA0;
	sub_82CA9F20(ctx, base);
	// 83250BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83250BB0 size=12
    let mut pc: u32 = 0x83250BB0;
    'dispatch: loop {
        match pc {
            0x83250BB0 => {
    //   block [0x83250BB0..0x83250BBC)
	// 83250BB0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83250BB4: 386B9598  addi r3, r11, -0x6a68
	ctx.r[3].s64 = ctx.r[11].s64 + -27240;
	// 83250BB8: 4BA59368  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83250BC0 size=12
    let mut pc: u32 = 0x83250BC0;
    'dispatch: loop {
        match pc {
            0x83250BC0 => {
    //   block [0x83250BC0..0x83250BCC)
	// 83250BC0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83250BC4: 386B95A8  addi r3, r11, -0x6a58
	ctx.r[3].s64 = ctx.r[11].s64 + -27224;
	// 83250BC8: 4BA59358  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250BD0 size=64
    let mut pc: u32 = 0x83250BD0;
    'dispatch: loop {
        match pc {
            0x83250BD0 => {
    //   block [0x83250BD0..0x83250C10)
	// 83250BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250BDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250BE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250BE4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83250BE8: 386A7FB0  addi r3, r10, 0x7fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 32688;
	// 83250BEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250BF0: 4AFDC2E1  bl 0x8222ced0
	ctx.lr = 0x83250BF4;
	sub_8222CED0(ctx, base);
	// 83250BF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250BF8: 386995B8  addi r3, r9, -0x6a48
	ctx.r[3].s64 = ctx.r[9].s64 + -27208;
	// 83250BFC: 4BA59325  bl 0x82ca9f20
	ctx.lr = 0x83250C00;
	sub_82CA9F20(ctx, base);
	// 83250C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250C10 size=64
    let mut pc: u32 = 0x83250C10;
    'dispatch: loop {
        match pc {
            0x83250C10 => {
    //   block [0x83250C10..0x83250C50)
	// 83250C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250C20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250C24: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83250C28: 386A7FB4  addi r3, r10, 0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32692;
	// 83250C2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250C30: 4AFDC2A1  bl 0x8222ced0
	ctx.lr = 0x83250C34;
	sub_8222CED0(ctx, base);
	// 83250C34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250C38: 386995C8  addi r3, r9, -0x6a38
	ctx.r[3].s64 = ctx.r[9].s64 + -27192;
	// 83250C3C: 4BA592E5  bl 0x82ca9f20
	ctx.lr = 0x83250C40;
	sub_82CA9F20(ctx, base);
	// 83250C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250C50 size=64
    let mut pc: u32 = 0x83250C50;
    'dispatch: loop {
        match pc {
            0x83250C50 => {
    //   block [0x83250C50..0x83250C90)
	// 83250C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250C5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250C60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250C64: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83250C68: 386A7FB8  addi r3, r10, 0x7fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32696;
	// 83250C6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250C70: 4AFDC261  bl 0x8222ced0
	ctx.lr = 0x83250C74;
	sub_8222CED0(ctx, base);
	// 83250C74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250C78: 386995D8  addi r3, r9, -0x6a28
	ctx.r[3].s64 = ctx.r[9].s64 + -27176;
	// 83250C7C: 4BA592A5  bl 0x82ca9f20
	ctx.lr = 0x83250C80;
	sub_82CA9F20(ctx, base);
	// 83250C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250C90 size=56
    let mut pc: u32 = 0x83250C90;
    'dispatch: loop {
        match pc {
            0x83250C90 => {
    //   block [0x83250C90..0x83250CC8)
	// 83250C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250C9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250CA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83250CA4: 386B9D2C  addi r3, r11, -0x62d4
	ctx.r[3].s64 = ctx.r[11].s64 + -25300;
	// 83250CA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83250CAC: 4AFA30AD  bl 0x821f3d58
	ctx.lr = 0x83250CB0;
	sub_821F3D58(ctx, base);
	// 83250CB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250CB4: 906A7FBC  stw r3, 0x7fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32700 as u32), ctx.r[3].u32 ) };
	// 83250CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250CC8 size=64
    let mut pc: u32 = 0x83250CC8;
    'dispatch: loop {
        match pc {
            0x83250CC8 => {
    //   block [0x83250CC8..0x83250D08)
	// 83250CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250CD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250CD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250CDC: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 83250CE0: 386A7FC0  addi r3, r10, 0x7fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 32704;
	// 83250CE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250CE8: 4AFDC1E9  bl 0x8222ced0
	ctx.lr = 0x83250CEC;
	sub_8222CED0(ctx, base);
	// 83250CEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250CF0: 386995E8  addi r3, r9, -0x6a18
	ctx.r[3].s64 = ctx.r[9].s64 + -27160;
	// 83250CF4: 4BA5922D  bl 0x82ca9f20
	ctx.lr = 0x83250CF8;
	sub_82CA9F20(ctx, base);
	// 83250CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250D08 size=64
    let mut pc: u32 = 0x83250D08;
    'dispatch: loop {
        match pc {
            0x83250D08 => {
    //   block [0x83250D08..0x83250D48)
	// 83250D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250D14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250D18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250D1C: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 83250D20: 386A7FC4  addi r3, r10, 0x7fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32708;
	// 83250D24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250D28: 4AFDC1A9  bl 0x8222ced0
	ctx.lr = 0x83250D2C;
	sub_8222CED0(ctx, base);
	// 83250D2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250D30: 386995F8  addi r3, r9, -0x6a08
	ctx.r[3].s64 = ctx.r[9].s64 + -27144;
	// 83250D34: 4BA591ED  bl 0x82ca9f20
	ctx.lr = 0x83250D38;
	sub_82CA9F20(ctx, base);
	// 83250D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250D48 size=64
    let mut pc: u32 = 0x83250D48;
    'dispatch: loop {
        match pc {
            0x83250D48 => {
    //   block [0x83250D48..0x83250D88)
	// 83250D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250D54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250D58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250D5C: 388B28E4  addi r4, r11, 0x28e4
	ctx.r[4].s64 = ctx.r[11].s64 + 10468;
	// 83250D60: 386A7FC8  addi r3, r10, 0x7fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 32712;
	// 83250D64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250D68: 4AFDC169  bl 0x8222ced0
	ctx.lr = 0x83250D6C;
	sub_8222CED0(ctx, base);
	// 83250D6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250D70: 38699608  addi r3, r9, -0x69f8
	ctx.r[3].s64 = ctx.r[9].s64 + -27128;
	// 83250D74: 4BA591AD  bl 0x82ca9f20
	ctx.lr = 0x83250D78;
	sub_82CA9F20(ctx, base);
	// 83250D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250D88 size=64
    let mut pc: u32 = 0x83250D88;
    'dispatch: loop {
        match pc {
            0x83250D88 => {
    //   block [0x83250D88..0x83250DC8)
	// 83250D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250D94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250D98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250D9C: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 83250DA0: 386A7FCC  addi r3, r10, 0x7fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 32716;
	// 83250DA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250DA8: 4AFDC129  bl 0x8222ced0
	ctx.lr = 0x83250DAC;
	sub_8222CED0(ctx, base);
	// 83250DAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250DB0: 38699618  addi r3, r9, -0x69e8
	ctx.r[3].s64 = ctx.r[9].s64 + -27112;
	// 83250DB4: 4BA5916D  bl 0x82ca9f20
	ctx.lr = 0x83250DB8;
	sub_82CA9F20(ctx, base);
	// 83250DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250DC8 size=64
    let mut pc: u32 = 0x83250DC8;
    'dispatch: loop {
        match pc {
            0x83250DC8 => {
    //   block [0x83250DC8..0x83250E08)
	// 83250DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250DD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250DD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250DDC: 388B28F8  addi r4, r11, 0x28f8
	ctx.r[4].s64 = ctx.r[11].s64 + 10488;
	// 83250DE0: 386A7FD0  addi r3, r10, 0x7fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 32720;
	// 83250DE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250DE8: 4AFDC0E9  bl 0x8222ced0
	ctx.lr = 0x83250DEC;
	sub_8222CED0(ctx, base);
	// 83250DEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250DF0: 38699628  addi r3, r9, -0x69d8
	ctx.r[3].s64 = ctx.r[9].s64 + -27096;
	// 83250DF4: 4BA5912D  bl 0x82ca9f20
	ctx.lr = 0x83250DF8;
	sub_82CA9F20(ctx, base);
	// 83250DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250E08 size=64
    let mut pc: u32 = 0x83250E08;
    'dispatch: loop {
        match pc {
            0x83250E08 => {
    //   block [0x83250E08..0x83250E48)
	// 83250E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250E14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250E18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250E1C: 388B2904  addi r4, r11, 0x2904
	ctx.r[4].s64 = ctx.r[11].s64 + 10500;
	// 83250E20: 386A7FD4  addi r3, r10, 0x7fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32724;
	// 83250E24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250E28: 4AFDC0A9  bl 0x8222ced0
	ctx.lr = 0x83250E2C;
	sub_8222CED0(ctx, base);
	// 83250E2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250E30: 38699638  addi r3, r9, -0x69c8
	ctx.r[3].s64 = ctx.r[9].s64 + -27080;
	// 83250E34: 4BA590ED  bl 0x82ca9f20
	ctx.lr = 0x83250E38;
	sub_82CA9F20(ctx, base);
	// 83250E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250E48 size=64
    let mut pc: u32 = 0x83250E48;
    'dispatch: loop {
        match pc {
            0x83250E48 => {
    //   block [0x83250E48..0x83250E88)
	// 83250E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250E54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250E58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250E5C: 388B2914  addi r4, r11, 0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + 10516;
	// 83250E60: 386A7FD8  addi r3, r10, 0x7fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32728;
	// 83250E64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250E68: 4AFDC069  bl 0x8222ced0
	ctx.lr = 0x83250E6C;
	sub_8222CED0(ctx, base);
	// 83250E6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250E70: 38699648  addi r3, r9, -0x69b8
	ctx.r[3].s64 = ctx.r[9].s64 + -27064;
	// 83250E74: 4BA590AD  bl 0x82ca9f20
	ctx.lr = 0x83250E78;
	sub_82CA9F20(ctx, base);
	// 83250E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250E88 size=64
    let mut pc: u32 = 0x83250E88;
    'dispatch: loop {
        match pc {
            0x83250E88 => {
    //   block [0x83250E88..0x83250EC8)
	// 83250E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250E94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250E98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250E9C: 388B291C  addi r4, r11, 0x291c
	ctx.r[4].s64 = ctx.r[11].s64 + 10524;
	// 83250EA0: 386A7FDC  addi r3, r10, 0x7fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 32732;
	// 83250EA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250EA8: 4AFDC029  bl 0x8222ced0
	ctx.lr = 0x83250EAC;
	sub_8222CED0(ctx, base);
	// 83250EAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250EB0: 38699658  addi r3, r9, -0x69a8
	ctx.r[3].s64 = ctx.r[9].s64 + -27048;
	// 83250EB4: 4BA5906D  bl 0x82ca9f20
	ctx.lr = 0x83250EB8;
	sub_82CA9F20(ctx, base);
	// 83250EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250EC8 size=64
    let mut pc: u32 = 0x83250EC8;
    'dispatch: loop {
        match pc {
            0x83250EC8 => {
    //   block [0x83250EC8..0x83250F08)
	// 83250EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250ED4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250ED8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250EDC: 388B292C  addi r4, r11, 0x292c
	ctx.r[4].s64 = ctx.r[11].s64 + 10540;
	// 83250EE0: 386A7FE0  addi r3, r10, 0x7fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 32736;
	// 83250EE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250EE8: 4AFDBFE9  bl 0x8222ced0
	ctx.lr = 0x83250EEC;
	sub_8222CED0(ctx, base);
	// 83250EEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250EF0: 38699668  addi r3, r9, -0x6998
	ctx.r[3].s64 = ctx.r[9].s64 + -27032;
	// 83250EF4: 4BA5902D  bl 0x82ca9f20
	ctx.lr = 0x83250EF8;
	sub_82CA9F20(ctx, base);
	// 83250EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250F08 size=64
    let mut pc: u32 = 0x83250F08;
    'dispatch: loop {
        match pc {
            0x83250F08 => {
    //   block [0x83250F08..0x83250F48)
	// 83250F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250F14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250F18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250F1C: 388B2934  addi r4, r11, 0x2934
	ctx.r[4].s64 = ctx.r[11].s64 + 10548;
	// 83250F20: 386A7FE4  addi r3, r10, 0x7fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 32740;
	// 83250F24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250F28: 4AFDBFA9  bl 0x8222ced0
	ctx.lr = 0x83250F2C;
	sub_8222CED0(ctx, base);
	// 83250F2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250F30: 38699678  addi r3, r9, -0x6988
	ctx.r[3].s64 = ctx.r[9].s64 + -27016;
	// 83250F34: 4BA58FED  bl 0x82ca9f20
	ctx.lr = 0x83250F38;
	sub_82CA9F20(ctx, base);
	// 83250F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250F48 size=64
    let mut pc: u32 = 0x83250F48;
    'dispatch: loop {
        match pc {
            0x83250F48 => {
    //   block [0x83250F48..0x83250F88)
	// 83250F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250F54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250F58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250F5C: 388B293C  addi r4, r11, 0x293c
	ctx.r[4].s64 = ctx.r[11].s64 + 10556;
	// 83250F60: 386A7FE8  addi r3, r10, 0x7fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 32744;
	// 83250F64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250F68: 4AFDBF69  bl 0x8222ced0
	ctx.lr = 0x83250F6C;
	sub_8222CED0(ctx, base);
	// 83250F6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250F70: 38699688  addi r3, r9, -0x6978
	ctx.r[3].s64 = ctx.r[9].s64 + -27000;
	// 83250F74: 4BA58FAD  bl 0x82ca9f20
	ctx.lr = 0x83250F78;
	sub_82CA9F20(ctx, base);
	// 83250F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250F88 size=64
    let mut pc: u32 = 0x83250F88;
    'dispatch: loop {
        match pc {
            0x83250F88 => {
    //   block [0x83250F88..0x83250FC8)
	// 83250F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250F94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250F98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250F9C: 388B2948  addi r4, r11, 0x2948
	ctx.r[4].s64 = ctx.r[11].s64 + 10568;
	// 83250FA0: 386A7FEC  addi r3, r10, 0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + 32748;
	// 83250FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250FA8: 4AFDBF29  bl 0x8222ced0
	ctx.lr = 0x83250FAC;
	sub_8222CED0(ctx, base);
	// 83250FAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250FB0: 38699698  addi r3, r9, -0x6968
	ctx.r[3].s64 = ctx.r[9].s64 + -26984;
	// 83250FB4: 4BA58F6D  bl 0x82ca9f20
	ctx.lr = 0x83250FB8;
	sub_82CA9F20(ctx, base);
	// 83250FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250FC8 size=64
    let mut pc: u32 = 0x83250FC8;
    'dispatch: loop {
        match pc {
            0x83250FC8 => {
    //   block [0x83250FC8..0x83251008)
	// 83250FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250FD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250FD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250FDC: 388B2950  addi r4, r11, 0x2950
	ctx.r[4].s64 = ctx.r[11].s64 + 10576;
	// 83250FE0: 386A7FF0  addi r3, r10, 0x7ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 32752;
	// 83250FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250FE8: 4AFDBEE9  bl 0x8222ced0
	ctx.lr = 0x83250FEC;
	sub_8222CED0(ctx, base);
	// 83250FEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250FF0: 386996A8  addi r3, r9, -0x6958
	ctx.r[3].s64 = ctx.r[9].s64 + -26968;
	// 83250FF4: 4BA58F2D  bl 0x82ca9f20
	ctx.lr = 0x83250FF8;
	sub_82CA9F20(ctx, base);
	// 83250FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251008 size=64
    let mut pc: u32 = 0x83251008;
    'dispatch: loop {
        match pc {
            0x83251008 => {
    //   block [0x83251008..0x83251048)
	// 83251008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325100C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251014: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251018: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325101C: 388B2958  addi r4, r11, 0x2958
	ctx.r[4].s64 = ctx.r[11].s64 + 10584;
	// 83251020: 386A7FF4  addi r3, r10, 0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 32756;
	// 83251024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251028: 4AFDBEA9  bl 0x8222ced0
	ctx.lr = 0x8325102C;
	sub_8222CED0(ctx, base);
	// 8325102C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251030: 386996B8  addi r3, r9, -0x6948
	ctx.r[3].s64 = ctx.r[9].s64 + -26952;
	// 83251034: 4BA58EED  bl 0x82ca9f20
	ctx.lr = 0x83251038;
	sub_82CA9F20(ctx, base);
	// 83251038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325103C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251048 size=64
    let mut pc: u32 = 0x83251048;
    'dispatch: loop {
        match pc {
            0x83251048 => {
    //   block [0x83251048..0x83251088)
	// 83251048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251054: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251058: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325105C: 388B2960  addi r4, r11, 0x2960
	ctx.r[4].s64 = ctx.r[11].s64 + 10592;
	// 83251060: 386A7FF8  addi r3, r10, 0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + 32760;
	// 83251064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251068: 4AFDBE69  bl 0x8222ced0
	ctx.lr = 0x8325106C;
	sub_8222CED0(ctx, base);
	// 8325106C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251070: 386996C8  addi r3, r9, -0x6938
	ctx.r[3].s64 = ctx.r[9].s64 + -26936;
	// 83251074: 4BA58EAD  bl 0x82ca9f20
	ctx.lr = 0x83251078;
	sub_82CA9F20(ctx, base);
	// 83251078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325107C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251088 size=64
    let mut pc: u32 = 0x83251088;
    'dispatch: loop {
        match pc {
            0x83251088 => {
    //   block [0x83251088..0x832510C8)
	// 83251088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325108C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251094: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251098: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325109C: 388B2968  addi r4, r11, 0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + 10600;
	// 832510A0: 386A7FFC  addi r3, r10, 0x7ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 32764;
	// 832510A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832510A8: 4AFDBE29  bl 0x8222ced0
	ctx.lr = 0x832510AC;
	sub_8222CED0(ctx, base);
	// 832510AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832510B0: 386996D8  addi r3, r9, -0x6928
	ctx.r[3].s64 = ctx.r[9].s64 + -26920;
	// 832510B4: 4BA58E6D  bl 0x82ca9f20
	ctx.lr = 0x832510B8;
	sub_82CA9F20(ctx, base);
	// 832510B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832510BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832510C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832510C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832510C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832510C8 size=64
    let mut pc: u32 = 0x832510C8;
    'dispatch: loop {
        match pc {
            0x832510C8 => {
    //   block [0x832510C8..0x83251108)
	// 832510C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832510CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832510D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832510D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832510D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832510DC: 388B2970  addi r4, r11, 0x2970
	ctx.r[4].s64 = ctx.r[11].s64 + 10608;
	// 832510E0: 386A8000  addi r3, r10, -0x8000
	ctx.r[3].s64 = ctx.r[10].s64 + -32768;
	// 832510E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832510E8: 4AFDBDE9  bl 0x8222ced0
	ctx.lr = 0x832510EC;
	sub_8222CED0(ctx, base);
	// 832510EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832510F0: 386996E8  addi r3, r9, -0x6918
	ctx.r[3].s64 = ctx.r[9].s64 + -26904;
	// 832510F4: 4BA58E2D  bl 0x82ca9f20
	ctx.lr = 0x832510F8;
	sub_82CA9F20(ctx, base);
	// 832510F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832510FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251108 size=64
    let mut pc: u32 = 0x83251108;
    'dispatch: loop {
        match pc {
            0x83251108 => {
    //   block [0x83251108..0x83251148)
	// 83251108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251114: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325111C: 388B2978  addi r4, r11, 0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + 10616;
	// 83251120: 386A8004  addi r3, r10, -0x7ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -32764;
	// 83251124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251128: 4AFDBDA9  bl 0x8222ced0
	ctx.lr = 0x8325112C;
	sub_8222CED0(ctx, base);
	// 8325112C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251130: 386996F8  addi r3, r9, -0x6908
	ctx.r[3].s64 = ctx.r[9].s64 + -26888;
	// 83251134: 4BA58DED  bl 0x82ca9f20
	ctx.lr = 0x83251138;
	sub_82CA9F20(ctx, base);
	// 83251138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325113C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251148 size=64
    let mut pc: u32 = 0x83251148;
    'dispatch: loop {
        match pc {
            0x83251148 => {
    //   block [0x83251148..0x83251188)
	// 83251148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251154: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325115C: 388B2980  addi r4, r11, 0x2980
	ctx.r[4].s64 = ctx.r[11].s64 + 10624;
	// 83251160: 386A8008  addi r3, r10, -0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -32760;
	// 83251164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251168: 4AFDBD69  bl 0x8222ced0
	ctx.lr = 0x8325116C;
	sub_8222CED0(ctx, base);
	// 8325116C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251170: 38699708  addi r3, r9, -0x68f8
	ctx.r[3].s64 = ctx.r[9].s64 + -26872;
	// 83251174: 4BA58DAD  bl 0x82ca9f20
	ctx.lr = 0x83251178;
	sub_82CA9F20(ctx, base);
	// 83251178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251188 size=64
    let mut pc: u32 = 0x83251188;
    'dispatch: loop {
        match pc {
            0x83251188 => {
    //   block [0x83251188..0x832511C8)
	// 83251188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325118C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251194: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325119C: 388B298C  addi r4, r11, 0x298c
	ctx.r[4].s64 = ctx.r[11].s64 + 10636;
	// 832511A0: 386A800C  addi r3, r10, -0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -32756;
	// 832511A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832511A8: 4AFDBD29  bl 0x8222ced0
	ctx.lr = 0x832511AC;
	sub_8222CED0(ctx, base);
	// 832511AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832511B0: 38699718  addi r3, r9, -0x68e8
	ctx.r[3].s64 = ctx.r[9].s64 + -26856;
	// 832511B4: 4BA58D6D  bl 0x82ca9f20
	ctx.lr = 0x832511B8;
	sub_82CA9F20(ctx, base);
	// 832511B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832511BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832511C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832511C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832511C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832511C8 size=64
    let mut pc: u32 = 0x832511C8;
    'dispatch: loop {
        match pc {
            0x832511C8 => {
    //   block [0x832511C8..0x83251208)
	// 832511C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832511CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832511D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832511D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832511D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832511DC: 388B8CD0  addi r4, r11, -0x7330
	ctx.r[4].s64 = ctx.r[11].s64 + -29488;
	// 832511E0: 386A8010  addi r3, r10, -0x7ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -32752;
	// 832511E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832511E8: 4AFDBCE9  bl 0x8222ced0
	ctx.lr = 0x832511EC;
	sub_8222CED0(ctx, base);
	// 832511EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832511F0: 38699728  addi r3, r9, -0x68d8
	ctx.r[3].s64 = ctx.r[9].s64 + -26840;
	// 832511F4: 4BA58D2D  bl 0x82ca9f20
	ctx.lr = 0x832511F8;
	sub_82CA9F20(ctx, base);
	// 832511F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832511FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251208 size=64
    let mut pc: u32 = 0x83251208;
    'dispatch: loop {
        match pc {
            0x83251208 => {
    //   block [0x83251208..0x83251248)
	// 83251208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325120C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251214: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325121C: 388B2994  addi r4, r11, 0x2994
	ctx.r[4].s64 = ctx.r[11].s64 + 10644;
	// 83251220: 386A8014  addi r3, r10, -0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + -32748;
	// 83251224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251228: 4AFDBCA9  bl 0x8222ced0
	ctx.lr = 0x8325122C;
	sub_8222CED0(ctx, base);
	// 8325122C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251230: 38699738  addi r3, r9, -0x68c8
	ctx.r[3].s64 = ctx.r[9].s64 + -26824;
	// 83251234: 4BA58CED  bl 0x82ca9f20
	ctx.lr = 0x83251238;
	sub_82CA9F20(ctx, base);
	// 83251238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325123C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251248 size=56
    let mut pc: u32 = 0x83251248;
    'dispatch: loop {
        match pc {
            0x83251248 => {
    //   block [0x83251248..0x83251280)
	// 83251248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325124C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251254: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251258: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325125C: 386B0478  addi r3, r11, 0x478
	ctx.r[3].s64 = ctx.r[11].s64 + 1144;
	// 83251260: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251264: 4AFA2AF5  bl 0x821f3d58
	ctx.lr = 0x83251268;
	sub_821F3D58(ctx, base);
	// 83251268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325126C: 906A8018  stw r3, -0x7fe8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32744 as u32), ctx.r[3].u32 ) };
	// 83251270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83251280 size=12
    let mut pc: u32 = 0x83251280;
    'dispatch: loop {
        match pc {
            0x83251280 => {
    //   block [0x83251280..0x8325128C)
	// 83251280: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251284: 386B9748  addi r3, r11, -0x68b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26808;
	// 83251288: 4BA58C98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251290 size=192
    let mut pc: u32 = 0x83251290;
    'dispatch: loop {
        match pc {
            0x83251290 => {
    //   block [0x83251290..0x832512E8)
	// 83251290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325129C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832512A0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832512A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832512A8: 388B2EF0  addi r4, r11, 0x2ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 12016;
	// 832512AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832512B0: 4AFDBC21  bl 0x8222ced0
	ctx.lr = 0x832512B4;
	sub_8222CED0(ctx, base);
	// 832512B4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832512B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832512BC: 4AF9D87D  bl 0x821eeb38
	ctx.lr = 0x832512C0;
	sub_821EEB38(ctx, base);
	// 832512C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832512C4: 4B9B252D  bl 0x82c037f0
	ctx.lr = 0x832512C8;
	sub_82C037F0(ctx, base);
	// 832512C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832512CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832512D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832512D4: 916A802C  stw r11, -0x7fd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32724 as u32), ctx.r[11].u32 ) };
	// 832512D8: 4AF75491  bl 0x821c6768
	ctx.lr = 0x832512DC;
	sub_821C6768(ctx, base);
	// 832512DC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832512E0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832512E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832512E8; continue 'dispatch;
            }
            0x832512E8 => {
    //   block [0x832512E8..0x83251314)
	// 832512E8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832512EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832512F0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832512F4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832512F8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832512FC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251300: 4082FFE8  bne 0x832512e8
	if !ctx.cr[0].eq {
	pc = 0x832512E8; continue 'dispatch;
	}
	// 83251304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251308: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325130C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251310: 4AF75459  bl 0x821c6768
	ctx.lr = 0x83251314;
	sub_821C6768(ctx, base);
	pc = 0x83251314; continue 'dispatch;
            }
            0x83251314 => {
    //   block [0x83251314..0x83251350)
	// 83251314: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251318: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325131C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83251320: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83251324: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83251328: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325132C: 4082FFE8  bne 0x83251314
	if !ctx.cr[0].eq {
	pc = 0x83251314; continue 'dispatch;
	}
	// 83251330: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251334: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 83251338: 4BA58BE9  bl 0x82ca9f20
	ctx.lr = 0x8325133C;
	sub_82CA9F20(ctx, base);
	// 8325133C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325134C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251350 size=192
    let mut pc: u32 = 0x83251350;
    'dispatch: loop {
        match pc {
            0x83251350 => {
    //   block [0x83251350..0x832513A8)
	// 83251350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325135C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251360: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251368: 388B2EF0  addi r4, r11, 0x2ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 12016;
	// 8325136C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83251370: 4AFDBB61  bl 0x8222ced0
	ctx.lr = 0x83251374;
	sub_8222CED0(ctx, base);
	// 83251374: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83251378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325137C: 4AF9D7BD  bl 0x821eeb38
	ctx.lr = 0x83251380;
	sub_821EEB38(ctx, base);
	// 83251380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251384: 4B9B246D  bl 0x82c037f0
	ctx.lr = 0x83251388;
	sub_82C037F0(ctx, base);
	// 83251388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325138C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251394: 916A8030  stw r11, -0x7fd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32720 as u32), ctx.r[11].u32 ) };
	// 83251398: 4AF753D1  bl 0x821c6768
	ctx.lr = 0x8325139C;
	sub_821C6768(ctx, base);
	// 8325139C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832513A0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832513A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832513A8; continue 'dispatch;
            }
            0x832513A8 => {
    //   block [0x832513A8..0x832513D4)
	// 832513A8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832513AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513B0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832513B4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832513B8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832513BC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513C0: 4082FFE8  bne 0x832513a8
	if !ctx.cr[0].eq {
	pc = 0x832513A8; continue 'dispatch;
	}
	// 832513C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832513C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832513CC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832513D0: 4AF75399  bl 0x821c6768
	ctx.lr = 0x832513D4;
	sub_821C6768(ctx, base);
	pc = 0x832513D4; continue 'dispatch;
            }
            0x832513D4 => {
    //   block [0x832513D4..0x83251410)
	// 832513D4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 832513D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513DC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832513E0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832513E4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832513E8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513EC: 4082FFE8  bne 0x832513d4
	if !ctx.cr[0].eq {
	pc = 0x832513D4; continue 'dispatch;
	}
	// 832513F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832513F4: 386B9760  addi r3, r11, -0x68a0
	ctx.r[3].s64 = ctx.r[11].s64 + -26784;
	// 832513F8: 4BA58B29  bl 0x82ca9f20
	ctx.lr = 0x832513FC;
	sub_82CA9F20(ctx, base);
	// 832513FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325140C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251410 size=192
    let mut pc: u32 = 0x83251410;
    'dispatch: loop {
        match pc {
            0x83251410 => {
    //   block [0x83251410..0x83251468)
	// 83251410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325141C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251420: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251428: 388B2F14  addi r4, r11, 0x2f14
	ctx.r[4].s64 = ctx.r[11].s64 + 12052;
	// 8325142C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83251430: 4AFDBAA1  bl 0x8222ced0
	ctx.lr = 0x83251434;
	sub_8222CED0(ctx, base);
	// 83251434: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83251438: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325143C: 4AF9D6FD  bl 0x821eeb38
	ctx.lr = 0x83251440;
	sub_821EEB38(ctx, base);
	// 83251440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251444: 4B9B23AD  bl 0x82c037f0
	ctx.lr = 0x83251448;
	sub_82C037F0(ctx, base);
	// 83251448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325144C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251454: 916A8034  stw r11, -0x7fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32716 as u32), ctx.r[11].u32 ) };
	// 83251458: 4AF75311  bl 0x821c6768
	ctx.lr = 0x8325145C;
	sub_821C6768(ctx, base);
	// 8325145C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83251460: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83251464: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83251468; continue 'dispatch;
            }
            0x83251468 => {
    //   block [0x83251468..0x83251494)
	// 83251468: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325146C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251470: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83251474: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83251478: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325147C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251480: 4082FFE8  bne 0x83251468
	if !ctx.cr[0].eq {
	pc = 0x83251468; continue 'dispatch;
	}
	// 83251484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251488: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325148C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251490: 4AF752D9  bl 0x821c6768
	ctx.lr = 0x83251494;
	sub_821C6768(ctx, base);
	pc = 0x83251494; continue 'dispatch;
            }
            0x83251494 => {
    //   block [0x83251494..0x832514D0)
	// 83251494: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251498: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325149C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832514A0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832514A4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832514A8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832514AC: 4082FFE8  bne 0x83251494
	if !ctx.cr[0].eq {
	pc = 0x83251494; continue 'dispatch;
	}
	// 832514B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832514B4: 386B9768  addi r3, r11, -0x6898
	ctx.r[3].s64 = ctx.r[11].s64 + -26776;
	// 832514B8: 4BA58A69  bl 0x82ca9f20
	ctx.lr = 0x832514BC;
	sub_82CA9F20(ctx, base);
	// 832514BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832514C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832514C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832514C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832514CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832514D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832514D0 size=192
    let mut pc: u32 = 0x832514D0;
    'dispatch: loop {
        match pc {
            0x832514D0 => {
    //   block [0x832514D0..0x83251528)
	// 832514D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832514D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832514D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832514DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832514E0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832514E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832514E8: 388B2F38  addi r4, r11, 0x2f38
	ctx.r[4].s64 = ctx.r[11].s64 + 12088;
	// 832514EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832514F0: 4AFDB9E1  bl 0x8222ced0
	ctx.lr = 0x832514F4;
	sub_8222CED0(ctx, base);
	// 832514F4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832514F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832514FC: 4AF9D63D  bl 0x821eeb38
	ctx.lr = 0x83251500;
	sub_821EEB38(ctx, base);
	// 83251500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251504: 4B9B22ED  bl 0x82c037f0
	ctx.lr = 0x83251508;
	sub_82C037F0(ctx, base);
	// 83251508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325150C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251514: 916A8038  stw r11, -0x7fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32712 as u32), ctx.r[11].u32 ) };
	// 83251518: 4AF75251  bl 0x821c6768
	ctx.lr = 0x8325151C;
	sub_821C6768(ctx, base);
	// 8325151C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83251520: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83251524: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83251528; continue 'dispatch;
            }
            0x83251528 => {
    //   block [0x83251528..0x83251554)
	// 83251528: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325152C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251530: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83251534: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83251538: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325153C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251540: 4082FFE8  bne 0x83251528
	if !ctx.cr[0].eq {
	pc = 0x83251528; continue 'dispatch;
	}
	// 83251544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251548: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325154C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251550: 4AF75219  bl 0x821c6768
	ctx.lr = 0x83251554;
	sub_821C6768(ctx, base);
	pc = 0x83251554; continue 'dispatch;
            }
            0x83251554 => {
    //   block [0x83251554..0x83251590)
	// 83251554: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251558: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325155C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83251560: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83251564: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83251568: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325156C: 4082FFE8  bne 0x83251554
	if !ctx.cr[0].eq {
	pc = 0x83251554; continue 'dispatch;
	}
	// 83251570: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251574: 386B9770  addi r3, r11, -0x6890
	ctx.r[3].s64 = ctx.r[11].s64 + -26768;
	// 83251578: 4BA589A9  bl 0x82ca9f20
	ctx.lr = 0x8325157C;
	sub_82CA9F20(ctx, base);
	// 8325157C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251590 size=192
    let mut pc: u32 = 0x83251590;
    'dispatch: loop {
        match pc {
            0x83251590 => {
    //   block [0x83251590..0x832515E8)
	// 83251590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325159C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832515A0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832515A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832515A8: 388B2F58  addi r4, r11, 0x2f58
	ctx.r[4].s64 = ctx.r[11].s64 + 12120;
	// 832515AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832515B0: 4AFDB921  bl 0x8222ced0
	ctx.lr = 0x832515B4;
	sub_8222CED0(ctx, base);
	// 832515B4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832515B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832515BC: 4AF9D57D  bl 0x821eeb38
	ctx.lr = 0x832515C0;
	sub_821EEB38(ctx, base);
	// 832515C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832515C4: 4B9B222D  bl 0x82c037f0
	ctx.lr = 0x832515C8;
	sub_82C037F0(ctx, base);
	// 832515C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832515CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832515D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832515D4: 916A803C  stw r11, -0x7fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32708 as u32), ctx.r[11].u32 ) };
	// 832515D8: 4AF75191  bl 0x821c6768
	ctx.lr = 0x832515DC;
	sub_821C6768(ctx, base);
	// 832515DC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832515E0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832515E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832515E8; continue 'dispatch;
            }
            0x832515E8 => {
    //   block [0x832515E8..0x83251614)
	// 832515E8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832515EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832515F0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832515F4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832515F8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832515FC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251600: 4082FFE8  bne 0x832515e8
	if !ctx.cr[0].eq {
	pc = 0x832515E8; continue 'dispatch;
	}
	// 83251604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251608: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325160C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251610: 4AF75159  bl 0x821c6768
	ctx.lr = 0x83251614;
	sub_821C6768(ctx, base);
	pc = 0x83251614; continue 'dispatch;
            }
            0x83251614 => {
    //   block [0x83251614..0x83251650)
	// 83251614: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251618: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325161C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83251620: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83251624: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83251628: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325162C: 4082FFE8  bne 0x83251614
	if !ctx.cr[0].eq {
	pc = 0x83251614; continue 'dispatch;
	}
	// 83251630: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251634: 386B9778  addi r3, r11, -0x6888
	ctx.r[3].s64 = ctx.r[11].s64 + -26760;
	// 83251638: 4BA588E9  bl 0x82ca9f20
	ctx.lr = 0x8325163C;
	sub_82CA9F20(ctx, base);
	// 8325163C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325164C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251650 size=192
    let mut pc: u32 = 0x83251650;
    'dispatch: loop {
        match pc {
            0x83251650 => {
    //   block [0x83251650..0x832516A8)
	// 83251650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325165C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251660: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251668: 388B2F78  addi r4, r11, 0x2f78
	ctx.r[4].s64 = ctx.r[11].s64 + 12152;
	// 8325166C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83251670: 4AFDB861  bl 0x8222ced0
	ctx.lr = 0x83251674;
	sub_8222CED0(ctx, base);
	// 83251674: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83251678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325167C: 4AF9D4BD  bl 0x821eeb38
	ctx.lr = 0x83251680;
	sub_821EEB38(ctx, base);
	// 83251680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251684: 4B9B216D  bl 0x82c037f0
	ctx.lr = 0x83251688;
	sub_82C037F0(ctx, base);
	// 83251688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325168C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251690: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251694: 916A8040  stw r11, -0x7fc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32704 as u32), ctx.r[11].u32 ) };
	// 83251698: 4AF750D1  bl 0x821c6768
	ctx.lr = 0x8325169C;
	sub_821C6768(ctx, base);
	// 8325169C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832516A0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832516A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832516A8; continue 'dispatch;
            }
            0x832516A8 => {
    //   block [0x832516A8..0x832516D4)
	// 832516A8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832516AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516B0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832516B4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832516B8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832516BC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516C0: 4082FFE8  bne 0x832516a8
	if !ctx.cr[0].eq {
	pc = 0x832516A8; continue 'dispatch;
	}
	// 832516C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832516C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832516CC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832516D0: 4AF75099  bl 0x821c6768
	ctx.lr = 0x832516D4;
	sub_821C6768(ctx, base);
	pc = 0x832516D4; continue 'dispatch;
            }
            0x832516D4 => {
    //   block [0x832516D4..0x83251710)
	// 832516D4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 832516D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516DC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832516E0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832516E4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832516E8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516EC: 4082FFE8  bne 0x832516d4
	if !ctx.cr[0].eq {
	pc = 0x832516D4; continue 'dispatch;
	}
	// 832516F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832516F4: 386B9780  addi r3, r11, -0x6880
	ctx.r[3].s64 = ctx.r[11].s64 + -26752;
	// 832516F8: 4BA58829  bl 0x82ca9f20
	ctx.lr = 0x832516FC;
	sub_82CA9F20(ctx, base);
	// 832516FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325170C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251710 size=56
    let mut pc: u32 = 0x83251710;
    'dispatch: loop {
        match pc {
            0x83251710 => {
    //   block [0x83251710..0x83251748)
	// 83251710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325171C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251724: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83251728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325172C: 4AFA262D  bl 0x821f3d58
	ctx.lr = 0x83251730;
	sub_821F3D58(ctx, base);
	// 83251730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251734: 906A8044  stw r3, -0x7fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32700 as u32), ctx.r[3].u32 ) };
	// 83251738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325173C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251748 size=56
    let mut pc: u32 = 0x83251748;
    'dispatch: loop {
        match pc {
            0x83251748 => {
    //   block [0x83251748..0x83251780)
	// 83251748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325174C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325175C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83251760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251764: 4AFA25F5  bl 0x821f3d58
	ctx.lr = 0x83251768;
	sub_821F3D58(ctx, base);
	// 83251768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325176C: 906A8048  stw r3, -0x7fb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32696 as u32), ctx.r[3].u32 ) };
	// 83251770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251780 size=56
    let mut pc: u32 = 0x83251780;
    'dispatch: loop {
        match pc {
            0x83251780 => {
    //   block [0x83251780..0x832517B8)
	// 83251780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325178C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251794: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83251798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325179C: 4AFA25BD  bl 0x821f3d58
	ctx.lr = 0x832517A0;
	sub_821F3D58(ctx, base);
	// 832517A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832517A4: 906A804C  stw r3, -0x7fb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32692 as u32), ctx.r[3].u32 ) };
	// 832517A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832517AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832517B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832517B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832517B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832517B8 size=56
    let mut pc: u32 = 0x832517B8;
    'dispatch: loop {
        match pc {
            0x832517B8 => {
    //   block [0x832517B8..0x832517F0)
	// 832517B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832517BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832517C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832517C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832517C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832517CC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832517D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832517D4: 4AFA2585  bl 0x821f3d58
	ctx.lr = 0x832517D8;
	sub_821F3D58(ctx, base);
	// 832517D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832517DC: 906A8050  stw r3, -0x7fb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32688 as u32), ctx.r[3].u32 ) };
	// 832517E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832517E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832517E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832517EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832517F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832517F0 size=56
    let mut pc: u32 = 0x832517F0;
    'dispatch: loop {
        match pc {
            0x832517F0 => {
    //   block [0x832517F0..0x83251828)
	// 832517F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832517F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832517F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832517FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251804: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83251808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325180C: 4AFA254D  bl 0x821f3d58
	ctx.lr = 0x83251810;
	sub_821F3D58(ctx, base);
	// 83251810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251814: 906A8054  stw r3, -0x7fac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32684 as u32), ctx.r[3].u32 ) };
	// 83251818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325181C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251828 size=56
    let mut pc: u32 = 0x83251828;
    'dispatch: loop {
        match pc {
            0x83251828 => {
    //   block [0x83251828..0x83251860)
	// 83251828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325183C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83251840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251844: 4AFA2515  bl 0x821f3d58
	ctx.lr = 0x83251848;
	sub_821F3D58(ctx, base);
	// 83251848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325184C: 906A8058  stw r3, -0x7fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32680 as u32), ctx.r[3].u32 ) };
	// 83251850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325185C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251860 size=56
    let mut pc: u32 = 0x83251860;
    'dispatch: loop {
        match pc {
            0x83251860 => {
    //   block [0x83251860..0x83251898)
	// 83251860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325186C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251874: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83251878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325187C: 4AFA24DD  bl 0x821f3d58
	ctx.lr = 0x83251880;
	sub_821F3D58(ctx, base);
	// 83251880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251884: 906A805C  stw r3, -0x7fa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32676 as u32), ctx.r[3].u32 ) };
	// 83251888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251898 size=56
    let mut pc: u32 = 0x83251898;
    'dispatch: loop {
        match pc {
            0x83251898 => {
    //   block [0x83251898..0x832518D0)
	// 83251898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832518A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832518A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832518A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832518AC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832518B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832518B4: 4AFA24A5  bl 0x821f3d58
	ctx.lr = 0x832518B8;
	sub_821F3D58(ctx, base);
	// 832518B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832518BC: 906A8060  stw r3, -0x7fa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32672 as u32), ctx.r[3].u32 ) };
	// 832518C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832518C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832518C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832518CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832518D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832518D0 size=56
    let mut pc: u32 = 0x832518D0;
    'dispatch: loop {
        match pc {
            0x832518D0 => {
    //   block [0x832518D0..0x83251908)
	// 832518D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832518D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832518D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832518DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832518E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832518E4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832518E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832518EC: 4AFA246D  bl 0x821f3d58
	ctx.lr = 0x832518F0;
	sub_821F3D58(ctx, base);
	// 832518F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832518F4: 906A8064  stw r3, -0x7f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32668 as u32), ctx.r[3].u32 ) };
	// 832518F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832518FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251908 size=56
    let mut pc: u32 = 0x83251908;
    'dispatch: loop {
        match pc {
            0x83251908 => {
    //   block [0x83251908..0x83251940)
	// 83251908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325191C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83251920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251924: 4AFA2435  bl 0x821f3d58
	ctx.lr = 0x83251928;
	sub_821F3D58(ctx, base);
	// 83251928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325192C: 906A8068  stw r3, -0x7f98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32664 as u32), ctx.r[3].u32 ) };
	// 83251930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325193C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


