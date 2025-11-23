pub fn sub_832B1C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1C18 size=12
    let mut pc: u32 = 0x832B1C18;
    'dispatch: loop {
        match pc {
            0x832B1C18 => {
    //   block [0x832B1C18..0x832B1C24)
	// 832B1C18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1C1C: 386BE684  addi r3, r11, -0x197c
	ctx.r[3].s64 = ctx.r[11].s64 + -6524;
	// 832B1C20: 4AF631B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B1C28 size=72
    let mut pc: u32 = 0x832B1C28;
    'dispatch: loop {
        match pc {
            0x832B1C28 => {
    //   block [0x832B1C28..0x832B1C70)
	// 832B1C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B1C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B1C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B1C34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B1C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832B1C3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B1C40: 3BEAE688  addi r31, r10, -0x1978
	ctx.r[31].s64 = ctx.r[10].s64 + -6520;
	// 832B1C44: 396B2544  addi r11, r11, 0x2544
	ctx.r[11].s64 = ctx.r[11].s64 + 9540;
	// 832B1C48: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1C4C: 916AE688  stw r11, -0x1978(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6520 as u32), ctx.r[11].u32 ) };
	// 832B1C50: 4B061D49  bl 0x82313998
	ctx.lr = 0x832B1C54;
	sub_82313998(ctx, base);
	// 832B1C54: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B1C58: 4B061BB1  bl 0x82313808
	ctx.lr = 0x832B1C5C;
	sub_82313808(ctx, base);
	// 832B1C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B1C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B1C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B1C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B1C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1C70 size=12
    let mut pc: u32 = 0x832B1C70;
    'dispatch: loop {
        match pc {
            0x832B1C70 => {
    //   block [0x832B1C70..0x832B1C7C)
	// 832B1C70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1C74: 386BE698  addi r3, r11, -0x1968
	ctx.r[3].s64 = ctx.r[11].s64 + -6504;
	// 832B1C78: 4AF63160  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1C80 size=12
    let mut pc: u32 = 0x832B1C80;
    'dispatch: loop {
        match pc {
            0x832B1C80 => {
    //   block [0x832B1C80..0x832B1C8C)
	// 832B1C80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1C84: 386BE69C  addi r3, r11, -0x1964
	ctx.r[3].s64 = ctx.r[11].s64 + -6500;
	// 832B1C88: 4AF63150  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1C90 size=12
    let mut pc: u32 = 0x832B1C90;
    'dispatch: loop {
        match pc {
            0x832B1C90 => {
    //   block [0x832B1C90..0x832B1C9C)
	// 832B1C90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1C94: 386BE6A0  addi r3, r11, -0x1960
	ctx.r[3].s64 = ctx.r[11].s64 + -6496;
	// 832B1C98: 4AF63140  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1CA0 size=12
    let mut pc: u32 = 0x832B1CA0;
    'dispatch: loop {
        match pc {
            0x832B1CA0 => {
    //   block [0x832B1CA0..0x832B1CAC)
	// 832B1CA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1CA4: 386BE6A4  addi r3, r11, -0x195c
	ctx.r[3].s64 = ctx.r[11].s64 + -6492;
	// 832B1CA8: 4AF63130  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1CB0 size=12
    let mut pc: u32 = 0x832B1CB0;
    'dispatch: loop {
        match pc {
            0x832B1CB0 => {
    //   block [0x832B1CB0..0x832B1CBC)
	// 832B1CB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1CB4: 386BE6A8  addi r3, r11, -0x1958
	ctx.r[3].s64 = ctx.r[11].s64 + -6488;
	// 832B1CB8: 4AF63120  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1CC0 size=12
    let mut pc: u32 = 0x832B1CC0;
    'dispatch: loop {
        match pc {
            0x832B1CC0 => {
    //   block [0x832B1CC0..0x832B1CCC)
	// 832B1CC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1CC4: 386BE6AC  addi r3, r11, -0x1954
	ctx.r[3].s64 = ctx.r[11].s64 + -6484;
	// 832B1CC8: 4AF63110  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1CD0 size=12
    let mut pc: u32 = 0x832B1CD0;
    'dispatch: loop {
        match pc {
            0x832B1CD0 => {
    //   block [0x832B1CD0..0x832B1CDC)
	// 832B1CD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1CD4: 386BE6B0  addi r3, r11, -0x1950
	ctx.r[3].s64 = ctx.r[11].s64 + -6480;
	// 832B1CD8: 4AF63100  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1CE0 size=12
    let mut pc: u32 = 0x832B1CE0;
    'dispatch: loop {
        match pc {
            0x832B1CE0 => {
    //   block [0x832B1CE0..0x832B1CEC)
	// 832B1CE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1CE4: 386BE6B4  addi r3, r11, -0x194c
	ctx.r[3].s64 = ctx.r[11].s64 + -6476;
	// 832B1CE8: 4AF630F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1CF0 size=12
    let mut pc: u32 = 0x832B1CF0;
    'dispatch: loop {
        match pc {
            0x832B1CF0 => {
    //   block [0x832B1CF0..0x832B1CFC)
	// 832B1CF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1CF4: 386BE6B8  addi r3, r11, -0x1948
	ctx.r[3].s64 = ctx.r[11].s64 + -6472;
	// 832B1CF8: 4AF630E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D00 size=12
    let mut pc: u32 = 0x832B1D00;
    'dispatch: loop {
        match pc {
            0x832B1D00 => {
    //   block [0x832B1D00..0x832B1D0C)
	// 832B1D00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D04: 386BE6BC  addi r3, r11, -0x1944
	ctx.r[3].s64 = ctx.r[11].s64 + -6468;
	// 832B1D08: 4AF630D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D10 size=12
    let mut pc: u32 = 0x832B1D10;
    'dispatch: loop {
        match pc {
            0x832B1D10 => {
    //   block [0x832B1D10..0x832B1D1C)
	// 832B1D10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D14: 386BE6C0  addi r3, r11, -0x1940
	ctx.r[3].s64 = ctx.r[11].s64 + -6464;
	// 832B1D18: 4AF630C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D20 size=12
    let mut pc: u32 = 0x832B1D20;
    'dispatch: loop {
        match pc {
            0x832B1D20 => {
    //   block [0x832B1D20..0x832B1D2C)
	// 832B1D20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D24: 386BE6C4  addi r3, r11, -0x193c
	ctx.r[3].s64 = ctx.r[11].s64 + -6460;
	// 832B1D28: 4AF630B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D30 size=12
    let mut pc: u32 = 0x832B1D30;
    'dispatch: loop {
        match pc {
            0x832B1D30 => {
    //   block [0x832B1D30..0x832B1D3C)
	// 832B1D30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D34: 386BE6C8  addi r3, r11, -0x1938
	ctx.r[3].s64 = ctx.r[11].s64 + -6456;
	// 832B1D38: 4AF630A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D40 size=12
    let mut pc: u32 = 0x832B1D40;
    'dispatch: loop {
        match pc {
            0x832B1D40 => {
    //   block [0x832B1D40..0x832B1D4C)
	// 832B1D40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D44: 386BE6CC  addi r3, r11, -0x1934
	ctx.r[3].s64 = ctx.r[11].s64 + -6452;
	// 832B1D48: 4AF63090  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D50 size=12
    let mut pc: u32 = 0x832B1D50;
    'dispatch: loop {
        match pc {
            0x832B1D50 => {
    //   block [0x832B1D50..0x832B1D5C)
	// 832B1D50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D54: 386BE6D0  addi r3, r11, -0x1930
	ctx.r[3].s64 = ctx.r[11].s64 + -6448;
	// 832B1D58: 4AF63080  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D60 size=12
    let mut pc: u32 = 0x832B1D60;
    'dispatch: loop {
        match pc {
            0x832B1D60 => {
    //   block [0x832B1D60..0x832B1D6C)
	// 832B1D60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D64: 386BE6D4  addi r3, r11, -0x192c
	ctx.r[3].s64 = ctx.r[11].s64 + -6444;
	// 832B1D68: 4AF63070  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D70 size=12
    let mut pc: u32 = 0x832B1D70;
    'dispatch: loop {
        match pc {
            0x832B1D70 => {
    //   block [0x832B1D70..0x832B1D7C)
	// 832B1D70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D74: 386BE6D8  addi r3, r11, -0x1928
	ctx.r[3].s64 = ctx.r[11].s64 + -6440;
	// 832B1D78: 4AF63060  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D80 size=12
    let mut pc: u32 = 0x832B1D80;
    'dispatch: loop {
        match pc {
            0x832B1D80 => {
    //   block [0x832B1D80..0x832B1D8C)
	// 832B1D80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D84: 386BE6DC  addi r3, r11, -0x1924
	ctx.r[3].s64 = ctx.r[11].s64 + -6436;
	// 832B1D88: 4AF63050  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1D90 size=12
    let mut pc: u32 = 0x832B1D90;
    'dispatch: loop {
        match pc {
            0x832B1D90 => {
    //   block [0x832B1D90..0x832B1D9C)
	// 832B1D90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1D94: 386BE6E0  addi r3, r11, -0x1920
	ctx.r[3].s64 = ctx.r[11].s64 + -6432;
	// 832B1D98: 4AF63040  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1DA0 size=12
    let mut pc: u32 = 0x832B1DA0;
    'dispatch: loop {
        match pc {
            0x832B1DA0 => {
    //   block [0x832B1DA0..0x832B1DAC)
	// 832B1DA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1DA4: 386BE6E4  addi r3, r11, -0x191c
	ctx.r[3].s64 = ctx.r[11].s64 + -6428;
	// 832B1DA8: 4AF63030  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1DB0 size=12
    let mut pc: u32 = 0x832B1DB0;
    'dispatch: loop {
        match pc {
            0x832B1DB0 => {
    //   block [0x832B1DB0..0x832B1DBC)
	// 832B1DB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1DB4: 386BE6E8  addi r3, r11, -0x1918
	ctx.r[3].s64 = ctx.r[11].s64 + -6424;
	// 832B1DB8: 4AF63020  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1DC0 size=12
    let mut pc: u32 = 0x832B1DC0;
    'dispatch: loop {
        match pc {
            0x832B1DC0 => {
    //   block [0x832B1DC0..0x832B1DCC)
	// 832B1DC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1DC4: 386BE6EC  addi r3, r11, -0x1914
	ctx.r[3].s64 = ctx.r[11].s64 + -6420;
	// 832B1DC8: 4AF63010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1DD0 size=12
    let mut pc: u32 = 0x832B1DD0;
    'dispatch: loop {
        match pc {
            0x832B1DD0 => {
    //   block [0x832B1DD0..0x832B1DDC)
	// 832B1DD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1DD4: 386BE6F0  addi r3, r11, -0x1910
	ctx.r[3].s64 = ctx.r[11].s64 + -6416;
	// 832B1DD8: 4AF63000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1DE0 size=12
    let mut pc: u32 = 0x832B1DE0;
    'dispatch: loop {
        match pc {
            0x832B1DE0 => {
    //   block [0x832B1DE0..0x832B1DEC)
	// 832B1DE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1DE4: 386BE6F4  addi r3, r11, -0x190c
	ctx.r[3].s64 = ctx.r[11].s64 + -6412;
	// 832B1DE8: 4AF62FF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1DF0 size=12
    let mut pc: u32 = 0x832B1DF0;
    'dispatch: loop {
        match pc {
            0x832B1DF0 => {
    //   block [0x832B1DF0..0x832B1DFC)
	// 832B1DF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1DF4: 386BE6F8  addi r3, r11, -0x1908
	ctx.r[3].s64 = ctx.r[11].s64 + -6408;
	// 832B1DF8: 4AF62FE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E00 size=12
    let mut pc: u32 = 0x832B1E00;
    'dispatch: loop {
        match pc {
            0x832B1E00 => {
    //   block [0x832B1E00..0x832B1E0C)
	// 832B1E00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E04: 386BE6FC  addi r3, r11, -0x1904
	ctx.r[3].s64 = ctx.r[11].s64 + -6404;
	// 832B1E08: 4AF62FD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E10 size=12
    let mut pc: u32 = 0x832B1E10;
    'dispatch: loop {
        match pc {
            0x832B1E10 => {
    //   block [0x832B1E10..0x832B1E1C)
	// 832B1E10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E14: 386BE700  addi r3, r11, -0x1900
	ctx.r[3].s64 = ctx.r[11].s64 + -6400;
	// 832B1E18: 4AF62FC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E20 size=12
    let mut pc: u32 = 0x832B1E20;
    'dispatch: loop {
        match pc {
            0x832B1E20 => {
    //   block [0x832B1E20..0x832B1E2C)
	// 832B1E20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E24: 386BE704  addi r3, r11, -0x18fc
	ctx.r[3].s64 = ctx.r[11].s64 + -6396;
	// 832B1E28: 4AF62FB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E30 size=12
    let mut pc: u32 = 0x832B1E30;
    'dispatch: loop {
        match pc {
            0x832B1E30 => {
    //   block [0x832B1E30..0x832B1E3C)
	// 832B1E30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E34: 386BE708  addi r3, r11, -0x18f8
	ctx.r[3].s64 = ctx.r[11].s64 + -6392;
	// 832B1E38: 4AF62FA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E40 size=12
    let mut pc: u32 = 0x832B1E40;
    'dispatch: loop {
        match pc {
            0x832B1E40 => {
    //   block [0x832B1E40..0x832B1E4C)
	// 832B1E40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E44: 386BE70C  addi r3, r11, -0x18f4
	ctx.r[3].s64 = ctx.r[11].s64 + -6388;
	// 832B1E48: 4AF62F90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E50 size=12
    let mut pc: u32 = 0x832B1E50;
    'dispatch: loop {
        match pc {
            0x832B1E50 => {
    //   block [0x832B1E50..0x832B1E5C)
	// 832B1E50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E54: 386BE710  addi r3, r11, -0x18f0
	ctx.r[3].s64 = ctx.r[11].s64 + -6384;
	// 832B1E58: 4AF62F80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E60 size=12
    let mut pc: u32 = 0x832B1E60;
    'dispatch: loop {
        match pc {
            0x832B1E60 => {
    //   block [0x832B1E60..0x832B1E6C)
	// 832B1E60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E64: 386BE714  addi r3, r11, -0x18ec
	ctx.r[3].s64 = ctx.r[11].s64 + -6380;
	// 832B1E68: 4AF62F70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E70 size=12
    let mut pc: u32 = 0x832B1E70;
    'dispatch: loop {
        match pc {
            0x832B1E70 => {
    //   block [0x832B1E70..0x832B1E7C)
	// 832B1E70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E74: 386BE718  addi r3, r11, -0x18e8
	ctx.r[3].s64 = ctx.r[11].s64 + -6376;
	// 832B1E78: 4AF62F60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E80 size=12
    let mut pc: u32 = 0x832B1E80;
    'dispatch: loop {
        match pc {
            0x832B1E80 => {
    //   block [0x832B1E80..0x832B1E8C)
	// 832B1E80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E84: 386BE71C  addi r3, r11, -0x18e4
	ctx.r[3].s64 = ctx.r[11].s64 + -6372;
	// 832B1E88: 4AF62F50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1E90 size=12
    let mut pc: u32 = 0x832B1E90;
    'dispatch: loop {
        match pc {
            0x832B1E90 => {
    //   block [0x832B1E90..0x832B1E9C)
	// 832B1E90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1E94: 386BE720  addi r3, r11, -0x18e0
	ctx.r[3].s64 = ctx.r[11].s64 + -6368;
	// 832B1E98: 4AF62F40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1EA0 size=12
    let mut pc: u32 = 0x832B1EA0;
    'dispatch: loop {
        match pc {
            0x832B1EA0 => {
    //   block [0x832B1EA0..0x832B1EAC)
	// 832B1EA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1EA4: 386BE724  addi r3, r11, -0x18dc
	ctx.r[3].s64 = ctx.r[11].s64 + -6364;
	// 832B1EA8: 4AF62F30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1EB0 size=12
    let mut pc: u32 = 0x832B1EB0;
    'dispatch: loop {
        match pc {
            0x832B1EB0 => {
    //   block [0x832B1EB0..0x832B1EBC)
	// 832B1EB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1EB4: 386BE728  addi r3, r11, -0x18d8
	ctx.r[3].s64 = ctx.r[11].s64 + -6360;
	// 832B1EB8: 4AF62F20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1EC0 size=12
    let mut pc: u32 = 0x832B1EC0;
    'dispatch: loop {
        match pc {
            0x832B1EC0 => {
    //   block [0x832B1EC0..0x832B1ECC)
	// 832B1EC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1EC4: 386BE72C  addi r3, r11, -0x18d4
	ctx.r[3].s64 = ctx.r[11].s64 + -6356;
	// 832B1EC8: 4AF62F10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1ED0 size=12
    let mut pc: u32 = 0x832B1ED0;
    'dispatch: loop {
        match pc {
            0x832B1ED0 => {
    //   block [0x832B1ED0..0x832B1EDC)
	// 832B1ED0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1ED4: 386BE730  addi r3, r11, -0x18d0
	ctx.r[3].s64 = ctx.r[11].s64 + -6352;
	// 832B1ED8: 4AF62F00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1EE0 size=12
    let mut pc: u32 = 0x832B1EE0;
    'dispatch: loop {
        match pc {
            0x832B1EE0 => {
    //   block [0x832B1EE0..0x832B1EEC)
	// 832B1EE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1EE4: 386BE734  addi r3, r11, -0x18cc
	ctx.r[3].s64 = ctx.r[11].s64 + -6348;
	// 832B1EE8: 4AF62EF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1EF0 size=12
    let mut pc: u32 = 0x832B1EF0;
    'dispatch: loop {
        match pc {
            0x832B1EF0 => {
    //   block [0x832B1EF0..0x832B1EFC)
	// 832B1EF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1EF4: 386BE738  addi r3, r11, -0x18c8
	ctx.r[3].s64 = ctx.r[11].s64 + -6344;
	// 832B1EF8: 4AF62EE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F00 size=12
    let mut pc: u32 = 0x832B1F00;
    'dispatch: loop {
        match pc {
            0x832B1F00 => {
    //   block [0x832B1F00..0x832B1F0C)
	// 832B1F00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F04: 386BE73C  addi r3, r11, -0x18c4
	ctx.r[3].s64 = ctx.r[11].s64 + -6340;
	// 832B1F08: 4AF62ED0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F10 size=12
    let mut pc: u32 = 0x832B1F10;
    'dispatch: loop {
        match pc {
            0x832B1F10 => {
    //   block [0x832B1F10..0x832B1F1C)
	// 832B1F10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F14: 386BE740  addi r3, r11, -0x18c0
	ctx.r[3].s64 = ctx.r[11].s64 + -6336;
	// 832B1F18: 4AF62EC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F20 size=12
    let mut pc: u32 = 0x832B1F20;
    'dispatch: loop {
        match pc {
            0x832B1F20 => {
    //   block [0x832B1F20..0x832B1F2C)
	// 832B1F20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F24: 386BE744  addi r3, r11, -0x18bc
	ctx.r[3].s64 = ctx.r[11].s64 + -6332;
	// 832B1F28: 4AF62EB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F30 size=12
    let mut pc: u32 = 0x832B1F30;
    'dispatch: loop {
        match pc {
            0x832B1F30 => {
    //   block [0x832B1F30..0x832B1F3C)
	// 832B1F30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F34: 386BE748  addi r3, r11, -0x18b8
	ctx.r[3].s64 = ctx.r[11].s64 + -6328;
	// 832B1F38: 4AF62EA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F40 size=12
    let mut pc: u32 = 0x832B1F40;
    'dispatch: loop {
        match pc {
            0x832B1F40 => {
    //   block [0x832B1F40..0x832B1F4C)
	// 832B1F40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F44: 386BE74C  addi r3, r11, -0x18b4
	ctx.r[3].s64 = ctx.r[11].s64 + -6324;
	// 832B1F48: 4AF62E90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F50 size=12
    let mut pc: u32 = 0x832B1F50;
    'dispatch: loop {
        match pc {
            0x832B1F50 => {
    //   block [0x832B1F50..0x832B1F5C)
	// 832B1F50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F54: 386BE750  addi r3, r11, -0x18b0
	ctx.r[3].s64 = ctx.r[11].s64 + -6320;
	// 832B1F58: 4AF62E80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F60 size=12
    let mut pc: u32 = 0x832B1F60;
    'dispatch: loop {
        match pc {
            0x832B1F60 => {
    //   block [0x832B1F60..0x832B1F6C)
	// 832B1F60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F64: 386BE754  addi r3, r11, -0x18ac
	ctx.r[3].s64 = ctx.r[11].s64 + -6316;
	// 832B1F68: 4AF62E70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F70 size=12
    let mut pc: u32 = 0x832B1F70;
    'dispatch: loop {
        match pc {
            0x832B1F70 => {
    //   block [0x832B1F70..0x832B1F7C)
	// 832B1F70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F74: 386BE758  addi r3, r11, -0x18a8
	ctx.r[3].s64 = ctx.r[11].s64 + -6312;
	// 832B1F78: 4AF62E60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F80 size=12
    let mut pc: u32 = 0x832B1F80;
    'dispatch: loop {
        match pc {
            0x832B1F80 => {
    //   block [0x832B1F80..0x832B1F8C)
	// 832B1F80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F84: 386BE75C  addi r3, r11, -0x18a4
	ctx.r[3].s64 = ctx.r[11].s64 + -6308;
	// 832B1F88: 4AF62E50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1F90 size=12
    let mut pc: u32 = 0x832B1F90;
    'dispatch: loop {
        match pc {
            0x832B1F90 => {
    //   block [0x832B1F90..0x832B1F9C)
	// 832B1F90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1F94: 386BE760  addi r3, r11, -0x18a0
	ctx.r[3].s64 = ctx.r[11].s64 + -6304;
	// 832B1F98: 4AF62E40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1FA0 size=12
    let mut pc: u32 = 0x832B1FA0;
    'dispatch: loop {
        match pc {
            0x832B1FA0 => {
    //   block [0x832B1FA0..0x832B1FAC)
	// 832B1FA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1FA4: 386BE764  addi r3, r11, -0x189c
	ctx.r[3].s64 = ctx.r[11].s64 + -6300;
	// 832B1FA8: 4AF62E30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1FB0 size=12
    let mut pc: u32 = 0x832B1FB0;
    'dispatch: loop {
        match pc {
            0x832B1FB0 => {
    //   block [0x832B1FB0..0x832B1FBC)
	// 832B1FB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1FB4: 386BE768  addi r3, r11, -0x1898
	ctx.r[3].s64 = ctx.r[11].s64 + -6296;
	// 832B1FB8: 4AF62E20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1FC0 size=12
    let mut pc: u32 = 0x832B1FC0;
    'dispatch: loop {
        match pc {
            0x832B1FC0 => {
    //   block [0x832B1FC0..0x832B1FCC)
	// 832B1FC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1FC4: 386BE76C  addi r3, r11, -0x1894
	ctx.r[3].s64 = ctx.r[11].s64 + -6292;
	// 832B1FC8: 4AF62E10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1FD0 size=12
    let mut pc: u32 = 0x832B1FD0;
    'dispatch: loop {
        match pc {
            0x832B1FD0 => {
    //   block [0x832B1FD0..0x832B1FDC)
	// 832B1FD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1FD4: 386BE770  addi r3, r11, -0x1890
	ctx.r[3].s64 = ctx.r[11].s64 + -6288;
	// 832B1FD8: 4AF62E00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1FE0 size=12
    let mut pc: u32 = 0x832B1FE0;
    'dispatch: loop {
        match pc {
            0x832B1FE0 => {
    //   block [0x832B1FE0..0x832B1FEC)
	// 832B1FE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1FE4: 386BE774  addi r3, r11, -0x188c
	ctx.r[3].s64 = ctx.r[11].s64 + -6284;
	// 832B1FE8: 4AF62DF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B1FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B1FF0 size=12
    let mut pc: u32 = 0x832B1FF0;
    'dispatch: loop {
        match pc {
            0x832B1FF0 => {
    //   block [0x832B1FF0..0x832B1FFC)
	// 832B1FF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B1FF4: 386BE778  addi r3, r11, -0x1888
	ctx.r[3].s64 = ctx.r[11].s64 + -6280;
	// 832B1FF8: 4AF62DE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2000 size=12
    let mut pc: u32 = 0x832B2000;
    'dispatch: loop {
        match pc {
            0x832B2000 => {
    //   block [0x832B2000..0x832B200C)
	// 832B2000: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2004: 386BE77C  addi r3, r11, -0x1884
	ctx.r[3].s64 = ctx.r[11].s64 + -6276;
	// 832B2008: 4AF62DD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2010 size=84
    let mut pc: u32 = 0x832B2010;
    'dispatch: loop {
        match pc {
            0x832B2010 => {
    //   block [0x832B2010..0x832B2038)
	// 832B2010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B201C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2020: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B2024: 3BEB3B40  addi r31, r11, 0x3b40
	ctx.r[31].s64 = ctx.r[11].s64 + 15168;
	// 832B2028: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B202C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B2030: 419A0008  beq cr6, 0x832b2038
	if ctx.cr[6].eq {
	pc = 0x832B2038; continue 'dispatch;
	}
	// 832B2034: 4AF69D05  bl 0x8221bd38
	ctx.lr = 0x832B2038;
	sub_8221BD38(ctx, base);
	pc = 0x832B2038; continue 'dispatch;
            }
            0x832B2038 => {
    //   block [0x832B2038..0x832B2064)
	// 832B2038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B203C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B2040: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B2044: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B2048: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B204C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B2050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B205C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2068 size=84
    let mut pc: u32 = 0x832B2068;
    'dispatch: loop {
        match pc {
            0x832B2068 => {
    //   block [0x832B2068..0x832B2090)
	// 832B2068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2078: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B207C: 3BEB3B54  addi r31, r11, 0x3b54
	ctx.r[31].s64 = ctx.r[11].s64 + 15188;
	// 832B2080: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2084: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B2088: 419A0008  beq cr6, 0x832b2090
	if ctx.cr[6].eq {
	pc = 0x832B2090; continue 'dispatch;
	}
	// 832B208C: 4AF69CAD  bl 0x8221bd38
	ctx.lr = 0x832B2090;
	sub_8221BD38(ctx, base);
	pc = 0x832B2090; continue 'dispatch;
            }
            0x832B2090 => {
    //   block [0x832B2090..0x832B20BC)
	// 832B2090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B2098: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B209C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B20A0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B20A4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B20A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B20AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B20B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B20B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B20B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B20C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B20C0 size=84
    let mut pc: u32 = 0x832B20C0;
    'dispatch: loop {
        match pc {
            0x832B20C0 => {
    //   block [0x832B20C0..0x832B20E8)
	// 832B20C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B20C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B20C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B20CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B20D0: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B20D4: 3BEB3B68  addi r31, r11, 0x3b68
	ctx.r[31].s64 = ctx.r[11].s64 + 15208;
	// 832B20D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B20DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B20E0: 419A0008  beq cr6, 0x832b20e8
	if ctx.cr[6].eq {
	pc = 0x832B20E8; continue 'dispatch;
	}
	// 832B20E4: 4AF69C55  bl 0x8221bd38
	ctx.lr = 0x832B20E8;
	sub_8221BD38(ctx, base);
	pc = 0x832B20E8; continue 'dispatch;
            }
            0x832B20E8 => {
    //   block [0x832B20E8..0x832B2114)
	// 832B20E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B20EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B20F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B20F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B20F8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B20FC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B2100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B210C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2118 size=12
    let mut pc: u32 = 0x832B2118;
    'dispatch: loop {
        match pc {
            0x832B2118 => {
    //   block [0x832B2118..0x832B2124)
	// 832B2118: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B211C: 386BE7C0  addi r3, r11, -0x1840
	ctx.r[3].s64 = ctx.r[11].s64 + -6208;
	// 832B2120: 4AF62CB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2128 size=12
    let mut pc: u32 = 0x832B2128;
    'dispatch: loop {
        match pc {
            0x832B2128 => {
    //   block [0x832B2128..0x832B2134)
	// 832B2128: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B212C: 386BE7C4  addi r3, r11, -0x183c
	ctx.r[3].s64 = ctx.r[11].s64 + -6204;
	// 832B2130: 4AF62CA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2138 size=12
    let mut pc: u32 = 0x832B2138;
    'dispatch: loop {
        match pc {
            0x832B2138 => {
    //   block [0x832B2138..0x832B2144)
	// 832B2138: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B213C: 386BE7C8  addi r3, r11, -0x1838
	ctx.r[3].s64 = ctx.r[11].s64 + -6200;
	// 832B2140: 4AF62C98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2148 size=12
    let mut pc: u32 = 0x832B2148;
    'dispatch: loop {
        match pc {
            0x832B2148 => {
    //   block [0x832B2148..0x832B2154)
	// 832B2148: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B214C: 386BE7CC  addi r3, r11, -0x1834
	ctx.r[3].s64 = ctx.r[11].s64 + -6196;
	// 832B2150: 4AF62C88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2158 size=12
    let mut pc: u32 = 0x832B2158;
    'dispatch: loop {
        match pc {
            0x832B2158 => {
    //   block [0x832B2158..0x832B2164)
	// 832B2158: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B215C: 386BE7D0  addi r3, r11, -0x1830
	ctx.r[3].s64 = ctx.r[11].s64 + -6192;
	// 832B2160: 4AF62C78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2168 size=12
    let mut pc: u32 = 0x832B2168;
    'dispatch: loop {
        match pc {
            0x832B2168 => {
    //   block [0x832B2168..0x832B2174)
	// 832B2168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B216C: 386BE7D4  addi r3, r11, -0x182c
	ctx.r[3].s64 = ctx.r[11].s64 + -6188;
	// 832B2170: 4AF62C68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2178 size=12
    let mut pc: u32 = 0x832B2178;
    'dispatch: loop {
        match pc {
            0x832B2178 => {
    //   block [0x832B2178..0x832B2184)
	// 832B2178: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B217C: 386BE7D8  addi r3, r11, -0x1828
	ctx.r[3].s64 = ctx.r[11].s64 + -6184;
	// 832B2180: 4AF62C58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2188 size=12
    let mut pc: u32 = 0x832B2188;
    'dispatch: loop {
        match pc {
            0x832B2188 => {
    //   block [0x832B2188..0x832B2194)
	// 832B2188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B218C: 386BE7DC  addi r3, r11, -0x1824
	ctx.r[3].s64 = ctx.r[11].s64 + -6180;
	// 832B2190: 4AF62C48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2198 size=12
    let mut pc: u32 = 0x832B2198;
    'dispatch: loop {
        match pc {
            0x832B2198 => {
    //   block [0x832B2198..0x832B21A4)
	// 832B2198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B219C: 386BE7E0  addi r3, r11, -0x1820
	ctx.r[3].s64 = ctx.r[11].s64 + -6176;
	// 832B21A0: 4AF62C38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B21A8 size=12
    let mut pc: u32 = 0x832B21A8;
    'dispatch: loop {
        match pc {
            0x832B21A8 => {
    //   block [0x832B21A8..0x832B21B4)
	// 832B21A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B21AC: 386BE7E4  addi r3, r11, -0x181c
	ctx.r[3].s64 = ctx.r[11].s64 + -6172;
	// 832B21B0: 4AF62C28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B21B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B21B8 size=12
    let mut pc: u32 = 0x832B21B8;
    'dispatch: loop {
        match pc {
            0x832B21B8 => {
    //   block [0x832B21B8..0x832B21C4)
	// 832B21B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B21BC: 386BE7E8  addi r3, r11, -0x1818
	ctx.r[3].s64 = ctx.r[11].s64 + -6168;
	// 832B21C0: 4AF62C18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B21C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B21C8 size=12
    let mut pc: u32 = 0x832B21C8;
    'dispatch: loop {
        match pc {
            0x832B21C8 => {
    //   block [0x832B21C8..0x832B21D4)
	// 832B21C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B21CC: 386BE7EC  addi r3, r11, -0x1814
	ctx.r[3].s64 = ctx.r[11].s64 + -6164;
	// 832B21D0: 4AF62C08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B21D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B21D8 size=12
    let mut pc: u32 = 0x832B21D8;
    'dispatch: loop {
        match pc {
            0x832B21D8 => {
    //   block [0x832B21D8..0x832B21E4)
	// 832B21D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B21DC: 386BE7F0  addi r3, r11, -0x1810
	ctx.r[3].s64 = ctx.r[11].s64 + -6160;
	// 832B21E0: 4AF62BF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B21E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B21E8 size=12
    let mut pc: u32 = 0x832B21E8;
    'dispatch: loop {
        match pc {
            0x832B21E8 => {
    //   block [0x832B21E8..0x832B21F4)
	// 832B21E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B21EC: 386BE7F4  addi r3, r11, -0x180c
	ctx.r[3].s64 = ctx.r[11].s64 + -6156;
	// 832B21F0: 4AF62BE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B21F8 size=12
    let mut pc: u32 = 0x832B21F8;
    'dispatch: loop {
        match pc {
            0x832B21F8 => {
    //   block [0x832B21F8..0x832B2204)
	// 832B21F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B21FC: 386BE7F8  addi r3, r11, -0x1808
	ctx.r[3].s64 = ctx.r[11].s64 + -6152;
	// 832B2200: 4AF62BD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2208 size=12
    let mut pc: u32 = 0x832B2208;
    'dispatch: loop {
        match pc {
            0x832B2208 => {
    //   block [0x832B2208..0x832B2214)
	// 832B2208: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B220C: 386BE7FC  addi r3, r11, -0x1804
	ctx.r[3].s64 = ctx.r[11].s64 + -6148;
	// 832B2210: 4AF62BC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2218 size=12
    let mut pc: u32 = 0x832B2218;
    'dispatch: loop {
        match pc {
            0x832B2218 => {
    //   block [0x832B2218..0x832B2224)
	// 832B2218: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B221C: 386BE800  addi r3, r11, -0x1800
	ctx.r[3].s64 = ctx.r[11].s64 + -6144;
	// 832B2220: 4AF62BB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2228 size=12
    let mut pc: u32 = 0x832B2228;
    'dispatch: loop {
        match pc {
            0x832B2228 => {
    //   block [0x832B2228..0x832B2234)
	// 832B2228: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B222C: 386BE804  addi r3, r11, -0x17fc
	ctx.r[3].s64 = ctx.r[11].s64 + -6140;
	// 832B2230: 4AF62BA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2238 size=12
    let mut pc: u32 = 0x832B2238;
    'dispatch: loop {
        match pc {
            0x832B2238 => {
    //   block [0x832B2238..0x832B2244)
	// 832B2238: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B223C: 386BE808  addi r3, r11, -0x17f8
	ctx.r[3].s64 = ctx.r[11].s64 + -6136;
	// 832B2240: 4AF62B98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2248 size=12
    let mut pc: u32 = 0x832B2248;
    'dispatch: loop {
        match pc {
            0x832B2248 => {
    //   block [0x832B2248..0x832B2254)
	// 832B2248: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B224C: 386BE80C  addi r3, r11, -0x17f4
	ctx.r[3].s64 = ctx.r[11].s64 + -6132;
	// 832B2250: 4AF62B88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2258 size=12
    let mut pc: u32 = 0x832B2258;
    'dispatch: loop {
        match pc {
            0x832B2258 => {
    //   block [0x832B2258..0x832B2264)
	// 832B2258: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B225C: 386BE810  addi r3, r11, -0x17f0
	ctx.r[3].s64 = ctx.r[11].s64 + -6128;
	// 832B2260: 4AF62B78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2268 size=12
    let mut pc: u32 = 0x832B2268;
    'dispatch: loop {
        match pc {
            0x832B2268 => {
    //   block [0x832B2268..0x832B2274)
	// 832B2268: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B226C: 386BE814  addi r3, r11, -0x17ec
	ctx.r[3].s64 = ctx.r[11].s64 + -6124;
	// 832B2270: 4AF62B68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2278 size=12
    let mut pc: u32 = 0x832B2278;
    'dispatch: loop {
        match pc {
            0x832B2278 => {
    //   block [0x832B2278..0x832B2284)
	// 832B2278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B227C: 386BE818  addi r3, r11, -0x17e8
	ctx.r[3].s64 = ctx.r[11].s64 + -6120;
	// 832B2280: 4AF62B58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2288 size=12
    let mut pc: u32 = 0x832B2288;
    'dispatch: loop {
        match pc {
            0x832B2288 => {
    //   block [0x832B2288..0x832B2294)
	// 832B2288: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B228C: 386BE81C  addi r3, r11, -0x17e4
	ctx.r[3].s64 = ctx.r[11].s64 + -6116;
	// 832B2290: 4AF62B48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2298 size=12
    let mut pc: u32 = 0x832B2298;
    'dispatch: loop {
        match pc {
            0x832B2298 => {
    //   block [0x832B2298..0x832B22A4)
	// 832B2298: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B229C: 386BE820  addi r3, r11, -0x17e0
	ctx.r[3].s64 = ctx.r[11].s64 + -6112;
	// 832B22A0: 4AF62B38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B22A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B22A8 size=12
    let mut pc: u32 = 0x832B22A8;
    'dispatch: loop {
        match pc {
            0x832B22A8 => {
    //   block [0x832B22A8..0x832B22B4)
	// 832B22A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B22AC: 386BE824  addi r3, r11, -0x17dc
	ctx.r[3].s64 = ctx.r[11].s64 + -6108;
	// 832B22B0: 4AF62B28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B22B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B22B8 size=12
    let mut pc: u32 = 0x832B22B8;
    'dispatch: loop {
        match pc {
            0x832B22B8 => {
    //   block [0x832B22B8..0x832B22C4)
	// 832B22B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B22BC: 386BE828  addi r3, r11, -0x17d8
	ctx.r[3].s64 = ctx.r[11].s64 + -6104;
	// 832B22C0: 4AF62B18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B22C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B22C8 size=12
    let mut pc: u32 = 0x832B22C8;
    'dispatch: loop {
        match pc {
            0x832B22C8 => {
    //   block [0x832B22C8..0x832B22D4)
	// 832B22C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B22CC: 386BE82C  addi r3, r11, -0x17d4
	ctx.r[3].s64 = ctx.r[11].s64 + -6100;
	// 832B22D0: 4AF62B08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B22D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B22D8 size=12
    let mut pc: u32 = 0x832B22D8;
    'dispatch: loop {
        match pc {
            0x832B22D8 => {
    //   block [0x832B22D8..0x832B22E4)
	// 832B22D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B22DC: 386BE830  addi r3, r11, -0x17d0
	ctx.r[3].s64 = ctx.r[11].s64 + -6096;
	// 832B22E0: 4AF62AF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B22E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B22E8 size=12
    let mut pc: u32 = 0x832B22E8;
    'dispatch: loop {
        match pc {
            0x832B22E8 => {
    //   block [0x832B22E8..0x832B22F4)
	// 832B22E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B22EC: 386BE834  addi r3, r11, -0x17cc
	ctx.r[3].s64 = ctx.r[11].s64 + -6092;
	// 832B22F0: 4AF62AE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B22F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B22F8 size=12
    let mut pc: u32 = 0x832B22F8;
    'dispatch: loop {
        match pc {
            0x832B22F8 => {
    //   block [0x832B22F8..0x832B2304)
	// 832B22F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B22FC: 386BE838  addi r3, r11, -0x17c8
	ctx.r[3].s64 = ctx.r[11].s64 + -6088;
	// 832B2300: 4AF62AD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2308 size=12
    let mut pc: u32 = 0x832B2308;
    'dispatch: loop {
        match pc {
            0x832B2308 => {
    //   block [0x832B2308..0x832B2314)
	// 832B2308: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B230C: 386BE83C  addi r3, r11, -0x17c4
	ctx.r[3].s64 = ctx.r[11].s64 + -6084;
	// 832B2310: 4AF62AC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2318 size=12
    let mut pc: u32 = 0x832B2318;
    'dispatch: loop {
        match pc {
            0x832B2318 => {
    //   block [0x832B2318..0x832B2324)
	// 832B2318: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B231C: 386BE840  addi r3, r11, -0x17c0
	ctx.r[3].s64 = ctx.r[11].s64 + -6080;
	// 832B2320: 4AF62AB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2328 size=12
    let mut pc: u32 = 0x832B2328;
    'dispatch: loop {
        match pc {
            0x832B2328 => {
    //   block [0x832B2328..0x832B2334)
	// 832B2328: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B232C: 386BE844  addi r3, r11, -0x17bc
	ctx.r[3].s64 = ctx.r[11].s64 + -6076;
	// 832B2330: 4AF62AA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2338 size=12
    let mut pc: u32 = 0x832B2338;
    'dispatch: loop {
        match pc {
            0x832B2338 => {
    //   block [0x832B2338..0x832B2344)
	// 832B2338: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B233C: 386BE848  addi r3, r11, -0x17b8
	ctx.r[3].s64 = ctx.r[11].s64 + -6072;
	// 832B2340: 4AF62A98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2348 size=12
    let mut pc: u32 = 0x832B2348;
    'dispatch: loop {
        match pc {
            0x832B2348 => {
    //   block [0x832B2348..0x832B2354)
	// 832B2348: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B234C: 386BE84C  addi r3, r11, -0x17b4
	ctx.r[3].s64 = ctx.r[11].s64 + -6068;
	// 832B2350: 4AF62A88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2358 size=12
    let mut pc: u32 = 0x832B2358;
    'dispatch: loop {
        match pc {
            0x832B2358 => {
    //   block [0x832B2358..0x832B2364)
	// 832B2358: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B235C: 386BE850  addi r3, r11, -0x17b0
	ctx.r[3].s64 = ctx.r[11].s64 + -6064;
	// 832B2360: 4AF62A78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2368 size=12
    let mut pc: u32 = 0x832B2368;
    'dispatch: loop {
        match pc {
            0x832B2368 => {
    //   block [0x832B2368..0x832B2374)
	// 832B2368: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B236C: 386BE854  addi r3, r11, -0x17ac
	ctx.r[3].s64 = ctx.r[11].s64 + -6060;
	// 832B2370: 4AF62A68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2378 size=12
    let mut pc: u32 = 0x832B2378;
    'dispatch: loop {
        match pc {
            0x832B2378 => {
    //   block [0x832B2378..0x832B2384)
	// 832B2378: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B237C: 386BE858  addi r3, r11, -0x17a8
	ctx.r[3].s64 = ctx.r[11].s64 + -6056;
	// 832B2380: 4AF62A58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2388 size=12
    let mut pc: u32 = 0x832B2388;
    'dispatch: loop {
        match pc {
            0x832B2388 => {
    //   block [0x832B2388..0x832B2394)
	// 832B2388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B238C: 386BE85C  addi r3, r11, -0x17a4
	ctx.r[3].s64 = ctx.r[11].s64 + -6052;
	// 832B2390: 4AF62A48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2398 size=12
    let mut pc: u32 = 0x832B2398;
    'dispatch: loop {
        match pc {
            0x832B2398 => {
    //   block [0x832B2398..0x832B23A4)
	// 832B2398: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B239C: 386BE860  addi r3, r11, -0x17a0
	ctx.r[3].s64 = ctx.r[11].s64 + -6048;
	// 832B23A0: 4AF62A38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B23A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B23A8 size=12
    let mut pc: u32 = 0x832B23A8;
    'dispatch: loop {
        match pc {
            0x832B23A8 => {
    //   block [0x832B23A8..0x832B23B4)
	// 832B23A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B23AC: 386BE864  addi r3, r11, -0x179c
	ctx.r[3].s64 = ctx.r[11].s64 + -6044;
	// 832B23B0: 4AF62A28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B23B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B23B8 size=12
    let mut pc: u32 = 0x832B23B8;
    'dispatch: loop {
        match pc {
            0x832B23B8 => {
    //   block [0x832B23B8..0x832B23C4)
	// 832B23B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B23BC: 386BE868  addi r3, r11, -0x1798
	ctx.r[3].s64 = ctx.r[11].s64 + -6040;
	// 832B23C0: 4AF62A18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B23C8 size=12
    let mut pc: u32 = 0x832B23C8;
    'dispatch: loop {
        match pc {
            0x832B23C8 => {
    //   block [0x832B23C8..0x832B23D4)
	// 832B23C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B23CC: 386BE86C  addi r3, r11, -0x1794
	ctx.r[3].s64 = ctx.r[11].s64 + -6036;
	// 832B23D0: 4AF62A08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B23D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B23D8 size=12
    let mut pc: u32 = 0x832B23D8;
    'dispatch: loop {
        match pc {
            0x832B23D8 => {
    //   block [0x832B23D8..0x832B23E4)
	// 832B23D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B23DC: 386BE870  addi r3, r11, -0x1790
	ctx.r[3].s64 = ctx.r[11].s64 + -6032;
	// 832B23E0: 4AF629F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B23E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B23E8 size=12
    let mut pc: u32 = 0x832B23E8;
    'dispatch: loop {
        match pc {
            0x832B23E8 => {
    //   block [0x832B23E8..0x832B23F4)
	// 832B23E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B23EC: 386BE874  addi r3, r11, -0x178c
	ctx.r[3].s64 = ctx.r[11].s64 + -6028;
	// 832B23F0: 4AF629E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B23F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B23F8 size=12
    let mut pc: u32 = 0x832B23F8;
    'dispatch: loop {
        match pc {
            0x832B23F8 => {
    //   block [0x832B23F8..0x832B2404)
	// 832B23F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B23FC: 386BE878  addi r3, r11, -0x1788
	ctx.r[3].s64 = ctx.r[11].s64 + -6024;
	// 832B2400: 4AF629D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2408 size=12
    let mut pc: u32 = 0x832B2408;
    'dispatch: loop {
        match pc {
            0x832B2408 => {
    //   block [0x832B2408..0x832B2414)
	// 832B2408: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B240C: 386BE87C  addi r3, r11, -0x1784
	ctx.r[3].s64 = ctx.r[11].s64 + -6020;
	// 832B2410: 4AF629C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2418 size=12
    let mut pc: u32 = 0x832B2418;
    'dispatch: loop {
        match pc {
            0x832B2418 => {
    //   block [0x832B2418..0x832B2424)
	// 832B2418: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B241C: 386BE880  addi r3, r11, -0x1780
	ctx.r[3].s64 = ctx.r[11].s64 + -6016;
	// 832B2420: 4AF629B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2428 size=12
    let mut pc: u32 = 0x832B2428;
    'dispatch: loop {
        match pc {
            0x832B2428 => {
    //   block [0x832B2428..0x832B2434)
	// 832B2428: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B242C: 386BE884  addi r3, r11, -0x177c
	ctx.r[3].s64 = ctx.r[11].s64 + -6012;
	// 832B2430: 4AF629A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2438 size=12
    let mut pc: u32 = 0x832B2438;
    'dispatch: loop {
        match pc {
            0x832B2438 => {
    //   block [0x832B2438..0x832B2444)
	// 832B2438: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B243C: 386BE888  addi r3, r11, -0x1778
	ctx.r[3].s64 = ctx.r[11].s64 + -6008;
	// 832B2440: 4AF62998  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2448 size=12
    let mut pc: u32 = 0x832B2448;
    'dispatch: loop {
        match pc {
            0x832B2448 => {
    //   block [0x832B2448..0x832B2454)
	// 832B2448: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B244C: 386BE88C  addi r3, r11, -0x1774
	ctx.r[3].s64 = ctx.r[11].s64 + -6004;
	// 832B2450: 4AF62988  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2458 size=12
    let mut pc: u32 = 0x832B2458;
    'dispatch: loop {
        match pc {
            0x832B2458 => {
    //   block [0x832B2458..0x832B2464)
	// 832B2458: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B245C: 386BE890  addi r3, r11, -0x1770
	ctx.r[3].s64 = ctx.r[11].s64 + -6000;
	// 832B2460: 4AF62978  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2468 size=12
    let mut pc: u32 = 0x832B2468;
    'dispatch: loop {
        match pc {
            0x832B2468 => {
    //   block [0x832B2468..0x832B2474)
	// 832B2468: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B246C: 386BE894  addi r3, r11, -0x176c
	ctx.r[3].s64 = ctx.r[11].s64 + -5996;
	// 832B2470: 4AF62968  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2478 size=12
    let mut pc: u32 = 0x832B2478;
    'dispatch: loop {
        match pc {
            0x832B2478 => {
    //   block [0x832B2478..0x832B2484)
	// 832B2478: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B247C: 386BE898  addi r3, r11, -0x1768
	ctx.r[3].s64 = ctx.r[11].s64 + -5992;
	// 832B2480: 4AF62958  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2488 size=12
    let mut pc: u32 = 0x832B2488;
    'dispatch: loop {
        match pc {
            0x832B2488 => {
    //   block [0x832B2488..0x832B2494)
	// 832B2488: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B248C: 386BE89C  addi r3, r11, -0x1764
	ctx.r[3].s64 = ctx.r[11].s64 + -5988;
	// 832B2490: 4AF62948  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2498 size=12
    let mut pc: u32 = 0x832B2498;
    'dispatch: loop {
        match pc {
            0x832B2498 => {
    //   block [0x832B2498..0x832B24A4)
	// 832B2498: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B249C: 386BE8A0  addi r3, r11, -0x1760
	ctx.r[3].s64 = ctx.r[11].s64 + -5984;
	// 832B24A0: 4AF62938  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B24A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B24A8 size=12
    let mut pc: u32 = 0x832B24A8;
    'dispatch: loop {
        match pc {
            0x832B24A8 => {
    //   block [0x832B24A8..0x832B24B4)
	// 832B24A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B24AC: 386BE8A4  addi r3, r11, -0x175c
	ctx.r[3].s64 = ctx.r[11].s64 + -5980;
	// 832B24B0: 4AF62928  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B24B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B24B8 size=12
    let mut pc: u32 = 0x832B24B8;
    'dispatch: loop {
        match pc {
            0x832B24B8 => {
    //   block [0x832B24B8..0x832B24C4)
	// 832B24B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B24BC: 386BE8A8  addi r3, r11, -0x1758
	ctx.r[3].s64 = ctx.r[11].s64 + -5976;
	// 832B24C0: 4AF62918  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B24C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B24C8 size=12
    let mut pc: u32 = 0x832B24C8;
    'dispatch: loop {
        match pc {
            0x832B24C8 => {
    //   block [0x832B24C8..0x832B24D4)
	// 832B24C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B24CC: 386BE8AC  addi r3, r11, -0x1754
	ctx.r[3].s64 = ctx.r[11].s64 + -5972;
	// 832B24D0: 4AF62908  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B24D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B24D8 size=12
    let mut pc: u32 = 0x832B24D8;
    'dispatch: loop {
        match pc {
            0x832B24D8 => {
    //   block [0x832B24D8..0x832B24E4)
	// 832B24D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B24DC: 386BE8B0  addi r3, r11, -0x1750
	ctx.r[3].s64 = ctx.r[11].s64 + -5968;
	// 832B24E0: 4AF628F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B24E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B24E8 size=12
    let mut pc: u32 = 0x832B24E8;
    'dispatch: loop {
        match pc {
            0x832B24E8 => {
    //   block [0x832B24E8..0x832B24F4)
	// 832B24E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B24EC: 386BE8B4  addi r3, r11, -0x174c
	ctx.r[3].s64 = ctx.r[11].s64 + -5964;
	// 832B24F0: 4AF628E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B24F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B24F8 size=12
    let mut pc: u32 = 0x832B24F8;
    'dispatch: loop {
        match pc {
            0x832B24F8 => {
    //   block [0x832B24F8..0x832B2504)
	// 832B24F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B24FC: 386BE8B8  addi r3, r11, -0x1748
	ctx.r[3].s64 = ctx.r[11].s64 + -5960;
	// 832B2500: 4AF628D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2508 size=12
    let mut pc: u32 = 0x832B2508;
    'dispatch: loop {
        match pc {
            0x832B2508 => {
    //   block [0x832B2508..0x832B2514)
	// 832B2508: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B250C: 386BE8BC  addi r3, r11, -0x1744
	ctx.r[3].s64 = ctx.r[11].s64 + -5956;
	// 832B2510: 4AF628C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2518 size=12
    let mut pc: u32 = 0x832B2518;
    'dispatch: loop {
        match pc {
            0x832B2518 => {
    //   block [0x832B2518..0x832B2524)
	// 832B2518: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B251C: 386BE8C0  addi r3, r11, -0x1740
	ctx.r[3].s64 = ctx.r[11].s64 + -5952;
	// 832B2520: 4AF628B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2528 size=12
    let mut pc: u32 = 0x832B2528;
    'dispatch: loop {
        match pc {
            0x832B2528 => {
    //   block [0x832B2528..0x832B2534)
	// 832B2528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B252C: 386BE8C4  addi r3, r11, -0x173c
	ctx.r[3].s64 = ctx.r[11].s64 + -5948;
	// 832B2530: 4AF628A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2538 size=12
    let mut pc: u32 = 0x832B2538;
    'dispatch: loop {
        match pc {
            0x832B2538 => {
    //   block [0x832B2538..0x832B2544)
	// 832B2538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B253C: 386BE8C8  addi r3, r11, -0x1738
	ctx.r[3].s64 = ctx.r[11].s64 + -5944;
	// 832B2540: 4AF62898  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2548 size=12
    let mut pc: u32 = 0x832B2548;
    'dispatch: loop {
        match pc {
            0x832B2548 => {
    //   block [0x832B2548..0x832B2554)
	// 832B2548: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B254C: 386BE8CC  addi r3, r11, -0x1734
	ctx.r[3].s64 = ctx.r[11].s64 + -5940;
	// 832B2550: 4AF62888  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2558 size=12
    let mut pc: u32 = 0x832B2558;
    'dispatch: loop {
        match pc {
            0x832B2558 => {
    //   block [0x832B2558..0x832B2564)
	// 832B2558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B255C: 386BE8D0  addi r3, r11, -0x1730
	ctx.r[3].s64 = ctx.r[11].s64 + -5936;
	// 832B2560: 4AF62878  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2568 size=12
    let mut pc: u32 = 0x832B2568;
    'dispatch: loop {
        match pc {
            0x832B2568 => {
    //   block [0x832B2568..0x832B2574)
	// 832B2568: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B256C: 386BE8D4  addi r3, r11, -0x172c
	ctx.r[3].s64 = ctx.r[11].s64 + -5932;
	// 832B2570: 4AF62868  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2578 size=12
    let mut pc: u32 = 0x832B2578;
    'dispatch: loop {
        match pc {
            0x832B2578 => {
    //   block [0x832B2578..0x832B2584)
	// 832B2578: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B257C: 386BE8D8  addi r3, r11, -0x1728
	ctx.r[3].s64 = ctx.r[11].s64 + -5928;
	// 832B2580: 4AF62858  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2588 size=12
    let mut pc: u32 = 0x832B2588;
    'dispatch: loop {
        match pc {
            0x832B2588 => {
    //   block [0x832B2588..0x832B2594)
	// 832B2588: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B258C: 386BE8DC  addi r3, r11, -0x1724
	ctx.r[3].s64 = ctx.r[11].s64 + -5924;
	// 832B2590: 4AF62848  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2598 size=12
    let mut pc: u32 = 0x832B2598;
    'dispatch: loop {
        match pc {
            0x832B2598 => {
    //   block [0x832B2598..0x832B25A4)
	// 832B2598: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B259C: 386BE8E0  addi r3, r11, -0x1720
	ctx.r[3].s64 = ctx.r[11].s64 + -5920;
	// 832B25A0: 4AF62838  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B25A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B25A8 size=12
    let mut pc: u32 = 0x832B25A8;
    'dispatch: loop {
        match pc {
            0x832B25A8 => {
    //   block [0x832B25A8..0x832B25B4)
	// 832B25A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B25AC: 386BE8E4  addi r3, r11, -0x171c
	ctx.r[3].s64 = ctx.r[11].s64 + -5916;
	// 832B25B0: 4AF62828  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B25B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B25B8 size=12
    let mut pc: u32 = 0x832B25B8;
    'dispatch: loop {
        match pc {
            0x832B25B8 => {
    //   block [0x832B25B8..0x832B25C4)
	// 832B25B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B25BC: 386BE8E8  addi r3, r11, -0x1718
	ctx.r[3].s64 = ctx.r[11].s64 + -5912;
	// 832B25C0: 4AF62818  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B25C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B25C8 size=12
    let mut pc: u32 = 0x832B25C8;
    'dispatch: loop {
        match pc {
            0x832B25C8 => {
    //   block [0x832B25C8..0x832B25D4)
	// 832B25C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B25CC: 386BE8EC  addi r3, r11, -0x1714
	ctx.r[3].s64 = ctx.r[11].s64 + -5908;
	// 832B25D0: 4AF62808  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B25D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B25D8 size=12
    let mut pc: u32 = 0x832B25D8;
    'dispatch: loop {
        match pc {
            0x832B25D8 => {
    //   block [0x832B25D8..0x832B25E4)
	// 832B25D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B25DC: 386BE930  addi r3, r11, -0x16d0
	ctx.r[3].s64 = ctx.r[11].s64 + -5840;
	// 832B25E0: 4AF627F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B25E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B25E8 size=12
    let mut pc: u32 = 0x832B25E8;
    'dispatch: loop {
        match pc {
            0x832B25E8 => {
    //   block [0x832B25E8..0x832B25F4)
	// 832B25E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B25EC: 386BE934  addi r3, r11, -0x16cc
	ctx.r[3].s64 = ctx.r[11].s64 + -5836;
	// 832B25F0: 4AF627E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B25F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B25F8 size=12
    let mut pc: u32 = 0x832B25F8;
    'dispatch: loop {
        match pc {
            0x832B25F8 => {
    //   block [0x832B25F8..0x832B2604)
	// 832B25F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B25FC: 386BE938  addi r3, r11, -0x16c8
	ctx.r[3].s64 = ctx.r[11].s64 + -5832;
	// 832B2600: 4AF627D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2608 size=12
    let mut pc: u32 = 0x832B2608;
    'dispatch: loop {
        match pc {
            0x832B2608 => {
    //   block [0x832B2608..0x832B2614)
	// 832B2608: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B260C: 386BE93C  addi r3, r11, -0x16c4
	ctx.r[3].s64 = ctx.r[11].s64 + -5828;
	// 832B2610: 4AF627C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2618 size=12
    let mut pc: u32 = 0x832B2618;
    'dispatch: loop {
        match pc {
            0x832B2618 => {
    //   block [0x832B2618..0x832B2624)
	// 832B2618: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B261C: 386BE940  addi r3, r11, -0x16c0
	ctx.r[3].s64 = ctx.r[11].s64 + -5824;
	// 832B2620: 4AF627B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2628 size=12
    let mut pc: u32 = 0x832B2628;
    'dispatch: loop {
        match pc {
            0x832B2628 => {
    //   block [0x832B2628..0x832B2634)
	// 832B2628: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B262C: 386BE944  addi r3, r11, -0x16bc
	ctx.r[3].s64 = ctx.r[11].s64 + -5820;
	// 832B2630: 4AF627A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2638 size=12
    let mut pc: u32 = 0x832B2638;
    'dispatch: loop {
        match pc {
            0x832B2638 => {
    //   block [0x832B2638..0x832B2644)
	// 832B2638: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B263C: 386BE948  addi r3, r11, -0x16b8
	ctx.r[3].s64 = ctx.r[11].s64 + -5816;
	// 832B2640: 4AF62798  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2648 size=12
    let mut pc: u32 = 0x832B2648;
    'dispatch: loop {
        match pc {
            0x832B2648 => {
    //   block [0x832B2648..0x832B2654)
	// 832B2648: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B264C: 386BE94C  addi r3, r11, -0x16b4
	ctx.r[3].s64 = ctx.r[11].s64 + -5812;
	// 832B2650: 4AF62788  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2658 size=12
    let mut pc: u32 = 0x832B2658;
    'dispatch: loop {
        match pc {
            0x832B2658 => {
    //   block [0x832B2658..0x832B2664)
	// 832B2658: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B265C: 386BE950  addi r3, r11, -0x16b0
	ctx.r[3].s64 = ctx.r[11].s64 + -5808;
	// 832B2660: 4AF62778  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2668 size=12
    let mut pc: u32 = 0x832B2668;
    'dispatch: loop {
        match pc {
            0x832B2668 => {
    //   block [0x832B2668..0x832B2674)
	// 832B2668: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B266C: 386BE954  addi r3, r11, -0x16ac
	ctx.r[3].s64 = ctx.r[11].s64 + -5804;
	// 832B2670: 4AF62768  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2678 size=12
    let mut pc: u32 = 0x832B2678;
    'dispatch: loop {
        match pc {
            0x832B2678 => {
    //   block [0x832B2678..0x832B2684)
	// 832B2678: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B267C: 386BE958  addi r3, r11, -0x16a8
	ctx.r[3].s64 = ctx.r[11].s64 + -5800;
	// 832B2680: 4AF62758  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2688 size=12
    let mut pc: u32 = 0x832B2688;
    'dispatch: loop {
        match pc {
            0x832B2688 => {
    //   block [0x832B2688..0x832B2694)
	// 832B2688: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B268C: 386BE95C  addi r3, r11, -0x16a4
	ctx.r[3].s64 = ctx.r[11].s64 + -5796;
	// 832B2690: 4AF62748  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2698 size=12
    let mut pc: u32 = 0x832B2698;
    'dispatch: loop {
        match pc {
            0x832B2698 => {
    //   block [0x832B2698..0x832B26A4)
	// 832B2698: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B269C: 386BE960  addi r3, r11, -0x16a0
	ctx.r[3].s64 = ctx.r[11].s64 + -5792;
	// 832B26A0: 4AF62738  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B26A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B26A8 size=12
    let mut pc: u32 = 0x832B26A8;
    'dispatch: loop {
        match pc {
            0x832B26A8 => {
    //   block [0x832B26A8..0x832B26B4)
	// 832B26A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B26AC: 386BE964  addi r3, r11, -0x169c
	ctx.r[3].s64 = ctx.r[11].s64 + -5788;
	// 832B26B0: 4AF62728  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B26B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B26B8 size=12
    let mut pc: u32 = 0x832B26B8;
    'dispatch: loop {
        match pc {
            0x832B26B8 => {
    //   block [0x832B26B8..0x832B26C4)
	// 832B26B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B26BC: 386BE968  addi r3, r11, -0x1698
	ctx.r[3].s64 = ctx.r[11].s64 + -5784;
	// 832B26C0: 4AF62718  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B26C8 size=12
    let mut pc: u32 = 0x832B26C8;
    'dispatch: loop {
        match pc {
            0x832B26C8 => {
    //   block [0x832B26C8..0x832B26D4)
	// 832B26C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B26CC: 386BE96C  addi r3, r11, -0x1694
	ctx.r[3].s64 = ctx.r[11].s64 + -5780;
	// 832B26D0: 4AF62708  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B26D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B26D8 size=12
    let mut pc: u32 = 0x832B26D8;
    'dispatch: loop {
        match pc {
            0x832B26D8 => {
    //   block [0x832B26D8..0x832B26E4)
	// 832B26D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B26DC: 386BE970  addi r3, r11, -0x1690
	ctx.r[3].s64 = ctx.r[11].s64 + -5776;
	// 832B26E0: 4AF626F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B26E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B26E8 size=12
    let mut pc: u32 = 0x832B26E8;
    'dispatch: loop {
        match pc {
            0x832B26E8 => {
    //   block [0x832B26E8..0x832B26F4)
	// 832B26E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B26EC: 386BE974  addi r3, r11, -0x168c
	ctx.r[3].s64 = ctx.r[11].s64 + -5772;
	// 832B26F0: 4AF626E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B26F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B26F8 size=12
    let mut pc: u32 = 0x832B26F8;
    'dispatch: loop {
        match pc {
            0x832B26F8 => {
    //   block [0x832B26F8..0x832B2704)
	// 832B26F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B26FC: 386BE978  addi r3, r11, -0x1688
	ctx.r[3].s64 = ctx.r[11].s64 + -5768;
	// 832B2700: 4AF626D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2708 size=12
    let mut pc: u32 = 0x832B2708;
    'dispatch: loop {
        match pc {
            0x832B2708 => {
    //   block [0x832B2708..0x832B2714)
	// 832B2708: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B270C: 386BE97C  addi r3, r11, -0x1684
	ctx.r[3].s64 = ctx.r[11].s64 + -5764;
	// 832B2710: 4AF626C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2718 size=12
    let mut pc: u32 = 0x832B2718;
    'dispatch: loop {
        match pc {
            0x832B2718 => {
    //   block [0x832B2718..0x832B2724)
	// 832B2718: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B271C: 386BE980  addi r3, r11, -0x1680
	ctx.r[3].s64 = ctx.r[11].s64 + -5760;
	// 832B2720: 4AF626B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2728 size=12
    let mut pc: u32 = 0x832B2728;
    'dispatch: loop {
        match pc {
            0x832B2728 => {
    //   block [0x832B2728..0x832B2734)
	// 832B2728: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B272C: 386BE984  addi r3, r11, -0x167c
	ctx.r[3].s64 = ctx.r[11].s64 + -5756;
	// 832B2730: 4AF626A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2738 size=12
    let mut pc: u32 = 0x832B2738;
    'dispatch: loop {
        match pc {
            0x832B2738 => {
    //   block [0x832B2738..0x832B2744)
	// 832B2738: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B273C: 386BE988  addi r3, r11, -0x1678
	ctx.r[3].s64 = ctx.r[11].s64 + -5752;
	// 832B2740: 4AF62698  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2748 size=12
    let mut pc: u32 = 0x832B2748;
    'dispatch: loop {
        match pc {
            0x832B2748 => {
    //   block [0x832B2748..0x832B2754)
	// 832B2748: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B274C: 386BE98C  addi r3, r11, -0x1674
	ctx.r[3].s64 = ctx.r[11].s64 + -5748;
	// 832B2750: 4AF62688  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2758 size=12
    let mut pc: u32 = 0x832B2758;
    'dispatch: loop {
        match pc {
            0x832B2758 => {
    //   block [0x832B2758..0x832B2764)
	// 832B2758: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B275C: 386BE990  addi r3, r11, -0x1670
	ctx.r[3].s64 = ctx.r[11].s64 + -5744;
	// 832B2760: 4AF62678  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2768 size=12
    let mut pc: u32 = 0x832B2768;
    'dispatch: loop {
        match pc {
            0x832B2768 => {
    //   block [0x832B2768..0x832B2774)
	// 832B2768: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B276C: 386BE994  addi r3, r11, -0x166c
	ctx.r[3].s64 = ctx.r[11].s64 + -5740;
	// 832B2770: 4AF62668  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2778 size=12
    let mut pc: u32 = 0x832B2778;
    'dispatch: loop {
        match pc {
            0x832B2778 => {
    //   block [0x832B2778..0x832B2784)
	// 832B2778: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B277C: 386BE998  addi r3, r11, -0x1668
	ctx.r[3].s64 = ctx.r[11].s64 + -5736;
	// 832B2780: 4AF62658  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2788 size=12
    let mut pc: u32 = 0x832B2788;
    'dispatch: loop {
        match pc {
            0x832B2788 => {
    //   block [0x832B2788..0x832B2794)
	// 832B2788: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B278C: 386BE99C  addi r3, r11, -0x1664
	ctx.r[3].s64 = ctx.r[11].s64 + -5732;
	// 832B2790: 4AF62648  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2798 size=12
    let mut pc: u32 = 0x832B2798;
    'dispatch: loop {
        match pc {
            0x832B2798 => {
    //   block [0x832B2798..0x832B27A4)
	// 832B2798: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B279C: 386BE9A0  addi r3, r11, -0x1660
	ctx.r[3].s64 = ctx.r[11].s64 + -5728;
	// 832B27A0: 4AF62638  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B27A8 size=12
    let mut pc: u32 = 0x832B27A8;
    'dispatch: loop {
        match pc {
            0x832B27A8 => {
    //   block [0x832B27A8..0x832B27B4)
	// 832B27A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B27AC: 386BE9A4  addi r3, r11, -0x165c
	ctx.r[3].s64 = ctx.r[11].s64 + -5724;
	// 832B27B0: 4AF62628  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B27B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B27B8 size=12
    let mut pc: u32 = 0x832B27B8;
    'dispatch: loop {
        match pc {
            0x832B27B8 => {
    //   block [0x832B27B8..0x832B27C4)
	// 832B27B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B27BC: 386BE9A8  addi r3, r11, -0x1658
	ctx.r[3].s64 = ctx.r[11].s64 + -5720;
	// 832B27C0: 4AF62618  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B27C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B27C8 size=12
    let mut pc: u32 = 0x832B27C8;
    'dispatch: loop {
        match pc {
            0x832B27C8 => {
    //   block [0x832B27C8..0x832B27D4)
	// 832B27C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B27CC: 386BE9AC  addi r3, r11, -0x1654
	ctx.r[3].s64 = ctx.r[11].s64 + -5716;
	// 832B27D0: 4AF62608  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B27D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B27D8 size=12
    let mut pc: u32 = 0x832B27D8;
    'dispatch: loop {
        match pc {
            0x832B27D8 => {
    //   block [0x832B27D8..0x832B27E4)
	// 832B27D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B27DC: 386BE9B0  addi r3, r11, -0x1650
	ctx.r[3].s64 = ctx.r[11].s64 + -5712;
	// 832B27E0: 4AF625F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B27E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B27E8 size=12
    let mut pc: u32 = 0x832B27E8;
    'dispatch: loop {
        match pc {
            0x832B27E8 => {
    //   block [0x832B27E8..0x832B27F4)
	// 832B27E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B27EC: 386BE9B4  addi r3, r11, -0x164c
	ctx.r[3].s64 = ctx.r[11].s64 + -5708;
	// 832B27F0: 4AF625E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B27F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B27F8 size=12
    let mut pc: u32 = 0x832B27F8;
    'dispatch: loop {
        match pc {
            0x832B27F8 => {
    //   block [0x832B27F8..0x832B2804)
	// 832B27F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B27FC: 386BE9B8  addi r3, r11, -0x1648
	ctx.r[3].s64 = ctx.r[11].s64 + -5704;
	// 832B2800: 4AF625D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2808 size=12
    let mut pc: u32 = 0x832B2808;
    'dispatch: loop {
        match pc {
            0x832B2808 => {
    //   block [0x832B2808..0x832B2814)
	// 832B2808: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B280C: 386BE9BC  addi r3, r11, -0x1644
	ctx.r[3].s64 = ctx.r[11].s64 + -5700;
	// 832B2810: 4AF625C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2818 size=12
    let mut pc: u32 = 0x832B2818;
    'dispatch: loop {
        match pc {
            0x832B2818 => {
    //   block [0x832B2818..0x832B2824)
	// 832B2818: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B281C: 386BE9C0  addi r3, r11, -0x1640
	ctx.r[3].s64 = ctx.r[11].s64 + -5696;
	// 832B2820: 4AF625B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2828 size=12
    let mut pc: u32 = 0x832B2828;
    'dispatch: loop {
        match pc {
            0x832B2828 => {
    //   block [0x832B2828..0x832B2834)
	// 832B2828: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B282C: 386BE9C4  addi r3, r11, -0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + -5692;
	// 832B2830: 4AF625A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2838 size=12
    let mut pc: u32 = 0x832B2838;
    'dispatch: loop {
        match pc {
            0x832B2838 => {
    //   block [0x832B2838..0x832B2844)
	// 832B2838: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B283C: 386B3924  addi r3, r11, 0x3924
	ctx.r[3].s64 = ctx.r[11].s64 + 14628;
	// 832B2840: 4B520730  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2848 size=12
    let mut pc: u32 = 0x832B2848;
    'dispatch: loop {
        match pc {
            0x832B2848 => {
    //   block [0x832B2848..0x832B2854)
	// 832B2848: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B284C: 386B3914  addi r3, r11, 0x3914
	ctx.r[3].s64 = ctx.r[11].s64 + 14612;
	// 832B2850: 4B520720  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2858 size=12
    let mut pc: u32 = 0x832B2858;
    'dispatch: loop {
        match pc {
            0x832B2858 => {
    //   block [0x832B2858..0x832B2864)
	// 832B2858: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B285C: 386B3904  addi r3, r11, 0x3904
	ctx.r[3].s64 = ctx.r[11].s64 + 14596;
	// 832B2860: 4B520710  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2868 size=12
    let mut pc: u32 = 0x832B2868;
    'dispatch: loop {
        match pc {
            0x832B2868 => {
    //   block [0x832B2868..0x832B2874)
	// 832B2868: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B286C: 386B38F4  addi r3, r11, 0x38f4
	ctx.r[3].s64 = ctx.r[11].s64 + 14580;
	// 832B2870: 4B520700  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2878 size=12
    let mut pc: u32 = 0x832B2878;
    'dispatch: loop {
        match pc {
            0x832B2878 => {
    //   block [0x832B2878..0x832B2884)
	// 832B2878: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B287C: 386BE9C8  addi r3, r11, -0x1638
	ctx.r[3].s64 = ctx.r[11].s64 + -5688;
	// 832B2880: 4AF62558  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2888 size=12
    let mut pc: u32 = 0x832B2888;
    'dispatch: loop {
        match pc {
            0x832B2888 => {
    //   block [0x832B2888..0x832B2894)
	// 832B2888: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B288C: 386BE9CC  addi r3, r11, -0x1634
	ctx.r[3].s64 = ctx.r[11].s64 + -5684;
	// 832B2890: 4AF62548  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2898 size=12
    let mut pc: u32 = 0x832B2898;
    'dispatch: loop {
        match pc {
            0x832B2898 => {
    //   block [0x832B2898..0x832B28A4)
	// 832B2898: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B289C: 386BE9D0  addi r3, r11, -0x1630
	ctx.r[3].s64 = ctx.r[11].s64 + -5680;
	// 832B28A0: 4AF62538  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B28A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B28A8 size=12
    let mut pc: u32 = 0x832B28A8;
    'dispatch: loop {
        match pc {
            0x832B28A8 => {
    //   block [0x832B28A8..0x832B28B4)
	// 832B28A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B28AC: 386BE9D4  addi r3, r11, -0x162c
	ctx.r[3].s64 = ctx.r[11].s64 + -5676;
	// 832B28B0: 4AF62528  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B28B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B28B8 size=12
    let mut pc: u32 = 0x832B28B8;
    'dispatch: loop {
        match pc {
            0x832B28B8 => {
    //   block [0x832B28B8..0x832B28C4)
	// 832B28B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B28BC: 386BE9D8  addi r3, r11, -0x1628
	ctx.r[3].s64 = ctx.r[11].s64 + -5672;
	// 832B28C0: 4AF62518  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B28C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B28C8 size=12
    let mut pc: u32 = 0x832B28C8;
    'dispatch: loop {
        match pc {
            0x832B28C8 => {
    //   block [0x832B28C8..0x832B28D4)
	// 832B28C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B28CC: 386BE9DC  addi r3, r11, -0x1624
	ctx.r[3].s64 = ctx.r[11].s64 + -5668;
	// 832B28D0: 4AF62508  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B28D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B28D8 size=12
    let mut pc: u32 = 0x832B28D8;
    'dispatch: loop {
        match pc {
            0x832B28D8 => {
    //   block [0x832B28D8..0x832B28E4)
	// 832B28D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B28DC: 386BE9E0  addi r3, r11, -0x1620
	ctx.r[3].s64 = ctx.r[11].s64 + -5664;
	// 832B28E0: 4AF624F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B28E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B28E8 size=12
    let mut pc: u32 = 0x832B28E8;
    'dispatch: loop {
        match pc {
            0x832B28E8 => {
    //   block [0x832B28E8..0x832B28F4)
	// 832B28E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B28EC: 386BE9E4  addi r3, r11, -0x161c
	ctx.r[3].s64 = ctx.r[11].s64 + -5660;
	// 832B28F0: 4AF624E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B28F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B28F8 size=12
    let mut pc: u32 = 0x832B28F8;
    'dispatch: loop {
        match pc {
            0x832B28F8 => {
    //   block [0x832B28F8..0x832B2904)
	// 832B28F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B28FC: 386BE9E8  addi r3, r11, -0x1618
	ctx.r[3].s64 = ctx.r[11].s64 + -5656;
	// 832B2900: 4AF624D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2908 size=12
    let mut pc: u32 = 0x832B2908;
    'dispatch: loop {
        match pc {
            0x832B2908 => {
    //   block [0x832B2908..0x832B2914)
	// 832B2908: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B290C: 386BE9EC  addi r3, r11, -0x1614
	ctx.r[3].s64 = ctx.r[11].s64 + -5652;
	// 832B2910: 4AF05208  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2918 size=132
    let mut pc: u32 = 0x832B2918;
    'dispatch: loop {
        match pc {
            0x832B2918 => {
    //   block [0x832B2918..0x832B295C)
	// 832B2918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B292C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2930: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B2934: 3BEBE9F4  addi r31, r11, -0x160c
	ctx.r[31].s64 = ctx.r[11].s64 + -5644;
	// 832B2938: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B293C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B2940: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B2944: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2948: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B294C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2950: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B2954: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B2958: 419A0020  beq cr6, 0x832b2978
	if ctx.cr[6].eq {
	pc = 0x832B2978; continue 'dispatch;
	}
	pc = 0x832B295C; continue 'dispatch;
            }
            0x832B295C => {
    //   block [0x832B295C..0x832B2978)
	// 832B295C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B2960: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B2964: 4AF693D5  bl 0x8221bd38
	ctx.lr = 0x832B2968;
	sub_8221BD38(ctx, base);
	// 832B2968: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B296C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B2970: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B2974: 409AFFE8  bne cr6, 0x832b295c
	if !ctx.cr[6].eq {
	pc = 0x832B295C; continue 'dispatch;
	}
	pc = 0x832B2978; continue 'dispatch;
            }
            0x832B2978 => {
    //   block [0x832B2978..0x832B299C)
	// 832B2978: 4AF693C1  bl 0x8221bd38
	ctx.lr = 0x832B297C;
	sub_8221BD38(ctx, base);
	// 832B297C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2980: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B2984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B298C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2990: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B29A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B29A0 size=84
    let mut pc: u32 = 0x832B29A0;
    'dispatch: loop {
        match pc {
            0x832B29A0 => {
    //   block [0x832B29A0..0x832B29C8)
	// 832B29A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B29A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B29A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B29AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B29B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B29B4: 3BEBEA00  addi r31, r11, -0x1600
	ctx.r[31].s64 = ctx.r[11].s64 + -5632;
	// 832B29B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B29BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B29C0: 419A0008  beq cr6, 0x832b29c8
	if ctx.cr[6].eq {
	pc = 0x832B29C8; continue 'dispatch;
	}
	// 832B29C4: 4AF69375  bl 0x8221bd38
	ctx.lr = 0x832B29C8;
	sub_8221BD38(ctx, base);
	pc = 0x832B29C8; continue 'dispatch;
            }
            0x832B29C8 => {
    //   block [0x832B29C8..0x832B29F4)
	// 832B29C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B29CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B29D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B29D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B29D8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B29DC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B29E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B29E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B29E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B29EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B29F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2A18 size=56
    let mut pc: u32 = 0x832B2A18;
    'dispatch: loop {
        match pc {
            0x832B2A18 => {
    //   block [0x832B2A18..0x832B2A50)
	// 832B2A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2A28: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B2A2C: 807FEA18  lwz r3, -0x15e8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-5608 as u32) ) } as u64;
	// 832B2A30: 4AF69309  bl 0x8221bd38
	ctx.lr = 0x832B2A34;
	sub_8221BD38(ctx, base);
	// 832B2A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2A38: 917FEA18  stw r11, -0x15e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-5608 as u32), ctx.r[11].u32 ) };
	// 832B2A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2A50 size=12
    let mut pc: u32 = 0x832B2A50;
    'dispatch: loop {
        match pc {
            0x832B2A50 => {
    //   block [0x832B2A50..0x832B2A5C)
	// 832B2A50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2A54: 386BEA1C  addi r3, r11, -0x15e4
	ctx.r[3].s64 = ctx.r[11].s64 + -5604;
	// 832B2A58: 4B10C5B0  b 0x823bf008
	sub_823BF008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2A60 size=12
    let mut pc: u32 = 0x832B2A60;
    'dispatch: loop {
        match pc {
            0x832B2A60 => {
    //   block [0x832B2A60..0x832B2A6C)
	// 832B2A60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2A64: 386BEA28  addi r3, r11, -0x15d8
	ctx.r[3].s64 = ctx.r[11].s64 + -5592;
	// 832B2A68: 4B10C5A0  b 0x823bf008
	sub_823BF008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2A70 size=12
    let mut pc: u32 = 0x832B2A70;
    'dispatch: loop {
        match pc {
            0x832B2A70 => {
    //   block [0x832B2A70..0x832B2A7C)
	// 832B2A70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2A74: 386BEA34  addi r3, r11, -0x15cc
	ctx.r[3].s64 = ctx.r[11].s64 + -5580;
	// 832B2A78: 4AF62360  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2A80 size=12
    let mut pc: u32 = 0x832B2A80;
    'dispatch: loop {
        match pc {
            0x832B2A80 => {
    //   block [0x832B2A80..0x832B2A8C)
	// 832B2A80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2A84: 386BEA38  addi r3, r11, -0x15c8
	ctx.r[3].s64 = ctx.r[11].s64 + -5576;
	// 832B2A88: 4AF62350  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2A90 size=12
    let mut pc: u32 = 0x832B2A90;
    'dispatch: loop {
        match pc {
            0x832B2A90 => {
    //   block [0x832B2A90..0x832B2A9C)
	// 832B2A90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2A94: 386BEA50  addi r3, r11, -0x15b0
	ctx.r[3].s64 = ctx.r[11].s64 + -5552;
	// 832B2A98: 4AF62340  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2AA0 size=12
    let mut pc: u32 = 0x832B2AA0;
    'dispatch: loop {
        match pc {
            0x832B2AA0 => {
    //   block [0x832B2AA0..0x832B2AAC)
	// 832B2AA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2AA4: 386BEA54  addi r3, r11, -0x15ac
	ctx.r[3].s64 = ctx.r[11].s64 + -5548;
	// 832B2AA8: 4AF62330  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2AB0 size=192
    let mut pc: u32 = 0x832B2AB0;
    'dispatch: loop {
        match pc {
            0x832B2AB0 => {
    //   block [0x832B2AB0..0x832B2AD8)
	// 832B2AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2AB4: 4B9F6955  bl 0x82ca9408
	ctx.lr = 0x832B2AB8;
	sub_82CA93D0(ctx, base);
	// 832B2AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2ABC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2AC0: 3BA0000E  li r29, 0xe
	ctx.r[29].s64 = 14;
	// 832B2AC4: 396BEA58  addi r11, r11, -0x15a8
	ctx.r[11].s64 = ctx.r[11].s64 + -5544;
	// 832B2AC8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B2ACC: 3BEB00F8  addi r31, r11, 0xf8
	ctx.r[31].s64 = ctx.r[11].s64 + 248;
	// 832B2AD0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B2AD4: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832B2AD8; continue 'dispatch;
            }
            0x832B2AD8 => {
    //   block [0x832B2AD8..0x832B2AE8)
	// 832B2AD8: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B2ADC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B2AE0: 4AF13C89  bl 0x821c6768
	ctx.lr = 0x832B2AE4;
	sub_821C6768(ctx, base);
	// 832B2AE4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	pc = 0x832B2AE8; continue 'dispatch;
            }
            0x832B2AE8 => {
    //   block [0x832B2AE8..0x832B2B14)
	// 832B2AE8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B2AEC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2AF0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B2AF4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B2AF8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B2AFC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2B00: 4082FFE8  bne 0x832b2ae8
	if !ctx.cr[0].eq {
	pc = 0x832B2AE8; continue 'dispatch;
	}
	// 832B2B04: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832B2B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2B0C: 4AF13C5D  bl 0x821c6768
	ctx.lr = 0x832B2B10;
	sub_821C6768(ctx, base);
	// 832B2B10: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	pc = 0x832B2B14; continue 'dispatch;
            }
            0x832B2B14 => {
    //   block [0x832B2B14..0x832B2B40)
	// 832B2B14: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832B2B18: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2B1C: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832B2B20: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832B2B24: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B2B28: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2B2C: 4082FFE8  bne 0x832b2b14
	if !ctx.cr[0].eq {
	pc = 0x832B2B14; continue 'dispatch;
	}
	// 832B2B30: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B2B34: 387FFFF8  addi r3, r31, -8
	ctx.r[3].s64 = ctx.r[31].s64 + -8;
	// 832B2B38: 4AF13C31  bl 0x821c6768
	ctx.lr = 0x832B2B3C;
	sub_821C6768(ctx, base);
	// 832B2B3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x832B2B40; continue 'dispatch;
            }
            0x832B2B40 => {
    //   block [0x832B2B40..0x832B2B70)
	// 832B2B40: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 832B2B44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2B48: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 832B2B4C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832B2B50: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B2B54: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2B58: 4082FFE8  bne 0x832b2b40
	if !ctx.cr[0].eq {
	pc = 0x832B2B40; continue 'dispatch;
	}
	// 832B2B5C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832B2B60: 939FFFF8  stw r28, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 832B2B64: 4080FF74  bge 0x832b2ad8
	if !ctx.cr[0].lt {
	pc = 0x832B2AD8; continue 'dispatch;
	}
	// 832B2B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B2B6C: 4B9F68EC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2B70 size=192
    let mut pc: u32 = 0x832B2B70;
    'dispatch: loop {
        match pc {
            0x832B2B70 => {
    //   block [0x832B2B70..0x832B2B98)
	// 832B2B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2B74: 4B9F6895  bl 0x82ca9408
	ctx.lr = 0x832B2B78;
	sub_82CA93D0(ctx, base);
	// 832B2B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2B7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2B80: 3BA000C1  li r29, 0xc1
	ctx.r[29].s64 = 193;
	// 832B2B84: 396BEB48  addi r11, r11, -0x14b8
	ctx.r[11].s64 = ctx.r[11].s64 + -5304;
	// 832B2B88: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B2B8C: 3BEB0C28  addi r31, r11, 0xc28
	ctx.r[31].s64 = ctx.r[11].s64 + 3112;
	// 832B2B90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B2B94: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832B2B98; continue 'dispatch;
            }
            0x832B2B98 => {
    //   block [0x832B2B98..0x832B2BA8)
	// 832B2B98: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B2B9C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B2BA0: 4AF13BC9  bl 0x821c6768
	ctx.lr = 0x832B2BA4;
	sub_821C6768(ctx, base);
	// 832B2BA4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	pc = 0x832B2BA8; continue 'dispatch;
            }
            0x832B2BA8 => {
    //   block [0x832B2BA8..0x832B2BD4)
	// 832B2BA8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B2BAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2BB0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B2BB4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B2BB8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B2BBC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2BC0: 4082FFE8  bne 0x832b2ba8
	if !ctx.cr[0].eq {
	pc = 0x832B2BA8; continue 'dispatch;
	}
	// 832B2BC4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832B2BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2BCC: 4AF13B9D  bl 0x821c6768
	ctx.lr = 0x832B2BD0;
	sub_821C6768(ctx, base);
	// 832B2BD0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	pc = 0x832B2BD4; continue 'dispatch;
            }
            0x832B2BD4 => {
    //   block [0x832B2BD4..0x832B2C00)
	// 832B2BD4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832B2BD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2BDC: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832B2BE0: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832B2BE4: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B2BE8: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2BEC: 4082FFE8  bne 0x832b2bd4
	if !ctx.cr[0].eq {
	pc = 0x832B2BD4; continue 'dispatch;
	}
	// 832B2BF0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B2BF4: 387FFFF8  addi r3, r31, -8
	ctx.r[3].s64 = ctx.r[31].s64 + -8;
	// 832B2BF8: 4AF13B71  bl 0x821c6768
	ctx.lr = 0x832B2BFC;
	sub_821C6768(ctx, base);
	// 832B2BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x832B2C00; continue 'dispatch;
            }
            0x832B2C00 => {
    //   block [0x832B2C00..0x832B2C30)
	// 832B2C00: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 832B2C04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2C08: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 832B2C0C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832B2C10: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B2C14: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B2C18: 4082FFE8  bne 0x832b2c00
	if !ctx.cr[0].eq {
	pc = 0x832B2C00; continue 'dispatch;
	}
	// 832B2C1C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832B2C20: 939FFFF8  stw r28, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 832B2C24: 4080FF74  bge 0x832b2b98
	if !ctx.cr[0].lt {
	pc = 0x832B2B98; continue 'dispatch;
	}
	// 832B2C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B2C2C: 4B9F682C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C30 size=12
    let mut pc: u32 = 0x832B2C30;
    'dispatch: loop {
        match pc {
            0x832B2C30 => {
    //   block [0x832B2C30..0x832B2C3C)
	// 832B2C30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2C34: 386BF768  addi r3, r11, -0x898
	ctx.r[3].s64 = ctx.r[11].s64 + -2200;
	// 832B2C38: 4AF621A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C40 size=12
    let mut pc: u32 = 0x832B2C40;
    'dispatch: loop {
        match pc {
            0x832B2C40 => {
    //   block [0x832B2C40..0x832B2C4C)
	// 832B2C40: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2C44: 386BF76C  addi r3, r11, -0x894
	ctx.r[3].s64 = ctx.r[11].s64 + -2196;
	// 832B2C48: 4AF62190  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C50 size=4
    let mut pc: u32 = 0x832B2C50;
    'dispatch: loop {
        match pc {
            0x832B2C50 => {
    //   block [0x832B2C50..0x832B2C54)
	// 832B2C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C58 size=4
    let mut pc: u32 = 0x832B2C58;
    'dispatch: loop {
        match pc {
            0x832B2C58 => {
    //   block [0x832B2C58..0x832B2C5C)
	// 832B2C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C60 size=4
    let mut pc: u32 = 0x832B2C60;
    'dispatch: loop {
        match pc {
            0x832B2C60 => {
    //   block [0x832B2C60..0x832B2C64)
	// 832B2C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C68 size=4
    let mut pc: u32 = 0x832B2C68;
    'dispatch: loop {
        match pc {
            0x832B2C68 => {
    //   block [0x832B2C68..0x832B2C6C)
	// 832B2C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C70 size=12
    let mut pc: u32 = 0x832B2C70;
    'dispatch: loop {
        match pc {
            0x832B2C70 => {
    //   block [0x832B2C70..0x832B2C7C)
	// 832B2C70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2C74: 386BF780  addi r3, r11, -0x880
	ctx.r[3].s64 = ctx.r[11].s64 + -2176;
	// 832B2C78: 4AF62160  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C80 size=12
    let mut pc: u32 = 0x832B2C80;
    'dispatch: loop {
        match pc {
            0x832B2C80 => {
    //   block [0x832B2C80..0x832B2C8C)
	// 832B2C80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2C84: 386BF784  addi r3, r11, -0x87c
	ctx.r[3].s64 = ctx.r[11].s64 + -2172;
	// 832B2C88: 4AF62150  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2C90 size=12
    let mut pc: u32 = 0x832B2C90;
    'dispatch: loop {
        match pc {
            0x832B2C90 => {
    //   block [0x832B2C90..0x832B2C9C)
	// 832B2C90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2C94: 386BF788  addi r3, r11, -0x878
	ctx.r[3].s64 = ctx.r[11].s64 + -2168;
	// 832B2C98: 4AF62140  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2CA0 size=12
    let mut pc: u32 = 0x832B2CA0;
    'dispatch: loop {
        match pc {
            0x832B2CA0 => {
    //   block [0x832B2CA0..0x832B2CAC)
	// 832B2CA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2CA4: 386BF78C  addi r3, r11, -0x874
	ctx.r[3].s64 = ctx.r[11].s64 + -2164;
	// 832B2CA8: 4AF62130  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2CB0 size=12
    let mut pc: u32 = 0x832B2CB0;
    'dispatch: loop {
        match pc {
            0x832B2CB0 => {
    //   block [0x832B2CB0..0x832B2CBC)
	// 832B2CB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2CB4: 386BF790  addi r3, r11, -0x870
	ctx.r[3].s64 = ctx.r[11].s64 + -2160;
	// 832B2CB8: 4AF62120  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2CC0 size=12
    let mut pc: u32 = 0x832B2CC0;
    'dispatch: loop {
        match pc {
            0x832B2CC0 => {
    //   block [0x832B2CC0..0x832B2CCC)
	// 832B2CC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2CC4: 386BF794  addi r3, r11, -0x86c
	ctx.r[3].s64 = ctx.r[11].s64 + -2156;
	// 832B2CC8: 4AF62110  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B2CD0 size=12
    let mut pc: u32 = 0x832B2CD0;
    'dispatch: loop {
        match pc {
            0x832B2CD0 => {
    //   block [0x832B2CD0..0x832B2CDC)
	// 832B2CD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2CD4: 386BF798  addi r3, r11, -0x868
	ctx.r[3].s64 = ctx.r[11].s64 + -2152;
	// 832B2CD8: 4B79C9D0  b 0x82a4f6a8
	sub_82A4F6A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2CE0 size=84
    let mut pc: u32 = 0x832B2CE0;
    'dispatch: loop {
        match pc {
            0x832B2CE0 => {
    //   block [0x832B2CE0..0x832B2D08)
	// 832B2CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2CF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2CF4: 3BEBF7A8  addi r31, r11, -0x858
	ctx.r[31].s64 = ctx.r[11].s64 + -2136;
	// 832B2CF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B2CFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B2D00: 419A0008  beq cr6, 0x832b2d08
	if ctx.cr[6].eq {
	pc = 0x832B2D08; continue 'dispatch;
	}
	// 832B2D04: 4AF69035  bl 0x8221bd38
	ctx.lr = 0x832B2D08;
	sub_8221BD38(ctx, base);
	pc = 0x832B2D08; continue 'dispatch;
            }
            0x832B2D08 => {
    //   block [0x832B2D08..0x832B2D34)
	// 832B2D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2D0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B2D10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B2D14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B2D18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B2D1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B2D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2D48 size=76
    let mut pc: u32 = 0x832B2D48;
    'dispatch: loop {
        match pc {
            0x832B2D48 => {
    //   block [0x832B2D48..0x832B2D94)
	// 832B2D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2D58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B2D5C: 3BEBF7C8  addi r31, r11, -0x838
	ctx.r[31].s64 = ctx.r[11].s64 + -2104;
	// 832B2D60: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832B2D64: 4AF62075  bl 0x82214dd8
	ctx.lr = 0x832B2D68;
	sub_82214DD8(ctx, base);
	// 832B2D68: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832B2D6C: 4AF6206D  bl 0x82214dd8
	ctx.lr = 0x832B2D70;
	sub_82214DD8(ctx, base);
	// 832B2D70: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B2D74: 4AF62065  bl 0x82214dd8
	ctx.lr = 0x832B2D78;
	sub_82214DD8(ctx, base);
	// 832B2D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2D7C: 4AF6205D  bl 0x82214dd8
	ctx.lr = 0x832B2D80;
	sub_82214DD8(ctx, base);
	// 832B2D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B2D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2DC0 size=84
    let mut pc: u32 = 0x832B2DC0;
    'dispatch: loop {
        match pc {
            0x832B2DC0 => {
    //   block [0x832B2DC0..0x832B2DF4)
	// 832B2DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2DD4: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2DD8: 83FEF7DC  lwz r31, -0x824(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2084 as u32) ) } as u64;
	// 832B2DDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2DE0: 419A0014  beq cr6, 0x832b2df4
	if ctx.cr[6].eq {
	pc = 0x832B2DF4; continue 'dispatch;
	}
	// 832B2DE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2DE8: 4AFC2521  bl 0x82275308
	ctx.lr = 0x832B2DEC;
	sub_82275308(ctx, base);
	// 832B2DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2DF0: 4AF68F49  bl 0x8221bd38
	ctx.lr = 0x832B2DF4;
	sub_8221BD38(ctx, base);
	pc = 0x832B2DF4; continue 'dispatch;
            }
            0x832B2DF4 => {
    //   block [0x832B2DF4..0x832B2E14)
	// 832B2DF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2DF8: 917EF7DC  stw r11, -0x824(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2084 as u32), ctx.r[11].u32 ) };
	// 832B2DFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2E08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2E0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2E18 size=84
    let mut pc: u32 = 0x832B2E18;
    'dispatch: loop {
        match pc {
            0x832B2E18 => {
    //   block [0x832B2E18..0x832B2E4C)
	// 832B2E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2E20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2E24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2E2C: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2E30: 83FEF7E0  lwz r31, -0x820(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2080 as u32) ) } as u64;
	// 832B2E34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2E38: 419A0014  beq cr6, 0x832b2e4c
	if ctx.cr[6].eq {
	pc = 0x832B2E4C; continue 'dispatch;
	}
	// 832B2E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2E40: 4AFEAC59  bl 0x8229da98
	ctx.lr = 0x832B2E44;
	sub_8229DA98(ctx, base);
	// 832B2E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2E48: 4AF68EF1  bl 0x8221bd38
	ctx.lr = 0x832B2E4C;
	sub_8221BD38(ctx, base);
	pc = 0x832B2E4C; continue 'dispatch;
            }
            0x832B2E4C => {
    //   block [0x832B2E4C..0x832B2E6C)
	// 832B2E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2E50: 917EF7E0  stw r11, -0x820(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2080 as u32), ctx.r[11].u32 ) };
	// 832B2E54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2E70 size=84
    let mut pc: u32 = 0x832B2E70;
    'dispatch: loop {
        match pc {
            0x832B2E70 => {
    //   block [0x832B2E70..0x832B2EA4)
	// 832B2E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2E78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2E7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2E84: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2E88: 83FEF7E4  lwz r31, -0x81c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2076 as u32) ) } as u64;
	// 832B2E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2E90: 419A0014  beq cr6, 0x832b2ea4
	if ctx.cr[6].eq {
	pc = 0x832B2EA4; continue 'dispatch;
	}
	// 832B2E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2E98: 4AFEAC01  bl 0x8229da98
	ctx.lr = 0x832B2E9C;
	sub_8229DA98(ctx, base);
	// 832B2E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2EA0: 4AF68E99  bl 0x8221bd38
	ctx.lr = 0x832B2EA4;
	sub_8221BD38(ctx, base);
	pc = 0x832B2EA4; continue 'dispatch;
            }
            0x832B2EA4 => {
    //   block [0x832B2EA4..0x832B2EC4)
	// 832B2EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2EA8: 917EF7E4  stw r11, -0x81c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2076 as u32), ctx.r[11].u32 ) };
	// 832B2EAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2EB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2EC8 size=84
    let mut pc: u32 = 0x832B2EC8;
    'dispatch: loop {
        match pc {
            0x832B2EC8 => {
    //   block [0x832B2EC8..0x832B2EFC)
	// 832B2EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2ED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2ED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2EDC: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2EE0: 83FEF7E8  lwz r31, -0x818(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2072 as u32) ) } as u64;
	// 832B2EE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2EE8: 419A0014  beq cr6, 0x832b2efc
	if ctx.cr[6].eq {
	pc = 0x832B2EFC; continue 'dispatch;
	}
	// 832B2EEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2EF0: 4AFEABA9  bl 0x8229da98
	ctx.lr = 0x832B2EF4;
	sub_8229DA98(ctx, base);
	// 832B2EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2EF8: 4AF68E41  bl 0x8221bd38
	ctx.lr = 0x832B2EFC;
	sub_8221BD38(ctx, base);
	pc = 0x832B2EFC; continue 'dispatch;
            }
            0x832B2EFC => {
    //   block [0x832B2EFC..0x832B2F1C)
	// 832B2EFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2F00: 917EF7E8  stw r11, -0x818(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2072 as u32), ctx.r[11].u32 ) };
	// 832B2F04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2F10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2F20 size=84
    let mut pc: u32 = 0x832B2F20;
    'dispatch: loop {
        match pc {
            0x832B2F20 => {
    //   block [0x832B2F20..0x832B2F54)
	// 832B2F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2F34: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2F38: 83FEF7EC  lwz r31, -0x814(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2068 as u32) ) } as u64;
	// 832B2F3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2F40: 419A0014  beq cr6, 0x832b2f54
	if ctx.cr[6].eq {
	pc = 0x832B2F54; continue 'dispatch;
	}
	// 832B2F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2F48: 4AFEAB51  bl 0x8229da98
	ctx.lr = 0x832B2F4C;
	sub_8229DA98(ctx, base);
	// 832B2F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2F50: 4AF68DE9  bl 0x8221bd38
	ctx.lr = 0x832B2F54;
	sub_8221BD38(ctx, base);
	pc = 0x832B2F54; continue 'dispatch;
            }
            0x832B2F54 => {
    //   block [0x832B2F54..0x832B2F74)
	// 832B2F54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2F58: 917EF7EC  stw r11, -0x814(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2068 as u32), ctx.r[11].u32 ) };
	// 832B2F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2F68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2F78 size=84
    let mut pc: u32 = 0x832B2F78;
    'dispatch: loop {
        match pc {
            0x832B2F78 => {
    //   block [0x832B2F78..0x832B2FAC)
	// 832B2F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2F80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2F84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2F8C: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2F90: 83FEF7F0  lwz r31, -0x810(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2064 as u32) ) } as u64;
	// 832B2F94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2F98: 419A0014  beq cr6, 0x832b2fac
	if ctx.cr[6].eq {
	pc = 0x832B2FAC; continue 'dispatch;
	}
	// 832B2F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2FA0: 4AFEAAF9  bl 0x8229da98
	ctx.lr = 0x832B2FA4;
	sub_8229DA98(ctx, base);
	// 832B2FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2FA8: 4AF68D91  bl 0x8221bd38
	ctx.lr = 0x832B2FAC;
	sub_8221BD38(ctx, base);
	pc = 0x832B2FAC; continue 'dispatch;
            }
            0x832B2FAC => {
    //   block [0x832B2FAC..0x832B2FCC)
	// 832B2FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B2FB0: 917EF7F0  stw r11, -0x810(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2064 as u32), ctx.r[11].u32 ) };
	// 832B2FB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B2FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B2FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B2FC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B2FC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B2FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B2FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B2FD0 size=84
    let mut pc: u32 = 0x832B2FD0;
    'dispatch: loop {
        match pc {
            0x832B2FD0 => {
    //   block [0x832B2FD0..0x832B3004)
	// 832B2FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B2FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B2FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B2FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B2FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B2FE4: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B2FE8: 83FEF7F4  lwz r31, -0x80c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2060 as u32) ) } as u64;
	// 832B2FEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B2FF0: 419A0014  beq cr6, 0x832b3004
	if ctx.cr[6].eq {
	pc = 0x832B3004; continue 'dispatch;
	}
	// 832B2FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B2FF8: 4AFEAAA1  bl 0x8229da98
	ctx.lr = 0x832B2FFC;
	sub_8229DA98(ctx, base);
	// 832B2FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3000: 4AF68D39  bl 0x8221bd38
	ctx.lr = 0x832B3004;
	sub_8221BD38(ctx, base);
	pc = 0x832B3004; continue 'dispatch;
            }
            0x832B3004 => {
    //   block [0x832B3004..0x832B3024)
	// 832B3004: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3008: 917EF7F4  stw r11, -0x80c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-2060 as u32), ctx.r[11].u32 ) };
	// 832B300C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3018: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B301C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3028 size=116
    let mut pc: u32 = 0x832B3028;
    'dispatch: loop {
        match pc {
            0x832B3028 => {
    //   block [0x832B3028..0x832B304C)
	// 832B3028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B302C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3030: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3038: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B303C: 807FF7F8  lwz r3, -0x808(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2056 as u32) ) } as u64;
	// 832B3040: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3044: 419A0044  beq cr6, 0x832b3088
	if ctx.cr[6].eq {
	pc = 0x832B3088; continue 'dispatch;
	}
	// 832B3048: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B304C; continue 'dispatch;
            }
            0x832B304C => {
    //   block [0x832B304C..0x832B3080)
	// 832B304C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B3050: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B3054: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B3058: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B305C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B3060: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B3064: 4082FFE8  bne 0x832b304c
	if !ctx.cr[0].eq {
	pc = 0x832B304C; continue 'dispatch;
	}
	// 832B3068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B306C: 409A0014  bne cr6, 0x832b3080
	if !ctx.cr[6].eq {
	pc = 0x832B3080; continue 'dispatch;
	}
	// 832B3070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3074: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3078: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B307C: 4E800421  bctrl
	ctx.lr = 0x832B3080;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B3080 => {
    //   block [0x832B3080..0x832B3088)
	// 832B3080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3084: 917FF7F8  stw r11, -0x808(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-2056 as u32), ctx.r[11].u32 ) };
	pc = 0x832B3088; continue 'dispatch;
            }
            0x832B3088 => {
    //   block [0x832B3088..0x832B309C)
	// 832B3088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B308C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30A0 size=4
    let mut pc: u32 = 0x832B30A0;
    'dispatch: loop {
        match pc {
            0x832B30A0 => {
    //   block [0x832B30A0..0x832B30A4)
	// 832B30A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30A8 size=12
    let mut pc: u32 = 0x832B30A8;
    'dispatch: loop {
        match pc {
            0x832B30A8 => {
    //   block [0x832B30A8..0x832B30B4)
	// 832B30A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B30AC: 386BF818  addi r3, r11, -0x7e8
	ctx.r[3].s64 = ctx.r[11].s64 + -2024;
	// 832B30B0: 4B79D478  b 0x82a50528
	sub_82A50528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30B8 size=12
    let mut pc: u32 = 0x832B30B8;
    'dispatch: loop {
        match pc {
            0x832B30B8 => {
    //   block [0x832B30B8..0x832B30C4)
	// 832B30B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B30BC: 386BF820  addi r3, r11, -0x7e0
	ctx.r[3].s64 = ctx.r[11].s64 + -2016;
	// 832B30C0: 4AF04A58  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30C8 size=12
    let mut pc: u32 = 0x832B30C8;
    'dispatch: loop {
        match pc {
            0x832B30C8 => {
    //   block [0x832B30C8..0x832B30D4)
	// 832B30C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B30CC: 386BF828  addi r3, r11, -0x7d8
	ctx.r[3].s64 = ctx.r[11].s64 + -2008;
	// 832B30D0: 4AF04A48  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30D8 size=12
    let mut pc: u32 = 0x832B30D8;
    'dispatch: loop {
        match pc {
            0x832B30D8 => {
    //   block [0x832B30D8..0x832B30E4)
	// 832B30D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B30DC: 386BF830  addi r3, r11, -0x7d0
	ctx.r[3].s64 = ctx.r[11].s64 + -2000;
	// 832B30E0: 4AF04A38  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30E8 size=12
    let mut pc: u32 = 0x832B30E8;
    'dispatch: loop {
        match pc {
            0x832B30E8 => {
    //   block [0x832B30E8..0x832B30F4)
	// 832B30E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B30EC: 386BF838  addi r3, r11, -0x7c8
	ctx.r[3].s64 = ctx.r[11].s64 + -1992;
	// 832B30F0: 4AF04A28  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B30F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B30F8 size=12
    let mut pc: u32 = 0x832B30F8;
    'dispatch: loop {
        match pc {
            0x832B30F8 => {
    //   block [0x832B30F8..0x832B3104)
	// 832B30F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B30FC: 386BF840  addi r3, r11, -0x7c0
	ctx.r[3].s64 = ctx.r[11].s64 + -1984;
	// 832B3100: 4AF04A18  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3108 size=84
    let mut pc: u32 = 0x832B3108;
    'dispatch: loop {
        match pc {
            0x832B3108 => {
    //   block [0x832B3108..0x832B313C)
	// 832B3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B3114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B311C: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B3120: 83FEF848  lwz r31, -0x7b8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1976 as u32) ) } as u64;
	// 832B3124: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B3128: 419A0014  beq cr6, 0x832b313c
	if ctx.cr[6].eq {
	pc = 0x832B313C; continue 'dispatch;
	}
	// 832B312C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3130: 4B7C8B09  bl 0x82a7bc38
	ctx.lr = 0x832B3134;
	sub_82A7BC38(ctx, base);
	// 832B3134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3138: 4AF68C01  bl 0x8221bd38
	ctx.lr = 0x832B313C;
	sub_8221BD38(ctx, base);
	pc = 0x832B313C; continue 'dispatch;
            }
            0x832B313C => {
    //   block [0x832B313C..0x832B315C)
	// 832B313C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3140: 917EF848  stw r11, -0x7b8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-1976 as u32), ctx.r[11].u32 ) };
	// 832B3144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B314C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3160 size=92
    let mut pc: u32 = 0x832B3160;
    'dispatch: loop {
        match pc {
            0x832B3160 => {
    //   block [0x832B3160..0x832B31BC)
	// 832B3160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B316C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3174: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B3178: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B317C: 3BDFA978  addi r30, r31, -0x5688
	ctx.r[30].s64 = ctx.r[31].s64 + -22152;
	// 832B3180: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B3184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3188: 917FA978  stw r11, -0x5688(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22152 as u32), ctx.r[11].u32 ) };
	// 832B318C: 4AF48E7D  bl 0x821fc008
	ctx.lr = 0x832B3190;
	sub_821FC008(ctx, base);
	// 832B3190: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3198: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B319C: 917FA978  stw r11, -0x5688(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22152 as u32), ctx.r[11].u32 ) };
	// 832B31A0: 4AF48E69  bl 0x821fc008
	ctx.lr = 0x832B31A4;
	sub_821FC008(ctx, base);
	// 832B31A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B31A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B31AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B31B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B31B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B31B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B31C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B31C0 size=92
    let mut pc: u32 = 0x832B31C0;
    'dispatch: loop {
        match pc {
            0x832B31C0 => {
    //   block [0x832B31C0..0x832B321C)
	// 832B31C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B31C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B31C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B31CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B31D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B31D4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B31D8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B31DC: 3BDFA980  addi r30, r31, -0x5680
	ctx.r[30].s64 = ctx.r[31].s64 + -22144;
	// 832B31E0: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B31E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B31E8: 917FA980  stw r11, -0x5680(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22144 as u32), ctx.r[11].u32 ) };
	// 832B31EC: 4AF48E1D  bl 0x821fc008
	ctx.lr = 0x832B31F0;
	sub_821FC008(ctx, base);
	// 832B31F0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B31F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B31F8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B31FC: 917FA980  stw r11, -0x5680(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22144 as u32), ctx.r[11].u32 ) };
	// 832B3200: 4AF48E09  bl 0x821fc008
	ctx.lr = 0x832B3204;
	sub_821FC008(ctx, base);
	// 832B3204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B320C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3210: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3220 size=92
    let mut pc: u32 = 0x832B3220;
    'dispatch: loop {
        match pc {
            0x832B3220 => {
    //   block [0x832B3220..0x832B327C)
	// 832B3220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B322C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3234: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B3238: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B323C: 3BDFA988  addi r30, r31, -0x5678
	ctx.r[30].s64 = ctx.r[31].s64 + -22136;
	// 832B3240: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B3244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3248: 917FA988  stw r11, -0x5678(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22136 as u32), ctx.r[11].u32 ) };
	// 832B324C: 4AF48DBD  bl 0x821fc008
	ctx.lr = 0x832B3250;
	sub_821FC008(ctx, base);
	// 832B3250: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3258: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B325C: 917FA988  stw r11, -0x5678(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-22136 as u32), ctx.r[11].u32 ) };
	// 832B3260: 4AF48DA9  bl 0x821fc008
	ctx.lr = 0x832B3264;
	sub_821FC008(ctx, base);
	// 832B3264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B326C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3270: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3280 size=88
    let mut pc: u32 = 0x832B3280;
    'dispatch: loop {
        match pc {
            0x832B3280 => {
    //   block [0x832B3280..0x832B32C4)
	// 832B3280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B328C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3290: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3294: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B3298: 3BEBA990  addi r31, r11, -0x5670
	ctx.r[31].s64 = ctx.r[11].s64 + -22128;
	// 832B329C: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B32A0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B32A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B32A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B32AC: 419A0018  beq cr6, 0x832b32c4
	if ctx.cr[6].eq {
	pc = 0x832B32C4; continue 'dispatch;
	}
	// 832B32B0: 4AF49071  bl 0x821fc320
	ctx.lr = 0x832B32B4;
	sub_821FC320(ctx, base);
	// 832B32B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B32B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B32BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B32C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x832B32C4; continue 'dispatch;
            }
            0x832B32C4 => {
    //   block [0x832B32C4..0x832B32D8)
	// 832B32C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B32C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B32CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B32D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B32D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B32D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B32D8 size=88
    let mut pc: u32 = 0x832B32D8;
    'dispatch: loop {
        match pc {
            0x832B32D8 => {
    //   block [0x832B32D8..0x832B331C)
	// 832B32D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B32DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B32E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B32E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B32E8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B32EC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B32F0: 3BEBA99C  addi r31, r11, -0x5664
	ctx.r[31].s64 = ctx.r[11].s64 + -22116;
	// 832B32F4: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B32F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B32FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B3300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3304: 419A0018  beq cr6, 0x832b331c
	if ctx.cr[6].eq {
	pc = 0x832B331C; continue 'dispatch;
	}
	// 832B3308: 4AF49019  bl 0x821fc320
	ctx.lr = 0x832B330C;
	sub_821FC320(ctx, base);
	// 832B330C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3314: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3318: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x832B331C; continue 'dispatch;
            }
            0x832B331C => {
    //   block [0x832B331C..0x832B3330)
	// 832B331C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B332C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3330 size=12
    let mut pc: u32 = 0x832B3330;
    'dispatch: loop {
        match pc {
            0x832B3330 => {
    //   block [0x832B3330..0x832B333C)
	// 832B3330: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3334: 386BA9A8  addi r3, r11, -0x5658
	ctx.r[3].s64 = ctx.r[11].s64 + -22104;
	// 832B3338: 4AF48C58  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3340 size=12
    let mut pc: u32 = 0x832B3340;
    'dispatch: loop {
        match pc {
            0x832B3340 => {
    //   block [0x832B3340..0x832B334C)
	// 832B3340: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3344: 386BA9CC  addi r3, r11, -0x5634
	ctx.r[3].s64 = ctx.r[11].s64 + -22068;
	// 832B3348: 4AF48C48  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3350 size=12
    let mut pc: u32 = 0x832B3350;
    'dispatch: loop {
        match pc {
            0x832B3350 => {
    //   block [0x832B3350..0x832B335C)
	// 832B3350: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3354: 386BA9F0  addi r3, r11, -0x5610
	ctx.r[3].s64 = ctx.r[11].s64 + -22032;
	// 832B3358: 4AF48C38  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3360 size=12
    let mut pc: u32 = 0x832B3360;
    'dispatch: loop {
        match pc {
            0x832B3360 => {
    //   block [0x832B3360..0x832B336C)
	// 832B3360: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3364: 386BAA14  addi r3, r11, -0x55ec
	ctx.r[3].s64 = ctx.r[11].s64 + -21996;
	// 832B3368: 4AF48C28  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3370 size=12
    let mut pc: u32 = 0x832B3370;
    'dispatch: loop {
        match pc {
            0x832B3370 => {
    //   block [0x832B3370..0x832B337C)
	// 832B3370: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3374: 386BAA38  addi r3, r11, -0x55c8
	ctx.r[3].s64 = ctx.r[11].s64 + -21960;
	// 832B3378: 4AF48C18  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3380 size=12
    let mut pc: u32 = 0x832B3380;
    'dispatch: loop {
        match pc {
            0x832B3380 => {
    //   block [0x832B3380..0x832B338C)
	// 832B3380: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B3384: 386BAA5C  addi r3, r11, -0x55a4
	ctx.r[3].s64 = ctx.r[11].s64 + -21924;
	// 832B3388: 4AF48C08  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3390 size=12
    let mut pc: u32 = 0x832B3390;
    'dispatch: loop {
        match pc {
            0x832B3390 => {
    //   block [0x832B3390..0x832B339C)
	// 832B3390: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3394: 386BF850  addi r3, r11, -0x7b0
	ctx.r[3].s64 = ctx.r[11].s64 + -1968;
	// 832B3398: 4B799890  b 0x82a4cc28
	sub_82A4CC28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B33A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B33A0 size=12
    let mut pc: u32 = 0x832B33A0;
    'dispatch: loop {
        match pc {
            0x832B33A0 => {
    //   block [0x832B33A0..0x832B33AC)
	// 832B33A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B33A4: 386BF8E0  addi r3, r11, -0x720
	ctx.r[3].s64 = ctx.r[11].s64 + -1824;
	// 832B33A8: 4B799880  b 0x82a4cc28
	sub_82A4CC28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B33B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B33B0 size=80
    let mut pc: u32 = 0x832B33B0;
    'dispatch: loop {
        match pc {
            0x832B33B0 => {
    //   block [0x832B33B0..0x832B33D0)
	// 832B33B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B33B4: 4B9F6059  bl 0x82ca940c
	ctx.lr = 0x832B33B8;
	sub_82CA93D0(ctx, base);
	// 832B33B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B33BC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B33C0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B33C4: 396BF970  addi r11, r11, -0x690
	ctx.r[11].s64 = ctx.r[11].s64 + -1680;
	// 832B33C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 832B33CC: 3BEB0044  addi r31, r11, 0x44
	ctx.r[31].s64 = ctx.r[11].s64 + 68;
	pc = 0x832B33D0; continue 'dispatch;
            }
            0x832B33D0 => {
    //   block [0x832B33D0..0x832B33E4)
	// 832B33D0: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 832B33D4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B33D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B33DC: 419A0008  beq cr6, 0x832b33e4
	if ctx.cr[6].eq {
	pc = 0x832B33E4; continue 'dispatch;
	}
	// 832B33E0: 4AF68959  bl 0x8221bd38
	ctx.lr = 0x832B33E4;
	sub_8221BD38(ctx, base);
	pc = 0x832B33E4; continue 'dispatch;
            }
            0x832B33E4 => {
    //   block [0x832B33E4..0x832B3400)
	// 832B33E4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B33E8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B33EC: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832B33F0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 832B33F4: 4080FFDC  bge 0x832b33d0
	if !ctx.cr[0].lt {
	pc = 0x832B33D0; continue 'dispatch;
	}
	// 832B33F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B33FC: 4B9F6060  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3400 size=80
    let mut pc: u32 = 0x832B3400;
    'dispatch: loop {
        match pc {
            0x832B3400 => {
    //   block [0x832B3400..0x832B3434)
	// 832B3400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B340C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3410: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B3414: 807FF84C  lwz r3, -0x7b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-1972 as u32) ) } as u64;
	// 832B3418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B341C: 419A0018  beq cr6, 0x832b3434
	if ctx.cr[6].eq {
	pc = 0x832B3434; continue 'dispatch;
	}
	// 832B3420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3424: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B3428: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B342C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B3430: 4E800421  bctrl
	ctx.lr = 0x832B3434;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B3434 => {
    //   block [0x832B3434..0x832B3450)
	// 832B3434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3438: 917FF84C  stw r11, -0x7b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-1972 as u32), ctx.r[11].u32 ) };
	// 832B343C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B344C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3450 size=88
    let mut pc: u32 = 0x832B3450;
    'dispatch: loop {
        match pc {
            0x832B3450 => {
    //   block [0x832B3450..0x832B3470)
	// 832B3450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3454: 4B9F5FB5  bl 0x82ca9408
	ctx.lr = 0x832B3458;
	sub_82CA93D0(ctx, base);
	// 832B3458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B345C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3460: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 832B3464: 396BF9B0  addi r11, r11, -0x650
	ctx.r[11].s64 = ctx.r[11].s64 + -1616;
	// 832B3468: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B346C: 3BCB0010  addi r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B3470; continue 'dispatch;
            }
            0x832B3470 => {
    //   block [0x832B3470..0x832B3494)
	// 832B3470: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 832B3474: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3478: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B347C: 419A0018  beq cr6, 0x832b3494
	if ctx.cr[6].eq {
	pc = 0x832B3494; continue 'dispatch;
	}
	// 832B3480: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3484: 4AF688B5  bl 0x8221bd38
	ctx.lr = 0x832B3488;
	sub_8221BD38(ctx, base);
	// 832B3488: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B348C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3490: 4AF688A9  bl 0x8221bd38
	ctx.lr = 0x832B3494;
	sub_8221BD38(ctx, base);
	pc = 0x832B3494; continue 'dispatch;
            }
            0x832B3494 => {
    //   block [0x832B3494..0x832B34A8)
	// 832B3494: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832B3498: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B349C: 4080FFD4  bge 0x832b3470
	if !ctx.cr[0].lt {
	pc = 0x832B3470; continue 'dispatch;
	}
	// 832B34A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B34A4: 4B9F5FB4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B34A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B34A8 size=124
    let mut pc: u32 = 0x832B34A8;
    'dispatch: loop {
        match pc {
            0x832B34A8 => {
    //   block [0x832B34A8..0x832B34DC)
	// 832B34A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B34AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B34B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B34B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B34B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B34BC: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B34C0: 83FEF9C0  lwz r31, -0x640(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1600 as u32) ) } as u64;
	// 832B34C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B34C8: 419A003C  beq cr6, 0x832b3504
	if ctx.cr[6].eq {
	pc = 0x832B3504; continue 'dispatch;
	}
	// 832B34CC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832B34D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B34D4: 419A0008  beq cr6, 0x832b34dc
	if ctx.cr[6].eq {
	pc = 0x832B34DC; continue 'dispatch;
	}
	// 832B34D8: 4B88A2F1  bl 0x82b3d7c8
	ctx.lr = 0x832B34DC;
	sub_82B3D7C8(ctx, base);
	pc = 0x832B34DC; continue 'dispatch;
            }
            0x832B34DC => {
    //   block [0x832B34DC..0x832B3504)
	// 832B34DC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832B34E0: 4B3871A9  bl 0x8263a688
	ctx.lr = 0x832B34E4;
	sub_8263A688(ctx, base);
	// 832B34E4: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832B34E8: 4B3871A1  bl 0x8263a688
	ctx.lr = 0x832B34EC;
	sub_8263A688(ctx, base);
	// 832B34EC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832B34F0: 4B7EE169  bl 0x82aa1658
	ctx.lr = 0x832B34F4;
	sub_82AA1658(ctx, base);
	// 832B34F4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B34F8: 4B7EE161  bl 0x82aa1658
	ctx.lr = 0x832B34FC;
	sub_82AA1658(ctx, base);
	// 832B34FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3500: 4AF68839  bl 0x8221bd38
	ctx.lr = 0x832B3504;
	sub_8221BD38(ctx, base);
	pc = 0x832B3504; continue 'dispatch;
            }
            0x832B3504 => {
    //   block [0x832B3504..0x832B3524)
	// 832B3504: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3508: 917EF9C0  stw r11, -0x640(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-1600 as u32), ctx.r[11].u32 ) };
	// 832B350C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3518: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B351C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


