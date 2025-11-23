pub fn sub_832AFE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE60 size=4
    let mut pc: u32 = 0x832AFE60;
    'dispatch: loop {
        match pc {
            0x832AFE60 => {
    //   block [0x832AFE60..0x832AFE64)
	// 832AFE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE68 size=4
    let mut pc: u32 = 0x832AFE68;
    'dispatch: loop {
        match pc {
            0x832AFE68 => {
    //   block [0x832AFE68..0x832AFE6C)
	// 832AFE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE70 size=4
    let mut pc: u32 = 0x832AFE70;
    'dispatch: loop {
        match pc {
            0x832AFE70 => {
    //   block [0x832AFE70..0x832AFE74)
	// 832AFE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE78 size=4
    let mut pc: u32 = 0x832AFE78;
    'dispatch: loop {
        match pc {
            0x832AFE78 => {
    //   block [0x832AFE78..0x832AFE7C)
	// 832AFE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE80 size=4
    let mut pc: u32 = 0x832AFE80;
    'dispatch: loop {
        match pc {
            0x832AFE80 => {
    //   block [0x832AFE80..0x832AFE84)
	// 832AFE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE88 size=4
    let mut pc: u32 = 0x832AFE88;
    'dispatch: loop {
        match pc {
            0x832AFE88 => {
    //   block [0x832AFE88..0x832AFE8C)
	// 832AFE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE90 size=4
    let mut pc: u32 = 0x832AFE90;
    'dispatch: loop {
        match pc {
            0x832AFE90 => {
    //   block [0x832AFE90..0x832AFE94)
	// 832AFE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFE98 size=12
    let mut pc: u32 = 0x832AFE98;
    'dispatch: loop {
        match pc {
            0x832AFE98 => {
    //   block [0x832AFE98..0x832AFEA4)
	// 832AFE98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFE9C: 386BD7A0  addi r3, r11, -0x2860
	ctx.r[3].s64 = ctx.r[11].s64 + -10336;
	// 832AFEA0: 4AF64F38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFEA8 size=12
    let mut pc: u32 = 0x832AFEA8;
    'dispatch: loop {
        match pc {
            0x832AFEA8 => {
    //   block [0x832AFEA8..0x832AFEB4)
	// 832AFEA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFEAC: 386BD7A4  addi r3, r11, -0x285c
	ctx.r[3].s64 = ctx.r[11].s64 + -10332;
	// 832AFEB0: 4AF64F28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFEB8 size=12
    let mut pc: u32 = 0x832AFEB8;
    'dispatch: loop {
        match pc {
            0x832AFEB8 => {
    //   block [0x832AFEB8..0x832AFEC4)
	// 832AFEB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFEBC: 386BD7A8  addi r3, r11, -0x2858
	ctx.r[3].s64 = ctx.r[11].s64 + -10328;
	// 832AFEC0: 4AF64F18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFEC8 size=12
    let mut pc: u32 = 0x832AFEC8;
    'dispatch: loop {
        match pc {
            0x832AFEC8 => {
    //   block [0x832AFEC8..0x832AFED4)
	// 832AFEC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFECC: 386BD7AC  addi r3, r11, -0x2854
	ctx.r[3].s64 = ctx.r[11].s64 + -10324;
	// 832AFED0: 4AF64F08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFED8 size=12
    let mut pc: u32 = 0x832AFED8;
    'dispatch: loop {
        match pc {
            0x832AFED8 => {
    //   block [0x832AFED8..0x832AFEE4)
	// 832AFED8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFEDC: 386BD7B0  addi r3, r11, -0x2850
	ctx.r[3].s64 = ctx.r[11].s64 + -10320;
	// 832AFEE0: 4AF64EF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFEE8 size=12
    let mut pc: u32 = 0x832AFEE8;
    'dispatch: loop {
        match pc {
            0x832AFEE8 => {
    //   block [0x832AFEE8..0x832AFEF4)
	// 832AFEE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFEEC: 386BD808  addi r3, r11, -0x27f8
	ctx.r[3].s64 = ctx.r[11].s64 + -10232;
	// 832AFEF0: 4AF64EE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFEF8 size=12
    let mut pc: u32 = 0x832AFEF8;
    'dispatch: loop {
        match pc {
            0x832AFEF8 => {
    //   block [0x832AFEF8..0x832AFF04)
	// 832AFEF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFEFC: 386BD80C  addi r3, r11, -0x27f4
	ctx.r[3].s64 = ctx.r[11].s64 + -10228;
	// 832AFF00: 4AF64ED8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF08 size=12
    let mut pc: u32 = 0x832AFF08;
    'dispatch: loop {
        match pc {
            0x832AFF08 => {
    //   block [0x832AFF08..0x832AFF14)
	// 832AFF08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF0C: 386BD810  addi r3, r11, -0x27f0
	ctx.r[3].s64 = ctx.r[11].s64 + -10224;
	// 832AFF10: 4AF64EC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF18 size=12
    let mut pc: u32 = 0x832AFF18;
    'dispatch: loop {
        match pc {
            0x832AFF18 => {
    //   block [0x832AFF18..0x832AFF24)
	// 832AFF18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF1C: 386BD814  addi r3, r11, -0x27ec
	ctx.r[3].s64 = ctx.r[11].s64 + -10220;
	// 832AFF20: 4AF64EB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF28 size=12
    let mut pc: u32 = 0x832AFF28;
    'dispatch: loop {
        match pc {
            0x832AFF28 => {
    //   block [0x832AFF28..0x832AFF34)
	// 832AFF28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF2C: 386BD818  addi r3, r11, -0x27e8
	ctx.r[3].s64 = ctx.r[11].s64 + -10216;
	// 832AFF30: 4AF64EA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF38 size=12
    let mut pc: u32 = 0x832AFF38;
    'dispatch: loop {
        match pc {
            0x832AFF38 => {
    //   block [0x832AFF38..0x832AFF44)
	// 832AFF38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF3C: 386BD81C  addi r3, r11, -0x27e4
	ctx.r[3].s64 = ctx.r[11].s64 + -10212;
	// 832AFF40: 4AF64E98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF48 size=12
    let mut pc: u32 = 0x832AFF48;
    'dispatch: loop {
        match pc {
            0x832AFF48 => {
    //   block [0x832AFF48..0x832AFF54)
	// 832AFF48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF4C: 386BD820  addi r3, r11, -0x27e0
	ctx.r[3].s64 = ctx.r[11].s64 + -10208;
	// 832AFF50: 4AF64E88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF58 size=12
    let mut pc: u32 = 0x832AFF58;
    'dispatch: loop {
        match pc {
            0x832AFF58 => {
    //   block [0x832AFF58..0x832AFF64)
	// 832AFF58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF5C: 386BD824  addi r3, r11, -0x27dc
	ctx.r[3].s64 = ctx.r[11].s64 + -10204;
	// 832AFF60: 4AF64E78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF68 size=12
    let mut pc: u32 = 0x832AFF68;
    'dispatch: loop {
        match pc {
            0x832AFF68 => {
    //   block [0x832AFF68..0x832AFF74)
	// 832AFF68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF6C: 386BD828  addi r3, r11, -0x27d8
	ctx.r[3].s64 = ctx.r[11].s64 + -10200;
	// 832AFF70: 4AF64E68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF78 size=12
    let mut pc: u32 = 0x832AFF78;
    'dispatch: loop {
        match pc {
            0x832AFF78 => {
    //   block [0x832AFF78..0x832AFF84)
	// 832AFF78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF7C: 386BD82C  addi r3, r11, -0x27d4
	ctx.r[3].s64 = ctx.r[11].s64 + -10196;
	// 832AFF80: 4AF64E58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF88 size=12
    let mut pc: u32 = 0x832AFF88;
    'dispatch: loop {
        match pc {
            0x832AFF88 => {
    //   block [0x832AFF88..0x832AFF94)
	// 832AFF88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF8C: 386BD830  addi r3, r11, -0x27d0
	ctx.r[3].s64 = ctx.r[11].s64 + -10192;
	// 832AFF90: 4AF64E48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFF98 size=12
    let mut pc: u32 = 0x832AFF98;
    'dispatch: loop {
        match pc {
            0x832AFF98 => {
    //   block [0x832AFF98..0x832AFFA4)
	// 832AFF98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFF9C: 386BD834  addi r3, r11, -0x27cc
	ctx.r[3].s64 = ctx.r[11].s64 + -10188;
	// 832AFFA0: 4AF64E38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFFA8 size=12
    let mut pc: u32 = 0x832AFFA8;
    'dispatch: loop {
        match pc {
            0x832AFFA8 => {
    //   block [0x832AFFA8..0x832AFFB4)
	// 832AFFA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFFAC: 386BD88C  addi r3, r11, -0x2774
	ctx.r[3].s64 = ctx.r[11].s64 + -10100;
	// 832AFFB0: 4AF64E28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFFB8 size=12
    let mut pc: u32 = 0x832AFFB8;
    'dispatch: loop {
        match pc {
            0x832AFFB8 => {
    //   block [0x832AFFB8..0x832AFFC4)
	// 832AFFB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFFBC: 386BD890  addi r3, r11, -0x2770
	ctx.r[3].s64 = ctx.r[11].s64 + -10096;
	// 832AFFC0: 4AF64E18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFFC8 size=12
    let mut pc: u32 = 0x832AFFC8;
    'dispatch: loop {
        match pc {
            0x832AFFC8 => {
    //   block [0x832AFFC8..0x832AFFD4)
	// 832AFFC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFFCC: 386BD894  addi r3, r11, -0x276c
	ctx.r[3].s64 = ctx.r[11].s64 + -10092;
	// 832AFFD0: 4AF64E08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFFD8 size=12
    let mut pc: u32 = 0x832AFFD8;
    'dispatch: loop {
        match pc {
            0x832AFFD8 => {
    //   block [0x832AFFD8..0x832AFFE4)
	// 832AFFD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFFDC: 386BD898  addi r3, r11, -0x2768
	ctx.r[3].s64 = ctx.r[11].s64 + -10088;
	// 832AFFE0: 4AF64DF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFFE8 size=12
    let mut pc: u32 = 0x832AFFE8;
    'dispatch: loop {
        match pc {
            0x832AFFE8 => {
    //   block [0x832AFFE8..0x832AFFF4)
	// 832AFFE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFFEC: 386BD89C  addi r3, r11, -0x2764
	ctx.r[3].s64 = ctx.r[11].s64 + -10084;
	// 832AFFF0: 4AF64DE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832AFFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832AFFF8 size=12
    let mut pc: u32 = 0x832AFFF8;
    'dispatch: loop {
        match pc {
            0x832AFFF8 => {
    //   block [0x832AFFF8..0x832B0004)
	// 832AFFF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832AFFFC: 386BD8A0  addi r3, r11, -0x2760
	ctx.r[3].s64 = ctx.r[11].s64 + -10080;
	// 832B0000: 4AF64DD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0008 size=12
    let mut pc: u32 = 0x832B0008;
    'dispatch: loop {
        match pc {
            0x832B0008 => {
    //   block [0x832B0008..0x832B0014)
	// 832B0008: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B000C: 386BD8A4  addi r3, r11, -0x275c
	ctx.r[3].s64 = ctx.r[11].s64 + -10076;
	// 832B0010: 4AF64DC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0018 size=12
    let mut pc: u32 = 0x832B0018;
    'dispatch: loop {
        match pc {
            0x832B0018 => {
    //   block [0x832B0018..0x832B0024)
	// 832B0018: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B001C: 386BD8A8  addi r3, r11, -0x2758
	ctx.r[3].s64 = ctx.r[11].s64 + -10072;
	// 832B0020: 4AF64DB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0028 size=12
    let mut pc: u32 = 0x832B0028;
    'dispatch: loop {
        match pc {
            0x832B0028 => {
    //   block [0x832B0028..0x832B0034)
	// 832B0028: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B002C: 386BD8AC  addi r3, r11, -0x2754
	ctx.r[3].s64 = ctx.r[11].s64 + -10068;
	// 832B0030: 4AF64DA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0038 size=12
    let mut pc: u32 = 0x832B0038;
    'dispatch: loop {
        match pc {
            0x832B0038 => {
    //   block [0x832B0038..0x832B0044)
	// 832B0038: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B003C: 386BD8B0  addi r3, r11, -0x2750
	ctx.r[3].s64 = ctx.r[11].s64 + -10064;
	// 832B0040: 4AF64D98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0048 size=12
    let mut pc: u32 = 0x832B0048;
    'dispatch: loop {
        match pc {
            0x832B0048 => {
    //   block [0x832B0048..0x832B0054)
	// 832B0048: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B004C: 386BD8B4  addi r3, r11, -0x274c
	ctx.r[3].s64 = ctx.r[11].s64 + -10060;
	// 832B0050: 4AF64D88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0058 size=12
    let mut pc: u32 = 0x832B0058;
    'dispatch: loop {
        match pc {
            0x832B0058 => {
    //   block [0x832B0058..0x832B0064)
	// 832B0058: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B005C: 386BD8B8  addi r3, r11, -0x2748
	ctx.r[3].s64 = ctx.r[11].s64 + -10056;
	// 832B0060: 4AF64D78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0068 size=12
    let mut pc: u32 = 0x832B0068;
    'dispatch: loop {
        match pc {
            0x832B0068 => {
    //   block [0x832B0068..0x832B0074)
	// 832B0068: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B006C: 386BD8BC  addi r3, r11, -0x2744
	ctx.r[3].s64 = ctx.r[11].s64 + -10052;
	// 832B0070: 4AF64D68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0078 size=12
    let mut pc: u32 = 0x832B0078;
    'dispatch: loop {
        match pc {
            0x832B0078 => {
    //   block [0x832B0078..0x832B0084)
	// 832B0078: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B007C: 386BD8C0  addi r3, r11, -0x2740
	ctx.r[3].s64 = ctx.r[11].s64 + -10048;
	// 832B0080: 4AF64D58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0088 size=12
    let mut pc: u32 = 0x832B0088;
    'dispatch: loop {
        match pc {
            0x832B0088 => {
    //   block [0x832B0088..0x832B0094)
	// 832B0088: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B008C: 386BD8C4  addi r3, r11, -0x273c
	ctx.r[3].s64 = ctx.r[11].s64 + -10044;
	// 832B0090: 4AF64D48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0098 size=12
    let mut pc: u32 = 0x832B0098;
    'dispatch: loop {
        match pc {
            0x832B0098 => {
    //   block [0x832B0098..0x832B00A4)
	// 832B0098: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B009C: 386BD8C8  addi r3, r11, -0x2738
	ctx.r[3].s64 = ctx.r[11].s64 + -10040;
	// 832B00A0: 4AF64D38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B00A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B00A8 size=12
    let mut pc: u32 = 0x832B00A8;
    'dispatch: loop {
        match pc {
            0x832B00A8 => {
    //   block [0x832B00A8..0x832B00B4)
	// 832B00A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B00AC: 386BD8CC  addi r3, r11, -0x2734
	ctx.r[3].s64 = ctx.r[11].s64 + -10036;
	// 832B00B0: 4AF64D28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B00B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B00B8 size=12
    let mut pc: u32 = 0x832B00B8;
    'dispatch: loop {
        match pc {
            0x832B00B8 => {
    //   block [0x832B00B8..0x832B00C4)
	// 832B00B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B00BC: 386BD8D0  addi r3, r11, -0x2730
	ctx.r[3].s64 = ctx.r[11].s64 + -10032;
	// 832B00C0: 4AF64D18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B00C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B00C8 size=12
    let mut pc: u32 = 0x832B00C8;
    'dispatch: loop {
        match pc {
            0x832B00C8 => {
    //   block [0x832B00C8..0x832B00D4)
	// 832B00C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B00CC: 386BD8D4  addi r3, r11, -0x272c
	ctx.r[3].s64 = ctx.r[11].s64 + -10028;
	// 832B00D0: 4AF64D08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B00D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B00D8 size=12
    let mut pc: u32 = 0x832B00D8;
    'dispatch: loop {
        match pc {
            0x832B00D8 => {
    //   block [0x832B00D8..0x832B00E4)
	// 832B00D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B00DC: 386BD8D8  addi r3, r11, -0x2728
	ctx.r[3].s64 = ctx.r[11].s64 + -10024;
	// 832B00E0: 4AF64CF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B00E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B00E8 size=12
    let mut pc: u32 = 0x832B00E8;
    'dispatch: loop {
        match pc {
            0x832B00E8 => {
    //   block [0x832B00E8..0x832B00F4)
	// 832B00E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B00EC: 386BD8DC  addi r3, r11, -0x2724
	ctx.r[3].s64 = ctx.r[11].s64 + -10020;
	// 832B00F0: 4AF64CE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B00F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B00F8 size=12
    let mut pc: u32 = 0x832B00F8;
    'dispatch: loop {
        match pc {
            0x832B00F8 => {
    //   block [0x832B00F8..0x832B0104)
	// 832B00F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B00FC: 386BD8E0  addi r3, r11, -0x2720
	ctx.r[3].s64 = ctx.r[11].s64 + -10016;
	// 832B0100: 4AF64CD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0108 size=12
    let mut pc: u32 = 0x832B0108;
    'dispatch: loop {
        match pc {
            0x832B0108 => {
    //   block [0x832B0108..0x832B0114)
	// 832B0108: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B010C: 386BD8E4  addi r3, r11, -0x271c
	ctx.r[3].s64 = ctx.r[11].s64 + -10012;
	// 832B0110: 4AF64CC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0118 size=12
    let mut pc: u32 = 0x832B0118;
    'dispatch: loop {
        match pc {
            0x832B0118 => {
    //   block [0x832B0118..0x832B0124)
	// 832B0118: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B011C: 386BD8E8  addi r3, r11, -0x2718
	ctx.r[3].s64 = ctx.r[11].s64 + -10008;
	// 832B0120: 4AF64CB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0128 size=12
    let mut pc: u32 = 0x832B0128;
    'dispatch: loop {
        match pc {
            0x832B0128 => {
    //   block [0x832B0128..0x832B0134)
	// 832B0128: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B012C: 386BD8EC  addi r3, r11, -0x2714
	ctx.r[3].s64 = ctx.r[11].s64 + -10004;
	// 832B0130: 4AF64CA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0138 size=12
    let mut pc: u32 = 0x832B0138;
    'dispatch: loop {
        match pc {
            0x832B0138 => {
    //   block [0x832B0138..0x832B0144)
	// 832B0138: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B013C: 386BD8F0  addi r3, r11, -0x2710
	ctx.r[3].s64 = ctx.r[11].s64 + -10000;
	// 832B0140: 4AF64C98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0148 size=12
    let mut pc: u32 = 0x832B0148;
    'dispatch: loop {
        match pc {
            0x832B0148 => {
    //   block [0x832B0148..0x832B0154)
	// 832B0148: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B014C: 386BD8F4  addi r3, r11, -0x270c
	ctx.r[3].s64 = ctx.r[11].s64 + -9996;
	// 832B0150: 4AF64C88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0158 size=12
    let mut pc: u32 = 0x832B0158;
    'dispatch: loop {
        match pc {
            0x832B0158 => {
    //   block [0x832B0158..0x832B0164)
	// 832B0158: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B015C: 386BD8F8  addi r3, r11, -0x2708
	ctx.r[3].s64 = ctx.r[11].s64 + -9992;
	// 832B0160: 4AF64C78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0168 size=12
    let mut pc: u32 = 0x832B0168;
    'dispatch: loop {
        match pc {
            0x832B0168 => {
    //   block [0x832B0168..0x832B0174)
	// 832B0168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B016C: 386BD8FC  addi r3, r11, -0x2704
	ctx.r[3].s64 = ctx.r[11].s64 + -9988;
	// 832B0170: 4AF64C68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0178 size=12
    let mut pc: u32 = 0x832B0178;
    'dispatch: loop {
        match pc {
            0x832B0178 => {
    //   block [0x832B0178..0x832B0184)
	// 832B0178: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B017C: 386BD900  addi r3, r11, -0x2700
	ctx.r[3].s64 = ctx.r[11].s64 + -9984;
	// 832B0180: 4AF64C58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0188 size=12
    let mut pc: u32 = 0x832B0188;
    'dispatch: loop {
        match pc {
            0x832B0188 => {
    //   block [0x832B0188..0x832B0194)
	// 832B0188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B018C: 386BD904  addi r3, r11, -0x26fc
	ctx.r[3].s64 = ctx.r[11].s64 + -9980;
	// 832B0190: 4AF64C48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0198 size=12
    let mut pc: u32 = 0x832B0198;
    'dispatch: loop {
        match pc {
            0x832B0198 => {
    //   block [0x832B0198..0x832B01A4)
	// 832B0198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B019C: 386BD908  addi r3, r11, -0x26f8
	ctx.r[3].s64 = ctx.r[11].s64 + -9976;
	// 832B01A0: 4AF64C38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B01A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B01A8 size=12
    let mut pc: u32 = 0x832B01A8;
    'dispatch: loop {
        match pc {
            0x832B01A8 => {
    //   block [0x832B01A8..0x832B01B4)
	// 832B01A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B01AC: 386BD90C  addi r3, r11, -0x26f4
	ctx.r[3].s64 = ctx.r[11].s64 + -9972;
	// 832B01B0: 4AF64C28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B01B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B01B8 size=12
    let mut pc: u32 = 0x832B01B8;
    'dispatch: loop {
        match pc {
            0x832B01B8 => {
    //   block [0x832B01B8..0x832B01C4)
	// 832B01B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B01BC: 386BD910  addi r3, r11, -0x26f0
	ctx.r[3].s64 = ctx.r[11].s64 + -9968;
	// 832B01C0: 4AF64C18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B01C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B01C8 size=12
    let mut pc: u32 = 0x832B01C8;
    'dispatch: loop {
        match pc {
            0x832B01C8 => {
    //   block [0x832B01C8..0x832B01D4)
	// 832B01C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B01CC: 386BD914  addi r3, r11, -0x26ec
	ctx.r[3].s64 = ctx.r[11].s64 + -9964;
	// 832B01D0: 4AF64C08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B01D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B01D8 size=12
    let mut pc: u32 = 0x832B01D8;
    'dispatch: loop {
        match pc {
            0x832B01D8 => {
    //   block [0x832B01D8..0x832B01E4)
	// 832B01D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B01DC: 386BD918  addi r3, r11, -0x26e8
	ctx.r[3].s64 = ctx.r[11].s64 + -9960;
	// 832B01E0: 4AF64BF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B01E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B01E8 size=12
    let mut pc: u32 = 0x832B01E8;
    'dispatch: loop {
        match pc {
            0x832B01E8 => {
    //   block [0x832B01E8..0x832B01F4)
	// 832B01E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B01EC: 386BD91C  addi r3, r11, -0x26e4
	ctx.r[3].s64 = ctx.r[11].s64 + -9956;
	// 832B01F0: 4AF64BE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B01F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B01F8 size=12
    let mut pc: u32 = 0x832B01F8;
    'dispatch: loop {
        match pc {
            0x832B01F8 => {
    //   block [0x832B01F8..0x832B0204)
	// 832B01F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B01FC: 386BD920  addi r3, r11, -0x26e0
	ctx.r[3].s64 = ctx.r[11].s64 + -9952;
	// 832B0200: 4AF64BD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0208 size=12
    let mut pc: u32 = 0x832B0208;
    'dispatch: loop {
        match pc {
            0x832B0208 => {
    //   block [0x832B0208..0x832B0214)
	// 832B0208: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B020C: 386BD924  addi r3, r11, -0x26dc
	ctx.r[3].s64 = ctx.r[11].s64 + -9948;
	// 832B0210: 4AF64BC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0218 size=12
    let mut pc: u32 = 0x832B0218;
    'dispatch: loop {
        match pc {
            0x832B0218 => {
    //   block [0x832B0218..0x832B0224)
	// 832B0218: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B021C: 386BD928  addi r3, r11, -0x26d8
	ctx.r[3].s64 = ctx.r[11].s64 + -9944;
	// 832B0220: 4AF64BB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0228 size=12
    let mut pc: u32 = 0x832B0228;
    'dispatch: loop {
        match pc {
            0x832B0228 => {
    //   block [0x832B0228..0x832B0234)
	// 832B0228: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B022C: 386BD92C  addi r3, r11, -0x26d4
	ctx.r[3].s64 = ctx.r[11].s64 + -9940;
	// 832B0230: 4AF64BA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0238 size=12
    let mut pc: u32 = 0x832B0238;
    'dispatch: loop {
        match pc {
            0x832B0238 => {
    //   block [0x832B0238..0x832B0244)
	// 832B0238: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B023C: 386BD930  addi r3, r11, -0x26d0
	ctx.r[3].s64 = ctx.r[11].s64 + -9936;
	// 832B0240: 4AF64B98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0248 size=12
    let mut pc: u32 = 0x832B0248;
    'dispatch: loop {
        match pc {
            0x832B0248 => {
    //   block [0x832B0248..0x832B0254)
	// 832B0248: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B024C: 386BD988  addi r3, r11, -0x2678
	ctx.r[3].s64 = ctx.r[11].s64 + -9848;
	// 832B0250: 4AF64B88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0258 size=12
    let mut pc: u32 = 0x832B0258;
    'dispatch: loop {
        match pc {
            0x832B0258 => {
    //   block [0x832B0258..0x832B0264)
	// 832B0258: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B025C: 386BD98C  addi r3, r11, -0x2674
	ctx.r[3].s64 = ctx.r[11].s64 + -9844;
	// 832B0260: 4AF64B78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0268 size=12
    let mut pc: u32 = 0x832B0268;
    'dispatch: loop {
        match pc {
            0x832B0268 => {
    //   block [0x832B0268..0x832B0274)
	// 832B0268: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B026C: 386BD990  addi r3, r11, -0x2670
	ctx.r[3].s64 = ctx.r[11].s64 + -9840;
	// 832B0270: 4AF64B68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0278 size=12
    let mut pc: u32 = 0x832B0278;
    'dispatch: loop {
        match pc {
            0x832B0278 => {
    //   block [0x832B0278..0x832B0284)
	// 832B0278: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B027C: 386BD994  addi r3, r11, -0x266c
	ctx.r[3].s64 = ctx.r[11].s64 + -9836;
	// 832B0280: 4AF64B58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0288 size=12
    let mut pc: u32 = 0x832B0288;
    'dispatch: loop {
        match pc {
            0x832B0288 => {
    //   block [0x832B0288..0x832B0294)
	// 832B0288: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B028C: 386BD998  addi r3, r11, -0x2668
	ctx.r[3].s64 = ctx.r[11].s64 + -9832;
	// 832B0290: 4AF64B48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0298 size=12
    let mut pc: u32 = 0x832B0298;
    'dispatch: loop {
        match pc {
            0x832B0298 => {
    //   block [0x832B0298..0x832B02A4)
	// 832B0298: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B029C: 386BD99C  addi r3, r11, -0x2664
	ctx.r[3].s64 = ctx.r[11].s64 + -9828;
	// 832B02A0: 4AF64B38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B02A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B02A8 size=12
    let mut pc: u32 = 0x832B02A8;
    'dispatch: loop {
        match pc {
            0x832B02A8 => {
    //   block [0x832B02A8..0x832B02B4)
	// 832B02A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B02AC: 386BD9A0  addi r3, r11, -0x2660
	ctx.r[3].s64 = ctx.r[11].s64 + -9824;
	// 832B02B0: 4AF64B28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B02B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B02B8 size=12
    let mut pc: u32 = 0x832B02B8;
    'dispatch: loop {
        match pc {
            0x832B02B8 => {
    //   block [0x832B02B8..0x832B02C4)
	// 832B02B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B02BC: 386BD9A4  addi r3, r11, -0x265c
	ctx.r[3].s64 = ctx.r[11].s64 + -9820;
	// 832B02C0: 4AF64B18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B02C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B02C8 size=12
    let mut pc: u32 = 0x832B02C8;
    'dispatch: loop {
        match pc {
            0x832B02C8 => {
    //   block [0x832B02C8..0x832B02D4)
	// 832B02C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B02CC: 386BD9A8  addi r3, r11, -0x2658
	ctx.r[3].s64 = ctx.r[11].s64 + -9816;
	// 832B02D0: 4AF64B08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B02D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B02D8 size=12
    let mut pc: u32 = 0x832B02D8;
    'dispatch: loop {
        match pc {
            0x832B02D8 => {
    //   block [0x832B02D8..0x832B02E4)
	// 832B02D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B02DC: 386BD9AC  addi r3, r11, -0x2654
	ctx.r[3].s64 = ctx.r[11].s64 + -9812;
	// 832B02E0: 4AF64AF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B02E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B02E8 size=12
    let mut pc: u32 = 0x832B02E8;
    'dispatch: loop {
        match pc {
            0x832B02E8 => {
    //   block [0x832B02E8..0x832B02F4)
	// 832B02E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B02EC: 386BD9B0  addi r3, r11, -0x2650
	ctx.r[3].s64 = ctx.r[11].s64 + -9808;
	// 832B02F0: 4AF64AE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B02F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B02F8 size=12
    let mut pc: u32 = 0x832B02F8;
    'dispatch: loop {
        match pc {
            0x832B02F8 => {
    //   block [0x832B02F8..0x832B0304)
	// 832B02F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B02FC: 386BD9B4  addi r3, r11, -0x264c
	ctx.r[3].s64 = ctx.r[11].s64 + -9804;
	// 832B0300: 4AF64AD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0308 size=12
    let mut pc: u32 = 0x832B0308;
    'dispatch: loop {
        match pc {
            0x832B0308 => {
    //   block [0x832B0308..0x832B0314)
	// 832B0308: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B030C: 386BD9B8  addi r3, r11, -0x2648
	ctx.r[3].s64 = ctx.r[11].s64 + -9800;
	// 832B0310: 4AF64AC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0318 size=12
    let mut pc: u32 = 0x832B0318;
    'dispatch: loop {
        match pc {
            0x832B0318 => {
    //   block [0x832B0318..0x832B0324)
	// 832B0318: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B031C: 386BD9BC  addi r3, r11, -0x2644
	ctx.r[3].s64 = ctx.r[11].s64 + -9796;
	// 832B0320: 4AF64AB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0328 size=12
    let mut pc: u32 = 0x832B0328;
    'dispatch: loop {
        match pc {
            0x832B0328 => {
    //   block [0x832B0328..0x832B0334)
	// 832B0328: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B032C: 386BD9C0  addi r3, r11, -0x2640
	ctx.r[3].s64 = ctx.r[11].s64 + -9792;
	// 832B0330: 4AF64AA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0338 size=12
    let mut pc: u32 = 0x832B0338;
    'dispatch: loop {
        match pc {
            0x832B0338 => {
    //   block [0x832B0338..0x832B0344)
	// 832B0338: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B033C: 386BD9C4  addi r3, r11, -0x263c
	ctx.r[3].s64 = ctx.r[11].s64 + -9788;
	// 832B0340: 4AF64A98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0348 size=12
    let mut pc: u32 = 0x832B0348;
    'dispatch: loop {
        match pc {
            0x832B0348 => {
    //   block [0x832B0348..0x832B0354)
	// 832B0348: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B034C: 386BD9C8  addi r3, r11, -0x2638
	ctx.r[3].s64 = ctx.r[11].s64 + -9784;
	// 832B0350: 4AF64A88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0358 size=12
    let mut pc: u32 = 0x832B0358;
    'dispatch: loop {
        match pc {
            0x832B0358 => {
    //   block [0x832B0358..0x832B0364)
	// 832B0358: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B035C: 386BD9CC  addi r3, r11, -0x2634
	ctx.r[3].s64 = ctx.r[11].s64 + -9780;
	// 832B0360: 4AF64A78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0368 size=12
    let mut pc: u32 = 0x832B0368;
    'dispatch: loop {
        match pc {
            0x832B0368 => {
    //   block [0x832B0368..0x832B0374)
	// 832B0368: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B036C: 386BD9D0  addi r3, r11, -0x2630
	ctx.r[3].s64 = ctx.r[11].s64 + -9776;
	// 832B0370: 4AF64A68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0378 size=12
    let mut pc: u32 = 0x832B0378;
    'dispatch: loop {
        match pc {
            0x832B0378 => {
    //   block [0x832B0378..0x832B0384)
	// 832B0378: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B037C: 386BD9D4  addi r3, r11, -0x262c
	ctx.r[3].s64 = ctx.r[11].s64 + -9772;
	// 832B0380: 4AF64A58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0388 size=12
    let mut pc: u32 = 0x832B0388;
    'dispatch: loop {
        match pc {
            0x832B0388 => {
    //   block [0x832B0388..0x832B0394)
	// 832B0388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B038C: 386BD9D8  addi r3, r11, -0x2628
	ctx.r[3].s64 = ctx.r[11].s64 + -9768;
	// 832B0390: 4AF64A48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0398 size=12
    let mut pc: u32 = 0x832B0398;
    'dispatch: loop {
        match pc {
            0x832B0398 => {
    //   block [0x832B0398..0x832B03A4)
	// 832B0398: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B039C: 386BD9DC  addi r3, r11, -0x2624
	ctx.r[3].s64 = ctx.r[11].s64 + -9764;
	// 832B03A0: 4AF64A38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B03A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B03A8 size=12
    let mut pc: u32 = 0x832B03A8;
    'dispatch: loop {
        match pc {
            0x832B03A8 => {
    //   block [0x832B03A8..0x832B03B4)
	// 832B03A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B03AC: 386BD9E0  addi r3, r11, -0x2620
	ctx.r[3].s64 = ctx.r[11].s64 + -9760;
	// 832B03B0: 4AF64A28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B03B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B03B8 size=12
    let mut pc: u32 = 0x832B03B8;
    'dispatch: loop {
        match pc {
            0x832B03B8 => {
    //   block [0x832B03B8..0x832B03C4)
	// 832B03B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B03BC: 386BDA38  addi r3, r11, -0x25c8
	ctx.r[3].s64 = ctx.r[11].s64 + -9672;
	// 832B03C0: 4AF64A18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B03C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B03C8 size=12
    let mut pc: u32 = 0x832B03C8;
    'dispatch: loop {
        match pc {
            0x832B03C8 => {
    //   block [0x832B03C8..0x832B03D4)
	// 832B03C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B03CC: 386BDA3C  addi r3, r11, -0x25c4
	ctx.r[3].s64 = ctx.r[11].s64 + -9668;
	// 832B03D0: 4AF64A08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B03D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B03D8 size=12
    let mut pc: u32 = 0x832B03D8;
    'dispatch: loop {
        match pc {
            0x832B03D8 => {
    //   block [0x832B03D8..0x832B03E4)
	// 832B03D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B03DC: 386BDA40  addi r3, r11, -0x25c0
	ctx.r[3].s64 = ctx.r[11].s64 + -9664;
	// 832B03E0: 4AF649F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B03E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B03E8 size=12
    let mut pc: u32 = 0x832B03E8;
    'dispatch: loop {
        match pc {
            0x832B03E8 => {
    //   block [0x832B03E8..0x832B03F4)
	// 832B03E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B03EC: 386BDA44  addi r3, r11, -0x25bc
	ctx.r[3].s64 = ctx.r[11].s64 + -9660;
	// 832B03F0: 4AF649E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B03F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B03F8 size=12
    let mut pc: u32 = 0x832B03F8;
    'dispatch: loop {
        match pc {
            0x832B03F8 => {
    //   block [0x832B03F8..0x832B0404)
	// 832B03F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B03FC: 386BDA48  addi r3, r11, -0x25b8
	ctx.r[3].s64 = ctx.r[11].s64 + -9656;
	// 832B0400: 4AF649D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0408 size=12
    let mut pc: u32 = 0x832B0408;
    'dispatch: loop {
        match pc {
            0x832B0408 => {
    //   block [0x832B0408..0x832B0414)
	// 832B0408: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B040C: 386BDA4C  addi r3, r11, -0x25b4
	ctx.r[3].s64 = ctx.r[11].s64 + -9652;
	// 832B0410: 4AF649C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0418 size=12
    let mut pc: u32 = 0x832B0418;
    'dispatch: loop {
        match pc {
            0x832B0418 => {
    //   block [0x832B0418..0x832B0424)
	// 832B0418: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B041C: 386BDA50  addi r3, r11, -0x25b0
	ctx.r[3].s64 = ctx.r[11].s64 + -9648;
	// 832B0420: 4AF649B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0428 size=12
    let mut pc: u32 = 0x832B0428;
    'dispatch: loop {
        match pc {
            0x832B0428 => {
    //   block [0x832B0428..0x832B0434)
	// 832B0428: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B042C: 386BDA54  addi r3, r11, -0x25ac
	ctx.r[3].s64 = ctx.r[11].s64 + -9644;
	// 832B0430: 4AF649A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0438 size=12
    let mut pc: u32 = 0x832B0438;
    'dispatch: loop {
        match pc {
            0x832B0438 => {
    //   block [0x832B0438..0x832B0444)
	// 832B0438: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B043C: 386BDA58  addi r3, r11, -0x25a8
	ctx.r[3].s64 = ctx.r[11].s64 + -9640;
	// 832B0440: 4AF64998  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0448 size=12
    let mut pc: u32 = 0x832B0448;
    'dispatch: loop {
        match pc {
            0x832B0448 => {
    //   block [0x832B0448..0x832B0454)
	// 832B0448: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B044C: 386BDA5C  addi r3, r11, -0x25a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9636;
	// 832B0450: 4AF64988  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B0458 size=104
    let mut pc: u32 = 0x832B0458;
    'dispatch: loop {
        match pc {
            0x832B0458 => {
    //   block [0x832B0458..0x832B0480)
	// 832B0458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B045C: 4B9F8FAD  bl 0x82ca9408
	ctx.lr = 0x832B0460;
	sub_82CA93D0(ctx, base);
	// 832B0460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B0464: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0468: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832B046C: 396BDAB8  addi r11, r11, -0x2548
	ctx.r[11].s64 = ctx.r[11].s64 + -9544;
	// 832B0470: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B0474: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B0478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B047C: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832B0480; continue 'dispatch;
            }
            0x832B0480 => {
    //   block [0x832B0480..0x832B0490)
	// 832B0480: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B0484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B0488: 4AF162E1  bl 0x821c6768
	ctx.lr = 0x832B048C;
	sub_821C6768(ctx, base);
	// 832B048C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832B0490; continue 'dispatch;
            }
            0x832B0490 => {
    //   block [0x832B0490..0x832B04C0)
	// 832B0490: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B0494: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B0498: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B049C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B04A0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B04A4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B04A8: 4082FFE8  bne 0x832b0490
	if !ctx.cr[0].eq {
	pc = 0x832B0490; continue 'dispatch;
	}
	// 832B04AC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B04B0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B04B4: 4080FFCC  bge 0x832b0480
	if !ctx.cr[0].lt {
	pc = 0x832B0480; continue 'dispatch;
	}
	// 832B04B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B04BC: 4B9F8F9C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B04C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B04C0 size=104
    let mut pc: u32 = 0x832B04C0;
    'dispatch: loop {
        match pc {
            0x832B04C0 => {
    //   block [0x832B04C0..0x832B04E8)
	// 832B04C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B04C4: 4B9F8F45  bl 0x82ca9408
	ctx.lr = 0x832B04C8;
	sub_82CA93D0(ctx, base);
	// 832B04C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B04CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B04D0: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832B04D4: 396BDAF8  addi r11, r11, -0x2508
	ctx.r[11].s64 = ctx.r[11].s64 + -9480;
	// 832B04D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B04DC: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832B04E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B04E4: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832B04E8; continue 'dispatch;
            }
            0x832B04E8 => {
    //   block [0x832B04E8..0x832B04F8)
	// 832B04E8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832B04EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B04F0: 4AF16279  bl 0x821c6768
	ctx.lr = 0x832B04F4;
	sub_821C6768(ctx, base);
	// 832B04F4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832B04F8; continue 'dispatch;
            }
            0x832B04F8 => {
    //   block [0x832B04F8..0x832B0528)
	// 832B04F8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B04FC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B0500: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B0504: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B0508: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B050C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B0510: 4082FFE8  bne 0x832b04f8
	if !ctx.cr[0].eq {
	pc = 0x832B04F8; continue 'dispatch;
	}
	// 832B0514: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B0518: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B051C: 4080FFCC  bge 0x832b04e8
	if !ctx.cr[0].lt {
	pc = 0x832B04E8; continue 'dispatch;
	}
	// 832B0520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B0524: 4B9F8F34  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0528 size=12
    let mut pc: u32 = 0x832B0528;
    'dispatch: loop {
        match pc {
            0x832B0528 => {
    //   block [0x832B0528..0x832B0534)
	// 832B0528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B052C: 386BDAB4  addi r3, r11, -0x254c
	ctx.r[3].s64 = ctx.r[11].s64 + -9548;
	// 832B0530: 4AF648A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0538 size=12
    let mut pc: u32 = 0x832B0538;
    'dispatch: loop {
        match pc {
            0x832B0538 => {
    //   block [0x832B0538..0x832B0544)
	// 832B0538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B053C: 386BDB38  addi r3, r11, -0x24c8
	ctx.r[3].s64 = ctx.r[11].s64 + -9416;
	// 832B0540: 4AF64898  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0548 size=12
    let mut pc: u32 = 0x832B0548;
    'dispatch: loop {
        match pc {
            0x832B0548 => {
    //   block [0x832B0548..0x832B0554)
	// 832B0548: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B054C: 386BDB3C  addi r3, r11, -0x24c4
	ctx.r[3].s64 = ctx.r[11].s64 + -9412;
	// 832B0550: 4AF64888  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0558 size=12
    let mut pc: u32 = 0x832B0558;
    'dispatch: loop {
        match pc {
            0x832B0558 => {
    //   block [0x832B0558..0x832B0564)
	// 832B0558: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B055C: 386BDB40  addi r3, r11, -0x24c0
	ctx.r[3].s64 = ctx.r[11].s64 + -9408;
	// 832B0560: 4AF64878  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0568 size=12
    let mut pc: u32 = 0x832B0568;
    'dispatch: loop {
        match pc {
            0x832B0568 => {
    //   block [0x832B0568..0x832B0574)
	// 832B0568: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B056C: 386BDB44  addi r3, r11, -0x24bc
	ctx.r[3].s64 = ctx.r[11].s64 + -9404;
	// 832B0570: 4AF64868  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0578 size=12
    let mut pc: u32 = 0x832B0578;
    'dispatch: loop {
        match pc {
            0x832B0578 => {
    //   block [0x832B0578..0x832B0584)
	// 832B0578: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B057C: 386BDB48  addi r3, r11, -0x24b8
	ctx.r[3].s64 = ctx.r[11].s64 + -9400;
	// 832B0580: 4AF64858  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0588 size=12
    let mut pc: u32 = 0x832B0588;
    'dispatch: loop {
        match pc {
            0x832B0588 => {
    //   block [0x832B0588..0x832B0594)
	// 832B0588: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B058C: 386BDB4C  addi r3, r11, -0x24b4
	ctx.r[3].s64 = ctx.r[11].s64 + -9396;
	// 832B0590: 4AF64848  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0598 size=12
    let mut pc: u32 = 0x832B0598;
    'dispatch: loop {
        match pc {
            0x832B0598 => {
    //   block [0x832B0598..0x832B05A4)
	// 832B0598: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B059C: 386BDB50  addi r3, r11, -0x24b0
	ctx.r[3].s64 = ctx.r[11].s64 + -9392;
	// 832B05A0: 4AF64838  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B05A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B05A8 size=12
    let mut pc: u32 = 0x832B05A8;
    'dispatch: loop {
        match pc {
            0x832B05A8 => {
    //   block [0x832B05A8..0x832B05B4)
	// 832B05A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B05AC: 386BDB54  addi r3, r11, -0x24ac
	ctx.r[3].s64 = ctx.r[11].s64 + -9388;
	// 832B05B0: 4AF64828  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B05B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B05B8 size=12
    let mut pc: u32 = 0x832B05B8;
    'dispatch: loop {
        match pc {
            0x832B05B8 => {
    //   block [0x832B05B8..0x832B05C4)
	// 832B05B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B05BC: 386BDB58  addi r3, r11, -0x24a8
	ctx.r[3].s64 = ctx.r[11].s64 + -9384;
	// 832B05C0: 4AF64818  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B05C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B05C8 size=12
    let mut pc: u32 = 0x832B05C8;
    'dispatch: loop {
        match pc {
            0x832B05C8 => {
    //   block [0x832B05C8..0x832B05D4)
	// 832B05C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B05CC: 386BDB5C  addi r3, r11, -0x24a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9380;
	// 832B05D0: 4AF64808  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B05D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B05D8 size=12
    let mut pc: u32 = 0x832B05D8;
    'dispatch: loop {
        match pc {
            0x832B05D8 => {
    //   block [0x832B05D8..0x832B05E4)
	// 832B05D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B05DC: 386BDB60  addi r3, r11, -0x24a0
	ctx.r[3].s64 = ctx.r[11].s64 + -9376;
	// 832B05E0: 4AF647F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B05E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B05E8 size=12
    let mut pc: u32 = 0x832B05E8;
    'dispatch: loop {
        match pc {
            0x832B05E8 => {
    //   block [0x832B05E8..0x832B05F4)
	// 832B05E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B05EC: 386BDB64  addi r3, r11, -0x249c
	ctx.r[3].s64 = ctx.r[11].s64 + -9372;
	// 832B05F0: 4AF647E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B05F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B05F8 size=12
    let mut pc: u32 = 0x832B05F8;
    'dispatch: loop {
        match pc {
            0x832B05F8 => {
    //   block [0x832B05F8..0x832B0604)
	// 832B05F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B05FC: 386BDBBC  addi r3, r11, -0x2444
	ctx.r[3].s64 = ctx.r[11].s64 + -9284;
	// 832B0600: 4AF647D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0608 size=12
    let mut pc: u32 = 0x832B0608;
    'dispatch: loop {
        match pc {
            0x832B0608 => {
    //   block [0x832B0608..0x832B0614)
	// 832B0608: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B060C: 386BDBC0  addi r3, r11, -0x2440
	ctx.r[3].s64 = ctx.r[11].s64 + -9280;
	// 832B0610: 4AF647C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0618 size=12
    let mut pc: u32 = 0x832B0618;
    'dispatch: loop {
        match pc {
            0x832B0618 => {
    //   block [0x832B0618..0x832B0624)
	// 832B0618: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B061C: 386BDBC4  addi r3, r11, -0x243c
	ctx.r[3].s64 = ctx.r[11].s64 + -9276;
	// 832B0620: 4AF647B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0628 size=12
    let mut pc: u32 = 0x832B0628;
    'dispatch: loop {
        match pc {
            0x832B0628 => {
    //   block [0x832B0628..0x832B0634)
	// 832B0628: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B062C: 386BDBC8  addi r3, r11, -0x2438
	ctx.r[3].s64 = ctx.r[11].s64 + -9272;
	// 832B0630: 4AF647A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0638 size=12
    let mut pc: u32 = 0x832B0638;
    'dispatch: loop {
        match pc {
            0x832B0638 => {
    //   block [0x832B0638..0x832B0644)
	// 832B0638: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B063C: 386BDBCC  addi r3, r11, -0x2434
	ctx.r[3].s64 = ctx.r[11].s64 + -9268;
	// 832B0640: 4AF64798  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0648 size=12
    let mut pc: u32 = 0x832B0648;
    'dispatch: loop {
        match pc {
            0x832B0648 => {
    //   block [0x832B0648..0x832B0654)
	// 832B0648: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B064C: 386BDBD0  addi r3, r11, -0x2430
	ctx.r[3].s64 = ctx.r[11].s64 + -9264;
	// 832B0650: 4AF64788  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0658 size=12
    let mut pc: u32 = 0x832B0658;
    'dispatch: loop {
        match pc {
            0x832B0658 => {
    //   block [0x832B0658..0x832B0664)
	// 832B0658: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B065C: 386BDBD4  addi r3, r11, -0x242c
	ctx.r[3].s64 = ctx.r[11].s64 + -9260;
	// 832B0660: 4AF64778  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0668 size=12
    let mut pc: u32 = 0x832B0668;
    'dispatch: loop {
        match pc {
            0x832B0668 => {
    //   block [0x832B0668..0x832B0674)
	// 832B0668: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B066C: 386BDBD8  addi r3, r11, -0x2428
	ctx.r[3].s64 = ctx.r[11].s64 + -9256;
	// 832B0670: 4AF64768  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0678 size=12
    let mut pc: u32 = 0x832B0678;
    'dispatch: loop {
        match pc {
            0x832B0678 => {
    //   block [0x832B0678..0x832B0684)
	// 832B0678: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B067C: 386BDBDC  addi r3, r11, -0x2424
	ctx.r[3].s64 = ctx.r[11].s64 + -9252;
	// 832B0680: 4AF64758  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0688 size=12
    let mut pc: u32 = 0x832B0688;
    'dispatch: loop {
        match pc {
            0x832B0688 => {
    //   block [0x832B0688..0x832B0694)
	// 832B0688: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B068C: 386BDBE0  addi r3, r11, -0x2420
	ctx.r[3].s64 = ctx.r[11].s64 + -9248;
	// 832B0690: 4AF64748  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0698 size=12
    let mut pc: u32 = 0x832B0698;
    'dispatch: loop {
        match pc {
            0x832B0698 => {
    //   block [0x832B0698..0x832B06A4)
	// 832B0698: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B069C: 386BDBE4  addi r3, r11, -0x241c
	ctx.r[3].s64 = ctx.r[11].s64 + -9244;
	// 832B06A0: 4AF64738  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B06A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B06A8 size=12
    let mut pc: u32 = 0x832B06A8;
    'dispatch: loop {
        match pc {
            0x832B06A8 => {
    //   block [0x832B06A8..0x832B06B4)
	// 832B06A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B06AC: 386BDBE8  addi r3, r11, -0x2418
	ctx.r[3].s64 = ctx.r[11].s64 + -9240;
	// 832B06B0: 4AF64728  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B06B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B06B8 size=12
    let mut pc: u32 = 0x832B06B8;
    'dispatch: loop {
        match pc {
            0x832B06B8 => {
    //   block [0x832B06B8..0x832B06C4)
	// 832B06B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B06BC: 386BDBEC  addi r3, r11, -0x2414
	ctx.r[3].s64 = ctx.r[11].s64 + -9236;
	// 832B06C0: 4AF64718  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B06C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B06C8 size=12
    let mut pc: u32 = 0x832B06C8;
    'dispatch: loop {
        match pc {
            0x832B06C8 => {
    //   block [0x832B06C8..0x832B06D4)
	// 832B06C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B06CC: 386BDBF0  addi r3, r11, -0x2410
	ctx.r[3].s64 = ctx.r[11].s64 + -9232;
	// 832B06D0: 4AF64708  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B06D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B06D8 size=12
    let mut pc: u32 = 0x832B06D8;
    'dispatch: loop {
        match pc {
            0x832B06D8 => {
    //   block [0x832B06D8..0x832B06E4)
	// 832B06D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B06DC: 386BDBF4  addi r3, r11, -0x240c
	ctx.r[3].s64 = ctx.r[11].s64 + -9228;
	// 832B06E0: 4AF646F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B06E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B06E8 size=12
    let mut pc: u32 = 0x832B06E8;
    'dispatch: loop {
        match pc {
            0x832B06E8 => {
    //   block [0x832B06E8..0x832B06F4)
	// 832B06E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B06EC: 386BDC4C  addi r3, r11, -0x23b4
	ctx.r[3].s64 = ctx.r[11].s64 + -9140;
	// 832B06F0: 4AF646E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B06F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B06F8 size=12
    let mut pc: u32 = 0x832B06F8;
    'dispatch: loop {
        match pc {
            0x832B06F8 => {
    //   block [0x832B06F8..0x832B0704)
	// 832B06F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B06FC: 386BDC50  addi r3, r11, -0x23b0
	ctx.r[3].s64 = ctx.r[11].s64 + -9136;
	// 832B0700: 4AF646D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0708 size=12
    let mut pc: u32 = 0x832B0708;
    'dispatch: loop {
        match pc {
            0x832B0708 => {
    //   block [0x832B0708..0x832B0714)
	// 832B0708: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B070C: 386BDC54  addi r3, r11, -0x23ac
	ctx.r[3].s64 = ctx.r[11].s64 + -9132;
	// 832B0710: 4AF646C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0718 size=12
    let mut pc: u32 = 0x832B0718;
    'dispatch: loop {
        match pc {
            0x832B0718 => {
    //   block [0x832B0718..0x832B0724)
	// 832B0718: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B071C: 386BDC58  addi r3, r11, -0x23a8
	ctx.r[3].s64 = ctx.r[11].s64 + -9128;
	// 832B0720: 4AF646B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0728 size=12
    let mut pc: u32 = 0x832B0728;
    'dispatch: loop {
        match pc {
            0x832B0728 => {
    //   block [0x832B0728..0x832B0734)
	// 832B0728: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B072C: 386BDC5C  addi r3, r11, -0x23a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9124;
	// 832B0730: 4AF646A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0738 size=12
    let mut pc: u32 = 0x832B0738;
    'dispatch: loop {
        match pc {
            0x832B0738 => {
    //   block [0x832B0738..0x832B0744)
	// 832B0738: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B073C: 386BDC60  addi r3, r11, -0x23a0
	ctx.r[3].s64 = ctx.r[11].s64 + -9120;
	// 832B0740: 4AF64698  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0748 size=12
    let mut pc: u32 = 0x832B0748;
    'dispatch: loop {
        match pc {
            0x832B0748 => {
    //   block [0x832B0748..0x832B0754)
	// 832B0748: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B074C: 386BDC64  addi r3, r11, -0x239c
	ctx.r[3].s64 = ctx.r[11].s64 + -9116;
	// 832B0750: 4AF64688  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0758 size=12
    let mut pc: u32 = 0x832B0758;
    'dispatch: loop {
        match pc {
            0x832B0758 => {
    //   block [0x832B0758..0x832B0764)
	// 832B0758: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B075C: 386BDC68  addi r3, r11, -0x2398
	ctx.r[3].s64 = ctx.r[11].s64 + -9112;
	// 832B0760: 4AF64678  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0768 size=12
    let mut pc: u32 = 0x832B0768;
    'dispatch: loop {
        match pc {
            0x832B0768 => {
    //   block [0x832B0768..0x832B0774)
	// 832B0768: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B076C: 386BDC6C  addi r3, r11, -0x2394
	ctx.r[3].s64 = ctx.r[11].s64 + -9108;
	// 832B0770: 4AF64668  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0778 size=12
    let mut pc: u32 = 0x832B0778;
    'dispatch: loop {
        match pc {
            0x832B0778 => {
    //   block [0x832B0778..0x832B0784)
	// 832B0778: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B077C: 386BDC70  addi r3, r11, -0x2390
	ctx.r[3].s64 = ctx.r[11].s64 + -9104;
	// 832B0780: 4AF64658  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0788 size=12
    let mut pc: u32 = 0x832B0788;
    'dispatch: loop {
        match pc {
            0x832B0788 => {
    //   block [0x832B0788..0x832B0794)
	// 832B0788: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B078C: 386BDC74  addi r3, r11, -0x238c
	ctx.r[3].s64 = ctx.r[11].s64 + -9100;
	// 832B0790: 4AF64648  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0798 size=12
    let mut pc: u32 = 0x832B0798;
    'dispatch: loop {
        match pc {
            0x832B0798 => {
    //   block [0x832B0798..0x832B07A4)
	// 832B0798: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B079C: 386BDC78  addi r3, r11, -0x2388
	ctx.r[3].s64 = ctx.r[11].s64 + -9096;
	// 832B07A0: 4AF64638  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B07A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B07A8 size=12
    let mut pc: u32 = 0x832B07A8;
    'dispatch: loop {
        match pc {
            0x832B07A8 => {
    //   block [0x832B07A8..0x832B07B4)
	// 832B07A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B07AC: 386BDC7C  addi r3, r11, -0x2384
	ctx.r[3].s64 = ctx.r[11].s64 + -9092;
	// 832B07B0: 4AF64628  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B07B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B07B8 size=12
    let mut pc: u32 = 0x832B07B8;
    'dispatch: loop {
        match pc {
            0x832B07B8 => {
    //   block [0x832B07B8..0x832B07C4)
	// 832B07B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B07BC: 386BDC80  addi r3, r11, -0x2380
	ctx.r[3].s64 = ctx.r[11].s64 + -9088;
	// 832B07C0: 4AF64618  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B07C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B07C8 size=12
    let mut pc: u32 = 0x832B07C8;
    'dispatch: loop {
        match pc {
            0x832B07C8 => {
    //   block [0x832B07C8..0x832B07D4)
	// 832B07C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B07CC: 386BDC84  addi r3, r11, -0x237c
	ctx.r[3].s64 = ctx.r[11].s64 + -9084;
	// 832B07D0: 4AF64608  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B07D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B07D8 size=12
    let mut pc: u32 = 0x832B07D8;
    'dispatch: loop {
        match pc {
            0x832B07D8 => {
    //   block [0x832B07D8..0x832B07E4)
	// 832B07D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B07DC: 386BDC88  addi r3, r11, -0x2378
	ctx.r[3].s64 = ctx.r[11].s64 + -9080;
	// 832B07E0: 4AF645F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B07E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B07E8 size=12
    let mut pc: u32 = 0x832B07E8;
    'dispatch: loop {
        match pc {
            0x832B07E8 => {
    //   block [0x832B07E8..0x832B07F4)
	// 832B07E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B07EC: 386BDC8C  addi r3, r11, -0x2374
	ctx.r[3].s64 = ctx.r[11].s64 + -9076;
	// 832B07F0: 4AF645E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B07F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B07F8 size=12
    let mut pc: u32 = 0x832B07F8;
    'dispatch: loop {
        match pc {
            0x832B07F8 => {
    //   block [0x832B07F8..0x832B0804)
	// 832B07F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B07FC: 386BDC90  addi r3, r11, -0x2370
	ctx.r[3].s64 = ctx.r[11].s64 + -9072;
	// 832B0800: 4AF645D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0808 size=12
    let mut pc: u32 = 0x832B0808;
    'dispatch: loop {
        match pc {
            0x832B0808 => {
    //   block [0x832B0808..0x832B0814)
	// 832B0808: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B080C: 386BDC94  addi r3, r11, -0x236c
	ctx.r[3].s64 = ctx.r[11].s64 + -9068;
	// 832B0810: 4AF645C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0818 size=12
    let mut pc: u32 = 0x832B0818;
    'dispatch: loop {
        match pc {
            0x832B0818 => {
    //   block [0x832B0818..0x832B0824)
	// 832B0818: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B081C: 386BDC98  addi r3, r11, -0x2368
	ctx.r[3].s64 = ctx.r[11].s64 + -9064;
	// 832B0820: 4AF645B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0828 size=12
    let mut pc: u32 = 0x832B0828;
    'dispatch: loop {
        match pc {
            0x832B0828 => {
    //   block [0x832B0828..0x832B0834)
	// 832B0828: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B082C: 386BDC9C  addi r3, r11, -0x2364
	ctx.r[3].s64 = ctx.r[11].s64 + -9060;
	// 832B0830: 4AF645A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0838 size=12
    let mut pc: u32 = 0x832B0838;
    'dispatch: loop {
        match pc {
            0x832B0838 => {
    //   block [0x832B0838..0x832B0844)
	// 832B0838: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B083C: 386BDCA0  addi r3, r11, -0x2360
	ctx.r[3].s64 = ctx.r[11].s64 + -9056;
	// 832B0840: 4AF64598  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0848 size=12
    let mut pc: u32 = 0x832B0848;
    'dispatch: loop {
        match pc {
            0x832B0848 => {
    //   block [0x832B0848..0x832B0854)
	// 832B0848: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B084C: 386BDCF8  addi r3, r11, -0x2308
	ctx.r[3].s64 = ctx.r[11].s64 + -8968;
	// 832B0850: 4AF64588  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0858 size=12
    let mut pc: u32 = 0x832B0858;
    'dispatch: loop {
        match pc {
            0x832B0858 => {
    //   block [0x832B0858..0x832B0864)
	// 832B0858: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B085C: 386BDCFC  addi r3, r11, -0x2304
	ctx.r[3].s64 = ctx.r[11].s64 + -8964;
	// 832B0860: 4AF64578  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0868 size=12
    let mut pc: u32 = 0x832B0868;
    'dispatch: loop {
        match pc {
            0x832B0868 => {
    //   block [0x832B0868..0x832B0874)
	// 832B0868: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B086C: 386BDD00  addi r3, r11, -0x2300
	ctx.r[3].s64 = ctx.r[11].s64 + -8960;
	// 832B0870: 4AF64568  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0878 size=12
    let mut pc: u32 = 0x832B0878;
    'dispatch: loop {
        match pc {
            0x832B0878 => {
    //   block [0x832B0878..0x832B0884)
	// 832B0878: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B087C: 386BDD04  addi r3, r11, -0x22fc
	ctx.r[3].s64 = ctx.r[11].s64 + -8956;
	// 832B0880: 4AF64558  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0888 size=12
    let mut pc: u32 = 0x832B0888;
    'dispatch: loop {
        match pc {
            0x832B0888 => {
    //   block [0x832B0888..0x832B0894)
	// 832B0888: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B088C: 386BDD08  addi r3, r11, -0x22f8
	ctx.r[3].s64 = ctx.r[11].s64 + -8952;
	// 832B0890: 4AF64548  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0898 size=12
    let mut pc: u32 = 0x832B0898;
    'dispatch: loop {
        match pc {
            0x832B0898 => {
    //   block [0x832B0898..0x832B08A4)
	// 832B0898: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B089C: 386BDD0C  addi r3, r11, -0x22f4
	ctx.r[3].s64 = ctx.r[11].s64 + -8948;
	// 832B08A0: 4AF64538  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B08A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B08A8 size=12
    let mut pc: u32 = 0x832B08A8;
    'dispatch: loop {
        match pc {
            0x832B08A8 => {
    //   block [0x832B08A8..0x832B08B4)
	// 832B08A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B08AC: 386BDD10  addi r3, r11, -0x22f0
	ctx.r[3].s64 = ctx.r[11].s64 + -8944;
	// 832B08B0: 4AF64528  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B08B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B08B8 size=12
    let mut pc: u32 = 0x832B08B8;
    'dispatch: loop {
        match pc {
            0x832B08B8 => {
    //   block [0x832B08B8..0x832B08C4)
	// 832B08B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B08BC: 386BDD14  addi r3, r11, -0x22ec
	ctx.r[3].s64 = ctx.r[11].s64 + -8940;
	// 832B08C0: 4AF64518  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B08C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B08C8 size=12
    let mut pc: u32 = 0x832B08C8;
    'dispatch: loop {
        match pc {
            0x832B08C8 => {
    //   block [0x832B08C8..0x832B08D4)
	// 832B08C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B08CC: 386BDD18  addi r3, r11, -0x22e8
	ctx.r[3].s64 = ctx.r[11].s64 + -8936;
	// 832B08D0: 4AF64508  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B08D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B08D8 size=12
    let mut pc: u32 = 0x832B08D8;
    'dispatch: loop {
        match pc {
            0x832B08D8 => {
    //   block [0x832B08D8..0x832B08E4)
	// 832B08D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B08DC: 386BDD1C  addi r3, r11, -0x22e4
	ctx.r[3].s64 = ctx.r[11].s64 + -8932;
	// 832B08E0: 4AF644F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B08E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B08E8 size=12
    let mut pc: u32 = 0x832B08E8;
    'dispatch: loop {
        match pc {
            0x832B08E8 => {
    //   block [0x832B08E8..0x832B08F4)
	// 832B08E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B08EC: 386BDD20  addi r3, r11, -0x22e0
	ctx.r[3].s64 = ctx.r[11].s64 + -8928;
	// 832B08F0: 4AF644E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B08F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B08F8 size=12
    let mut pc: u32 = 0x832B08F8;
    'dispatch: loop {
        match pc {
            0x832B08F8 => {
    //   block [0x832B08F8..0x832B0904)
	// 832B08F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B08FC: 386BDD24  addi r3, r11, -0x22dc
	ctx.r[3].s64 = ctx.r[11].s64 + -8924;
	// 832B0900: 4AF644D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0908 size=12
    let mut pc: u32 = 0x832B0908;
    'dispatch: loop {
        match pc {
            0x832B0908 => {
    //   block [0x832B0908..0x832B0914)
	// 832B0908: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B090C: 386BDD28  addi r3, r11, -0x22d8
	ctx.r[3].s64 = ctx.r[11].s64 + -8920;
	// 832B0910: 4AF644C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0918 size=12
    let mut pc: u32 = 0x832B0918;
    'dispatch: loop {
        match pc {
            0x832B0918 => {
    //   block [0x832B0918..0x832B0924)
	// 832B0918: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B091C: 386BDD2C  addi r3, r11, -0x22d4
	ctx.r[3].s64 = ctx.r[11].s64 + -8916;
	// 832B0920: 4AF644B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0928 size=12
    let mut pc: u32 = 0x832B0928;
    'dispatch: loop {
        match pc {
            0x832B0928 => {
    //   block [0x832B0928..0x832B0934)
	// 832B0928: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B092C: 386BDD84  addi r3, r11, -0x227c
	ctx.r[3].s64 = ctx.r[11].s64 + -8828;
	// 832B0930: 4AF644A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0938 size=12
    let mut pc: u32 = 0x832B0938;
    'dispatch: loop {
        match pc {
            0x832B0938 => {
    //   block [0x832B0938..0x832B0944)
	// 832B0938: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B093C: 386BDD88  addi r3, r11, -0x2278
	ctx.r[3].s64 = ctx.r[11].s64 + -8824;
	// 832B0940: 4AF64498  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0948 size=12
    let mut pc: u32 = 0x832B0948;
    'dispatch: loop {
        match pc {
            0x832B0948 => {
    //   block [0x832B0948..0x832B0954)
	// 832B0948: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B094C: 386BDD8C  addi r3, r11, -0x2274
	ctx.r[3].s64 = ctx.r[11].s64 + -8820;
	// 832B0950: 4AF64488  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0958 size=12
    let mut pc: u32 = 0x832B0958;
    'dispatch: loop {
        match pc {
            0x832B0958 => {
    //   block [0x832B0958..0x832B0964)
	// 832B0958: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B095C: 386BDD90  addi r3, r11, -0x2270
	ctx.r[3].s64 = ctx.r[11].s64 + -8816;
	// 832B0960: 4AF64478  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0968 size=12
    let mut pc: u32 = 0x832B0968;
    'dispatch: loop {
        match pc {
            0x832B0968 => {
    //   block [0x832B0968..0x832B0974)
	// 832B0968: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B096C: 386BDD94  addi r3, r11, -0x226c
	ctx.r[3].s64 = ctx.r[11].s64 + -8812;
	// 832B0970: 4AF64468  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0978 size=12
    let mut pc: u32 = 0x832B0978;
    'dispatch: loop {
        match pc {
            0x832B0978 => {
    //   block [0x832B0978..0x832B0984)
	// 832B0978: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B097C: 386BDD98  addi r3, r11, -0x2268
	ctx.r[3].s64 = ctx.r[11].s64 + -8808;
	// 832B0980: 4AF64458  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0988 size=12
    let mut pc: u32 = 0x832B0988;
    'dispatch: loop {
        match pc {
            0x832B0988 => {
    //   block [0x832B0988..0x832B0994)
	// 832B0988: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B098C: 386BDD9C  addi r3, r11, -0x2264
	ctx.r[3].s64 = ctx.r[11].s64 + -8804;
	// 832B0990: 4AF64448  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0998 size=12
    let mut pc: u32 = 0x832B0998;
    'dispatch: loop {
        match pc {
            0x832B0998 => {
    //   block [0x832B0998..0x832B09A4)
	// 832B0998: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B099C: 386BDDA0  addi r3, r11, -0x2260
	ctx.r[3].s64 = ctx.r[11].s64 + -8800;
	// 832B09A0: 4AF64438  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B09A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B09A8 size=12
    let mut pc: u32 = 0x832B09A8;
    'dispatch: loop {
        match pc {
            0x832B09A8 => {
    //   block [0x832B09A8..0x832B09B4)
	// 832B09A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B09AC: 386BDDA4  addi r3, r11, -0x225c
	ctx.r[3].s64 = ctx.r[11].s64 + -8796;
	// 832B09B0: 4AF64428  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B09B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B09B8 size=12
    let mut pc: u32 = 0x832B09B8;
    'dispatch: loop {
        match pc {
            0x832B09B8 => {
    //   block [0x832B09B8..0x832B09C4)
	// 832B09B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B09BC: 386BDDA8  addi r3, r11, -0x2258
	ctx.r[3].s64 = ctx.r[11].s64 + -8792;
	// 832B09C0: 4AF64418  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B09C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B09C8 size=12
    let mut pc: u32 = 0x832B09C8;
    'dispatch: loop {
        match pc {
            0x832B09C8 => {
    //   block [0x832B09C8..0x832B09D4)
	// 832B09C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B09CC: 386BDDAC  addi r3, r11, -0x2254
	ctx.r[3].s64 = ctx.r[11].s64 + -8788;
	// 832B09D0: 4AF64408  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B09D8 size=12
    let mut pc: u32 = 0x832B09D8;
    'dispatch: loop {
        match pc {
            0x832B09D8 => {
    //   block [0x832B09D8..0x832B09E4)
	// 832B09D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B09DC: 386BDDB0  addi r3, r11, -0x2250
	ctx.r[3].s64 = ctx.r[11].s64 + -8784;
	// 832B09E0: 4AF643F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B09E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B09E8 size=12
    let mut pc: u32 = 0x832B09E8;
    'dispatch: loop {
        match pc {
            0x832B09E8 => {
    //   block [0x832B09E8..0x832B09F4)
	// 832B09E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B09EC: 386BDDB4  addi r3, r11, -0x224c
	ctx.r[3].s64 = ctx.r[11].s64 + -8780;
	// 832B09F0: 4AF643E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B09F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B09F8 size=12
    let mut pc: u32 = 0x832B09F8;
    'dispatch: loop {
        match pc {
            0x832B09F8 => {
    //   block [0x832B09F8..0x832B0A04)
	// 832B09F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B09FC: 386BDDB8  addi r3, r11, -0x2248
	ctx.r[3].s64 = ctx.r[11].s64 + -8776;
	// 832B0A00: 4AF643D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A08 size=12
    let mut pc: u32 = 0x832B0A08;
    'dispatch: loop {
        match pc {
            0x832B0A08 => {
    //   block [0x832B0A08..0x832B0A14)
	// 832B0A08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A0C: 386BDDBC  addi r3, r11, -0x2244
	ctx.r[3].s64 = ctx.r[11].s64 + -8772;
	// 832B0A10: 4AF643C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A18 size=12
    let mut pc: u32 = 0x832B0A18;
    'dispatch: loop {
        match pc {
            0x832B0A18 => {
    //   block [0x832B0A18..0x832B0A24)
	// 832B0A18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A1C: 386BDDC0  addi r3, r11, -0x2240
	ctx.r[3].s64 = ctx.r[11].s64 + -8768;
	// 832B0A20: 4AF643B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A28 size=12
    let mut pc: u32 = 0x832B0A28;
    'dispatch: loop {
        match pc {
            0x832B0A28 => {
    //   block [0x832B0A28..0x832B0A34)
	// 832B0A28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A2C: 386BDDC4  addi r3, r11, -0x223c
	ctx.r[3].s64 = ctx.r[11].s64 + -8764;
	// 832B0A30: 4AF643A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A38 size=12
    let mut pc: u32 = 0x832B0A38;
    'dispatch: loop {
        match pc {
            0x832B0A38 => {
    //   block [0x832B0A38..0x832B0A44)
	// 832B0A38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A3C: 386BDDC8  addi r3, r11, -0x2238
	ctx.r[3].s64 = ctx.r[11].s64 + -8760;
	// 832B0A40: 4AF64398  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A48 size=12
    let mut pc: u32 = 0x832B0A48;
    'dispatch: loop {
        match pc {
            0x832B0A48 => {
    //   block [0x832B0A48..0x832B0A54)
	// 832B0A48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A4C: 386BDDCC  addi r3, r11, -0x2234
	ctx.r[3].s64 = ctx.r[11].s64 + -8756;
	// 832B0A50: 4AF64388  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A58 size=12
    let mut pc: u32 = 0x832B0A58;
    'dispatch: loop {
        match pc {
            0x832B0A58 => {
    //   block [0x832B0A58..0x832B0A64)
	// 832B0A58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A5C: 386BDDD0  addi r3, r11, -0x2230
	ctx.r[3].s64 = ctx.r[11].s64 + -8752;
	// 832B0A60: 4AF64378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A68 size=12
    let mut pc: u32 = 0x832B0A68;
    'dispatch: loop {
        match pc {
            0x832B0A68 => {
    //   block [0x832B0A68..0x832B0A74)
	// 832B0A68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A6C: 386BDDD4  addi r3, r11, -0x222c
	ctx.r[3].s64 = ctx.r[11].s64 + -8748;
	// 832B0A70: 4AF64368  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A78 size=12
    let mut pc: u32 = 0x832B0A78;
    'dispatch: loop {
        match pc {
            0x832B0A78 => {
    //   block [0x832B0A78..0x832B0A84)
	// 832B0A78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A7C: 386BDDD8  addi r3, r11, -0x2228
	ctx.r[3].s64 = ctx.r[11].s64 + -8744;
	// 832B0A80: 4AF64358  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A88 size=12
    let mut pc: u32 = 0x832B0A88;
    'dispatch: loop {
        match pc {
            0x832B0A88 => {
    //   block [0x832B0A88..0x832B0A94)
	// 832B0A88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A8C: 386BDDDC  addi r3, r11, -0x2224
	ctx.r[3].s64 = ctx.r[11].s64 + -8740;
	// 832B0A90: 4AF64348  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0A98 size=12
    let mut pc: u32 = 0x832B0A98;
    'dispatch: loop {
        match pc {
            0x832B0A98 => {
    //   block [0x832B0A98..0x832B0AA4)
	// 832B0A98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0A9C: 386BDDE0  addi r3, r11, -0x2220
	ctx.r[3].s64 = ctx.r[11].s64 + -8736;
	// 832B0AA0: 4AF64338  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0AA8 size=12
    let mut pc: u32 = 0x832B0AA8;
    'dispatch: loop {
        match pc {
            0x832B0AA8 => {
    //   block [0x832B0AA8..0x832B0AB4)
	// 832B0AA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0AAC: 386BDDE4  addi r3, r11, -0x221c
	ctx.r[3].s64 = ctx.r[11].s64 + -8732;
	// 832B0AB0: 4AF64328  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0AB8 size=12
    let mut pc: u32 = 0x832B0AB8;
    'dispatch: loop {
        match pc {
            0x832B0AB8 => {
    //   block [0x832B0AB8..0x832B0AC4)
	// 832B0AB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0ABC: 386BDDE8  addi r3, r11, -0x2218
	ctx.r[3].s64 = ctx.r[11].s64 + -8728;
	// 832B0AC0: 4AF64318  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0AC8 size=12
    let mut pc: u32 = 0x832B0AC8;
    'dispatch: loop {
        match pc {
            0x832B0AC8 => {
    //   block [0x832B0AC8..0x832B0AD4)
	// 832B0AC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0ACC: 386BDDEC  addi r3, r11, -0x2214
	ctx.r[3].s64 = ctx.r[11].s64 + -8724;
	// 832B0AD0: 4AF64308  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0AD8 size=12
    let mut pc: u32 = 0x832B0AD8;
    'dispatch: loop {
        match pc {
            0x832B0AD8 => {
    //   block [0x832B0AD8..0x832B0AE4)
	// 832B0AD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0ADC: 386BDDF0  addi r3, r11, -0x2210
	ctx.r[3].s64 = ctx.r[11].s64 + -8720;
	// 832B0AE0: 4AF642F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0AE8 size=12
    let mut pc: u32 = 0x832B0AE8;
    'dispatch: loop {
        match pc {
            0x832B0AE8 => {
    //   block [0x832B0AE8..0x832B0AF4)
	// 832B0AE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0AEC: 386BDDF4  addi r3, r11, -0x220c
	ctx.r[3].s64 = ctx.r[11].s64 + -8716;
	// 832B0AF0: 4AF642E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0AF8 size=12
    let mut pc: u32 = 0x832B0AF8;
    'dispatch: loop {
        match pc {
            0x832B0AF8 => {
    //   block [0x832B0AF8..0x832B0B04)
	// 832B0AF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0AFC: 386BDDF8  addi r3, r11, -0x2208
	ctx.r[3].s64 = ctx.r[11].s64 + -8712;
	// 832B0B00: 4AF642D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B08 size=12
    let mut pc: u32 = 0x832B0B08;
    'dispatch: loop {
        match pc {
            0x832B0B08 => {
    //   block [0x832B0B08..0x832B0B14)
	// 832B0B08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B0C: 386BDDFC  addi r3, r11, -0x2204
	ctx.r[3].s64 = ctx.r[11].s64 + -8708;
	// 832B0B10: 4AF642C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B18 size=12
    let mut pc: u32 = 0x832B0B18;
    'dispatch: loop {
        match pc {
            0x832B0B18 => {
    //   block [0x832B0B18..0x832B0B24)
	// 832B0B18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B1C: 386BDE00  addi r3, r11, -0x2200
	ctx.r[3].s64 = ctx.r[11].s64 + -8704;
	// 832B0B20: 4AF642B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B28 size=12
    let mut pc: u32 = 0x832B0B28;
    'dispatch: loop {
        match pc {
            0x832B0B28 => {
    //   block [0x832B0B28..0x832B0B34)
	// 832B0B28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B2C: 386BDE04  addi r3, r11, -0x21fc
	ctx.r[3].s64 = ctx.r[11].s64 + -8700;
	// 832B0B30: 4AF642A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B38 size=12
    let mut pc: u32 = 0x832B0B38;
    'dispatch: loop {
        match pc {
            0x832B0B38 => {
    //   block [0x832B0B38..0x832B0B44)
	// 832B0B38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B3C: 386BDE08  addi r3, r11, -0x21f8
	ctx.r[3].s64 = ctx.r[11].s64 + -8696;
	// 832B0B40: 4AF64298  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B48 size=12
    let mut pc: u32 = 0x832B0B48;
    'dispatch: loop {
        match pc {
            0x832B0B48 => {
    //   block [0x832B0B48..0x832B0B54)
	// 832B0B48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B4C: 386BDE0C  addi r3, r11, -0x21f4
	ctx.r[3].s64 = ctx.r[11].s64 + -8692;
	// 832B0B50: 4AF64288  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B58 size=12
    let mut pc: u32 = 0x832B0B58;
    'dispatch: loop {
        match pc {
            0x832B0B58 => {
    //   block [0x832B0B58..0x832B0B64)
	// 832B0B58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B5C: 386BDE10  addi r3, r11, -0x21f0
	ctx.r[3].s64 = ctx.r[11].s64 + -8688;
	// 832B0B60: 4AF64278  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B68 size=12
    let mut pc: u32 = 0x832B0B68;
    'dispatch: loop {
        match pc {
            0x832B0B68 => {
    //   block [0x832B0B68..0x832B0B74)
	// 832B0B68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B6C: 386BDE14  addi r3, r11, -0x21ec
	ctx.r[3].s64 = ctx.r[11].s64 + -8684;
	// 832B0B70: 4AF64268  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B78 size=12
    let mut pc: u32 = 0x832B0B78;
    'dispatch: loop {
        match pc {
            0x832B0B78 => {
    //   block [0x832B0B78..0x832B0B84)
	// 832B0B78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B7C: 386BDE18  addi r3, r11, -0x21e8
	ctx.r[3].s64 = ctx.r[11].s64 + -8680;
	// 832B0B80: 4AF64258  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B88 size=12
    let mut pc: u32 = 0x832B0B88;
    'dispatch: loop {
        match pc {
            0x832B0B88 => {
    //   block [0x832B0B88..0x832B0B94)
	// 832B0B88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B8C: 386BDE1C  addi r3, r11, -0x21e4
	ctx.r[3].s64 = ctx.r[11].s64 + -8676;
	// 832B0B90: 4AF64248  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0B98 size=12
    let mut pc: u32 = 0x832B0B98;
    'dispatch: loop {
        match pc {
            0x832B0B98 => {
    //   block [0x832B0B98..0x832B0BA4)
	// 832B0B98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0B9C: 386BDE20  addi r3, r11, -0x21e0
	ctx.r[3].s64 = ctx.r[11].s64 + -8672;
	// 832B0BA0: 4AF64238  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0BA8 size=12
    let mut pc: u32 = 0x832B0BA8;
    'dispatch: loop {
        match pc {
            0x832B0BA8 => {
    //   block [0x832B0BA8..0x832B0BB4)
	// 832B0BA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0BAC: 386BDE24  addi r3, r11, -0x21dc
	ctx.r[3].s64 = ctx.r[11].s64 + -8668;
	// 832B0BB0: 4AF64228  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0BB8 size=12
    let mut pc: u32 = 0x832B0BB8;
    'dispatch: loop {
        match pc {
            0x832B0BB8 => {
    //   block [0x832B0BB8..0x832B0BC4)
	// 832B0BB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0BBC: 386BDE28  addi r3, r11, -0x21d8
	ctx.r[3].s64 = ctx.r[11].s64 + -8664;
	// 832B0BC0: 4AF64218  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0BC8 size=12
    let mut pc: u32 = 0x832B0BC8;
    'dispatch: loop {
        match pc {
            0x832B0BC8 => {
    //   block [0x832B0BC8..0x832B0BD4)
	// 832B0BC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0BCC: 386BDE2C  addi r3, r11, -0x21d4
	ctx.r[3].s64 = ctx.r[11].s64 + -8660;
	// 832B0BD0: 4AF64208  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0BD8 size=12
    let mut pc: u32 = 0x832B0BD8;
    'dispatch: loop {
        match pc {
            0x832B0BD8 => {
    //   block [0x832B0BD8..0x832B0BE4)
	// 832B0BD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0BDC: 386BDE30  addi r3, r11, -0x21d0
	ctx.r[3].s64 = ctx.r[11].s64 + -8656;
	// 832B0BE0: 4AF641F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0BE8 size=12
    let mut pc: u32 = 0x832B0BE8;
    'dispatch: loop {
        match pc {
            0x832B0BE8 => {
    //   block [0x832B0BE8..0x832B0BF4)
	// 832B0BE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0BEC: 386BDE34  addi r3, r11, -0x21cc
	ctx.r[3].s64 = ctx.r[11].s64 + -8652;
	// 832B0BF0: 4AF641E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0BF8 size=12
    let mut pc: u32 = 0x832B0BF8;
    'dispatch: loop {
        match pc {
            0x832B0BF8 => {
    //   block [0x832B0BF8..0x832B0C04)
	// 832B0BF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0BFC: 386BDE38  addi r3, r11, -0x21c8
	ctx.r[3].s64 = ctx.r[11].s64 + -8648;
	// 832B0C00: 4AF641D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C08 size=12
    let mut pc: u32 = 0x832B0C08;
    'dispatch: loop {
        match pc {
            0x832B0C08 => {
    //   block [0x832B0C08..0x832B0C14)
	// 832B0C08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C0C: 386BDE3C  addi r3, r11, -0x21c4
	ctx.r[3].s64 = ctx.r[11].s64 + -8644;
	// 832B0C10: 4AF641C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C18 size=12
    let mut pc: u32 = 0x832B0C18;
    'dispatch: loop {
        match pc {
            0x832B0C18 => {
    //   block [0x832B0C18..0x832B0C24)
	// 832B0C18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C1C: 386BDE40  addi r3, r11, -0x21c0
	ctx.r[3].s64 = ctx.r[11].s64 + -8640;
	// 832B0C20: 4AF641B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C28 size=12
    let mut pc: u32 = 0x832B0C28;
    'dispatch: loop {
        match pc {
            0x832B0C28 => {
    //   block [0x832B0C28..0x832B0C34)
	// 832B0C28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C2C: 386BDE44  addi r3, r11, -0x21bc
	ctx.r[3].s64 = ctx.r[11].s64 + -8636;
	// 832B0C30: 4AF641A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C38 size=12
    let mut pc: u32 = 0x832B0C38;
    'dispatch: loop {
        match pc {
            0x832B0C38 => {
    //   block [0x832B0C38..0x832B0C44)
	// 832B0C38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C3C: 386BDE48  addi r3, r11, -0x21b8
	ctx.r[3].s64 = ctx.r[11].s64 + -8632;
	// 832B0C40: 4AF64198  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C48 size=12
    let mut pc: u32 = 0x832B0C48;
    'dispatch: loop {
        match pc {
            0x832B0C48 => {
    //   block [0x832B0C48..0x832B0C54)
	// 832B0C48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C4C: 386BDE4C  addi r3, r11, -0x21b4
	ctx.r[3].s64 = ctx.r[11].s64 + -8628;
	// 832B0C50: 4AF64188  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C58 size=12
    let mut pc: u32 = 0x832B0C58;
    'dispatch: loop {
        match pc {
            0x832B0C58 => {
    //   block [0x832B0C58..0x832B0C64)
	// 832B0C58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C5C: 386BDE50  addi r3, r11, -0x21b0
	ctx.r[3].s64 = ctx.r[11].s64 + -8624;
	// 832B0C60: 4AF64178  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C68 size=12
    let mut pc: u32 = 0x832B0C68;
    'dispatch: loop {
        match pc {
            0x832B0C68 => {
    //   block [0x832B0C68..0x832B0C74)
	// 832B0C68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C6C: 386BDE54  addi r3, r11, -0x21ac
	ctx.r[3].s64 = ctx.r[11].s64 + -8620;
	// 832B0C70: 4AF64168  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C78 size=12
    let mut pc: u32 = 0x832B0C78;
    'dispatch: loop {
        match pc {
            0x832B0C78 => {
    //   block [0x832B0C78..0x832B0C84)
	// 832B0C78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C7C: 386BDE58  addi r3, r11, -0x21a8
	ctx.r[3].s64 = ctx.r[11].s64 + -8616;
	// 832B0C80: 4AF64158  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C88 size=12
    let mut pc: u32 = 0x832B0C88;
    'dispatch: loop {
        match pc {
            0x832B0C88 => {
    //   block [0x832B0C88..0x832B0C94)
	// 832B0C88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C8C: 386BDE5C  addi r3, r11, -0x21a4
	ctx.r[3].s64 = ctx.r[11].s64 + -8612;
	// 832B0C90: 4AF64148  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0C98 size=12
    let mut pc: u32 = 0x832B0C98;
    'dispatch: loop {
        match pc {
            0x832B0C98 => {
    //   block [0x832B0C98..0x832B0CA4)
	// 832B0C98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0C9C: 386BDE60  addi r3, r11, -0x21a0
	ctx.r[3].s64 = ctx.r[11].s64 + -8608;
	// 832B0CA0: 4AF64138  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0CA8 size=12
    let mut pc: u32 = 0x832B0CA8;
    'dispatch: loop {
        match pc {
            0x832B0CA8 => {
    //   block [0x832B0CA8..0x832B0CB4)
	// 832B0CA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0CAC: 386BDE64  addi r3, r11, -0x219c
	ctx.r[3].s64 = ctx.r[11].s64 + -8604;
	// 832B0CB0: 4AF64128  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0CB8 size=12
    let mut pc: u32 = 0x832B0CB8;
    'dispatch: loop {
        match pc {
            0x832B0CB8 => {
    //   block [0x832B0CB8..0x832B0CC4)
	// 832B0CB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0CBC: 386BDE68  addi r3, r11, -0x2198
	ctx.r[3].s64 = ctx.r[11].s64 + -8600;
	// 832B0CC0: 4AF64118  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0CC8 size=12
    let mut pc: u32 = 0x832B0CC8;
    'dispatch: loop {
        match pc {
            0x832B0CC8 => {
    //   block [0x832B0CC8..0x832B0CD4)
	// 832B0CC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0CCC: 386BDE6C  addi r3, r11, -0x2194
	ctx.r[3].s64 = ctx.r[11].s64 + -8596;
	// 832B0CD0: 4AF64108  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0CD8 size=12
    let mut pc: u32 = 0x832B0CD8;
    'dispatch: loop {
        match pc {
            0x832B0CD8 => {
    //   block [0x832B0CD8..0x832B0CE4)
	// 832B0CD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0CDC: 386BDE70  addi r3, r11, -0x2190
	ctx.r[3].s64 = ctx.r[11].s64 + -8592;
	// 832B0CE0: 4AF640F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0CE8 size=12
    let mut pc: u32 = 0x832B0CE8;
    'dispatch: loop {
        match pc {
            0x832B0CE8 => {
    //   block [0x832B0CE8..0x832B0CF4)
	// 832B0CE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0CEC: 386BDE74  addi r3, r11, -0x218c
	ctx.r[3].s64 = ctx.r[11].s64 + -8588;
	// 832B0CF0: 4AF640E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0CF8 size=12
    let mut pc: u32 = 0x832B0CF8;
    'dispatch: loop {
        match pc {
            0x832B0CF8 => {
    //   block [0x832B0CF8..0x832B0D04)
	// 832B0CF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0CFC: 386BDE78  addi r3, r11, -0x2188
	ctx.r[3].s64 = ctx.r[11].s64 + -8584;
	// 832B0D00: 4AF640D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D08 size=12
    let mut pc: u32 = 0x832B0D08;
    'dispatch: loop {
        match pc {
            0x832B0D08 => {
    //   block [0x832B0D08..0x832B0D14)
	// 832B0D08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D0C: 386BDE7C  addi r3, r11, -0x2184
	ctx.r[3].s64 = ctx.r[11].s64 + -8580;
	// 832B0D10: 4AF640C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D18 size=12
    let mut pc: u32 = 0x832B0D18;
    'dispatch: loop {
        match pc {
            0x832B0D18 => {
    //   block [0x832B0D18..0x832B0D24)
	// 832B0D18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D1C: 386BDE80  addi r3, r11, -0x2180
	ctx.r[3].s64 = ctx.r[11].s64 + -8576;
	// 832B0D20: 4AF640B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D28 size=12
    let mut pc: u32 = 0x832B0D28;
    'dispatch: loop {
        match pc {
            0x832B0D28 => {
    //   block [0x832B0D28..0x832B0D34)
	// 832B0D28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D2C: 386BDE84  addi r3, r11, -0x217c
	ctx.r[3].s64 = ctx.r[11].s64 + -8572;
	// 832B0D30: 4AF640A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D38 size=12
    let mut pc: u32 = 0x832B0D38;
    'dispatch: loop {
        match pc {
            0x832B0D38 => {
    //   block [0x832B0D38..0x832B0D44)
	// 832B0D38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D3C: 386BDE88  addi r3, r11, -0x2178
	ctx.r[3].s64 = ctx.r[11].s64 + -8568;
	// 832B0D40: 4AF64098  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D48 size=12
    let mut pc: u32 = 0x832B0D48;
    'dispatch: loop {
        match pc {
            0x832B0D48 => {
    //   block [0x832B0D48..0x832B0D54)
	// 832B0D48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D4C: 386BDE8C  addi r3, r11, -0x2174
	ctx.r[3].s64 = ctx.r[11].s64 + -8564;
	// 832B0D50: 4AF64088  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D58 size=12
    let mut pc: u32 = 0x832B0D58;
    'dispatch: loop {
        match pc {
            0x832B0D58 => {
    //   block [0x832B0D58..0x832B0D64)
	// 832B0D58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D5C: 386BDE90  addi r3, r11, -0x2170
	ctx.r[3].s64 = ctx.r[11].s64 + -8560;
	// 832B0D60: 4AF64078  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D68 size=12
    let mut pc: u32 = 0x832B0D68;
    'dispatch: loop {
        match pc {
            0x832B0D68 => {
    //   block [0x832B0D68..0x832B0D74)
	// 832B0D68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D6C: 386BDE94  addi r3, r11, -0x216c
	ctx.r[3].s64 = ctx.r[11].s64 + -8556;
	// 832B0D70: 4AF64068  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D78 size=12
    let mut pc: u32 = 0x832B0D78;
    'dispatch: loop {
        match pc {
            0x832B0D78 => {
    //   block [0x832B0D78..0x832B0D84)
	// 832B0D78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D7C: 386BDE98  addi r3, r11, -0x2168
	ctx.r[3].s64 = ctx.r[11].s64 + -8552;
	// 832B0D80: 4AF64058  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D88 size=12
    let mut pc: u32 = 0x832B0D88;
    'dispatch: loop {
        match pc {
            0x832B0D88 => {
    //   block [0x832B0D88..0x832B0D94)
	// 832B0D88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D8C: 386BDE9C  addi r3, r11, -0x2164
	ctx.r[3].s64 = ctx.r[11].s64 + -8548;
	// 832B0D90: 4AF64048  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0D98 size=12
    let mut pc: u32 = 0x832B0D98;
    'dispatch: loop {
        match pc {
            0x832B0D98 => {
    //   block [0x832B0D98..0x832B0DA4)
	// 832B0D98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0D9C: 386BDEA0  addi r3, r11, -0x2160
	ctx.r[3].s64 = ctx.r[11].s64 + -8544;
	// 832B0DA0: 4AF64038  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0DA8 size=12
    let mut pc: u32 = 0x832B0DA8;
    'dispatch: loop {
        match pc {
            0x832B0DA8 => {
    //   block [0x832B0DA8..0x832B0DB4)
	// 832B0DA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0DAC: 386BDEA4  addi r3, r11, -0x215c
	ctx.r[3].s64 = ctx.r[11].s64 + -8540;
	// 832B0DB0: 4AF64028  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0DB8 size=12
    let mut pc: u32 = 0x832B0DB8;
    'dispatch: loop {
        match pc {
            0x832B0DB8 => {
    //   block [0x832B0DB8..0x832B0DC4)
	// 832B0DB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0DBC: 386BDEA8  addi r3, r11, -0x2158
	ctx.r[3].s64 = ctx.r[11].s64 + -8536;
	// 832B0DC0: 4AF64018  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0DC8 size=12
    let mut pc: u32 = 0x832B0DC8;
    'dispatch: loop {
        match pc {
            0x832B0DC8 => {
    //   block [0x832B0DC8..0x832B0DD4)
	// 832B0DC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0DCC: 386BDEAC  addi r3, r11, -0x2154
	ctx.r[3].s64 = ctx.r[11].s64 + -8532;
	// 832B0DD0: 4AF64008  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0DD8 size=12
    let mut pc: u32 = 0x832B0DD8;
    'dispatch: loop {
        match pc {
            0x832B0DD8 => {
    //   block [0x832B0DD8..0x832B0DE4)
	// 832B0DD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0DDC: 386BDEB0  addi r3, r11, -0x2150
	ctx.r[3].s64 = ctx.r[11].s64 + -8528;
	// 832B0DE0: 4AF63FF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0DE8 size=12
    let mut pc: u32 = 0x832B0DE8;
    'dispatch: loop {
        match pc {
            0x832B0DE8 => {
    //   block [0x832B0DE8..0x832B0DF4)
	// 832B0DE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0DEC: 386BDEB4  addi r3, r11, -0x214c
	ctx.r[3].s64 = ctx.r[11].s64 + -8524;
	// 832B0DF0: 4AF63FE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0DF8 size=12
    let mut pc: u32 = 0x832B0DF8;
    'dispatch: loop {
        match pc {
            0x832B0DF8 => {
    //   block [0x832B0DF8..0x832B0E04)
	// 832B0DF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0DFC: 386BDEB8  addi r3, r11, -0x2148
	ctx.r[3].s64 = ctx.r[11].s64 + -8520;
	// 832B0E00: 4AF63FD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E08 size=12
    let mut pc: u32 = 0x832B0E08;
    'dispatch: loop {
        match pc {
            0x832B0E08 => {
    //   block [0x832B0E08..0x832B0E14)
	// 832B0E08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E0C: 386BDEBC  addi r3, r11, -0x2144
	ctx.r[3].s64 = ctx.r[11].s64 + -8516;
	// 832B0E10: 4AF63FC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E18 size=12
    let mut pc: u32 = 0x832B0E18;
    'dispatch: loop {
        match pc {
            0x832B0E18 => {
    //   block [0x832B0E18..0x832B0E24)
	// 832B0E18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E1C: 386BDEC0  addi r3, r11, -0x2140
	ctx.r[3].s64 = ctx.r[11].s64 + -8512;
	// 832B0E20: 4AF63FB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E28 size=12
    let mut pc: u32 = 0x832B0E28;
    'dispatch: loop {
        match pc {
            0x832B0E28 => {
    //   block [0x832B0E28..0x832B0E34)
	// 832B0E28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E2C: 386BDEC4  addi r3, r11, -0x213c
	ctx.r[3].s64 = ctx.r[11].s64 + -8508;
	// 832B0E30: 4AF63FA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E38 size=12
    let mut pc: u32 = 0x832B0E38;
    'dispatch: loop {
        match pc {
            0x832B0E38 => {
    //   block [0x832B0E38..0x832B0E44)
	// 832B0E38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E3C: 386BDEC8  addi r3, r11, -0x2138
	ctx.r[3].s64 = ctx.r[11].s64 + -8504;
	// 832B0E40: 4AF63F98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E48 size=12
    let mut pc: u32 = 0x832B0E48;
    'dispatch: loop {
        match pc {
            0x832B0E48 => {
    //   block [0x832B0E48..0x832B0E54)
	// 832B0E48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E4C: 386BDECC  addi r3, r11, -0x2134
	ctx.r[3].s64 = ctx.r[11].s64 + -8500;
	// 832B0E50: 4AF63F88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E58 size=12
    let mut pc: u32 = 0x832B0E58;
    'dispatch: loop {
        match pc {
            0x832B0E58 => {
    //   block [0x832B0E58..0x832B0E64)
	// 832B0E58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E5C: 386BDED0  addi r3, r11, -0x2130
	ctx.r[3].s64 = ctx.r[11].s64 + -8496;
	// 832B0E60: 4AF63F78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B0E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B0E68 size=12
    let mut pc: u32 = 0x832B0E68;
    'dispatch: loop {
        match pc {
            0x832B0E68 => {
    //   block [0x832B0E68..0x832B0E74)
	// 832B0E68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B0E6C: 386BDED4  addi r3, r11, -0x212c
	ctx.r[3].s64 = ctx.r[11].s64 + -8492;
	// 832B0E70: 4AF63F68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


